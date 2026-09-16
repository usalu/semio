# 🧩️ D1 capability-topic scoping, the per-app contributions admission, and the D2 close cliff — 2026-09-16

Repo root `/Users/ueli/Documents/semio`. Executes the two coordinator DECISIONS left open by
[`📓️fix-2026-09-16-sourcing-contributions-envelope.md`](./📓️fix-2026-09-16-sourcing-contributions-envelope.md)
§5.2 (D1) and §5.3 (D2), plus the admission ceilings the coordinator added once D1 made the real
packs actually cross.

Three defects, one chain: the host CUT every capability pack (§1) → nothing ever reached a guest, so
the guests' `setContributions` wire and retained-config envelopes had never been exercised and all
three refused the real pack (§2) → and a retained config past ~one envelope page could not close at
all (§3).

---

## 1. D1 — the host scoping rule

### 1.1 Finding (measured, not assumed)

`reachableKindsFromUnknown` over the BUILT dev manifests
(`🧰️…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/*/🔣️.json`):

| plugin | topic | operator kinds |
|---|---|---|
| `flow-extension-brep` | `flow.extension` | **98** (`brep.bool.cut`, `brep.curve.arc`, …) |
| `process-extension-wood` | `process.machines` | **0** |
| `cad-extension-aec-building` | `cad.computer` | **0** |
| `sourcing-module-slabs` | `sourcing.module` | **0** |

`contributionReachesKinds` could therefore never be true for the three capability topics, and
`scopeContributionsJson` cut them to `[]` for every foreign contributor. Confirmed §1.4 of the prior
report: process's "11 machines from 4 extensions" were `builtin_installed_catalogs()`.

### 1.2 Design — two cuts, one per topic kind

**An OPERATOR-KEYED contribution is cut by reachability from the open document's graph. A CAPABILITY
pack — one that names no operator kind at all — is cut by the receiver's `consumes` row in the plugin
registry.**

The registry DOES carry the topic metadata the coordinator asked me to prefer
(`🧰️…/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts`): `demonstrator` consumes
`["forms.questionKind","flow.extension","process.machines","cad.computer","sourcing.module"]`, `cad`
consumes `["cad.computer"]`, `sourcing` `["sourcing.module"]`, `process` `["process.machines"]`.

⚠️ Passing EVERY capability pack (the first cut of this fix) is wrong and was measured wrong: `gis`
contributes a **196 400-byte** `stdio.artifact-catalog.v1` that no plugin consumes, which made the
demonstrator pack **226 310 bytes** and blew every app's wire admission. With the `consumes` cut:

| receiver | pack chars | entries |
|---|---|---|
| `demonstrator` | **29 909** | 4 `cad.computer` + 4 `process.machines` + 3 `sourcing.module` |
| `cad` | 6 385 | 4 `cad.computer` |
| `process` | 19 823 | 4 `process.machines` |
| `sourcing` | 3 703 | 3 `sourcing.module` |
| `procedural` / `puzzle` | 2 (`[]`) | — (their topics are operator-keyed or unconsumed) |

The sibling defect is the same law one level up: when the receiver's document resolves **no operator
graph at all** (`no-operator-graph` / `no-document-pack` — the normal state of a curation, workshop or
cad document), the publisher returned `unresolved` and skipped the WHOLE push. An empty graph never
scoped a capability pack either, so the run now continues against empty kinds; `unresolved` survives
as the outcome only when the resulting pack is itself empty, and the resolved-kinds cache is NOT
poisoned, so a document that resolves later still widens the pack.

### 1.3 Edits

- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:338` — new `export function contributionIsCapabilityPack`.
- `…/🟦️.ts:344` — `topicOf`, the contribution's declared topic.
- `…/🟦️.ts:350` — `contributionPassesScope` replaces `contributionReachesKinds`: zero contributed
  kinds ⇒ pass iff `consumed` holds the topic; otherwise the intersection with the document's kinds.
- `…/🟦️.ts:377-392` — `scopeContributionsJson` takes a 4th `consumedTopics` parameter (default `[]`,
  i.e. forward NO foreign capability pack), with the `gis` measurement in its docstring.
- `🧰️…/🛠️ShellHelpers/🧩️contributions/🟦️.ts:77-92,99` — `run` no longer returns early on an
  unresolved scope; it carries `unresolvedReason`, continues with `kinds = []`, and reports
  `unresolved { reason }` only when the pack is empty.
- `🧰️…/🏛️ShellHost/🟦️.tsx:4642` — `ContributionsEnvironment.consumedTopics`;
  `:4758` fills it from the live `registry` row; `:4682-4684` passes it to `scopeContributionsJson`
  and logs `consumes` alongside `kinds`; `:4610` the log is now
  `[DEBUG] contributions unresolved document operators — capability packs only`.
- `🧰️…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:88,730,740` — `wgpuConsumedTopics` off
  `PLUGIN_CATALOG`, defaulted into `wgpuBuildScopedContributionsPack`.

### 1.4 Tests (vitest)

`🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️scope-contributions/🟦️.ts`:
- `:53` *passes a capability pack the receiver consumes* — the three real capability payload shapes
  are `contributionIsCapabilityPack`, the four `flow.extension` fixtures are not, and with **empty
  kinds** the three cross to `demonstrator` while `flow-extension-bim` is still cut.
- `:61` *cuts a capability pack on a topic the receiver does not consume* — `gis`'s
  `stdio.artifact-catalog.v1` is dropped for `demonstrator`, and `sourcing` receives only
  `sourcing-module-beams`.
- `:69` *still cuts an unreachable operator pack when capability packs pass alongside it*.
- The pre-existing law *drops every foreign contribution when the graph names no operator kind*
  passes unchanged.

`🧰️…/🧫️fixtures/🧩️contributions-push/🔣️.json:8,45` — new `capabilityPack` and scenario
`unresolved-scope-still-pushes-capability-packs` (2 superseded refreshes, 1 document read, **1 push**,
`installedKeys: ["7::CAPABILITY"]`, outcomes `installed, installed`); the old
`unresolved-scope-pushes-nothing` scenario is kept verbatim for an operator-keyed-only closure.
`🧰️…/🧪️tests/🧩️contributions-push/🟦️.ts:48,58,84` — the harness's `buildPack` answers empty kinds
with the capability pack, as the live `scopeContributionsJson` does.

---

## 2. The per-app `setContributions` admission (`typed command raw JSON exceeds its registered retained-page admission`)

### 2.1 Finding

Once §1 let the packs cross, `admit_command_json_with_proof`
(`🧰️…/🔌️plugin/🦀️.rs:21438`) refused them: it re-encodes the invocation as
`["setContributions",{"json":…}]` and prices it against the ADDRESSED TOOL's
`max_raw_wire_bytes`, and cad, sourcing and process3d all priced `setContributions` on the 8 KiB
**gesture** envelope every retained tool of the app shares. The demonstrator's real command wire is
**38 081 bytes**.

Three further real ceilings sat behind it, all only reachable once a pack actually arrives:

1. **process3d's retained config** — `PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES = 16_384` and
   `admit_process3d_config_mutation` priced `SetContributions` on `PROCESS3D_RETAINED_RAW_BYTES =
   8_192`, while process3d retains `contributions_json` whole.
2. **process3d's contribution envelope scanner** — `PROCESS_CONTRIBUTION_MAX_STRING_BYTES = 4 * 1024`,
   but the REAL `machinesJson` of the shipped bundles is 3 311 / 4 010 / 4 011 / **4 705** bytes. The
   wood catalog was silently refused: `contributed_machine_catalogs` returned nothing and the workshop
   showed built-ins only, whatever the host pushed.
3. **process3d's resumable work units** — `extent = len / PROCESS3D_SCAN_BYTES` capped at
   `PROCESS3D_RETAINED_WORK_ITEMS = 64`, i.e. a hard 16 384-byte ceiling on `setContributions`.

A factory declares ONE `execution_contract()` for every tool it serves, and the proof catalogue must
join it exactly (`interactive-job.catalog-authority`, `typed_join=false`), so a per-tool proof
contract is not expressible without a per-tool factory (procedural's
`Generation3dContributionsJobFactory` is the one app that already has one). The contract is therefore
widened per FACTORY, and the real per-tool ceiling is enforced one layer deeper — in the payload's
`maximum_raw_bytes` and in the wire factory — where it is exactly right.

### 2.2 Edits

**Framework** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13042`
`pub const CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES: usize = 49_152`, re-exported at `:38814`. 48 KiB:
above the measured 38 081-byte wire, under the 64 KiB contiguous guest-allocation ceiling
(`ArtifactRetainedCommandPayload::try_new` does `try_reserve_exact(maximum_raw_bytes)`, so this is a
real reservation per job, which is why procedural's 8 MiB `COMMAND_MAXIMUM_BYTES` is not the figure to
copy here).

