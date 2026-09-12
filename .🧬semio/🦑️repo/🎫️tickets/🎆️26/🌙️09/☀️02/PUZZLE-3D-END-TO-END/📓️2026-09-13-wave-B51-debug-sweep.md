# Wave B51 — Close-Out `[DEBUG]` Sweep

Input: `📓️2026-09-13-audit-A6-debug-trace-inventory.md` (138 session-added lines), method from
`📓️2026-09-10-wave-Z-debug-sweep.md`. Re-derived live against pre-session commit `46c3cb9de0`
(`git log --date=iso --before='2026-09-11 14:10' -1`), plus untracked files.

## 0. Live re-derivation (supersedes A6's counts)

Walk of the scoped areas, excluding `🧪️test*`/`.test.`/`.spec.`/`__tests__`, `/dist/`, `🤖️generated`,
`🗑️generated`, `node_modules`, `⚡️cache`, and non-source extensions. A line counts as session-added when its
whitespace-trimmed text is absent from `git show 46c3cb9de0:<file>` (or the file did not exist then).

| Area | total `[DEBUG]` hits | session-added | pre-session |
|---|---:|---:|---:|
| A1 guest `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/` | 33 | 6 | 27 |
| A2 plugin host `💻️os/🔨️modules/🔌️plugin/` | 64 | 30 | 34 |
| A3 renderer `💻️os/🔨️modules/📺️renderer/` (all) | 347 | 106 | 241 |
| A5 actor `🧰️framework/🔨️modules/🎭️actor/` | 22 | 6 | 16 |
| ui-react `🧰️framework/🔨️modules/🖱️ui/` | 5 | 0 | 5 |
| dev `🧰️framework/🔨️modules/🧑‍💻dev/` | 0 | 0 | 0 |
| **total** | **471** | **148** | **323** |

148 live vs A6's 138: waves B48-B50 added ~10 more, and this walk also reaches
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (22) that A6 folded into a shorter path label.

**Renderer scope.** The wave brief enumerates the renderer host elements in scope: `🏛️ShellHost`,
`🔌️PluginRuntime`, `🌐️World3dHost`, `🛠️ShellHelpers`, `🗣️Interpreter`, `📃️UiDocumentStore`. That selects **49**
of the 106 renderer session-added lines. The other 57 sit in
`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` (28), `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (22),
`🧱️elements/🕸️NodeGraph/🟦️.tsx` (5) and `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (2) — the WGPU native target and
NodeGraph, neither on the :6013 React battery path and both heavily marked
`26/09/09/PROCEDURAL-3D-END-TO-END` in A6. **Deferred, not swept** (see §5).

## 1. Corrections to A6's gating verdicts

A6 gated by a ±25-line text window, which misses a module-level `#[cfg(test)]`. Verified by walking back to
the enclosing attribute:

| A6 verdict | line | actual | evidence |
|---|---|---|---|
| REMOVE | `🔌️plugin/🦀️.rs`:17610 | **KEEP-gated** | `#[cfg(test)]` at 17558 |
| REMOVE | `🔌️plugin/🦀️.rs`:17892 | **KEEP-gated** | `#[cfg(test)]` at 17862 |
| REMOVE | `🔌️plugin/🦀️.rs`:23182 | **KEEP-gated** | emits via `debug_runtime_line`, which is `if runtime_diagnostics_enabled()` (31631-31635) |
| REMOVE | `🔌️plugin/🖥️host/🦀️.rs`:510 | **KEEP** | doc comment on `guest_diagnostics_text`, not an emitter |
| REMOVE | `🌐️browser-bundle/🏗️materialization/🟦️.ts`:100 | **KEEP** | the guest stdout/stderr channel classifier that routes `[DEBUG]`-prefixed guest lines to `console.debug` — infrastructure for the retained diagnostics facility, not a tap |

So every one of the 17 session-added hits in the `semio-framework-plugin` crate root is already
`cfg(test)`- or `runtime_diagnostics_enabled()`-gated. Nothing to remove there.

## 2. Disposition policy

Three outcomes, because "delete the line" is wrong for some of them:

- **REMOVE** — a pure trace tap: logs normal-path state, nothing branches on it. Line deleted, plus any
  variable/helper only it used.
