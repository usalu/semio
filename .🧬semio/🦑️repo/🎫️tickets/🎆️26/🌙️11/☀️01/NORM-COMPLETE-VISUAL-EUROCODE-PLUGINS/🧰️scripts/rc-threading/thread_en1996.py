import sys
sys.path.insert(0, ".")
from rc_lib import *
import rc_lib

R = ART + "/🪨️en1996" + SUBSET
S = R + "/🧬️schema"
CHAIN = [
    S + "/📐️masonry-formulas/🔥️fire-and-design-effects/🦀️.rs",
    S + "/⚖️masonry/🧱️evaluate-wall/🦀️.rs",
    S + "/⚖️masonry/🧱️evaluate-wall/🔥️fire-durability-and-reinforcement/🦀️.rs",
    S + "/⚖️masonry/🧱️evaluate-wall/🏋️load-case-checks/🦀️.rs",
    S + "/📇️masonry-catalogue/🧮️document-cases/🦀️.rs",
    S + "/📇️masonry-catalogue/🧮️document-cases/🧰️members/🦀️.rs",
]
RCP = "reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass"
KEEP = [r"matches!\(default_situation,", r"for_document\(annex, masonry_class, default_situation\)", r"matches!\(default_sit,"]


def thread_chain(path):
    t = rd(path)
    if "reliability_class" in t:
        return
    for i, k in enumerate(KEEP):
        t = re.sub(k, lambda m, i=i: m.group(0).replace("default_situation", f"@@KEEP{i}@@").replace("default_sit", f"@@KEEP{i}b@@"), t)
    t = re.sub(r"default_situation: DesignSituation\b", "default_situation: DesignSituation, " + RCP, t)
    t = re.sub(r"default_sit: DesignSituation\b", "default_sit: DesignSituation, " + RCP, t)
    t = re.sub(r"\bdefault_situation(?=[,)])(?!: )", "default_situation, reliability_class", t)
    t = t.replace(", reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, reliability_class", ", reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass")
    t = re.sub(r"@@KEEP(\d)@@", "default_situation", t)
    t = re.sub(r"@@KEEP(\d)b@@", "default_sit", t)
    t = re.sub(r"\bgamma_q\(annex, exceptional\)", "gamma_q(annex, reliability_class, exceptional)", t)
    t = re.sub(r"\baction_factors\(annex, (false|exceptional)\)", r"action_factors(annex, reliability_class, \1)", t)
    wr(path, t)


def run(dry):
    rc_lib.DRY = dry
    for p in CHAIN:
        thread_chain(p)
    f = CHAIN[0]
    t = rd(f)
    if "reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, exceptional" not in t:
        t = t.replace("pub fn action_factors(annex: AnnexChoice, exceptional: bool)", "pub fn action_factors(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, exceptional: bool)")
        t = t.replace("semio_s_plugin_norm_compliance::ReliabilityClass::default(), if exceptional", "reliability_class, if exceptional")
        t = t.replace("pub fn gamma_q(annex: AnnexChoice, exceptional: bool)", "pub fn gamma_q(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, exceptional: bool)")
        t = re.sub(r"(/// ⚖️ The EN 1990 partial and combination factors of the annex from the registry \(`ActionFactors`\): persistent, or accidental when `exceptional`;) the reliability class is RC2[^\n]*\n", r"\1 the reliability class of the document scales γ by K_FI.\n", t)
        wr(f, t)
    # conformance + public entry points
    c = S + "/⚖️masonry/🛡️conformance-and-integrity/🦀️.rs"
    t = rd(c)
    if "reliability_class" not in t:
        t = re.sub(r"(    design_situation: DesignSituation,\n)(    storeys: u32,)", r"\1    reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,\n\2", t)
        t = t.replace("evaluate_scoped(annex, masonry_class, design_situation, storeys, walls, Scope::All)", "evaluate_scoped(annex, masonry_class, design_situation, reliability_class, storeys, walls, Scope::All)")
        t = t.replace("evaluate_wall(annex, masonry_class, design_situation, storeys, wall)", "evaluate_wall(annex, masonry_class, design_situation, reliability_class, storeys, wall)")
        wr(c, t)
    n = S + "/📚️masonry-tables/📚️normative-tables/🦀️.rs"
    sub_once(n, r"\.then\(\|\| action_factors\(annex, false\)\.psi_known\(&key\)\)", ".then(|| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, &key))", marker="psi_row_of_key(annex, &key)")
    p = R + "/🎬️scene/🧮️profile/🦀️.rs"
    t = rd(p)
    if "reliability_class" not in t:
        t = t.replace("pub fn effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default: DesignSituation) -> Vec<Effects> {\n    m::design_effects(annex, wall, lc, default)", "pub fn effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<Effects> {\n    m::design_effects(annex, wall, lc, default, reliability_class)")
        t = t.replace("effects(document.annex, wall, lc, document.design_situation)", "effects(document.annex, wall, lc, document.design_situation, document.reliability_class)")
        t = t.replace("m::gamma_q(document.annex, exceptional(lc, document.design_situation))", "m::gamma_q(document.annex, document.reliability_class, exceptional(lc, document.design_situation))")
        wr(p, t)
    for q in [S + "/💡️inferences/🦀️.rs"]:
        sub_once(q, r"(        document\.design_situation,\n)(        document\.storeys,)", r"\1        document.reliability_class,\n\2", marker="document.reliability_class,")
    for dirpath, _, names in os.walk(R):
        for nme in names:
            if not nme.endswith(".rs"):
                continue
            path = f"{dirpath}/{nme}"
            t = open(path, encoding="utf-8").read()
            if "evaluate_building(" not in t and "document_cases(" not in t:
                continue
            new = re.sub(r"(\b(\w+)\.design_situation,\s*)(\2\.storeys)", r"\1\2.reliability_class, \3", t)
            if new != t and "/📚️normative-tables/" not in path:
                wr(path, new)
    for q in CHANGED:
        show(q)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
