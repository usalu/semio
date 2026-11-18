import re, os
N = "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm"
EXT = {"📗️eurocode": ["en1990", "en1991", "en1997", "en1998"], "📙️eurocode-materials": ["en1992", "en1993", "en1994", "en1995", "en1996", "en1999"]}
NORM = ["din4108", "din16798", "din18599", "iso16757", "vdi3805"]
ALL = NORM + EXT["📗️eurocode"] + EXT["📙️eurocode-materials"]
def rd(p): return open(os.path.join(N, p), encoding="utf8").read()
def wr(p, s):
    p = os.path.join(N, p); os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf8").write(s)
def fam_of(name): return re.match(r"([a-z]+\d+)", name).group(1)
def blocks(text, start_re, end_re):
    out, cur = [], None
    for line in text.split("\n"):
        if re.match(start_re, line): cur = [line]
        elif cur is not None: cur.append(line)
        if cur is not None and re.match(end_re, line): out.append("\n".join(cur)); cur = None
    return out

surf = rd("🧪️tests/🔬️surface/🦀️.rs")
laws = [l for l in surf.split("\n") if l.startswith("surface_law!(")]
sets = blocks(surf, r"^set_active_example_case!\($", r"^\);$")
rosters = [l for l in surf.split("\n") if l.startswith("roster_examples_case!(")]
assert len(laws) == len(sets) == len(rosters) == 15
def pick(items, fams, key):
    return [i for i in items if fam_of(key(i)) in fams]
lines = surf.split("\n")
def sect(a, b):
    ia = next(i for i, l in enumerate(lines) if a in l); ib = next(i for i, l in enumerate(lines) if b in l)
    return "\n".join(lines[ia:ib])
macro_surface = sect("macro_rules! surface_law", "surface_law!(din4108")
set_macro = sect("//#region 🎨️SetActiveExample", "set_active_example_case!(\n") if False else None
i0 = surf.index("//#region 🎨️SetActiveExample"); i1 = surf.index("set_active_example_case!(\n    din4108")
set_macro = surf[i0:i1]
j0 = surf.index("//#region 🗂️RosterExamples"); j1 = surf.index("roster_examples_case!(din4108")
roster_macro = surf[j0:j1]
roster_macro = roster_macro.replace('fixture_path!("../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts")', "fixture_path!(ARTIFACTS_ROOT)")
shared_surface = ("use semio_framework_test_fixtures::fixture_path;\nuse semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};\n\n"
  + macro_surface + "\n" + set_macro + "\n" + roster_macro + "\n//#endregion 🗂️RosterExamples\n")
shared_surface = shared_surface.replace("\n//#endregion 🎨️SetActiveExample\n\n//#region 🗂️RosterExamples", "\n//#endregion 🎨️SetActiveExample\n\n//#region 🗂️RosterExamples")
wr("🧪️tests/🧰️laws/🔬️surface/🦀️.rs", shared_surface)

aa = rd("🧪️tests/🎯️action-args/🦀️.rs").split("\n")
k = next(i for i, l in enumerate(aa) if l == "#[test]")
helpers = "\n".join(aa[5:k]).rstrip() + "\n"
shared_aa = helpers + """
macro_rules! value_tree_case {
    ($name:ident, $create:path) => {
        #[test]
        fn $name() {
            assert_norm_value_tree_arguments($create());
        }
    };
}
"""
shared_aa = shared_aa.replace("use semio_plugin::{ActionArgDef", "use semio_plugin::{ActionArgDef") 
head_aa = "".join(l + "\n" for l in aa[5:6])
wr("🧪️tests/🧰️laws/🎯️action-args/🦀️.rs", "use semio_plugin::{ActionArgDef, AppDefinition, ArgSchema};\n\n" + "\n".join(aa[7:k]).rstrip() + shared_aa[len(helpers):] if False else
   "use semio_plugin::{ActionArgDef, AppDefinition, ArgSchema};\n\n" + "\n".join(aa[6:k]).rstrip() + "\n" + shared_aa[len(helpers):])

def aa_doc(n):
    return ("//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them\n"
            "//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema\n"
            "//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`/`setSnapshot`. Ticket 26/09/23 slice S19.\n")
