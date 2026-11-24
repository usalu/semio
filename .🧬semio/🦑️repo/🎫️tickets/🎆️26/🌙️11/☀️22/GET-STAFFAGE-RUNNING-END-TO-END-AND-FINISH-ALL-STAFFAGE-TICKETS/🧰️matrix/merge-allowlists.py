"""🧷 Merges the fixers' allowlist entries (`<BATCH>/allowlist.json`) into the multiview coverage fixture, sorted and without duplicates; idempotent.

Usage: `python3 merge-allowlists.py [--write]`. Entries named in `REFUSED` are left out with the reason beside them. See `../multiview-shared.md` section 4.
"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = HERE.parents[7] / "🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🔭️multiview-coverage/🔣️.json"
LISTS = ("derivedViewAllowlist", "bandCopyAllowlist")
REFUSED = {("derivedViewAllowlist", "steel-column", "front"): "the fixer's own measurement says the 1:100 front is not the same drawing as the side"}


def key(name: str, entry: dict) -> tuple:
    """🔑 Identity of an entry: list, asset, view and its sorted bands."""
    return (name, entry["asset"], entry["view"], tuple(sorted(entry.get("bands", []), key=int)))


def main(write: bool) -> None:
    """✍️ Adds every new entry and prints the totals."""
    fixture = json.loads(FIXTURE.read_text("utf8"))
    known = {key(name, entry) for name in LISTS for entry in fixture[name]}
    added = 0
    for file in sorted(HERE.glob("*/allowlist.json")):
        data = json.loads(file.read_text("utf8"))
        data = {"derivedViewAllowlist": data} if isinstance(data, list) else data
        for name in LISTS:
            for entry in data.get(name, []):
                if (name, entry["asset"], entry["view"]) in REFUSED or key(name, entry) in known:
                    continue
                known.add(key(name, entry))
                fixture[name].append({k: entry[k] for k in ("asset", "view", "bands", "reason") if k in entry})
                added += 1
    for name in LISTS:
        fixture[name].sort(key=lambda entry: key(name, entry))
    print(f"added {added}; " + ", ".join(f"{name} {len(fixture[name])}" for name in LISTS))
    if write and added:
        FIXTURE.write_text(spliced(FIXTURE.read_text("utf8"), fixture), "utf8")


def spliced(text: str, fixture: dict) -> str:
    """🪡 Replaces only the two lists in the handwritten fixture text, one entry per line."""
    lines = text.split("\n")
    for name in LISTS:
        start = next(i for i, line in enumerate(lines) if line.startswith(f'  "{name}": ['))
        end = start if lines[start].rstrip().endswith("],") else next(i for i in range(start, len(lines)) if lines[i] == "  ],")
        rows = [f"    {json.dumps(entry, ensure_ascii=False)}" for entry in fixture[name]]
        lines[start : end + 1] = [f'  "{name}": [', ",\n".join(rows), "  ],"]
    return "\n".join(lines)

if __name__ == "__main__":
    main("--write" in sys.argv)
