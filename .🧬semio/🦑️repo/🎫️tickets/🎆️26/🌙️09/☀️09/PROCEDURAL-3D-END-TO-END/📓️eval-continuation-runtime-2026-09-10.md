# Eval Continuation Runtime — Why The Served Flow Stalled At Its First Extension Node, And The Fix

Ticket 2026/09/09/PROCEDURAL-3D-END-TO-END, lane "flow eval / extension round trip / brep extension
path". Session ⚪9f5f6952 (Fable 5.1). Repo MCP was down all session (`invalid initialize params`);
ticket bookkeeping is on disk and no ticket was opened/closed/reopened.

Continues `📓️extension-round-trip-2026-09-09.md` (the round trip's three hops) and answers
`📓️runtime-verification-2026-09-09.md` boot #3's first blocker: the flow graph reached the renderer
with `extrusion-axis: computing`, `extrude: queued` and `data-meshes-json="[]"` forever.

---

## 1. TL;DR

The round trip built yesterday is correct end to end. What was wrong is the **address** on the wire.

`Effect::InvokeExtension.extension_id` is the id the HOST resolves an extension actor by, and the
host resolves it against a loaded program's **`pluginId`** — `flow-extension-math`,
`flow-extension-brep`. The flow registry was putting the **flow manifest's own id** there — `math`,
`brep` — and explicitly throwing the owning plugin id away
(`let _ = plugin_id;`). No loaded plugin is named `math`, so the shell answered every single
evaluation with `extension.missing`; the app's `flowEvalResolve` then decoded an empty `outputJson`,
`seed_node_cache` failed, the tick re-armed and re-dispatched the SAME node forever. Nothing ever
progressed past the first extension-evaluated node, and no mesh was ever tessellated.

Two places carried the defect, one on each side of the wire; both are fixed:

| # | file:line | before | after |
|---|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:124` (`register_contributed_manifest`) | `let extension_id = manifest.id.clone();` … `let _ = plugin_id;` | `let invocation_address = plugin_id.to_string();` — the stub raises the CONTRIBUTING PLUGIN's id |
| 2 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs:2233` (`preview_tessellate_invocations`) | `ExtensionInvocation::new("brep", "tessellate", …)` — a hard-coded flow-domain id | resolved through `flow_extension_plugin_id(GENERATION_3D_GEOMETRY_EXTENSION_ID)`, i.e. `flow-extension-brep` |

and one dead branch on the host side was removed rather than repaired:

| # | file:line | what |
|---|---|---|
| 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1615` | the resolver's fallback `entry.manifest.contributions?.some(c => "extensionId" in c && c.extensionId === extensionId)` scanned a lane that **no manifest has ever carried** (`manifest.contributions` is the `playbookBlockKind` lane; contributed extension payloads live under `manifest.topicContributions[].payload`). It matched nothing, always, and made the miss silent. Deleted: `extensionId` IS a `pluginId`, full stop, and a domain that names its extensions differently translates on ITS side. |

`flow_extension_plugin_id` (`📔️registry/🦀️.rs:302`) existed with **zero callers** since it was
written — it is the translation surface that was never wired. It now has two: fix 2 and the new law
test.

---

## 2. Native reproduction (task 1)

### 2.0 Why the existing suite could not see it

`renders_world_preview_scene` and `switching_active_example_changes_preview_meshes` were **green**
against the defect (`🗑️generated/eval-preview-subset.txt` records the same five green after the fix;
they were equally green before it). The reason is the fixture wiring, not luck:

`crate::flow_operators::installed()` installed the two packaged kernels **LINKED** —
`install_flow_extension(FlowExtensionSpec { … install })`, real `OperatorImpl`s inside the test
process. A linked operator never yields `EvalError::PendingExtension`, so **no evaluate invocation is
ever emitted**, so the address on it is never exercised. In the browser the same two kernels arrive
as `flow.extension` **topic contributions** (`buildContributionsJson` →
`sync_host_flow_extension_contributions`), every operator becomes a `ContributedExtensionStub`, and
every geometry node crosses the wire. The suite was measuring a different machine from the one that
ships.

**Fix:** `🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` now contributes both manifests under the
plugin ids their own `ExtensionBundle::new(..)` declares —
`install_flow_extension_manifest("flow-extension-brep", …)` /
`install_flow_extension_manifest("flow-extension-math", …)`. The in-process host half
(`🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs`) now resolves a capability **by plugin id**, which is
exactly what `dispatchInvokeExtensionEffect` does against `handle.pluginId`, and answers an
unresolvable address with `extension.missing` instead of a private `extension.unsupported`.

### 2.1 The new test

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
→ `hex_column_evaluates_end_to_end_through_the_extension_round_trip`

Boots the registered app on the boot fixture `hexagonal-mushroom-column` with the real `brep`+`math`
kernels, drives the chain through `testkit::drain_flow_eval_ticks` — which uses the SDK's own drain
(`reactor::drain_queued_effects`), the SDK's own continuation resolution
(`reactor::take_extension_response`) and re-dispatch through `PluginApp::handle_command`, the exact
three steps `⚛️reactor/🔄️turn` performs on `Event::Completed`; no handler is called directly — and then
asserts, on the REAL projected surfaces:

- every one of the seven widgets reaches `{"status":"ok"}` in the flow window's `statusJson`
  (`height`, `radius`, `sides`, `profile`, `extrusion-axis`, `extrude`, `column-preview`);
- the preview body decodes as a world-3d scene with **≥ 1 mesh**.

Command (private target, `RUSTC_WRAPPER=""`):

```
CARGO_TARGET_DIR=$S/target-eval RUST_MIN_STACK=536870912 \
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib \
  -- --test-threads=2 --nocapture hex_column_evaluates_end_to_end
```

**After the fix — green** (`🗑️generated/eval-native-green.txt`), 5 ticks, chain converges:

```
[DEBUG] flowEvalTick invoke extension=flow-extension-math operatorId=math.vector nodeHash=17647890481751977919
[DEBUG] extension invocation minted instance=1 extension=flow-extension-math capability=evaluate responseAction=flowEvalResolve req=Some(2)
[DEBUG] extension runner received extension=flow-extension-math capability=evaluate ok=true
[DEBUG] flowEvalResolve nodeHash=17647890481751977919 outputBytes=… seeded=true
…
[DEBUG] flowTessellate invoke extension=flow-extension-brep nodeHash=4862305879059431961
[DEBUG] drain tick 5 dispatch=650ms settle=17µs effects=0 answered=0
[DEBUG] hex column round trip finished: status={"height":{"status":"ok"},"radius":{"status":"ok"},"sides":{"status":"ok"},"profile":{"status":"ok"},"extrusion-axis":{"status":"ok"},"extrude":{"status":"ok"},"column-preview":{"status":"ok"}} meshes=3
test … hex_column_evaluates_end_to_end_through_the_extension_round_trip ... ok
test result: ok. 1 passed; 0 failed
```

**With the defect deliberately reinstated** (`invocation_address = manifest.id.clone()`, one line,
reverted immediately after) — the served symptom, exactly
(`🗑️generated/eval-native-repro.txt`, killed at tick 50 of a 1000-tick budget):

```
[DEBUG] flowEvalTick invoke extension=math operatorId=math.vector nodeHash=17647890481751977919
[DEBUG] extension invocation minted instance=1 extension=math capability=evaluate responseAction=flowEvalResolve req=Some(48)
[DEBUG] extension runner received extension=math capability=evaluate ok=false
[DEBUG] flowEvalResolve nodeHash=17647890481751977919 outputBytes=0 seeded=false
[DEBUG] drain tick 48 … effects=1 answered=1
…same node, forever…
```

`math.vector` is the `extrusion-axis` widget — the precise node boot #3 found stuck on `computing`,
with `extrude` behind it on `queued` and an empty mesh table. The reproduction is exact.

Note the failure mode is a **live spin**, not a dead stall: the SDK does answer, with a fault; the
app's lenient `flowEvalResolve` arg decode (`u64_arg`/`str_arg` … `unwrap_or_default`) turns the
faulted outcome into `outputJson = ""`, `seed_node_cache` refuses it, and the tick re-arms. From the
outside — statusJson and `data-meshes-json` — that is indistinguishable from a stall, which is why
boot #3 read as one.

### 2.2 The two contract tests (language-agnostic pair)

Same defect, pinned once on each side of the wire, both against the SAME committed fixtures:

| test | file | pins |
|---|---|---|
| `contributed_operators_are_addressed_by_their_contributing_plugin_id` | `🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` (fixture `📔️registry/🧫️fixtures/🔣️.json`, whose `pluginId: "flow-extension-owned"` ≠ `manifest.id: "owned"`) | the GUEST half: a contributed operator's `EvalError::PendingExtension.extension_id` is the plugin id, and `flow_extension_plugin_id("owned") == Some("flow-extension-owned")` |
| `resolves the extension actor by plugin id and faults a flow-domain address` | `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (fixture `🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json`, new field `"foreignAddress": "text"`) | the HOST half: `dispatchInvokeExtensionEffect` runs the extension for `flow-extension-text` and answers `extension.missing` for `text` — even when the entry declares `topicContributions[].payload.extensionId === "text"` |

TS result (`🗑️generated/eval-ts-engine-contract.txt`): `Test Files 1 passed`, `Tests 22 passed`.

---

## 3. Instrumentation — the exact console markers to look for (task 2)

Every line is `[DEBUG] `-prefixed and removable by that prefix alone. Hops in wire order:

| # | hop | where | marker |
|---|---|---|---|
| 1 | app declares the evaluate invocation | `…/🎮️commands/⏱️flow-eval-tick/🦀️.rs:31` (Rust `eprintln!`) | `[DEBUG] flowEvalTick invoke extension=<id> operatorId=<op> nodeHash=<h>` |
| 1b | app declares the tessellate invocation | `…/✏️editor/🦀️.rs:2232` | `[DEBUG] flowTessellate invoke extension=<id> nodeHash=<h>` — and, if no plugin contributes `brep`, `[DEBUG] flowTessellate skipped: no plugin contributes flow extension 'brep'` |
| 2 | SDK mints `req` + parks the continuation | `🔌️plugin/⚛️reactor/🦀️.rs:716` | `[DEBUG] extension invocation minted instance=<i> extension=<id> capability=<c> responseAction=<a> req=<n>` |
| 3 | host resolves the extension actor | `🏛️ShellHost/🟦️.tsx:1616` (`console.debug`) | `[DEBUG] invokeExtension resolve {extensionId, capability, req, resolved, loaded:[…]}` — **`resolved: null` is the defect's signature**, and `loaded` prints every id that WAS available |
| 3b | host ran the capability | `🏛️ShellHost/🟦️.tsx` (pre-existing) | `[DEBUG] extension invocation completed {extensionId, capability, instanceId, req, status}` |
| 3c | host dispatch threw | `🏛️ShellHost/🟦️.tsx` (pre-existing) | `[DEBUG] invokeExtension dispatch failed {extensionId, capability, req, error}` |
| 4 | `Event::Completed` submitted to the guest turn | `🔌️PluginRuntime/🟦️.tsx:2067` (`console.debug`) | `[DEBUG] extension completion submitted {instanceId, req, status}` |
| 5 | reactor receives `Event::Completed` | `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:286` | `[DEBUG] Event::Completed req=<n> ok=<bool>` |
| 6 | registry resolve hit / dropped / miss | same, `:290 / :306 / :308` | `[DEBUG] continuation resolve hit req=<n> instance=<i> action=<a>` · `… dropped req=<n> — owning instance has no acknowledged live lifetime` · `… miss req=<n> — routed to the parked-future registry` |
| 7 | app takes the result | `…/🎮️commands/✅️flow-eval-resolve/🦀️.rs:20` | `[DEBUG] flowEvalResolve nodeHash=<h> outputBytes=<n> seeded=<bool>` |

**What a healthy boot now looks like in the console**, per geometry node:
`flowEvalTick invoke extension=flow-extension-math` → `extension invocation minted … req=N` →
`invokeExtension resolve … resolved:"flow-extension-math"` → `extension invocation completed …
status:"ok"` → `extension completion submitted … status:"ok"` → `Event::Completed req=N ok=true` →
`continuation resolve hit req=N action=flowEvalResolve` → `flowEvalResolve nodeHash=… seeded=true`.
Then the tessellate pair for each preview handle, ending with the flow window's `statusJson` all
`ok` and a non-empty `data-meshes-json`.

**What a regression looks like:** `resolved: null` (address wrong, or the extension plugin is not in
the session's closure), or `seeded=false` repeating on ONE `nodeHash` (the spin above), or
`continuation resolve miss` (a hand-minted `req` that owns no registry slot).

---

## 4. Gates

| gate | command | result |
|---|---|---|
| generation3d e2e round trip | `cargo test … --lib -- hex_column_evaluates_end_to_end` | **1 passed** (§2.1) |
| generation3d preview subset | `cargo test … --lib -- modes::edit::windows::preview` | **5 passed / 0 failed** (`🗑️generated/eval-preview-subset.txt`) |
| flow registry addressing law | `cargo test -p semio-framework-os-flow --lib -- contributed_operators_are_addressed` (private `$S/target-eval`) | **1 passed** (`🗑️generated/eval-registry-law.txt`) |
| renderer engine contract (TS) | `SEMIO_TEST_LEVEL=exhaustive bunx vitest run 🔬️engine-contract -t "extension invocation completion ownership"` | **22 passed** (`🗑️generated/eval-ts-engine-contract.txt`) |
| procedural native | `cargo check -p semio-s-plugin-procedural --keep-going` (private `$S/target-eval`) | **Finished, 0 errors** (`🗑️generated/eval-check-native.txt`) |
| procedural wasm | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | **Finished, 0 errors** (`🗑️generated/eval-check-wasm.txt`) |

⚠️ **Deviation from the brief, recorded:** the wasm check ran against the SHARED
`target/wasm32-wasip2` rather than a private clone. The scratchpad's warm wasm target
(`$S/target-suite-wasm`) had been swept by a peer, the data volume is at **100 % (4.2 GiB free)**, and
a cold 14 GiB wasm target could not be created. No other cargo held that lock at the time (`lsof`
checked), and the restage below writes the same directory anyway.

### 4.1 Full lib suite — NOT a clean gate, and not this lane's

`cargo test … --lib -- --test-threads=4` over all 312 tests still **aborts** (SIGABRT) on a
pool-worker double panic — `final Dictionary ownership must be explicitly retired or owned by a cold
boundary` → `FlowSnapshotRetirement must reach terminal-empty before release` — reached through
`add_generation::{add_generation_records_an_undoable_generation_operation,
select_generation_does_not_mutate_the_document}` and
`remove_widget::patch_flow_widgets_recomputes_preview_geometry`. Those are the 3d-suite lane's
documented **class A** (`📓️unit-suite-3d-2026-09-09.md` §0/§1, "flow diff/projection retirement", still
red at 02:37) — a `MutationDiff<FlowFixture>` that inherits the plain-drop retirement defaults, not
this lane's addressing change. Left to that lane; flagged here because the fixture switch in §2.0
makes those tests execute more flow replay per run and therefore hit it sooner.

---

## 5. Restage proof (task 2)

```
CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000 \
SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react \
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
```

Started 03:46:14, finished 04:04:39 (18m 25s) — `NX Successfully ran target
activate-generation3d-react-dev for project @semio-tech/framework-os-dev and 38 tasks it depends on`
(`🗑️generated/eval-activate.txt`). Ran against the shared default target; no other cargo held the
wasm target dir.

**Staged procedural component**

| | |
|---|---|
| path | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm` |
| mtime | **2026-09-10 04:02** (was 2026-09-10 02:41) |
| size | 80 590 821 B |
| sha256 | `aab242190c478eb46518aef0c133c14d5b9fc736bfe073803aa99dbcf14f6f1c` |

**Activation receipt** `…/dist/runtime/dev/generation3d/activation/🔣️receipt.json`, rewritten 04:02,
11 plugins — `procedural` packageHash `86d71c265566…`, `flow-extension-brep` `2fd814ccf6e9…`,
`flow-extension-math` `2c30eb78f7ff…` (all three changed from the 02:41 receipt). All nine extension
component wasms restaged 03:58–04:00.

**Instrumentation present in the staged wasm** (`strings … | grep -c`):

| marker | count |
|---|---|
| `flowEvalResolve` | 4 |
| `flowTessellateResolve` | 2 |
| `flowEvalTick invoke` | 1 |
| `flowTessellate invoke` | 1 |
| `extension invocation minted` | 1 |
| `Event::Completed req` | 1 |
| `continuation resolve hit` | 1 |
| `flowEvalResolve nodeHash` | 1 |

⚠️ The two TypeScript hops (`ShellHost` hop 3, `PluginRuntime` hop 4) are served by vite from source
with HMR off — they reach the browser only after the **serve is restarted**, which this lane did not
do (the coordinator owns the serve on 6018). The Rust hops are already in the staged wasm above.

---

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` | **root fix** — `ContributedExtensionStub.extension_id` → `invocation_address`, fed from `plugin_id` instead of `manifest.id`; the `let _ = plugin_id;` dead-drop removed; docstring states the addressing contract |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` | new law `contributed_operators_are_addressed_by_their_contributing_plugin_id` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` | `[DEBUG]` hop 2 (minted `req`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `[DEBUG]` hops 5/6 (`Event::Completed`, continuation resolve hit/dropped/miss) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | dead `manifest.contributions` fallback removed; addressing contract documented; `[DEBUG]` hop 3 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json` | `+ "foreignAddress": "text"` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | new host-half law test |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `[DEBUG]` hop 4 (completion submitted) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` | `GENERATION_3D_GEOMETRY_EXTENSION_ID` + `preview_tessellate_invocations` resolves the brep plugin id; `[DEBUG]` hop 1b |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | `[DEBUG]` hop 1 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` | `seed_node_cache` result observed; `[DEBUG]` hop 7 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | new e2e test `hex_column_evaluates_end_to_end_through_the_extension_round_trip` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` | linked install → **contributed** install under the two real plugin ids (the browser's own wiring) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs` | host half resolves by plugin id; unresolvable address → `extension.missing`; `[DEBUG]` runner line |
