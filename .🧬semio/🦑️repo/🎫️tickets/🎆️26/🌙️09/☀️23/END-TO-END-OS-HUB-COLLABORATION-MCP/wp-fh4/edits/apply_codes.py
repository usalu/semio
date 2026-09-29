"""🛡️ After S20's apply codemod (`MutationApplyError { code: MutationCode, target }`) my families' laws still compared the
rejection's code to its old string, and the TS twins, oracle notes, generator catalogues and doc comments still named the
`mutation.apply.*` / bare `invalid-*` codes: the laws assert the `MutationCode` variant, the rest names the frozen code the
Rust rejection now carries (S20's mapping: missing-* → TargetMissing, duplicate-* → DuplicateId, every other → Invariant)."""
import re

LAW_FILES = [
    "🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
    "📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs",
    "🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧬️direct-leaves/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧰️triples/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️result-apply/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️result-apply/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️result-apply/🦀️.rs",
    "🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs",
]
LAW = re.compile(r'(\.code), "((?:mutation\.apply\.|mutation\.absorb\.|mutation\.inverse\.)?(?:missing|duplicate|invalid|conflicting|kind)[a-z-]*)"\)')


def variant(code: str) -> str:
    """🛡️ S20's apply mapping, read off the reason after the namespace."""
    reason = code.split(".")[-1]
    return "TargetMissing" if reason.startswith("missing") else "DuplicateId" if reason.startswith("duplicate") else "Invariant"


def code_laws(text: str) -> str:
    """⚖️ `assert_eq!(error.code, "<old string>")` → `assert_eq!(error.code, protocol::MutationCode::<variant>)`."""
    return LAW.sub(lambda match: f"{match.group(1)}, protocol::MutationCode::{variant(match.group(2))})", text)


APPLY_CODES = {code: variant(code) for code in (
    "mutation.apply.missing-target", "mutation.apply.duplicate-target", "mutation.apply.conflicting-target", "mutation.apply.invalid-order",
    "mutation.apply.invalid-index", "mutation.apply.invalid-add-key", "mutation.apply.invalid-add-index", "mutation.apply.invalid-remove-index",
    "mutation.apply.invalid-modify-key", "mutation.apply.kind-mismatch", "mutation.apply.invalid-number")}

TRANSFORMS = [(path, code_laws) for path in LAW_FILES]
RENAMES = [("🗄️stdio/🗿️artifacts/🎒️zip", APPLY_CODES), ("🗄️stdio/🗿️artifacts/📼️avi", APPLY_CODES), ("🗄️stdio/🗿️artifacts/📜️docx", APPLY_CODES)]
DOC_RENAMES = [("🖨️raster/🗿️artifacts/🖨️raster", APPLY_CODES), ("🗄️stdio/🗿️artifacts/🧿️semio", APPLY_CODES), ("🗄️stdio/🗿️artifacts/🌐️html", APPLY_CODES),
               ("🗄️stdio/🗿️artifacts/📼️avi", APPLY_CODES)]
