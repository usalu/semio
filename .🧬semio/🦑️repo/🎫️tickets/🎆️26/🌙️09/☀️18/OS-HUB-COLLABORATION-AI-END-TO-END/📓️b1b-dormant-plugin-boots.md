# B1b — Dormant Plugin Boots (batch B): reasoning · norm · playbook · imperative · dag · space

Slice B1b of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Scope: drive the six batch-B plugins from
`📓️audit-plugins-artifacts.md` Q1's **UNTESTED** list through a real React-renderer boot, probe them headless,
fix boot faults at the root, and register launch entries. Follows the `raster`/`note` boot-recipe pattern
(`26/09/05/RASTER-PLUGIN-END-TO-END`, `26/09/17/NOTE-PLUGIN-END-TO-END`).

## Headline

- **Five of six now boot the React shell** (`data-semio-os-ready` set, no `data-semio-os-error`, panes render,
  Actions panes populated). None of the six had a boot on record (audit Q1 UNTESTED) and all six were cold — no `dist/component-dev` wasm existed for any of them when this slice started.
- **`norm` boots clean** — zero fault lines, a fully rendered two-pane compliance editor with 19 DIN 4108 check
  rows evaluated from the seeded inputs. It is the healthiest plugin of the batch by a wide margin.
- **Six real root-cause defects found and fixed** across `reasoning` and `playbook` (4 files, +143/−11). `playbook` could
  not activate at all before the fix — it died inside `materialize dev`'s descriptor probe.
- **No plugin yet passes the full "booted" bar** (`no fault lines AND an interaction that changed state`).
  The batch shares one systemic blocker chain, walked end to end below on `reasoning`.
- **`space` is not booted**: the `s` hub has no activation receipt and `dev s served` still resolves a
  **132-task** Nx chain that rebuilds the whole plugin catalog — that cold build belongs to worker C1.

## Per-plugin results

| plugin | variant | react port | cold activate | boot result | faults found (file:line) | fixed | interaction verified | probe |
|---|---|---|---|---|---|---|---|---|
| 💡️reasoning | `reasoning-wires` | 6015 | 1 m 31 s (`reasoning-plugin:component-dev` 41 s) | 🟡 boots — 1 window (`reasoning-wires-composite`), 21 actions, canvas surface; 2 fault lines | F1 classification, F2 proofs, F3 no `command_from_action`, F4 example-id drift, F5 no document-store init job, **F6 retained publication admits only `MoveNode`** | F1–F5 fixed (3 files) | ❌ `addNode` now reaches the typed operation and fails in F6 | `🐍️b1b-reasoning-boot-probe.mjs` |
| 📕️norm | `din4108` (1 of 15) | 6091 | 4 m 25 s (`norm-plugin:component-dev` 3 m 25 s) | 🟢 boots clean — Inputs + Results windows, 19 evaluated checks, 12 actions, **0 fault lines at boot** | F3 only: `evaluate`/`setSnapshot`/`setSelectedCheckIndex` have no `command_from_action` bridge | — | ❌ `evaluate` refused (F3) | `🐍️b1b-norm-boot-probe.mjs` |
| 📖️playbook | `playbook` | 6085 | 1 m 40 s (first two runs **failed**, see F7/F8) | 🟡 boots — Builder window with the step/block palette (104 svg nodes), 20 actions; 3 fault lines | **F7 unclassified extension verbs**, **F8 missing plugin-bundle dependency**, F1 (`addStep` BatchOnly), F9 no `setActiveExample` | **F7, F8 fixed** (1 file) — these were the hard activation blockers | ❌ `addStep` refused (F1) | `🐍️b1b-playbook-boot-probe.mjs` |
| 📜️imperative | `imperative` | 6076 | 2 m 04 s | 🟡 boots and **renders its document** (`# Id Kind / 1 step-1 state.set / 2 step-2 log.print`); 2 fault lines | **F10 no `window_kind_actions`/`window_kind_action_refs` at all → Actions pane empty (0 actions)**, F9, F1 (10 BatchOnly verbs) | — | ❌ no dispatchable action exists in the shell | `🐍️b1b-imperative-boot-probe.mjs` |
| 🕸️dag | `dag` | 6017 | 1 m 32 s (`dag-plugin:component-dev` 42.4 s) | 🟡 boots — 2 windows (`dag-main`, `dag-compiled-dag`), DAG/DSL tabs, 24 actions; 3 fault lines | F9 no `setActiveExample` command at all, F1 (10 BatchOnly verbs) | — | ❌ `addNode` refused (F1) | `🐍️b1b-dag-boot-probe.mjs` |
| 🪐️space | `s` | 6070 | **not reached** | 🔴 not booted | F11 no `dist/runtime/react/dev/s/activation` receipt; `dev s served` resolves 132 Nx tasks and rebuilds the catalog | — | — | — |

