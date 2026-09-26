# S13 — os `s` Frontend Audit (Outcome 1: working `s`, all plugins/artifacts)

Read-only auditor A13-os, session 13. No edits/builds/servers started by me (one `curl` to an already-running hub, `ps`/`lsof`
read-only, per the rules). 2026-09-26, ~19:10–19:5x. Session 13 opened 19:00; by audit time the landing window
(`📓️landing.md` §"Session 13 Landing Window") is still an **empty table** — nothing has compile-atomic landed yet — and
`wp-la/`, `wp-lb/` are empty scaffolds, `wp-lc/` holds only U5's un-applied preference-lane patch, `wp-w3/` holds only an
import-scan script. This is a snapshot at the very start of session 13, before any of its own work is visible.

Sources read in full: `AGENTS.md`, `📓️session-13-preamble.md`, `📓️audit-s12-os-frontend.md`, the `## Session 12` sections of
`📓️wp-s15.md`, `📓️wp-u5.md`, `📓️wp-f1.md`, `📓️wp-t12.md`, `📓️wp-r8.md`, `📓️wp-w2.md` (full, start to end of file), plus
targeted reads of `📓️wp-c10.md` and `📓️wp-h9.md` (the two P0 owners from the last audit) and one line of `📓️wp-g10.md`
(the one still-open P1 from two audits back). Verified live against the running system: `curl 127.0.0.1:7800/readyz`,
`lsof -iTCP -sTCP:LISTEN` over every session-13 port range, `ps -axo pid,ppid,command` for live cargo/nx, and
`/usr/bin/grep` for `unimplemented!()`/`todo!()` across `✏️s/` and `🧰️framework/`.

## 0. Headline — 4 P0

1. **Only 9 of 34 plugins (16 of ~89 open-creatable kinds) are live on any hub right now; `--packages all` has failed
   four times** (06:34 exact-pin, 11:43 agent-cut, 15:54 disk-guard, **18:45 imperative wasm codegen — the current
   blocker**). Outcome 1 literally reads "all plugins and artifacts"; this is not met at the infrastructure level.
2. **Hub 7800 — confirmed `status:"ready"` by my own `curl` just now — is still running the 02:31 binary**, i.e. none
   of session 12's streaming-execution-target fixes (S12-3e/S12-1b/S12-3h) are live on it. The 503/stuck-creation
   defects those fixes targeted are still reproducible on the hub a dev would actually open today.
3. **No live re-verification of the editor/viewer/en+de matrix exists against the tree that is about to be published.**
   S15's confirmed "75/75 en, 75/75 de, 70/70 viewers" ran against the *session-11* restage4 (09-25 18:56). The tree
   then moved through a **second** restage2→restage3→restage4 cycle inside session 12 itself (09-26, finishing
   ~18:1x+, `📓️wp-w2.md` lines 446–465) because peers' shared-crate edits invalidated the first build. No `s` serve is
   running right now on any dedicated audit port (confirmed by `lsof`) to re-run it. This is the same "matrix is stale
   against the currently staged tree" gap the session-11 and session-12 audits each already flagged once — it has now
   recurred a third time, for a different tree, at the point where session 13 is about to publish.
4. **Collaborative undo/redo over a hub document is unproven end to end.** C10's collab-e2e sits at 4/14 steps; writer
   typing over the hub is blocked by "the guest store re-announces an accepted operation on the next keystroke → hub
   replay conflict" (`📓️wp-c10.md` "Found 09:4x", routed to T12, frozen). Writer is one of the 9 plugins that **is**
   live on 7800 today, so this directly blocks the "edited + undo + redone as a hub document" bar for a plugin that
   otherwise looks done.

Full list, with the closed/still-open comparison against `📓️audit-s12-os-frontend.md`, in §§2–6.

---

## 1. Live infrastructure snapshot (measured just now, read-only)

