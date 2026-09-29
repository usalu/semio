"""🔎 FH5 (session 15): per-code review digest for the class work lists (family A + H): entry, en/de, and every raise
site's surrounding source lines read from the live tree (census line → nearest occurrence of the quoted code).
Usage: python3 fh5-digest.py <A|H> [max_sites]"""
from __future__ import annotations

import collections
import json
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
OVERLAY = REPO / ".🧬semio/🌐hub/s14-s20-overlay-faults"
CENSUS = OVERLAY / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance/🧯️fault-census.json"
CATALOG = OVERLAY / "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json"
SETS = REPO / ".🧬semio/🌐hub/s14-s20-sets/class"
OUT = REPO / ".tmp-ticket/wp-fh5/🗑️generated"
CACHE: dict[str, list[str]] = {}


def lines_of(root: Path, path: str) -> list[str]:
    key = f"{root}/{path}"
    if key not in CACHE:
        file = root / path
        CACHE[key] = file.read_text(errors="replace").splitlines() if file.exists() else []
    return CACHE[key]


def site(raise_: dict) -> str:
    needle = f'"{raise_["code"]}"'
    tree = "live"
    for root, tree in ((REPO, "live"), (OVERLAY, "overlay")):
        lines = lines_of(root, raise_["path"])
        hits = [index for index, line in enumerate(lines) if needle in line]
        if hits:
            break
    else:
        lines, tree = lines_of(OVERLAY, raise_["path"]), "overlay@census"
        hits = [raise_["line"] - 1] if len(lines) >= raise_["line"] else []
    if not hits:
        return f"  @ {raise_['path']}:{raise_['line']} (file not found)"
    at = min(hits, key=lambda index: abs(index + 1 - raise_["line"]))
    body = "\n".join(f"    {index + 1}: {lines[index].strip()[:220]}" for index in range(max(0, at - 4), min(len(lines), at + 3)))
    short = "/".join(part for part in raise_["path"].split("/") if not part.startswith(("🏅️", "🔖️", "🪆️", "✳️")))
    return f"  @ [{tree}] {short}:{at + 1}\n{body}"


def main() -> None:
    family = sys.argv[1]
    limit = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    census = json.loads(CENSUS.read_text())
    raises = collections.defaultdict(list)
    for raise_ in census["raises"]:
        raises[raise_["code"]].append(raise_)
    de = {entry["code"]: entry.get("de", "") for entry in json.loads(CATALOG.read_text())["faults"]}
    for declaration in census["declarations"]:
        de.setdefault((declaration["path"].split("/")[2], declaration["code"]), declaration.get("de", ""))
    entries = json.loads((SETS / f"family-{family}.json").read_text())
    out = []
    for number, entry in enumerate(entries):
        owner = entry["owner"]
        sites = raises[entry["code"]]
        if owner != "framework":
            own = [raise_ for raise_ in sites if f"/{owner}/" in raise_["path"]]
            sites = own or sites
        seen, chosen = set(), []
        for raise_ in sites:
            key = raise_["path"]
            if key in seen and len(sites) > limit:
                continue
            seen.add(key)
            chosen.append(raise_)
        out.append(f"#{number} [{owner}] {entry['code']} :: {entry['class']} ({entry['rule']}) sites={len(sites)}\n  en: {entry['en']}")
        out.extend(site(raise_) for raise_ in chosen[:limit])
        out.append("")
    (OUT / f"digest-{family}.txt").write_text("\n".join(out))
    print(f"family {family}: {len(entries)} entries → {OUT / f'digest-{family}.txt'}")


if __name__ == "__main__":
    main()
