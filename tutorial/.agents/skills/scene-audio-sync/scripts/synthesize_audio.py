#!/usr/bin/env python3
"""Synthesize slow, clear narration audio from an agent-authored transcript.json."""

from __future__ import annotations

import argparse
import asyncio
import json
import os
import sys
import wave
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any, Optional

BEAT_GAP_SECONDS = 0.18


def wav_duration_seconds(path: Path) -> float:
    try:
        with wave.open(str(path), "rb") as wf:
            rate = wf.getframerate()
            if rate <= 0:
                return 0.0
            return wf.getnframes() / float(rate)
    except Exception:  # noqa: BLE001
        return 0.0


def pcm_to_wav(
    pcm: bytes,
    path: Path,
    *,
    sample_rate: int = 24000,
    channels: int = 1,
    sample_width: int = 2,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(path), "wb") as wf:
        wf.setnchannels(channels)
        wf.setsampwidth(sample_width)
        wf.setframerate(sample_rate)
        wf.writeframes(pcm)


def concat_wav_files(
    parts: list[Path],
    output_path: Path,
    *,
    gap_seconds: float = BEAT_GAP_SECONDS,
) -> list[float]:
    readable = [p for p in parts if p.exists()]
    if not readable:
        return []
    with wave.open(str(readable[0]), "rb") as first:
        params = first.getparams()
    gap_frames = max(0, int(params.framerate * max(0.0, gap_seconds)))
    silence = b"\x00" * (gap_frames * params.sampwidth * params.nchannels)
    durations: list[float] = []
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(output_path), "wb") as out:
        out.setnchannels(params.nchannels)
        out.setsampwidth(params.sampwidth)
        out.setframerate(params.framerate)
        for i, part in enumerate(readable):
            with wave.open(str(part), "rb") as wf:
                if (
                    wf.getnchannels() != params.nchannels
                    or wf.getsampwidth() != params.sampwidth
                    or wf.getframerate() != params.framerate
                ):
                    return []
                frames = wf.readframes(wf.getnframes())
                count = wf.getnframes()
            out.writeframes(frames)
            if i < len(readable) - 1 and gap_frames:
                out.writeframes(silence)
                count += gap_frames
            durations.append(count / float(params.framerate))
    return durations


async def _edge_save(text: str, path: Path, *, voice: str, rate: str, pitch: str) -> None:
    import edge_tts

    path.parent.mkdir(parents=True, exist_ok=True)
    # edge-tts writes mp3; we convert to wav via ffmpeg if available, else keep mp3→wav with decode
    mp3_path = path.with_suffix(".mp3")
    communicate = edge_tts.Communicate(text, voice=voice, rate=rate, pitch=pitch)
    await communicate.save(str(mp3_path))
    _mp3_to_wav(mp3_path, path)
    if path.exists() and path.suffix == ".wav":
        mp3_path.unlink(missing_ok=True)


def _mp3_to_wav(mp3: Path, wav: Path) -> None:
    import shutil
    import subprocess

    if not shutil.which("ffmpeg"):
        # Keep mp3 if ffmpeg missing — mux script accepts both
        if mp3 != wav:
            wav.write_bytes(mp3.read_bytes()) if wav.suffix == ".mp3" else None
        return
    wav.parent.mkdir(parents=True, exist_ok=True)
    proc = subprocess.run(
        [
            "ffmpeg",
            "-y",
            "-i",
            str(mp3),
            "-acodec",
            "pcm_s16le",
            "-ar",
            "24000",
            "-ac",
            "1",
            str(wav),
        ],
        capture_output=True,
        timeout=120,
    )
    if proc.returncode != 0 or not wav.exists():
        raise RuntimeError(f"ffmpeg mp3→wav failed: {(proc.stderr or b'')[-400:]!r}")


def synthesize_edge(text: str, path: Path, *, voice: str, rate: str, pitch: str) -> Path:
    text = (text or "").strip()
    if not text:
        raise ValueError("Empty narration text")
    out = path.with_suffix(".wav")
    asyncio.run(_edge_save(text, out, voice=voice, rate=rate, pitch=pitch))
    return out