**cad** (`✏️s/🔌️plugins/📐️cad/…/✏️editor/🦀️.rs`)
- `:1477` `cad_retained_contract()` is now priced on the contributions wire, with the one-contract-per
  -factory law in its docstring.
- `:1484` new `cad_retained_raw_bytes(tool_id)` — the real per-tool ceiling.
- `:1546` the wire factory refuses past `payload.maximum_raw_bytes` (was the shared constant).
- `:2168` the payload reserves `cad_retained_raw_bytes(tool_id)`.

**sourcing** (`✏️s/🔌️plugins/🪵️sourcing/…/✏️editor/🦀️.rs`)
- `:287` new `sourcing_curation_retained_raw_bytes(tool_id)`; `:301`
  `sourcing_curation_bounded_contract()` priced on the contributions wire and used by every proof row
  (they were 14 literal copies of the same contract).
- `:997` the payload reserves per tool; the wire factory refuses past `payload.maximum_raw_bytes`.

**process3d** (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs`)
- `:319-326` `PROCESS3D_CONFIG_FILTER_STORE_BYTES` (16 384, unchanged behaviour) +
  `PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES` (24 576, sized from the real 19 823-byte `process.machines`
  share); `PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES` is their sum.
- `:332` `process3d_retained_raw_bytes(tool_id)`; `:344` `process3d_retained_work_items(tool_id)`
  (wire ceiling ÷ scan granularity), `:389` both used by `process3d_resumable_extent`.
- `:369` `process3d_resumable_contract()` priced on the contributions wire.
- `:735` `admit_process3d_config_mutation` routes `SetContributions` to the contributions lane.
- `:2007` `PROCESS_CONTRIBUTION_MAX_STRING_BYTES` 4 KiB → **16 KiB**, with the measured real
  `machinesJson` sizes in its docstring.
- `:2081` new `pub(crate) fn installable_contributions(json, maximum_bytes)` — distils the host pack to
  the `process.machines` entries addressed to this app, in host order, while the re-encoded roster
  fits the lane (the `sourcing.module` precedent). Wired into the command handler
  (`✏️editor/🎮️commands/🧩️contribution/🦀️.rs:24`) and the host bridge (`✏️editor/🦀️.rs:1657`), so the
  app never retains the whole closure.

### 2.3 Tests — one per app, on the real extension-shaped pack

| app | test | measured |
|---|---|---|
| process3d | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:1172` `the_real_demonstrator_pack_is_admitted_distilled_and_retained` — the four `process.machines` packs built from this crate's OWN `MachineCatalog` impls (the extensions emit the same payload) plus the foreign `cad.computer` / `sourcing.module` entries | pack **20 325** B, wire **26 537** B; distils to exactly 4 entries; `admit_process3d_config_mutation` + `prepare_process3d_config` accept it; `installed_catalogs` yields built-ins + 4 |
| cad | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:1977` `the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire` — the verbatim `shipped_cad_computer_contributions()` mirror plus the real per-entry bulk of the seven foreign entries | pack **20 846** B, wire **21 282** B; past `CAD_RETAINED_RAW_BYTES`; inside the config lane; `validate_cad_computer_contributions` installs exactly the 4 cad modules and ignores the foreign topics |
| sourcing | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:335` `the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire` | past the gesture envelope; the wire fits the registered admission; what is RETAINED is `installable_contributions`'s distilled roster, inside `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` |

Also updated: cad's `the_shipped_cad_computer_pack_is_admitted_by_the_retained_config_envelope` no
longer pins the 4 KiB close budget (that was the D2 livelock, now fixed — §3), and process3d's
`retained_resumable_extent_accepts_exact_byte_maximum_and_rejects_max_plus_one` now pins the
contributions lane for `setContributions` AND that a GESTURE past its own envelope is still refused.

---

## 3. D2 — the ~4 KiB retained-config close cliff

### 3.1 Finding — reproduced in the framework's own crate, then localized

Reproduced with `ToyRunApp` (`🧰️…/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`), which uses the exact
`bounded_config_store_owners` / `bounded_config_store_disposer` pair cad, sourcing and process3d use:
apply one config mutation of N bytes, render one body, then drive
`close_step(1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)`.

```
[STATS] bytes=2909 rendered=true  turns=1834      lastProductive=1834 last=Pending { 1, 26 }
[STATS] bytes=3706 rendered=true  turns=1834      lastProductive=1834 last=Pending { 1, 26 }
[STATS] bytes=4360 rendered=true  turns=14985972  lastProductive=1703 last=Pending { 0, 0 }   ← livelock
```

(log `🗑️generated/close-cliff-repro-1.txt`). The stall state, printed at the stall:
`close_owned_stage=8` — `close_retained_fields_step` — with `command_log` holding **2** entries:

```
[STATS] cmdLogEntry action=11 label=32   timestamp=13 editId=None configEditIds=[21]
[STATS] cmdLogEntry action=11 label=4391 timestamp=13 editId=None configEditIds=[]
```

**The cause is not the config store. It is the command log.** A render calls `refresh_cache`, which
backfills the command log from the store history, and a config edit's LABEL carries the applied op
text — 4 391 B for a 4 360 B config. `close_retained_fields_step` priced the whole log entry
atomically (`action_id + label + timestamp + edit_id`) against one turn's grant and, on
`bytes > maximum_bytes`, pushed the entry back and returned `Pending { 0, 0 }`. A value bigger than one
grant can never be afforded, however many turns the host spends — a permanent livelock, not a slow
drain. That is why both halves were required: no render ⇒ no backfilled log ⇒ no oversized label.

Five sibling branches of the same step had the identical refuse-on-over-grant shape
(`config_edit_ids`, `child_edit_ids`, `pending_transaction_proposal.local_ops`, `last_emit_wire`,
`tool_job_controller_id`).

### 3.2 Design

**A retained field is drained IN PLACE, in pages, never priced atomically against one turn's grant.**
This is the convention the rest of the ladder already uses for byte buffers (`retire_source_step`,
`🦀️.rs:15547`, `:20221`): `released = len.min(grant); truncate(len - released)`. For a `String` the cut
lands on the first char boundary **at or after** the grant's, so a page never exceeds the grant and a
multi-byte scalar is never split.

### 3.3 Edits — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

- `:27576` new `close_retained_string_page(&mut String, maximum_bytes) -> Option<PluginCloseStep>`,
  with the measurement and the page-never-refuse law in its docstring.
- `:27594` new `close_retained_bytes_page(&mut Vec<u8>, …)`, the `Vec<u8>` twin.
- `:27606-27640` `close_retained_fields_step` drains the last command-log entry field by field
  (`config_edit_ids` → `child_edit_ids` → `action_id`/`label`/`timestamp` → `edit_id`) in pages, then
  pops the now-empty entry for `Pending { 1, 0 }`. The atomic `command_log.pop()` pricing is gone.
- `pending_transaction_proposal.local_ops`, `last_emit_wire`'s three buffers and
  `tool_job_controller_id` page instead of refusing; `interaction_hover` /
  `interaction_ui_topology` keys (map keys, not reachable by `&mut`) drop with
  `released_bytes = len.min(grant)` instead of refusing, so they can no longer stall either.

### 3.4 Test — fails before, passes after

`🧰️…/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs:1776` `a_retained_config_over_one_envelope_page_closes_after_a_render`
(region `🧹️RetainedConfigCloseCliff`). Six rows, each asserting terminal-empty, that every page stays
inside its grant, and that **no** close turn spends zero progress.

| row | before | after |
|---|---|---|
| 2 909 B, rendered | closes, 1 834 turns | closes, 1 842 turns |
| 3 706 B, rendered (the real sourcing pack) | closes, 1 834 turns | closes, 1 842 turns |
| 4 360 B, rendered | **livelock, 14 985 972 turns, `Pending { 0, 0 }`** | closes, 1 843 turns |
| 16 384 B, rendered (`PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES`, old) | — | closes, 1 846 turns |
| 65 536 B, rendered (`CAD_CONFIG_STORE_MAXIMUM_BYTES`) | — | closes, 1 858 turns |
| 3 820 B, no render | closes | closes, 1 823 turns |

Every declared config maximum is now cashable; the lane ceiling is no longer one
`ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page. That is what made §2's 24 576-byte process3d
contributions lane safe to declare at all.

⚠️ Orthogonality: this pass touched only `close_retained_fields_step` and its two new helpers. The
concurrent worker's close-cleanup stall-credit change lives in `⚛️reactor/🔄️turn/🦀️.rs` and the
`plugin_runtime` hunks — disjoint.

---

## 4. Verification

```sh
cd /Users/ueli/Documents/semio
export DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true \
       CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 NX_TUI=false NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false

bun nx run @semio-tech/framework-kernel:test
(cd 🧰️framework/…/🎯️targets/⚛️react/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bun …/vitest.mjs run \
   --config ../../🧪️tests/🎚️config/🟦️.ts contributions-push window-fault wgpu-extension-dispatch)
cargo test  -p semio-framework-plugin --lib a_retained_config_over_one_envelope_page_closes_after_a_render -- --nocapture
cargo test  -p semio-s-artifact-cad-cad --lib contributions -- --nocapture
cargo test  -p semio-s-artifact-sourcing-curation --lib
cargo test  -p semio-s-artifact-process-process3d --lib -- --skip vcs_artifact_app_production_maintenance_swap
cargo check --target wasm32-wasip2 -p semio-framework-plugin -p semio-s-artifact-cad-cad \
            -p semio-s-artifact-sourcing-curation -p semio-s-artifact-process-process3d
cargo check --target wasm32-wasip2 -p semio-s-plugin-cad -p semio-s-plugin-sourcing -p semio-s-plugin-process
bun nx run @semio-tech/demonstrator-plugin:materialize-dev
bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev
```

| gate | before | after |
|---|---|---|
| `@semio-tech/framework-kernel:test` | 57 passed, 1 failed | **58 passed, 1 failed** — the failure is `📤️return/📦️content` `can't resolve reference … NonZeroU64` (an ajv `$ref` fault, pre-existing, untouched). All **9** `scope-contributions` laws pass, incl. the 3 new ones |
| engine `contributions-push` + `window-fault` + `wgpu-extension-dispatch` | 30 tests | **3 files, 33 tests, 0 failed** |
| `cargo test -p semio-framework-plugin --lib a_retained_config…` | **livelock** (§3.4) | **1 passed** |
| `cargo test -p semio-s-artifact-cad-cad --lib contributions` | 3 passed | **4 passed, 0 failed** (`[STATS] cad demonstrator pack packChars=20846 wireChars=21282`) |
| `cargo test -p semio-s-artifact-sourcing-curation --lib` | 143 passed, 1 ignored | **144 passed, 0 failed, 1 ignored** |
| `cargo test -p semio-s-artifact-process-process3d --lib` | 294 passed, **45 failed** | **308 passed, 33 failed** — 14 more pass, 12 fewer fail; the remaining 33 are the pre-existing `typed-operation pending publication rejected a stale immutable document root` family |
| `cargo check --target wasm32-wasip2` (4 artifact crates + 3 plugin crates) | — | **exit 0, no output** |
| `@semio-tech/demonstrator-plugin:materialize-dev` | — | **ok, 9m 13s** |
| `@semio-tech/mit-bestand-demonstrator:activate-dev` | — | **ok, 42m 25s, 83 tasks, `Prepared Demonstrator dev: 7 runtime variants`** |
| pack re-measured against the RESTAGED dev manifests | 226 310 B / 12 entries (capability-pass-all) | **29 909 B / 11 entries** for `demonstrator`; 6 385 / 3 703 / 19 823 for standalone cad / sourcing / process; `[]` for procedural and puzzle |

Logs under `🗑️generated/`: `close-cliff-repro-1.txt` (the failing before-state),
`plugin-lib-tests-after.txt`, `plugin-wasm-check.txt`, `contributions-wasm-check.txt`,
`kernel-vitest-1.txt`, `renderer-react-vitest-1.txt`, `renderer-react-typecheck.txt`,
`process3d-serial.txt`, `demonstrator-materialize-dev-3.txt`, `demonstrator-activate-dev-{3,4,5}.txt`
(3 and 4 are the two disk-full / peer-mid-edit failures described below; **5 is the green run**).

### 4.1 Two things this pass did NOT cause, for the record

- **`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` (process3d) spins.**
  It is skipped above. It is NOT this pass's: the whole process3d suite ran in 26.15 s WITH every
  process3d edit of §2 already in place, and the test still spins when
  `CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES` is set back to process3d's original 8 192 (measured twice,
  11 min and 70 min at ~45 % CPU, `sample` showing the time in `native_pool::worker_loop`). It drives
  a 200 000-turn `maintenance_step` loop over six apps; the change between the green run and the spin
  is framework work by the concurrent worker, not any file in §1–§3.
