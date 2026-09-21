# S9 — Home lists the user's studios, and the sweep's staged forms

Slice S9, session 6, 2026-09-21. Inherited from `📓️s8-typed-operation-drain-and-undo-gaps.md` §3
(the bootstrap reaches `idle` with a byte-exact receipt + ACK, Home still renders "No studios yet")
and §4 (the 13 kinds without undo attributed), and `📓️s7-spawned-refresh-lag-and-agent-targeting.md`
§4 (22/35 full round trip, the permanent sweep probe).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read from
source only.

## 0. tl;dr

| item | result |
|---|---|
| **the root of "No studios yet"** | **found and named by the guest itself, then fixed in host TS.** Neither `runUiRefreshPass`'s render view state nor `onAction`'s `baseDispatchViewState` ever stamped `sessionIdentity`. Only `resolvedTargetViewState` (browser-actor + `setAppRegistrations`) did. So the *whole product* rendered and dispatched every app identity-less, and `🪐️space`'s Home — which answers `None` identity with its EMPTY case by design — showed "No studios yet" on a signed-in shell whose bootstrap was green, receipt byte-exact and ACK published (§2) |
| **the witness** | `createStudio refused: dispatch-failed (user window=s-home-main) — retained command reducer rejected operation: app.message s.home.session-identity-required`, on a shell signed in for 90 s (§2.2) |
| **S8 §3's two candidates** | both **disproved**: it is not an instance mismatch and not `home_space_rows`' membership filter (that function has no user filter at all — it unions every directory space with the local catalog) (§2.1) |
| **S8 §4.1 corrected** | the `↶` S8 read as an "undone marker" is the History panel's **revert BUTTON glyph** (`entry.revertible && !isViewer`, `🏛️ShellHost/🟦️.tsx:9791`). `#s-checkin` is NOT the wrong oracle: it counts `kind === "mutation"` entries of the FOCUSED program's projection and is exactly right. `flow`/`sequence`/`procedural` do **not** round-trip (§3.1) |
| **`norm`'s refusal is guest-side, not host-side** | S8 called it an `s`-host routing defect. It is `ArtifactEditor::command_from_action`'s own trait default, raised by the guest: **14 of the 15 norm editors never overrode it**, so every Actions-rail row of those apps is inert in any shell. Fixed with one shared macro + a codemod (§3.2) |
| **the sweep's staged forms** | the filler is landed (combobox / native select / `input[type=range]` / Radix `[role=slider]` / id fallbacks / execute fallbacks / covered-row recovery), ported from `🐍️b3a-interaction-probe.mjs`, and the verb+arg maps are now **baked into the probe** instead of living in a shell variable. First chunk measured `energy rename-zone` reaching the guest and being refused for a DOMAIN reason (§3.3) |
| **the sweep number** | **not re-measured to completion**: the 02:30 machine clean killed the run mid-chunk and took every serve and both hubs with it. Honest state in §3.4 |

## 1. Inherited state, and the 02:30 outage (measured)

| thing | at slice start (02:14) | after the user's clean (02:30) |
|---|---|---|
| serves `:6070` `:6071` `:6092` `:6190` `:6191` | all `HTTP 200` | all `000` |
| hubs `:7501` / `:7611` | alive | both `000` |
| `🗑️generated/` | present | **wiped**, with every S9 capture taken before 02:30 |
| free disk | 22 GiB | 142 GiB |
| predecessor S9 work | none | — |