def aa_cases(fams): return "\n".join(f"value_tree_case!({f}_declares_its_value_tree_arguments, semio_s_artifact_norm_{f}_editor::create_{f}_app);" for f in fams) + "\n"
def surf_file(fams, up, root):
    return ("//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).\n"
        f'const ARTIFACTS_ROOT: &str = "{root}";\n\ninclude!("{up}🧪️tests/🧰️laws/🔬️surface/🦀️.rs");\n\n'
        + "\n".join(pick(laws, fams, lambda l: re.search(r"\((\w+?)_", l).group(1))) + "\n\n"
        + "\n".join(pick(sets, fams, lambda b: b.split("\n")[1].strip())) + "\n\n"
        + "\n".join(pick(rosters, fams, lambda l: re.search(r"\((\w+?)_", l).group(1))) + "\n")
UP_NORM = "../"
UP_EXT = "../../../../"
ROOT_NORM = "../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ROOT_EXT = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts"
def surface_up(prefix): return prefix
wr("🧪️tests/🔬️surface/🦀️.rs", surf_file(NORM, "../", ROOT_NORM).replace('include!("../🧪️tests/🧰️laws/🔬️surface/🦀️.rs")', 'include!("../🧰️laws/🔬️surface/🦀️.rs")'))
wr("🧪️tests/🎯️action-args/🦀️.rs", aa_doc(0) + '\ninclude!("../🧰️laws/🎯️action-args/🦀️.rs");\n\n' + aa_cases(NORM))
for d, fams in EXT.items():
    wr(f"🧩️extensions/{d}/🧪️tests/🔬️surface/🦀️.rs", surf_file(fams, "../../../../", ROOT_EXT))
    wr(f"🧩️extensions/{d}/🧪️tests/🎯️action-args/🦀️.rs", aa_doc(0) + '\ninclude!("../../../../🧪️tests/🧰️laws/🎯️action-args/🦀️.rs");\n\n' + aa_cases(fams))

g = rd("🧪️tests/🚦️compliance-gate/🦀️.rs").split("\n")
def L(a, b): return "\n".join(g[a-1:b])
harness = "\n".join([L(7, 9), "", L(11, 12), "", L(16, 17), "", L(22, 247).rstrip(), "", L(331, 420).rstrip(), ""])
wr("🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs", harness)
res = blocks("\n".join(g[423:]), r"^    results\.push\(", r"^    \)\);$")
assert len(res) == 15, len(res)
def gate_test(fams, scan):
    sel = [r for r in res if re.search(r'^        "(\w+)",', r, re.M).group(1) in fams]
    body = ("    let gaming = scan_evaluate_gaming();\n    if !gaming.is_empty() {\n        panic!(\"evaluate-path gaming source-scan failed ({} hits):\\n{}\", gaming.len(), gaming.join(\"\\n\"));\n    }\n\n" if scan else "")
    n = len(fams)
    return ("#[test]\nfn compliance_gate_all_families() {\n" + body + "    let mut results = Vec::new();\n\n" + "\n".join(sel) + "\n\n"
        "    let mut summary = String::new();\n    let mut failed = 0usize;\n    for result in &results {\n        let _ = writeln!(summary, \"{}\", result.summary());\n        if !result.ok() {\n            failed += 1;\n        }\n    }\n"
        f"    assert_eq!(results.len(), {n}, \"gate must cover all {n} families this plugin owns\");\n    if failed > 0 {{\n        panic!(\"{{failed}}/{n} families failed compliance gate:\\n{{summary}}\");\n    }}\n}}\n")
head = "//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.\n\n#![allow(clippy::type_complexity)]\n\n"
scan = "\n".join(["use semio_framework_test_fixtures::fixture_path;", "use std::fmt::Write as _;", "use std::path::{Path, PathBuf};", "", L(19, 20), "", L(248, 296).rstrip(), "", L(302, 330).rstrip().replace('fixture_path!("../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts")', 'fixture_path!("../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts")'), ""])
wr("🧪️tests/🚦️compliance-gate/🦀️.rs", head + scan + '\ninclude!("🧰️harness/🦀️.rs");\n\n' + gate_test(NORM, True))
leak = "\n".join([L(297, 300).rstrip(), ""])
for d, fams in EXT.items():
    pre = head + "use std::fmt::Write as _;\n\n" + ('include!("../../../../🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs");\n\n')
    wr(f"🧩️extensions/{d}/🧪️tests/🚦️compliance-gate/🦀️.rs", pre + (leak + "\n" if d == "📗️eurocode" else "") + gate_test(fams, False))
