#!/usr/bin/env python3
"""🖼️ R10 prepared window-3 patch (guest-linked, frozen during the chain): the surface crate's public error enums stop
carrying the external `image::ImageError` (AGENTS.md: no exported API may require a type from outside this codebase).
`FrameworkSurfacePaintError::Image` and `FrameworkSurfaceTiledMapError::Image` carry the decoder's message instead; the
`From<image::ImageError>` conversions stay private-in-effect (they only feed `?`), `Display` prints the message and
`source` ends there. Idempotent; exact-once anchors; `--apply` writes, default is a dry run.
Checks after apply: native `cargo check -p semio-framework-surface --lib --tests` + its unit tests (`Image(_)` still matches).
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
FILES = {
    "🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs": "FrameworkSurfacePaintError",
    "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs": "FrameworkSurfaceTiledMapError",
}


def edits(enum: str) -> list[tuple[str, str]]:
    return [
        ("    Image(image::ImageError),\n", "    Image(String),\n"),
        ("            Self::Image(error) => error.fmt(formatter),\n", "            Self::Image(message) => formatter.write_str(message),\n"),
        ("            Self::Image(error) => Some(error),\n", ""),
        (f"impl From<image::ImageError> for {enum} {{\n    fn from(error: image::ImageError) -> Self {{\n        Self::Image(error)\n    }}\n}}\n",
         f"impl From<image::ImageError> for {enum} {{\n    fn from(error: image::ImageError) -> Self {{\n        Self::Image(error.to_string())\n    }}\n}}\n"),
    ]


def main() -> int:
    apply = "--apply" in sys.argv
    planned: dict[Path, str] = {}
    for relative, enum in FILES.items():
        path = ROOT / relative
        text = path.read_text(encoding="utf-8")
        for before, after in edits(enum):
            if text.count(before) == 1:
                text = text.replace(before, after)
            elif after and text.count(after) >= 1 and before not in text:
                continue
            elif not after and before not in text:
                continue
            else:
                print(f"REFUSED {relative}: anchor found {text.count(before)}x: {before.strip()[:60]}")
                return 1
        if enum == "FrameworkSurfaceTiledMapError":
            if "            Self::Image(_) => None,\n" not in text:
                text = text.replace("            Self::Mvt(error) => Some(error),\n        }", "            Self::Mvt(error) => Some(error),\n            Self::Image(_) => None,\n        }", 1)
        else:
            text = text.replace("            Self::UnsupportedSchema(_) | Self::Composite(_) => None,\n", "            Self::Image(_) | Self::UnsupportedSchema(_) | Self::Composite(_) => None,\n", 1)
        planned[path] = text
    for path, text in planned.items():
        changed = text != path.read_text(encoding="utf-8")
        print(f"{'write' if apply and changed else 'would write' if changed else 'unchanged'} {path.relative_to(ROOT)}")
        if apply and changed:
            path.write_text(text, encoding="utf-8")
    print("applied" if apply else "dry run clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
