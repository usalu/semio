# 📓️ Sourcing End to End — Status

- [🔍️diagnosis.md](🔍️diagnosis.md) — why `dev:sourcing` failed
- [🧪️test-triage.md](🧪️test-triage.md) — the test suite, failure by failure
- [📓️batch-only-migration.md](📓️batch-only-migration.md) — the eight unreachable commands
- [🧪️runtime-verification.md](🧪️runtime-verification.md) — what the browser actually shows

## The headline
`bun run dev:sourcing` failed because **`semio-s-plugin-sourcing` had stopped compiling** — 109
errors — so the browser had been serving a five-day-old wasm built against a different framework.
`cargo check -p semio-s-plugin-sourcing --target wasm32-wasip2` is now clean, the plugin rebuilds,
and the app mounts on :6081 with its four windows.

## Landed
### Compile (109 → 0)
1. **Aggregate mutation primary renamed to the taxonomy-canonical name.**
   `🧬️schema/🧬️mutations/🦀️component.rs` → `🦀️.rs`. `#[derive(dsl::Mutations)]` validates that the file
   carrying it is named after the taxonomy's `mutationComponentFileKindId` (`🦀️` + `.rs`); it wasn't,
   so the derive emitted a `compile_error!` and **no** `impl Mutation<CurateSnapshot>` at all — which
   cascaded into ~108 `E0277`s across the editor, viewer, io and operations surfaces. `📦️glue.rs`,
   the facet `include_str!` and the `mutate-curate-1` docstrings follow.
2. **Leaf descriptors consolidated onto `🔣️.json`**, keeping the truthful content (binary tags 0/1/2,
   seven language surfaces) that the aggregate's own structural test asserts and the leaf directories
   actually carry; the duplicate `🔣️component.json` files are deleted.
3. **`Mutation::DESCRIPTORS` + `descriptor()`** added to the two hand-written impls (config, presence).

### Runtime reach
4. **All eight `BatchOnlyPendingRewrite` commands migrated** — the whole curation vocabulary, the
   module filter, and both whole-document replacements. They were unreachable from the browser, not
   merely untested. This needed a new document-lane `ArtifactStoreOneItemPreparationFactory`, the
   config lane extended to `SetFilterModules`, tool proofs and publication lanes for all fourteen
   UI-reachable commands, and every classification flipped to `Migrated`.
   Compiles clean for `wasm32-wasip2`.

### Diagnostics
5. **Guest plugin faults no longer report `[object Object]`.** `replyError` stringified a lifted fault
   record with `String(error)`; it now serializes the record. Without this the console, the host and
   the on-screen error surface all showed nothing for the most common failure there is.

### Test harness (the suite could not run at all before)
6. One registry-backed `new_app()`; a `SourcingTestApp` guard that drains the bounded close protocol
   in `Drop` (four un-closed `ArtifactStore`s used to abort the whole test binary); instance binding;
   store owners + close in the io/binary round trip; and a framework fix so
   `test_support::assert_document_{text,pack}_round_trip` retires the comparison document it parses.
   115 passed / 11 failed, down from "does not compile".

## 🚧️ Blocked: a peer's `ToValue`/`FromValue` serde-removal migration
Since ~13:20 the workspace has not compiled, in crates this ticket never touched. The breakage walked
up the dependency chain over ~2h of retries — `semio-framework-mesh-engine` → `semio-framework` →
`semio-framework-plugin` → `semio-framework-os-kernel` → `semio-framework-ui` → `semio-s-plugin-stdio`
— which is a refactor in progress, not a stuck one. It currently stops at **2206 errors in
`semio-s-plugin-stdio`**, all of one shape: the new derives reject tuple structs and multi-payload
enum variants that stdio's artifacts are full of, e.g.

```
#[derive(ToValue, FromValue)] supports named-field structs …, not tuple/unit structs
  📰xml/…/🧬️mutation-support/🦀️component.rs:6   pub struct XmlNodePath(pub Vec<usize>);
#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields
  🏗️ifc/…/📸️snapshot/🦀️component.rs:39          TypedValue(String, Vec<IfcValue>),
```

`semio-s-plugin-sourcing` depends on `semio-s-plugin-stdio`, so nothing here can be built or tested
until that lands. Retry with `cargo test -p semio-s-plugin-sourcing`; the ticket's own work needs no
changes to resume. Everything in this ticket that COULD be verified was: the migration compiled clean
for `wasm32-wasip2` at ~13:07, before the first of these errors appeared.

**17:36 recheck — every framework crate has recovered; only stdio is left.** A
`cargo check -p semio-s-plugin-sourcing --target wasm32-wasip2` now reports **zero** errors outside
`semio-s-plugin-stdio`, and 2196 inside it, concentrated in its largest artifacts:

| artifact | errors | | artifact | errors |
| --- | ---: | --- | --- | ---: |
| `🧿️semio` | 593 | | `🎞️gif` | 74 |
| `🧊️gltf` | 259 | | `🏗️ifc` | 72 |
| `📄️pdf` | 220 | | `🎨️svg` | 62 |
| `📐️step` | 82 | | `📜️docx` | 58 |

The peer has landed a new `🧰️framework/🔨️modules/🌱️value/✨️derive` crate and worked the migration down
the dependency chain to its largest consumer, so this reads as nearly through rather than stalled. Not
picked up here: those are files a peer is editing right now, and 2200 mechanical derive fixes across
someone else's in-flight refactor is the one thing guaranteed to collide.

## Open
- The remaining test failures, re-run after the migration (blocked, above).
- **Grid window scene overflows its fixed 32 KiB surface payload** (`ui.fixed-capacity` at
  `mesh-window.scene`) for the unfiltered demo stock — a runtime defect, not just a test one. Needs
  measuring before choosing between a smaller payload and the `World3dSnapshotLease` page path.
- Final `plugin sourcing` rebuild to refresh `🛂️descriptor.semio` / `🔣️descriptor.json`, then the
  interactive browser pass (load example → add to curation → drag between panes → filter).

## Not ours
`semio-s-plugin-stdio` does not link (`functions count exceeds limit of 1000000`), so
`🔌️plugin-modules/stdio/` has no descriptor and the OS logs `plugin.descriptor-invalid` on every boot.
Sourcing's own pool does not depend on it — `stock_of` reads `stock_extra` only — but it is noise on
every load and it is a peer-owned crate.
