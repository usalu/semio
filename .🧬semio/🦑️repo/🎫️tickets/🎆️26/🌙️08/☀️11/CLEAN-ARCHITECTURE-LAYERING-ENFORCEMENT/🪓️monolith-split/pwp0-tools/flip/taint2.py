# 🧪️ Used (not dead) upward / undeclared reference census per crate and gate state.
import re, collections, sys
from common import *
import model
from modules import module_map
from rs import blank, expand_use
pt = model.pt
mm = module_map()
rank = {k: v["rank"] for k, v in CRATES.items()}
deps = {k: set(v["deps"]) for k, v in CRATES.items()}
def mod_crate(name):
    m = mm.get(name)
    if not m: return None
    if m["kind"] == "file": return model.file_owner(m["file"])
    return m["lib"].replace("semio_framework_plugin_", "").replace("_", "-")
o = model.owners()
GATE = re.compile(r"cfg\((?:any\()?[^)]*(?:test|artifact-app-testing|media-turn-testing|reserved-producer-testing|original-actor-context-testing)")
USE = re.compile(r"(?m)^[ \t]*(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+(crate::[^;]*);")
def run(k, verbose=False):
    res = collections.defaultdict(lambda: [0, 0, 0]); detail = collections.defaultdict(set)
    for f in o[k]:
        text = read(PLUG + "/" + f); clean = blank(text); lines = text.split("\n")
        testfile = "/🧪️tests/" in f
        use_spans = [(m.start(), m.end()) for m in USE.finditer(clean)]
        def in_use(p): return any(a <= p < b for a, b in use_spans)
        def gate_at(pos):
            ln = clean.count("\n", 0, pos)
            return testfile or bool(GATE.search("\n".join(lines[max(0, ln - 3):ln + 1])))
        def bump(c, gated, used, what):
            res[c][(1 if gated else 0) if used else 2] += 1
            if used and not gated: detail[c].add(what)
        for m in USE.finditer(clean):
            gated = gate_at(m.start())
            for path, alias, glob in expand_use(m.group(1)):
                if len(path) < 2: continue
                c = mod_crate(path[1]) or "ROOTNAME:" + path[1]
                if c == k: continue
                if glob: used = True
                else:
                    local = alias or path[-1]
                    rest = "".join(clean[a:b] if False else " " * (b - a) for a, b in use_spans)
                    used = len(re.findall(r"(?<![A-Za-z0-9_$])" + re.escape(local) + r"(?![A-Za-z0-9_])", clean_wo_uses(clean, use_spans))) > 0
                bump(c, gated, used, "::".join(path[1:]))
        for m in re.finditer(r"(?<![A-Za-z0-9_$:])crate::([A-Za-z_]\w*)", clean):
            if in_use(m.start()): continue
            c = mod_crate(m.group(1)) or "ROOTNAME:" + m.group(1)
            if c == k: continue
            bump(c, gate_at(m.start()), True, m.group(1))
    bad = {c: v for c, v in res.items() if c.startswith(("TOP", "UNMAPPED", "ROOTNAME")) or c not in rank or rank[c] >= rank[k] or c not in deps[k]}
    return bad, detail
def clean_wo_uses(clean, spans):
    out, last = [], 0
    for a, b in spans:
        out.append(clean[last:a]); out.append(" " * (b - a)); last = b
    out.append(clean[last:]); return "".join(out)
if __name__ == "__main__":
    want = sys.argv[1:] or [k for k in CRATES if CRATES[k]["kind"] == "lower"]
    for k in want:
        if k not in o: continue
        bad, detail = run(k)
        print(f"== {k}: " + ", ".join(f"{c}=ungated-used {v[0]} / gated-used {v[1]} / dead {v[2]}" + (f" [{' '.join(sorted(detail[c])[:4])}]" if v[0] else "") for c, v in sorted(bad.items()) if v[0] or v[1]))
