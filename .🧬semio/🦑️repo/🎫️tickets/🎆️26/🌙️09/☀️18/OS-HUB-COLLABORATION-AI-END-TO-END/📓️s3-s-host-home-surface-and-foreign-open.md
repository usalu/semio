# S3 — `s` host Home surface publication, and foreign-kind open inside `s`

Slice S3, fleet 5, started 2026-09-20 ~09:20. Scope handed over from S2 (`📓️s2-cold-s-boot-and-foreign-kind-open.md`
§3.4) and C1c (`📓️c1-collaboration-e2e.md` → `### E2E run 5 (session 5b)`):

1. **the one narrow question**: `s-home-main` / `.semio-table-host` is absent before AND after sign-in —
   refusal gone, tree live, no pageerror, body still not published. Find it by measurement, root-fix it.
2. update C1c's `🐍️c1c-s-host-probe.mjs` check 1 to the NEW contract and run to all-PASS on 6071.
3. run S2's `🐍️s2-s-host-foreign-kind-probe.mjs` signed-in across plugin kinds: open → lazy install →
   one real mutation + undo + redo per plugin, cheapest first, in a table.
4. make that probe an nx target next to `cold-boot-check-s` with a launch row.

Status legend: **measured** = this slice ran it and captured output; **unverified** = read from source only.

> **RESOLVED — `s-home-main` now publishes, before and after sign-in** (`c1c-s-host: all checks
> passed`, §4.2). Two faults, both found by measurement and both fixed: a guest-side
> `DuplicateSiblingKey` that made the UI document refuse the whole Home body (§3), and a shell-side
> `/hub` route that tore the host app's canvas down the moment anyone signed in (§4.1). Foreign-kind
> open is NOT proven: `spawn.<plugin>` needs a studio window, and this session had no space (§5).
>
> **INTERIM FINDING (13:58, measured).** `s-home-main` is not published because the guest's own UI
> document REFUSES the tree: `ui.surface-render: 1:s-home-main [producer]: DuplicateSiblingKey
> parent=#0 key=#0` — read verbatim out of the running shell on 6071 (§2). The window body's root
> stack holds `["#0", "#1", "s-home-create-space", "#0"]`: `Buildable::try_build()` stamps the
> ROOT-position key `#0` on the empty-catalog message, while `try_child` only fills keys that are
> still empty, so it collides with the first dead-line spacer (§3.1). Nothing about identity,
> `refreshUi` or re-establishing was ever the cause. Fixed by keying every child of that stack
> explicitly; pinned by two new laws; **90 passed / 2 pre-existing failures** (§3.3). Not yet observed
> live: the fixed wasm has not been staged — three build attempts were lost to the shared build-dir
> lock (§3.4).

## 0. tl;dr

**The Home body was refused by the UI document itself, not by anything about identity.** Measured live
on 6071 at 09:22 with a new diagnostic probe (`🐍️s3-home-surface-diagnose.mjs`, capture
`🗑️generated/s3-home-diagnose.txt`) — the shell's own body text carries the answer verbatim:

```
[DEBUG] PluginRuntime: actor space#1 stopped without publishing requested UI surfaces
  (missing=["1:s-home-main"], status=idle,
   faults=["ui.surface-render: 1:s-home-main [producer]: DuplicateSiblingKey parent=#0 key=#0"])
```

So the chain is: `HomeApp::render` DOES build a tree → the reactor's `ComponentTreeProducer` REFUSES it
with `DuplicateSiblingKey` → no patch is published → the settle throws "stopped without publishing" →
S2's `reportRefreshFault` funnel turns it into a window fault box. Every earlier reading
("identity", "refreshUi vs re-establish", "the actor stopped") was a symptom of this one refusal.

## 1. Inherited state

- serves alive and reused, never restarted: 6070 pid 26173 (S2's, no hub env), 6071 pid 33969 (C1c's,
  `S_HUB_URL=http://127.0.0.1:7501`); hub 7501 `/healthz` 200. Measured at 09:20 and again at 11:15.
