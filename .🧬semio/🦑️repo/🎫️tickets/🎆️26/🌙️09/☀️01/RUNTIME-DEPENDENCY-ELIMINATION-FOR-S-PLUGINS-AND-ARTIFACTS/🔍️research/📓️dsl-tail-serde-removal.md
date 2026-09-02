# The tail: `🗣️dsl` / `💡️inference` / `🧩️extension` / `🌿️vcs` / `⚙️engine` / `🎒️pack`

Scope: the six smallest measured modules under `🧰️framework/🛍️products/💻️os/🔨️modules/`. Method
followed `📓️serde-fanout-playbook.md`: classify production vs `#[cfg(test)]`/oracle first, convert
only production, never touch a generic seam whose blast radius crosses into another agent's
in-flight territory (`🏪️store`, `📇️directory`/`📡️spr`) or into modules outside `🔨️modules/` entirely.

## Modules driven to **zero production serde**

- **`🧩️extension`** — 0 → 0 refs remaining (was ~20). `ExtensionPackageManifest` /
  `PackagePluginDependency` (the `.sxt` package manifest, genuinely written/read as JSON text inside
  a zip entry) had their `serde` derives replaced outright with hand-written `to_json`/`from_json`
  methods over `crate::os_pack::json::{Value, Object, object}` — not `ToValue`/`FromValue`, since the
  two dynamic fields (`topic_contributions`, `contributions`) are genuinely untyped JSON, not
  DslValue-shaped data. `serde_json::to_vec`/`from_slice` → `pack::json::to_string`/`parse_bytes`.
  `ExtensionPackageError::ManifestJson` narrowed from `serde_json::Error` to `String`. All 6 tests in
  the file (including the two that built raw JSON via `serde_json::json!`/`to_value`) converted to
  the same `pack::json` vocabulary — not oracle tests, they assert directly on this crate's own
  wire format.
- **`⚙️engine`** — 0 → 0 remaining (was 3: 1 import + `EngineKey`/`EngineHandle` derives). Both
  derives were dead weight: grepped every call site of `EngineKey`/`EngineHandle` repo-wide, found
  zero `serde_json::`/`to_value`/`from_value` usage anywhere and no struct embeds either type behind
  a `Serialize`-derived container. Deleted the derives and the now-unused `use serde::{..}` import —
  no `ToValue`/`FromValue` added either, since nothing consumes that either (same disposition as
  `DepHash` below).
- **`🗣️dsl`** — 43 production → 0. Two real production sites, both converted:
  - `🧬️schema/🦀️component.rs`: `shape_json_schema`/`record_spec_json_schema`/
    `collect_record_spec_properties` (JSON-Schema-2020-12 generation over `Shape`/`RecordSpec`) —
    rebuilt by hand over `pack::json::{Value, Object, object}` (no derive applies here; it's a
    recursive tree-builder, not a data struct). The `#[cfg(test)] mod json_schema_tests` asserting
    directly on this function's output was converted alongside it (not an oracle — it pins this
    crate's own schema shape).
  - `🧠️lsp/🦀️component.rs`: `handle_json_rpc`/`semantic_tokens_lsp` (LSP 3.17 JSON-RPC) — `serde_json`
    parse/build replaced with `pack::json::parse`/`to_string` + hand-assembled `Object`.
  - Everything else that matched `serde` in this module is **out of scope by inspection, not by
    omission**: the main `🦀️component.rs`'s 21 `#[derive(.., serde::Serialize, serde::Deserialize)]`
    lines are all inside `#[cfg(test)] mod tests { .. }` (opens line 729, no other `mod` before EOF at
    1586); `🧪️tests/🔢️checked-integers` is a `#[cfg(test)]`-gated integration test (decimal-boundary
    oracle); and **`✨️derive`** (the `dsl_derive`/`semio-framework-os-kernel-dsl-derive` proc-macro
    crate) is `proc-macro = true` in its own `Cargo.toml` — confirmed with
    `cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -e normal -i serde_json`, which
    shows `serde_json v1.0.149 └── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro)`. Per the
    ticket's own note, a `(proc-macro)` leaf never links into the `.wasm`; it's a build-time-only
    dependency of a macro that runs on the host during `cargo check`/`build`. **No conversion needed
    or attempted there.** (The same `cargo tree -i` run also shows the other `serde_json` instance
    reaching the graph via `semio-framework-os-kernel` directly and via `semio-framework-replication`
    — both explicitly out of scope: os-kernel's own ~150 direct refs and replication's are separately
    scoped later waves per `📓️verified-outcomes.md`.)

