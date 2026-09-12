# Audit: `invokeExtension` / window-transient "rejected authority" — 2026-09-12

Read-only audit. No source edited, nothing built, nothing restaged.

## Headline finding

The exact fault string in the task brief — **`window-transient publication is retiring a
rejected authority`** — no longer exists anywhere in source
(`grep -rn "is retiring a rejected authority" 🧰️framework ✏️s` → 0 hits, all doors checked).
It was removed, along with the identical bug on six sibling publication lanes, in commit
**`de93f84300ad0b5aa360ca52efcfa290540b9952`** (author date `2026-09-12 00:35:59 +0200`,
commit-message timestamp `🎆️26🌙️09☀️11⏰️15⌚️42⏱️04` = 2026-09-11 15:42), which is the current
tip of `HEAD`. This lines up exactly with `📓️invoke-extension-rejected-authority-2026-09-11.md`'s
"Source now (15:35) … gone from `🔌️plugin/🦀️.rs`" note.

**What is unresolved:** the served wasm was staged *before* the fix in every ticket note read
(`📓️unknown-kind-after-restage-2026-09-11.md`: 94 138 842 B at 14:15). A **newer** build exists
on disk — `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_procedural.wasm`,
94 394 569 B, mtime **2026-09-11 15:41** (6 minutes after the fix commit's message timestamp,
different byte count than the stale 14:15 copy) — but no ticket file records a restage+reprobe of
6018 against it, and no status.md entry exists past `## 2026-09-11 restage wasm 14:15`. **Nobody
has yet proven live-browser recovery of this specific fault against the fixed source.**

---

## Q1 — Where does the string come from; what is a window-transient authority; who retires it; what makes it "rejected"

**Not present in current source.** Confirmed by direct grep and by diffing the fix commit
against its parent:

```
$ grep -rn "is retiring a rejected authority" 🧰️framework ✏️s   → (no output)
```

The **host-visible wrapper** `typed-operation failed: {message}` is built in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`:

- `:646` (inside `withTypedOperationCall`): `if (fault !== null) throw new Error(\`typed-operation failed: ${fault}\`);`
- `:674` (inside `consumeTypedOperationEffects`, on a `page.lane === 11` result page):
  `if (!call || call.fault(page.operation, page.sequence, message)) throw new Error(\`typed-operation failed: ${message}\`);`

`message` at `:673` is `new TextDecoder().decode(page.payload)` — raw UTF-8, not pack-decoded
(this is a *different* ABI door than the `completion-result.fault` pack door
`📓️fault-arm-symmetry-2026-09-10.md` audited; lane 11's payload is `Fault::as_bytes()` text, per
`🔌️plugin/🦀️.rs:24006`, so no mismatch to flag here).

**A "window-transient authority"** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs`)
is the guest-owned, per-window capture of `{ window_id, window_kind_id, generation, snapshot }`
(`WindowTransientAuthority`, `:174-260`) that lets one plugin instance publish ephemeral computed
state (e.g. the generation3d preview's `preview_eval_text`) into one specific open window without
racing another window of the same kind. It is:

- **owned by the guest** (`semio-framework-plugin`, Rust) — captured once per command admission at
  `🔌️plugin/🦀️.rs:24375` (`self.window_transient_store.capture(...)`), *not* by host TypeScript.
- **retired** through `PendingArtifactStorePublication::WindowTransient`'s `Closing` phase in
  `🔌️plugin/🦀️.rs` (`:23958-23969`), driven by `store::SnapshotRetirementStep`/`close_step`.

**"Rejected"** = the publication's own `fault()` is `Some` — i.e. a *previous* attempt to publish
into this window's transient store was rejected (superseded, cancelled, or based on a stale
generation) and is now draining its retirement ("closing") steps before the slot is free again.

---

## Q2 — Full trace, `Effect::InvokeExtension` → dispatch → typed operation → answer

1. **Guest emits the invocation.** `flow-eval-tick::evaluate`
   (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:70-80`)
   builds `ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve")`
   **and, independently, in the SAME `Emit`, a `window_transient` ephemeral mutation** via
   `session.eval_publication_for(retained_eval)` (`:82`) — the eval-status JSON (including any
   `flow.extension-not-contributed` fault payload) republishes into the SAME preview window's
   transient store on *every* tick, whether or not that tick also fires an extension call. This is
   the answer to "where does window-transient get involved in an evaluate invocation at all": **it
   is not involved in the extension call itself — it is involved because the flow-eval-tick command
   that wraps the call always also republishes its own status text into the window's transient
   store as a side channel**, and that side channel and the extension call share the SAME guest
   turn / SAME `mounted.pending_artifact_publication` slot in `🔌️plugin/🦀️.rs`.
