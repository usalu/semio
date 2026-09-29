"""🧬️ S20 pass 2 on the p2 overlay — first compile round (`p2-framework-1`): the shared map delta (Rust apply, its neutral
fixture and the TS twin) names frozen codes: a missing required entry → `mutation.target-missing`, an entry that must be
absent → `mutation.duplicate-id`, an unsatisfiable composed precondition → `mutation.invariant`. Idempotent.
Usage: python3 p2-fixups-1.py"""
from pathlib import Path

MAP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map")
EDITS = [
    ("🦀️.rs", """                let (code, message) = match entry.precondition {
                    MapPresence::Present => ("mutation.apply.missing-target", "required map entry does not exist"),
                    MapPresence::Absent => ("mutation.apply.target-precondition", "map entry must be absent"),
                    _ => ("mutation.apply.unsatisfiable-precondition", "composed map entry has no accepted base"),
                };
                return Err(MutationApplyError::new(code, message).at([key.as_str()]));""",
     """                let code = match entry.precondition {
                    MapPresence::Present => MutationCode::TargetMissing,
                    MapPresence::Absent => MutationCode::DuplicateId,
                    _ => MutationCode::Invariant,
                };
                return Err(MutationApplyError::new(code).at([key.as_str()]));"""),
    ("🧪️tests/🧪️shared-map-delta-native/🦀️.rs", "assert_eq!(outcome.unwrap_err().code, code, ", "assert_eq!(outcome.unwrap_err().code.as_str(), code, "),
    ("🧬️schema/🟦️.ts", '? "mutation.apply.unsatisfiable-precondition" : entry.precondition === "present" ? "mutation.apply.missing-target" : "mutation.apply.target-precondition"',
     '? "mutation.invariant" : entry.precondition === "present" ? "mutation.target-missing" : "mutation.duplicate-id"'),
    ("🦀️.rs", "use super::{MutationApplyError, MutationApplyResult};", "use super::{MutationApplyError, MutationApplyResult, MutationCode};"),
    ("🧫️fixtures/🔣️.json", None, None),
]
for rel, old, new in EDITS:
    path = MAP / rel
    text = path.read_text()
    if old is None:
        replaced = text.replace('"error": "mutation.apply.missing-target"', '"error": "mutation.target-missing"').replace('"error": "mutation.apply.target-precondition"', '"error": "mutation.duplicate-id"').replace('"error": "mutation.apply.unsatisfiable-precondition"', '"error": "mutation.invariant"')
        if replaced != text:
            path.write_text(replaced)
            print("fixture codes rewritten")
        continue
    if new in text:
        continue
    assert text.count(old) == 1, rel
    path.write_text(text.replace(old, new))
    print("fixed", rel)