## Modules reduced but **deliberately not driven to zero** — with reasons

- **`💡️inference`** — 9 → 8 production-shaped refs remain. Removed `DepHash`'s
  `Serialize, serde::Deserialize` derive (dead: grepped every `DepHash` use repo-wide, it is only
  ever passed as raw `[u8; 32]`-backed bytes via `DepHash::root`/`::chain`, never through
  `serde_json`/`to_value`, and no container's own derive needs it). **Left untouched, and why**: the
  `InferredField<P>` trait's `type Key: .. + Serialize + DeserializeOwned` / `type Value: .. +
  Serialize + DeserializeOwned` bound, and its `encode`/`decode` helpers
  (`serde_json::to_vec`/`from_slice`). This is a framework seam exactly like `MutationDiff`/
  `Mutation` — flipping it to `ToValue + FromValue` would force every `impl InferredField<P>` to
  convert its `Key`/`Value` associated types. Surveyed all real implementors
  (`grep -rn "impl.*InferredField"`): **~15 concrete types across 8 plugin crates**
  (`stdio` ×6 — `FlattenedNode`, `SemioColumnEntropy`, `SemioColumnMoments`,
  `Vec<BrepValidationDiagnostic>`, `SemioGraphNodeConnectivity`, `SemioAabb`; `procedural`/assembly
  ×3 — `AssemblySolveResult`; `remodel` — `RemodelPoseDelta`; `puzzle` — `FlattenPlane`;
  `mathematical` — `MathematicalRoot`). Two of these (`mathematical`'s `MathematicalRoot`, `stdio`'s
  `SemioAabb`) **already** carry `ToValue`/`FromValue` from other in-flight work, but the rest
  (notably `remodel`'s `RemodelPoseDelta`, and five more in `stdio`) still only have
  `Serialize`/`Deserialize` — flipping the trait bound today would break those crates immediately.
  `stdio` and `procedural` are both explicitly called out in `📓️verified-outcomes.md` as separate
  large deferred waves (563 and ~1277 sites respectively) owned outside this ticket slice; touching
  their inference `Value` types now risks colliding with whichever agent is mid-edit there. Deferred
  as one coherent seam, not hand-waved.
- **`🌿️vcs`** — 0 of ~33 refs touched (ticket's own scan measured 13). Inspected the whole file
  structurally rather than converting blind. Every remaining serde site is load-bearing and
  interconnected, not incidental:
  - `Author` (already dual-derived `Serialize+Deserialize+ToValue+FromValue` — pre-existing, not
    introduced this session), `Change`, `CompositionPin`, `Checkpoint`, `Alternative` are all stored
    inside `ArtifactHistoryLedger<T>`, whose hand-written `impl<T: Serialize> Serialize for
    ArtifactHistoryLedger<T>` (a fixed-capacity ring buffer, custom `SerializeSeq` logic, ~160 lines)
    requires `T: Serialize` — cannot drop serde from any of the five without first converting the
    ledger.
  - `ArtifactVcs<P, Mutation>` has a **hand-written** `impl<P: Serialize, Mutation: Serialize>
    Serialize for ArtifactVcs<..>` that reshapes group-visibility before delegating to
    `ArtifactVcsRead`'s derive — `P`/`Mutation` here are the SAME snapshot/mutation type parameters
    threaded through every artifact in the framework, i.e. flipping this is the same class of
    chokepoint as the `🏪️store` "997 workspace errors downstream" seam already assigned to another
    agent (`ArtifactStore`/`MemberStoreOwners`, 71 bounds) — not a `🌿️vcs`-local decision.
  - `ItemPatch<TId,TPatch>` / `CollectionDiff<TId,TPatch,TAdded>` / `CollectionMutation<TId,TItem,
    TPatch>` are re-exported by `📡️spr::command` as, per their own doc comment, "the one wire shape
    every caller sees," and are used directly by `🗺️surface/🕸️node-graph`, `♾️infinite/…/🕸️dag`, and
    `🪐️space` — three modules entirely outside `🔨️modules/`'s six-module tail and outside this
    session's remit. (Checked they are NOT wired into any `#[mutations(diff = ...)]` seam — `gltf`'s
    `GltfCollectionDiff<T,D>` is a separate, already-converted, plugin-local type — so this isn't the
    `Mutation`/`MutationDiff` seam recurring; it would be a new, self-inflicted one.)
  - The two `serde_json::to_vec(change)` content-hashing call sites (`content_addressed_checkpoint_id_core`)
    hash a `&Change`/`&PendingChangeRef` for a stable id — convertible in isolation to
    `pack::json::to_json_string`, but only once `Change`/`PendingChangeRef` can safely drop
    `Serialize`, which they cannot while the ledger bound stands.
  - Net: this is one coherent, already-partially-touched (by a peer, on `Author`) seam that ripples
    into three modules outside this ticket slice. Converting it here risked exactly the kind of
    half-done, dual-derive-left-behind state CLAUDE.md forbids. Left fully alone, measured precisely
    instead of estimated.

