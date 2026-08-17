# Lane J2 — Runtime proof (mutation messages + merge policy, end to end in the browser)

## Honest headline

**Steps 3–7 were NOT observed at runtime.** No browser dev-preview of any plugin (`dag`, `s`, `cad`)
could be kept alive long enough in this session to load a document. This is reported as a genuine
environment/tree condition, not invented as a pass — see "Boot attempts" below for the evidence. In
its place this report gives (a) a real, previously-undetected **compile break** found while trying to
boot, and (b) a thorough **static/code-level trace** of the exact runtime paths for every one of steps
3–7, with file:line citations, so the coordinator knows precisely what is wired and what is still
dead, pending an environment where the boot actually completes.

## Boot attempts (what was actually tried)

`.claude/launch.json` was extended with `dag-react` (`bun ./📜️script.ts dev dag`, port 6017) and
`s-react`/`cad-react` variants, all via `mcp__Claude_Browser__preview_start`
(never `Bash` — per the brief). Roughly a dozen `preview_start` calls were made across three plugins
(`s`, `dag`, `cad`) over this session. Two distinct real problems surfaced:

1. **Concurrent Cargo Workspace Churn** (matches the documented pattern in project memory). `ps aux`
   repeatedly showed sibling lanes' own `cargo test -p semio-framework-os-kernel --lib -- <single
   test>` / `cargo check -p semio-s-plugin-space` runs holding the shared `target/` dir — the dev
   server's own `wasm-pack build` logged `Blocking waiting for file lock on build directory` for
   several-minute stretches, multiple times, across multiple attempts.

2. **A real compile error**, once the lock cleared, in `dag`'s wasm target (see next section) — this
   is the actionable finding.

3. On top of both, **dev-server processes kept dying on their own within roughly a minute of spawn**,
   independent of the above two causes — confirmed via `ps aux` immediately after `preview_start`
   (process present) followed by a `preview_logs`/`ps aux` check moments later (process and its
   tracked `serverId` both gone, `preview_logs` returning "not found", `preview_list` consistently
   returning an empty process array all session). Whether this is a spawn-lifetime limit in the
   preview tool itself or resource pressure on a machine running many other concurrent agent
   sessions' `rustc`/`cargo` processes (`ps` showed several such processes at 79–105% CPU
   throughout) could not be determined from inside this session. `curl http://localhost:<port>`
   never once returned a response across ~10 distinct attempts spanning `6017` (dag) and `6020` (cad).

Net result: **zero successful page loads**, so no console transcript or screenshot exists to attach.

## Finding 1 — real compile break in DAG's WASM bridge (new)

Captured verbatim in `🧪️j2-dag-wasm-pack-error.txt` (this folder), from `preview_logs` on the one
attempt that got past the lock:

```
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:8650:43
    |
8650 |             Ok(Self { store: RefCell::new(store) })
    |                              ------------ ^^^^^ expected `ArtifactStore<DagSnapshot, DagMutation>`, found `Result<ArtifactStore<..., ...>, ...>`
```

`♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:8628-8651`, `mod wasm_bridge` (gated
`#[cfg(target_arch = "wasm32")]`), `DagSnapshotVcs::new`: `DagStore::new(envelope)` resolves to
`ArtifactStore::<DagSnapshot, DagMutation>::new`, whose signature (`🏪️store/🦀️component.rs:4289`,
lane 1's/contract C6 change) is `pub fn new(envelope: …) -> Result<Self, VcsError>` — this call site
was never updated to unwrap/propagate the `Result`. `git status --porcelain` on the file is empty:
this is the tree's actual current committed state, not a mid-edit race from a concurrent lane.
Scanned every sibling `♾️infinite/🎲️board/🔌️ports/➡️directed/*/🦀️component.rs` for the same
`Store::new(envelope)` pattern outside `.expect()`/`?`/`map_err` — **only `🕸️dag` has it**.

**Why this slipped through every lane's `cargo check`/`cargo test` in this ticket's entire fan-out**:
`mod wasm_bridge` is `target_arch = "wasm32"`-gated, so it is skipped entirely by every native-target
build. It only compiles under `wasm-pack build --target web` — i.e. an actual dev-server boot, which
appears not to have been exercised by any prior lane's verification (all `🧪️*.txt` logs in this ticket
folder are native `cargo check`/`cargo test` output). **This is a real, currently-broken, wasm32-only
gap in this ticket's own verification coverage**, worth the coordinator's attention independent of
J2's runtime-proof mandate. Not fixed here (outside this lane's `[DEBUG]`-only lease).

## Finding 2 — merge-policy Settings control is wire-dead (confirms an already-flagged gap)