Screenshots: `🗑️generated/b1b-<plugin>.png`. Console + accessibility dumps: `🗑️generated/b1b-<plugin>-console.txt`.
Structured step reports: `🗑️generated/b1b-<plugin>/report.json`.

## The shared blocker chain

Every batch-B plugin fails somewhere on the same five-link chain between "the shell dispatched a verb" and
"the document changed". `reasoning` was walked all the way down; each link surfaced only after the previous one
was fixed, which is why they are numbered in discovery order.

**F1 — `InteractiveJobClassification::BatchOnlyPendingRewrite` (dag 10 verbs, imperative 10, playbook 7, reasoning 4).**
`validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12594`) admits **only**
`Migrated`; everything else is refused as `interactive-job.not-ui-safe`. Every document verb of dag, imperative and
playbook is therefore hard-dead in the shell today. (Matches `📓️project-interactive-job-classification-gates-dispatch`.)

**F2 — flipping the label alone traps the guest at boot.** Setting `Migrated` without the machinery panics in
`🧰️framework/…/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21904` with
`interactive-job.catalog-incomplete: a migrated generated command lacks its exact owner-local bounded reducer proof`
→ `RuntimeError: unreachable` → `Framework OS boot failed`, i.e. a **worse** state than the refusal. `Migrated`
requires all four of: the id in the app's retained `TOOL_IDS`, a `PUBLICATION_CONTRACTS` row with exactly the
lanes the handler emits, an arm in the extent function, and a `bounded_first_step_tool_proofs!` `tools:` entry
bound to a concrete `factory_type`.

**F3 — `ArtifactApp::command_from_action` is not overridden** (reasoning, norm, imperative, dag — 4 of 6).
The trait default (`🧰️framework/…/🔌️plugin/🦀️.rs:11762`) refuses every id with
`app.command.unsupported: action '<id>' is not a framework-reserved action … dispatched exclusively through the
typed command channel now`. The React/wgpu shells still speak `{action, args}`, so with no bridge **no** Actions-pane
row, example pick or canvas gesture can ever reach `Command::dispatch`. `app_commands!` generates `command_id`,
`dispatch`, `TOOL_JOB_IDS` and both codecs, but no `from_action` — each app must write the bridge (note's is
`✏️s/🔌️plugins/🗒️note/…/✏️editor/🦀️.rs:313`).

**F4 — example-id drift.** `reasoning`'s `SetActiveExample` compared against the literal `"metabolism"` while the
subset registers exactly one example, `crate::examples::demo` (`ID = "demo"`), whose asset **is** the metabolism
DSL. The shell only ever dispatches a registered id, so every example pick silently loaded the EMPTY document.
(Same shape as note fault 9.)

**F5 — `build_document_store_initialization_job` not overridden.** The trait default refuses the envelope, so the
host answers every `Effect::LoadDocument` with
`artifact-store.persisted-initializer-refused: app refused the persisted document's retained initialization authority`.

**F6 — the app's artifact-lane publication authority admits only one mutation kind.** With F1–F5 fixed,
`reasoning`'s `addNode` reaches the typed operation and dies on
`typed-operation failed: Wires retained publication only admits MoveNode`
(`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:303`,
guard also at `:325`). That module is a bespoke ~700-line bounded cursor written for the canvas **drag** path only.
`setActiveExample`'s whole-document swap fails the same authority
(`plugin.internal: document archive replacement failed closure, authority, or retained publication validation`).

## Root causes fixed

### 1. `playbook` could not activate at all — unclassified extension verbs (F7)

`bun nx run @semio-tech/framework-os-dev:activate-playbook-react-dev` exited **130** inside `materialize dev`:

```
Plugin descriptor failed for playbook-module-procedural: assertion `left == right` failed:
extension and app bundle identities must match
  left: "assembly-failed"
 right: "playbook-module-procedural"
RuntimeError: unreachable
```

