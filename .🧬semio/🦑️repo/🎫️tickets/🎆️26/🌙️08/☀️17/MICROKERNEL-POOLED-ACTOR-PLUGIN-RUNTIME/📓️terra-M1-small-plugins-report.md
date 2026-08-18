# 📓️ terra M1-small-plugins report

Packet: **M1-small-plugins** — migrate five low-effect plugins (`🖍️draw`, `📋️forms`, `➗️mathematical`,
`📏️layout`, `🖨️raster`) to the new SDK. Read: `📌️important.md`, `📓️design-abi.md` §6,
`📓️terra-M0-stdio-report.md`, `📓️luna-imports-audit.md`.

## Status: declarations done for all five crates (item 1 of the packet). Wiring (item 2's
## prerequisite) confirmed already correct for all five, nothing to fix. Descriptor emission
## (item 4) **not attempted** — coordinator (sol) is running acceptance directly; see §5.

**Note on process**: my own `cargo check -p semio-s-plugin-forms --all-targets` was still running
when sol messaged that a backgrounded cargo run cannot survive my turn boundary here and took over
acceptance. I stopped issuing further cargo commands per that instruction. §5 below reports the one
real result I saw before stopping, plus what it means for the other four.

## 1. What each of the five crates genuinely does (measured, not assumed)

Confirmed by grep, all five, before any edit:

| crate | `HostEffect::` usages | `pending_effects`/self-tick | owned artifact kinds | `.handler(...)` / extensions |
|---|---|---|---|---|
| 🖍️draw | 0 (already `Effect::` — A3's mechanical rename covered it) | 0 | 1 | 0, no `🧩️extensions/` dir |
| 📋️forms | 0 | 0 | 1 | 0, no `🧩️extensions/` dir |
| ➗️mathematical | 0 | 0 | 1 | 0, no `🧩️extensions/` dir |
| 📏️layout | 0 | 0 | 1 | 0, no `🧩️extensions/` dir |
| 🖨️raster | 0 | 0 | 1 | 0, no `🧩️extensions/` dir |

Items 2 (`HostEffect` rename) and 3 (`pending_effects`/self-tick → timers/jobs) of the packet brief
are **no-ops for all five crates**, confirmed by grep, not assumed:
- `grep -rn HostEffect` under each crate root → zero hits in all five. `📓️luna-imports-audit.md`'s
  14/20 `HostEffect` counts for 🖍️draw/📏️layout predate A3's repo-wide mechanical
  `HostEffect::X` → `Effect::X` rename; I confirmed the exact call sites the audit cited
  (`📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:64/83/88/93`,
  `🖍️draw/…/✏️editor/🦀️component.rs:162/613/629/672`) are now `Effect::DispatchAction` /
  `Effect::SetActiveUtility` / `Effect::ReplayShellCommand` / `Effect::LoadDocument` — already
  migrated by the time I started.
- `grep -rn pending_effects` → zero hits, all five. The one `loop {`/`fn tick` hit in
  🖍️draw's `🔄️fsm` sub-crate is a bounded (`MICROSTEP_LIMIT`-capped) synchronous statechart
  dispatch loop invoked within a single turn/command, not a self-tick poll — read the code at
  `🖍️draw/🔄️fsm/🦀️component.rs:874-905` and `:1508-1530` to confirm before ruling it out.

**Each of these five crates owns exactly one artifact kind** — confirmed by grepping for
`pub fn artifact_kind` under each crate's `🗿️artifacts/` tree: exactly one hit per crate (unlike
`🗄️stdio`'s 36 format modules, each with its own `artifact_kind()`). So one `.activation(...)` call
per crate is complete, not partial:

| crate | `artifact_kind().id` | source |
|---|---|---|
| 🖍️draw | `"2d.drawing"` | `🖍️draw/🗿️artifacts/🖍️draw/🦀️component.rs:382` |
| 📋️forms | `"form.dictionary"` | `📋️forms/🗿️artifacts/📋️forms/🦀️component.rs:442` |
| ➗️mathematical | `"computation.mathematical"` | `➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs:377` |
| 📏️layout | `"2d.layout"` | `📏️layout/🗿️artifacts/📏️layout/🦀️component.rs:431` |
| 🖨️raster | `"2d.raster"` | `🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs:333` |

## 2. Declarations added (item 1) — one edit per crate, `<crate>/🦀️component.rs`

Same shape as `🗒️note`'s E2 proof migration and `🗄️stdio`'s M0 migration, applied per crate:

- **One `.activation(ActivationEvent::OnArtifactKind { kind: … })` per crate**, reading the kind
  live from that crate's own `artifact_kind().id` function (never hardcoded) — verified by grep,
  §1 table above, and by the fact the field reference is `crate::artifacts::<name>::artifact_kind().id`
  in every file, not a string literal.
- **`.execution(ExecutionMode::Isolated)`** for all five — the SDK default. Nothing in any of the
  five crates' own code justifies otherwise: no `.handler(...)`, no `🧩️extensions/` directory, no
  cross-plugin extension attachment anywhere in the tree (confirmed by `find` for a
  `🧩️extensions/` dir under each of the five plugin roots — none exist).
  - 🖍️draw specifically: its `🔄️fsm` sub-crate (canvas gesture statechart) runs synchronously
    within a turn, bounded by `MICROSTEP_LIMIT` — no evidence of a background/isolated-worker need.
- **One `.requests(CapabilityRequest{ id: "documents.write", scope: "plugin", optional: false, reason: … })`
  per crate**, mirroring `🗒️note`'s and `🗄️stdio`'s own single `documents.write` request: each
  crate's `.editor()` registration (`DrawPlayApp`/`FormsPlayApp`/`MathematicalPlayApp`/
  `LayoutPlayApp`/`RasterPlayApp`, all paired with `.editor_mutation_roster::<…>()`) is the one
  mutation-capable surface that persists edits back to the open document — the same real behavior
  the OLD `documents::{Read,Write}` `CapabilityRequirement` per contract §2.3 clause 4 already
  attaches; this is the NEW broker-scoped ask for it. Reason strings are per-crate, not copy-pasted
  boilerplate: "persist draw/form dictionary/mathematical graph/layout/raster edits to the open
  document."