Brought back by this slice (pids recorded, nothing of anyone else's killed):

| what | how | pid | verified |
|---|---|---|---|
| hub `:7611` on `.🧬semio/🌐hub/gm1-boot` | HS1 §6's line, HS1's copied binary `⚡️cache/hs1/os-hub-7611` | holder `39735` | `GET /readyz` → **200** |
| serve `:6190` (`s` react dev → hub 7611) | `📜️c2-serve.sh s 6190 http://127.0.0.1:7611` — **serve only**, the staged `s-react-dev` tree survived the clean | holder `39936` | `GET /` → **200** |

No activation was needed: `.🧬semio/🦑️repo/⚡️cache/vite/os-dev/s-react-dev` (42 MB) was intact.

## 2. Home lists the user's studios — **root found and fixed; the guest named it**

### 2.1 What was measured before the fix (serve `:6190` → hub `7611`, `user1@semio.dev`)

`🐍️s9-home-diagnose.mjs` + `🐍️s9-home-rows-probe.mjs` (both new, permanent), captures
`🗑️generated/s9-diagnose-1.txt`, `s9-home-1.txt`:

```
bootstrap: []                       ← idle: receipt matched, ACK published, 0 faults, ~35 s
windows:   ["s-home-main"]
rows:      []
body:      "… Studios  Create Space  No studios yet. Create one from the navbar. …"

GET /directory/spaces  → [{"access":"author","space":{"id":"01a0c00f-4f3c-7834-a7e6-2ccf9de925db",
                           "name":"GM1 Shared Map 7161c795","kind":"studio","role":"author",
                           "memberCount":2,"documentCount":1,
                           "ownerUserId":"01a0c00d-cd33-7948-91cb-da23affa54ec"}}]
GET /auth/sessions/me  → {"userId":"01a0c00d-cd33-7948-91cb-da23affa54ec","displayName":"User 1"}
GET /directory/event-page/v1?after=0 → throughSeqInclusive 9, seq 4 = space.created(GM1 Shared Map)
```

So S8's §3 picture is reproduced exactly on a freshly booted pair.

**S8's two named candidates are both wrong, by reading:**

- *(b) `home_space_rows`' membership filter.* There is none.
  `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:544` iterates **every** `directory.spaces` entry and unions the
  local catalog; `user_id` is used only to decorate each row with `role: caller_role(...)`, which
  chooses which row-scoped affordances are offered — never whether the row exists.
- *(a) an instance mismatch.* `openDirectoryHomeOwnerV1` is called with
  `instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }`
  (`🏛️ShellHost/🟦️.tsx:3697-3712`), so the bootstrap owner IS the visible session's instance, and
  `beforeAcknowledge` refuses outright (`directory-bootstrap.visible-owner-stale`) if it ever is not.

### 2.2 The guest named the real one

The navbar `Create Space` button opens no dialog headlessly, so `🐍️s9-home-actions-probe.mjs` (new,
permanent) drives Home's own Actions rail instead — the same dispatch path `🐍️s6-all-kinds-sweep.mjs`
uses for every other kind. `createStudio` writes the **local** catalog and needs no directory at all,
so it separates "the projection never arrived" from "the render never reached the row builder".
Capture `🗑️generated/s9-home-actions-a.txt`:

```
STEP settled      bootstrap: []   rows: []   empty: true
STEP rail unfolded 26 rows incl. action.createStudio, action.createSpace, action.applyDirectoryEventPage
STEP click action.createStudio: ok
REFUSALS ["warning: input #1 createStudio refused: dispatch-failed (user window=s-home-main)
           — typed-operation failed: retained command reducer rejected operation:
             app.message s.home.session-identity-required"]
```

`s.home.session-identity-required` is raised by
`✏️s/…/🏠️home/…/✏️editor/🦀️.rs:31` (`require_session_identity`) out of
`home_retained_reduce`'s `context.view_state`. **The shell had been signed in for 90 s.** So the view
state the shell hands the guest carries no identity.

### 2.3 The root, at file:line

Three view-state projections cross to the guest, and only ONE stamped the human:

| projection | file:line | stamped `sessionIdentity`? | what it feeds |
|---|---|---|---|
| `resolvedTargetViewState` | `🏛️ShellHost/🟦️.tsx:4800-4811` | **yes** | browser-actor view state, `setAppRegistrations` |
| `runUiRefreshPass`'s `viewState` | `🏛️ShellHost/🟦️.tsx:5238-5254` | **no** | **every window body, panel and label of every app, every refresh** |
| `onAction`'s `baseDispatchViewState` | `🏛️ShellHost/🟦️.tsx:7359-7370` | **no** | **every rail row, panel row and keybound verb** |

Both of the hot two spread `session.viewState` and then re-derive `locale`, `terminology`,
`windowInstances`, `activeUtilityByWindowId`, `focusedWindowId` and the tree windows from live refs —
a fresh per-call projection which, by construction, carries only what it lists. `sessionIdentity` was
not listed, and `session.viewState` does not hold it either.

The comment at `🏛️ShellHost/🟦️.tsx:5607` states the opposite (`resolvedTargetViewState` stamps
`sessionIdentity` … and apps that need it refuse to assemble without it) and re-establishes the
session on a human change on the strength of it — the re-establishment happens, and then the
re-render still crosses identity-less.

And Home's render is explicitly written to treat `None` as a **state, not a fault** (S2 §3.3 made it
so, for the good reason that a signed-out landing window must still publish):

