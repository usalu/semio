"""🔬️ The shapes of one view of an asset at given scales, one truncated JSON line each: `python3 view.py <asset.json> <view> <scale,scale> [width]`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
w = int(sys.argv[4]) if len(sys.argv) > 4 else 300
for scale in sys.argv[3].split(","):
    shapes = a["representations"][scale]["geometry"][sys.argv[2]]
    print(f"== 1:{scale} {sys.argv[2]} ({len(shapes)})")
    for s in shapes:
        s = {k: v for k, v in s.items() if k not in ("material", "tone", "region", "regions")}
        print("  ", json.dumps(s, ensure_ascii=False)[:w])
