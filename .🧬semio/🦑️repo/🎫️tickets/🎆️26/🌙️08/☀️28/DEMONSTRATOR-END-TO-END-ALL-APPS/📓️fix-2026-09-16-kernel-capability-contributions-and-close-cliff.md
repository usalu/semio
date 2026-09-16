# 🧩️ D1 host capability-topic scoping + D2 retained-config close cliff — 2026-09-16

Repo root `/Users/ueli/Documents/semio`. Executes the two coordinator DECISIONS left open by
[`📓️fix-2026-09-16-sourcing-contributions-envelope.md`](./📓️fix-2026-09-16-sourcing-contributions-envelope.md)
§5.2 (D1) and §5.3 (D2). Both are framework fixes; no app crate changed.

---

## 1. D1 — the host reachability cut blocked every capability topic

### 1.1 Finding (re-measured, not assumed)

`reachableKindsFromUnknown` run over the BUILT manifests of the shipped extensions (script:
`bun` against `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, sources under
`.🧬semio/🦑️repo/⚡️cache/…/🔌️plugin-modules/*/🔣️.json`):

| plugin | topic | operator kinds |
|---|---|---|
| `flow-extension-brep` | `flow.extension` | **98** (`brep.bool.cut`, `brep.curve.arc`, …) |
| `process-extension-wood` | `process.machines` | **0** |
| `cad-extension-aec-building` | `cad.computer` | **0** |
| `sourcing-module-slabs` | `sourcing.module` | **0** |

`contributionReachesKinds` therefore could never be true for the three capability topics, and
`scopeContributionsJson` cut them to `[]` for every foreign contributor. Confirmed §1.4 of the prior
report: process's "11 machines from 4 extensions" were `builtin_installed_catalogs()`.

There is no topic-kind metadata on the registry to key this off: `TopicContribution`
(`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1123`) is `{ topic: string; payload: unknown }` and its
docstring is explicit that the type "does not enumerate topics". So the rule is expressed
structurally, as the coordinator's fallback prescribed.

### 1.2 Design

**A contribution that names no operator kind is a CAPABILITY pack and is never scoped by a document
graph.** Operator reachability stays the cut for operator-keyed topics; what scopes a capability pack
is the consuming app's `consumes` declaration, which the shell already applied when it picked the
receiver (`pluginShouldReceiveContributions` + `appOwnsCommand(app, "setContributions")`).

The sibling defect is the same law one level up: when the receiver's document resolves **no operator
graph at all** (`no-operator-graph` / `no-document-pack` — the normal state of a curation, workshop or
cad document), the publisher returned `unresolved` and skipped the WHOLE push. An empty graph never
scoped a capability pack either, so the run now continues against empty kinds and installs whatever
the closure still yields; `unresolved` survives as the outcome only when that capability pack is
itself empty, and the resolved-kinds cache is NOT poisoned, so a document that resolves later still
widens the pack.

### 1.3 Edits

- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:328` — new `export function contributionIsCapabilityPack`,
  carrying the measurement above in its docstring.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:334` — `contributionPassesScope` replaces
  `contributionReachesKinds`: zero contributed kinds ⇒ pass; otherwise the intersection with the
  document's kinds. The old `if (kinds.size === 0) return false` short-circuit is gone (it is implied
  for an operator-keyed pack).
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:356` — `scopeContributionsJson` calls the new predicate.
- `🧰️…/🛠️ShellHelpers/🧩️contributions/🟦️.ts:77-92` — `run` no longer returns early on an unresolved
  scope; it carries `unresolvedReason` and continues with `kinds = []`.
- `🧰️…/🛠️ShellHelpers/🧩️contributions/🟦️.ts:99` — an empty pack reports `unresolved { reason }` when
  the scope was unresolved, `empty` otherwise, so the diagnosis is not lost.
- `🧰️…/🏛️ShellHost/🟦️.tsx:4610` — the log is now
  `[DEBUG] contributions unresolved document operators — capability packs only` (was "push skipped
  unresolved document operators"): the push is no longer skipped.

### 1.4 Tests added

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️scope-contributions/🟦️.ts:52` —
  *passes every capability pack*: the three real capability payload shapes
  (`process.machines`, `cad.computer`, `sourcing.module`) are `contributionIsCapabilityPack`, all four
  `flow.extension` fixtures are not, and with `kinds = []` the three capability packs cross while
  `flow-extension-bim` is still cut.
- `…:59` — *still cuts an unreachable operator pack when capability packs pass alongside it*: with the
  graph naming only `brep.solid.extrude`, the pack keeps `flow-extension-brep` + the three capability
  packs and drops `bim.wall` and `math.vector`.
- The pre-existing law *drops every foreign contribution when the graph names no operator kind*
  passes unchanged (all four of its fixtures declare operator kinds).
- `🧰️…/🧫️fixtures/🧩️contributions-push/🔣️.json:8,45` — new `capabilityPack` plus scenario
  `unresolved-scope-still-pushes-capability-packs` (2 superseded refreshes, 1 document read, **1
  push**, `installedKeys: ["7::CAPABILITY"]`, outcomes `installed, installed`). The old
  `unresolved-scope-pushes-nothing` scenario is kept verbatim for a closure that holds only
  operator-keyed contributions.
- `🧰️…/🧪️tests/🧩️contributions-push/🟦️.ts:48,58,84` — the harness's `buildPack` now answers empty
  kinds with the capability pack, as the live `scopeContributionsJson` does.

---

## 2. D2 — the ~4 KiB retained-config close cliff

### 2.1 Finding — reproduced in the framework's own crate, then localized

Reproduced with `ToyRunApp` (`🧰️…/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`), which uses the exact
`bounded_config_store_owners` / `bounded_config_store_disposer` pair cad, sourcing and process3d use:
apply one `ChangeTestConfigSelection` of N bytes, render one body, then drive
`close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)`.

```
[STATS] bytes=2909 rendered=true  turns=1834      lastProductive=1834 last=Pending { 1, 26 }
[STATS] bytes=3706 rendered=true  turns=1834      lastProductive=1834 last=Pending { 1, 26 }
[STATS] bytes=4360 rendered=true  turns=14985972  lastProductive=1703 last=Pending { 0, 0 }   ← livelock
```

(log `🗑️generated/close-cliff-repro-1.txt`). The stall state, printed at the stall:
`close_owned_stage=8` — i.e. `close_retained_fields_step` — with `command_log` holding **2** entries:

```
[STATS] cmdLogEntry action=11 label=32   timestamp=13 editId=None configEditIds=[21]
[STATS] cmdLogEntry action=11 label=4391 timestamp=13 editId=None configEditIds=[]
```

**The cause is not the config store at all. It is the command log.** A render calls `refresh_cache`,
which backfills the command log from the store history, and a config edit's LABEL carries the applied
op text — 4 391 B for a 4 360 B config. `close_retained_fields_step` priced the whole log entry
atomically (`action_id + label + timestamp + edit_id`) against one turn's grant, and on
`bytes > maximum_bytes` pushed the entry back and returned `Pending { 0, 0 }`. A value bigger than one
grant can never be afforded, however many turns the host spends — a permanent livelock, not a slow
drain. That is why both halves were required: no render ⇒ no backfilled log ⇒ no oversized label.

Five sibling branches of the same step had the identical refuse-on-over-grant shape
(`config_edit_ids`, `child_edit_ids`, `pending_transaction_proposal.local_ops`, `last_emit_wire`,
`tool_job_controller_id`), each a live livelock for a large enough value.

### 2.2 Design

**A retained field is drained IN PLACE, in pages, never priced atomically against one turn's grant.**
This is the convention the rest of the ladder already uses for byte buffers
(`retire_source_step`, `🦀️.rs:15547`, `:20221`): `released = len.min(grant); truncate(len - released)`.
For a `String` the cut lands on the first char boundary **at or after** the grant's, so a page never
exceeds the grant and a multi-byte scalar is never split.

### 2.3 Edits — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

- `:27313` — new `close_retained_string_page(&mut String, maximum_bytes) -> Option<PluginCloseStep>`,
  with the measurement and the page-never-refuse law in its docstring.
- `:27331` — new `close_retained_bytes_page(&mut Vec<u8>, …)`, the `Vec<u8>` twin.
- `:27343-27377` — `close_retained_fields_step` drains the last command-log entry field by field
  (`config_edit_ids` → `child_edit_ids` → `action_id`/`label`/`timestamp` → `edit_id`) in pages, then
  pops the now-empty entry for `Pending { 1, 0 }`. The atomic `command_log.pop()` pricing is gone.
- `:27415` — `pending_transaction_proposal.local_ops` pages through `close_retained_bytes_page`.
- `:27436` — `last_emit_wire`'s three buffers page instead of refusing an over-grant buffer.
- `:27483` — `tool_job_controller_id` pages.
- `:27392-27400` — `interaction_hover` / `interaction_ui_topology` keys (map keys, not reachable by
  `&mut`) drop with `released_bytes = len.min(grant)` instead of refusing; they can no longer stall.

### 2.4 Test — must fail before, passes after

`🧰️…/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs:1776` `a_retained_config_over_one_envelope_page_closes_after_a_render`
(region `🧹️RetainedConfigCloseCliff`). Six rows — 2 909 / 3 706 / 4 360 / 16 384 / 65 536 B rendered
and 3 820 B unrendered — each asserting terminal-empty, that every page stays inside its grant, and
that **no** close turn spends zero progress.

| row | before the fix | after |
|---|---|---|
| 2 909 B, rendered | closes, 1 834 turns | closes, 1 842 turns |
| 3 706 B, rendered (the real sourcing pack) | closes, 1 834 turns | closes, 1 842 turns |
| 4 360 B, rendered | **livelock, 14 985 972 turns, `Pending { 0, 0 }`** | closes, 1 843 turns |
| 16 384 B, rendered (`PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES`) | — | closes, 1 846 turns |
| 65 536 B, rendered (`CAD_CONFIG_STORE_MAXIMUM_BYTES`) | — | closes, 1 858 turns |
| 3 820 B, no render | closes | closes, 1 823 turns |

The two declared maxima are now cashable; the lane ceiling is no longer one
`ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page. `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` can be
raised (the prior report's §5.3 one-liner) but was left at 2 048 in this pass — that is an app call.

⚠️ Orthogonality: this pass touched only `close_retained_fields_step` and its two new helpers
(hunks `@@ -27282` … `@@ -27429` of `🔌️plugin/🦀️.rs`). The concurrent worker's close-cleanup
stall-credit change lives in `⚛️reactor/🔄️turn/🦀️.rs` and the `plugin_runtime` hunks (`@@ -6523`,
`@@ -32945` …) — disjoint.

---

## 3. Verification

```sh
cd /Users/ueli/Documents/semio
export DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true \
       CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 NX_TUI=false NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false

bun nx run @semio-tech/framework-kernel:test
(cd 🧰️framework/…/🎯️targets/⚛️react/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bun …/vitest.mjs run \
   --config ../../🧪️tests/🎚️config/🟦️.ts contributions-push window-fault wgpu-extension-dispatch)
bun nx run @semio-tech/framework-renderer-react:typecheck
cargo test  -p semio-framework-plugin --lib a_retained_config_over_one_envelope_page_closes_after_a_render -- --nocapture
cargo test  -p semio-framework-plugin --lib tool_run_tests
cargo check --target wasm32-wasip2 -p semio-framework-plugin
bun nx run @semio-tech/demonstrator-plugin:materialize-dev
bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev
```

| gate | result |
|---|---|
| `@semio-tech/framework-kernel:test` | **57 passed, 1 failed** — the failure is `📤️return/📦️content` `can't resolve reference … NonZeroU64` (an ajv `$ref` resolution fault, pre-existing, untouched by this pass). All 8 `scope-contributions` laws incl. the 2 new ones pass. |
| engine `contributions-push` + `window-fault` + `wgpu-extension-dispatch` | **3 files, 30 tests, 0 failed** |
| `@semio-tech/framework-renderer-react:typecheck` | 836 pre-existing errors repo-wide; **0 in any file this pass touched** (grep for `🎠️kernel/🟦️.ts(`, `🧩️contributions/🟦️.ts(`, `🔬️scope-contributions`, `🧩️contributions-push` ⇒ no hits) |
| `cargo test -p semio-framework-plugin --lib a_retained_config…` | **1 passed** (failed before the fix — §2.4) |
| `cargo test -p semio-framework-plugin --lib tool_run_tests` | **35 passed, 0 failed** |
| `cargo test -p semio-framework-plugin --lib close` (69+6) | 69 passed, 6 failed — **all six pre-existing/peer-owned**, none in `close_retained_fields_step`: `app_close_step_drains_at_most_one_segment_and_one_chunk_budget` stalls on the cancellation-cursor branch (`TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS`, upstream of my hunks), the others on `stale roster outcome` / `presence.is_empty()` |
| `cargo test -p semio-framework-plugin --lib` (whole crate) | 94 failures, dominated by `app-definition.interactive-job-classification: unclassified interactive command 'main:resize'` — a classification-registry family unrelated to either fix, in the peer's in-flight hunks |
| `cargo check --target wasm32-wasip2 -p semio-framework-plugin` | **exit 0** |
| `@semio-tech/demonstrator-plugin:materialize-dev` | ok, 6m 20s |
| `@semio-tech/mit-bestand-demonstrator:activate-dev` | see `🗑️generated/demonstrator-activate-dev-capability.txt` |

Logs under `🗑️generated/`: `close-cliff-repro-1.txt` (the failing before-state),
`plugin-lib-tests-after.txt`, `plugin-wasm-check.txt`, `kernel-vitest-1.txt`,
`renderer-react-vitest-1.txt`, `renderer-react-typecheck.txt`,
`demonstrator-materialize-dev-capability.txt`, `demonstrator-activate-dev-capability.txt`.

---

## 4. What the coordinator should now see at `:6029`

### 4.1 Console

For the demonstrator session (focused mode ⇒ only `demonstrator` receives), per app:

1. `[DEBUG] contributions document sources …` — unchanged.
2. For cad / process / puzzle / sourcing, the line that used to read
   `contributions push skipped unresolved document operators` now reads
   **`[DEBUG] contributions unresolved document operators — capability packs only {"plugin":"demonstrator","app":"s.process.process3d…","reason":"no-operator-graph"}`** — and the push
   CONTINUES instead of returning there.
3. `[DEBUG] contributions scoped pack {"chars":…}` with a **non-zero** `chars` (it was
   `contributions push refused empty pack {"chars":2}`).
4. `[DEBUG] contributions push {"plugin":"demonstrator","app":"s.process.process3d@1/*#play",…,"crossings":1,"encoding":"pack","skipped":null}`
   (`runtimeDiagnosticsEnabled()` must be on for this one).
5. **`[DEBUG] contributions publish {"plugin":"demonstrator","instanceId":N,"outcome":{"status":"installed","chars":…,"kinds":[]}}`**
   (`🏛️ShellHost/🟦️.tsx:4705`). `kinds: []` is the signature of a capability-only pack —
   an operator-keyed push carries a populated `kinds`.
6. No `runtime close cleanup faulted … zero-progress` from a retained config, and no silent stall on
   pane close after a large `setContributions` (D2).

If you still see `refused empty pack` for an app, its extensions are disabled in the registry
(`disabledExtensionIds`) — that cut is upstream of `scopeContributionsJson` and deliberate.

### 4.2 Process workshop — telling a contributed machine from a built-in

`installed_catalogs` (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs:2054`) is
`builtin_installed_catalogs()` **concatenated with** `contributed_machine_catalogs(contributions_json)`,
**with no dedupe by `catalog_id`**. Built-ins are five sections in fixed order —
`generic`, `wood`, `concrete`, `metal`, `robotic` — and the contributed ones are appended after them
in host pack order.

So the tell is **position and duplication**: with the host push landing you get **nine** catalog
sections, the last four being a second `Wood` / `Metal` / `Concrete` / `Robotic` after the built-in
five, because the four `process-extension-*` bundles contribute the SAME `catalog_id`s the crate
already compiles in (`🧩️extensions/🪵️wood/🦀️.rs:168` contributes `moduleId = catalog.catalog_id() =
"wood"`). Anything after the fifth section came off the wire; the first five never do.

That duplication is the correct *proof* the push landed and the wrong *product*. Sourcing already
solved it at the app boundary (`schema::installable_contributions` drops a contributed id an
installed module already serves — §2.3 of the prior report); **process3d has no such filter**. Follow-up
for the process owner: dedupe `installed_catalogs` by `catalog_id`, built-in wins, exactly as
`sourcing_modules` does. Until then, expect nine sections, not eleven-machines-from-four-extensions.

`cad` has the same shape but is invisible: `syncCadComputerContributions` falls back to
`shippedCadComputerContributionsJson` when no push arrives, so the four modules `spatial-shape`,
`aec-building`, `aec-building-energy`, `aec-building-structure` appear either way — the `[DEBUG]
contributions publish … status installed` line is the only proof there.

---

## 5. Not done here

- Raising `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` past 2 048 now that the close cliff is gone
  (app-owned, one constant).
- Deduping `installed_catalogs` by `catalog_id` in process3d (§4.2).
- The 94 pre-existing `semio-framework-plugin --lib` failures, and the kernel's
  `📤️return/📦️content` ajv `$ref` failure.
