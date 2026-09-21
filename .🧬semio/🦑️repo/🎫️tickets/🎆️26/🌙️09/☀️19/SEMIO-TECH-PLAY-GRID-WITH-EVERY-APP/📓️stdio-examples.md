# 📓️ stdio-examples — a curated, visible example in every stdio pane (session 5, 2026-09-21)

Topic report (tracked). Scratch, logs and the generator template live under
`🗑️generated/stdio-examples/`.

## Goal and starting point

`📓️play-stdio.md` §"Left open" item 1: the nine stdio panes boot their app's **genesis** document
(Markdown shows the single line `semio stdio.md.dsl v1`, CSV shows "No data", …) because

- no stdio editor declared `ActionDefinition::new("setActiveExample", …)`, so the renderer's
  `appSwitchesExamples` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:416`)
  gated `exampleOptions` to `[]`, hid the navbar picker and never announced a boot example
  (`🏛️ShellHost/🟦️.tsx:10929`); and
- the committed descriptor `✏️s/🔌️plugins/🗄️stdio/🔣️.json` published **one** example in total
  (`demo` on `s.stdio.md@commonmark/*`), although csv/tsv/txt/json/xml/html ship authored ones on disk.

## What the guest actually needs (researched, not assumed)

An app action on an `ArtifactEditor` travels: `dispatch_action` → `AppActionRegistry::get` →
`validate_ui_dispatch_classification` (only `InteractiveJobClassification::Migrated` survives) →
`A::command_from_action` (whose trait default refuses EVERY id) → `admit_command_wire` →
`qualified_tool_proof`, which needs an **exact app-owned `ArtifactOwnedToolJobFactory` registration** —
a generic bounded proof alone is refused with `interactive-job.missing-owned-reducer`. And
`AppActionRegistry::validate_tool_job_rows` demands that `OpBinary::TOOL_JOB_IDS ∩ migrated` equals the
set of `bounded_first_step_tool_proofs!` rows. So a declaration alone is inert; the whole retained
route has to exist. The template followed is `🕸️dag`'s editor (same `ArtifactEditor` shape): the switch
publishes on the **HostOnly** lane and emits `Effect::LoadDocument { pack, spr }` — a whole-document
load outside undo history, so no stdio subset needs a whole-snapshot mutation (three of the nine —
txt, json `*`, xml `*` — have none).

## What landed (absolute paths)

### 1. Shared contract helpers
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`
new `🎬️ExampleSwitch` region: `SET_ACTIVE_EXAMPLE_ACTION_ID`, `example_id_argument`
(`exampleId`/`example_id`/`id`/`value`), `load_example_effect` (pack + `empty_document_spr`),
`set_active_example_action`, `set_active_example_args` — so the nine editors cannot drift on the verb
id, its label, its kind or its argument shape.

### 2. The nine editors (7 crates)
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/…/✏️editor/🦀️.rs`
- `📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs`
- `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs`
- `📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs`

each gaining: a `SetActiveExample { example_id }` command variant plus its `OpText`/`OpBinary` branch,
`OpBinary::TOOL_JOB_IDS`, a `🎬️ExampleSwitch` region (retained roster, payload schema, HostOnly
publication contract, execution contract, example snapshot resolver, `command_from_action` bridge,
`command_id`, extent, reducer, and a concrete `…RetainedCommandJobFactory`), and on the
`ArtifactEditor` impl: `bounded_first_step_tool_proofs!`, `register_tool_job_factories`,
`build_tool_job`, the bounded document-store owners/disposer/initialization job, the full `No…` close
protocol for the config/draft/presence/transient lanes, `command_id`, `command_from_action`, a
`setActiveExample` arm in `handle`, and the manifest rows
`.action_with(..)/.action_args(..)/.action_destructive(..)/.action_interactive_job(.., Migrated)`.

### 3. Two authored examples that did not exist
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/📚️examples/🎬️demo/`
  (`🦀️.rs`, `🟦️.ts`, `🖼️assets/🗣️.dsl.semio`, `🧪️tests/🔬️unit/🦀️.rs`) — an RFC 7493 conformant document.
  Its test asserts BOTH that `print_dsl(parse_dsl(PRIMARY_TEXT)) == PRIMARY_TEXT` (so the asset is the
  crate's own printer output, never hand-matched) and that `check_i_json_conformance` reports no hard issue.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/📚️examples/🎬️demo/`
  — the POSITIVE counterpart of the subset's existing NEGATIVE `🚫️no-doctype` asset (which is documented
  to fail the `valid` gate and therefore cannot be a boot example). Its test asserts
  `check_valid_conformance` reports no hard issue.
  Both wired into their crate roots (`🧾️json/🦀️.rs`, `📰️xml/🦀️.rs`).

### 4. Plugin root
`/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs` — all nine editors now register
through `editor_with_examples` instead of `editor` (16 call sites: both the shipped
`component-app-assembly` roster and the library `full-app-catalog` roster).

### 5. Cargo
`semio-framework-job = { workspace = true }` added to the seven artifact crates'
`📦️packages/🦀️rust/Cargo.toml` (the retained factory's `create_job` names `semio_framework_job::Operation`).

### 6. Play
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` — `"example": "demo"` on the
  eight stdio panes that had none (`stdio` already had it).
- `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts` — `stdio` removed from
  `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` and from `PLUGINS_WITHOUT_A_COMMITTED_DESCRIPTOR` (stdio does
  commit one), and the stale `EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS` comment rewritten.

### 7. New native laws (nine editors)
A `🎬️ExampleSwitchLaws` region in each `✏️editor/🧪️tests/🔬️unit/🦀️.rs`. The decisive one builds a
registry-backed `VcsArtifactApp<EditorApp<E>>` and closes it: `with_registry_on_bus` `.expect()`s the
tool-proof join, so a disagreement between the roster, the publication contract, the `Migrated`
classification and `TOOL_JOB_IDS` **panics at construction**. It immediately earned its keep — see below.

## Evidence from runs I made

| run | result | log |
|---|---|---|
| `cargo check -p semio-s-artifact-stdio-contract --lib` | Finished, clean | — |
| `cargo check` csv (+ 6 sibling crates) `--features component-app-assembly` | Finished, 0 errors, 0 warnings in the touched files | — |
| `cargo test` csv/tsv/txt/json/xml/md/html/contract, first pass | ok 50/37/66/111/88/54/50/2, **0 failed** | `🗑️generated/stdio-examples/test-run1.txt` |
| same, after the new laws | ok **54/41/70/119/96/58/54/2, 0 failed** (2 pre-existing ignored) | `🗑️generated/stdio-examples/test-run2.txt` |
| `cargo check -p semio-s-plugin-stdio --lib` (native, whole plugin root) | Finished in 4m 00s, 0 errors | `🗑️generated/stdio-examples/check-plugin-native.txt` |
| play unit suite, BEFORE the descriptor regeneration | 61 passed / 2 failed — both are the descriptor-dependent example laws | `🗑️generated/stdio-examples/play-unit-before-describe.txt` |

Command shape: `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-stdio-examples CARGO_INCREMENTAL=0
CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test
-p … --lib --tests --no-fail-fast --features component-app-assembly -- --test-threads=4`. The private
target dir was adopted after the ambient one sat ~15 min behind five peers on the artifact-directory lock.

### A real defect the new law caught
Installing `bounded_document_store_owners` obliges the controller to own **every** store it opens: the
first run of the law failed `interactive-job.close-owned-disposer-missing` ("app owner did not provide
the required bounded disposer for config-store"). All nine editors now declare the complete `No…` close
protocol (config/draft owners + disposers, presence/transient disposers + root/peer retirement
factories). Without this the panes would have booted and then failed their close protocol at runtime.

## What is left, and why

1. **The descriptor is not regenerated yet — and the activation lane will NOT do it.**
   `describePluginComponent` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts:324`)
   is the only writer of `✏️s/🔌️plugins/🗄️stdio/{🛂️.descriptor.semio,🔣️.json}`, and its only callers are
   the plugin crates' own `📜️script.ts describe` commands — nothing under `🧑‍💻dev/♻️activation` calls it.
   The activation lane COPIES the owner descriptor into the staged module dir and refuses when the bytes
   differ (`♻️activation/🌐️browser-host/🟦️.ts:207,247`: `descriptorSha256 !== stagedDescriptorSha256 →
   throw`). So `describe` is REQUIRED and must run BEFORE the lane.

   Exact command (run from `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust`, through
   the fleet mutex, env `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432
   DEVELOPER_DIR=/Library/Developer/CommandLineTools`):

   ```
   zsh "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️wasm-build-mutex.sh" \
       stdio-examples -- bun ./📜️script.ts describe
   ```
   (equivalently `bun nx run @semio-tech/stdio-plugin:describe`). **It is already queued and detached** —
   pid 9074, ppid 1, mutex ticket `20260921153110-9074-stdio-examples`; it survives my exit and runs
   unattended when the mutex reaches it. Its log is
   `🗑️generated/stdio-examples/describe.txt` (success prints `described stdio (…) -> ✏️s/🔌️plugins/🗄️stdio (wasm=… core=… descriptor=…)`).
   A second detached watcher (pid 9074's sibling, pid 48657, ppid 1) waits for it, compares the
   descriptor's md5 and appends the verdict to `🗑️generated/stdio-examples/WATCHER.txt`.
   The mutex was held by `tc3c` (13:44–16:14, hub `wasm-release` stdio/gis/note), then `pz1`, then `rb1`
   from 16:56; `c7` and `stdio-a` are still ahead of this ticket.
2. **Then the stdio lane must be re-activated** (coordinator-owned):
   `bun nx run @semio-tech/framework-os-dev:activate-stdio-react-dev`, then the play merge
   (`🏢️semio-tech/🎡️play` `prepare`/`activate`), then a `:6033` recycle
   (`touch "$T/🗑️generated/serve-restart.request"`). Every one of the nine stdio panes shares the same
   component closure, so ONE lane covers all nine. `🗑️generated/activate.request/stdio` is written —
   but honour it only AFTER `describe.txt`/`WATCHER.txt` report the descriptor changed; activating
   earlier stages the stale manifest and has to be repeated.
3. **Browser verification is not done.** It needs (1) and (2) first.
4. **Follow-up (not done, deliberately):** `stdio` can now be dropped from
   `NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts:15`) — it was
   exempt because only one of its editor apps published an example. All nine publish one now, and the
   viewers share their editors' dialects, so `auditNavbarExamplePickerCoverage` would pass. I left it
   in because I cannot verify that audit against a descriptor that does not exist yet.
5. **Not in scope, still broken:** the stdio editors' own edit verbs (`replace-text`, `set-cell`,
   `set-node`) remain undispatchable — they are `TableWindowKit`/`TextWindowKit`/`TreeWindowKit`
   window-kind actions with no app-owned retained route, so `qualified_tool_proof` refuses them with
   `interactive-job.missing-factory`. That is pre-existing (their `command_from_action` did not exist at
   all before this change) and orthogonal to the example picker; giving each one a retained route is the
   same pattern this ticket just established.
