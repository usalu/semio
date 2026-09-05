# 📓️ Day 3 run — 2026-09-05

Resuming [📓️day2-resume.md](📓️day2-resume.md). The ticket's source work was complete and the wasm gate
went green at 01:35 on 2026-09-03; everything from step 2 of
[✅️end-to-end-checklist.md](✅️end-to-end-checklist.md) was still open.

## Step 2 baseline — the served artifacts are still the pre-migration build

Measured directly from the served descriptor
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🔣️.json`
before touching anything:

| file | mtime | note |
| --- | --- | --- |
| `semio_s_plugin_sourcing_component.core.wasm` | Sep 1 12:30 | 42 MB, **stale** — the pre-migration build |
| `semio_s_plugin_sourcing_component.js` | Sep 4 17:20 | peer-regenerated |
| `🔣️.json` | Sep 4 11:18 | |
| `🛂️.descriptor.semio` | Sep 5 03:26 | peer-regenerated minutes before this session |

142 command/action rows carry an `interactiveJob`: **110 `migrated`, 32 `batchOnlyPendingRewrite`**.
Those 32 rows collapse to exactly **8 unique ids**, and they are precisely the eight this ticket
migrated in source:

```
curationAdd  curationRemove  curationSetCount  dropOnCurated
dropOnPool   setDocument     setFilterModule   stockFromCatalogue
```

The other six of the fourteen UI-contract ids already read `migrated` (`setActiveExample`,
`setContributions`, `setFilterMinAvailability`, `setFilterQuery`, `setFilterTypology`, `sortTable`).
So the served descriptor is the documented **6 migrated / 8 batch-only** pre-migration state, and the
rebuild's success criterion is unambiguous: all eight flip to `migrated`.

`setLocale` is correctly absent from the descriptor's UI surface — it is `ForbiddenFromUi` and reaches
the app only through the host-configuration route.

## Build strategy — why the plugin build had to be pre-warmed by hand

Two traps, both hit before:
1. **Profile mismatch.** The dev script builds plugins with `cargo rustc --profile wasm-dev`
   (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:102`), not the default `dev`. A `cargo check`
   warm-up on the default profile lands in `wasm32-wasip2/debug/` and is **not reused** by the plugin
   build, which reads `wasm32-wasip2/wasm-dev/`. A first warm-up here was discarded for this reason.
2. **The 20-minute budget.** `runCmdStatus` passes `budgetMs: buildBudgetMs()` and
   `BUILD_BUDGET_MS = 1_200_000` (`🦑️repo/…/📚️library/…/🟦️.ts:1242`). A cold `wasm-dev` build of the
   full dependency chain exceeds that and dies as a silent `spawnSync ETIMEDOUT` — the plugin build
   simply reports failure with no compiler error to show for it.

So the crate is built directly first, with the exact profile and link-arg the dev script uses, under an
isolated `CARGO_TARGET_DIR` with `RUSTC_WRAPPER=""` (peers hold the shared `target/` lock, and sccache
serialises concurrent builds). The dev script honours `CARGO_TARGET_DIR`
(`📜️script.ts:969`), so the subsequent `plugin sourcing` run finds every artifact cached and only has
to link, emit descriptors and stage — comfortably inside the budget.

```
CARGO_TARGET_DIR=target-sourcing-e2e RUSTC_WRAPPER="" \
  cargo rustc -p semio-s-plugin-sourcing --target wasm32-wasip2 --profile wasm-dev \
    -- -C link-arg=-zstack-size=8388608
```

## Verified independently, not taken on trust

**The Pool does not need the broken `stdio` plugin.** Two earlier notes in this folder contradicted
each other on this, and it decides whether the app can be demonstrated at all while
`semio-s-plugin-stdio` fails to link. Settled by reading the accessor itself
(`🗿️artifacts/🗂️curation/🦀️.rs:219`):

```rust
pub fn stock_of(document: &CurationSnapshot) -> Vec<ObjectKind> {
    let _ = &document.catalog;
    document.stock_extra.iter().map(...).collect()
}
```

It explicitly discards `catalog` — the composed `ArtifactChild<SemioKitSnapshot>` half that comes from
`s.stdio.semio.kit` — and reads only the snapshot-owned `stock_extra` overflow, which
`curation_snapshot_from_stock` fills with every field the Pool renders (id, name, module, typology
path, availability, geometry). `📓️status.md` was right and `🧪️runtime-verification.md` was wrong.
So stdio's link failure is boot-log noise for this ticket, not a blocker.

**The fourteen ids really are `Migrated` in source** (`…/✏️editor/🦀️.rs:1149-1162`), with `setLocale`
alone as `ForbiddenFromUi`, and `SOURCING_CURATION_BATCH_ONLY_TOOL_IDS` is gone — only a docstring at
:228 still mentions the old list. The rebuild therefore has a well-defined effect to prove.

**The demo stock is sourcing's own.** `demo_stock()` (`🧬️schema/🦀️.rs:739`) is
`sourcing_modules("[]").flat_map(|m| m.demo_kinds())` — the three built-in modules, ten kinds:
four beams, three windows, three slabs. `default_document()` parses `DEMO_STOCK_TEXT`.