- **No other capability requested.** I checked the kernel's `Effect` enum
  (`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`) for every non-`HostEffect`-derived variant
  🖍️draw and 📏️layout actually emit (`LoadDocument`, `SetActiveUtility`, `ReplayShellCommand`,
  `DispatchAction`, `DownloadMediaExport`) against `📓️design-abi.md` §5's documented
  `CapabilityId` set (`storage.*`, `http:*`, `timers`, `messaging.*`, `documents.*`, `blobs.*`,
  `jobs.*`, `ui.window|dialog`, `shell.navigate|clipboard`, `extension-registry.query`,
  `extension.invoke:*`) — none of those five effect variants map to a documented capability id
  (they're host-owned UI-chrome/RPC effects with no broker gate today, same category `🗒️note`'s
  and `🗄️stdio`'s own accepted migrations left ungated). Inventing a capability id with no
  broker-recognized meaning would be fabrication, not a "requests" declaration grounded in real
  code — so I didn't.
- **No quotas declared, any crate.** Grepped every crate for evidence of a genuine need (long-
  running computation held across turns, large in-memory buffers, high-frequency timers) — found
  none. 🖍️draw's FSM and 📏️layout's ~20/~14 `Effect` call sites are all per-turn UI/document
  effects, not accumulating state. `QuotaSchema::default()` (all `None`, inherit) is honest here,
  matching `🗄️stdio`'s "declared none, correctly" precedent the packet brief itself cites.

Exact edits (imports + builder chain), all five files:
```rust
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};
…
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::<name>::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist <name> edits to the open document".into(), optional: false })
        .try_build()
```

## 3. Wiring check (M0's two findings) — already correct in all five, nothing to fix

M0 found stdio missing both of these; I checked both for all five of mine and **neither gap
exists here** — no edit was needed:

| crate | `Cargo.toml` requests `component-guest` on `semio-framework-plugin`? | `📦️glue.rs` calls `plugin_exports!(plugin::plugin)`? |
|---|---|---|
| 🖍️draw | yes — `📦️packages/🦀️rust/Cargo.toml:31` | yes — `📦️glue.rs:581` |
| 📋️forms | yes — `Cargo.toml:30` | yes — `📦️glue.rs:488` |
| ➗️mathematical | yes — `Cargo.toml:51` | yes — `📦️glue.rs:535` |
| 📏️layout | yes — `Cargo.toml:31` | yes — `📦️glue.rs:678` |
| 🖨️raster | yes — `Cargo.toml:31` | yes — `📦️glue.rs:639` |

(🖍️draw's `🔄️fsm` sub-crate is a plain support library — no `semio-framework-plugin` dependency,
no actor world, not itself a plugin — correctly has neither.)

## 4. Descriptor emission (item 4) — not attempted, honest reason

I did not reach descriptor emission (`🛂️descriptor.semio` + `🔣️descriptor.json` at each owner
root) for any of the five crates. Sequence of events: my first acceptance command
(`cargo check -p semio-s-plugin-forms --all-targets`) ran past the tool's foreground window and
was moved to a background task; while I was monitoring it for completion (rather than proceeding
to a second concurrent cargo build against the same shared `🎯️target-m1` dir, per `📌️important.md`'s
"only ONE packet may hold a cargo build" rule), sol messaged that this can't survive my turn
boundary and took over acceptance directly. I stopped rather than continue issuing cargo commands,
per that instruction.

**So no crate has been verified to pass `try_build()`'s claim-set check
(`🔌️plugin/🦀️component.rs:2568`, the same rule that blocked stdio on ~35 of 36 formats) or to
emit a real (non-`assembly-failed`) descriptor.** I have not committed any descriptor for any of
the five crates — correctly nothing to un-commit, since none was ever generated. If sol's real run
hits the same claim-set mismatch M0 found on stdio, the correct action is what M0 did: do not
commit a descriptor carrying `pluginId: "assembly-failed"` or empty `activationEvents`/
`capabilityRequests` — delete it and report the mismatch per crate instead.

## 5. Acceptance — run by the coordinator (sol), not by me, except one interrupted attempt

**I did not complete any acceptance command.** The one real result I observed, for the record:

```
$ export CARGO_TARGET_DIR=.../🎯️target-m1
$ cargo check -p semio-s-plugin-forms --all-targets
   … (workspace deps compile clean, including semio-framework-plugin, semio-s-plugin-stdio) …
    Checking semio-framework-os-flow v0.1.0 (…/🌊️flow/📦️packages/🦀️rust)
error[E0560]: struct `BlockListScene` has no field named `domain_id`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../../📖️playbook/🦀️component.rs:966:247
966 | ...map(String::from), dragging_id: None, domain_id: None }
    |                                          ^^^^^^^^^ `BlockListScene` does not have this field
error: could not compile `semio-framework-os-flow` (lib) due to 1 previous error
```

This is **upstream of my scope, not caused by my edits**:
- The failing file is `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs` —
  inside `🧰️framework/**`, explicitly not-mine per the packet's owned-paths list.
- `git status --porcelain` on that file shows `M` (modified, uncommitted) at the time I checked,
  with a file mtime (`11:44`) about 9 minutes before I checked (`11:53`) — consistent with a live,
  in-progress peer edit, not a stable pre-existing baseline. I did not touch it and did not wait it
  out before sol took over.
- Root cause, for whoever owns that file: `BlockListScene` (defined in
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:3830-3838`) has
  fields `steps_json`, `palette_json`, `selected_id`, `dragging_id` — **no `domain_id`** — while
  sibling scene structs in the same file (`World3dScene`, `TableScene`, `DiffViewScene`,
  `EventFeedScene`) do carry `domain_id: Option<String>`. `📖️playbook/🦀️component.rs:966`
  constructs a `BlockListScene` passing `domain_id: None` as if the field existed. Looks like an
  in-progress "add `domain_id` to every scene type" edit that hasn't reached `BlockListScene`'s own
  struct definition yet (that same wgpu file's own line 4846 constructs a `BlockListScene` with
  `domain_id: None` too, so the struct edit is mid-flight there as well).
- `semio-s-plugin-forms` pulls this in transitively via its own declared dependency on `flow`
  (`= { …, package = "semio-framework-os-flow" }`, `📋️forms/📦️packages/🦀️rust/Cargo.toml:28`) —
  a real, pre-existing dependency edge, not something my edit introduced.

**I do not know whether this same edge blocks 🖍️draw/➗️mathematical/📏️layout/🖨️raster** — I did
not get to run their `cargo check` before being told to stop. Worth checking each crate's
`Cargo.toml` for a `semio-framework-os-flow`/`flow` dependency before assuming it does or doesn't
apply; I have not done that check.

**No `cargo check --target wasm32-wasip2` was run for any of the five.** **No
`descriptor_is_fresh()` was run for any of the five.**

## 6. Summary table

| crate | `.activation` | `.execution` | `.requests` | quota | wiring gaps | `HostEffect`/self-tick | descriptor | acceptance |
|---|---|---|---|---|---|---|---|---|
| 🖍️draw | done, 1 (2d.drawing) | Isolated | documents.write | none, honest | none found | 0/0, confirmed | not attempted | not run by me |
| 📋️forms | done, 1 (form.dictionary) | Isolated | documents.write | none, honest | none found | 0/0, confirmed | not attempted | red — upstream `flow`/`playbook` `BlockListScene.domain_id` E0560, not mine, not in my paths |
| ➗️mathematical | done, 1 (computation.mathematical) | Isolated | documents.write | none, honest | none found | 0/0, confirmed | not attempted | not run by me |
| 📏️layout | done, 1 (2d.layout) | Isolated | documents.write | none, honest | none found | 0/0, confirmed | not attempted | not run by me |
| 🖨️raster | done, 1 (2d.raster) | Isolated | documents.write | none, honest | none found | 0/0, confirmed | not attempted | not run by me |

## Files touched

**Modified** (all within my owned paths):
- `✏️s/🔌️plugins/🖍️draw/🦀️component.rs`
- `✏️s/🔌️plugins/📋️forms/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🦀️component.rs`

Each: added `use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};`
+ `use semio_framework_plugin::{ExecutionMode, Plugin};`, and appended `.activation(...)`,
`.execution(ExecutionMode::Isolated)`, `.requests(CapabilityRequest{ id: "documents.write", ... })`
to the existing `Plugin::builder(...)` chain before `.try_build()`.

**Not touched**: `🧰️framework/**` (including the `📖️playbook`/`flow` blocker in §5 — flagged, not
fixed, not mine), any other plugin, root manifests, `.vscode/*`.

**Scratch (ticket folder)**: none created beyond this report — the one background cargo run's
output lived in the harness's own temp task-output path, not the ticket folder, and is not repo
state.

## Not started / handed off

- **Acceptance** (`cargo check --all-targets` / `--target wasm32-wasip2` / `descriptor_is_fresh()`
  for all five crates) — sol is running this directly per their message in §4/§5.
- **The upstream `flow`/`playbook` `BlockListScene.domain_id` compile error** (§5) — real, blocks
  at least `📋️forms` (confirmed dependency edge), possibly others; outside `🧰️framework/**`, not
  mine to fix, appears to be a live in-progress peer edit rather than a stable baseline.
- **Descriptor emission and commit** for all five crates — blocked on acceptance actually running;
  if the same claim-set rule that hit stdio on ~35/36 formats also hits any of these five
  single-artifact-kind crates, the next agent should report it precisely per crate and not commit
  a placeholder, mirroring M0's ruling.
