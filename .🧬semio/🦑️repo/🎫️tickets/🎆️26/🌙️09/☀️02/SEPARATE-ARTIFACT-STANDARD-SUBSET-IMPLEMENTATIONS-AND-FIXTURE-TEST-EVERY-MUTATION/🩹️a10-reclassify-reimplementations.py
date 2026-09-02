#!/usr/bin/env python3
# 🩹️ A10 — reclassify the 20 reimplementation-registered-as-third-party oracle entries to
# cross-semio-implementation, honestly. Each of these owners' 🦀️oracle.rs computes the mutation's
# expected RESULT itself (an `apply`/`apply_kind` dispatch whose catch-all arm is literally
# "mutation kind {other:?} has no oracle implementation" -- the exact anti-pattern the breach rule
# documents) while the named crate only parses/encodes bytes. Confirmed by direct reading of all 20
# files -- see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/.../📓️a10-oracle-honesty.md.
import json, collections

RECLASSIFY = [
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "html5ever-html-5-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "zip-2-0-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🔣️.json", "zip-2-0-iso21320-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "pptx-ecma-376-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json", "pptx-ecma-376-strict-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json", "pptx-ecma-376-transitional-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️oracle/🔣️.json", "quick-xml-svg-1-1-basic-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🔣️.json", "quick-xml-svg-1-1-tiny-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "ruststep-ifc-2x3-any-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧪️oracle/🔣️.json", "ruststep-ifc-2x3-cobie-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧪️oracle/🔣️.json", "ruststep-ifc-2x3-cv20-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧪️oracle/🔣️.json", "ruststep-ifc-2x3-sav-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "ruststep-ifc-4-any-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "ruststep-step-ap214-any-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "xlsx-ecma-376-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json", "xlsx-ecma-376-strict-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json", "xlsx-ecma-376-transitional-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🔣️.json", "docx-ecma-376-strict-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧪️oracle/🔣️.json", "docx-ecma-376-transitional-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧪️oracle/🔣️.json", "quick-xml-xml-1-0-valid-mutate"),
]

NOTE = (
    "🩹️ RECLASSIFIED (ticket SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION, "
    "shard A10, {date}). This entry was registered `third-party-library` on the strength of a real crate that "
    "does genuine codec work, but the mutation's expected RESULT is computed by this owner's own "
    "`🦀️oracle.rs` -- its dispatch function's catch-all arm is `\"mutation kind {{...}} has no oracle "
    "implementation\"`, proof that every OTHER arm is an implementation, not a reading. Both halves of the "
    "comparison then descend from the same specification, so a shared misreading produces two agreeing wrong "
    "answers -- the one failure a differential oracle exists to catch. This is a `cross-semio-implementation`: "
    "a required SUPPLEMENT (metamorphic/inverse/round-trip evidence), never a substitute. A qualifying "
    "third-party reference for the mutation semantics is still owed. See "
    "$TICKET/📓️a10-oracle-honesty.md."
)

import datetime
today = datetime.date.today().isoformat()

changed = []
for path, oracle_id in RECLASSIFY:
    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
    found = False
    for oracle in data.get("oracles", []):
        if oracle["id"] == oracle_id:
            found = True
            old_kind = oracle.get("kind")
            old_caps = list(oracle.get("capabilities", []))
            new_caps = [f"{c}-second-implementation" for c in old_caps]
            oracle["kind"] = "cross-semio-implementation"
            oracle["capabilities"] = new_caps
            oracle["rationale"] = NOTE.format(date=today) + "\n\n" + oracle.get("rationale", "")
            changed.append((path, oracle_id, old_kind, old_caps, new_caps))
    if not found:
        raise SystemExit(f"oracle {oracle_id} not found in {path}")
    with open(path, "w", encoding="utf-8") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=2)
        fh.write("\n")

for c in changed:
    print(c[1], "|", c[2], "->", "cross-semio-implementation", "|", c[3], "->", c[4])
print(f"\n{len(changed)} oracle entries reclassified.")