`validate_interactive_job_classification` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:850`) rejects the
`Unclassified` manifest default; `App::try_from_builder` turns that into a `PluginAssemblyError`; the guest then
ships the `assembly-failed` manifest stub (`🧰️framework/…/🔌️plugin/🦀️.rs:34105`), and the extension-identity
assert in `🧰️framework/…/🔌️plugin/🛂️describe/🦀️.rs:166` fires before anything prints the real message. The two
`.mutation(...)` verbs carried no disposition:

`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:687-688` (new)

```rust
.action_interactive_job(ACTION_EXPORT_SOLID, InteractiveJobClassification::BatchOnlyPendingRewrite).await
.action_interactive_job(ACTION_IMPORT_SOLID, InteractiveJobClassification::BatchOnlyPendingRewrite).await,
```

`BatchOnlyPendingRewrite` is the **truthful** disposition: this extension owns no bounded tool-job factory, so
claiming `Migrated` would trip F2's boot check. (`📓️project-unclassified-verbs-abort-plugin-descriptor-probe`.)

### 2. `playbook` — the extension's plugin bundle never declared its host plugin (F8)

With F7 fixed the descriptor probe reported the next assembly error (recovered natively via
`cargo test -p semio-s-plugin-playbook-procedural --lib -- procedural_actor_descriptor_matches_the_json_oracle`,
which had been failing at HEAD for the same reason):

```
bundle assembly: app `s.playbook.procedural@1/*#editor` contributes a surface for
`s.playbook.procedural@1/*` owned by `playbook`, which `playbook-module-procedural` does not declare as a dependency
```

`surface_dependency_breaches` (`🧰️framework/…/🔌️plugin/🦀️.rs:4473-4486`) requires a manifest contributing a
foreign artifact kind's surface to name that kind's owner. The sibling `ExtensionBundle` already declared
`.extends("playbook").depends_on("playbook", …)`; the **plugin** bundle did not.

`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:708` (new)

```rust
.depends_on("playbook", VersionReq::parse("^0.1.0").expect("declared playbook version"))
```

`activate-playbook-react-dev` now exits 0 (1 m 40 s) and the playground serves.

### 3. `reasoning` — the four document verbs promoted to real retained tools (F1 + F2)

All in `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

- `:153` `WIRES_RETAINED_TOOL_IDS` += `setActiveExample`, `addNode`, `addRelationship`, `deleteSelection`.
- `:157` `WIRES_RETAINED_PUBLICATION_CONTRACTS` += `setActiveExample` → `HostOnly` (effect-only `LoadDocument`),
  the other three → `Artifact` (their handlers emit `artifact_mutations` only).
- `:172` `wires_retained_extent` += a `Some(1)` arm for the four one-shot verbs.
- `:566` the proofs block's `tools: [...]` += the same four ids.
- `:587` `build_tool_job` now picks the framework's generic
  `retained_command::BoundedArtifactCommandWork` (raster/trinity precedent) for the document verbs and keeps the
  bespoke `WiresWindowDragWork` cursor for the four gesture verbs.
