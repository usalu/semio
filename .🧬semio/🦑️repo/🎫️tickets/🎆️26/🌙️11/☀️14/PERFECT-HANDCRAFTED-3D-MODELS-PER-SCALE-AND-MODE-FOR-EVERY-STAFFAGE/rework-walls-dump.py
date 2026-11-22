"""🧾 Compact listing of a volumes source: one line per solid with its kind, bone, flags and placement numbers."""
import json, sys


def short(value, limit):
    text = json.dumps(value, ensure_ascii=False)
    return text if len(text) <= limit else text[: limit - 1] + "…"


def main():
    source = json.load(open(sys.argv[1]))
    volumes = source.get("volumes", source)
    scales = sys.argv[2].split(",") if len(sys.argv) > 2 else list(volumes)
    lists = sys.argv[3].split(",") if len(sys.argv) > 3 else ["body", "thermal", "structure"]
    limit = int(sys.argv[4]) if len(sys.argv) > 4 else 60
    for scale in scales:
        for name in lists:
            solids = volumes[scale][name]
            print(f"== 1:{scale} {name} ({len(solids)})")
            for index, solid in enumerate(solids):
                head = [solid.get("id"), solid.get("kind"), solid.get("bone"), solid.get("material"), solid.get("role") or solid.get("part") or solid.get("region")]
                rest = {key: value for key, value in solid.items() if key not in ("id", "kind", "bone", "material", "role", "tone")}
                print(f"  {index} " + " ".join(str(item) for item in head) + " " + " ".join(f"{key}={short(value, limit)}" for key, value in rest.items()))
        print("  decisions:", short(volumes[scale].get("decisions"), 3000))


if __name__ == "__main__":
    main()