- **STRIP** — a genuine fault report or a `.catch()` handler that happens to carry the session-added
  `[DEBUG] ` marker. The marker goes, the report stays. Deleting these would silence real error paths and,
  for `.catch()`, create unhandled rejections.
- **KEEP / PEER** — gated behind `runtime_diagnostics_enabled()` / `runtimeDiagnosticsEnabled()` /
  `cfg(test)`; or a doc comment describing the retained facility; or inside a
  `26/09/09/PROCEDURAL-3D-END-TO-END` block.

After the sweep the scoped areas carry **no ungated session-added `[DEBUG]` emitter**.

## 3. Area 1 — puzzle 3D guest (6 session-added, 6 REMOVE, 0 KEEP, 0 PEER)

All six were ungated `eprintln!`. The brief asks whether the kept taps should become
`runtime_diagnostics_enabled()` diagnostics instead. **No — all six removed**, because the gate is not
reachable: `semio-s-artifact-puzzle-3d`'s manifest
(`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`) has no `semio-framework-job` /
`semio-framework-trace` edge (`grep -c semio-framework-job` → 0), and the only guest that gates this way is
the peer's `🌀️procedural/…/🧵️preview-eval/🦀️.rs:519`, via `semio_framework_job::runtime_diagnostics_enabled()`.
Keeping three import traces would mean adding a runtime crate dependency to the puzzle plugin purely to carry
debug output — the wrong trade at close-out.

| file:line (pre-edit) | literal | wave | action |
|---|---|---|---|
| `…/✏️editor/🎮️commands/📥️import-fixture/🦀️.rs`:10-14 | `puzzle3d.import.ingress args= payload_len=` | B6/B9 | REMOVE |
| `…/✏️editor/🎮️commands/📥️import-fixture/🦀️.rs`:40 | `puzzle3d.import.parsed objects= before=` | B6/B9 | REMOVE |
| `…/✏️editor/🦀️.rs`:3475-3477 | `puzzle3d.import.apply ops= after_objects=` | B6/B9 | REMOVE (with its `if action == "importFixture"` wrapper, now dead) |
| `…/✏️editor/🎮️commands/🔖️set-selection-flag/🦀️.rs`:23 | `puzzle3d.flag.explicit entity= flag= value= ids=` | B47 | REMOVE |
| `…/✏️editor/🎮️commands/🔖️set-selection-flag/🦀️.rs`:30-36 | `puzzle3d.flag.selection entity= …` | B47 | REMOVE |
| `…/✏️editor/🦀️.rs`:7693 | `puzzle3d.clipboard.copy marks= resolved= objects=` | B47 | REMOVE |

