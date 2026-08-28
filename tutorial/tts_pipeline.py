"""🔊 Tutorial TTS — speech generated per subtitle clause and placed on the video's own clock.

The voiceover reads the German subtitles that are already on screen, so the two
can only drift if the audio is treated as one opaque block. Every clause is
therefore synthesized, measured and positioned separately:

1. ``synthesize_clause_audio`` renders one file per NARRATION clause.
2. ``write_vo_timing_manifest`` feeds the measured lengths back to ``hold_for``.
3. Rendering with ``VO_TRACE=1`` records when each subtitle appears.
4. ``assemble_aligned_track`` lays each clause's speech on that exact timestamp.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
import wave
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any, Optional, Protocol

from manim_visuals import Clause

_DEFAULT_NOWI_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
_GEMINI_MODEL = "google/gemini-3.1-flash-tts-preview"
_CLAUSE_WORKERS = 4


#region Configuration
def configure_tutorial_tts() -> None:
    """⚙️ Point TTS at Gemini on OpenRouter, reusing NowIGetIt credentials when present."""
    if _DEFAULT_NOWI_ROOT.is_dir():
        try:
            from dotenv import dotenv_values, load_dotenv  # noqa: PLC0415

            load_dotenv(_DEFAULT_NOWI_ROOT / ".env", override=False)
            local = _DEFAULT_NOWI_ROOT / ".env.local"
            if local.is_file():
                for key, value in dotenv_values(local).items():
                    if key and value is not None and str(value).strip():
                        os.environ.setdefault(key, str(value))
        except Exception:  # noqa: BLE001
            pass
    os.environ["TTS_MODEL"] = os.getenv("TUTORIAL_TTS_MODEL") or _GEMINI_MODEL
    os.environ["TTS_BASE_URL"] = os.getenv("TUTORIAL_TTS_BASE_URL") or "https://openrouter.ai/api/v1"
    if not (os.getenv("TTS_API_KEY") or "").strip():
        openrouter = (os.getenv("OPENROUTER_API_KEY") or "").strip()
        if openrouter:
            os.environ["TTS_API_KEY"] = openrouter
#endregion


#region Subtitle source
def subtitle_clauses(narration: list[Clause]) -> list[str]:
    """💬 One German subtitle line per clause — empty entries stay silent staging."""
    return [text_de.strip() for _, _, text_de in narration]


def subtitle_narration_text(narration: list[Clause], key: str | None = None) -> str:
    """📝 Join subtitle clauses into one spoken string."""
    return " ".join(
        text_de for section, _, text_de in narration
        if text_de.strip() and (key is None or section == key)
    )
#endregion


#region Audio measurement and assembly
def probe_duration(path: Path) -> float:
    """⏱️ Media length in seconds — WAV via stdlib, everything else via ffprobe."""
    path = Path(path)
    if not path.is_file():
        return 0.0
    if path.suffix.lower() == ".wav":
        try:
            with wave.open(str(path), "rb") as wf:
                rate = wf.getframerate()
                if rate > 0:
                    return wf.getnframes() / float(rate)
        except Exception:  # noqa: BLE001
            pass
    if not shutil.which("ffprobe"):
        return 0.0
    try:
        proc = subprocess.run(
            [
                "ffprobe", "-v", "error", "-show_entries", "format=duration",
                "-of", "default=nw=1:nk=1", str(path),
            ],
            capture_output=True, text=True, timeout=30, check=False,
        )
        return float((proc.stdout or "0").strip() or 0)
    except Exception:  # noqa: BLE001
        return 0.0


def assemble_aligned_track(
    placements: list[tuple[Path, float]],
    output_path: Path,
    *,
    total_duration: float,
) -> Optional[Path]:
    """🧩 Mix each clause's speech in at its own start time, padded to ``total_duration``."""
    usable = [(Path(p), max(0.0, float(t))) for p, t in placements if Path(p).is_file()]
    if not usable or not shutil.which("ffmpeg"):
        return None
    output_path.parent.mkdir(parents=True, exist_ok=True)
    total = max(total_duration, max(start for _, start in usable) + 0.5)

    inputs: list[str] = []
    chains: list[str] = []
    labels: list[str] = []
    for i, (path, start) in enumerate(usable):
        inputs += ["-i", str(path)]
        delay_ms = int(round(start * 1000))
        chains.append(
            f"[{i}:a]aresample=44100,aformat=sample_fmts=fltp:channel_layouts=mono,"
            f"adelay={delay_ms}|{delay_ms}[a{i}]"
        )
        labels.append(f"[a{i}]")
    mix = (
        f"{''.join(labels)}amix=inputs={len(usable)}:normalize=0:dropout_transition=0[m];"
        f"[m]apad=whole_dur={total:.3f},atrim=0:{total:.3f}[out]"
    )
    filter_complex = ";".join(chains + [mix])
    cmd = [
        "ffmpeg", "-y", *inputs,
        "-filter_complex", filter_complex,
        "-map", "[out]", "-c:a", "libmp3lame", "-q:a", "3",
        str(output_path),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=600, check=False)
    if proc.returncode != 0 or not output_path.is_file():
        raise RuntimeError(f"ffmpeg mix failed: {proc.stderr[-600:]}")
    return output_path
