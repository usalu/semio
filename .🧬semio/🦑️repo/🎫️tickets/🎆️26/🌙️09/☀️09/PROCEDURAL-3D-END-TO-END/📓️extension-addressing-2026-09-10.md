# Extension Addressing — One Translation Surface, A Typed Preview Fault, And Why The Browser Still Has No Mesh

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "flow extension addressing". Session ⚪9f5f6952
(Fable 5.1, Opus 5). Repo MCP was down all session (`invalid initialize params`); ticket bookkeeping
is on disk and no ticket was opened, closed or reopened.

Answers `📓️runtime-verification-2026-09-09.md` boot #7 (06:35, on the 06:19 restage): the tick chain
runs, but the console repeats `flowTessellate skipped: no plugin contributes flow extension 'brep'`
46× per boot, the flow status stays `extrusion-axis: computing / profile, extrude: queued`, the
preview status reads `idle, evalLen 0`, and there is no mesh.

---

## 1. TL;DR

**The dispatched hypothesis was wrong, and the truth is one level lower down.**

The brief (and boot #7's own reading of the console line) assumed *"the producers still emit the
manifest id `brep` while the registry addresses extensions by owning plugin id"*. They do not.
`preview_tessellate_invocations` has resolved through `flow_extension_plugin_id` since yesterday's
eval lane, and `⏱️flow-eval-tick`'s `pending.extension_id` is the stub's `invocation_address`, which
`register_contributed_manifest` already writes as the OWNING PLUGIN id. **Both producers were already
correct.** `brep` appears in that console line because it is the *lookup key that failed*, not the
address that was emitted.

The lookup failed because **the served procedural plugin's flow extension registry is completely
empty**:

> `sync_host_flow_extension_contributions` — the one function that installs contributed
> `flow.extension` manifests into a plugin's flow registry — **has no production caller anywhere in
> the repository.** Its only callers are tests. `register_linked_flow_extension_installer` likewise
> has no production caller. So in the browser the procedural plugin has *no operators at all*: no
> `math.vector`, no `brep.extrude`, nothing.

That single fact explains every symptom of boot #7 at once, and explains them better than the
addressing hypothesis did:

| boot #7 observation | empty-registry explanation |
|---|---|
| `flowTessellate skipped: … 'brep'` **46×** | `flow_extension_plugin_id("brep")` → `None`, every tick |
| flow status `extrusion-axis: computing`, `extrude: queued` | no operator exists for those nodes, so no node ever resolves |
| preview `idle, evalLen 0`, `data-meshes-json="[]"` | the eval session never produced a single output |
| the tick chain nevertheless cycles healthily | `session.tick` settles instantly with `more == false`, which is exactly the branch that reaches the skip line |

The addressing hypothesis predicts a *spin on one node with a fault answer* (that is what yesterday's
lane reproduced natively: `outputBytes=0 seeded=false` repeating). Boot #7 shows the opposite —
`evalLen 0`, nothing evaluated at all. The empty registry is the only reading consistent with both.

**Delivered this lane** (§2–§5): one translation surface with a typed miss, the miss projected into
the preview window as `phase: "faulted"` with English/German labels instead of a per-tick
`eprintln!`, two new law tests, and the shell's per-action `[DEBUG]` chatter gated behind the same
runtime-diagnostics switch the hot-path lane introduced.

**NOT delivered, and out of this lane's reach** (§6): actually delivering the contributions. The only
host→guest channel for them (`setContributions`) is capped at **8 192 bytes** by every app that owns
it, the view-context lane at **65 536 characters**, and the payload the shell builds for the
generation3d closure is **293 642 characters**. That is a framework-capacity work package, sized and
designed in §6, not a producer bug.

---

## 2. The one translation surface (task 2, first half)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs`

`flow_extension_plugin_id(&str) -> Option<String>` is **gone** — an `Option` lets a caller drop the
reason on the floor, which is precisely how a missing contribution became 46 console lines nobody
could act on. It is replaced by the typed pair:

```rust
pub struct FlowExtensionAddressMiss { pub extension_id: String, pub contributed: Vec<(String, String)> }
impl FlowExtensionAddressMiss { pub const CODE: &str = "flow.extension-not-contributed"; pub fn labels(&self) -> (String, String); }

pub fn flow_extension_invocation_address(extension_id: &str) -> Result<String, FlowExtensionAddressMiss>;
```

The miss names **both** sides the brief asked for: the flow manifest id that was asked for, and every
`<manifest id> → <owning plugin id>` translation the live contribution table *does* carry. Its
`labels()` are English and German with no default language.

Every producer and every projection now goes through that one function:

| caller | file |
|---|---|
| the tessellate invocation producer | `…/🧊️generation3d/…/✏️editor/🦀️.rs` `preview_tessellate_invocations`, via the app-local one-liner `geometry_extension_address()` |
| the preview status projection | same file, `preview_progress_status_json` (§3) |
| the contributed operator stub (implicitly — it *is* the plugin id) | `📔️registry/🦀️.rs` `register_contributed_manifest`, unchanged |
| the registry addressing law | `📔️registry/🧪️tests/📔️registry/🦀️.rs` |

`⏱️flow-eval-tick` needs no translation at all: `EvalError::PendingExtension.extension_id` is the
stub's `invocation_address`, already the plugin id, and generation2d's twin is the same code path.
The host resolver (`dispatchInvokeExtensionEffect`, `🏛️ShellHost/🟦️.tsx`) was already correct — it
resolves `extensionId` against `handle.pluginId` and warns once when nothing answers — so it is
untouched.

---

## 3. The miss reaches the surface, not the console (task 2, second half)

`PreviewTessellatePhase` (`🌊️flow/🖥️host/🦀️.rs`) gains one boundary-only variant beside `Invalid`
and `Failed`:

| variant | tag | English | German |
|---|---|---|---|
| `Faulted` | `"faulted"` | `Geometry extension unavailable` | `Geometrie-Erweiterung nicht verfügbar` |

`preview_progress_status_json` splits into a live wrapper and a **pure** projection
`preview_progress_status_json_for(session, address)`. An unaddressable kernel outranks whatever the
tessellation ledger last recorded — nothing was ever invoked, so the ledger's `idle` is a lie the
surface cannot act on — and the status object gains a `fault` member. What the preview window now
publishes (verbatim from the new test):

```json
{"phase":"faulted",
 "phaseLabel":{"en":"Geometry extension unavailable","de":"Geometrie-Erweiterung nicht verfügbar"},
 "cancellable":false,
 "fault":{"code":"flow.extension-not-contributed",
          "extensionId":"brep",
          "message":{"en":"No loaded plugin contributes the flow extension \"brep\"",
                     "de":"Kein geladenes Plugin steuert die Flow-Erweiterung \"brep\" bei"},
          "contributed":[{"extensionId":"math","pluginId":"flow-extension-math"}]}}
```

The `eprintln!("flowTessellate skipped: …")` is deleted. Boot #8 should show **zero** occurrences of
that string and the preview window should read `Geometry extension unavailable` instead of an empty
scene with no explanation.

**Why the projection is pure.** The first draft of the law test proved the faulted branch by
`uninstall_flow_extension("brep")` → render → reinstall. That poisons the flow catalogue's
`neuron kind info cache` lock and leaves the neural registry unretired, which then aborted
`hex_column_evaluates_end_to_end_through_the_extension_round_trip` and both preview tests with
`final Dictionary ownership must be explicitly retired or owned by a cold boundary` — a
process-global mutation is not a testing tool here. Splitting the resolution out of the projection is
what makes the branch provable without touching the global table at all.

---

## 4. Tests (task 1)

### 4.1 The new addressing law — through the CONTRIBUTED install

`…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
→ `extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview`

Boots the registered app on `hexagonal-mushroom-column` with the contributed harness
(`🧪️tests/🔬️flow-operators/🦀️.rs`), drives the real tick chain, and settles every invocation through
`testkit::settle_extension_invocations` with a **recording** answerer, so the assertion is on the
address that actually crossed the wire rather than on an internal:

- every emitted address is a contributing plugin id (`flow-extension-brep` / `flow-extension-math`),
  never a flow manifest id;
- at least one is `("flow-extension-brep", "tessellate")` — the hop the brief asked to pin;
- the pure projection, handed an `Err(miss)`, publishes `phase: "faulted"`, both labels, `cancellable:
  false`, the typed `code`, the asked-for `extensionId`, both message languages, and both live
  translations in `contributed`.

Observed: `[DEBUG] invocation addresses: [("flow-extension-brep", "tessellate") ×3]`.

### 4.2 The registry law, both directions

`🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` ·
`contributed_operators_are_addressed_by_their_contributing_plugin_id` now also pins the miss: an
uncontributed id yields a `FlowExtensionAddressMiss` naming the asked-for id, carrying the live
`<manifest id, plugin id>` pair, and labelling itself in both languages.

### 4.3 Results

| gate | command | result |
|---|---|---|
| generation3d addressing + fault law | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- extension_invocations_address` | **1 passed** |
| generation3d round trip + preview subset + the new law | `… --lib -- hex_column_evaluates_end_to_end modes::edit::windows::preview extension_invocations_address` | **7 passed / 0 failed** |
| flow registry addressing law | `cargo test -p semio-framework-os-flow --lib -- contributed_operators_are_addressed` | **1 passed** |
| renderer engine contract (TS, whole file) | `SEMIO_TEST_LEVEL=exhaustive bunx vitest run 🔬️engine-contract` | **482 passed** |

**Two pre-existing `semio-framework-os-flow` registry failures, NOT this lane's** (both reproduce on
a filtered run of that crate alone, and neither touches any file this lane edited):

