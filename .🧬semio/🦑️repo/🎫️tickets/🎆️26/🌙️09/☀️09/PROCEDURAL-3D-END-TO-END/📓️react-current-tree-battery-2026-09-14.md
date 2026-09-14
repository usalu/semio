# React Current-Tree Battery — lane `react-current-tree-battery` (2026-09-14, session 5)

Port 6018 (React dev serve). This lane's job: prove the generation3d editor works end to end on the
CURRENT tree, turn the battery's three `undecided` keyboard steps into real verdicts, and fix the
product defects that stand in the way.

- Evidence: `🗑️generated/react-s5/` (restage logs, boot console, native laws) and
  `🗑️generated/react-verify/scoreboard.json` (the battery's own root).
- Probes changed: `🐍️editor-verbs-keyboard-probe.mjs`, `🐍️react-battery.mjs` (`keyboard-verbs` verdict).
- Script added: `📜️restage-retry-s5.sh`.

---

## 1. What was stale

| thing | state at 17:05 | why it mattered |
|---|---|---|
| served guest wasm | 15:31 | one file of its dependency closure was newer |
| `🗣️dsl/✨️derive/🦀️.rs` | 16:57 | a PROC MACRO in the closure — a change there invalidates every crate downstream, so the whole guest was formally stale (the change itself, a Windows `\\?\` canonicalization strip, is a no-op on macOS) |
| `🔌️plugin/⏯️tool-run/🦀️.rs` | 17:14, 17:28 | the peer's `recent_trace` de-duplication and the new ready group — landed AFTER the served wasm |
| host TS on 6018 | fresh | verified by `curl`-ing `/@fs/<abs path>` for the modules edited at 14:55–15:08 and 18:3x and diffing against disk: vite re-transforms on change even with `SEMIO_VITE_HMR=0`, so the serve did NOT need recycling this session |

**Method note for the next reader:** `grep` treats these source files as BINARY (emoji + long lines) and
prints nothing without `-a`. A `grep -rn` that finds no hit in `🟦️.tsx`/`🦀️.rs` is very likely a false
negative — always `grep -a`. Half an hour went into a "the serve is stale, the symbol is gone from disk"
conclusion that was only a suppressed grep.

Restage command (foreground, retried through peer churn): `📜️restage-retry-s5.sh` →
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`.
First restage 17:29 → 18:12 green. Two later attempts died on shared-tree failures that were NOT this
lane's: `No space left on device` writing the wasm32 incremental cache (239 GB under
`⚡️cache/cargo/build`), and a peer's `DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE` half-landed in
`♾️infinite/🎲️board`. Both cleared by themselves; the retry loop is the right shape for this tree.

---

## 2. The defect that stopped every evaluation

**Symptom** (boot console, `🗑️generated/react-s5/boot/console.txt`, 17:14 run on the 15:31 wasm): 17
lines of

```
typed-operation completion effects failed SemioFaultError: 1:framework.panel.toolRun: DuplicateSiblingKey
    at retainedUiRefreshEffects (🔌️PluginRuntime/🟦️.tsx)
[DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-brep, capability: tessellate, req: 6n, …}
```

one per extension round trip (`flow-extension-math evaluate req 1`, `flow-extension-brep evaluate`
and `tessellate` req 2..6). The preview pill stayed `Running · Evaluating nodes (1/2)` and no example
ever converged.

**Chain.** Three layers, each wrong on its own terms:

1. **The guest published a panel with duplicate sibling keys.** `ToolRunEntry::apply_tick` pushed every
   `ToolRunTraceOp::Upsert` key onto `recent_trace` without removing an earlier copy, and ignored
   `Retire` entirely. A trace key is re-upserted whenever a node's verdict changes
   (testing → success/danger — the normal life of every evaluated node), so the panel's Attempts tree
   emitted `framework.toolRun.<run>.trace.<key>` twice and the retained-UI producer refused the whole
   surface. **Fixed by the `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` peer at 17:14**, with a law
   (`tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position`) — not by this lane.
   This lane confirmed the cause from their diff and verified the law runs green (§5).

2. **One surface's render fault aborted every other operation's completion effects.** THIS lane's fix.
   The guest reports a mounted surface's terminal through `shell_fault_effect` on whatever turn its
   reconciler happened to terminate on, which is almost never the turn that caused it. The React host
   read EVERY `AppFrame::Error` in a turn's effects as that turn's answer and threw, so
   `applyHostEffects` rejected and the `flow-extension-brep` completion lost its effects — the
   evaluation result never landed. A surface-render fault is now scoped to the surface it names:
   reported (`console.error`, still matched by the battery's fault regex), the turn keeps its other
   effects, and every other `Error` frame still throws.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
     — `SURFACE_RENDER_FAULT` + the `retainedUiRefreshEffects` law.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — `SURFACE_RENDER_FAULT_CODE`,
     the Rust half of the same wire constant; `⚛️reactor/🔄️turn/🦀️.rs` now emits it by name.
   - Law: `🧪️tests/🔌️plugin-runtime/🟦️.tsx` — "scopes a surface-render fault to its surface and keeps
     the same turn's other effects" (the pre-existing "reports a refresh fault frame" throw law is
     untouched and still green).

3. **The fault named a surface and nothing else.** A `DuplicateSiblingKey` in a hundred-node panel gave
   an author no way to find the node. `ComponentTreeProducer::duplicate_sibling` now reads the refused
   child out of its frame (the producer already retains both), and `PatchTracker::take_render_fault`
   reports `<surface> [producer]: DuplicateSiblingKey parent=<key> key=<key>`. It also names WHICH
   authority read the fault, because the two know their surface differently: a producer terminal carries
   its own `SurfaceId`, while a reconcile terminal is matched back to a slot BY GENERATION and degrades
   to `unknown surface` once that slot is gone — a reader could not previously tell an exact attribution
   from a derived one.
   - `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs`,
     `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`,
     fixture `⚛️reactor/🩹️patches/🧫️fixtures/🗂️catalogue-surface.json`,
     law `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`.

**The ToolRun pill's frozen label.** Reported by the coordinator on 6021 at 18:28: a `Finalized` run
still read `Evaluating nodes (1/2)`. `tool_run_panel_group` published the last ANNOUNCED string rather
than the current one — the announcement throttle was implemented by refusing to change the text, and a
sighted reader watches the same string a screen reader hears. The text is now always current and only
`Liveness` is rationed (`Off` between announcements, `Polite`/`Assertive` on a state change or once per
`TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`.

---

## 3. The three `undecided` keyboard steps, now decided

`🐍️editor-verbs-keyboard-probe.mjs` used to press every chord in edit mode on a quiet shell and the
battery exempted the three rows that could not possibly do anything there. Both sides are fixed: the
probe presses each chord in the state that OWNS it and publishes its own `ok` plus the `oracle`
sentence it was decided by; the battery's `keyboard-verbs` verdict has no exemption list left and
treats a row without an `ok` as red.

| row | pressed in | oracle |
|---|---|---|
| `baseline` | boot, no chord | the preview window published a status AND the shell invoked nothing |
| `add-generation` (`mod+shift+g`) | GENERATE mode, after `Meta+Alt+ArrowRight` mounts the generate preview | `addGeneration` is invoked AND the Generations roster grows by exactly one row |
| `cancel-preview-eval` (`mod+.`) | on an evaluation in flight — the slowest example loaded, polled until the preview publishes `cancellable: true` | `toolRunAbort` is invoked AND the preview stamps `phase: "cancelled"` with `cancellable: false`, which is `🧫️fixtures/🛑️preview-cancel.json`'s own cancellation law |

Two new rows come with them (`generate-mode`, `cancellable-evaluation-armed`), so a failure can say
whether the chord missed or the STATE was never reached.

---

## 4. Battery

<!-- BATTERY RESULTS -->

---

## 5. Laws run

<!-- LAWS -->

---

## 6. Not claimed

<!-- NOT CLAIMED -->