def synthesize_macos(text: str, path: Path, *, voice: str = "Samantha", rate_wpm: int = 145) -> Path:
    import subprocess

    aiff = path.with_suffix(".aiff")
    wav = path.with_suffix(".wav")
    aiff.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["say", "-v", voice, "-r", str(rate_wpm), "-o", str(aiff), text],
        check=True,
        timeout=120,
    )
    _mp3_to_wav(aiff, wav)  # ffmpeg handles aiff too
    aiff.unlink(missing_ok=True)
    return wav


def synthesize_openrouter(
    text: str,
    path: Path,
    *,
    voice: str = "Kore",
) -> Path:
    import re

    import httpx

    api_key = (os.getenv("TTS_API_KEY") or os.getenv("OPENROUTER_API_KEY") or "").strip()
    if not api_key:
        raise RuntimeError("OPENROUTER_API_KEY / TTS_API_KEY required for --engine openrouter")
    base = (os.getenv("TTS_BASE_URL") or "https://openrouter.ai/api/v1").rstrip("/")
    model = os.getenv("TTS_MODEL") or "google/gemini-3.1-flash-tts-preview"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
        "HTTP-Referer": os.getenv("OPENROUTER_SITE_URL", "https://nowigetit.app"),
        "X-Title": os.getenv("OPENROUTER_APP_NAME", "scene-audio-sync"),
    }
    use_pcm = "gemini" in model.lower() and "tts" in model.lower()
    payload = {
        "model": model,
        "input": text.strip(),
        "voice": voice,
        "response_format": "pcm" if use_pcm else "mp3",
    }
    with httpx.Client(timeout=httpx.Timeout(120.0, connect=30.0)) as client:
        resp = client.post(f"{base}/audio/speech", headers=headers, json=payload)
    if resp.status_code != 200:
        raise RuntimeError(f"TTS failed {resp.status_code}: {resp.text[:500]}")
    path.parent.mkdir(parents=True, exist_ok=True)
    if use_pcm:
        rate, channels = 24000, 1
        ct = resp.headers.get("content-type") or ""
        m_rate = re.search(r"rate=(\d+)", ct, re.I)
        m_ch = re.search(r"channels=(\d+)", ct, re.I)
        if m_rate:
            rate = int(m_rate.group(1))
        if m_ch:
            channels = int(m_ch.group(1))
        out = path.with_suffix(".wav")
        pcm_to_wav(resp.content, out, sample_rate=rate, channels=channels)
        return out
    out = path.with_suffix(".mp3")
    out.write_bytes(resp.content)
    wav = path.with_suffix(".wav")
    _mp3_to_wav(out, wav)
    out.unlink(missing_ok=True)
    return wav


def _synth_one(
    engine: str,
    text: str,
    path: Path,
    *,
    voice: str,
    rate: str,
    pitch: str,
) -> Path:
    if engine == "edge":
        return synthesize_edge(text, path, voice=voice, rate=rate, pitch=pitch)
    if engine == "macos":
        return synthesize_macos(text, path, voice=voice if voice else "Samantha")
    if engine == "openrouter":
        return synthesize_openrouter(text, path, voice=voice or "Kore")
    raise ValueError(f"Unknown engine: {engine}")