| what | result |
|---|---|
| `curl -s 127.0.0.1:7800/readyz` | `status:"ready"`, `runId 8d0ec7a5…`, `openPlan/openPlanExchange/rebootstrap/mcpWorkspace` true, gis inference route present — matches the preamble's "catalog B2, binary from 02:31, hold 28673/hub 54029" |
| `lsof -iTCP -sTCP:LISTEN` over 6500–6659 / 8000–8159 | **only** 6523, 6524 (unidentified bun), 6552/6553 (WG7 lineage), 6590 (WG8), 8050 (WG9/WG10's hub, pid 79010) — **no serve on any of S15/F1/U5/T12's own ports (6540s/6560s/6580s/6620s)**, i.e. nobody has a live `s` React session open right now |
| `ps` for cargo/rustc | one live `trusted-catalog-bootstrap --packages stdio,gis,note,writer,draw,puzzle` (pid 7797→13179, `cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2 --profile wasm-release`, 9 rustc children, started ~14 min before this check) — a **6-package** bootstrap distinct from B2's 9 or the failed all-34 publish; owner not identified from the files I read this pass — **UNVERIFIED whose build this is** |
| `📓️landing.md` §"Session 13 Landing Window" | table header only, zero rows — nothing landed yet |
| `.vscode/launch.json` / `🧩️launch.seed.jsonc` | 456 / 300 `"name"` entries; `🔁️rebuild-all🔌️plugin-registry` present in both (AGENTS.md launch-row rule met for this command) |

---

## 2. Session-12 audit items — closed vs still open (evidence-checked this pass)

### Closed since `audit-s12-os-frontend.md`

| s12 item | now | evidence |
|---|---|---|
| P0-2 "re-run the full matrix on restage4" | **measured PASS against the session-11 tree** (en 75/75, de 75/75, viewers 70/70, 0 retries) — but see Headline §0.3: the tree moved again after this run, so it does not cover the tree session 13 will publish | `📓️wp-s15.md` "03:4x … confirmation matrix finished at 00:16 … en 75/75, de 75/75, viewers 70/70, every row first pass" |
| P1-6 (carried from s11) architect `setAdjacencyKind` duplicate-capability-id | **CLOSED, measured**: `capabilities_search` returns the capability once, 0 duplicates in 50 hits | `📓️wp-g10.md:25,38`, `g10-adjacency-probe.py` → `s12-g10-logs/adjacency-probe-1.json` |
| P2-11 approval affordance not cleared after cancel | **CLOSED, live-measured** (not just "fixed in source" as U5 first reported): gateway sends `ApprovalWithdrawn`, live-agent-loop row reads "Withdrawn — the agent's request was cancelled", en+de, no countdown/decisions leftover | `📓️wp-g10.md:38` (S5 entry) |
| P2-13 layout persistence unknown | **CLOSED, measured**: dock panels + tab, saved named layouts, appearance, language, keybinding overrides all survive a reload | `📓️wp-u5.md` §12-4 |
| P2-14 no live spawned-job row in Tasks | **CLOSED, measured**: remodel reconstruction spawned job, live progress, Suspend/Resume/Cancel, en+de, two root fixes landed | `📓️wp-u5.md` §12-2 |
| P2-15 puzzle en/de rows + raster "Hover" not confirmed live | **CLOSED, measured**: puzzle 11/11 History rows switch en↔de live; raster ships "Darüberfahren" in the staged descriptor | `📓️wp-u5.md` §12-3 |
| P1-10 (numbering per s12 audit) Home ≥ 9 spaces / ≥30 live proof | **CLOSED, measured**: 36–38 hub rows on 7800, 0 faults, keyboard + de | `📓️wp-u5.md` §12-1 |
| (s15 census) gis viewer `setCamera`, procedural/generation2d viewer, vcs viewer | **CLOSED, measured** in the same S15 matrix run above (same staleness caveat as P0-2) | `📓️wp-s15.md` §12-1c log |
| stdio 9 editors, demonstrator/sourcing curation, trinity jack/rewriting, norm din18599/en1990 | **CLOSED, measured** in the same S15 matrix run (same staleness caveat) | ditto |
| architect/energy "no open target" (missing `.artifact_kind()` stitch) | **CLOSED, landed+checked**: both now answer one open target each | `📓️wp-w2.md` "03:50 … two more open-target root fixes … architect and energy" |

### Still open (unchanged or only partially advanced)

| s12 item | status now |
|---|---|
| P1-5 stdio has no hub open target by design | **still open, unowned**; T12 confirmed the root cause (plugin-level `stdio.<x>` vs dialect `s.stdio.<x>`) and a rewrite patch is **prepared** (`wp-t12/postpublish-open-kinds.py`, dry run 79 files clean) but explicitly deferred to the post-publish landing window — not landed |
| H9 item L, hub kind-picker labels every kind "Editor"/"Editor" | **still open** — "PATCH SET READY … post-publish landing" (`📓️wp-h9.md:32`) |
| "a plugin absent from the local registry cannot be installed from the hub" | **still open, unowned** — U5 explicitly declined it this session ("unowned and large … not started", `📓️wp-u5.md` §12-5 row 7) |
| trinity rewriting's window vocabulary unregistered | **still open** — confirmed needs the rename slice (`📓️wp-t12.md:39`, §S12-4), not scheduled to any session-13 slice yet |
| cad object kinds have no wire vector (pane-child materialization) | **still open, explicitly deferred** past the freeze (unchanged from s12) |
| native wgpu/winit accessibility (WG7 backlog) | **UNVERIFIED this pass** — I did not re-read `wp-wg7.md`; nothing in the six required reports references it being picked up |
| Home viewer lists 0 hub rows (read-only projection feed missing) | **still open** — found by U5, explicitly routed as "after the guest freeze", not started |

### New P0/P1 this session, not in the s12 audit (see §0 and §6)

- Publish-all still failing (now on imperative wasm codegen, a different blocker each of the 4 attempts).
- Hub 7800 running a stale binary despite being "ready".
- The matrix's second staleness (against the session-12-tail restage4).
- Writer's guest re-announce → hub replay conflict blocking hub-collaborative undo/redo.
- See §4 for gaps found by reading code this pass (dead UI paths, external-runtime-dependency violations, a persisted-id
  correctness gap).

---

## 3. Plugin/kind census update against outcome 1's four bars (a–d)

Full per-plugin table is `📓️audit-s12-os-frontend.md` §1 (60 rows); this section states only what changed and what the
four bars look like **today**, live-hub-first (per my own `curl`/`lsof` above):

- **(a) opened locally** — 60/60 staged (receipt unchanged since 18:15:24 activation window); S15's matrix says 75/75
  editors + 70/70 viewers PASS, but that run predates the tree's second restage (§0.3) — treat as **plausible, not
  proven**, exactly the wording the last two audits used for the *previous* tree move.
- **(b) created + opened as a hub document** — **16 kinds across 9 plugins** (stdio, gis, note, animate, block, writer,
  draw, puzzle, wfc) are open-creatable on any live hub; the other 25 plugins have zero live hub presence (queued behind
  the failing all-package publish). Of those 16, S15's most recent live sweep on the *actual running* 7800 binary
  (`b4`, `📓️wp-s15.md` §S12-3) got **9/16 in German** (block2d/wfc2d/grid2d block3d/wfc3d block5d animate gis note
  writer) — the other 7 rows fail with `execution-target/component 503` or a creation that never resolves, **because
  the fix for exactly that (S12-3e) is not on the 7800 binary right now** (§0.2). This is directly re-checkable by
  anyone: it needs no new work, only the hub restart session 13 is already planning.
- **(c) edited + undo + redone as a hub document** — same 9–10/16 rows as (b) (each ran verb→undo→redo `[0,1,0,1]`).
  Writer's own hub-document edit/undo/redo is not separately proven under sustained typing — the local (non-hub) 1379-
  keystroke proof (`📓️wp-f1.md` "F1-6", `f1-typing-run-live-writer-7.json`) is local-only (`serve 6620`, no hub), and
  the hub path is the one blocked by the re-announce/replay-conflict defect in §0.4.
- **(d) exported/imported** — **no session-12 slice measured this axis directly against a running `s` session**; the
  closest evidence is T12's parity/oracle work, which tests round-trip fidelity offline against the plugin crates, not
  through the `s` UI's own export/import commands. It found real, still-open defects: brep vertex-count drift, a kit
  pack-twin mismatch, a drawing-reference rename-codemod injury, and — most consequential — **661 committed files
  across 6 plugins mint persisted ids with `std::collections::hash_map::DefaultHasher`, whose algorithm Rust leaves
  unspecified across releases**, so a toolchain bump can silently re-key every one of them on export/reimport
  (`📓️wp-t12.md` "09:2x … Finding F9", `en1990_qk_scene_id` and 17 sibling sites). All of these are **prepared patches,
  not landed** (frozen until post-publish). Separately, T12's command-reachability census found **architect's own
  `export`/`import`/`search`/`Report`/`Validation` commands are unreachable in the running app** (see §4) — a direct,
  concrete "no create/export path in `s`" gap for one live-hub-adjacent plugin that nothing in session 11 or 12 flagged.

---

## 4. New gaps found by reading code this pass

- **`unimplemented!()`/`todo!()` census (clean).** `/usr/bin/grep -rn "unimplemented!\|todo!()" ✏️s 🧰️framework --include="*.rs"`
  finds exactly 2 hits, both inside test files (`🧰️framework/🔨️modules/🔄️machine/🧪️tests/🔬️host-unit/🦀️.rs`,
  `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️element-unit/🦀️.rs`) — consistent with the audit-s11 finding of "one
  test-only `unimplemented!()`, documented, non-blocking." **No new one found in production shell/plugin UI code.**
- **Dead UI paths (new finding).** T12's reachability census: **42 commands are `BatchOnlyPendingRewrite`**, i.e.
  registered but unreachable from any control in the running app — space studio 24, **architect 8 including
  `runAnalysis`/`Report`/`Validation`/`search`/`import`/`export`**, cad 5, home 4, animate 1
  (`📓️wp-t12.md` "01:2x … Found on the way", no file:line given by T12 beyond the command names). This is a genuine,
  previously-unflagged "no create/export path in `s`" gap for architect specifically, and a smaller one for cad/home/
  animate/space-studio. **Owner: unowned — recommend T13 (plugin-contract debt) or S16 (frontend reachability).**
- **AGENTS.md "no runtime dependency on external libraries" — two live violations found by T12, not by me
  independently, but worth restating as compliance findings:**
  - `✏️editor/🦀️.rs:108` (reasoning's wires editor) builds interaction args with `serde_json` **in production**
    (`📓️wp-t12.md` "01:2x").
  - `🎯️action-bus`'s `optional_json_to_dsl` takes a `serde_json::Value` parameter; the dependency registry
    (`🔒️dependencies.json`) classifies this as production-reachable across 211 manifests, recorded by R8 as
    `productionDebt`, not yet fixed (`📓️wp-t12.md` "03:4x").
  Sequence's own production `serde_json` use for the same reason was found and root-fixed this session
  (`📓️wp-t12.md` "00:3x", moved to `[dev-dependencies]`) — proving the pattern is fixable in-place; the two above are
  not yet. **Owner: unowned — recommend T13.**
- **Localization** — no new hard-coded-English site found this pass beyond the already-tracked H9 kind-picker label bug
  (§2). U5's static sweep (2801 sites, 0 identical en=de, 0 English leaks) stands; I did not re-run it.
- **Accessibility / mobile / tablet / customization** — nothing new found; U5's session-12 measurements (WCAG AA
  18/18, 29 keyboard tab stops, phone/tablet no horizontal scroll, dark+language+keybindings+layout persist across
  reload) are the current state and I found no code-level regression risk in what I read. The one still-open item is
  cross-**device** (not cross-reload) sync of preferences: U5's per-user preference lane is TS-landed + lawed but its
  Rust/directory-schema half is a **prepared, unapplied** patch sitting in `wp-lc/u5-preference-lane-apply.orig.py`
  (confirmed present on disk) — i.e. session 13's LC slice is already positioned to land exactly this, but has not yet.

---

## 5. AGENTS.md compliance of the shell

- **Launch rows**: spot-checked `rebuild-all` (the one new registered chain command from session 12) — present in both
  `.vscode/launch.json` (456 entries) and `🧩️launch.seed.jsonc` (300 entries). No broken rule found for this command.
  I did not exhaustively diff every runnable script against launch.json this pass (time-boxed); flag as spot-checked
  only, not exhaustive.
- **No legacy/compat/shims/deprecations**: nothing in the six required reports describes adding one; W2's version-pin
  work explicitly avoids a compatibility layer ("the SDK-type narrowing … is NOT landed now … prepared for the
  post-publish landing window", i.e. deferred cleanly rather than shimmed). No violation found.
- **External runtime dependencies**: two open violations found (§4, wires editor + action-bus), one fixed this session
  (sequence). This is the one AGENTS.md rule with concrete, named, still-open breaches.
- **`📜️script.ts`-only permanent scripts**: not independently re-audited this pass; no report flagged a new stray
  script file. UNVERIFIED beyond what session 12 already checked.
- **Progress + cancellation for expensive operations**: measured largely DONE by U5/S15 (TaskManager, plugin-install
  progress+cancel, execution-target retry bands, command-stall watch) — unchanged good state, no regression found.

---

## 6. Ranked gap list — P0/P1/P2

Owner column names the session-13 slice per the fleet's naming convention (successor slices: S15→S16, C10→C11,
H9→H10/DB1, T12→T13, F1→F2, W2→W3) where the report itself does not already state an owner.

### P0 — blocks "working `s` with all plugins/artifacts" outright

1. **`--packages all` still fails; 25/34 plugins have zero live-hub presence.** Current blocker: imperative wasm
   codegen, `browser actor artifact: unsupported import interface` at
   `🔌️plugin/🌐️browser-bundle/📜️script.ts:317` (`📓️session-13-preamble.md:21`). **Owner: W3.**
2. **Hub 7800 is "ready" but on the stale 02:31 binary** (confirmed live by my own `curl`); every fix landed for
   execution-target 503s/stuck creations (S12-3e/1b/3h) is absent from the hub a user would open right now.
   **Owner: W3** (the restart is already inside its planned sequence — verify it actually happens before declaring
   outcome 1 met).
3. **The editor/viewer/en+de matrix has not been re-run against the tree about to be published** (§0.3) — the
   "75/75/70/70" number is real but for a superseded tree; no serve is currently up to re-check it.
   **Owner: S16.**
4. **Hub-collaborative undo/redo is unproven and, for writer, actively broken** by a guest re-announce → replay
   conflict (`📓️wp-c10.md` "Found 09:4x"). Blocks the "edited + undo + redone as a hub document" bar for a plugin
   that is otherwise live on 7800 today. **Owner: C11 (collab proof) / T13 (guest fix, currently frozen).**

### P1

5. stdio has no hub open target by design (26 kinds, 9 editors) — patch prepared, not landed. **Owner: T13 (lands
   post-publish, already scoped in `wp-t12/postpublish-open-kinds.py`).**
6. Hub creation-catalog kind picker labels every kind "Editor"/"Editor" — patch ready, not landed. **Owner: H10/DB1
   (H9's item L, handed off).**
7. A plugin absent from the local registry cannot be installed from the hub at all — unowned, not started.
   **Owner: NEW (nobody has taken it across three sessions).**
8. **Architect's own `runAnalysis`/`Report`/`Validation`/`search`/`import`/`export` commands are unreachable in the
   running app** (42-command `BatchOnlyPendingRewrite` census, §4) — a direct hole in outcome 1's "exported/imported"
   bar for a live plugin. **Owner: T13 or S16 (NEW finding, unowned).**
9. **661 committed files mint persisted ids via an unspecified-algorithm hasher** (`DefaultHasher`), a latent
   export/reimport correctness risk across a toolchain bump (§3/§4, T12 Finding F9). **Owner: T13 (rename-slice
   class, per T12's own routing).**
10. Two production `serde_json` dependencies still violate AGENTS.md's no-external-runtime-dependency rule (wires
    editor `✏️editor/🦀️.rs:108`, `🎯️action-bus::optional_json_to_dsl`). **Owner: T13 (NEW, unowned before this pass).**
11. trinity rewriting's window vocabulary unregistered, cad's object kinds have no wire vector — both carried,
    explicitly deferred, still unscheduled to any session-13 slice. **Owner: parked (rename slice).**
12. Per-user preference cross-device sync — TS half done, Rust/directory half prepared and sitting in `wp-lc/`
    unapplied. **Owner: LC (already positioned; land it).**
13. flow_core wasm-pack bindings stale since 09-25 01:53 (`📓️wp-w2.md` request-triage row "r8"), needs a rebuild
    inside the consolidated pass. **Owner: W3.**
14. R8's two prepared-but-unlanded correctness/perf patch sets (ui-contract item-metered retirement fix; Nx narrowed
    inputs that cut materialize-dev's hashed-file median from 73k to 13k) — both dry-run clean, both waiting on the
    same post-publish landing window. **Owner: W3/S16 landing.**

### P2

15. Native (non-browser) wgpu/winit accessibility — status unverified this pass, presumed still unowned (WG7 backlog,
    not re-checked). **Owner: unowned, UNVERIFIED.**
16. Home viewer lists 0 hub rows (read-only projection feed missing). **Owner: unowned, routed "after the guest freeze".**
17. `/directory/spaces` and `/directory/events` answer a forged bearer as anonymous (200, public view) instead of 401
    — H9 flagged as an open design question for H10, not a defect fix. **Owner: H10/DB1.**

---

## 7. Explicitly UNVERIFIED this pass

- Which build the live `trusted-catalog-bootstrap --packages stdio,gis,note,writer,draw,puzzle` (pid 7797) belongs to
  (§1) — I did not trace it to a specific slice's log.
- Whether the session-12-tail restage2→restage3→restage4 cycle (`📓️wp-w2.md` ending at line 465, "18:1x restage4:
  describe-all again …") actually finished `verify rc=0, 60/60` before session 13 opened — the report's log ends
  mid-step.
- WG7/WG8's session-13 movement on native accessibility — not re-read this pass.
- Export/import as a UI-driven axis for any plugin other than architect's dead commands — no slice measures this
  directly; my finding is limited to what T12's offline oracle/parity work and reachability census surfaced.
- Exhaustive `launch.json` coverage of every runnable command, and an exhaustive scan for legacy/compat code — both
  spot-checked only, per §5.