- **`@semio-tech/framework-renderer-react:typecheck`** is 836 errors repo-wide and was before; **0** of
  them are in any file this pass touched (grep for `🎠️kernel/🟦️.ts(`, `🧩️contributions/🟦️.ts(`,
  `🔬️scope-contributions`, `🧩️contributions-push` ⇒ no hits).
- **Disk** cost three `activate-dev` attempts, none of them a code fault:
  attempts 1 and 3 died on `rustc-LLVM ERROR: IO failure on output stream: No space left on device`
  (the shared build dir was at 5–23 GiB free and the wasm-dev incremental cache regrows ~10–20 GB/h
  under the peer fleet); attempt 2 died on a transient `E0308 expected TypedOperationUiProgress, found
  UiDirtyScope` at `⏯️tool-run/🦀️.rs:1329` — a peer's mid-edit state, gone on re-read and
  `cargo check -p semio-framework-plugin` green immediately after. Freed by pruning cargo incremental
  directories untouched for >30/60/90 min and by dropping `⚡️cache/cargo/build/wasm32-wasip2/debug`
  (30 GB of `cargo check` output, idle >20 min). All of that is derived cache cargo regenerates;
  nothing else was deleted and no peer process was killed.

---

## 5. What the coordinator should now see at `:6029`

### 5.1 Console, per demonstrator app

