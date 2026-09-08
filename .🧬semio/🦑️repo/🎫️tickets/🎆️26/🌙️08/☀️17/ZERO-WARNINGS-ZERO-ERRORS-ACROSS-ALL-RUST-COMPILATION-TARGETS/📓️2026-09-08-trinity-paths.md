# Trinity Artifact Paths

Native532 reported renamed framework crate identifiers embedded in Trinity's internal artifact paths. Current source had already received the schema-path corrections from concurrent work. Pass535 corrected the remaining geometry and graph paths in the Jack root and its editor, and removed the invalid early super::snapshot reexport. The existing canonical JackSnapshot reexport remains, with its documentation attached there.

The root's Stdio paths now name the actual base/schema/geometry and graph/schema/snapshot modules. The editor paths name its existing crate-visible edit/windows/graph module. Framework crate paths remain unchanged.

GIS failures from native532 were compared with its captured Cargo metadata: that run loaded an unrenamed framework-schema dependency and no surface dependency. Current GIS source already supplies the schema alias, optional surface dependency, and app-assembly feature conditions. Those concurrent corrections require a fresh check; no redundant GIS edits were made by this pass.

Changed:
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

Native532 is still a diagnostic run in progress with compilation errors. Neither these edits nor the concurrent GIS corrections have a fresh successful compiler result yet.