def process_transcript(
    transcript: dict[str, Any],
    *,
    out_dir: Path,
    engine: str,
    voice: str,
    rate: str,
    pitch: str,
) -> dict[str, Any]:
    scenes = transcript.get("scenes") or []
    voice = voice or transcript.get("voice") or "en-US-JennyNeural"
    rate = rate or transcript.get("rate") or "-20%"
    pitch = pitch or transcript.get("pitch") or "+0Hz"
    transcript["voice"] = voice
    transcript["rate"] = rate
    transcript["pitch"] = pitch
    transcript["engine"] = engine

    for scene in scenes:
        if not isinstance(scene, dict):
            continue
        sid = str(scene.get("id") or "scene")
        sdir = out_dir / "scenes" / sid
        sdir.mkdir(parents=True, exist_ok=True)
        beats = scene.get("beats") if isinstance(scene.get("beats"), list) else []
        spoken_beats = [
            (i, str(b.get("narration") or "").strip())
            for i, b in enumerate(beats)
            if isinstance(b, dict) and str(b.get("narration") or "").strip()
        ]
        full = str(scene.get("full_narration") or "").strip()
        if not full:
            full = " ".join(t for _, t in spoken_beats)
            scene["full_narration"] = full

        if len(spoken_beats) >= 2 and engine in {"edge", "openrouter", "macos"}:
            beat_dir = sdir / "beats"
            beat_dir.mkdir(parents=True, exist_ok=True)

            def _render(item: tuple[int, str]) -> tuple[int, Optional[Path]]:
                idx, text = item
                try:
                    p = _synth_one(
                        engine,
                        text,
                        beat_dir / f"beat_{idx:02d}.wav",
                        voice=voice,
                        rate=rate,
                        pitch=pitch,
                    )
                    return idx, p
                except Exception as exc:  # noqa: BLE001
                    print(f"WARN beat {sid}/{idx}: {exc}", file=sys.stderr)
                    return idx, None

            with ThreadPoolExecutor(max_workers=4) as pool:
                rendered = dict(pool.map(_render, spoken_beats))
            if all(rendered.get(i) for i, _ in spoken_beats):
                ordered = [Path(rendered[i]) for i, _ in spoken_beats]  # type: ignore[arg-type]
                audio_path = sdir / "audio.wav"
                measured = concat_wav_files(ordered, audio_path)
                if measured:
                    per = [0.0] * len(beats)
                    for (idx, _), sec in zip(spoken_beats, measured):
                        per[idx] = sec
                        if isinstance(beats[idx], dict):
                            beats[idx]["audio_duration_seconds"] = sec
                    scene["beats"] = beats
                    scene["audio_path"] = str(audio_path)
                    scene["audio_duration_seconds"] = sum(measured)
                    print(f"{sid}: {scene['audio_duration_seconds']:.2f}s ({len(spoken_beats)} beats)")
                    continue

        # Whole-scene fallback
        if not full:
            print(f"SKIP {sid}: empty narration", file=sys.stderr)
            continue
        audio_path = _synth_one(
            engine, full, sdir / "audio.wav", voice=voice, rate=rate, pitch=pitch
        )
        scene["audio_path"] = str(audio_path)
        scene["audio_duration_seconds"] = wav_duration_seconds(audio_path)
        print(f"{sid}: {scene['audio_duration_seconds']:.2f}s (whole)")

    return transcript


def main(argv: Optional[list[str]] = None) -> int:
    raw_argv = list(argv) if argv is not None else sys.argv[1:]
    normalized_argv: list[str] = []
    i = 0
    while i < len(raw_argv):
        current = raw_argv[i]
        if current in {"--rate", "--pitch"} and i + 1 < len(raw_argv):
            value = raw_argv[i + 1]
            normalized_argv.append(f"{current}={value}")
            i += 2
            continue
        normalized_argv.append(current)
        i += 1

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("transcript", type=Path)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--engine", choices=("edge", "openrouter", "macos"), default="edge")
    parser.add_argument("--voice", default="")
    parser.add_argument("--rate", default="")
    parser.add_argument("--pitch", default="")
    args = parser.parse_args(normalized_argv)

    data = json.loads(args.transcript.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        print("transcript must be a JSON object", file=sys.stderr)
        return 2
    args.out_dir.mkdir(parents=True, exist_ok=True)
    updated = process_transcript(
        data,
        out_dir=args.out_dir,
        engine=args.engine,
        voice=args.voice,
        rate=args.rate,
        pitch=args.pitch,
    )
    out_transcript = args.out_dir / "transcript.json"
    out_transcript.write_text(json.dumps(updated, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    # Keep source in sync if different path
    if args.transcript.resolve() != out_transcript.resolve():
        args.transcript.write_text(json.dumps(updated, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Updated {out_transcript}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
