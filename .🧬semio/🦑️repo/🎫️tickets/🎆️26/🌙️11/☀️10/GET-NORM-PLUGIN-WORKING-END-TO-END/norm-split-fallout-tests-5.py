import re, os
N = "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/📕️norm"
OLD = {"": ["din4108","din16798","din18599","iso16757","vdi3805"], "📗️eurocode": ["en1990","en1991","en1997","en1998"], "📙️eurocode-materials": ["en1992","en1993","en1994","en1995","en1996","en1999"]}
NEW = {"": ["din4108","din16798","iso16757","vdi3805"], "📒️energy": ["din18599"], "📗️eurocode": ["en1990","en1991","en1997","en1998"], "📓️eurocode-structural": ["en1992","en1993","en1994"], "📙️eurocode-materials": ["en1995","en1996","en1999"]}
CRATE = {"": "semio_hub_norm", "📒️energy": "semio_hub_norm_energy", "📗️eurocode": "semio_hub_norm_eurocode", "📓️eurocode-structural": "semio_hub_norm_eurocode_structural", "📙️eurocode-materials": "semio_hub_norm_eurocode_materials"}
def base(d): return N if d == "" else f"{N}/🧩️extensions/{d}"
def rd(d, p): return open(f"{base(d)}/{p}", encoding="utf8").read()
def wr(d, p, s):
    q = f"{base(d)}/{p}"; os.makedirs(os.path.dirname(q), exist_ok=True); open(q, "w", encoding="utf8").write(s)
def fam(name): return re.search(r"([a-z]+\d+)", name).group(1)
def blocks(text, start, end):
    out, cur = [], None
    for line in text.split("\n"):
        if re.match(start, line): cur = [line]
        elif cur is not None: cur.append(line)
        if cur is not None and re.match(end, line): out.append("\n".join(cur)); cur = None
    return out
S = {"laws": {}, "sets": {}, "rost": {}, "aa": {}, "gate": {}, "pair": {}}
for d in OLD:
    t = "🧪️tests/" if d == "" else "🧪️tests/"
    s = rd(d, "🧪️tests/🔬️surface/🦀️.rs")
    for l in s.split("\n"):
        if l.startswith("surface_law!("): S["laws"][fam(l)] = l
        if l.startswith("roster_examples_case!("): S["rost"][fam(l)] = l
    for b in blocks(s, r"^set_active_example_case!\($", r"^\);$"): S["sets"][fam(b.split("\n")[1])] = b
    for l in rd(d, "🧪️tests/🎯️action-args/🦀️.rs").split("\n"):
        if l.startswith("value_tree_case!("): S["aa"][fam(l)] = l
    g = rd(d, "🧪️tests/🚦️compliance-gate/🦀️.rs")
    for b in blocks(g, r"^    results\.push\(", r"^    \)\);$"): S["gate"][re.search(r'^        "(\w+)",', b, re.M).group(1)] = b
    a = rd(d, "🖥️app-surface/🧪️tests/🖥️app-surface/🦀️.rs" if d == "" else "🧪️tests/🖥️app-surface/🦀️.rs")
    for l in a.split("\n"):
        m = re.match(r"\s+semio_s_artifact_norm_(\w+?) =>", l)
        if m: S["pair"][m.group(1)] = l.strip()
    if d == "":
        g0 = g.split("\n")
        i = next(i for i, l in enumerate(g0) if l.startswith("/// 🚫 Forbidden")); j = next(i for i, l in enumerate(g0) if l.startswith("include!"))
        scan = "\n".join(g0[i:j]).rstrip() + "\n"
