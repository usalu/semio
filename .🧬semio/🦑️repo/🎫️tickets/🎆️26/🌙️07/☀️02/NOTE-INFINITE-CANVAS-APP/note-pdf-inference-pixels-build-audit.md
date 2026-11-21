# Note guest PDF inference dependency audit

Attempt 46 (driver `wQgb47c`) stopped in `semio_s_artifact_stdio_pdf_inferences` with E0432 at the PNG scanline decoder import. Cargo’s initial `.cache/b/r/m/components/runs/wQgb47c/l/unit-graph-prototype-00.json` recorded its inference crate as unit 692 at 11:21:39 local time on 2026-11-21. That unit has no `semio_framework_pixels` edge. The matching diagnostic is in `cargo-prototype-00.jsonl` alongside the graph.

The PDF foreign-artifact source, inference manifest and Hub lock arrived through the shared worktree at 12:05:14 local time. The current dependency path resolves to the named crate, and the inferred Cargo import is backed by an unconditional public reexport.

- [PDF foreign-artifact decoder import](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📦️foreign-artifacts/🦀️.rs#L329)
- [PDF inference manifest dependency](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml#L19)
- [Framework pixels public decoder reexport](../../../../../../../🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/🦀️.rs#L3)
- [Hub Cargo lock](../../../../../../../🌎️hub/Cargo.lock) records `semio-framework-pixels` under `semio-s-artifact-stdio-pdf-inferences`.

This audit verified the original graph and the current files without changing product sources or starting another native build. A Cargo generation based on the current files still needs to compile the PDF inference crate.
