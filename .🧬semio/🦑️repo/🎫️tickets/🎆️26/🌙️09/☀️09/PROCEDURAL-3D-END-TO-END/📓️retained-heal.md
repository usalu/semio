# Retained Flow BRep Heal

`brep.util.heal` admits an original-funded geometry job. The job walks the selected solid, checks every vertex against its incident curve endpoints, and journals coordinate repairs within the caller’s tolerance. It rejects divergent endpoints, out-of-tolerance repairs, and edits to vertices owned by another solid. Selected validation reads the journal when checking triangular intersections. The job publishes the existing selected handle after dictionary construction and commits the validated coordinates as one step. Cancellation retires the journal and reachability storage before publication. Tessellation cache entries carry the original shape revision so a repair invalidates stale preview data during publication.

The retained repair covers displaced vertices with consistent incident curve geometry. It keeps the input handle and preserves unrelated solids. Remaining repair work includes loop gap and topology reconstruction, sliver removal, and unsupported curved validations for uncertified input. A publication can carry up to sixteen vertex edits.

The portable fixture describes a displaced corner and two tolerance thresholds. Both Three.js tests passed, covering the restored corner and the earlier scope fixture. The Flow integration test drives the original registry with one work unit per turn and tracks heap receipts, cancellation before publication, out-of-tolerance refusal, and a real coordinate repair. Its first invocation stopped at the test crate’s source-size boundary. A dedicated Cargo integration target passed source admission and compiled the BRep validation and engine crates. Its 96 GiB Rust cache image reached 0 GiB free before Flow linkage, including after Cargo’s managed retry. The narrower linked native BRep test passed in 0.23 seconds. It checked one-turn heap receipts, private coordinates before commit, underfunded publication refusal, the restored selected corner, and the unchanged corner on the unrelated solid. Source admission preflight passed for Wasm, macOS, Windows, and Linux.

- Repair cursor `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🩹️solid-healing/🦀️.rs`
- Original Flow job `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves/🦀️.rs`
- Native funded repair test `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🔬️unit/🦀️.rs`
- Flow integration test `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🩹️retained-heal.rs`
- Independent oracle `🔍️retained-heal-oracle.test.ts`