- C1c's probe capture at takeover (`🗑️generated/c1c-s-host.txt`): checks 1-badge, 2-signed-in,
  2-no-fault, 2-command-palette PASS; **1-"Home refuses" FAIL (inverted by S2's fix, by design)** and
  **2-"host app surface is published" FAIL**.
- The staged tree is behind its sources for 22 plugins (`[stale] … source-newer` in the serve's own
  freshness pass) — reported, non-fatal, and NOT re-activated by this slice.

## 2. The trace — guest render → surface publication → host admission → React → DOM

Probe `🐍️s3-home-surface-diagnose.mjs` (new, permanent): headless chromium `--use-angle=metal`,
1600×1000 (a real viewport — a 0×0 pane throttles plugin boot), collects every console line and
`pageerror`, the shell's own `window.__semioOsCatalogProbe` (install status + `routerFault` per plugin),
the DOM ids and `data-slot`s that DID render, and the body text.

Measured signed out on 6071, 45 s settle:

| fact | value |
|---|---|
| plugins installed | 60, **0 failed**, **0 `routerFault`** |
| spawnable programs | 148 |
| `s-home-main` in the DOM | **false** |
| `.semio-table-host` | **0** |
| `#s-home-create-space` | **false** |
| shell ids that DID render | `framework.panel.artifact`, `framework.chat`, `framework.category.display`, `s-sync-status`, `s-presence-peers`, `framework.hub.signIn`, `framework.settings`, `framework.marketplace`, `framework.panel.history`, `framework.category.command` |
| `pageerror` | **none** (S2's funnel holds) |

That row of ids is what makes the trace conclusive: the window chrome, the panel rail, the presence
and sync chips and the whole navbar are live — the shell is healthy and the ONLY thing missing is the
one surface the guest refused to hand over. The fault text quoted in §0 rides in the shell's own
window-fault box, i.e. the host admission leg is working exactly as S2 built it; the refusal is
upstream of it, inside the guest's own tree producer.

## 3. Root cause and fix

### 3.1 The mechanism, named exactly

Pinned in a unit test before any fix was written, so the cause is measured and not inferred. New law
`the_signed_out_window_body_survives_the_component_tree_producer` (+ a `first_duplicate_sibling` walker
that names the offender) reproduced it off the browser:

```
left: Some(("#0", ["#0", "#1", "s-home-create-space", "#0"]))
```

The window body's root stack has four children and **two of them are keyed `#0`**. Why:

- `Buildable::try_build()` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:849-853`) stamps
  `positional_key(0)` = `#0` on any node whose author set no id. That is the **root-position** default.
- `HasChildren::try_child` (`:737-747`) only fills a key that is still **empty**. A child that already
  carries `#0` is left alone.

So a `try_build()`-ed node used as a CHILD arrives pre-keyed `#0` and collides with whichever sibling
the parent numbered `#0`. In `render_rows_wrapped` the first dead-line spacer is a raw
`BuiltNode::empty_separator()` (empty key → positioned `#0`) and the empty-catalog message came from
`built_text_node` → `text(…).try_build()` (→ `#0`). The UI document refuses the tree with
`DuplicateSiblingKey`, so nothing is published.

**This is why only the signed-OUT paint was broken.** With rows, the body is
`TableWindowKit::render_rows`, whose root carries the explicit id `TableWindowKit::KIND_ID`
(`🔌️plugin/🦀️.rs:31460`) — no collision. The sibling law
`the_signed_in_window_body_survives_the_component_tree_producer` passed from the first run and still does.
It also explains the whole history: the surface has *never* published signed out, and until S2 removed
the `s.home.session-identity-required` refusal the guest stopped before reaching this tree at all, so the
collision was masked. Nothing about identity, `refreshUi` or re-establishing was ever the cause.

### 3.2 The fix

`✏️s/🔌️plugins/🪐️space/…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs` — **every child of the
window body's stack now carries an explicit key**, which is the rule the table kit already follows:

| child | before | after |
|---|---|---|
| dead-line spacer A | positional `#0` | `s-home-dead-line-a` |
| dead-line spacer B | positional `#1` | `s-home-dead-line-b` |
| create button | `s-home-create-space` | unchanged |
| body (empty) | `#0` from `try_build` | `s-home-empty` |
| body (rows) | `framework.window.table` | unchanged (never overwritten — it is the table's addressing key) |

The empty branch is now built through the contract builder with `try_id(S_HOME_EMPTY)` instead of
`built_text_node`, and `window_content_dead_line_spacer` takes its key. The `render_rows_wrapped` doc
states the trap so the next author does not reintroduce it. The **viewer twin was checked and left
alone**: its `render_rows` result IS the body root, so a root-position `#0` is correct there.

### 3.3 Two stale laws fixed in passing, and one honest non-fix

- `home_render_requires_current_host_identity_and_changes_roles_with_it` still asserted S2's removed
  refusal (`unwrap_err() == "s.home.session-identity-required"`). Rewritten as
  `home_render_publishes_signed_out_and_changes_roles_with_the_identity`: the signed-out render must
  **project** (not merely build) and must carry the create button but no `manageSpace`.
- That test's second half had **never executed** (it sat behind the failing `unwrap_err`), and it was
  wrong: `config_with_one_folded_space` folds only `space.created`, which leaves
  `DirectorySpace.members` empty (`📇️directory/🦀️.rs:98-118`), so `caller_role` answered `None` for
  everyone and `manageSpace` could not appear for any user. The fixture now folds a
  `member.upserted` event making `u1` the author, so the role law has something real to distinguish.
- **Not fixed, and deliberately**: the framework-wide trap itself. `try_build()` meaning "root
  position" while `try_child` only fills empty keys is a live foot-gun for every plugin author who
  composes already-built children. Changing it re-keys reconciliation identity across all 60 plugins
  and rebuilds every wasm — out of this slice's scope. Recorded here for a successor.

**Measured**: `cargo test -p semio-s-artifact-space-home --lib --features component-app-assembly`
→ **90 passed, 2 failed**. The 2 are the pre-existing crate-wide
`interactive-job.missing-factory` red (`create_studio` tests, B3b/F1's lane) — untouched by this slice
and failing identically before it. Before the fix the same command read 89 passed / 3 failed with the
duplicate-key law red.

## 3.4 Re-stage of the fixed guest — and a build-lock record (preamble rules 23, 25, 26)

The fix is guest Rust, so it needs the narrow re-stage S2 documented (§3.4 item 3): one component →
`space-plugin:materialize-dev` → `activate s react dev`.

- 11:20 `native component dev --manifest …🪐️space/…/Cargo.toml` on the SHARED target dir: **48 min,
  0 % CPU, no `rustc` child, `sample` showing 3 `prebuild_lock`/`flock` frames while 12 `rustc` ran
  elsewhere** — the rule-25 uplift starvation, not the rule-23 deadlock.
- 12:10 killed by pid and rerun with a **private `CARGO_TARGET_DIR=…/cargo/target-s3`** and the shared
  build dir. It started compiling within seconds — the cure works for this verb too (DS1 runs the same
  verb the same way), which is worth recording because S2 paid 16 min + a kill for the same build.
- It then wedged again in `prebuild_lock` at the final uplift. At 13:34 the rule-23(a) test came back
  positive for the whole machine: **`cargo deadlock at 2026-09-20 13:36`** — **ZERO `rustc` processes
  anywhere**, seven `cargo` processes all at 0 % CPU, six of them with 2–5
  `prebuild_lock_exclusive → flock` frames and no `rustc` child.
- The lock my build needs (`…/cargo/build/wasm32-wasip2/wasm-dev/.cargo-build-lock`, established with
  `lsof`) is **held by pid 18001, a peer's `🎞️animate` component build (ppid 17997 → a LIVE peer
  agent, not an orphan), itself blocked in `flock` for 1 h 42 m with no `rustc` child**. That is the
  partial cycle preamble rule 26 describes; rule 25's private target dir does not cure it.
- Rule 23(a) applied once: my own cargo killed by pid at 13:36. **The set did not unwedge** — still
  zero `rustc`, still seven 0 % cargos — so this slice was not the keystone. Peers' cargos are not
  mine to kill (rule 15), so this is handed to the coordinator, who owns breaking such a set
  (the 06:12 precedent in rule 23).

**Consequence, stated plainly**: the guest fix is proven by unit test (§3.3) and its cause was proven
live (§2), but the fixed wasm could not be staged into the running `s`, so §5's signed-in matrix and
LB1's f1–f8 could not be run by this slice. Everything needed is one command away once the build set
is broken:

```
CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false \
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-s3" \
bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts \
    native component dev --manifest ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml
bun nx run @semio-tech/space-plugin:materialize-dev
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript && bun ./📜️script.ts activate s react dev
```

## 4. C1c probe check 1 updated to the new contract

`🐍️c1c-s-host-probe.mjs` check 1 asserted the DEFECT (`identityFault: true`, i.e. Home refusing its
surface without a signed-in human). S2 removed the refusal and S3 removed the `DuplicateSiblingKey`
underneath it, so the contract is now: **the landing window publishes a live, empty surface signed
out, beside the shell's own sign-in call to action.** Rewritten accordingly (header doc + the three
check-1 rows):

| check 1 row | asserts |
|---|---|
| the `s` host renders its hub badge signed out | `signInOffered === 1` (unchanged) |
| **Home publishes a live signed-out surface, not a fault** | `[identityFault, homeSurface \|\| tableHosts > 0] === [false, true]` |
| **the signed-out surface offers the sign-in call to action** | `signInOffered > 0` |
| **a signed-out human is shown no spaces** | `spaceRows === 0` |

Check 2 (the space table after sign-in) is unchanged — it is the half this slice's fix is judged by.

### 4.1 A SECOND fault, found by that run: `/hub` tore the host app's canvas down

With the fixed guest staged, check 1 went green and check 2 still failed — but for a **new and
different reason**, measured with `🐍️s3-home-surface-diagnose.mjs --signin`:

```
[before]            homeSurface=true   "… Studios  Create Space  No studios yet. Create one from the navbar. …"
[after-workspace]   route=/hub   homeSurface=false   "… Route not found: /hub  Home …"
```

The signed-out body is fully alive (§3 landed). What removes it is the sign-in itself: opening the hub
workspace navigates a host-mode shell to `/hub` (`openHubWorkspace` → `navigateHistory("/hub")`), and

- `applyShellUri` treats `/hub` correctly — `setHubWorkspaceOpen(true); return;` — with the doc comment
  stating the contract: *"It is an overlay over whatever is already open, so it never tears down the
  running session"* (`🏛️ShellHost/🟦️.tsx:6247-6250`);
- but the canvas memo asks `parseShellRoute`, which has **no `/hub` concept** (S2 §4 recorded that it
  is matched locally and `parseShellRoute` is never edited), so `shellRoute.kind === "notFound"` and
  `ShellRouteNotFoundPage` **replaced the whole host app canvas** (`:10940`).

So the overlay route silently unmounted the landing app for every human who signed in. This also
explains S2 §4's `not-found:s` beacon after navigating to `/hub`, which was read there as a
re-boot artefact.

**Fix** (`🏛️ShellHost/🟦️.tsx`, TypeScript — vite picks it up on reload, no re-stage): a named
`SHELL_HUB_ROUTE` constant carrying the overlay contract in its doc, now used at all three `/hub`
sites, and the canvas guard exempts it:
`if (hostMode && shellRoute.kind === "notFound" && shellRoute.path !== SHELL_HUB_ROUTE)`.

### 4.2 Judged — all-PASS

`bun 🐍️c1c-s-host-probe.mjs http://127.0.0.1:6071 http://127.0.0.1:7501 1`, capture
`🗑️generated/s3-c1c-probe.txt`:

```
PASS 1 the s host renders its hub badge signed out: 1
PASS 1 Home publishes a live signed-out surface, not a fault: [false,true]
PASS 1 the signed-out surface offers the sign-in call to action: true
PASS 1 a signed-out human is shown no spaces: 0
PASS 2 the human is signed in inside s: 0
PASS 2 Home no longer answers session-identity-required: false
PASS 2 the host app surface is published: true
PASS 2 the command palette tab is present: true
c1c-s-host: all checks passed
```

`s-home-main` is in the DOM **before and after** the sign-in (`ids: ["s-home-main", "s-sync-status",
"s-presence-peers"]` in both censuses). This is the first time the `s` product's landing window has
been observed published at runtime.

## 5. Signed-in foreign-kind open inside `s` — two probe defects fixed, then the real blocker

Run signed in against 6071 with `S2_SIGN_IN_EMAIL`/`_PASSWORD` (credentials read from C1c's probe,
hub 7501 never modified). Captures `🗑️generated/s3-foreign-kind-{1,2,3}.txt`.

**The probe was wrong in two ways, both fixed at the root and both worth recording** — every
`command palette never opened` row S2 published came from these, not from the shell:

1. **The palette chord is `Meta+p`, not `Meta+K`.** Measured with `🐍️s3-journey-diagnose.mjs`:
   `Meta+KeyK` opens nothing on this shell; `Meta+p` opens `[role='dialog'] [data-slot='command-input']`
   immediately. It is also the chord the shipped studio e2e uses
   (`🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:72`). Fixed in `🐍️s2-s-host-foreign-kind-probe.mjs`.
2. **Signing in leaves the shell on `/hub` with the workspace overlay covering everything**, so
   everything the probe did afterwards reached through an overlay. `applyShellUri` closes the workspace
   for any path that is not `/hub`, so `page.goBack()` is the shell's own way back — measured
   `route=/ homeSurface=1 overlay=0`, session kept, no reload. Added to the probe's `signIn`.
   A redo step and a `historyState` reader were added too (the task's mutation + undo + **redo**).

With both fixed the palette opens, `spawn.draw` is found and clicked — and the shell answers with an
attributable refusal instead of silence:

```
semio: app "s.space.home@1/*#editor" dropped action "spawnApp" dispatched from window kind
"s-home-main": no window kind declares it (window kinds: s-home-main).
```

| plugin | loaded before open | lazy install | palette item | window | mutation | undo | redo |
|---|---|---|---|---|---|---|---|
| draw | yes (60/60 at boot) | n/a — see below | **found + clicked** | **none** — `spawnApp` not declared on `s-home-main` | — | — | — |
| note | yes | n/a | **found + clicked** | **none** — same refusal | — | — | — |

**What this establishes, honestly.** `spawn.<pluginId>` is a **studio** affordance: it needs a window
kind that declares `spawnApp`, which `🪐️space`'s studio app has and its Home landing app does not.
So the journey for outcome 1 is Home → open a space (studio) → palette → spawn, and this session had
**no space to open**: the shell reports `offline` and `Retrying directory update through sequence 3`
against hub 7501, so the directory never delivered one, and a local one could not be created either
(§5.1). **No foreign kind was opened, mutated or undone inside `s` by this slice** — that is stated
plainly rather than inferred from the palette entry existing.

Two findings handed on rather than fixed:

- **The palette offers a verb the active window cannot accept.** `spawn.draw` is listed and clickable
  from Home, and the dispatch is then dropped. That is the "panel actions dispatch in the active
  window" class: the palette should either scope program spawning to a window kind that declares it,
  or Home should route it. Owner: whoever owns `spacePrograms` in `🏛️ShellHost`.
- **The `s` shell could not reach hub 7501's directory** (`offline`, retry loop at sequence 3) while
  identity worked perfectly on the same page (`signInOffered: 0`, principal
  `01a0bbc1-126b-758b-a461-e6b9763fdb99` echoed in the workspace). Consistent with hub `/readyz`
  answering 503 on `artifactAuthority` (S2 §3.3, DS1's lane).

### 5.1 Why no space could be created

- `#s-home-create-space` IS published (DOM id `window:s-home-main/s-home-create-space`) but a
  synthetic click on it times out: it sits under the window's own engagement rail / utility bar
  (`framework.window.sHomeMain.engagement`, `…utilityBar` are siblings in the same body). This is the
  known "docked panel over the engagement rail" class B3a2 fixed in the shared interaction probe.
- The hub workspace's own **Create space** button is reachable only while the overlay is open, and the
  overlay's re-open affordance (`framework.hub.signIn`) no longer exists once signed in, so the probe
  could not get back into it in the same session.

Neither is a fault in this slice's fix; both are the next owner's concrete starting points.

### 5.2 LB1's live-bridge gate

Not rerun. Its `boot`/`ui_focus` steps and f1–f8 need a foreign-kind **editor open inside `s`**, which
§5 could not reach. What this slice does remove from LB1's path is the first half of its blocker: the
gate's `ready=null error=s windows=` reading came from the signed-out Home never publishing, and Home
now publishes (`ready:s`, `s-home-main` in the DOM, 148 spawnable programs). The remaining half is a
space to open a studio in.

## 6. Permanent wiring (nx target + launch row)

S2 wired `cold-boot-check-s` but not the live probe. Added, next to it:

- nx target **`s-host-foreign-kind-s`** (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`,
  immediately after `cold-boot-check-s`), `nx:run-commands`, `cache: false`, `forwardAllArgs: true`,
  running the probe from the workspace root.
- `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` row **`🛠️dev🪐️os-s🔭️foreign-kind`**,
  group `3_dev`, order `387.066` (next to `🛠️dev🪐️os-s🩺️cold-boot` at `387.065`), defaulting to
  `http://127.0.0.1:6071/ draw note layout`.

Honest note: the target points at the probe in this ticket folder. Its permanent home is
`🧑‍💻dev/🧪️tests/` beside `🔬️catalog-smoke`; it was not moved because it writes its captures and
screenshots relative to the ticket folder, and moving it would have been an unverified change on top
of an already long build queue.

## 7. Measured vs unverified, honest gaps

**Measured at runtime by this slice** (capture named for each):

- the refusal itself, verbatim from the running shell — `s3-home-diagnose.txt`
- the offending sibling keys `["#0", "#1", "s-home-create-space", "#0"]`, off the browser, in a unit
  test written before the fix — §3.1
- `cargo test -p semio-s-artifact-space-home --lib --features component-app-assembly`: **90 passed**,
  2 pre-existing failures (`interactive-job.missing-factory`, B3b/F1's lane)
- the narrow re-stage: component build 13 m 49 s through the fleet mutex, `materialize-dev` green,
  `Activated s react dev: 60 completed components` — `s3-space-component.txt`, `s3-materialize.txt`,
  `s3-activate.txt`
- **`c1c-s-host: all checks passed`** — `s3-c1c-probe.txt`; `s-home-main` in the DOM before AND after
  sign-in. First observed publication of the `s` product's landing window.
- the `/hub` canvas teardown, before and after the fix — `s3-home-diagnose.txt` + the all-PASS run
- the palette chord (`Meta+p` opens, `Meta+K` does not) and the post-sign-in route/overlay state —
  `🐍️s3-journey-diagnose.mjs` output
- `spawn.draw`/`spawn.note` clicked, dispatch dropped by `s-home-main` — `s3-foreign-kind-3.txt`

**Not done / unverified**:

- **One mutation + undo + redo in any foreign plugin inside `s`** — not reached (§5). The probe is now
  correct up to the spawn; the gap is a studio to spawn into.
- **Lazy cross-plugin install as *lazy*** — still unexercisable: host mode installs all 60 at boot, so
  `lazyInstalled` is `false` by construction (S2 §3.2 point 1 still stands).
- **LB1's f1–f8** — §5.2.
- **The framework-wide `try_build()` key trap** — deliberately not changed (§3.3).
- The `s-host-foreign-kind-s` nx target was added but **not executed through nx** (the probe itself was
  run directly, three times); its `forwardAllArgs` path is unverified.
- The signed-in Home shows no space rows because the hub directory is offline for this shell, so
  "the space table appears after sign-in" is verified only as *the surface publishes*, not as *rows
  arrive*.

## 8. Files changed

| file | change |
|---|---|
| `✏️s/🔌️plugins/🪐️space/…/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs` | **the fix**: `S_HOME_EMPTY` + `S_HOME_DEAD_LINE` keys; the empty branch built through the contract builder with `try_id` instead of `built_text_node`; `window_content_dead_line_spacer(key)`; `render_rows_wrapped` doc states the `try_build()` root-key trap |
| `…/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs` | new `first_duplicate_sibling` walker + two laws (`the_signed_out_…` / `the_signed_in_window_body_survives_the_component_tree_producer`) |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | stale identity-refusal law rewritten as `home_render_publishes_signed_out_and_changes_roles_with_the_identity`; `config_with_one_folded_space` now folds a `member.upserted` event so the role half can actually distinguish |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `SHELL_HUB_ROUTE` constant (doc carries the overlay contract), used at all three `/hub` sites; the canvas' `notFound` guard exempts it, so the hub workspace no longer tears down the host app |
| `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | new nx target `s-host-foreign-kind-s` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | row `🛠️dev🪐️os-s🔭️foreign-kind`, group `3_dev`, order `387.066` |
| `🐍️c1c-s-host-probe.mjs` (ticket) | check 1 rewritten to the new contract (publishes signed out, offers sign-in, shows no spaces) + header doc |
| `🐍️s2-s-host-foreign-kind-probe.mjs` (ticket) | `Meta+p` chord, introduction-veil dismissal, `goBack()` to the landing route after sign-in, redo step, `historyState` reader |
| `🐍️s3-home-surface-diagnose.mjs`, `🐍️s3-journey-diagnose.mjs`, `🐍️s3-palette-diagnose.mjs` (ticket) | new diagnostics that produced §2, §4.1 and §5 |

## 9. Live state handed on

- 6070 (pid 26173) and 6071 (pid 33969) still serving, **not restarted**; hub 7501 untouched.
- The staged `s` output now carries the fixed `🪐️space` guest (receipt 60/60, `s3-activate.txt`).
- `SHELL_HUB_ROUTE` is TypeScript — a reload picks it up, no re-stage.
- Next owner's first move: get one space into the shell (hub directory is offline for it today), open
  its studio, then `bun 🐍️s2-s-host-foreign-kind-probe.mjs http://127.0.0.1:6071/ draw note layout`
  with `S2_SIGN_IN_EMAIL`/`_PASSWORD` — the probe is correct up to the spawn.
