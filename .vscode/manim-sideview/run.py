#!/usr/bin/env python3
"""🎬 Manim Sideview entry that injects teaching env and ASCII-local media, then execs manim.

Manim Sideview has no env-var setting; teaching scenes require ``SEMIO_TEACHING_LANGUAGE``.
Its webview ``localResourceRoots`` only includes the scene file's folder, so media must
land under that folder (not the emoji teaching cache) or the player stays empty at 0:00.

@see ../../🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/📁️output/🐍️.py — ``output_root()``
"""

from __future__ import annotations

import os
import shutil
import sys
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_LAUNCHERS = (
    Path.home() / ".local" / "share" / "semio" / "launcher" / "bin",
    Path.home() / "AppData" / "Local" / "semio" / "launcher" / "bin",
)


def _manim() -> Path:
    for candidate in (
        _ROOT / ".venv" / "bin" / "manim",
        _ROOT / ".venv" / "Scripts" / "manim.exe",
        _ROOT / ".venv" / "Scripts" / "manim",
    ):
        if candidate.is_file():
            return candidate
    found = shutil.which("manim")
    if not found:
        raise SystemExit("manim not found in .venv or PATH")
    return Path(found)


def _scene_directory(argv: list[str]) -> Path | None:
    for arg in argv:
        if arg.startswith("-") or not arg.endswith(".py"):
            continue
        path = Path(arg).expanduser()
        if not path.is_absolute():
            path = (Path.cwd() / path).resolve()
        else:
            path = path.resolve()
        if path.suffix == ".py":
            return path.parent
    return None


def main() -> None:
    os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
    os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
    os.environ.setdefault("SEMIO_TEACHING_PROFILE", "preview")
    scene_dir = _scene_directory(sys.argv[1:])
    if scene_dir is not None:
        # Force under the scene folder so Sideview's webview can load the mp4.
        os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(scene_dir / ".sideview")
    path_parts = [str(part) for part in _LAUNCHERS if part.is_dir()]
    if path_parts:
        os.environ["PATH"] = os.pathsep.join([*path_parts, os.environ.get("PATH", "")])
    manim = _manim()
    os.execv(str(manim), [str(manim), *sys.argv[1:]])


if __name__ == "__main__":
    main()