Traced by direct code read (not runtime-observed, but unambiguous from the source): the Settings
panel's merge-policy `Select` (`ChromePanels/🟦️component.tsx` ~466-483) dispatches through
`ShellHost/🟦️component.tsx`'s `dispatchSetMergePolicy` (3685-3693), which does
`dispatch({type:"SET_MERGE_POLICY", …})` (real, updates the visible Redux/UI state) then
`plugin.handle.setMergePolicy?.(...)` — a call that **always silently no-ops**: `plugin.handle` is
built by `adaptPluginHandle` (`PluginRuntime/🟦️component.tsx:100-102,329-410`), whose returned
object's key set (`pluginId, manifest, createApp, destroyApp, handleAction, handleCommand,
refreshUi, contextMenu, readHistory, applyMutations, readAppDocument, loadAppDocument,
loadAppDocumentPack, attachBackbone, detachBackbone, ephemeralSnapshot, documentPack,
transactionPrepare/Commit/Rollback/Undo/Redo, dispose`) has no `setMergePolicy` (or
`resolveConflict`/`readConflicts`) member, even though `AppChannelClient`
(`💻️os/🟦️component.ts:2492`, methods at 2668/2677/2682) genuinely implements and wire-tests all
three (byte-vector parity tests per `📓️w2-c-report.md`). `ShellHost` casts `plugin.handle as
PendingAppChannelMethods` — a local, ad hoc, structurally unbacked type (3643-3655) — so the
optional-chained call is always `undefined?.(...)`.

**Consequence**: in this dev preview, moving the Settings merge-policy `Select` would change what the
UI *displays* but would not reach the guest — task step 5 ("switch to Vigilant… confirm now rejected
too") cannot pass as-is, independent of whatever plugin is open. This is not a new discovery — lane
2-D's own `📓️w2-d-report.md` "Known gap" section already names this exact seam ("`PluginWasmHandle`
doesn't expose `setMergePolicy`/`resolveConflict` yet — same documented gap as
`openArtifact`/`setDefaultApp`"). J2 confirms it is still open and pins the exact dead call path.

## Static trace of steps 3–7 (code-certain, not runtime-observed)

- **Facet**: `s.dag.dag@1/*#editor`, `app_id = "dag-play"`
  (`✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️component.rs:40`).
- **Step 3, Warning**: rename a node to its own current id →
  `🧬️mutations/🏷️rename-node/🔺️diff/🦀️component.rs:16-18`,
  `MutationOutcome::empty().warn("mutation.no-op", "Node \"{id}\" already has that id.")`.
- **Step 3, Error**: act on a node id absent from the snapshot (rename or delete) →
  `🏷️rename-node/🔺️diff/…:14` / `🗑️delete-node/🔺️diff/…:12`,
  `MutationOutcome::error("mutation.target-missing", "Node \"{id}\" does not exist.", [id])`. Note:
  the task prompt's own phrasing loosely offered "delete something already gone" as a *Warning*
  example — the frozen contract (`📋️contract-freeze.md` C2 table) and the actual code agree
  `mutation.target-missing` is always **Error**, not Warning; the two example gestures the prompt
  suggests (idempotent rename vs. act-on-missing-target) are the right split, just both landing as
  Error for the latter, never Warning.
- **Step 4, toast + rejection wiring**: `ShellHost/🟦️component.tsx` — both `onAction`'s
  `plugin.handleAction` `.catch` (~2959) and `onCommand`'s `plugin.handleCommand` `.catch` (~4656)
  call `isMutationRejectedFault`/`showMutationRejectedNotice` (3832-3844): tone = `fault.severity`,
  body = `ui.mutation.code.<code>.label` + prose, DOM marker `data-notice-code="mutation.rejected"`
  (5852), auto-dismiss 4000ms (3821). `VcsError::Rejected` (C6) means nothing was applied — the
  document-unchanged half of step 4 is a store-level invariant (`replay_mutations`/`ingest_remote`
  atomicity, `📋️contract-freeze.md` C6), not separately re-verified here at runtime.
- **Step 5**: Normal/Vigilant/LaissezFaire semantics are in `MergePolicy::rejects`
  (`📡️spr/🧾️wire/🦀️component.rs`, contract C3) and are exercised by a real wasmtime e2e test
  (`📓️w2-b-report.md` §4: `merge_policy_gates_a_real_dispatch_and_laissez_faire_still_surfaces_its_message`,
  using the real `block.wasm`, asserting Normal rejects `mutation.target-missing` with the document
  byte-for-byte unchanged, and LaissezFaire applies it while still surfacing the message) — this is
  genuine runtime proof, but at the **Rust wasmtime host** layer, not the **browser**. The browser
  layer cannot currently reach parity per Finding 2 above.
- **Step 6, Conflicts panel**: `ChromePanels/🟦️component.tsx` region `🔖️ConflictsPanel` (979-1034),
  tab `framework.settings.conflicts`, label `ui.conflict.panel`, icon `triangle-alert`, order 4 in the
  Settings tab strip; empty state renders `ui.settings.unavailable` — mounts whenever a
  `ConflictsHostApi` is supplied, independent of whether any conflicts exist (satisfies "empty state
  is acceptable evidence it mounts" if reached).
- **Step 7, German locale**: `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:2422-2466`
  (de) / `3202-3318` (en) — real, non-placeholder translated strings under both `ui:` blocks, e.g.
  `ui.mutation.code.targetMissing` → "Ziel fehlt" / "Das Ziel dieser Änderung existiert nicht mehr.",
  `ui.mutation.policy.setting.label` → "Merge-Richtlinie", `ui.conflict.panel` → "Konflikte". Both
  bundles `satisfies UiTranslationSchema`, so a missing German leaf is a TS compile error by
  construction (per `📓️w2-d-report.md`'s own typecheck verification) — strong (compile-time, not
  merely visual) evidence against "raw key text"/crash, though not the same as seeing the rendered
  page.

## Files in this folder

- `📓️j2-runtime-proof.md` — this report.
- `🧪️j2-dag-wasm-pack-error.txt` — full captured compiler error + source context for Finding 1.
- `.claude/launch.json` (repo root, not this folder) — gained `s-react` (port 6070), `dag-react`
  (port 6017), `cad-react` (port 6020) entries for future boot attempts; left in place, not reverted,
  as they're reusable dev-server shortcuts, not scratch files.

No `[DEBUG]` logging was added to any product source file — the browser never stayed up long enough
to reach the point of needing it, so there is nothing to remove.