2. **Guest queues the effect.** `⚛️reactor/🦀️.rs:713` `queue_extension_invocation` mints
   `Effect::invoke_extension(req, extension_id, capability, request_json)` via
   `RequestRegistry::request_continuation`, recording `response_action = "flowEvalResolve"`.
   `⚛️reactor/🦀️.rs:725` `drain_queued_effects` folds it into `turn-result.effects` for that turn's
   `poll`.
3. **Host dispatches.** `🏛️ShellHost/🟦️.tsx:5217` (`"invokeExtension" in effect`) calls
   `dispatchInvokeExtensionEffect` (`🏛️ShellHost/🟦️.tsx:1713`), which resolves the extension plugin
   by `pluginId`, calls `captureExtensionCompletion` + `runCapturedExtensionEffect`
   (`🏛️ShellHost/🟦️.tsx:1660`): invokes `extensionEntry.handle.invoke(capability, requestJson)` on
   the flow-extension-math guest directly (no window-transient involvement on this side — that
   extension has no windows), packs the JSON answer (`encodePackValue`), then calls
   `completion.complete(outcome)` (`:1691`).
4. **Answer re-enters the REQUESTING guest** (generation3d) as a `flowEvalResolve` dispatch. This
   drives a fresh turn of the SAME actor. `TypedOperationCall`/`TypedOperationRouter`
   (`🔌️PluginRuntime/🟦️.tsx:567-627`) and `consumeTypedOperationEffects`/`withTypedOperationCall`
   (`:634-701`) consume that turn's `TurnResult.effects`, looking for
   `TypedOperationResultLane` pages (`Artifact 0 … Fault 11 … WindowTransient 13`, per the doc
   comment at `🔌️PluginRuntime/🟦️.tsx:516`).
