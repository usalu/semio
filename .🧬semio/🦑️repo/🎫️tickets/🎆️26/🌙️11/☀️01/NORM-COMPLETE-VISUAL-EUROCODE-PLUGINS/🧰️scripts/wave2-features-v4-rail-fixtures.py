"""Wave 2 features v4 (input script): writes the hand-made EN 1990 documents of `🧪️tests/🚄️rail-and-overrides-en1990-1` under `🧫️fixtures/🚄️rail-and-overrides/<id>/🔣️.json` from the committed insert-factor-override before-snapshot. Usage: python -I wave2-features-v4-rail-fixtures.py (repo root)."""
import copy
import json
import pathlib

TESTS = pathlib.Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests")
BASE = json.load(open(TESTS / "🧫️fixtures/🧬️mutations/🎚️insert-factor-override/✅apply/🔣️.before.json", encoding="utf8"))
JUSTIFICATION = "project specific agreement with the authority"


def document(annex="De", member=None, override=None):
    out = copy.deepcopy(BASE)
    out["annex"] = annex
    out["factorOverrides"] = [] if override is None else [dict(id="O1", justification=JUSTIFICATION, **override)]
    out["members"][0].update(member or {})
    return out


FIXTURES = {
    "override-psi0-de": document("De", override=dict(factor="psi0", target="Q-wind", value=0.3)),
    "override-gamma-en": document("En", override=dict(factor="gamma", target="Q-office", value=1.2)),
    "override-gamma-de": document("De", override=dict(factor="gamma", target="Q-office", value=1.2)),
    "override-kfi-en": document("En", override=dict(factor="k_fi", target="", value=1.0)),
    "material-fail": document(member=dict(materialStrengthK=355e6, materialConversionFactor=0.9, materialPartialFactor=1.1, materialDesignStrength=300e6)),
    "material-pass": document(member=dict(materialStrengthK=355e6, materialConversionFactor=0.9, materialPartialFactor=1.1, materialDesignStrength=280e6)),
    "geometry-fail": document(member=dict(nominalDimension=0.3, dimensionTolerance=0.005, designDimension=0.304)),
    "geometry-pass": document(member=dict(nominalDimension=0.3, dimensionTolerance=0.005, designDimension=0.31)),
    "analysis-fail": document(member=dict(memberAlphaCr=8.0, secondOrderAnalysis=0.0)),
    "analysis-pass": document(member=dict(memberAlphaCr=12.0, secondOrderAnalysis=0.0)),
    "analysis-second-order": document(member=dict(memberAlphaCr=8.0, secondOrderAnalysis=1.0)),
}
for name, body in FIXTURES.items():
    target = TESTS / "🧫️fixtures/🚄️rail-and-overrides" / name / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(body, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
print(len(FIXTURES), "fixtures")