```rust
// ✏️s/…/🏠️home/…/🪟️windows/🏠️main/🦀️.rs:214-218
let rows = match crate::home_session_identity(view_state) {
    Some(identity) => resolve_ready(crate::home_space_rows(&directory, &identity.user_id)),
    None => Vec::new(),
};
```

so the whole chain S8 proved live — the page, the receipt, the config publication, the ACK — is
irrelevant to the table: the render never asks for a row. **Every other identity-reading surface of
the product is on the same defect**; Home is simply the one that shows it as a blank table instead of
a fault.

### 2.4 The fix (landed)

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`, both
projections, the same expression and the same source as `resolvedTargetViewState`:

```ts
sessionIdentity: identityRef.current
  ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName }
  : undefined,
```

`parseResolvedPluginViewState` (`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:971`) already admits
`sessionIdentity` and validates both fields, and `structuredClone`s it through — no wire change.

### 2.5 After (filling)

## 3. The sweep

### 3.1 S8 §4.1 is a misreading, and `#s-checkin` is the right oracle (measured, by reading + captures)

S8 concluded that `flow`, `sequence` and `procedural` "already complete the round trip and are lost to
the `#s-checkin` oracle", because their verb's ledger row carries a `↶`. That glyph is not an undone
marker — it is the History panel's **revert control**:

```tsx
// 🏛️ShellHost/🟦️.tsx:9783-9791
dimmed: entry.applied === false,
control: entry.revertible && !isViewer
  ? <button id={`framework.history.entry.${entry.seq}.revert`} …>↶</button>
  : undefined,
```

The sweep's `readShell` takes each row's whole `innerText`, so the button's glyph lands in the label
of every revertible row, undone or not. The undone marker is `dimmed`, which the sweep already reads
separately.

`#s-checkin`'s count is `uncommittedEditCount` (`🏛️ShellHost/🟦️.tsx:9542`): entries of the FOCUSED
program's projection with `kind === "mutation"` and `applied !== false`, reset by a checkpoint. Since
S7 that projection is per program. Re-reading S7's own five captures:

```
22 PASS kinds   edits [0,1,0,1]   ← mutate → undo → redo, every one of them
13 FAIL kinds   edits [0,0,0,0]   ← no uncommitted mutation entry ever appeared
flow            edits [0,0,0,0]   applied [1,3,6,9]   last "Add Widget↶"
```

`flow`'s `applied` grows because the ledger counts the SHELL's own chrome commands too
(`shell.windowActivate` → `Activate Window`, and the undo/redo presses themselves). Its "Add Widget"
row exists but is not a `mutation` entry, so the document did not move. **`#s-checkin` is not replaced
— it is confirmed**, and the 13 are 13.

### 3.2 `norm` — the refusal is the GUEST's, and 14 editors carry it (fixed)

The captured refusal is
`action 'setSnapshot' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand)`.
That sentence is the text of `ArtifactEditor::command_from_action`'s **trait default**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11960`), i.e. an app that never overrode the
`{action,args}` bridge. Census of the fifteen norm editors:

```
🧱️din4108  … fn command_from_action  (hand-written, B2c.1's proven one)
the other fourteen … none
```

So `norm-din16798` — and `din18599`, `en1990`…`en1999`, `iso16757`, `vdi3805` — have an Actions rail
whose every row is inert in **any** shell, however green their `--lib` tests are. This is the
live-only class `project-editor-runtime-dispatch-chain-live-only-faults` names.

