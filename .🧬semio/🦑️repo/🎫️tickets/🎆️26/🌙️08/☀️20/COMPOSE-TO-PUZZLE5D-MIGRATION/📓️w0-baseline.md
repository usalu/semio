# Wave 0 — Pin and Baseline

## Pin
- Required baseline commit: `5904ebe289a4d149e659b23e1f728895ad8de4e8`
- Verified `git log -1` at session start: `5904ebe289a4d149e659b23e1f728895ad8de4e8` (2026-08-20 23:17:57 +0200)
- Branch: `🐙ueli/⛳wip`
- **HEAD == pin. Migration proceeds against this exact tree.**

## Confirmed facts (verified by direct read, not assumed)

### 28 atomic mutations
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
declares `enum Puzzle5dMutation` with exactly the 28 variants the plan lists, in the plan's order.
Each has a `<slug>/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` triad; `🦠️mutation` additionally
carries `🟦️component.ts`. **No `🧪️tests/` directory exists under any mutation leaf.**

### Missing binary diff protocol — CONFIRMED
`…/✳️any/🦀️component.rs` `io_declaration()`:
```rust
diff: LanguagePair { text: Some(&langs[2]), binary: None },
```
Snapshot and mutations both have text+binary. Diff has text only. Wiring the binary diff
protocol is a Wave-3 foundation task (S03), not per-mutation work.

### `IoDeclaration.entries: &[]` — foreign-format hops UNREGISTERED
The subset root's own docstring documents this as an honest gap: `🚪️io/🦀️component.rs` is still
on the old `ComposerEntry`/`io_registry` channel. Migration must land the typed
`serializer_entry`/`deserializer_entry` shape and relocate `io_declaration()` into `🚪️io` as `io()`.

### Untyped document twin — CONFIRMED
`🧬️mutations/🦀️component.rs` region `🔖️ValueBridge` holds:
- `impl MutationDiff<Value> for Puzzle5dDiff`
- `impl Mutation<Value> for Puzzle5dMutation`
- `normalize_kind_catalogs_for_snapshot_value` — the legacy embedded-catalog guard
Region `🔖️SnapshotDelta` holds a ~120-line before/after structural comparison that synthesizes
mutations by diffing two snapshots field-by-field — the "mutation producer" the plan removes.
Region `🔖️PlaySnapshot` holds the `Puzzle5dPlaySnapshot` newtype.

### Compose surface is larger than the plan's fixture list
`compose/` is not fixture-only. It contains:
- `compose/client/lib/{rs,js,go,net,py}` — five language implementations
- `compose/client/bin/{engine,gql,store}` — executables
- `compose/client/{schema,ui,benchmark,example}`
- `compose/server/hub`
- `compose/dev/{algorithm,schema}`
- `compose/fixture` — 50+ committed fixture assets
1132 tracked files excluding `target/` and `node_modules/`.
Consumer migration (Wave 7) and cleanup (Wave 9) must account for all five language clients,
not just the Rust one.

## Baseline commands
- `cargo check -p semio-s-plugin-puzzle` — baseline state captured in `📓️w0-cargo-check.txt`

## ⚠️ BASELINE IS NOT GREEN — blocker recorded, not worked around

`cargo check -p semio-s-plugin-puzzle` at the pinned commit FAILS:

```
error: could not compile `semio-framework-os-infinite` (lib) due to 927 previous errors
```

Dominant error shape: `error[E0308]: mismatched types: expected `Vec3`, found future`
(plus `no field 'x'/'y'/'z' on type 'impl …'`, `no method named 'add'/'sub' found for opaque
type 'impl …'`). Full log: `📓️w0-cargo-check.txt`.

### Attribution — settled by live predicate, not inference
- `git status --porcelain -- '…/♾️infinite'` → **empty**. The infinite crate has no working-tree
  edits. It is broken **in the committed tree at the pin**, not by anything in this session.
- Repo-wide dirty count: 23 files. The unstaged one is
  `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs` — the derive macro that produces the
  `impl Future` returns the errors complain about.
- Staged ticket files belong to `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP`.
- Conclusion: a **peer session is mid-flight de-asyncing the framework**, and
  `semio-framework-os-infinite` is currently in the broken middle of that sweep.
  `semio-s-plugin-puzzle` depends on it (`infinite_canvas`), so the puzzle crate cannot be
  compile-verified until that sweep lands.

### Consequence for this migration — direct collision, not a background nuisance
Every Puzzle5d mutation leaf this migration touches is written in the **async** style being
removed:
```rust
async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff>
async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation>
async fn label(&self) -> String
```
The de-async sweep will rewrite these signatures across all 28 mutation leaves. The plan's
§5.2 `Puzzle5dMutationBehavior` trait is specified with `fn`, not `async fn`, so the target
shape agrees with where the peer is heading — but any fixture harness or trait I land now
against the async signatures will be rewritten under me.

### Ruling for this session
- Wave 1 (read-only census) is **unaffected** — it reads, it does not compile. Proceed now.
- Wave 3 (foundation: harness, trait, binary diff codec) **must not** be written against the
  async signatures. It waits for the de-async sweep to land on the puzzle plugin, or is written
  signature-agnostic.
- No "tests pass" claim can be made for any Rust fixture until the workspace compiles.
