"""🧱 Write standalone entry scripts and series playlists for the Grasshopper and Excel clips."""

from __future__ import annotations

from series import ARCH, SERIES

ENTRY = '''"""🎬 Render the standalone {label} {topic} video through shared curriculum interfaces."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_ID = "{video}"


def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    """📚 Render this standalone topic from its canonical shared scene source."""
    return _runtime.compose_videos([VIDEO_ID], language=language, chapter_scenes=chapters, **settings)


def main(argv: list[str] | None = None) -> int:
    """▶️ Reuse explicit language, progress, cancellation and presentation settings."""
    return _runtime.video_main(VIDEO_ID, argv)


if __name__ == "__main__":
    raise SystemExit(main())
'''

PLAYLIST = '''"""🎬 Render the {count} independent {label} videos through their shared source playlist."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

sys.dont_write_bytecode = True
_WORKSPACE_ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_IDS = ({ids})


def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    """📚 Compose {count} separate outputs from the canonical topic source scenes."""
    return _runtime.compose_videos(list(VIDEO_IDS), language=language, chapter_scenes=chapters, **settings)


def main(argv: list[str] | None = None) -> int:
    """▶️ Select all {count} topic videos with explicit language and shared render controls."""
    renderer = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🖌️render.🐍️")
    selection = [argument for video in VIDEO_IDS for argument in ("--video", video)]
    return renderer.main([*selection, *(sys.argv[1:] if argv is None else argv)])


if __name__ == "__main__":
    raise SystemExit(main())
'''

COUNTS = {5: "five", 4: "four"}

for series in SERIES.values():
    base = ARCH / series["folder"]
    for video, _scene, folder, _chapter, _output, _icon, title in series["videos"]:
        directory = base / folder / "🎬️clip"
        directory.mkdir(parents=True, exist_ok=True)
        (directory / "🐍️.py").write_text(ENTRY.format(label=series["label"], topic=video.split("_", 1)[1], video=video), encoding="utf-8", newline="\n")
    ids = ", ".join(f'"{video[0]}"' for video in series["videos"])
    count = COUNTS[len(series["videos"])]
    (base / "🎬️clip" / "🐍️.py").write_text(PLAYLIST.format(label=series["label"], count=count, ids=ids), encoding="utf-8", newline="\n")
    print(f"[DEBUG] wrote {series['label']}")