Landed:

| file | change |
|---|---|
| `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` | new `norm_command_from_action!($command, $decode)` macro — the three verbs all fifteen declare, with din4108's proven body generalised |
| 14 × `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `semio_s_artifact_norm_contract::norm_command_from_action!(XCommand, …decode_x_snapshot_json);` |
| `🐍️s9-norm-command-bridge.py` (ticket) | the codemod: anchors on each editor's own `fn command_id`, **asserts the anchor matches exactly once per file** before touching it, idempotent, reports skips |

`14 bridged, 1 skipped`, diffstat `15 files changed, 72 insertions(+)` — 2 lines per editor, nothing else touched.
`cargo check -p semio-s-artifact-norm-din16798` → **Finished, 0 errors** (`🗑️generated/s9-norm-check.txt`).

### 3.3 The staged-form filler (landed in the permanent sweep probe)

S8 §4.2 measured `filled: []`, `submitted: "absent"` for every argument-carrying verb and concluded
the sweep "cannot fill a staged form". The cause is that `🐍️s6-all-kinds-sweep.mjs` looked for
`[data-slot="window-action-pane"] [id$=".arg.<key>"]:is(input,textarea)` and a native `<select>`, and
nothing else — while `🐍️b3a-interaction-probe.mjs`, which scores those same kinds PASS on their own
serves, drives five shapes. Ported into the sweep (`fillStagedArgument`, `submitStagedVerb`,
`clickUncovered`, `raiseHistory`):

| shape | why it was invisible |
|---|---|
| shadcn combobox `[role=combobox]` carrying the arg id | not an `input`/`select` at all |
| `input[type=range]` | Playwright's `fill` refuses it; React's own value setter + `input`/`change` events |
| Radix `[role=slider]` thumb | carries **no id**, so every id-keyed selector missed it — driven from the keyboard |
| bare-id / `name=` fallbacks outside the action pane | a spawned program's form is not always inside the host's pane |
| execute fallback chain | 🔋️energy's zones rail stages args fine and still answers `absent` to the exact-suffix `…execute` selector |
| covered rows | a docked panel over the rail makes `force: true` press the panel and still report `ok` (FL1/PB3) |

The verb and argument maps are now **baked into the probe** (`DEFAULT_VERBS`, `DEFAULT_ARGS`), with
`S6_VERBS`/`S6_ARGS` still overriding: S6's and S7's maps lived in a shell command and died with it,
which is why S8 had to reconstruct them. Two ids corrected against the artifacts' own manifests —
`wfc` `changeSeed` → **`change-seed`**, `procedural` `nodeGraphEdit` → **`generate`**.

**Measured, first chunk on `:6071` before the 02:30 clean** — the filler works and moves the
diagnosis one layer down:

```
energy  knownVerb=rename-zone OFFERED, clicked ok, submitted ok
        refusal: "input #11 rename-zone refused: dispatch-failed (user window=energy-3::energy.model.3d)
          — typed-operation failed: retained command reducer rejected operation:
            mutation.target-missing the energy model has no zone with id 1"
