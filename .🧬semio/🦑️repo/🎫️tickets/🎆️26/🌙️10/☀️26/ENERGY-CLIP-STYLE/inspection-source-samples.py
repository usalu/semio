"""🔍 Seek a small midpoint window from one certified current source for visual inspection."""
import argparse
import json
from pathlib import Path

import av

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated/iv"


def main():
    """📸 Decode only from the preceding keyframe through the requested small sample window."""
    parser = argparse.ArgumentParser()
    parser.add_argument("ordinal")
    parser.add_argument("--offsets", type=float, nargs="+", default=[-2, 0, 2])
    parser.add_argument("--label", default="")
    args = parser.parse_args()
    root = TICKET / "🗑️generated/i/de/spectator/s" / args.ordinal / "v"
    paths = [path.with_suffix(".mp4").with_name(path.name.removesuffix(".render.json") + ".mp4") for path in root.glob("*.render.json")]
    assert len(paths) == 1, paths
    path = paths[0]
    rows = []
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        duration = float(stream.duration * stream.time_base)
        targets = sorted(max(0, min(duration - 0.25, duration / 2 + offset)) for offset in args.offsets)
        media.seek(int(targets[0] / stream.time_base), stream=stream, backward=True)
        index = 0
        for frame in media.decode(stream):
            stamp = float(frame.pts * frame.time_base)
            if stamp + 1e-8 < targets[index]:
                continue
            label = args.label + "-" if args.label else ""
            output = OUTPUT / f"source-{args.ordinal}-{label}{index + 1}.png"
            frame.to_image().save(output)
            rows.append({"path": str(output), "target_seconds": targets[index], "actual_seconds": stamp})
            index += 1
            if index == len(targets):
                break
    assert len(rows) == len(targets)
    with (TICKET / "inspection-source-samples.md").open("a", encoding="utf-8") as report:
        report.write(f"\n## Source {args.ordinal}\n\nSelected midpoint offsets {args.offsets} of {path.name}; small-window seek only.\n\n```json\n" + json.dumps(rows, indent=2) + "\n```\n")
    print("[DEBUG] Sampled source", args.ordinal, json.dumps(rows), flush=True)


if __name__ == "__main__":
    main()
