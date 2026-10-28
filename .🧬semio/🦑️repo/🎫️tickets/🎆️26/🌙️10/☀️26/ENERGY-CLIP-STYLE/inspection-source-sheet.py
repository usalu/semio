"""🖼️ Preview completed canonical source midpoints while the catalog render continues."""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
from pathlib import Path

import av
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("inspection_verify", TICKET / "inspection-verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


def main():
    """📸 Decode a representative real frame from each atomically completed source."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--only", type=int, nargs="+")
    parser.add_argument("--name", default="source-progress")
    args = parser.parse_args()
    assert args.name.replace("-", "").isalnum()
    videos = json.loads(VERIFY.MANIFEST.read_text(encoding="utf-8"))["videos"]
    plan = VERIFY.plan(videos, TICKET / "🗑️generated/i/de/spectator")
    shared = {str((VERIFY.ROOT / path).resolve()).replace("\\", "/"): VERIFY.sha(VERIFY.ROOT / path)
              for path in ("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py",
                           "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py")}
    sources = {}
    for _, entries in plan.values():
        for entry in entries:
            path = entry["source"]
            if args.only and int(path.parents[1].name) not in args.only:
                continue
            certificate = path.with_suffix(".render.json")
            if certificate.is_file():
                state = json.loads(certificate.read_text(encoding="utf-8"))["sources"]
                code = {owner: digest for owner, digest in state.items() if owner.endswith(".py")}
                for owner in code:
                    if owner not in shared:
                        shared[owner] = VERIFY.sha(Path(owner))
                if all(shared[owner] == digest for owner, digest in code.items()):
                    sources.setdefault(path, entry["segment"]["scene"])
    assert sources
    image = Image.new("RGB", (1440, 296 * math.ceil(len(sources) / 3)), "#222222")
    draw = ImageDraw.Draw(image)
    labels = []
    for ordinal, (path, scene) in enumerate(sources.items()):
        with av.open(str(path)) as media:
            stream = media.streams.video[0]
            stream.codec_context.thread_count = 1
            target = float(stream.duration * stream.time_base) / 2
            media.seek(int(target / stream.time_base), stream=stream, backward=True)
            frame = next(frame for frame in media.decode(stream) if float(frame.pts * frame.time_base) >= target)
            decoded = frame.to_image()
            assert decoded.size == (480, 270)
        x, y = ordinal % 3 * 480, ordinal // 3 * 296
        image.paste(decoded, (x, y))
        label = f"source {path.parents[1].name}: {scene}"
        draw.text((x + 5, y + 275), label, fill="white")
        labels.append(label)
    output = TICKET / f"🗑️generated/iv/{args.name}.png"
    image.save(output)
    (TICKET / f"inspection-{args.name}.md").write_text("# Inspection Source Progress\n\nThis preliminary sheet contains real midpoint frames from " + str(len(sources)) + " completed canonical sources whose certificates match all current Python dependency hashes, including shared typography/visuals and their authored owner. It supports early body-content review; final envelope, all-frame ordering, chapter and clause coverage, audio and shared-root acceptance remain in inspection-verification.md.\n\n" + "\n".join("- " + label for label in labels) + "\n", encoding="utf-8")
    print("[DEBUG] Source preview sheet", len(sources), str(output), flush=True)


if __name__ == "__main__":
    main()
