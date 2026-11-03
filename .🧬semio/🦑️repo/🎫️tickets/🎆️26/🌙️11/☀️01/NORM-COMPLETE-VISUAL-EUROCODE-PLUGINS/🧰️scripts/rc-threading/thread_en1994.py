import sys
sys.path.insert(0, ".")
from rc_lib import *
import rc_lib
R = ART + "/🧩️en1994" + SUBSET
S = R + "/🧬️schema"
RCT = "semio_s_plugin_norm_compliance::ReliabilityClass"
NAMES = "uls_composite|uls_construction|uls_column|fire_column|fire_combination|stage_effects|sls_characteristic|sls_frequent|sls_quasi_permanent"

def call_ends(t, pattern):
    for m in re.finditer(pattern, t):
        o = m.end() - 1
        yield m, o, balanced(t, o, "(", ")") - 1

def split_args(a):
    out, d, cur = [], 0, ""
    for c in a:
        if c in "([{":
            d += 1
        elif c in ")]}":
            d -= 1
        if c == "," and d == 0:
            out.append(cur)
            cur = ""
        else:
            cur += c
    out.append(cur)
    return out

def rewrite_calls(t, pattern, annex_arg_to_class, skip_fn=True):
    """Insert the class argument after the annex-like argument of every call matching `pattern`."""
    out, last = [], 0
    for m, o, c in call_ends(t, pattern):
        ls = t.rfind("\n", 0, m.start()) + 1
        if re.search(r"\bfn\s+\w*$", t[ls:m.start()].rstrip() + t[m.start():m.end()-1][:0]) or re.match(r"\s*(pub(\([a-z]+\))? )?fn ", t[ls:m.end()]):
            continue
        args = split_args(t[o + 1:c])
        if any("reliability_class" in a for a in args):
            continue
        idx = next((i for i, a in enumerate(args) if a.strip() in annex_arg_to_class), None)
        assert idx is not None, (t[m.start():c + 1][:160])
        cls = annex_arg_to_class[args[idx].strip()]
        args.insert(idx + 1, " " + cls)
        out.append(t[last:o + 1])
        out.append(",".join(args))
        last = c
    out.append(t[last:])
    return "".join(out)

def module():
    p = S + "/💡️inferences/⚖️checks/🦀️.rs"
    t = rd(p)
    if "reliability_class" in t:
        return
    t = t.replace('''    /// ⚖️ The EN 1990 partial and combination factors of the annex from the registry (`ActionFactors`); the reliability class is RC2 (K_FI = 1.0) until the document stores a project-level class.
    pub fn factors(annex: AnnexChoice) -> semio_s_plugin_norm_compliance::ActionFactors {
        semio_s_plugin_norm_compliance::ActionFactors::new(annex, semio_s_plugin_norm_compliance::ReliabilityClass::default(), semio_s_plugin_norm_compliance::DesignSituation::Persistent)
    }''', f'''    /// ⚖️ The EN 1990 partial and combination factors of the annex and the reliability class of the document from the registry (`ActionFactors`): K_FI scales γ_G and γ_Q.
    pub fn factors(annex: AnnexChoice, reliability_class: {RCT}) -> semio_s_plugin_norm_compliance::ActionFactors {{
        semio_s_plugin_norm_compliance::ActionFactors::new(annex, reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent)
    }}''')
    old = 'let row = |annex: AnnexChoice| factors(annex).psi_known(key).or_else(|| factors(annex).psi_known("office")).map_or('
    assert old in t
    t = t.replace(old, 'let row = |annex: AnnexChoice| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, semio_s_plugin_norm_compliance::action_factors::canonical_category(key)).or_else(|| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, "office")).map_or(')
    for fn_ in ["gamma_g", "gamma_q"]:
        t = t.replace(f"pub fn {fn_}(annex: AnnexChoice) -> f64 {{\n        factors(annex)", f"pub fn {fn_}(annex: AnnexChoice, reliability_class: {RCT}) -> f64 {{\n        factors(annex, reliability_class)")
    for name in ["design_permanent_internals", "accumulate_column", "uls_column", "fire_column", "accumulate", "uls_composite", "uls_construction", "sls_characteristic", "sls_frequent", "sls_quasi_permanent", "stage_effects", "fire_combination"]:
        pat = re.compile(r"(\bfn " + name + r"\([^\n]*?annex: AnnexChoice)")
        assert len(pat.findall(t)) == 1, name
        t = pat.sub(lambda m: m.group(1) + f", reliability_class: {RCT}", t)
    t = rewrite_calls(t, r"(?<![\w:])(?:gamma_g|gamma_q|factors|design_permanent_internals|accumulate_column|accumulate)\(", {"annex": "reliability_class"})
    t = rewrite_calls(t, r"\bfactors\(", {"annex": "reliability_class"}) if "factors(annex)" in t else t
    wr(p, t)

