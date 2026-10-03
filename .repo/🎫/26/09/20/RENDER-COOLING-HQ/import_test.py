"""🧪 Prove venv Manim imports, then start the HQ cooling compose."""

from __future__ import annotations

import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
LOG = TICKET / "import_test.log"


def _log(msg: str) -> None:
    with LOG.open("a", encoding="utf-8") as fh:
        fh.write(msg + "\n")
        fh.flush()
    print(msg, flush=True)


def main() -> int:
    LOG.write_text("start\n", encoding="utf-8")
    _log(f"python {sys.version}")
    import manim

    _log(f"manim {getattr(manim, '__version__', '?')}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
