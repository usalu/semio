#!/usr/bin/env python3
"""🔗 Install ``manim-teaching`` next to the workspace venv manim for Manim Sideview.

@see ./run.py — the launcher Sideview should resolve as ``manim-teaching``
"""

from __future__ import annotations

from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_LAUNCHER = Path(__file__).resolve().parent / "run.py"


def _targets() -> list[Path]:
    unix = _ROOT / ".venv" / "bin" / "manim-teaching"
    win = _ROOT / ".venv" / "Scripts" / "manim-teaching.cmd"
    chosen: list[Path] = []
    if (_ROOT / ".venv" / "bin").is_dir():
        chosen.append(unix)
    if (_ROOT / ".venv" / "Scripts").is_dir():
        chosen.append(win)
    if not chosen:
        raise SystemExit(f"No .venv found under {_ROOT}")
    return chosen


def _unix_body() -> str:
    return (
        "#!/usr/bin/env bash\n"
        "set -euo pipefail\n"
        f'exec python3 "{_LAUNCHER}" "$@"\n'
    )


def _win_body() -> str:
    return (
        "@echo off\r\n"
        f'python "{_LAUNCHER}" %*\r\n'
        "exit /b %ERRORLEVEL%\r\n"
    )


def main() -> None:
    for target in _targets():
        target.parent.mkdir(parents=True, exist_ok=True)
        body = _win_body() if target.suffix.lower() == ".cmd" else _unix_body()
        target.write_bytes(body.encode("utf-8"))
        if target.suffix.lower() != ".cmd":
            target.chmod(target.stat().st_mode | 0o111)
        print(f"installed {target}")


if __name__ == "__main__":
    main()