def externals():
    pat = r"part_en1990::(?:gamma_g|gamma_q|" + NAMES + r")\("
    site = {
        "🎬️scene/🧮️facts/🦀️.rs": {"annex": "document.reliability_class", "document.annex": "document.reliability_class"},
        "🎬️scene/🔥️fire-section/🦀️.rs": {"f.annex": "f.reliability_class"},
        "🎬️scene/📈️diagrams/🦀️.rs": {"f.annex": "f.reliability_class"},
        "🎬️scene/🌈️fields/🦀️.rs": {"f.annex": "f.reliability_class"},
        "🧬️schema/💡️inferences/🧮️compliance/🔥️fire-and-bridge/🦀️.rs": {"a": "doc.reliability_class", "annex": "doc.reliability_class"},
        "🧬️schema/💡️inferences/🧮️compliance/🏛️columns/🦀️.rs": {"a": "doc.reliability_class"},
        "🧬️schema/💡️inferences/🧮️compliance/🧱️slabs/🦀️.rs": {"a": "doc.reliability_class", "annex": "reliability_class"},
        "🧬️schema/💡️inferences/🧮️compliance/➕️extensions/🦀️.rs": {"annex": "doc.reliability_class"},
        "🧬️schema/💡️inferences/🧮️compliance/🏗️beams/🦀️.rs": {"a": "c.reliability_class", "annex": "doc.reliability_class", "c.annex": "c.reliability_class"},
    }
    for rel, mp in site.items():
        p = R + "/" + rel
        t = rd(p)
        new = rewrite_calls(t, pat, mp)
        if new != t:
            wr(p, new)

def structs():
    f = R + "/🎬️scene/🧮️facts/🦀️.rs"
    t = rd(f)
    if "reliability_class" not in t:
        t = t.replace("    pub annex: AnnexChoice,\n", f"    pub annex: AnnexChoice,\n    pub reliability_class: {RCT},\n")
        t = t.replace("            annex,\n", "            annex,\n            reliability_class: document.reliability_class,\n")
        t = t.replace("annex: document.annex, l_total", "annex: document.annex, reliability_class: document.reliability_class, l_total")
        wr(f, t)
    b = S + "/💡️inferences/🧮️compliance/🏗️beams/🦀️.rs"
    t = rd(b)
    if "reliability_class" not in t:
        t = t.replace("struct BeamCtx<'a> {\n    annex: AnnexChoice,\n", f"struct BeamCtx<'a> {{\n    annex: AnnexChoice,\n    reliability_class: {RCT},\n", 1)
        t = t.replace("    let ctx = BeamCtx {\n        annex,\n", "    let ctx = BeamCtx {\n        annex,\n        reliability_class: doc.reliability_class,\n", 1)
        wr(b, t)

def slabs():
    p = S + "/💡️inferences/🧮️compliance/🧱️slabs/🦀️.rs"
    t = rd(p)
    if "fn p_g(annex: AnnexChoice, " in t:
        return
    t = t.replace("fn p_g(annex: AnnexChoice) -> f64 {\n    part_en1990::gamma_g(annex", f"fn p_g(annex: AnnexChoice, reliability_class: {RCT}) -> f64 {{\n    part_en1990::gamma_g(annex")
    t = t.replace("fn p_q(annex: AnnexChoice) -> f64 {\n    part_en1990::gamma_q(annex", f"fn p_q(annex: AnnexChoice, reliability_class: {RCT}) -> f64 {{\n    part_en1990::gamma_q(annex")
    t = re.sub(r"\bp_g\(a\)", "p_g(a, doc.reliability_class)", t)
    t = re.sub(r"\bp_q\(a\)", "p_q(a, doc.reliability_class)", t)
    t = t.replace("part_en1990::gamma_g(annex)", "part_en1990::gamma_g(annex, reliability_class)").replace("part_en1990::gamma_q(annex)", "part_en1990::gamma_q(annex, reliability_class)")
    wr(p, t)

def run(dry):
    rc_lib.DRY = dry
    module()
    structs()
    slabs()
    externals()
    for q in CHANGED:
        show(q)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