## Already at zero — nothing to do

- **`🎒️pack`** (the os-kernel-mounted facade, `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack`) — all 6
  `serde`/`Deserialize` refs live in `🔎️scalar-witness/🧪️component.rs`, which is reached only via
  `🔎️scalar-witness/🦀️component.rs:192-194`'s `#[cfg(test)] #[path = "🧪️component.rs"] mod tests;` —
  confirmed by reading the mount site, not assumed from the filename. It's a fixture-driven
  differential harness for the incremental `ScalarRecordWireWitness` cursor codec
  (`serde_json::from_str::<Fixture>(include_str!("🧪️fixture.json"))` as the independent oracle) —
  exactly the "check before touching" case the ticket flagged, and exactly the kind of test CLAUDE.md
  sanctions keeping. No change made.

## Verification

`cargo check -p semio-framework-os-kernel --message-format=short`, run twice (foreground, shared
target dir, one at a time), most recent tail:

```
warning: `semio-framework-os-kernel` (lib) generated 29 warnings
error: could not compile `semio-framework-os-kernel` (lib) due to 15 previous errors; 29 warnings emitted
```

All 15 errors are in `🏪️store` (`SpaceAlternative`, `SpaceCheckpoint`, `ArtifactCursorOwners`,
`ArtifactBackboneRef`, `MigrationProvenance`, `OwnerRef`, `HistoryLane` — all `E0277`/`E0599` on
`serde::Serialize`/`serde::Deserialize` not satisfied). **Zero errors trace to any of the six modules
in this report** (`grep`ed the full error list for `🗣️dsl|💡️inference|🧩️extension|🌿️vcs|⚙️engine|🎒️pack`
— no match). This is another agent's in-flight `🏪️store` work (the ticket's own "another agent, 280
refs" line), mid-removal of serde derives with call sites not yet updated — not something introduced
here, and not mine to fix per the ticket's hard constraint against touching peers' in-progress areas.
The crate was NOT observed at a clean 0-error baseline at any point this session (checked once before
starting edits, already red in `🏪️store` then too), so this isn't a regression I caused; it's a
snapshot of concurrent work.

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` was not re-run after these
edits: the shared `semio-framework-os-kernel` crate does not currently compile (see above, unrelated
cause), so a wasip2 build of any downstream plugin would fail on the same `🏪️store` errors regardless
of anything in this report. `cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -e normal -i
serde_json` (a structural command, not a compile, works even while the crate is red) was used instead
to confirm the `🗣️dsl/✨️derive` proc-macro-only claim — see above.

## Score

| module | production serde before | after | status |
|---|---|---|---|
| `🧩️extension` | ~20 | **0** | zero |
| `⚙️engine` | 3 | **0** | zero |
| `🗣️dsl` | 43 | **0** | zero (2 real sites converted; rest was already test/proc-macro-only by inspection) |
| `💡️inference` | 9 | 8 | 1 dead derive removed; `InferredField` seam deferred (~15 external implementors, 2 already converted, rest owned by other in-flight waves) |
| `🌿️vcs` | ~33 (ticket measured 13) | unchanged | deferred as one coherent seam reaching `📡️spr`/`🗺️surface`/`♾️infinite`/`🪐️space`, outside this ticket's six-module scope |
| `🎒️pack` (os-kernel) | 6 | 6 | already 100% test-gated oracle fixture; nothing to convert |

**Three of six modules at zero production serde: `🧩️extension`, `⚙️engine`, `🗣️dsl`.** `🎒️pack` was
already effectively zero. `💡️inference` and `🌿️vcs` are measured-and-deferred, not hand-waved, per
the ticket's own established pattern for `stdio`/`fem`/`procedural`/`mathematical`/`energy`.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧠️lsp/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs`

No `Cargo.toml` lines were touched (none of these six modules have their own manifest — they mount
directly into `semio-framework-os-kernel`'s single `Cargo.toml`, whose `serde`/`serde_json` lines
stay in place, correctly, until `🏪️store`/`InferredField`/`ArtifactVcs`/replication are also clear).
