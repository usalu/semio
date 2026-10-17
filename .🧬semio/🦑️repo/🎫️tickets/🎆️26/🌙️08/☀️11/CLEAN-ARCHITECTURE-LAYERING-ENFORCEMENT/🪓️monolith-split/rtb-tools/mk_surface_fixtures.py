"""🧰️ Moves the retained-surface helpers shared by the engine-surfaces, node-graph and paint2d laws into ui-engine-tests (`surface_fixtures`).
usage: mk_surface_fixtures.py plan|apply"""
import glob, re, sys
from lib import *
APPLY = "apply" in sys.argv
TEST = glob.glob(EL + "/⚙️EngineCanvas/🧪️tests/*wgpu-engine-surfaces/🦀️.rs")[0]
NG = glob.glob(EL + "/⚙️EngineCanvas/🧪️tests/*wgpu-node-graph/🦀️.rs")[0]
P2D = glob.glob(EL + "/⚙️EngineCanvas/🧪️tests/*wgpu-paint2d-engine/🦀️.rs")[0]
HOME = T + "/🧿️ui-engine-tests"
MOD = HOME + "/🧰️surface-fixtures/🦀️.rs"
ROOT = HOME + "/🦀️.rs"
MOVE = ["RetainedSurfacePaint", "close_retained_surface_fixture", "drop_world3d_states", "paint_retained_surface_in_window", "paint_retained_surface_scene", "retained_document", "retained_surface_record", "with_world_step"]
KEEP_LOCAL_DROP = "drop_engine_surface"
TS = "semio_framework_os_renderer_wgpu_ui_engine_tests::surface_fixtures"

text = read(TEST)
clean = blank(text)
lines, cll = text.split("\n"), clean.split("\n")
items, depth, i, n = [], 0, 0, len(cll)
while i < n:
    l = cll[i]
    if depth == 0 and l and not l[0].isspace() and l[0] not in "}#/":
        s = i
        while s > 0 and lines[s - 1].lstrip().startswith(("#[", "///", "//")):
            s -= 1
        d, seen, j = 0, False, i
        while j < n:
            d += cll[j].count("{") - cll[j].count("}")
            seen = seen or "{" in cll[j]
            if (seen and d <= 0) or (not seen and cll[j].rstrip().endswith(";")):
                break
            j += 1
        mm = re.match(r"(?:pub(?:\([^)]*\))? )?(?:const |async |unsafe )*(?:fn|struct|enum|const|static|type) (\w+)", l.strip())
        items.append(dict(s=s, e=j, name=mm.group(1) if mm else None))
        i = j + 1
        continue
    depth += l.count("{") - l.count("}")
    i += 1
byname = {it["name"]: it for it in items if it["name"]}
moved = [byname[m] for m in MOVE]
drop = byname[KEEP_LOCAL_DROP]
def chunk(it):
    return "\n".join(lines[it["s"]:it["e"] + 1])
body = "\n\n".join(re.sub(r"pub\(super\)", "pub", re.sub(r"(?m)^((?:///[^\n]*\n)*)(fn |struct )", r"\1pub \2", chunk(it))) for it in sorted(moved, key=lambda x: x["s"]))
# imports: the test file's own uses and the ui-engine-tests laws block, filtered by the tokens the moved code names
toks = tokens(body) | set(re.findall(r"[A-Za-z_]\w*", blank(body)))
sources = [read(TEST), read(ROOT)]
use_lines, seen = [], set()
for src in sources:
    for cfg, vis, path, local, glob_ in uses(src, top_only=False):
        if glob_ or not local or local not in toks or not path:
            continue
        if path[0] in ("super", "self") or local == "fixture_original_turn":
            continue
        piece = path[-1] if path[-1] == local else f"{path[-1]} as {local}"
        line = f"use {'::'.join(path[:-1])}::{piece};" if len(path) > 1 else f"use {piece};"
        if line not in seen:
            seen.add(line)
            use_lines.append((cfg + "\n" if cfg else "") + line)
head = "//! 🧰️ Retained-surface helpers of the EngineCanvas laws: paint a scene through the retained document and the Interpreter admission seam, then close it through the production ladder. Shared by the engine-surfaces, node-graph and paint2d laws, which live in two bins.\n\n"
new_mod = head + "\n".join(sorted(use_lines)) + "\nuse semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;\n\n" + body + "\n"
# test file without the moved items and the local drop_engine_surface
cut = set()
for it in moved + [drop]:
    cut.update(range(it["s"], it["e"] + 1))
rest = "\n".join(l for k, l in enumerate(lines) if k not in cut)
rest = re.sub(r"\n{3,}", "\n\n", rest)
anchor = "use super::*;\n"
assert rest.count(anchor) == 1
rest = rest.replace(anchor, anchor + f"use {TS}::*;\nuse semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;\n", 1)
print("moved:", MOVE, "| dropped local", KEEP_LOCAL_DROP)
print("module:", len(new_mod.split("\n")), "lines;", len(use_lines), "imports; test file:", len(lines), "->", len(rest.split("\n")), "lines")
if APPLY:
    write(MOD, new_mod)
    r = read(ROOT)
    old = '#[path = "../../../🧪️tests/🎟️original-turn/🦀️.rs"]\npub mod fixture_original_turn;\n'
    assert r.count(old) == 1
    edit(ROOT, r.replace(old, old + '\n#[path = "🧰️surface-fixtures/🦀️.rs"]\npub mod surface_fixtures;\n'))
    edit(TEST, rest)
    for f in (NG, P2D):
        t = read(f)
        t2 = t.replace("super::engine_surface_attach_tests::", TS + "::")
        edit(f, t2)