#endregion


#region Backends
class _TtsBackend(Protocol):
    def synthesize(self, text: str, output_path: Path, *, voice: str | None = None) -> tuple[Optional[str], bool]: ...


class _NowIGetItBackend:
    """🎛️ NowIGetIt pipeline — used when its credentials are the only ones available."""

    def __init__(self) -> None:
        root = _DEFAULT_NOWI_ROOT
        if root.is_dir() and str(root) not in sys.path:
            sys.path.insert(0, str(root))
        from backend.pipeline.tts import synthesize_narration  # noqa: PLC0415

        self._one = synthesize_narration

    def synthesize(self, text: str, output_path: Path, *, voice: str | None = None) -> tuple[Optional[str], bool]:
        return self._one(text, output_path, voice=voice)


class _OpenRouterGeminiBackend:
    """🌐 Gemini TTS over OpenRouter's OpenAI-compatible /audio/speech endpoint."""

    def __init__(self) -> None:
        import httpx  # noqa: PLC0415

        self._httpx = httpx
        self.api_key = (os.getenv("TTS_API_KEY") or os.getenv("OPENROUTER_API_KEY") or "").strip()
        self._base = (os.getenv("TTS_BASE_URL") or "https://openrouter.ai/api/v1").rstrip("/")
        self._model = os.getenv("TTS_MODEL") or _GEMINI_MODEL
        self._voice = os.getenv("TTS_VOICE") or "Kore"
        self._referer = os.getenv("OPENROUTER_SITE_URL", "https://nowigetit.app")
        self._title = os.getenv("OPENROUTER_APP_NAME", "semio-tutorial")

    def synthesize(self, text: str, output_path: Path, *, voice: str | None = None) -> tuple[Optional[str], bool]:
        if not self.api_key or not text.strip():
            return None, True
        use_pcm = "gemini" in self._model.lower() and "tts" in self._model.lower()
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": self._referer,
            "X-Title": self._title,
        }
        payload = {
            "model": self._model,
            "input": text.strip(),
            "voice": voice or self._voice,
            "response_format": "pcm" if use_pcm else "mp3",
        }
        with self._httpx.Client(timeout=self._httpx.Timeout(120.0, connect=30.0)) as client:
            resp = client.post(f"{self._base}/audio/speech", headers=headers, json=payload)
        if resp.status_code != 200:
            raise RuntimeError(f"TTS failed {resp.status_code}: {resp.text[:300]}")
        if use_pcm:
            out = output_path.with_suffix(".wav")
            rate, channels = 24000, 1
            content_type = resp.headers.get("content-type") or ""
            m_rate = re.search(r"rate=(\d+)", content_type, re.I)
            m_ch = re.search(r"channels=(\d+)", content_type, re.I)
            if m_rate:
                rate = int(m_rate.group(1))
            if m_ch:
                channels = int(m_ch.group(1))
            out.parent.mkdir(parents=True, exist_ok=True)
            with wave.open(str(out), "wb") as wf:
                wf.setnchannels(channels)
                wf.setsampwidth(2)
                wf.setframerate(rate)
                wf.writeframes(resp.content)
            return str(out), False
        out = output_path.with_suffix(".mp3")
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(resp.content)
        return str(out), False


