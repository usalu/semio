"""🔦 Compact listing of a volumes source: python3 rework-fittings-dump.py <category/slug> <scale> [list] [id substring]."""
import json
import sys

from rework_fittings_lib import load

BULK = ("profile", "holes", "path", "sections", "points")


def brief(solid):
    """📝 One line per solid: bulky members as counts and bounding ranges."""
    row = dict(solid)
    for key in BULK:
        if key in row and isinstance(row[key], list) and len(json.dumps(row[key])) > 90:
            flat = row[key]
            if key == "sections":
                row[key] = "<%d sections at %s .. %s>" % (len(flat), flat[0].get("at"), flat[-1].get("at"))
            elif flat and isinstance(flat[0], list) and flat[0] and not isinstance(flat[0][0], list):
                lo = [min(p[i] for p in flat) for i in range(len(flat[0]))]
                hi = [max(p[i] for p in flat) for i in range(len(flat[0]))]
                row[key] = "<%d pts %s..%s>" % (len(flat), lo, hi)
            else:
                row[key] = "<%d>" % len(flat)
    for key in ("role", "tone", "material"):
        row.pop(key, None)
    return json.dumps(row, ensure_ascii=False)


def main():
    """▶️ Prints the chosen lists."""
    asset, scale = sys.argv[1], sys.argv[2]
    lists = [sys.argv[3]] if len(sys.argv) > 3 and sys.argv[3] != "all" else ["body", "thermal", "structure"]
    needle = sys.argv[4] if len(sys.argv) > 4 else ""
    volumes = load(asset)[0]["volumes"][scale]
    for name in lists:
        print("==", scale, name, len(volumes[name]))
        for index, solid in enumerate(volumes[name]):
            if needle in solid["id"]:
                print(index, brief(solid))
    if not needle:
        for decision in volumes["decisions"]:
            print("D:", decision["en"])


if __name__ == "__main__":
    main()
