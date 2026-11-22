# Nakagin child projection

Native example activation reached child restoration and refused `InvalidSlot` on the authored Nakagin document. The typed snapshot declares `kindCatalogs` as a single owned child of kind `s.stdio.semio`. `Puzzle5dPlaySnapshot` forwarded child reference traversal to its typed root while inheriting the empty slot declaration. Derivable restoration looks up each visited child in the declared slot table, so the catalog reference could never pass the wrapper.

The 5D play snapshot now forwards both child and link slot tables to the typed root, matching 3D. The fast native regression parses the shipped language-agnostic Nakagin DSL and checks that its one catalog kind is admitted by the play snapshot’s slot, with both slot tables equal to the typed declarations. The existing example switch law exercises the same schema through `setActiveExample`, undo, and redo. An independent SQLite query over the authored DSL lines found exactly one owned catalog of the kind `s.stdio.semio`.

The implementation and regression are in `🧬️schema/🎞️play-snapshot/🦀️.rs` and `✏️editor/🧪️tests/🔬️unit/☑️option-groups-and-edits/🦀️.rs`, respectively. The production snapshot belongs to `semio-s-artifact-puzzle-5d-mut-any-5`.
