"""✂️ Splits the cad core-interactions suite: each group of top-level `describe` blocks moves into a child `registerXTests` function.

usage: cadsplit.py [--apply]
The snapshot is the pre-edit file. Describe blocks keep their text and order; the parent becomes an orchestrator that bootstraps the cad
modules once and calls the children in the original order. Each child re-declares only the imports, dependency bindings and fixture
constants its own text uses; relative import specifiers are re-based one folder deeper.
"""
import json, os, re, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LIVE = json.load(open(F + "/c4p21-tools/files.json", encoding="utf-8"))["f1"]
SNAP = F + "/🗑️generated/c4p21-snap/f1.ts"
APPLY = "--apply" in sys.argv
src = open(SNAP, encoding="utf-8").read()
L = src.split("\n")
assert L[-1] == ""
L = L[:-1]

GROUPS = [
    ("🕹️transforms", "registerTransformInteractionTests", "Interaction transforms: curve arc, move and copy sessions and numeric entry commit.", [0]),
    ("🗂️registries", "registerRegistryTests", "Action and interaction registries: declarative specs, typology construction, nested interaction calls, selection actions.", [1]),
    ("📦️box-length", "registerBoxLengthEntryTests", "Box construction, length entry and the stateEngine option.", [2, 3, 4]),
    ("📏️measure", "registerMeasureTests", "Measure distance and measure area interactions.", [5, 6]),
    ("🕰️history", "registerHistoryTests", "Document history, measure history, session undo and redo, and undo routing.", [7, 8, 9, 10]),
    ("🎞️e2e", "registerInteractionE2eTests", "End-to-end interaction fixtures over every shipped interaction.", [11]),
]

imports = []
i = 0
while L[i].startswith("import "):
    imports.append(L[i])
    i += 1
assert L[i] == "" and L[i + 1].startswith("type TestSource"), L[i : i + 2]
type_line = L[i + 1]
assert L[i + 3].startswith("export async function registerTests1(")
sig = L[i + 3]
dep_line = L[i + 4]
m = re.match(r"^  const \{ (.*) \} = dependencies;$", dep_line)
assert m
dep_names = [n.strip() for n in m.group(1).split(",")]
pre = L[i + 5 : i + 14]
assert pre[0] == "" and pre[1].startswith("  __artifactTestRuntime!.bootstrapCadModules();"), pre[:2]
pre_decls = [l for l in pre[2:] if l.strip()]
assert pre_decls[0].startswith("  const { BrepjsKernel }")
body_start = i + 5 + len(pre)
starts = [n for n in range(body_start, len(L)) if L[n].startswith("  describe(")]
assert L[-1] == "}", L[-1]
ends = [n for n in range(body_start, len(L)) if L[n] == "  });"]
assert len(starts) == len(ends) == 12, (len(starts), len(ends))
for s, e in zip(starts, ends):
    assert s < e
blocks = ["\n".join(L[s : e + 1]) for s, e in zip(starts, ends)]
for k in range(1, 12):
    assert starts[k] > ends[k - 1]
    assert all(l.strip() == "" for l in L[ends[k - 1] + 1 : starts[k]])
assert all(l.strip() == "" for l in L[ends[-1] + 1 : -1])

TOK = re.compile(r"[A-Za-z_$][\w$]*")
def toks(text):
    return set(TOK.findall(text))

parsed = []
for line in imports:
    mm = re.match(r'^import (type )?\{ (.*) \} from "(.*)";$', line)
    assert mm, line
    names = [n.strip() for n in mm.group(2).split(",")]
    parsed.append((bool(mm.group(1)), names, mm.group(3)))

ROOT_DIR = os.path.dirname(LIVE)
def rebase(spec, child_dir):
    if not spec.startswith("."):
        return spec
    target = os.path.normpath(os.path.join(ROOT_DIR, spec))
    r = os.path.relpath(target, child_dir)
    return r if r.startswith(".") else "./" + r

def decl_name(line):
    mm = re.match(r"^  const (\w+) = ", line)
    return mm.group(1) if mm else None

def child_text(dirname, fn, doc, idxs):
    child_dir = os.path.join(ROOT_DIR, dirname)
    body = "\n\n".join(blocks[k] for k in idxs)
    need = toks(body) | {"ArtifactTestDependencies"}
    emitted = []
    for line in pre_decls[1:]:
        pass
    pre_lines = []
    changed = True
    chosen = set()
    while changed:
        changed = False
        for line in pre_decls:
            nm = decl_name(line)
            if nm is None or nm in chosen:
                continue
            if nm in need:
                chosen.add(nm)
                need |= toks(line)
                changed = True
    for line in pre_decls:
        nm = decl_name(line)
        if nm in chosen:
            pre_lines.append(line)
    uses_kernel = "BrepjsKernel" in need
    if uses_kernel:
        need.add("__artifactTestKernel")
    deps = [n for n in dep_names if n in need and n != "__artifactTestRuntime"]
    imp_lines = []
    for is_type, names, spec in parsed:
        kept = [n for n in names if n.split(" as ")[-1].replace("type ", "").strip() in need]
        if kept:
            imp_lines.append("import %s{ %s } from \"%s\";" % ("type " if is_type else "", ", ".join(kept), rebase(spec, child_dir)))
    vit = [n for n in ("describe", "expect", "it") if n in toks(body)]
    out = ["/** %s %s */" % (dirname[: next(i for i, ch in enumerate(dirname) if ch.isascii() and ch.isalpha())], doc)]
    out += imp_lines
    out += ["", sig.replace("registerTests1", fn).replace(", source: TestSource", "")]
    out += ["  const { %s } = dependencies;" % ", ".join(deps)]
    if uses_kernel:
        out += ["  const { BrepjsKernel } = __artifactTestKernel!;"]
    out += [l for l in pre_lines if not l.startswith("  const { BrepjsKernel }")]
    out += ["  const { %s } = vitest;" % ", ".join(vit), "", body, "}", ""]
    return "\n".join(out)

outputs = []
for dirname, fn, doc, idxs in GROUPS:
    outputs.append((dirname, fn, child_text(dirname, fn, doc, idxs)))
assert sorted(k for g in GROUPS for k in g[3]) == list(range(12))

root_imports = ['import type { ArtifactTestDependencies } from "../../🟦️.ts";']
for dirname, fn, _ in outputs:
    root_imports.append('import { %s } from "./%s/🟦️.ts";' % (fn, dirname))
root = ["/** 🧪️ The `@semio-tech/cad-js/core` interaction suite: bootstraps the cad modules once, then registers each interaction group in its original order. */"]
root += root_imports + ["", type_line, "", sig, "  const { __artifactTestRuntime } = dependencies;", "", "  __artifactTestRuntime!.bootstrapCadModules();"]
root += ["  await %s(vitest, dependencies);" % fn for _, fn, _ in outputs]
root += ["}", ""]
root_text = "\n".join(root)

for d, fn, t in outputs:
    print("%-16s %-36s %5d lines" % (d, fn, len(t.split("\n")) - 1))
print("root %d lines" % (len(root_text.split("\n")) - 1))
if APPLY:
    assert open(LIVE, encoding="utf-8").read() == src, "live file changed since the snapshot"
    for d, fn, t in outputs:
        target = os.path.join(ROOT_DIR, d, "🟦️.ts")
        assert not os.path.exists(target), target
        os.makedirs(os.path.dirname(target))
        open(target, "w", encoding="utf-8").write(t)
    open(LIVE, "w", encoding="utf-8").write(root_text)
    print("applied")
