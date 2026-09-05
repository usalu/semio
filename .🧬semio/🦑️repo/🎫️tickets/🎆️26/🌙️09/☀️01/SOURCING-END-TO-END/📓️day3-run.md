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

## 🔬️ A compile log full of warnings is not evidence the crate type-checks

Several sessions tonight (this one included) read "stdio emitted thousands of warnings and zero
`E0277`" as evidence that stdio is healthy. It is not, and the test that settles it is cheap.

This ticket's attempt log shows **1335 stdio warning lines, 0 `E0277`, 0 `error[...]`**, with a single
`couldn't read` include failure. Broken down by lint, all 1335 are name-resolution-phase:

| count | lint |
| ---: | --- |
| 699 | `unnecessary qualification` |
| 310 | `unused import(s)` |
| 326 | remaining unused-import variants |

The crate is compiled with a `-W` set that includes lints which **cannot fire without type
information** — `clippy::redundant_clone`, `needless_pass_by_value`, `map_unwrap_or`,
`inefficient_to_string`, `cloned_instead_of_copied`, `unnecessary_wraps`. Every one of them scores
**zero** in this log. Type-checking never ran: the crate died during expansion at the
`📡️.protocol.semio` include, and rustc emitted 1335 resolution-phase diagnostics on its way down.

So the discriminator is not *"are there warnings"* but ***"are there warnings that could only exist
after type-check"***. Thousands of `unused import` lines prove only that name resolution ran.

### Consequence for this ticket
A peer session (`semio-4f`, lowpoly) has a build where type-checking demonstrably *did* run — it
reports `E0277`s, which cannot be produced without evaluating trait bounds — and it sees roughly 160 of
them. If those are in stdio, then the mount repairs are **not** the last wall: the `ToValue`/`FromValue`
type errors that blocked this ticket on 2026-09-01/02 are still behind them, and
`semio-s-plugin-sourcing` cannot link until they clear. Unconfirmed from here — this ticket's own build
has never reached stdio's type-check phase — but it is the risk to plan against, and it means "wait for
the rename applier to finish" understates what is required.

**Do not record a green wasm gate for this ticket on the strength of a log that shows no
type-dependent lints.** Step 0 of [✅️end-to-end-checklist.md](✅️end-to-end-checklist.md) is only
satisfied by a build that exits 0.

## 🔄️ 13:18 resume — the migration has converged

The session's processes were lost around 05:50 (build driver, dev server, scratch logs all gone).
What survived on disk and was re-verified at 13:18:

- the framework diagnostic fix (`ui_assembly_error_because`, 5 sites) — intact
- every note in this folder — intact
- the warm isolated `target-sourcing-e2e` (2.9 GB) — intact, so a rebuild is incremental
- the served `…/🪵️sourcing/semio_s_plugin_sourcing_component.core.wasm` — **still Sep 1 12:30**, so
  no peer rebuilt it in the interval and the step 2 baseline recorded above still stands

**The stdio blocker is gone.** The mount gate over `✏️s/🔌️plugins/🗄️stdio` now reports
`0 build-blocking mount(s); 0 test-only reference(s)` — down from 12 in-graph blockers at 04:00 and 65
raw hits at 03:50. Ticket `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` finished its pass
through stdio. There are also **zero peer `cargo`/`rustc` processes** running, against 8 concurrent
sessions and load 200+ overnight — the first genuinely clear build window of this ticket.

Build restarted at 13:19 under the same gate-then-compile driver.

### Still to settle
The overnight evidence never established whether stdio *type-checks*. The last attempt showed
`unreachable expression` (a post-typeck lint) in stdio's tiff module and **zero `E0277`**, which is
suggestive, but the build never exited, so it proves nothing on its own — see the discriminator above.
A peer's lowpoly build reported ~160 `E0277` from a run where typeck demonstrably did execute. This
build exiting 0 is what settles it.

## 🖥️ Step 3 — the launcher, and two blockers that were never about the wasm

`bun run dev:sourcing` failing with status 1 has **two causes independent of the stale plugin**, both
found and one fixed here.