for k in S: assert len(S[k]) == 15, (k, len(S[k]))
def pick(k, fams): return [S[k][f] for f in fams]
UP = {"": "../", "ext": "../../../../"}
for d, fams in NEW.items():
    ext = d != ""
    up = UP["ext"] if ext else UP["norm"] if False else ("../../../../" if ext else "../")
    root = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts" if ext else "../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts"
    pre = "🧩️extensions/" + d + "/" if ext else ""
    P = lambda x: wr(d, x[0], x[1])
    wr(d, "🧪️tests/🔬️surface/🦀️.rs",
       "//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).\n\n"
       f'const ARTIFACTS_ROOT: &str = "{root}";\n\ninclude!("{up}{"🧪️tests/" if ext else ""}🧰️laws/🔬️surface/🦀️.rs");\n\n'
       + "\n".join(pick("laws", fams)) + "\n\n" + "\n".join(pick("sets", fams)) + "\n\n" + "\n".join(pick("rost", fams)) + "\n")
    wr(d, "🧪️tests/🎯️action-args/🦀️.rs",
       "//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them\n//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema\n//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`/`setSnapshot`. Ticket 26/09/23 slice S19.\n\n"
       f'include!("{up}{"🧪️tests/" if ext else ""}🧰️laws/🎯️action-args/🦀️.rs");\n\n' + "\n".join(pick("aa", fams)) + "\n")
    harness = f'include!("{"../../../../🧪️tests/🚦️compliance-gate/🧰️harness/🦀️.rs" if ext else "🧰️harness/🦀️.rs"}");'
    head = "//! 🚦️ Compliance gate — examples, locales, paths, remedies, annex divergence for the families this plugin owns.\n\n#![allow(clippy::type_complexity)]\n\n"
    n = len(fams)
    body = ("    let gaming = scan_evaluate_gaming();\n    if !gaming.is_empty() {\n        panic!(\"evaluate-path gaming source-scan failed ({} hits):\\n{}\", gaming.len(), gaming.join(\"\\n\"));\n    }\n\n" if not ext else "")
    test = ("#[test]\nfn compliance_gate_all_families() {\n" + body + "    let mut results = Vec::new();\n\n" + "\n".join(pick("gate", fams)) + "\n\n"
        "    let mut summary = String::new();\n    let mut failed = 0usize;\n    for result in &results {\n        let _ = writeln!(summary, \"{}\", result.summary());\n        if !result.ok() {\n            failed += 1;\n        }\n    }\n"
        f"    assert_eq!(results.len(), {n}, \"gate must cover all {n} families this plugin owns\");\n    if failed > 0 {{\n        panic!(\"{{failed}}/{n} families failed compliance gate:\\n{{summary}}\");\n    }}\n}}\n")
    leak = "/// 🖨️ Leak a printed DSL document for gate example registration.\nfn leaked_print_dsl<D: pack::snapshot::ArtifactDsl>(doc: &D) -> &'static str {\n    Box::leak(doc.print_dsl().into_boxed_str())\n}\n\n" if "en1998" in fams else ""
    if not ext:
        gate = head + "use semio_framework_test_fixtures::fixture_path;\nuse std::fmt::Write as _;\nuse std::path::{Path, PathBuf};\n\n" + scan + "\n" + harness + "\n\n" + test
    else:
        gate = head + "use std::fmt::Write as _;\n\n" + harness + "\n\n" + leak + test
    wr(d, "🧪️tests/🚦️compliance-gate/🦀️.rs", gate)
    pairs = "\n".join("            " + S["pair"][f] for f in fams)
    inc = "../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs" if ext else "../🧰️shared/🦀️.rs"
    app = ("//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.\n\n"
        f"use {CRATE[d]}::plugin;\n\nconst VARIANTS: &[&str] = &[{', '.join(chr(34)+f+chr(34) for f in fams)}];\n\n"
        "macro_rules! assert_family_pairs {\n    () => {\n        assert_norm_pair! {\n" + pairs + "\n        }\n    };\n}\n\n" + f'include!("{inc}");\n')
    wr(d, "🧪️tests/🖥️app-surface/🦀️.rs" if ext else "🖥️app-surface/🧪️tests/🖥️app-surface/🦀️.rs", app)