For the demonstrator session (focused mode ⇒ only `demonstrator` receives; the target app is whichever
of its panes is open, and cad / sourcing / process3d apps are the cad/sourcing/process crates' own):

1. `[DEBUG] contributions document sources …` — unchanged.
2. Where cad / process / puzzle / sourcing used to log `contributions push skipped unresolved document
   operators`, they now log
   **`[DEBUG] contributions unresolved document operators — capability packs only`**
   `{"plugin":"demonstrator","app":"s.process.process3d@1/*#editor","reason":"no-operator-graph"}`
   — and the push CONTINUES instead of returning there.
3. **`[DEBUG] contributions scoped pack`**
   `{"chars":29909,"hasManifestJson":false,"hasPolygon":false,"kinds":[],"consumes":["forms.questionKind","flow.extension","process.machines","cad.computer","sourcing.module"]}`
   — it was `contributions push refused empty pack {"chars":2}`. `chars` ≈ **29 909** is the signature
   of the fix; `consumes` is the new field and must NOT contain `stdio.artifact-catalog.v1`.
4. `[DEBUG] contributions push {"plugin":"demonstrator","app":"s.cad.cad@1/*#editor",…,"crossings":1,"encoding":"pack","skipped":null}`
   (only with `runtimeDiagnosticsEnabled()`).
5. **`[DEBUG] contributions publish {"plugin":"demonstrator","instanceId":N,"outcome":{"status":"installed","chars":29909,"kinds":[]}}`**
   (`🏛️ShellHost/🟦️.tsx:4705`) — the proof line, for `koordinator` (`s.cad.cad@1/*#editor`),
   `aussuchen` (`s.sourcing.curation@1/*#editor`) and `bearbeiten`
   (`s.process.process3d@1/*#editor`) alike. **`kinds: []` is the signature of a capability-only pack**;
   an operator-keyed push carries a populated `kinds`.
6. What must be GONE: `setContributions command failed … typed command raw JSON exceeds its registered
   retained-page admission`, `contributions push refused empty pack`, and any
   `runtime close cleanup faulted … zero-progress` traceable to a retained config.

If an app still logs `refused empty pack`, its extensions are disabled in the registry
(`disabledExtensionIds`) — that cut is upstream of `scopeContributionsJson` and deliberate.

### 5.2 Process workshop — telling a contributed machine from a built-in

`installed_catalogs` (`✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️.rs`) is `builtin_installed_catalogs()`
**concatenated with** `contributed_machine_catalogs(contributions_json)`, **with no dedupe by
`catalog_id`**. Built-ins are five sections in fixed order — `geometry`, `wood`, `concrete`, `metal`,
`robotic` — and contributed ones are appended after them in host order.

So the tell is **position and duplication**: with the push landing you get **nine** catalog sections,
the last four being a second `Wood` / `Metal` / `Concrete` / `Robotic` after the built-in five,
because the four `process-extension-*` bundles contribute the SAME `catalog_id`s the crate already
compiles in (`🧩️extensions/🪵️wood/🦀️.rs:168` contributes `moduleId = catalog.catalog_id() = "wood"`).
Anything after the fifth section came off the wire; the first five never do. The process3d unit test
above pins exactly that: `installed_catalogs(distilled).len() == builtin.len() + 4`.

That duplication is the correct *proof* the push landed and the wrong *product*. Sourcing already
solves it at the app boundary (`schema::installable_contributions` drops a contributed id an installed
module already serves); process3d's distillation added here filters by TOPIC and APP, not by
`catalog_id`. **Follow-up for the process owner**: dedupe `installed_catalogs` by `catalog_id`,
built-in wins, exactly as `sourcing_modules` does. Until then, expect nine sections.

`cad` has the same shape but is invisible: `syncCadComputerContributions` falls back to
`shippedCadComputerContributionsJson` when no push arrives, so the four modules `spatial-shape`,
`aec-building`, `aec-building-energy`, `aec-building-structure` appear either way — the
`contributions publish … status installed` line is the only proof there.

---

## 6. Not done here

- Raising `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` past 2 048 now that the close cliff is gone
  (app-owned, one constant; sourcing distils, so it does not need it).
- Deduping process3d's `installed_catalogs` by `catalog_id` (§5.2).
- Giving `setContributions` its own job factory per app, which would let the proof catalogue carry a
  per-tool contract instead of widening the factory's (§2.1). Procedural is the working precedent.
- The 33 pre-existing process3d `--lib` failures, the spinning
  `vcs_artifact_app_production_maintenance_swap…` (§4.1), and the kernel's
  `📤️return/📦️content` ajv `$ref` failure.