### a. A stale generated worker (fixed)
The boot died at
`🎞️frame-worker.js is stale; run the generate-frame-worker target`
(`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:238`), which cascaded into
`wgpu trunk serve failed` and exit 1. Regenerated with the sanctioned target; `check-frame-worker` now
reports `🎞️frame-worker.js is fresh`.

```
bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker
```

### b. The bare command targets wgpu, not the React shell on :6081
`runFrameworkOsPlaygroundDev`'s own docstring (root `📜️script.ts:183`) states it outright:
`frameworkOsPlaygroundDevEnv` **defaults `SEMIO_RENDERER` to `wgpu`**, so a bare `dev sourcing`
builds all 59 crates and hands off to `trunk serve`, *never* to Vite on `S_OS_PORT`. While stdio is
un-buildable that path cannot complete at all — so the ticket's original symptom was never going to be
cured by rebuilding the sourcing plugin alone.

The React shell this ticket's checklist targets is reached with the `served` segment
(`SEMIO_RENDERER=react` + `SKIP_PLUGIN_BUILD=1` + `SKIP_ENGINE_BUILD=1`), or directly:

```
SEMIO_RENDERER=react SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 S_OS_PORT=6081 \
  bun nx run @semio-tech/framework-os-dev:dev -- sourcing
```

That **boots and serves HTTP 200 on 127.0.0.1:6081** — verified twice.

### What the shell shows, and why it is not yet evidence
The React shell mounts but renders `No plugins loaded / Agent disconnected`, with two console faults:
- `plugin.descriptor-unavailable: /🔌️plugin-modules/🗄️stdio/🔣️.json (HTTP 404)` — stdio's module
  directory holds only an Aug 18 `.core.wasm` and no descriptor. Pre-existing and peer-owned.
- shard workers 0–3 failing with `non-JavaScript MIME type "text/html"` → `shard 0 terminated` →
  `Framework OS boot failed`.

The shard worker is **not** missing: `🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` exists and is
regenerated on each boot, and `SHARD_WORKER_URL`
(`🎭️actor/🧵️shard-runtime/🟦️.ts:22`) correctly names `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`.
Direct `curl` of that URL — and of the sourcing descriptor — **times out at 60s** rather than 404ing.
Under 22 concurrent peer `cargo`/`rustc` processes the Vite host cannot serve its static
plugin-module routes at all. The observed `/plugin-modules/_shard/…` requests came from a *previous*
page load retained in the console buffer, not from this boot.

**So the empty shell is currently explained by host saturation, not by a sourcing defect.** It must be
re-observed on an unloaded box before any conclusion is recorded. The dev server was stopped so the
plugin build can have the machine.

## ✅️ Settled: `semio-s-plugin-stdio` type-checks clean

The open question from the overnight work — whether stdio merely *aborted before* type-checking or
genuinely type-checks — is now answered, using the discriminator recorded above.

Measured on the 13:19 build log at 18:21 (`attempt-1.txt`, 2141 lines, 547 KB):

| probe | count |
| --- | ---: |
| `E0277` | **0** |
| `error[...]` | **0** |
| `could not compile` | **0** |
| `couldn't read` | **0** |
| stdio warnings | 1479 |
| `never constructed` | 14 |
| `never used` | 100 |
| `unreachable expression` | 1 |

Dead-code and reachability lints (`never constructed`, `never used`, `unreachable expression`) run
**after** type-checking — a crate that aborts at expansion cannot emit them. Their presence proves the
type-checker ran; the zero `E0277` therefore means something, unlike the overnight measurement where
all 1335 warnings were resolution-phase and the same zero meant nothing.

**So the `ToValue`/`FromValue` migration that blocked this ticket on 2026-09-01/02 is fully landed in
stdio.** A peer's report of ~160 `E0277` from a lowpoly build described an older tree, not the current
one. Both walls this ticket feared — the `#[path]` mount drift and the type errors behind it — are
down.

The build is still running at 4h55m, inside stdio's codegen/link phase (377 MB wasm, no diagnostics
emitted during codegen, hence a log that stops growing while `rustc` stays busy at ~15% CPU under 22
competing peer cargo processes). A silent log here is expected, not a hang — confirmed by a live
`rustc` child with advancing CPU time.
