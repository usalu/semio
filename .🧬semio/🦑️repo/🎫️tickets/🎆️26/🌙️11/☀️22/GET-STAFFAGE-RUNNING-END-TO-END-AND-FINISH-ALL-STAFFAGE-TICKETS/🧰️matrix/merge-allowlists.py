"""🧷 Merges the fixers' allowlist entries (`<BATCH>/allowlist.json`) into the multiview coverage fixture, sorted and without duplicates; idempotent.

Usage: `python3 merge-allowlists.py [--write]`. An entry listed under `withdraw` in any `<BATCH>/allowlist.json` (`{ "list", "asset", "view", "bands": null | [...] }`) is removed from the fixture and is not added again from any file (an entry is replaced by a narrower one this way). Entries named in `REFUSED` are left out with the reason beside them; an entry named in `BAND_LIMITED` is merged only when it names exactly those bands (D51) and is left out otherwise. See `../multiview-shared.md` section 4.
"""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = HERE.parents[7] / "🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🔭️multiview-coverage/🔣️.json"
LISTS = ("derivedViewAllowlist", "bandCopyAllowlist")
REFUSED = {}
BAND_LIMITED = {("derivedViewAllowlist", "steel-column", "front"): (("200", "500"), "band-limited (D51): the entry holds for 1:200 and 1:500 only; 1:50 and 1:100 pass by their drawing (the gusset is drawn, 90 mm above the plate), so an entry that names no bands or other bands is left out")}


def key(name: str, entry: dict) -> tuple:
    """🔑 Identity of an entry: list, asset, view and its sorted bands."""
    return (name, entry["asset"], entry["view"], tuple(sorted(entry.get("bands", []), key=int)))


def main(write: bool) -> None:
    """✍️ Adds every new entry and prints the totals."""
    fixture = json.loads(FIXTURE.read_text("utf8"))
    known = {key(name, entry) for name in LISTS for entry in fixture[name]}
    added = 0
    withdrawn = set()
    for file in sorted(HERE.glob("*/allowlist.json")):
        listed = json.loads(file.read_text("utf8"))
        for entry in listed.get("withdraw", []) if isinstance(listed, dict) else []:
            withdrawn.add((entry["list"], entry["asset"], entry["view"], tuple(sorted(entry.get("bands") or [], key=int))))
    for name in LISTS:
        kept = [entry for entry in fixture[name] if key(name, entry) not in withdrawn]
        if len(kept) != len(fixture[name]):
            print(f"withdrawn {len(fixture[name]) - len(kept)} from {name}")
            added += len(fixture[name]) - len(kept)
        fixture[name] = kept
    known = {key(name, entry) for name in LISTS for entry in fixture[name]}
    for file in sorted(HERE.glob("*/allowlist.json")):
        data = json.loads(file.read_text("utf8"))
        data = {"derivedViewAllowlist": data} if isinstance(data, list) else data
        for name in LISTS:
            for entry in data.get(name, []):
                limit = BAND_LIMITED.get((name, entry["asset"], entry["view"]))
                if limit is not None and tuple(sorted(entry.get("bands", []), key=int)) != limit[0]:
                    print(f"left out {file.parent.name}: {entry['asset']} {entry['view']} {entry.get('bands', 'every band')}: {limit[1]}")
                    continue
                if (name, entry["asset"], entry["view"]) in REFUSED or key(name, entry) in known or key(name, entry) in withdrawn:
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
