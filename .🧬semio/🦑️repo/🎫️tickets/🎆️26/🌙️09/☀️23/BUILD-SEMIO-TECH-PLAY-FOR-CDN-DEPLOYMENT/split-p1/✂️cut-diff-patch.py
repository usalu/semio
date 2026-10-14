#!/usr/bin/env python3
"""✂️ Byte-budget follow-up of the schema layer (lane P1, slice S2g).

- The diff's patch vocabulary (`🔺️diff/🩹️patch`) and its splice (`🔺️diff/✂️splice`) become `-diff-patch` and
  `-diff-splice`, so the diff crate keeps room for its 202 KB of facet leaves and its tests; every `Rows`/`ListEdit`/…
  reference names the patch crate.
- The snapshot facet's five leaves move into the snapshot crate (`snapshot::FACET`), beside the snapshot they describe,
  so the schema descriptor only embeds the artifact and mutation facets.
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
DIFF = f"{L.ANY}/🧬️schema/🔺️diff"; SNAP = f"{L.ANY}/🧬️schema/📸️snapshot"; SCH = f"{L.ANY}/🧬️schema"
HEADER = "#![allow(async_fn_in_trait)]\n#![allow(clippy::too_many_arguments)]\n\n"
d = L.rd(f"{DIFF}/🦀️.rs")
if '#[path = "🩹️patch/🦀️.rs"]' not in d: sys.exit("already applied")
d = L.replace_once(d, '#[path = "🩹️patch/🦀️.rs"]\npub mod patch;\n#[path = "✂️splice/🦀️.rs"]\npub mod splice;\n', "")
m = re.search(r"pub use patch::\{([^}]*)\};\npub use splice::Splice;\n", d)
PATCH_NAMES = [x.strip() for x in m.group(1).split(",")] + ["Splice"]
d = d.replace(m.group(0), "")
L.wr(f"{DIFF}/🦀️.rs", d)
for folder in ("🩹️patch", "✂️splice"):
    t = L.rd(f"{DIFF}/{folder}/🦀️.rs"); i = t.index("\n\n") + 2
    t = t[:i] + HEADER + "@@ALIASES@@\n\n" + t[i:]
    t = t.replace("use super::splice::", "use crate::diff_splice::")
    L.wr(f"{DIFF}/{folder}/🦀️.rs", t)
ent = L.rd(f"{DIFF}/🧱️entities/🦀️.rs")
ent = ent.replace("use super::patch::", "use crate::diff_patch::")
L.wr(f"{DIFF}/🧱️entities/🦀️.rs", ent)
names = set(PATCH_NAMES) - {"Splice"}
def fix_groups(text, owner_alias):
    def g(m):
        items = [x.strip() for x in m.group(2).split(",") if x.strip()]
        moved = [x for x in items if re.match(r"\w+", x).group(0) in PATCH_NAMES]
        keep = [x for x in items if x not in moved]
        out = []
        if keep: out.append(f"use {m.group(1)}::{{{', '.join(keep)}}};")
        if moved: out.append(f"use crate::{'diff_splice' if moved == ['Splice'] else 'diff_patch'}::{{{', '.join(moved)}}};")
        return "\n".join(out)
    text = re.sub(r"use (crate::diff)::\{([^{}]*)\};", g, text)
    return L.rewrite_code(text, [(r"\bcrate\s*::\s*diff\s*::\s*(" + "|".join(sorted(names, key=len, reverse=True)) + r")\b", r"crate::diff_patch::\1"),
                                 (r"\bcrate\s*::\s*diff\s*::\s*Splice\b", "crate::diff_splice::Splice")])
diff_files = set(L.closure_files(f"{DIFF}/🦀️.rs"))
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(r, f); t = L.rd(p)
        if p in diff_files:
            t2 = L.rewrite_code(t, [(r"(?<![\w:])(" + "|".join(sorted(PATCH_NAMES, key=len, reverse=True)) + r")\b(?=\s*(?:::|<|\{|\(|;|,|\)|>|\s))", r"\1")])
        t2 = fix_groups(t, None)
        if t2 != t: L.wr(p, t2)
# snapshot facet moves to the snapshot crate
s = L.rd(f"{SCH}/🦀️.rs")
s, snap_leaves = L.take(s, "        snapshot: semio_framework_schema_registry::FacetLeaves {\n", "        },\n")
s = L.replace_once(s, "        diff: crate::diff::FACET,\n", "        snapshot: crate::snapshot::FACET,\n        diff: crate::diff::FACET,\n")
L.wr(f"{SCH}/🦀️.rs", s)
sn = L.rd(f"{SNAP}/🦀️.rs")
facet = ('\n//#region 🧬️Facet\n/// 🧬️ The snapshot facet\'s five handcrafted language leaves, owned here beside the snapshot they describe; the\n'
         '/// artifact schema descriptor (`semio_s_artifact_energy_model_schema`) assembles them.\npub const FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {\n'
         '    rust: include_str!("🦀️.rs"),\n    typescript: include_str!("🟦️.ts"),\n    graphql: include_str!("🔗️.graphql"),\n    json_schema: include_str!("🔣️.json"),\n    proto: include_str!("🛰️.proto"),\n};\n//#endregion 🧬️Facet\n')
sn = L.replace_once(sn, "\n//#region 🚪️Representations\n", facet + "\n//#region 🚪️Representations\n")
L.wr(f"{SNAP}/🦀️.rs", sn)
print("patch names:", PATCH_NAMES)