## ⚠️ Host under heavy load
At 03:44 the machine showed **load average 201** on 10 cores with **37.2 GB of 38.9 GB swap in use**,
with peer `bun 📜️script.ts dev s` and `📜️script.ts check` runs in flight. This is the documented
silent-kill condition for cargo here, so no second cargo job was started alongside the plugin build —
the native `cargo test` pass is sequenced after it rather than run concurrently.

## 🏁️ Step 0 attempt 1 — failed on someone else's live rename, and the error was already stale

The first `wasm-dev` build ran 15 minutes clean through the whole framework chain and died in the last
dependency before sourcing:

```
🗄️stdio/…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️.rs:37:1:
error: couldn't read …/🧬️mutations/🟤️set-snapshot/🦀️.rs: No such file or directory (os error 2)
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```

`semio-s-plugin-sourcing` depends on `semio-s-plugin-stdio` as a real `[dependencies]` entry
(`📦️packages/🦀️rust/Cargo.toml:47`), so this is a hard gate on the build even though — as established
above — the Pool needs nothing from stdio at runtime.

**The error described a tree that no longer existed.** Inspecting that file immediately after the
failure, line 36 already read `#[path = "📸️set-snapshot/🦀️.rs"]`, matching the on-disk
`📸️set-snapshot`, with all ten sibling mounts in the file resolving. Ticket
`26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` is running an applier through stdio right
now; it renames directories first and fixes reference strings after, and it repaired this file in the
window between rustc reading it and the inspection. Compare
[📓️day2-resume.md](📓️day2-resume.md)'s same-shaped incident two days earlier.

### Consequences worth keeping
- **A red build is not evidence the tree is red.** During a live migration, re-read the exact file the
  error names before concluding anything. rustc reports the source as of when it opened it.
- **Unresolved-mount counts are not build predictors.** A gate over `✏️s/🔌️plugins/🗄️stdio` reported 62
  unresolved references; **60 were `include_str!`/`include_bytes!` fixture paths inside test functions**
  (`#[semio_framework_async_macros::async_test]`, `#[cfg(all(test, feature = "oracles"))]`), which break
  `cargo test` and `--all-targets` but never a plain lib or component build. Only the handful of real
  `#[path]` module mounts outside `cfg(test)` can block, and that is the set worth watching.
- **Retry beats repair when the tree has an owner.** Sourcing touches nothing in stdio. The build was
  restarted as a loop that retries only on the `couldn't read` rename-race signature and bails
  immediately on any other error, against the warm isolated `CARGO_TARGET_DIR` so each retry recompiles
  only stdio and downstream rather than the framework again.

Peers `semio-f4` (procedural) and `semio-1d` are gated on the same migration and have stood down; the
findings above were shared with them so nobody double-repairs a tree its owner is mid-flight in.

## Step 5 groundwork — the Grid error was unreadable by construction

[🔬️grid-overflow-analysis.md](🔬️grid-overflow-analysis.md) traced
`ui.fixed-capacity: fixed UI admission failed at mesh-window.scene` to a discarded error, and the code
confirms it (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26203`):

```rust
let props = semio_framework_ui_scene::encode(SurfaceKind::World3d, &scene).map_err(|_| ui_assembly_error("mesh-window.scene"))?;
```

`|_|` throws the whole `SurfaceEncodeError` away. That type already distinguishes the three causes and
already implements `Display` for them
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs:39`):

| variant | message |
| --- | --- |
| `Pack(PackError)` | `surface payload encode failed: {error}` |
| `Payload(Vec<u8>)` | `surface payload exceeds fixed capacity with {n} bytes` |
| `Schema(&'static str)` | `surface schema exceeds fixed capacity: {schema}` |

So the prior session's measurement — 18,606 bytes against a 32,768 cap — could never have been
reconciled with the error text, because the text is identical for a pack failure, a capacity overflow
and a schema overflow. **The 57%-of-cap figure does not disprove an overflow; it just means the
`Payload` variant is not the only candidate and the message cannot tell us which fired.**

### Change
`scene_surface` at :351 was already doing this correctly, inline. That inline `format!` is now folded
into a shared helper next to `ui_assembly_error`, and the three window kits that discarded the cause
use it:

```rust
fn ui_assembly_error_because(stage: &'static str, cause: impl std::fmt::Display) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}: {cause}"))
}
```

| site | stage | before |
| --- | --- | --- |
| `:352` `scene_surface` | `scene-surface.encode` | duplicated `format!` inline |
| `:25770` `TextWindowKit` | `text-window.scene` | `\|_\|` |
| `:25807` `TableWindowKit` | `table-window.scene` | `\|_\|` |
| `:26211` `MeshWindowKit` | `mesh-window.scene` | `\|_\|` |

The error code (`ui.fixed-capacity`) and stage names are unchanged, so nothing that matches on them
moves; only the cause is appended. This is diagnostic, not a fix — it is what makes the Grid failure
diagnosable at all, and it is the same shape as this ticket's earlier `replyError` repair, which fixed
guest faults reporting `[object Object]`. Batched into the pending build rather than paying a second
30-minute framework cycle for it.