- `contributed_registry_replacement_preserves_readers_and_drains_old_versions` — the round-tripped
  schema now carries `"label": null` where the fixture omits it, because
  `FieldSpec::label`'s `skip_serializing_if` is written
  `#[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:375`). `cfg(test)` is
  per-crate: when the *flow* crate's test binary compiles, the neural crate is built without it, so
  the attribute can never apply on this path. Last touched at `025ec86a42`, already in `HEAD` when
  this lane started.
- `registry_maintenance_does_not_initialize_a_registry` — asserts `FLOW_EXTENSION_STATE.get().is_none()`;
  passes alone, fails whenever any other registry test in the same binary ran first. An isolation
  defect in the test, not in the registry.

---

## 5. The shell's per-action chatter is now gated (task 3)

`🏛️ShellHost/🟦️.tsx` gains a `//#region 🩺️RuntimeDiagnostics` mirroring the guest switch
`RUNTIME_DIAGNOSTICS_ENV` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`, the hot-path lane's,
`📓️hotpath-optimization-2026-09-10.md` §2) key for key:

```ts
export const RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";
export function runtimeDiagnosticsEnabled(): boolean;
export function setRuntimeDiagnostics(enabled: boolean | undefined): void;
```

Resolution order, resolved once per page: an explicit `setRuntimeDiagnostics` override, then the
build's `import.meta.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS`, then a `localStorage` key of the same name
so a live tab can be armed without a rebuild. `1`/`true`/`on`/`yes` arm it; **absent leaves it off**.
Both reads are wrapped — a sandboxed tab throws on `localStorage`, a non-Vite host has no
`import.meta.env`.

Gated (all three previously unconditional, one per dispatched action):

| site | line |
|---|---|
| `[DEBUG] applyHostEffects refresh` | `🏛️ShellHost/🟦️.tsx` `applyHostEffects` |
| `[DEBUG] applyHostEffects skipped refresh: session not current` | same |
| `[DEBUG] completion apply` | typed-operation completion pass |

`🔬️engine-contract/🟦️.ts` gains `describe("shell runtime diagnostics switch")`: the key string is
pinned (the two languages cannot share a declaration), the default is asserted **off**, and the
override is asserted to win in both directions.

**The other two lines the brief named do not exist in the tree.** `grep` for `thunk start`,
`thunk done`, `command ingress settled` and `command-ingress` across every `.ts`/`.tsx` source finds
no `console.*` site — a sibling lane removed them between boot #7 and this lane. `🔌️PluginRuntime/🟦️.tsx`
has **zero** `console.*` calls, so nothing there needed gating either.

---

## 6. Why there is still no mesh in the browser — and what the next work package is

This is the finding that matters most, and it is not fixable inside this lane.

### 6.1 Nothing installs contributions into a served plugin

```
sync_host_flow_extension_contributions   — callers: 3, all tests
register_linked_flow_extension_installer — callers: 2, both the generation3d test harness
```

The shell *does* build the payload and *does* try to push it
(`🏛️ShellHost/🟦️.tsx`, `buildContributionsJson` → the `setContributions` push loop), and the nine flow
extension plugins *are* staged and loaded for the generation3d variant (`dist/runtime/dev/generation3d/extensions/`,
`flow-extension-brep` and `flow-extension-math` among them, activation receipt lists all eleven
plugins). But the push only reaches an app that **owns the `setContributions` command**, and the
procedural plugin declares no such command — `grep -r setContributions ✏️s/🔌️plugins/🌀️procedural/`
returns nothing. `cad`, `process3d`, `forms` and `playbook` own it; `procedural` does not, and even if
it did nothing in it calls `sync_host_flow_extension_contributions`.

The plugin's own `Cargo.toml` declares `consumes = ["forms.questionKind", "flow.extension"]` and its
`🦀️.rs` declares nine `FlowExtensionDeclaration`s with `FlowExtensionExecutableIdentity::native(…)`.
Both are **metadata only**: `Plugin::flow_extensions()` returns the descriptors and nothing at
runtime consumes them. The consumption is declared and never realized.

### 6.2 …and the channel could not carry them anyway

| quantity | value | source |
|---|---|---|
| contributions payload for the generation3d closure | **293 642 characters** | sum of the eight flow extension manifests' `topicContributions`, in `buildContributionsJson`'s exact wire shape; `flow-extension-brep` alone is 190 656 |
| `setContributions` retained-command ceiling, generation3d | **8 192 bytes** | `GENERATION3D_RETAINED_RAW_BYTES`, enforced in `Generation3dBoundedCommandJobFactory::create` |
| same, `cad` / `process3d` | 8 192 | `CAD_RETAINED_RAW_BYTES`, `PROCESS3D_RETAINED_RAW_BYTES` |
| same, `forms` | 16 384 | `FORMS_RETAINED_RAW_BYTES` |
| `contributionsJson` on the view context | **65 536 characters** | `VIEW_CONTEXT_LONG_STRING_CHARS`; `parseResolvedPluginViewState` throws past it |

process3d even pins the refusal as a law:
`retained_resumable_extent_accepts_exact_byte_maximum_and_rejects_max_plus_one` asserts a
`SetContributions` of `PROCESS3D_RETAINED_RAW_BYTES + 1` is rejected. So the existing contribution
channel is **36× too small** for one flow extension manifest, in every app that has it.

### 6.3 The shape of the fix (recommended, not done here)

The framework already pages large retained payloads (`ArtifactRetainedCommandPayload`'s
`maximum_raw_bytes` accumulates across `input.page_count()` pages) and already ships a large-payload
precedent (`PUZZLE3D_MESH_COMMAND_RAW_BYTES` / `puzzle3dBrushMeshPages`). So the clean solution is a
**second tool factory** on the consuming app, not a raised global quota:

1. `Generation3dContributionsJobFactory` — its own `ToolExecutionContract` with a
   contributions-sized `maximum_raw_bytes` (512 KiB covers the whole closure with headroom) and its
   own `bounded_first_step_tool_proofs!` row, so the 8 KiB gesture quota every *interactive* command
   lives under is untouched.
2. One `setContributions` command on the generation3d and generation2d editor apps, publication lane
   `HostOnly` (it emits no store mutation), classification `Migrated`, handler body
   `semio_framework_os_flow::sync_host_flow_extension_contributions(&payload.json)` — which already
   de-duplicates on unchanged input, so the boot pays for it exactly once.
3. The registration surface it implies, per app: the `app_commands!` row, `GENERATION3D_RETAINED_TOOL_IDS`,
   `PUBLICATION_CONTRACTS`, the arg decode arm, `.command(…)` + `.action_interactive_job(…)` +
   `.action_args(…)`, the regenerated `🔣️.json`, and the bijection/exhaustiveness tests that assert
   the roster.
4. A law test that builds the contributions JSON exactly as `buildContributionsJson` does, dispatches
   it as the command, and then asserts the same chain this lane's §4.1 test asserts — but with **no**
   `install_flow_extension_manifest` and **no** linked installer anywhere, so it measures the machine
   that actually ships.

Until that lands, the served app's preview will keep saying `Geometry extension unavailable` — which
is at least now the truth, in two languages, on the surface.

---

## 7. Gates

All commands from the repo root with a private `CARGO_TARGET_DIR=$S/target-ext` (seeded from
`target/debug`, and its `wasm32-wasip2` seeded from the shared warm `target/wasm32-wasip2`, which cut
the wasm check from a cold build to **1 m 01 s**), `RUSTC_WRAPPER=""`, `RUST_MIN_STACK`, `--keep-going`.
Raw logs in `🗑️generated/ext-*.txt`.

| # | gate | result | log |
|---|---|---|---|
| 1 | `cargo check -p semio-framework-os-flow --keep-going` | **Finished, 0 errors** | `ext-1.txt` |
| 2 | `cargo check -p semio-s-plugin-procedural --keep-going` (native) | **Finished, 0 errors** | `ext-9.txt` |
| 3 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | **Finished, 0 errors** | `ext-10.txt` |
| 4 | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- hex_column_evaluates_end_to_end modes::edit::windows::preview extension_invocations_address` | **7 passed / 0 failed** | `ext-5.txt` |
| 5 | `cargo test -p semio-framework-os-flow --lib -- contributed_operators_are_addressed` | **1 passed** | `ext-6.txt` |
| 6 | `SEMIO_TEST_LEVEL=exhaustive bunx vitest run 🔬️engine-contract` | **482 passed** | `ext-8.txt` |