5. **This is where the window-transient publication surfaces.** The `flowEvalResolve` turn's own
   command work (`Generation3dFlowEvalWindowWork` in `✏️editor/🦀️.rs:501-568`, or the resolve
   command's equivalent) tries to advance/close the **pending** `WindowTransient` publication in
   `🔌️plugin/🦀️.rs`'s ACK loop (`:23958-23969`). If that publication is still `Closing` with
   `fault()` set (the OLD behaviour, pre-fix), every turn's attempt to progress it returned
   `Err(plugin_sdk_fault("window-transient publication is retiring a rejected authority"))`, which
   is serialized as a `TypedOperationResultLane::Fault` page. `PluginRuntime`'s
   `consumeTypedOperationEffects` (`:673-674`) turns that into the thrown
   `typed-operation failed: …` JS error, which propagates up through
   `completion.complete()` → `runCapturedExtensionEffect`'s `await completion.complete(outcome)`
   (`🏛️ShellHost/🟦️.tsx:1691`) → `dispatchInvokeExtensionEffect`'s `.catch` at
   `🏛️ShellHost/🟦️.tsx:5235-5237`, which is exactly the observed
   `console.error("[DEBUG] invokeExtension dispatch failed", …)`.

**Files named in the task that are NOT actually on this path** (checked by grep, zero hits for
`invokeExtension`/`typed-operation failed`/`window-transient`):
`🌐️World3dHost/🟦️.tsx` (wgpu mesh canvas host — consumes rendered mesh data, has no effect
dispatch) and `🗣️Interpreter/🟦️.tsx` (unrelated element). The real host-side pair is
**`🏛️ShellHost/🟦️.tsx`** (effect dispatch / extension invoke) and **`🔌️PluginRuntime/🟦️.tsx`**
(typed-operation result-page routing / fault-arm decode/throw), not `PluginRuntime`'s sibling
files that share the "PluginRuntime" directory name loosely implied by the task brief.

`⚛️reactor/🔄️turn/🦀️.rs` itself has almost nothing extension-specific
(`:1617` just names the effect for tracing, `:1671` is a test fixture); the real reactor-side
logic (`queue_extension_invocation`, `extension_response_args`, `take_extension_response`) lives
in `⚛️reactor/🦀️.rs` (no `🔄️turn/` subpath), `:707-803`.

---

## Q3 — State transitions; the exact gap and its fix

Confirmed by diffing `de93f84300` against its parent (`de93f84300^`) on
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

**Before (parent commit, matches the 09-11 bug reports):** every one of the 7 publication lanes
(Artifact, Config, Draft, Presence, Transient, WindowConfig, WindowTransient) had, inside its
`Closing`-phase `match publication.close_step(grant)…`:

```rust
store::SnapshotRetirementStep::Complete if publication.terminal_is_empty() => { … return if failed { Err(…"rejected stale or cancelled authority") } else { Ok(()) }; }
store::SnapshotRetirementStep::Complete => return Err(…"closed without terminal emptiness"),
_ if failed => return Err(plugin_sdk_fault("<lane> publication is retiring a rejected authority")),  // ← fires on EVERY incomplete step while `fault()` is Some
_ => return Ok(()),
```

**After (current `HEAD`):** the `_ if failed => Err(...)` arm is deleted from **all seven** lanes
(`grep -c "_ if failed" 🔌️plugin/🦀️.rs` = 0, was 7). WindowTransient additionally lost the
Complete-and-failed distinction too — it now always returns `Ok(())` on `Complete` +
`terminal_is_empty()` regardless of `fault()` (`:23961-23964`), the most lenient of the seven.

**The missing re-arm, found and fixed in the same commit:** the window-transient "begin a new
publication" call site (two symmetric copies, `Emit(Ok)` branch and `Download(Ok)` branch) changed
from

```rust
let authority = mounted.window_transient_authority.as_ref().ok_or_else(...)?;
let publication = self.window_transient_store.begin(mounted.operation.operation, authority, mutation)?;
```

to

```rust
let authority = mounted.window_transient_authority.as_mut().ok_or_else(...)?;
self.window_transient_store.refresh(authority)?;
let publication = self.window_transient_store.begin(mounted.operation.operation, authority, mutation)?;
```

`refresh()` itself is **new** — added to `🪟️window/🫧️transient/🦀️.rs` in the same commit
(`WindowTransientOwnerRegistry::refresh` at `:312-320`, `ErasedWindowTransientStoreOwner::refresh`
at `:207-219`; both did not exist before). It re-reads `partition.store.generation_now()` and a
fresh snapshot before `begin()` runs.

**So: the step that should have re-issued/refreshed the authority and did not (pre-fix) was the
"begin a new window-transient publication" call itself.** `mounted.window_transient_authority` was
captured exactly once, at command admission (`:24375`, unchanged), and never refreshed again. Once
the pre-contribution tick's fault put a publication into `Closing`, and that closing publication
kept hard-erroring every turn (the `_ if failed` bug), the retry loop never reached a point where a
fresh `begin()` with a current generation could even run — the loop was stuck re-raising the same
fault from the *old* closing publication indefinitely (bounded only by re-arm cadence), which is
exactly `📓️unknown-kind-after-restage-2026-09-11.md`'s observed steady-state: `invokeExtension`
failing on every math `evaluate` after contributions re-armed `flowEvalTick`.

---

## Q4 — Do the generation3d window-transient owners retire on fault themselves?

No — retire-on-fault is **not** owner-specific. Both owners
(`✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs` and
`✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🫧️transient/🦀️.rs`) only declare `State`/`Mutation`
(`Generation3dPreviewWindowTransient { preview_eval_text: Option<String> }` /
`Generation3dPreviewWindowTransientMutation::SetPreviewEval`), a `preflight`/`transfer` pair, and
`WindowTransientOwner::WINDOW_KIND_ID` + `build_owners()`. The generate-mode owner is a thin
wrapper reusing the edit-mode owner's `State`/`Mutation`/`build_owners()` verbatim
(`🎭️modes/🧬️generate/🪟️windows/👁️preview/🫧️transient/🦀️.rs:7-14`), differing only in
`WINDOW_KIND_ID`. Neither file contains `close_step`, `fault`, or any retirement logic — that
machinery is entirely generic, in `🔌️plugin/🪟️window/🫧️transient/🦀️.rs` (`TypedWindowTransientStoreOwner`,
`WindowTransientOwnerRegistry`) and `🔌️plugin/🦀️.rs`'s `PendingArtifactStorePublication::WindowTransient`
arm audited in Q3. The fix in Q3 therefore covers **both** preview windows identically and
automatically (`procedural-preview`/`generation3d-generate-preview`), since they share one owner
implementation and one closing code path — consistent with
`📓️generate-mode-eval-wiring-2026-09-11.md`'s "same state schema as edit, different
`WINDOW_KIND_ID`".

---

## Q5 — Minimal clean fix, ranked, with proof and expected console

**Ranked assessment: the fix is already landed on `HEAD` (`de93f84300`).** There is no further
source change to propose for the fault itself. What remains, ranked by confidence:

1. **(High confidence, mechanical) Restage the wasm and reprobe 6018.** A build newer than the fix
   already exists —
   `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_procedural.wasm`
   (94 394 569 B, mtime 2026-09-11 15:41, 6 min after the fix's commit-message timestamp) — but no
   ticket file records reprobing it against 6018, and `status.md`'s last entry
   (`## 2026-09-11 restage wasm 14:15`) predates it. Whoever owns the browser probe should confirm
   this build is what's actually served (or re-run `nx run @semio-tech/procedural-plugin:component-dev`
   to be certain) and re-run `🐍️restage-eval-probe.mjs`.
   - **Proof without wasm:** none needed — this is a serving-freshness check, not a code change.
   - **Expected console after restage:** no `invokeExtension dispatch failed` /
     `typed-operation failed: window-transient publication is retiring a rejected authority` line.
     `unknown kind: brep.curve.polygon` etc. may still appear (that was called out as a *separate*,
     still-open defect in `📓️unknown-kind-after-restage-2026-09-11.md` §"Next" item 1 — the
     set-contributions native laws use linked packs, not the served guest's unlinked shape).

2. **(Medium confidence, coverage gap) Add a native regression test for the fixed transition,
   since none currently exists.** `grep -rln "rejected stale or cancelled authority\|closed
   without terminal emptiness"` outside `🔌️plugin/🦀️.rs` returns nothing — no test asserts
   "a `Closing` + faulted `WindowTransient` publication does not error on an incomplete
   `close_step`" or "`begin()` after a `Closing` retirement refreshes the authority's generation
   first." Recommended location: `🔌️plugin/🦀️.rs`'s own `#[cfg(test)]` module (it already builds
   fixture apps with `close_step`/`close_terminal_is_empty` helpers around `:6796-6815`), a
   `PendingArtifactStorePublication::WindowTransient` fixture that: (a) begins a publication,
   forces a fault via a superseding mutation so `fault()` becomes `Some`, (b) drives multiple
   `close_step` turns asserting `Ok(())` throughout (not `Err`), (c) on completion, begins a new
   publication for the same window and asserts `refresh()` was applied (fresh `generation`) before
   `begin()` succeeds.
   - **Proof:** `cargo test -p semio-framework-plugin <new_test_name>` — no wasm restage required
     (this crate compiles and tests natively).

3. **(Lower confidence, separate lane) The `unknown kind` fault and the linked-vs-unlinked
   set-contributions test gap are unrelated to this audit's fault and were already flagged in
   `📓️unknown-kind-after-restage-2026-09-11.md`** ("Next" items 1 and 3) — not re-litigated here.

**What the browser console should show after a fresh, verified restage:** no `[DEBUG]
invokeExtension dispatch failed` entries at all for `flow-extension-math`/`evaluate`; `meshes > 0`
or a distinct, unrelated fault (e.g. an `unknown kind` operator-catalogue miss) — anything OTHER
than the word "authority" in an `invokeExtension` error would confirm this specific defect is
closed in the live app, not just in source.

---

## Files referenced

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`:23958-23969`, `:24038-24043`,
  `:24167-24172` window-transient closing/begin; commit `de93f84300` diff against its parent)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs` (`:174-260`
  `WindowTransientAuthority`/partition; `:207-219`, `:312-320` new `refresh()`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
  (`:516-701` `TypedOperationRouter`/`withTypedOperationCall`/`consumeTypedOperationEffects`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
  (`:1649-1729` `dispatchInvokeExtensionEffect`/`runCapturedExtensionEffect`; `:5217-5238` effect
  loop)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  (`:501-568` `Generation3dFlowEvalWindowWork`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`
  (`:47-84` `evaluate`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs`
  and the `🎭️modes/🧬️generate/…` sibling (owner definitions, no retirement logic)
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_procedural.wasm`
  (served build freshness — 94 394 569 B, mtime 2026-09-11 15:41)

Not touched: `🗑️generated` folders were left as found; no ticket state, source file, or git ref
was modified.