def resolve_backends() -> list[_TtsBackend]:
    """🔌 Gemini first, NowIGetIt as the fallback when its key is the working one."""
    configure_tutorial_tts()
    backends: list[_TtsBackend] = []
    gemini = _OpenRouterGeminiBackend()
    if gemini.api_key:
        backends.append(gemini)
    if _DEFAULT_NOWI_ROOT.is_dir():
        try:
            backends.append(_NowIGetItBackend())
        except Exception:  # noqa: BLE001
            pass
    return backends or [gemini]
#endregion


#region Clause synthesis
def synthesize_clause_audio(
    narration: list[Clause],
    clause_dir: Path,
    *,
    voice: str | None = None,
) -> list[tuple[str, Optional[Path], float]]:
    """🎙️ One audio file per subtitle clause, returned with its measured length."""
    backends = resolve_backends()
    clause_dir.mkdir(parents=True, exist_ok=True)
    texts = subtitle_clauses(narration)

    def _render(item: tuple[int, str]) -> tuple[int, Optional[Path]]:
        index, text = item
        if not text:
            return index, None
        target = clause_dir / f"clause_{index:02d}"
        last_error: Exception | None = None
        for backend in backends:
            try:
                path, skipped = backend.synthesize(text, target, voice=voice)
            except Exception as exc:  # noqa: BLE001
                last_error = exc
                continue
            if not skipped and path:
                return index, Path(path)
        if last_error:
            print(f"  ! clause {index}: {last_error}", file=sys.stderr)
        return index, None

    spoken = [(i, t) for i, t in enumerate(texts) if t]
    with ThreadPoolExecutor(max_workers=_CLAUSE_WORKERS) as pool:
        rendered = dict(pool.map(_render, spoken))

    results: list[tuple[str, Optional[Path], float]] = []
    for index, (section, _, _) in enumerate(narration):
        path = rendered.get(index)
        results.append((section, path, probe_duration(path) if path else 0.0))
    return results
#endregion


#region Manifests
def write_vo_timing_manifest(
    beats: list[tuple[str, list[tuple[str, Optional[Path], float]]]],
    path: Path,
    *,
    merge_existing: bool = True,
) -> None:
    """📋 Beat class → clause key → measured seconds, read back by ``hold_for``."""
    payload: dict[str, Any] = {"source": "subtitle", "beats": {}}
    if merge_existing and path.is_file():
        try:
            existing = json.loads(path.read_text(encoding="utf-8"))
            if isinstance(existing.get("beats"), dict):
                payload["beats"] = dict(existing["beats"])
        except Exception:  # noqa: BLE001
            pass
    for beat_name, clauses in beats:
        payload["beats"][beat_name] = {
            section: round(seconds, 3) for section, _, seconds in clauses if seconds > 0
        }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def read_vo_trace(path: Path) -> dict[str, dict[str, dict[str, float]]]:
    """📍 Subtitle appearance times captured by rendering with ``VO_TRACE=1``."""
    if not Path(path).is_file():
        return {}
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    beats = data.get("beats") if isinstance(data, dict) else None
    return beats if isinstance(beats, dict) else {}
#endregion
