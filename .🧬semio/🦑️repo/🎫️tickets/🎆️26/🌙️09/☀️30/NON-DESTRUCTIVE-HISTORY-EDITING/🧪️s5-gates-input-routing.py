"""🧭️ Routes the `schema mutation-inputs` report (design §22.8) to the session-5 owner work packages.

Input: the JSON a driver writes from `mutationInputUiReport` (`{diagnostics, census, inputs, multiline}`). Output: one Markdown
table per work package (plugin, failing findings per class with the top-level and nested split of `numericUndeclared`, the leaves
touched and the heaviest one, and — as counts, not findings — the leaf inputs labelled by the glossary) and one summary line per
work package.

    python3 🧪️s5-gates-input-routing.py <report.json> [--lines]
"""
import collections
import json
import re
import sys

OWNERS = {
    "S5-TEXT-STDIO": ["stdio", "writer", "trinity", "vcs"],
    "S5-GRAPHS-WIRES": ["dag", "sequence", "imperative", "space", "reasoning", "mathematical"],
    "S5-FLOWCAD": ["flow", "cad"],
    "S5-STROKES-NORM": ["raster", "remodel", "wfc", "process", "norm"],
    "S5-TOOLS": ["draw", "note", "layout", "fem", "lowpoly", "shooting", "energy", "forms", "gis", "procedural", "playbook", "block"],
    "S5-PUZZLE": ["puzzle"],
}
STALE = {"malformed", "leafUncatalogued"}
INFERENCE = {"numericUndeclared"}
DETAIL = re.compile(r'(\w+) at "((?:[^"\\]|\\.)*)": ', re.S)


def plugin_of(path):
    """🏷️ The plugin (or framework module) a repository path belongs to, its taxonomy emoji stripped."""
    parts = path.split("/")
    name = parts[2] if parts[0] == "✏️s" and len(parts) > 2 else next((part for part in parts if part.endswith(("flow", "workflow", "infinite", "store"))), parts[-1])
    return re.sub(r"^[^A-Za-z0-9]+", "", name)


def owner_of(plugin):
    """🧑‍🔧 The work package that owns a plugin, else `coordinator`."""
    return next((owner for owner, plugins in OWNERS.items() if plugin in plugins), "coordinator")


def main():
    report = json.load(open(sys.argv[1], encoding="utf-8"))
    lines_only = "--lines" in sys.argv
    table = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
    leaves = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
    glossed = collections.Counter()
    for row in report.get("inputs", []):
        if "inferred" in (row["label"] or {}).values():
            glossed[plugin_of(row["path"])] += 1
    for entry in report["diagnostics"]:
        match = DETAIL.match(entry["detail"])
        code, pointer = match.group(1), match.group(2)
        plugin = plugin_of(entry["path"])
        owner = owner_of(plugin)
        table[owner][plugin][code] += 1
        if code in INFERENCE:
            table[owner][plugin][code + (":top" if pointer.count("/") == 1 else ":nested")] += 1
            leaves[owner][plugin][entry["path"]] += 1
    order = list(OWNERS) + ["coordinator"]
    for owner in order:
        plugins = table.get(owner, {})
        real = sum(count for counts in plugins.values() for code, count in counts.items() if ":" not in code and code not in STALE)
        stale = sum(count for counts in plugins.values() for code, count in counts.items() if code in STALE)
        totals = collections.Counter()
        for counts in plugins.values():
            for code, count in counts.items():
                if ":" not in code and code not in STALE:
                    totals[code] += count
        top = totals.most_common(1)
        counted = sum(glossed[plugin] for plugin in glossed if owner_of(plugin) == owner)
        print(f"{owner}: {real} real finding(s) (+{stale} catalogue staleness) — top rule {top[0][0]} {top[0][1]}; {counted} glossary label(s) counted" if top else f"{owner}: 0 real finding(s) (+{stale} catalogue staleness); {counted} glossary label(s) counted")
        if lines_only:
            continue
        print()
        print("| Plugin | Findings | numericUndeclared (top / nested) | Reader classes | Leaves | Heaviest leaf | Glossary labels (count, no finding) |")
        print("| --- | ---: | ---: | --- | ---: | --- | ---: |")
        for plugin, counts in sorted(plugins.items(), key=lambda item: -sum(count for code, count in item[1].items() if ":" not in code)):
            reader = ", ".join(f"{code} {count}" for code, count in sorted(counts.items()) if ":" not in code and code not in INFERENCE)
            heavy = leaves[owner][plugin].most_common(1)
            heaviest = "" if not heavy else f"`{heavy[0][0].split('/🧬️mutations/')[-1].replace('/🧬️schema/🔣️.json', '')}` in `{heavy[0][0].split('/🗿️artifacts/')[-1].split('/')[0]}` ({heavy[0][1]})"
            plain = sum(count for code, count in counts.items() if ":" not in code and code not in STALE)
            print(f"| {plugin} | {plain} | {counts['numericUndeclared']} ({counts['numericUndeclared:top']} / {counts['numericUndeclared:nested']}) | {reader} | {len(leaves[owner][plugin])} | {heaviest} | {glossed[plugin]} |")
        print()


main()
