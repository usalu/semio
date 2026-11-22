# Store engine crate split

`semio-os-store-engine` (`🏪️store/⚙️engine/📦️packages/🦀️rust`) was refused by the source-boundary preflight at 13 files / 262,422 bytes (hard ceiling 262,144) and a read-bytes-exceeded on a file in the crate.

## Constraint

`ArtifactStore` is defined in the engine crate, so its inherent `impl` blocks cannot move to another crate. The movable subtree is therefore the derived history / report-replay family: it only touches the six store components (`log`, `owners`, `wire`, `cursor`, `live`) and mutates exactly one thing, `self.log.envelope.vcs.edits`.

## Change

New crate `semio-os-store-engine-derived` at `⚙️engine/🔭️derived-replay` (id `os-store-engine-derived`), one level above the component crates, below the engine, no cycle (depends on engine-core, engine-live, read, replay, replay-operation, ledger, vcs, spr, space-schema, operation-wire, value crates).

- `StoreDerived<'a, P, Mutation, Log>`: a borrowed bundle of the components. `Log = &StoreLog` serves the `&self` reads; `Log = &mut StoreLog` serves the steps and retirements that advance the history ledger (`impl ... Log: DerefMut`). Method bodies moved verbatim; `self.log`, `self.owners` etc. resolve as before and the disjoint field borrows are preserved because the fields are plain references.
- Moved into the new crate (all were `impl ArtifactStore`):
  - `🔭️history-preview`: derived history preview and report replay begin / step / retire, reference snapshots, retained derived reads, history edit read issuer, `funded_plan_retirement` (now in the crate root), plus small accessors (`generation`, `content_revision`, `merge_policy`, ...).
  - `📜️report-replay`: original report replay (was in the replay-operation crate directory but compiled into the engine through `#[path]`).
  - `🧩️operation`: derived operation coordinator (same `#[path]` situation).
  - `📸️checkpoint-read`: checkpoint preview and `CheckpointSource`.
- Engine keeps one-line delegates for every public method (`self.derived().x(..)` / `self.derived_mut().x(..)`), so all public paths and signatures are stable; `CheckpointSource` and `StoreDerived` are re-exported explicitly from `lifted_store`. The dead private helper delegates (`admit_derived_source`, ...) and the imports only the moved code used were removed from the engine; the engine no longer depends on `semio-framework-value-snapshot-read`.
- Removed old files: replay-operation `🌱️derived/🧩️store/🦀️.rs`, `🧬️native/🔗️history/📜️report/🦀️.rs`, engine `📸️checkpoint-read/🦀️.rs`.

## Registration

root `Cargo.toml` and `🧰️framework/Cargo.toml` workspace members, `📋️build-owners.json`, `📋️project.json`, engine `Cargo.toml` dependency, `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` boundary entry.

## Sizes (Rust source in the crate)

| crate | files | bytes |
| --- | --- | --- |
| semio-os-store-engine before | 13 | 262,422 |
| semio-os-store-engine after | 10 | 187,980 (about 184 KiB) |
| semio-os-store-engine-derived (new) | 5 | 86,361 |

The preflight for the store subtree reports no OVER.

## Generators

`store-engine-split.py` generated the moved files and the delegates from the live sources (header rewrite, `&self` / `&mut self` split, visibility); `store-engine-split-prune.py` removed the unused imports reported by rustc.

## Verification

- Preflight (`native-source-boundary-preflight.ts` on the store subtree): no OVER, exit 0.
- `repo cargo --store=shared --configuration=prototype -- check -p semio-os-store-engine -p semio-os-store-engine-derived --message-format=short`: exit 0 (run before the last import prune of `lifted/🦀️.rs` and the removal of the engine's `semio-framework-value-snapshot-read` dependency; both are removals of items rustc reported unused or that no engine source names).
- The same check with `--target wasm32-unknown-unknown` (started after those final edits) was still queued behind another build on the shared target directory / host capacity after more than 35 minutes, with no compile started; a first wasm attempt compiled the 45 upstream crates before hitting the same wait and was cut by its 590 s timeout. It needs a rerun when the host is free (output of the pending run: `/private/tmp/claude-501/derived6.txt`, the final line `exit N` is the result).
- Not run: `--tests` builds of the engine and downstream test crates; call sites outside the engine use the unchanged public methods through the delegates.
