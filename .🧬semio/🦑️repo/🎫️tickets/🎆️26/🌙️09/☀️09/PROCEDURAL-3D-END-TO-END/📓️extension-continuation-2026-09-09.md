# Extension Continuation — How an `Effect::InvokeExtension` Result Reaches the App (and why generation3d never sees one)

Ticket 2026/09/09/PROCEDURAL-3D-END-TO-END, W1 lane "extension continuation wiring". Session ⚪9f5f6952 (Fable 5.1).
Repo MCP was down for the whole session; ticket bookkeeping is on disk.

---

## 1. The mechanism, established (not assumed)

**There is exactly one delivery path, and it is a parked future in the guest — never an action redispatch.**

### 1a. Guest emits

`Effect::InvokeExtension { req, extension_id, capability, request_json }` — `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:531-536`.
Its own doc (`:528-529`) states the contract verbatim:

> "Asks the shell to invoke an extension capability — the SDK resumes the awaiting future on `Event::Completed { req, .. }` instead of a `response_action` redispatch."

and `:342-343`: "`InvokeExtension` loses `response_action` and gains `req` (the SDK resumes the awaiting future instead of a redispatch)."

The `req` is meant to be minted by the guest request registry, not written by hand. The ONLY intended producer is
`Host::invoke_extension` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:553-558`:

```rust
HostBackend::Poll(registry) => registry.request(move |req| Effect::InvokeExtension { req, extension_id, capability, request_json }).await,
```

`RequestRegistry::request` (`⚛️reactor/📮️requests/🦀️.rs:194-208`) allocates the id, inserts an admitted slot, queues the
effect, and returns a `RequestFuture` parked on that exact id.

### 1b. Shell runs the extension and answers

React/wgpu shell: `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4658-4672` → `dispatchInvokeExtensionEffect`
(`:1559-1574`) → `runCapturedExtensionEffect` (`:1517-1552`) calls the extension module's `invoke(capability, requestJson)`
and then `completion.complete(outcome)`.

`captureExtensionCompletion` is implemented at `🧱️elements/🔌️PluginRuntime/🟦️.tsx:1838-1874`; its `complete` submits
**one shard event** (`:1853`):

```js
submitTurn(actorId, [{ kind: "completed", payload: { req, outcome: {tag:"ok"|"fault", val} } }], { activation })
```

Native/wasmtime host: identical shape — `🔌️plugin/🖥️host/⚡️effects/🦀️.rs:1022-1025` dispatches
`RouterEffect::InvokeExtension`, and the answer is emitted as `Event::Completed { req, result }` at `:366` / `:370`.
The async-import variant awaits inline instead: `🖥️host/📥️imports/🦀️.rs:858-868` → `run_router_effect` (`:602-616`).

### 1c. Guest routes the answer

`⚛️reactor/🔄️turn/🦀️.rs:274-276` — the whole `Event::Completed` arm:

```rust
Event::Completed { req, result } => {
    REGISTRY.with(|registry| registry.resolve(req, crate::host::outcome_to_result(result)));
}
```

`RequestRegistry::resolve` (`📮️requests/🦀️.rs:206-214`) wakes the parked `RequestFuture` for that id. **A `resolve` on an
id with no slot is a documented harmless no-op** (`📮️requests/🦀️.rs:238-241`: "same as `resolve` on an unknown id").

There is **no** `on_extension_result` export, **no** capability→action mapping, **no** `response_action` field anywhere in
the tree (`grep response_action 🧰️framework/…/🔌️plugin/**` → 0 hits), and nothing feeds a `FlowEvalSession` host-side.
Contrast the two sibling shell-driven effects that DO carry continuations and are dispatched by the shell as actions:
`Effect::RequestFileOpen{ …, import_action }` (ShellHost `:4515-4524` → `dispatchOpenedFiles`) and
`Effect::RequestMediaFrames{ …, frame_action, done_action, fallback_action }` (ShellHost `:4636-4656`).
`InvokeExtension` is the one member of that class whose continuation was removed in favour of the await path.

**Three lines:**
1. The guest must allocate `req` from its own `RequestRegistry` (`Host::invoke_extension`) and `.await` the returned future.
2. The shell/host runs the extension and answers with a single `Event::Completed { req, outcome }`.
3. The guest reactor resolves that id in the registry — and does nothing else. Unknown ids are silently dropped.

---

## 2. Why generation3d's 3d preview can never appear (two independent defects)

### Defect A — hand-minted `RequestId`s are not registry slots, so every result is discarded

| producer | file:line | `req` |
|---|---|---|
| generation3d eval | `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:26` | `RequestId(104)` |
| generation3d tessellate | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs:1851` | `RequestId(105)` |
| generation2d eval | `✏️s/…/🌀️generation2d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:26` | `RequestId(101)` |
| flow artifact eval | `✏️s/…/🌊️flow/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:59` | `RequestId(106)` |

**Repo-wide, no plugin calls `Host::invoke_extension`** (verified: the only `invoke_extension(` call sites in `✏️s` +
`🧰️framework` are the four framework definitions/host impls listed in §1). Every one of the four producers above writes a
literal id into a synchronous command handler's `Emit.effects`. None of those ids exists in the `RequestRegistry`, so
`resolve` at `🔄️turn/🦀️.rs:275` is a no-op and the brep `evaluate`/`tessellate` output is dropped on the floor.

Consequence: `flowEvalResolve` / `flowTessellateResolve` are **dispatched by nobody**. A repo-wide search for
`flowEvalResolve|flowTessellateResolve|flow-eval-resolve|flow-tessellate-resolve` across `✏️s`, `🧰️framework`, `🌎️hub`
(`.rs`/`.ts`/`.tsx`/`.json`/`.semio`/`.md`) returns only: the two flow/generation3d command implementations, flow's own
manifest wiring, flow's `🧫️fixtures/🎬️action-cohort/🔣️.json` census, and four `🔣️taxonomy.json` member names. Zero
producers.

### Defect B — generation3d retains no evaluation session, so a continuation would land on a fresh one

`Generation3dPlayApp` is a true unit struct (`✏️editor/🦀️.rs:100-101`) and **every** entry point builds a throwaway
session: `handle` `:1106`, `pending_effects` `:1160`, `render_with_request_context` `:154`, the retained work `:287`,
`:361`. `FlowEvalSession::new()` (`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs:2397-2416`) allocates a **fresh private**
`NeuralCache` and empty `pending_tessellate_by_hash`.

So `preview_tessellate_effects`'s `session.note_pending_tessellate(node_hash, handle)` (`✏️editor/🦀️.rs:1846`) records
the in-flight handle into a session that dies at the end of that same dispatch, and a later
`FlowTessellateResolve::handle` → `session.resolve_preview_tessellate` would find `pending_tessellate_by_hash` empty and
return `false` (`🖥️host/🦀️.rs:2542-2551`). Likewise `seed_node_cache` (`:2510-2513`) would seed a cache nobody reads.

The framework already provides the fix shape and flow uses it: `ArtifactInstanceOperationOwner` —
`FlowInstanceOperationOwner { eval_session: Option<FlowEvalSession> }` with `with_session`,
`maintenance_step`/`close_step` (`✏️s/…/🌊️flow/…/✏️editor/🦀️.rs:1831-1878`). generation3d takes the owner handle and
ignores it (`✏️editor/🦀️.rs:1179`, `_owner: &ArtifactInstanceOperationOwnerHandle`).

**Related hazard, same root:** `FlowEvalSession`'s `Drop` rejects a live drop by design —
`🖥️host/🦀️.rs:2384-2394` plus the framework's own oracle
`live_session_drop_is_rejected_without_recursive_payload_destruction`
(`🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs:58-60`), and `terminal_is_empty()` requires `closing == true`
(`:2641-2654`). Any `FlowEvalSession::new()` that is dropped without `begin_close()` + a granted `close_step` loop
panics.

---

## 3. What this lane changed

Scope decision: the plugin-side continuation surface is wired exactly as the flow artifact (the repo's only reference
implementation of this contract) declares it, so the two continuations are first-class, dispatchable, retained-owned
routes. The framework-side delivery gap (Defect A) is **specified below but deliberately not landed** — see §5.

All changes in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`:

| # | file:line (post-edit) | change |
|---|---|---|
| 1 | `✏️editor/🦀️.rs:9` | payload modules `flow_eval_resolve`, `flow_tessellate_resolve` imported |
| 2 | `✏️editor/🦀️.rs:90-92` | `app_commands!` rows `"flowEvalResolve" as "flow-eval-resolve"`, `"flowTessellateResolve" as "flow-tessellate-resolve"` (27 → 29 rows) |
| 3 | `✏️editor/🦀️.rs:201-203` | `GENERATION3D_RETAINED_TOOL_IDS` += both ids |
| 4 | `✏️editor/🦀️.rs:455-456` | `PUBLICATION_CONTRACTS` += both, lane `HostOnly` **alone** (registry law `HostOnly` must be the sole lane — `🔌️plugin/🦀️.rs:12746-12749`) |
| 5 | `✏️editor/🦀️.rs:938-939` | `bounded_first_step_tool_proofs!` += both (`bounded_first_step(8_192, 32, 32, 16_384, 7_500)`, identical to the other 27) |
| 6 | `✏️editor/🦀️.rs:1014` | new `u64_arg` closure in `command_from_action` |
| 7 | `✏️editor/🦀️.rs:1095-1102` | `command_from_action` arms for both ids (`nodeHash`/`node_hash`, `outputJson`/`output_json`) |
| 8 | `✏️editor/🦀️.rs:1260-1261` | `CommandDefinition::bounded_catalog(..)` `in_palette: false` + `migrated_command(..)` for both, EN/DE labels |
| 9 | `✏️editor/🦀️.rs:1336-1337` | `.action_interactive_job(.., InteractiveJobClassification::Migrated)` for both |
| 10 | `✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` | `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` |
| 11 | `✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs` | same |
| 12 | `✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs` | NEW — `eval_result_seeds_the_node_cache_and_rearms_the_tick_chain` |
| 13 | `✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🧪️tests/🔬️unit/🦀️.rs` | NEW — `tessellate_result_resolves_the_pending_handle`, `unknown_node_hash_resolves_nothing` |
| 14 | `✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | NEW helpers `retire_flow_eval_session` (the `begin_close` + granted `close_step` loop the `Drop` contract demands) and `empty_history_view` (`HistoryView` derives no `Default`) |
| 15 | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | counts 27 → 29 in `command_ids_are_unique_and_cover_every_row` and `retained_route_dispositions_are_exact_and_exhaustive`; `expected_keywords` and `every_command()` extended in declaration order |

No `Cargo.toml` change was needed: both tests are `#[path]`-mounted `#[cfg(test)] mod tests` inside the lib, not
`tests/` targets, so the emoji-filename `[[test]]` rule does not apply. (A concurrent session added an unrelated
`[[test]] example-geometry` target to the same manifest mid-session; left untouched.)

No `generation2d` change: it has no resolve command at all. It inherits the framework-side fix (§5) unchanged, and
its own `flowEvalTick` is still classified `BatchOnlyPendingRewrite` (`…/🌀️generation2d/…/✏️editor/🦀️.rs:973`), i.e.
hard-dead at dispatch regardless.

---

## 4. Build / test evidence — NOT GREEN, and honestly so

**The two new tests were written but never executed.** Not because they failed — because
`semio-s-artifact-procedural-generation3d` could not be compiled at any point in this session's
window: a concurrent session is mid-way through a repo-wide store/child-composition refactor and the
shared framework crates on generation3d's own dependency path were broken continuously.

Lane: `CARGO_TARGET_DIR=$S/target-cont RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 cargo test -p
semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- flow_eval_resolve
flow_tessellate_resolve retained_route_dispositions command_ids_are_unique every_printed_op_line
every_command_round_trips`, driven by a retry loop (`$S/retry-test.sh`, 2-minute backoff).

`$S/cont-test-driver.log`, verbatim outcome per attempt:

```
=== attempt 1 15:30:24 === error: could not compile `semio-s-artifact-stdio-semio` (lib) due to 11 previous errors
=== attempt 2 15:37:10 === error: could not compile `semio-framework-plugin` (lib) due to 1 previous error
=== attempt 3 15:39:54 === error: could not compile `semio-framework-plugin` (lib) due to 1 previous error
=== attempt 4 15:42:14 === error: could not compile `semio-framework-os-flow` (lib) due to 2 previous errors
=== attempt 5 15:49:28 === error: could not compile `semio-framework-plugin` (lib) due to 9 previous errors
=== attempt 6 15:53:00 === error: could not compile `semio-framework-plugin` (lib) due to 9 previous errors
=== attempt 7 15:55:25 === error: could not compile `semio-framework-os-flow` (lib) due to 2 previous errors
=== attempt 8 16:04:53 === error: could not compile `semio-framework-os-flow` (lib) due to 2 previous errors
```

Earlier attempts in the same window hit two further, already-repaired peer states: a duplicate
`use semio_framework_value_derive::{FromValue, ToValue};` in the brand-new
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs` (created 15:11, fixed
15:12), and a missing `👁️viewer/🎚️config/🧪️tests/🔬️unit/🦀️.rs` inside generation3d itself that a peer
declared as `mod tests;` before writing the file (dir created 15:11, file landed 15:11).

The settled blocker, unchanged across attempts 4/7/8 (`$S/cont-test.log` tail):

```
error[E0599]: the method `close_owned_step` exists for mutable reference
  `&mut ArtifactStore<semio_framework_artifact_flow_flow::FlowFixture, FlowMutation>`,
  but its trait bounds were not satisfied
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2248:25
 260 | pub struct FlowFixture {
     | ---------------------- doesn't satisfy `_: ArtifactCompositionFields`
     = note: `FlowFixture: ArtifactCompositionFields` ... required by
             `ArtifactStore<FlowFixture, FlowMutation>: SpaceMember`
error: could not compile `semio-framework-os-flow` (lib) due to 2 previous errors
```

`semio-framework-os-flow` is where `FlowEvalSession` lives, i.e. a hard, direct dependency of the
generation3d artifact crate. Nothing in this lane can compile until the peer lands
`impl ArtifactCompositionFields for FlowFixture`.

**Therefore, per CLAUDE.md, no claim is made here that anything passes.** What IS established:

- the warming build at 14:50-15:11 (`$S/cont-warm.log`) compiled the entire dependency graph with
  zero errors up to `semio-s-artifact-procedural-generation3d` itself, and stopped only on the peer's
  missing viewer-config test file (`error: couldn't read …/👁️viewer/🎚️config/🧪️tests/🔬️unit/🦀️.rs`,
  `$S/cont-warm.log:301-307`) — a file unrelated to and untouched by this lane;
- no attempt ever produced `could not compile \`semio-s-artifact-procedural-generation3d\``, i.e. the
  edits in §3 were never themselves reached and rejected;
- the wiring was verified structurally instead, by reading every one of the 9 edited sites back out of
  the file (§3 line numbers are post-edit and were re-derived from disk).

### 4a. Attempt 9 (16:18) — the crate WAS reached, and the 16 errors are all peer-owned

Attempt 9 finally got a clean `semio-framework-os-flow` and compiled through to
`semio-s-artifact-procedural-generation3d` itself:

```
error: could not compile `semio-s-artifact-procedural-generation3d` (lib test) due to 16 previous errors; 10 warnings emitted
```

**10 warnings were emitted, so the crate genuinely reached and completed type-checking** — this is
not an early module-expansion abort. Every one of the 16 errors was attributed by reading its
`-->` source span; **none is in a file this lane touched**:

| error | span | owner |
|---|---|---|
| `couldn't read …/👁️preview/🫧️transient/🧪️tests/🔬️unit/../../../../🧫️fixtures/🔬️unit/🔣️.json` | preview transient test | peer, fixture not yet written |
| `MutationLeaf source authority failed: textOpcode must be lowercase kebab-case or null` | `👁️viewer/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` | peer, new `🌞️set-sun` mutation |
| `E0422 cannot find … Generation3dViewCamera` | `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs:99` | peer |
| `E0277 set_sun::SetSun: MutationLeaf` ×9 | `👁️viewer/🎚️config/🧬️schema/🧬️mutations/🌞️set-sun/🦀️.rs` + retained-authority-laws | peer |
| `E0277 Box<VcsArtifactApp<…>>: PluginApp` | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:27` (`testkit::close_registered_fixture_app`) | peer framework API change |
| `E0599 begin_close` on `Box<dyn RetirementCursor>`, `E0061`, `E0599 expect` on `RetirementStep` | `👁️viewer/🎮️commands/📷️set-camera/🧪️tests/🔬️unit`, `👁️viewer/🧪️tests/🔬️unit` | peer, new `♻️retirement` API |

The only diagnostics anywhere in this lane's own files were four `unnecessary qualification`
warnings in `✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — three pre-existing (lines 37/38/44), and one
introduced here at line 79 (`semio_framework_os_flow::FlowEvalSession`), which has been fixed to the
bare `FlowEvalSession` already in scope. So the §3 wiring and both new test modules parsed, resolved
and type-checked; the two tests still never got to *run*, because the lib-test binary could not link
while the peer's viewer/mutation/retirement work is mid-flight.

**Owed before this lane can be called done:** re-run the command above once
`semio-framework-os-flow` builds again; then `cargo check -p semio-s-plugin-procedural --keep-going`
natively and `--target wasm32-wasip2 --profile wasm-dev` (profile confirmed present in the root
`Cargo.toml`: `[profile.wasm-dev]`, with per-package overrides for `semio-s-artifact-lowpoly-lowpoly`
and `semio-s-artifact-puzzle-3d`), reporting warning counts as proof the crate really type-checked.
Neither check was run here: both sit behind the same broken `semio-framework-os-flow`, and the wasm
lane would additionally contend with the peer session that owns the react boot lane.

## 5. The framework-side fix that is still owed

Not landed in this session — recorded here with the exact shape so the next lane can execute it.

**Smallest correct change, and it needs NO wire/WIT/ABI change:** the `req` on `Effect::InvokeExtension` is just a `u64`
on the wire, identical whoever mints it. The defect is purely that the guest mints it outside the registry. The fix is
therefore entirely guest-side, in `semio-framework-plugin`:

1. Give `Emit` a first-class extension-invocation slot carrying `{ extension_id, capability, request_json,
   response_action }` instead of letting an app push a raw `Effect::InvokeExtension` with a literal id.
2. When the SDK drains `Emit` into the turn result, allocate the `req` through `RequestRegistry::request`
   (`⚛️reactor/📮️requests/🦀️.rs:194`) and park a guest task that awaits the returned `RequestFuture` and then dispatches
   `response_action` into the same app instance with `{ nodeHash, outputJson }`.

Everything downstream (WIT `invoke-extension-params`, `🖼️wire-turn.ts`, `PluginRuntime.wireEffectToFriendly`, ShellHost,
the wasmtime host, every checked-in jco `.d.ts` and every plugin's built wasm) stays byte-identical, because only the
*origin* of `req` changes.

Rejected alternative: re-adding a `response_action` field to `Effect::InvokeExtension` (mirroring `RequestFileOpen`'s
`import_action`). It is the idiom the kernel keeps for its two sibling shell-driven effects, but it is a component-type
change: WIT `effects.wit`, the Rust host + guest bindings, `🖼️wire-turn.ts`, ShellHost, and a full rebuild of every
plugin wasm and every checked-in jco `.d.ts`. Not the smallest correct change, and not safely landable while a peer
session owns the wasm boot lane.

Defect B must be fixed alongside it: generation3d needs an `ArtifactInstanceOperationOwner` holding one
`FlowEvalSession` per app instance, copied from `FlowInstanceOperationOwner`
(`✏️s/…/🌊️flow/…/✏️editor/🦀️.rs:1831-1878`), and every `FlowEvalSession::new()` in `✏️editor/🦀️.rs` (`:154`, `:287`,
`:361`, `:1106`, `:1160`) replaced by a borrow of that retained session. Without it the continuation lands on a fresh
session and resolves nothing.

## 6. Descriptor regeneration — owed, not run

`🛂️.descriptor.semio` + `🔣️.json` are emitted by `bun nx run @semio-tech/os-plugin-describe-rs:describe -- <plugin>`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📋️project.json`, target `describe`,
`dependsOn: ["build"]`; the pack filename constant is at that dir's `📜️script.ts:35`). Because the target's `describe`
runs the real `describe()` against a freshly built plugin **component wasm**, it is a full wasm build of
`semio-s-plugin-procedural`. That was not run here: the box is at load ~85 with swap exhausted and a peer session owns
the shared wasm target dir for the react boot lane. `✏️s/🔌️plugins/🌀️procedural/🛂️.descriptor.semio` and
`✏️s/🔌️plugins/🌀️procedural/🔣️.json` therefore still describe 27 generation3d actions and MUST be regenerated before
the plugin registry / native `run` host is trusted (`🏃️run/🦀️.rs:1687` faults on a stale/missing descriptor).
