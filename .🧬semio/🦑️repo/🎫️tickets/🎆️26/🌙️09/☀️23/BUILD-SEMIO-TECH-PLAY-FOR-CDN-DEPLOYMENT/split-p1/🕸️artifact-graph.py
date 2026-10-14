#!/usr/bin/env python3
"""🕸️ File-level reference graph of the energy model monolith, folded onto planning layers (lane P1).

Builds on the copied census `🕸️module-graph.py` (`Crate`, `strip`, `flatten`, `inline_spans`, `test_spans`) and adds the
monolith's own resolution facts: the pre-migration shim modules (`crate::{schema,diff,mutations,snapshot}`), the flat engine
re-exports (`crate::Model`) and the root-level re-exports/items. Prints the layer edge matrix (lib and test separately) and,
with `--edges A B`, the file pairs behind one layer edge. `--json` dumps the file graph for the cut planner.
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
g = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "mg", "exec"), g)
Crate, strip, flatten, inline_spans, test_spans = g["Crate"], g["strip"], g["flatten"], g["inline_spans"], g["test_spans"]
REPO = "/Users/ueli/Documents/semio"
ART = REPO + "/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/"
ENGINE = REPO + "/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/"
SHIMS = {"schema": ("standards", "v1", "subsets", "any", "schema"), "diff": ("standards", "v1", "subsets", "any", "schema", "diff"),
         "mutations": ("standards", "v1", "subsets", "any", "schema", "mutations"), "snapshot": ("standards", "v1", "subsets", "any", "schema", "snapshot")}
ITEM = re.compile(r"(?m)^pub(?:\([^)]*\))?\s+(?:async\s+)?(?:unsafe\s+)?(?:struct|enum|fn|const|static|trait|type|union|mod)\s+([A-Za-z_][A-Za-z0-9_]*)")
crate = Crate(ART + "/🦀️.rs")
root_items = {}
for f, mp in crate.files.items():
    if f.startswith(ENGINE) and len(mp) == 1:
        for name in ITEM.findall(crate.text[f]):
            root_items.setdefault(name, f)
for name in ITEM.findall(crate.text[crate.root]):
    if name not in SHIMS and name not in ("standards", "editor", "viewer", "examples"):
        root_items[name] = crate.root
for m in re.finditer(r"(?m)^pub use crate::((?:[a-z_0-9]+::)+)\{?([^;}]*)\}?;", crate.text[crate.root]):
    mods = tuple(m.group(1).strip(":").split("::"))
    for name in [x.strip() for x in m.group(2).split(",") if x.strip()]:
        root_items[name] = crate.resolve(mods + (name,))
ENGINE_MODS = {re.sub(r"^[^a-z_]+", "", d) for d in os.listdir(ENGINE) if os.path.isdir(os.path.join(ENGINE, d))} | {"epw"}
def resolve(mp):
    mp = tuple(mp)
    if mp and mp[0] in ENGINE_MODS: return "ENGINE:" + mp[0]
    if mp and mp[0] in SHIMS:
        mp = SHIMS[mp[0]] + mp[1:]
    if len(mp) >= 1 and mp[0] in root_items and (len(mp) == 1 or mp[0][0].isupper() or mp[0] not in {m[0] for m in crate.mods if m}):
        return root_items[mp[0]]
    return crate.resolve(mp)
USE_RE = re.compile(r"\b(crate|super|self)((?:\s*::\s*(?:[A-Za-z_][A-Za-z0-9_]*|super))+)(\s*::\s*\{)?")
def references(file):
    s = crate.text[file]; modpath = crate.files[file]; spans = inline_spans(s)
    def base_at(off):
        base = list(modpath)
        for a, b, name in spans:
            if a <= off < b: base.append(name)
        return base
    paths = []
    for m in re.finditer(r"\buse\s+(crate|super|self)\s*::", s):
        j = m.end(); d = 0; k = j
        while k < len(s):
            if s[k] == "{": d += 1
            elif s[k] == "}": d -= 1
            elif s[k] == ";" and d == 0: break
            k += 1
        grp = m.group(1) + "::" + s[j:k]; head, rest = grp.split("::", 1)
        for p in flatten([head], rest): paths.append((m.start(), p))
    for m in USE_RE.finditer(s):
        if m.group(3): continue
        paths.append((m.start(), [x for x in re.split(r"\s*::\s*", (m.group(1) + m.group(2)).strip()) if x]))
    tests = test_spans(s); out = {}
    for off, segs in paths:
        if not segs: continue
        if segs[0] == "crate": mp = segs[1:]
        else:
            base = base_at(off); k = 0
            while k < len(segs) and segs[k] in ("super", "self"):
                if segs[k] == "super" and base: base.pop()
                k += 1
            mp = base + segs[k:]
        target = resolve(mp)
        if target == file: continue
        t = any(a <= off < b for a, b in tests)
        out[target] = out.get(target, True) and t
    return out
def layer(f):
    if f.startswith("ENGINE:"): return "engine"
    if f.startswith(ENGINE): return "engine:" + re.sub(r"^[^a-z_]+", "", f[len(ENGINE):].split("/")[0])
    if "/🧩️extensions/🌦️epw/" in f: return "epw"
    if f == crate.root: return "root"
    if f.startswith(ANY):
        r = f[len(ANY):].split("/")
        if r[0] == "🧬️schema":
            if len(r) > 2 and r[1] == "🧬️mutations":
                if r[2] == "🧪️tests": return "mut-support"
                if r[2] == "🦀️.rs": return "mut-agg"
                return "mut-leaf-test" if "🧪️tests" in r else "mut-leaf"
            if len(r) > 2 and r[1] in ("🔺️diff", "📸️snapshot", "💡️inferences"): return {"🔺️diff": "diff", "📸️snapshot": "snapshot", "💡️inferences": "inferences"}[r[1]]
            return "schema"
        if r[0] == "🚪️io": return "io" if len(r) == 2 or r[1] == "🧪️tests" else "io-" + re.sub(r"^[^a-z]+", "", r[1])
        if r[0] == "✏️editor": return "editor" if len(r) == 2 or r[1] == "🧪️tests" else "editor-" + re.sub(r"^[^a-z]+", "", r[1])
        if r[0] == "👁️viewer": return "viewer"
        if r[0] == "🧵️simulation-session": return "session"
        if r[0] == "📚️examples": return "examples"
        return "any-" + r[0]
    return "?"
def is_test(f): return f in crate.test_files or "🧪️tests" in f
if __name__ == "__main__":
    edges = collections.Counter(); pairs = collections.defaultdict(set); fg = {}
    for f in crate.files:
        a = layer(f); refs = references(f); fg[f] = {t: v for t, v in refs.items()}
        for t, testonly in refs.items():
            b = layer(t)
            if a == b: continue
            kind = "test" if (is_test(f) or testonly) else "lib"
            edges[(kind, a, b)] += 1; pairs[(a, b)].add((os.path.relpath(f, REPO), os.path.relpath(t, REPO), kind))
    if "--json" in sys.argv:
        json.dump({"files": {os.path.relpath(f, REPO): {"layer": layer(f), "test": is_test(f), "module": "::".join(crate.files[f]), "refs": {(t if t.startswith("ENGINE:") else os.path.relpath(t, REPO)): v for t, v in fg[f].items()}} for f in crate.files}}, open(sys.argv[sys.argv.index("--json") + 1], "w"), ensure_ascii=False, indent=0)
    if "--edges" in sys.argv:
        i = sys.argv.index("--edges"); a, b = sys.argv[i + 1], sys.argv[i + 2]
        for x in sorted(pairs[(a, b)]): print(x[2], x[0], "->", x[1])
    else:
        coarse = lambda l: "engine" if l.startswith("engine:") else l
        agg = collections.Counter()
        for (k, a, b), n in edges.items():
            if coarse(a) != coarse(b): agg[(k, coarse(a), coarse(b))] += n
        for (k, a, b), n in sorted(agg.items()): print(f"{k}\t{a} -> {b}\t{n}")