⏳️ Gate 2 was red for ~6 minutes on a peer's uncommitted `[DEBUG] step_reactor_close` probe in
`🔌️plugin/⚛️reactor/🦀️.rs:1001` (`RequestCloseCursor` doesn't implement `Display`) — the close-ladder
lane's, not this one's. Polled, not chased; green on the sixth attempt at 07:03:41 once they fixed it.

⚠️ Two pre-existing `semio-framework-os-flow` registry test failures are documented in §4.3. Neither is
this lane's and neither touches a file it edited.

### 7.1 Restage

`CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000 SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`

`ps`/`lsof` confirmed no other cargo held `target/wasm32-wasip2` before the run. Result:
`Activated generation3d react dev: 11 completed components (changed)`, `Successfully ran target …
and 38 tasks it depends on`, 8 m 36 s (`ext-restage.txt`). Freshness witness on the staged component
the browser loads:

| | before | after |
|---|---|---|
| `…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm` mtime | `2026-09-10 06:19:14` | **`2026-09-10 07:14:35`** |
| size | 80 652 384 | 80 684 237 |
| sha256 (first 32) | `3b8fccb9fadc3c8775073cf24fffbf2e` | **`5c04f5944185a9fc4b986720d809fd45`** |
| `dist/runtime/dev/generation3d/activation/🔣️receipt.json` | 06:19 | **07:14** |

The vite serve on 6018 was **not** restarted (`lsof -ti :6018` still holds its pids, `curl` → `200`).
A browser reload picks the new component up.

### 7.2 What boot #8 should show

- **zero** `flowTessellate skipped: no plugin contributes flow extension 'brep'` (the string no longer
  exists in any Rust source — only in a doc comment explaining why it is gone);
- the preview window reading `Geometry extension unavailable` / `Geometrie-Erweiterung nicht
  verfügbar` with `phase: "faulted"` and the `flow.extension-not-contributed` fault object naming
  `brep` and every translation the session does carry — instead of a silent empty scene;
- **no** `[DEBUG] applyHostEffects refresh` / `[DEBUG] completion apply` per action (arm them with
  `localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1")` and reload when a perf run wants them);
- still **no mesh**, for the reason in §6 — that is the next work package, not a regression.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` | `flow_extension_plugin_id` → `flow_extension_invocation_address` + `FlowExtensionAddressMiss` (typed, both ids, en/de) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` | the addressing law now pins the miss in both directions |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | `PreviewTessellatePhase::Faulted` (`"faulted"`, en/de), `tag`/`from_tag`/`labels` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `geometry_extension_address()`; the `eprintln!` deleted; `preview_progress_status_json` split into a live wrapper + the pure `preview_progress_status_json_for`, which publishes the `fault` object |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | new law `extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` | `resolve_ready` made `pub` for the law; docstring follows the rename |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `//#region 🩺️RuntimeDiagnostics` (`RUNTIME_DIAGNOSTICS_KEY`, `runtimeDiagnosticsEnabled`, `setRuntimeDiagnostics`); three per-action `[DEBUG]` lines gated |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | `describe("shell runtime diagnostics switch")` — key pinned, default off, override wins both ways |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/📓️extension-addressing-2026-09-10.md` | this report |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🗑️generated/ext-*.txt` | raw gate + restage logs |