- `:233` new `wires_retained_document_reduce`, an `ArtifactCommandReducer` over `WiresCommand::dispatch`, with a
  `DeleteSelection` arm that reads the `"graph"` domain selection off the raw `protocol::InteractionState`
  (`InteractionView`'s fields are framework-private) through the new
  `delete_selection::apply_with_state` (`🎮️commands/🗑️delete-selection/🦀️.rs:45`).
- `:757-760` the four `.action_interactive_job(..., BatchOnlyPendingRewrite)` rows → `Migrated`.

### 4. `reasoning` — the `{action,args}` bridge (F3)

`…/✏️editor/🦀️.rs:187` new `wires_command_from_action`, wired at `:638` as
`ArtifactApp::command_from_action`. Resolves all eight declared verbs, tolerating the shells' key aliases
(`exampleId`/`example_id`/`id`/`value`, `kind`/`value`) and string-typed control values, and faults unknown ids
as `wires.unhandled-action` instead of the framework's generic refusal.

### 5. `reasoning` — the example id the shell actually sends (F4)

`…/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs:15`

```rust
-pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";
+pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = crate::examples::demo::ID;
```

### 6. `reasoning` — `LoadDocument` admission (F5)

`…/✏️editor/🦀️.rs:511` new `build_document_store_initialization_job` returning
`bounded_document_store_initialization_job(envelope, crate::MINDMAP_WIRES_SCHEMA, operation, generation)`
(note's shape).

`cargo check -p semio-s-artifact-reasoning-wires --message-format short` is green after all of the above;
`activate-reasoning-wires-react-dev` exits 0 and the guest boots without traps.

## Exact next steps (nothing here is speculative — each is the fault the probe prints today)

1. **`reasoning`, one file.** Replace the `MoveNode`-only `retained::factory()` with the framework's generic
   `semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>(...)`
   in `build_artifact_store_one_item_preparation_factory` (`…/✏️editor/🦀️.rs:518`) — the **dag** and **trinity**
   precedents do exactly this for the artifact lane (`✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️.rs:525`,
   `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/…/✏️editor/🦀️.rs:864`). That retires F6 for both `addNode` and
   `setActiveExample` and should make `reasoning` the batch's first fully interactive boot. It also orphans the
   bespoke `🧵️retained/🦀️.rs` cursor, so the drag path needs a decision, not just a deletion.
2. **`dag`** — write `command_from_action` (F3), reclassify the 10 verbs with the full F2 recipe, and add a
   `setActiveExample` command (it has `📚️examples/🎬️demo` on disk and the playground catalog advertises it, but
   the editor declares no such verb at all).
3. **`imperative`** — F10 first: the app declares 10 actions but never calls `.window_kind_actions(...)` /
   `.window_kind_action_refs(...)`, so both windows show an empty Actions pane and *every* shell dispatch is
   `undeclared-action`. Then F3 + F2, then `setActiveExample`.
4. **`playbook`** — extend its existing `command_from_action` to `setContributions` (it fails today with the
   same `app.command.unsupported`), declare `setActiveExample`, reclassify the 7 BatchOnly verbs.
5. **`norm`** — the cheapest win in the batch: only F3 stands between a clean boot and a working editor. Its
   three verbs are already `Migrated` **and** retained with a factory and publication contracts
   (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:408-423`), so a ~20-line bridge should be enough.
6. **`space`** — needs the whole-catalog cold build (worker C1). `dev s served` was started here and reached
   19 of 132 tasks (through `cad-extension-aec-building-rust:materialize-dev`) before it was stopped so it would
   not hold the shared Cargo lock against C1's own cold build; the log is `🗑️generated/b1b-space-dev.txt`.
   Re-run it once `dist/runtime/react/dev/s/activation` exists; `🪐️space` itself is already staged in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/`.

## Method notes (for the next boot-recipe slice)

- **Do not use `bun ./📜️script.ts dev <variant>` on a busy repo.** It runs `nx watch --all` and re-runs
  `activate-<variant>-react-dev` on *every* peer file change — during this slice it re-activated within three
  minutes and took the Vite server down mid-probe (`ERR_CONNECTION_REFUSED` on every request). Split it the way
  the raster/note tickets do: one `activate` then a detached `serve`. Both helpers are committed here:
  `📜️b1b-activate.sh <variant> <port>` and `📜️b1b-serve.sh <variant> <port>`.
- **`bun` resolves `import { chromium } from "playwright"` against its own global install cache** when the script
  lives outside the repo, and that cache wanted a Chromium revision that is not installed
  (`chromium_headless_shell-1243` vs the installed `-1234`). Probes under the ticket folder resolve the repo's
  `node_modules` correctly; scratch scripts must import
  `/Users/ueli/Documents/semio/node_modules/playwright/index.mjs` by absolute path.
- **A state witness must exclude the Actions pane.** Expanding an action row to reach its arguments is itself a
  `[role="treeitem"]` change, so a naive tree diff scores a *dead* verb as a state change — the first dag probe
  reported `addNode` "changed state" while the console said `refused: dispatch-failed`. The shared probe now
  filters out every row inside a `[id$=".engagement"]` container (`🐍️b1b-boot-probe.mjs:31,42`). Related: an
  argument-less action row IS its own trigger — no `.execute` control is ever rendered — so the witness has to be
  sampled before the row is clicked.
- **The descriptor probe hides the real assembly error.** `materialize dev` prints only the
  `assembly-failed` identity assert. Recover the real message by running the extension's own native descriptor
  test (`cargo test -p <crate> --lib -- <…>_descriptor_matches_the_json_oracle`), which asserts on the same
  identity and prints the `PluginAssemblyError` text in the assert message.
- `curl -sf http://127.0.0.1:<port>/` answered within 8–16 s of starting `serve` for every plugin here; the cold
  cost is entirely in `activate` (42 s – 3 m 25 s of `component-dev` per plugin).

## Captures kept / deleted

Kept in `🗑️generated/`: `b1b-<plugin>.png` (5), `b1b-<plugin>-console.txt` (5), `b1b-<plugin>/report.json` (5),
`b1b-<variant>-activate.txt` (4), `b1b-<variant>-serve.txt` (5), `b1b-dag-dev.txt` (the `nx watch` thrash
evidence), `b1b-space-dev.txt` (the 132-task chain). Probe scripts and the two shell helpers live at the ticket
root. Nothing under `🗑️generated` was swept.

## Launch entries

`.claude/launch.json` gained eight purely additive rows (76 insertions, nothing else touched) — one
`<plugin>-react` serve row plus the matching `<plugin>-react-attach` row, following the trailing group's shape:
`reasoning-react`/`-attach` (6015), `imperative-react`/`-attach` (6076), `playbook-react`/`-attach` (6085),
`norm-react`/`-attach` (6091, the `din4108` variant). `dag-react` (6017) and `s-react`/`s-react-served` (6070)
already existed and were left untouched.

All five dev servers started by this slice were stopped at the end of it (ports 6015/6017/6076/6085/6091 free),
as was the `dev s served` chain; no peer process was killed and no orphaned `cargo` was left behind.

## Environment

Host was saturated throughout (peer `cargo`/`rustc` live at every sample — `stdio-pdf`, `trinity`, `puzzle3d`,
`remodel`, `semio-hub`). One activate at a time, `CARGO_PROFILE_WASM_DEV_DEBUG=false`, `NX_DAEMON=false`,
no peer process killed. Headless Chromium with `--use-angle=metal`; the desktop browser pane was never used.