```

Under S8 the same row recorded `filled: []`, `submitted: "absent"` and no refusal at all. So the form
now stages and the verb now reaches the guest; what fails is the ARGUMENT VALUE — `zone=1` is b3d's
value for b3d's seeded document, and the studio-spawned `energy` instance holds a different one. That
is a third shape, distinct from both of S8's: **the sweep's staged values are document-specific and
must be read off the live document, not copied from a single-plugin probe's config.**

### 3.4 The sweep number — **not re-measured** (honest)

The chunk above was killed mid-run by the 02:30 clean (its JSON was never written; the quoted rows
are from its live log). No 35-kind re-sweep was run by this slice, so the number stands at S7's
**22/35** and the ≥ 30/35 target is **not reached**. What this slice leaves for it is: the filler and
the corrected verb map in the permanent probe, the `norm` bridge for 14 editors, the corrected
attribution of §3.1 (13, not 10), and the third shape named in §3.3.

## 4. `playbook-module-procedural` — **S10 fills**

S9 left this untouched. S10 root-fixed it; full detail in
`📓️s10-s-host-studios-and-sweep.md` §3. In brief: `Migrated` could not simply be declared, because
the registry refuses a `Migrated` classification with no owned tool factory behind it
(`interactive-job.owner-classification`) and refuses an `Artifact`-lane tool with no publication
authority (`interactive-job.publication-authority-missing`) — which is what the app's own comment
already said. So S10 wrote the factory: `ModuleRetainedCommandJobFactory` (plain-`ArtifactApp`
owner, the `🪐️space` studio shape rather than the `EditorApp<_>` one), both verbs contracted on the
`Artifact` lane, `bounded_first_step_tool_proofs!`, the framework's generic
`bounded_config_store_one_item_preparation_factory` as the artifact-lane authority, and
`.action_interactive_job(…, Migrated)`. `cargo check -p semio-s-plugin-playbook-procedural`: 0
errors. Runtime proof is behind one wasm re-activation and is recorded in S10 §3.

## 5. Honest gaps — **S10 fills**, from S10's measurements

S9's own three gaps, re-stated with what S10 measured against each:

1. **S9 §2.5 "After" was never filled.** S10 filled it: the `sessionIdentity` fix **works** — on a
   signed-in shell Home renders `Demo Studio  atelier  private  1  0  local  open` instead of
   `"No studios yet"`, and the table survives a full page reload (S10 §1.1, §1.4). S9's root fix is
   confirmed at runtime for the first time.
2. **S9 §3.4's sweep number stood at S7's 22/35 and was not re-measured.** Still not S9's number to
   claim; S10 re-measured the harness and the four decisive kinds (S10 §2.4) and owns the full
   re-sweep.
3. **S9 §3.1's attribution of `flow`/`sequence`/`procedural` was right, and its cause was one root,
   not three.** S10 found it: all three verbs publish on the `Child` or `Config` lane, and
   `build_history_view`'s `applied` asked only the parent document store, so all three reported
   `applied: false` — which the host reads as undone (S10 §4). S9's reading that `#s-checkin` is the
   correct oracle is confirmed; what was wrong was the field it reads, not the oracle.
4. **S9's `norm` bridge (§3.2) is compiled, not run.** 14 editors gained
   `norm_command_from_action!` and `cargo check -p semio-s-artifact-norm-din16798` was green; no
   `setSnapshot` has been observed landing inside `s`. S10 owns that proof (S10 §5).
5. **A defect S9 could not have seen, because it appeared after S9 ended**: an unguarded
   `historyEntryLabelText` read unmounts the entire shell whenever any ledger row's label lacks the
   terminology axis. S10 root-fixed it (S10 §2.1). It is named here because it invalidates any
   sweep number taken between ~10:35 and 11:15 on 2026-09-21.

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `sessionIdentity` stamped into `runUiRefreshPass`'s render view state and `onAction`'s `baseDispatchViewState` (§2.3, §2.4) |
| `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` | new `norm_command_from_action!` macro (§3.2) |
| 14 × norm artifact editors | invoke it (§3.2) |
| `🐍️s6-all-kinds-sweep.mjs` (ticket, permanent) | staged-form filler, execute fallback chain, covered-row recovery, non-toggling History raise, baked-in verb/arg maps with `wfc`/`procedural` corrected (§3.3) |

Ticket folder (not product code): `🐍️s9-home-rows-probe.mjs`, `🐍️s9-home-diagnose.mjs`,
`🐍️s9-create-space-probe.mjs`, `🐍️s9-home-actions-probe.mjs`, `🐍️s9-norm-command-bridge.py`,
`📜️s9-restage.sh`.

**S10 fills — corrections to this table.** The `🐍️s6-all-kinds-sweep.mjs` row above is accurate for
what S9 landed, but two of that probe's remaining defects made every S9/S10 row unreachable and are
fixed by S10, not S9: `signIn`'s `page.goBack()` and `enterStudio`'s unfiltered palette look
(S10 §2.2). S9's staged-form filler itself is confirmed working and is what exposed the third shape
S10 then implemented (S10 §2.3). No S9 product change was reverted or edited by S10.