Dead prose removed with them: the 7-line B47 comment at `🔖️set-selection-flag/🦀️.rs`:14-20 ("in the same kept
family as `puzzle3d.import.*`… Without this the two are indistinguishable") described only the tap. The
`"copy"` comment in `✏️editor/🦀️.rs` was trimmed to the three lines that still describe behaviour
(`Emit::default()` == zero effects) and keeps its `wave B47 §4` citation.

## 4. Area 2 — plugin host (30 session-added: 5 REMOVE, 20 KEEP-gated, 5 PEER)

| file:line | literal | action | why |
|---|---|---|---|
| `🔌️plugin/🦀️.rs` ×17 (17610, 17649, 17679, 17816, 17844, 17892, 18462, 18531, 18550, 18627, 18682, 18690, 21172, 21188, 23182, 34198, 34217) | test-law summaries + slot occupancy | **KEEP-gated** | 15 inside module-level `#[cfg(test)]`; 23182 via `debug_runtime_line` (gated); 34198 is that helper's doc comment |
| `🔌️plugin/🖥️host/🦀️.rs`:510, 1683 | doc comments | **KEEP** (1683 also PEER) | prose on `guest_diagnostics_text`, no emitter |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:81, 100 | stdout/stderr channel classifier + its docstring | **KEEP** | routes `[DEBUG]`-prefixed guest lines to `console.debug`; infrastructure for the retained facility |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:375, 393, 396 | guest diagnostics arming | **PEER** | `26/09/09/PROCEDURAL-3D-END-TO-END` block |
| `🌐️browser-bundle/🏗️materialization/🟦️.ts`:631 | `shard worker: retryable lifecycle deadline on …` | **STRIP** | fault-path notice, deliberate per the 626-629 comment |
| `🏪️store/📥️installation/🟦️.ts`:327, 349 | `extension store installed/uninstalled …` | **REMOVE** | success-path traces |
| `📇️registry/🗿️taxonomy-validation/🟦️.ts`:637, 684, 725 | `registry-rust-mounts capture=/case=`, `registry-plugin-root case=` | **REMOVE** | per-case traces inside two oracles that already print a one-shot summary (`registry-rust-mounts-oracle cases=…`); `status` stays used at 661/664 |

## 5. Area 3 — renderer host elements (49 session-added in scope)

### 5a. Removed (14 lines + 2 dead locals)

| file:line (pre-edit) | literal |
|---|---|
| `🏛️ShellHost/🟦️.tsx`:1985 | `leftover forceReload` — with the `const keys: string[]` + `keys.push(key)` that fed only it |
| `🏛️ShellHost/🟦️.tsx`:2034 | `leftover InteractionView` |
| `🏛️ShellHost/🟦️.tsx`:3532 | `… died with shard … re-establishing the session` (the `shardIndex` param it consumed is now `_shardIndex`) |
| `🏛️ShellHost/🟦️.tsx`:5152 | `trace: (step, detail) => console.warn(…)` — the whole property; `trace?:` is optional on `SessionAppSwitchOptions` (`🔀️surface-switch/🟦️.ts`:293) |
| `🏛️ShellHost/🟦️.tsx`:5157 | `surface-switch outcome … pending=` |
| `🏛️ShellHost/🟦️.tsx`:5595 | `invokeExtension requester missing` — with its now-empty `if (!pluginsNow.some(…))` guard |
| `🏛️ShellHost/🟦️.tsx`:6308 | `setActiveTool hop leftover` |
| `🏛️ShellHost/🟦️.tsx`:9546 | `leftover Inspection refresh` |
| `🏛️ShellHost/🟦️.tsx`:9597 | `leftover brushPreview refresh` |
| `🌐️World3dHost/🟦️.tsx`:5025-5027 | **`puzzle3d.brushPreview.bind`** — flood, with its `if (typeof console !== "undefined")` wrapper |
| `🌐️World3dHost/🟦️.tsx`:5664 | `fillBuildTick hop armed` |
| `🗣️Interpreter/🟦️.tsx`:1433-1438 | **`puzzle3d.brushPreview.assemble`** — flood, with its wrapper and the `preview` / `interaction` / `previewLane` locals computed only for it; the block collapsed to a direct `return world3dSceneFromLanes(…)` |
| `🔌️PluginRuntime/🟦️.tsx`:2722 | `extension request answered` — per-request `console.debug` |

### 5b. `[DEBUG] ` prefix stripped, report kept (11 lines)

Every one is a `.catch()` handler or a named fault report. Deleting them would silence a real error path
(and, for the `.catch()`s, produce unhandled rejections).

`🏛️ShellHost/🟦️.tsx` — 3534 `could not re-establish … after a worker loss`, 3693 `hot-swap … predecessor
retirement failed`, 4909 `refresh-owed host effects failed`, 5123 `surface-switch draining …`, 5155
`switchToPluginApp: predecessor … retirement failed`, 5747 `typed-operation completion effects failed`,
7409 / 7416 / 8948 `role switch to … failed`.
`🔌️PluginRuntime/🟦️.tsx` — 413 `<instance-lost fault> … died with shard …`, 418 `an instance-lost listener
threw`, 436 `releasing lost instance … threw`, 1048 `wireEffectToFriendly: send-message … has no host
route`, 1489 `actor … published, acknowledged and emitted nothing for N consecutive continuations`.

### 5c. The `sealedInstanceDropTextV1` contract (formatter + fixture + law, 3 files)

`🔀️surface-switch/🟦️.ts`:154 built its line as `` `[DEBUG] ${drop.code} dropped …` ``, and
`🧫️fixtures/🔀️surface-switch/🔣️.json`:966/972 carried that exact text as the expectation
(`🧪️tests/🔀️surface-switch/🟦️.ts`:626/628 asserts `sealedInstanceDropTextV1(drop) === fixture.….text`).
Prefix dropped in **all three places at once**, so the law stays green and the greppable half is
unchanged — the unit's own docstring at :143 already says *"`code` is the stable half: laws and log greps
match on it, never on the prose."*

### 5d. Reverted after the lane caught it — the contributions block is PEER (6 lines)

The push block's own docstring (`🏛️ShellHost/🟦️.tsx`:4503) names ticket
**26/09/09/PROCEDURAL-3D-END-TO-END**, and the peer's laws in `🧪️tests/🩺️window-fault/🟦️.ts` pin the log
literals as *source text*:

```
150:     expect(shellSource).toContain('scopedContributionsJson === "[]"');
151:     expect(shellSource).toContain("refused empty pack");
161:     expect(shellSource).toContain("unresolved document operators");
163:     expect(shellSource).toContain("contributions scoped from published examples");
```

A6 marked these REMOVE; the first lane run turned them into 2 failures. All six lines (4529, 4533,
4539, 4540, 4566, 4628 pre-edit, plus the 4625 `.catch()` prefix) are **restored verbatim and
reclassified PEER** — this is exactly the ownership re-check A6 §6 asked for. They are per-push, not
per-frame, so they are not a flood.

### 5e. `gumball pose delta skipped` — converted to a permanent record

`🌐️World3dHost/🟦️.tsx`:6054 was the landmark a wave-B31 source-text law slices the zero-delta branch on
(`🧪️tests/🔬️engine-contract/🟦️.ts`:8052 `branch.indexOf('"[DEBUG] gumball pose delta skipped"')`), so
removing it broke the law. This one is **this** ticket's own (wave B31), so instead of restoring the
marker the record was made **permanent**: `console.info("gumball pose delta skipped", …)` with no
`[DEBUG] `, the comment now says *"PERMANENT, not a `[DEBUG]` trace"*, and the law's landmark was updated
to match. Same precedent the codebase already uses at `🔌️PluginRuntime/🟦️.tsx`:1965 ("one console record
per dropped body, permanent (not a `[DEBUG]` trace)").

### 5f. Area 5 — actor package (6 session-added: 3 STRIP, 1 docstring, 2 KEEP/PEER)

`🖼️wire-turn/🟦️.ts`:39 `coerceWireBytes: unsupported payload` (a real `throw`), 552 `send-message … has no
host route`, 556 `unmapped effect … dropped` — prefixes stripped; the comment at :549 says this path
"stays loud" on purpose. The docstring at :483 said the fallback "degrades to an honest
`` `[DEBUG]` ``-logged drop" and now reads "console-logged". `🧵️shard-runtime/🟦️.ts`:50 is accurate prose
about the retained `shardRuntimeDiagnosticsArmed()` facility — KEEP.

## 6. Deferred, with reasons

| what | where | why not swept |
|---|---|---|
| 57 session-added lines | `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` (28), `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (22), `🕸️NodeGraph/🟦️.tsx` (5), `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (2) | outside the brief's enumerated element list, not on the :6013 React battery path, and A6 marks 9 of the wgpu-shell lines PEER |
| 3 lines | `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`:1240/1292/1296 (`ui-doc reconcile fault`, `paint stalled`, `paint fault`, via the **ungated** `document_debug_log`, :1154) | **a peer is writing this file live** — mtime 2026-09-12 20:27, three minutes before the sweep reached it, and line numbers had already shifted from the inventory with a new `paint stalled` line appearing. Not touched rather than racing a live writer |

## 7. ⚠️ Correction to A6 §4 — the biggest console flood is LIVE, and pre-session

A6 concluded that flood families #1-#4 (512 of 1197 console-tail lines, **43%**) were "stale — the literal
text no longer exists anywhere in the working tree". **That is wrong.** All four emitters are in the
current tree, in `🧱️elements/🔌️PluginRuntime/🟦️.tsx`:

| A6 rank | count in tail | emitter, current line | cadence |
|---:|---:|---|---|
| 1 | 148 | :2380 `console.warn(\`[DEBUG] command ingress settled status=…\`)` | once per command ingress |
| 2 | 147 | :2343 `console.warn("[DEBUG] command ingress lane", …)` | once per command ingress |
| 3 | 109 | :2904 `console.warn("[DEBUG] performInvocation settled", …)` | **once per invocation** |
| 4 | 108 | :2889 `console.warn("[DEBUG] performInvocation", …)` | **once per invocation** |

All four are **pre-session** (verbatim in `git show 46c3cb9de0:…`), so they are out of B51's scope by
attribution rule 2 and were left alone. They are now the single largest remaining source of console
noise — larger than the `brushPreview` pair this wave removed. **Recommend a follow-up wave** gate them
behind `runtimeDiagnosticsEnabled()`; they are all in one file and all ungated `console.warn`.

Two related pre-session, ungated, per-interaction taps in the same class, also left by rule 2:
`🌐️World3dHost/🟦️.tsx`:6066 `[DEBUG] gumball pose delta` (every committed gumball drag) and
`🏛️ShellHost/🟦️.tsx`:4582 `[DEBUG] contributions document sources`.

Method note: A6's "zero hits repo-wide" almost certainly came from `rg`/`grep` treating these
long-line UTF-8 sources as **binary** and skipping them. The same thing bit this wave —
`grep -c "DEBUG"` on `🔌️PluginRuntime/🟦️.tsx` returns nothing while `grep -ac` returns 58. **Use
`grep -a` (or Python) on this repo's emoji/long-line sources; a bare `grep` silently reports zero.**

## 8. Verification (all foreground, tails quoted)

| gate | result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished \`dev\` profile [unoptimized] target(s) in 33.30s` — **0 errors**, 115 warning lines (92 lib warnings, all pre-existing; none in the touched guest files) |
| `cargo check -p semio-framework-plugin` | `Finished \`dev\` profile [unoptimized] target(s) in 31.71s` — **0 errors**, 22 warning lines |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0 … Finished \`dev\` profile [unoptimized] target(s) in 23.27s` — **0 errors**, 125 warning lines |
| renderer-react lane, `SEMIO_TEST_LEVEL=long` | `Test Files 1 failed \| 33 passed (34)` / `Tests 6 failed \| 1037 passed (1043)` — **6 = the B49 baseline exactly, 0 new**; all 6 are `🧪️tests/🧩️package-integration/🟦️.ts` (`ReferenceError: Bun is not defined`, worker-byte determinism), untouched by this wave. Passing count rose 1033 → 1037 |
| *(first run, before §5d/§5e)* | `Tests 9 failed \| 1034 passed` — the 3 extra were the source-text laws in §5d/§5e; both fixed, re-run green |
| ui-react lane, `SEMIO_TEST_LEVEL=long` | `Test Files 2 failed \| 20 passed (22)` / `Tests 13 failed \| 718 passed (731)` — **13 ≤ the 14 pre-existing baseline, 0 new**; all 13 are icon-hover / celebrate / tutorial / ContextMenu / UIDialog laws, and this wave changed **no** ui-react file |
| `bun x tsc --noEmit` | 3556 pre-existing project-wide error lines (top offenders are generated `jco` `.d.ts` bindings and the FEM plugin `🚪️io` modules); **0 in any file this wave touched** |
| `🧪️tests/🔀️surface-switch` + `🧩️contributions-push` + `🩺️window-fault` + `🔬️engine-contract` | all four are engine suites in the react lane and all four are among the 33 passing files |

### Before / after `[DEBUG]` count per area

| area | before (total / session-added) | after (total / session-added) | session-added removed |
|---|---|---|---:|
| A1 guest | 33 / 6 | 27 / **0** | 6 |
| A2 plugin host | 64 / 30 | 58 / 24 | 6 |
| A3 renderer (all) | 347 / 106 | 318 / 77 | 29 |
| A3 renderer (**in scope**) | — / 49 | — / 20 | 29 |
| A5 actor | 22 / 6 | 18 / 2 | 4 |
| ui-react | 5 / 0 | 5 / 0 | 0 |
| dev | 0 / 0 | 0 / 0 | 0 |
| **total** | **471 / 148** | **418 / 95** | **53** |

The 95 remaining session-added split into **46 in the swept scope** — every one accounted for — and 49
deferred (§6):

| classification | count | detail |
|---|---:|---|
| KEEP gated | 21 | 15 `#[cfg(test)]` + `34217` (cfg(test) **and** a `runtime_diagnostics_enabled()` early-return at :34214) + `23182` (`debug_runtime_line`, gated at :31632) in the plugin crate; 5 `if (runtimeDiagnosticsEnabled())` in ShellHost (4780, 4908, 5699, 5702, 5735) |
| KEEP doc | 8 | doc comments describing the retained facility — plugin crate :34198, `🖥️host/🦀️.rs` :510/:1683, `materialization` :81/:375, ShellHost :4503, World3dHost :6052, `shard-runtime` :50 |
| KEEP infra | 1 | `materialization/🟦️.ts`:100 — the guest stdout/stderr classifier that routes `[DEBUG]`-prefixed guest lines to `console.debug`; the facility, not a tap |
| PEER | 14 | `materialization` 393/396; ShellHost 1752, 4524, 4528, 4534, 4535, 4548, 4561, 4620, 4623, 6156; wgpu Interpreter :1240; actor fixture `📨️effect-wire-routes/🔣️.json`:2 |
| DEFERRED | 2 | wgpu Interpreter :1292/:1296 — live peer writer (§6) |

**No ungated, non-peer, session-added `[DEBUG]` emitter remains in the swept scope.**

## 9. Close-out file list for the coordinator (nothing deleted by this wave)

### DELETE — tool output under `🗑️generated/`

| ext | files | size |
|---|---:|---:|
| `.png` | 2476 | 484.03 MB |
| `.md` (probe reports) | 465 | 37.73 MB |
| `.txt` | 454 | 27.71 MB |
| `.ndjson` | 357 | 2.34 MB |
| `.json` | 73 | 1.85 MB |
| `.js` | 2 | 0.43 MB |
| `.ts` | 2 | 0.00 MB |
| `.log` | 1 | 0.08 MB |
| `.bak` | 1 | 0.01 MB |
| **total** | **3831** | **554.18 MB** |

Two of those are input-shaped and worth rescuing to the ticket root before the sweep rather than losing:
`🗑️generated/wasm-net-inspect.ts` and `🗑️generated/vortex-dom-check.ts` (probe helpers). The `.js` pair
(`w-g3-served-enc-component.js`, `w-g3-served-enc-bridge.js`) are captured served artifacts — delete.

### ⚠️ Two paths a `🗑️generated` glob will MISS

1. **`🗑generated/`** — `1F5D1 67 65 …`, i.e. 🗑 with **no U+FE0F variation selector**, a sibling of the real
   `🗑️generated/` (`1F5D1 FE0F 67 …`). It holds **59 files, 3.57 MB** of pure tool output (`*.log`,
   `*.txt`, `before/after-heap-law.log`, `isolated-*.log`, `w-ab-*/w-g3-*.txt`, `wasm-check.log`). A
   close-out sweep keyed on the variation-selector spelling will leave all 59 behind.
2. **`<U+FFFD>️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑generated/scan-undo.py`** — a nested duplicate ticket
   tree whose year segment is a replacement character (the same corruption `git status` shows as
   `🎆️26/�…09/`). The one file in it, `scan-undo.py`, is an **input** script; the corrupted path is the
   problem, not the file.

### KEEP — ticket inputs and reports (211 root files)

| kind | files | size |
|---|---:|---:|
| `📓️`/`📋️` reports & audits `.md` (incl. this one) | 166 | 3.90 MB |
| `🔍️` probe scripts | 33 | 0.46 MB |
| `🔨️`/`🐍️`/`🐚️` repair & build scripts | 11 | 0.03 MB |
| `🎫️ticket.json` | 1 | — |

`📌️important/` (1 file) is untouched — per project memory it must be cleared **last**, after the summary.

## 10. Files changed by this wave

**Guest (3)** — `📥️import-fixture/🦀️.rs`, `🔖️set-selection-flag/🦀️.rs`, `✳️any/✏️editor/🦀️.rs`.
**Plugin host (3)** — `📇️registry/🗿️taxonomy-validation/🟦️.ts`, `🏪️store/📥️installation/🟦️.ts`,
`🌐️browser-bundle/🏗️materialization/🟦️.ts`.
**Renderer (6)** — `🏛️ShellHost/🟦️.tsx`, `🏛️ShellHost/🔀️surface-switch/🟦️.ts`,
`🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`, `🌐️World3dHost/🟦️.tsx`, `🗣️Interpreter/🟦️.tsx`,
`🔌️PluginRuntime/🟦️.tsx`.
**Tests (1)** — `🧪️tests/🔬️engine-contract/🟦️.ts` (the §5e landmark).
**Actor (1)** — `🖼️wire-turn/🟦️.ts`.

