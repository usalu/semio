# r11-store-step: semio-s-artifact-stdio-step

Result: gate (cargo check --lib via bim-model) shows 0 errors in the step crate (was 34).

Change: added `semio_framework_value::RetireOwned` (first in the derive list) to the three ladder leaf types that mutation payloads embed, in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪜️ladder/🦀️.rs`:
`ShapeRepresentationRow`, `ProductIdentity`, `EntityRestore`. Snapshot types already implement RetireOwned via `artifact_retire_struct!`.
No InteractiveJob or ArtifactApp hooks live in this crate.

Gate logs: `🗑️generated/r11-store-step/c1.txt` (before), `c2.txt` (after). Leftovers: none in step; svg (1 error) and pdf (8) belong to siblings.
