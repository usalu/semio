#!/usr/bin/env python3
"""🧮️ Checks a crate partition of one `#[path]`-tree crate against the global native Source admission ceiling.

Assigns every file of the crate (lib and test closure, embedded inputs following the file that embeds them) to a group
by the first matching path regex, measures each group's lib and test unit with the canonical closure metrics
(`📏️source-ceiling.ts --files`), and reports per-group totals against the budget, the group dependency graph (census
`crate::`/`super::` references), its cycles, and every inherent or trait `impl` whose type lives in another group.
Writes a `✂️crate-cut.py` layers file (`order`, `allow`, `rules`) when the partition is clean.

usage: 🧮️crate-plan.py <config.json> [--write-layers <layers.json>]
config: {"artifact": dir, "root": crate entry relative to artifact, "budget": {"lines", "bytes", "files"},
         "order": [group...], "rules": [[regex on artifact-relative path, group]...]}
"""
import collections, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
cut = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CUT, "exec"), cut)
Crate = cut["Crate"]
census = {}
CENSUS = os.path.join(HERE, "..", "e3-format-split", "🕸️module-graph.py")
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
census["strip"] = cut["strip"]
references = census["references"]
INCLUDE = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*"([^"]+)"\s*\)')
TYPE_DEF = re.compile(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|union|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)")
IMPL = re.compile(r"(?m)^\s*(?:unsafe\s+)?impl\b(?:\s*<(?:[^<>]|<[^<>]*>)*>)?\s+([^{]*?)\s*(?:where\b[^{]*)?\{")


def metrics(entry):
    out = subprocess.run(["bun", os.path.join(HERE, "📏️source-ceiling.ts"), "--files", entry], capture_output=True, text=True, cwd=REPO)
    if out.returncode:
        raise SystemExit(out.stderr)
    data = json.loads(out.stdout)
    return {unit: {os.path.normpath(os.path.join(REPO, f["path"])): f for f in files} for unit, files in data.items()}


def main():
    config = json.load(open(sys.argv[1], encoding="utf-8"))
    artifact = os.path.join(REPO, config["artifact"])
    entry = os.path.normpath(os.path.join(artifact, config["root"]))
    rules = [(re.compile(r), g) for r, g in config["rules"]]
    measured = metrics(entry)
    crate = Crate(entry)
    sources = {p for unit in measured.values() for p, f in unit.items() if f["kind"] == "source"}
    owner = {}
    for path in sources:
        raw = open(path, encoding="utf-8").read()
        for m in INCLUDE.finditer(raw):
            owner.setdefault(os.path.normpath(os.path.join(os.path.dirname(path), m.group(1))), path)

    def group(path):
        base = path if path in sources else owner.get(path, path)
        rel = os.path.relpath(base, artifact)
        for rule, name in rules:
            if rule.search(rel):
                return name
        return "?"

    budget = config["budget"]
    totals = {unit: collections.defaultdict(lambda: [0, 0, 0]) for unit in measured}
    for unit, files in measured.items():
        for path, f in files.items():
            t = totals[unit][group(path)]
            t[0] += f["activeLines"]
            t[1] += f["bytes"]
            t[2] += 1
    order = config.get("order") or sorted({g for unit in totals.values() for g in unit})
    ok = True
    print("| group | lib lines | lib bytes | lib files | test lines | test bytes | test files |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for g in order + sorted(set(totals["test"]) - set(order)):
        lib, test = totals["lib"].get(g, [0, 0, 0]), totals["test"].get(g, [0, 0, 0])
        over = [v > budget[k] for unit in (lib, test) for v, k in zip(unit, ("lines", "bytes", "files"))]
        ok &= not any(over)
        mark = lambda unit, i, k: f"**{unit[i]}**" if unit[i] > budget[k] else str(unit[i])
        print(f"| {g} | {mark(lib, 0, 'lines')} | {mark(lib, 1, 'bytes')} | {mark(lib, 2, 'files')} | {mark(test, 0, 'lines')} | {mark(test, 1, 'bytes')} | {mark(test, 2, 'files')} |")
    unassigned = sorted(os.path.relpath(p, artifact) for unit in measured.values() for p in unit if group(p) == "?")
    for path in sorted(set(unassigned)):
        print(f"UNASSIGNED {path}")
    ok &= not unassigned
    cut["layer_of"] = group

    def references(crate, path):
        text = crate.text[path]
        spans = cut["inline_spans"](text)
        found = set()
        targets = []
        for m in cut["USE"].finditer(text):
            for segs, _alias in cut["leaves"]([], m.group("body")):
                targets.append(([s for s in segs if s != "*"], m.start("body")))
        for m in cut["PATH"].finditer(text):
            targets.append(([x for x in re.split(cut["SEP"], m.group("path")) if x], m.start()))
        for segs, offset in targets:
            try:
                absolute = cut["absolute"](segs, cut["base_at"](crate, path, spans, offset))
            except IndexError:
                continue
            found.add(cut["module_of"](crate, absolute))
        found.discard(path)
        return found

    graph = collections.defaultdict(collections.Counter)
    for path in crate.files:
        if path not in sources:
            continue
        a = group(path)
        for target in references(crate, path):
            b = group(target)
            if a != b:
                graph[a][(b, os.path.relpath(path, artifact), os.path.relpath(target, artifact))] += 1
    defined = collections.defaultdict(set)
    for path in sources:
        for name in TYPE_DEF.findall(cut["strip"](open(path, encoding="utf-8").read())):
            defined[name].add(path)
    incoherent = []
    for path in sources:
        text = cut["strip"](open(path, encoding="utf-8").read())
        for m in IMPL.finditer(text):
            head = m.group(1)
            parts = re.split(r"\s+for\s+", head)
            ty = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", re.sub(r"<.*", "", parts[-1].replace("&", "").replace("dyn ", "")))
            trait = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", re.sub(r"<.*", "", parts[0])) if len(parts) == 2 else []
            homes = defined.get(ty[-1], set()) if ty else set()
            trait_homes = defined.get(trait[-1], set()) if trait else set()
            if len(homes) == 1 and group(next(iter(homes))) != group(path) and not any(group(h) == group(path) for h in trait_homes):
                incoherent.append((os.path.relpath(path, artifact), head.strip()[:80], os.path.relpath(next(iter(homes)), artifact)))
    print("\n## group edges")
    for a in sorted(graph):
        targets = sorted({b for (b, _, _) in graph[a]})
        print(f"{a} -> {targets}")
    index = {g: i for i, g in enumerate(order)}
    upward = [(a, b, s, t) for a in graph for (b, s, t) in graph[a] if index.get(b, -1) >= index.get(a, -1)]
    for a, b, s, t in sorted(set(upward))[:60]:
        print(f"UPWARD {a} -> {b}: {s} -> {t}")
    for path, head, home in incoherent[:60]:
        print(f"INCOHERENT {path}: impl {head} (type in {home})")
    ok &= not upward and not incoherent
    print(f"\nclean={ok}")
    if ok and "--write-layers" in sys.argv:
        target = sys.argv[sys.argv.index("--write-layers") + 1]
        allow = {g: sorted({b for (b, _, _) in graph[g]}) for g in order}
        json.dump({"order": order, "allow": allow, "rules": config["rules"]}, open(target, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        print(f"wrote {target}")


main()
