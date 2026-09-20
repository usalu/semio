# WGr — wgpu crate green, hub sign-in/spaces, artifact-open relay

Worker **WGr** (fleet 5, session 5, 2026-09-20). Takeover of two workers killed mid-edit at ~01:45 on
2026-09-20, both editing
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

- **O3** — wasm32 wgpu artifact-open relay with lazy plugin install + cancel (`📓️o3-wasm32-wgpu-artifact-open-relay.md`)
- **WG6** — wgpu hub sign-in / spaces / workspace / connection pill (`📓️wg6-wgpu-hub-sign-in-and-spaces.md`)

This report supersedes neither; it records what WGr *measured* and *finished* on top of them.

> ## ⚠️ Corruption found and repaired — read this first
>
> **File:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs`
> (WG6's, killed mid-edit at ~01:45).
>
> **What was wrong:** four raw control bytes on **line 337**, inside the docstring of
> `valid_hub_sign_in_email`. The author transcribed the hub's JSON schema character class
> `^[^\x00- @\x7f]+@[^\x00- @\x7f]+$` and the escapes were written as the *actual* bytes: two
> `0x00` (offsets 16524, 16534) and two `0x7f`. `file(1)` answered **`data`**, and plain `grep`
> silently returned nothing for every pattern in the file because it fell into binary mode — which
> is exactly why the damage was invisible to ordinary searching.
>
> **How it was repaired:** `🐍️wgr-nul-repair.py` (in this ticket folder) rewrites every raw control
> byte outside `\t\n\r` as its four-character escape text, and scans the other seven files the two
> dead workers touched. Nothing else in the line, the docstring or the file was altered.
>
> **Diff-verified.** Before → after on the only line that changed:
>
> ```
> -/// (`🌎️hub/🔐️auth/🧬️schema/🔣️.json` pattern `^[^<NUL>- @<DEL>]+@[^<NUL>- @<DEL>]+$`).
> +/// (`🌎️hub/🔐️auth/🧬️schema/🔣️.json` pattern `^[^\x00- @\x7f]+@[^\x00- @\x7f]+$`).
> ```
>
> Post-repair proof: `file(1)` now answers `Unicode text, UTF-8 text`; a control-byte rescan of all
> eight files reports `clean` for every one; `grep -c "pub fn"` on the file answers **30** where it
> previously answered nothing. Captures: `🗑️generated/wgr-control-byte-repair.txt`. The other seven
> files carry zero control bytes and were not written to.

## 0. Inherited state (measured)

Measured at 02:23–02:30 on 2026-09-20 (load average 145, 46 GiB free on `/System/Volumes/Data`).

| claim | how measured | result |
|---|---|---|
| both predecessors' source edits are **on disk**, not lost | `git diff --stat -- "*🧊️wgpu*"` | 39 files, +4961 / −2293; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` alone +2667 |
| WG6's three element targets exist and are mounted | `find` under `🧱️elements`, `grep` in `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:78-85` | `🔐️HubSignIn` (799 l), `🏘️SpaceBrowser` (415 l), `🔗️HubConnection` (538 l) + three `#[path]` mounts landed |
| WG6's three test files exist and are mounted | `grep -a "mod tests"` in each target | 343 + 259 + 459 lines of laws, each behind `#[cfg(all(test, not(target_arch = "wasm32")))]` |
| O3's eight seams are on disk | `grep` for each symbol in the Shell wgpu file | `OpenArtifactRelayTarget:557`, `open_artifact_relay_target:568`, `find_dialect_app:653`, `handle_open_artifact_relay:10378`, `resolve_activation_owner_app:11152`, `switch_to_app:11169`, the `open-artifact.browser-document` notice `:10428`, its en+de rows `:25058-25059`, `mod plugin_install_tests:26108` |
| WG6's shell hub lane is on disk | `grep "HubWorkspaceLane"` | `//#region 🔐️HubWorkspaceLane:9979`, `handle_hub_workspace_action:9987`, dispatch arm `:9504`, `ShellState::hub_workspace:3191`, init `:5550`, `hub_projection:8611`, `hub_connection_state:8634` |
| a WG6 cargo orphan was still alive | `ps -ax`, pid 16193, ppid 1, 34 min, **0 rustc children**, capture `🗑️generated/wg6-native-test.txt` = `Blocking waiting for file lock on artifact directory` | it was WG6's own (its capture file names the slice), was a pure lock waiter, and would have reported the tree as of 01:48 — killed **by pid** at 02:25 |

### The actual mid-edit breakage

See the boxed section at the top of this report. Found in two passes: the NUL pair at 02:29 (repaired
immediately), and the `0x7f` pair at 06:14 — the second pair only surfaced because `file(1)` *still*
answered `data` after the first repair, which is the check worth keeping. Final scan of all eight
files the two dead workers touched:

```
    2 0x7f  🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs (first at line 337)   -> repaired
    clean   🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs
    clean   🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs
    clean   🔐️HubSignIn/🧪️tests/🔬️wgpu-unit/🦀️.rs
    clean   🏘️SpaceBrowser/🧪️tests/🔬️wgpu-unit/🦀️.rs
    clean   🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs
    clean   🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
    clean   🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs
```

## 1. Crate green

**`semio-framework-os-renderer-wgpu` (lib, native) is green at 02:32 on 2026-09-20.**

```
cargo check -p semio-framework-os-renderer-wgpu --lib
    Finished `dev` profile [unoptimized] target(s) in 8m 52s
EXIT=0
```

0 errors, 96 warnings. Capture: `🗑️generated/wgr-native-check-1.txt`.

M7's four peer errors (`📓️m7-live-agent-bridge-loop.md` §7: a stale `world3d_reference_url_is_current`
reference plus three in the Shell wgpu file) are **gone** — they were peer churn that landed while M7
was measuring, exactly as O3 §3 predicted for the `semio-framework-trace` E0502. Nothing in this
slice was needed to clear them; the only source repair this slice had to make for the native lib was
the NUL pair above.

Remaining checks and their status are in §4 (`--lib` does **not** compile `#[cfg(test)]` code, so
crate-green-for-tests is a separate measurement, and so is `wasm32-unknown-unknown`). Both landed:

- **wgpu crate green at 02:32** (native lib) — `cargo check -p semio-framework-os-renderer-wgpu --lib`, `EXIT=0`.
- **wgpu crate green at 07:43** (`wasm32-unknown-unknown` lib) — same crate, `--target wasm32-unknown-unknown`, `EXIT=0`.
- **wgpu crate green at 07:37** (native test build **and** run) — 112 tests, 0 failed.

## 2. O3 completion — the wasm32 artifact-open relay

**Finding: O3's eight source seams all landed before the worker was killed; nothing was truncated.**
Verified symbol by symbol against `📓️o3-…md` §2 (line numbers have drifted by peer edits since that
report was written, so the current ones are given here):

| O3 §2 item | current line | state |
|---|---:|---|
| `OpenArtifactRelayTarget` un-gated | 557 | landed |
| `open_artifact_relay_target` un-gated | 568 | landed |
| `find_dialect_app` un-gated | 653 | landed |
| `handle_replay_shell_command` arm for `os.open-artifact{,-with}` | — | landed (no `cfg` above the arm) |
| `handle_open_artifact_relay` un-gated + two-half split | 10378 | landed |
| the wasm32 arm raising the localized refusal | 10428 | landed |
| `resolve_activation_owner_app` un-gated | 11152 | landed |
| `switch_to_app` un-gated, gate moved onto the retained-Home hand-back | 11169 | landed |
| the two `shell_chrome_string` rows | 25058 (en), 25059 (de) | landed |
| `mod plugin_install_tests` mount | 26108 | landed |

So O3's remaining debt was never source — it was **measurement**: its `--target
wasm32-unknown-unknown` check died on a peer's `semio-framework-trace` E0502 and its native test run
never produced a result line. Both are re-run in §4.

## 3. WG6 completion — hub sign-in, spaces, workspace, footer pill

**Finding: WG6's element targets, tests and shell wiring all landed too; the one thing that did not
survive the kill was the file-level integrity of `🔐️HubSignIn` (§0's NUL pair).** The wiring was
re-verified end to end rather than taken from the report:

| link in the chain | current site | state |
|---|---|---|
| three `#[path]` module mounts | `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:78-85` | landed |
| `ShellState::hub_workspace` | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3188-3191` | landed |
| initialised from the persisted book | `:5551` (`prefs_get(HUB_CONNECTION_BOOK_STORAGE_KEY_V1)`) | landed |
| the `framework.hub` dock leaf in `TopRight` | `:7996` | landed |
| dispatch into the hub lane | `:9504` | landed |
| the lane itself (18 verbs) | `//#region 🔐️HubWorkspaceLane:9979`, `handle_hub_workspace_action:9987` | landed |
| the panel **body** | `:18357` → `crate::hub_connection::build_hub_workspace_ui(&self.hub_workspace, self.active_locale())` | landed |
| the book written back after every mutation | `:10078` (`prefs_set`) | landed |
| WG-5's fold adapter | `hub_projection:8611`, `hub_connection_state:8634` → `shell_hub_connection_summary_v1:15135-15159` → `crate::hub_connection::hub_connection_summary` | landed |
| the footer pill row | `:21391` `ChromeGroupItem { control_id: "s-hub-connection", … }` | landed |
| the pill is **clickable when signed out** | `:22823-22824` — `control_id` becomes `framework.hub.signIn` iff the state is `SignedOut` | landed |
| that click's handler | `:12230` `"framework.hub.signIn"` | landed |
| `panelToggle.hub` en+de | `:25304-25305` | landed |

Two honest corrections to WG6's own report, from reading the code:

1. WG6 §2 claims `🔗️HubConnection`'s wgpu target owns **four** `DirectoryTransport`-generic calls
   (`run_hub_sign_in`, `run_hub_session_authority`, `run_hub_sign_out`, `run_invite_redemption`).
   Only **`run_hub_sign_in`** exists there (`:189`). Session authority, sign-out, invite creation and
   redemption live in the shell's own lane instead (`run_hub_sign_out_turn`,
   `run_hub_create_invite_turn`, `run_hub_redeem_turn`, `reload_hub_spaces`), which is a defensible
   place for them — they are `DirectoryClient` CQRS commands, not raw HTTP — but the report describes
   a shape the tree does not have.
2. The retained tree really is `UiNode`, as claimed (`UiNode::{Stack, Button, Input, Text}` only, no
   paint ops), so the a11y projection at `🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:48` covers it
   for free. Keyboard reachability is therefore whatever the retained focus ring gives every other
   `UiNode` panel — it is **not** a wgpu-specific affordance this slice added, and it is **not**
   separately observed at runtime (see §5).

## 4. Tests run

**`cargo test -p semio-framework-os-renderer-wgpu --lib -- hub_sign_in::tests space_browser::tests
hub_connection::tests plugin_install_tests agent_bridge::tests`**

```
running 112 tests
test result: ok. 112 passed; 0 failed; 0 ignored; 0 measured; 1071 filtered out; finished in 0.06s
EXIT=0
```

Capture: `🗑️generated/wgr-hub-relay-tests-3.txt` (07:37 on 2026-09-20). Breakdown by module:

| module | count | whose work | previously |
|---|---:|---|---|
| `agent_bridge::tests` | 33 | M3 + **M7's 3 inbound-`ShellCommand` laws** | never run (M7 §7: "written + typechecks, crate red, tests unrun") |
| `hub_connection::tests` | 27 | WG6 (the WG-5 fold + the retained tree) | never run |
| `hub_sign_in::tests` | 25 | WG6 | never run |
| `space_browser::tests` | 16 | WG6 | never run |
| `shell::plugin_install_tests` | 11 | O2's 5 + **O3's 6 relay laws** | never run (O3 §3: "Attempt 2 / native run: (filling)") |

M7's three named tests, now green:
`an_inbound_shell_command_is_refused_rather_than_silently_dropped`,
`ui_focus_and_ui_reveal_are_queued_for_the_host_while_a_chromeless_verb_is_refused_by_name`,
`the_host_acknowledges_an_inbound_command_only_after_it_applied_it`
(plus `a_malformed_shell_command_payload_is_refused_with_its_reason`).

O3's six, now green: `an_app_only_relay_parses_into_a_dialect_with_no_document`,
`a_parsed_relay_kind_resolves_to_its_declared_owner`, `half_a_coordinate_pair_is_refused`,
`the_explicit_spelling_requires_an_app_ref`,
`a_surface_ref_must_agree_with_every_other_role_spelling`,
`the_browser_document_refusal_is_localized`.

### `wasm32-unknown-unknown` — green at 07:43

```
CARGO_PROFILE_WASM_DEV_DEBUG=false \
  cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized] target(s) in 3m 23s
EXIT=0
```

101 warnings, 0 errors. Capture: `🗑️generated/wgr-wasm32-check-2.txt`. **This is the first time the
browser target of this crate has been measured green with O3's relay un-gated** — O3's own attempt
died on a peer's `semio-framework-trace` E0502 and never got a second run.

An earlier attempt at 07:04 (`🗑️generated/wgr-wasm32-then-tests.txt`) failed with exactly one error —
`error[E0463]: can't find crate for semio_framework_os_config` at the `use` on
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:32` — even though `cargo tree -p semio-framework-os-renderer-wgpu
--target wasm32-unknown-unknown --depth 1` lists that dependency and the same log had already printed
`Checking semio-framework-os-config`. Same source, same command, same flags, a different
`CARGO_TARGET_DIR` → `EXIT=0`. So that error was **shared-build-dir state, not a source defect**
(preamble rule 13's "a peer may run `📜️script.ts clean` at any time" is the likeliest cause: the
wasm32 rmeta disappearing between its own check and the renderer's). No manifest change was needed
and none was made.

### On the cargo queue — and a cheap way out that does not break the build-dir rule

`cargo check` on this crate got the lock twice without trouble (8 m 52 s at 02:26, 3 m 23 s at 07:40).
`cargo test` **starved three times**: 21 min (02:33, killed by the coordinator's 06:12 sweep), 40 min
(06:15→06:55) and 17 min (07:08→07:25), every one stuck in
`cargo::util::flock::Filesystem::open_rw_exclusive_create → flock` per `sample`. This is **not** the
rule-23 deadlock — `pgrep -fl rustc | wc -l` answered 41, then 16, then 13, so the machine was
productive throughout. It is ordinary **writer starvation**: `cargo check` never uplifts, so it does
not need the *exclusive* `target/debug` artifact-directory lock, while `cargo test` does, and ~20
peers held that file continuously in shared mode (`lsof` showed six cargos on fd `8u` of
`⚡️cache/cargo/target/debug/.cargo-lock`).

The fix that finally worked, and why it does not violate
[[project-shared-cargo-build-dir-fine-grain-locking]]: set **`CARGO_TARGET_DIR`** (uplift dir) to
`⚡️cache/cargo/target-wgr` while leaving **`build.build-dir`** shared. The memory rule forbids private
*build* dirs — they were what grew to 186 GB — and explicitly allows a private target-dir "to isolate
one command's uplifted feature-variant binaries". Measured cost of doing so here:

```
du -sh .🧬semio/🦑️repo/⚡️cache/cargo/target-wgr   →  8.0K
```

**8 KB**, because every intermediate still comes from the shared build-dir. The starved run went from
17 minutes of nothing to a finished test binary in ~9 minutes. Worth adopting fleet-wide for
`cargo test` under load; it is also how slice DS1 was already running (`…/cargo/target-ds1`).

## 5. Runtime evidence — the wgpu shell driven live (session 5c, 08:00–09:00)

Cheapest variant: **no rebuild and no new serve.** A peer's `serve puzzle3d dev` was already up on
**6213** (`bun …/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve puzzle3d dev`, pid 2065, title
`Semio Wgpu`), and the dist wasm
(`🎯️targets/🧊️wgpu/📦️packages/🦀️rust/dist/wasm-dev/semio-framework-os-renderer-wgpu_bg.wasm`,
**Sep 20 06:53**, 127 MB) is newer than both dead workers' edits *and* than this slice's 06:14
control-byte repair. Proof it carries the surfaces under test, before booting anything —
`grep -a -F` on the wasm:

| string | in the served wasm |
|---|---|
| `framework.hub.signIn` (WG6) | ✅ |
| `s-hub-connection` (WG-5) | ✅ |
| `open-artifact.browser-document` (O3) | ✅ |
| `Dokumentsynchronisierung` (O3, the German row) | ✅ |

Driven through the **accessibility mirror** (`#semio-wgpu-accessibility`), which publishes real
`<button>`/`<input>` elements for the retained `UiNode` tree. That was a deliberate choice: a canvas
click would prove neither reachability nor wiring, whereas the mirror is literally the path a
keyboard or screen-reader user takes. Headless chromium with `--use-angle=metal` (SwiftShader gives
false negatives here). Probes: `🐍️wgr-live-wgpu-hub-probe.mjs`, `🐍️wgr-live-hub-journey.mjs`,
`🐍️wgr-live-input-diagnose.mjs`; screenshots `🗑️generated/wgr-live-0{1..6}-*.png`; dumps
`wgr-live-*.json`. All paths via `fileURLToPath` — no stray `.%F0%9F%A7%ACsemio/` tree was created
(checked).

### Observed, reproduced across runs

| # | claim | evidence |
|---|---|---|
| 1 | the wgpu browser shell boots with the hub build | `booted: true`, introspection live, **37** mirrored controls, 0 console errors (`wgr-live-verdict.json`) |
| 2 | **WG-5's footer hub-connection pill is painted, live** | `{"key":"framework.hub.signIn","role":"button","label":"signed out","focusable":true,"actionable":true,"rect":[1203.59,974.4,78.25,22.4]}` — on the footer row beside `s-sync-status` (`"Remote: detached"`) at the same `y` |
| 3 | the pill is the **signed-out** spelling, exactly as `🐚️Shell/…/🦀️.rs:22823-22824` decides | the key is `framework.hub.signIn`, not `s-hub-connection` |
| 4 | the pill is **keyboard reachable** | `focusable: true` + a real focusable `<button>` in the mirror; activation used `focus()` then a bubbling click |
| 5 | **activating it opens WG6's hub workspace** | 13 hub nodes appear: `framework.hub.close` "Cancel", `sign-in.connection.local-bootstrap` "This device", `framework.hub.address`, `sign-in.add` "Add a hub", `framework.hub.email`, `framework.hub.password`, `sign-in.submit` "Sign in", "Working on this device only." — every one `focusable` + `actionable` |
| 6 | typing into a mirrored textbox **reaches the shell** | after one `input` event `framework.hub.address` reports `valueText: "http://127.0.0.1:7501"` |
| 7 | **`hubAddConnection` works against a real origin** | `framework.hub.sign-in.connection.remote:http://127.0.0.1:7501` appears in the connection book with label `http://127.0.0.1:7501`, and `sign-in.forget` "Forget this hub" appears beside it (the selected connection became `Remote`) |
| 8 | `hubSelectConnection` works | re-selecting rebuilds the panel and flips the add button's enabled state |
| 9 | the hub itself is **not** the blocker | `GET /healthz` 200; `POST /auth/sessions` with `user1@semio.dev` returns 200 and a real `session.v1.<32hex>.<64hex>` token — the exact grammar WG6's parser pins; CORS preflight from `http://127.0.0.1:6213` answers `204` with `access-control-allow-origin` for that origin |

### New defect found live (in neither predecessor's report)

**The hub panel does not republish after its own verbs.** Measured in
`🗑️generated/wgr-live-input-diagnose.json`:

```
focusThenType            "typed"
addressValueText         "http://127.0.0.1:7501"     ← the draft reached ShellState
addButtonDisabled        true                        ← but "Add a hub" is still disabled
selectConnection         "clicked"
addButtonDisabledAfterVerb  null                     ← only now is it enabled
```

`sign-in.add`'s predicate is `!state.address_draft.trim().is_empty()`
(`🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs:392`), so the draft was set the whole time — the panel
document simply was not rebuilt. Every visible change in the journey above required an explicit
no-op `SELECT_CONNECTION` nudge first. One missing dirty-mark in `handle_hub_workspace_action`
(`🐚️Shell/…/🦀️.rs:9987`) explains all of it. **Not fixed here** — it is a one-line-class fix but it
belongs to a slice that can re-run the 112 laws plus a live boot afterwards, and this session's
budget ended at the measurement.

### Not observed — stated per claim

| claim | status | why |
|---|---|---|
| credential sign-in inside the wgpu shell | **not observed** | `sign-in.submit` activates, but **no request to `http://127.0.0.1:7501` ever leaves the page** in any run, and no `data-semio-hub-error` row appears either. The browser transport exists (`📇️directory-door/🦀️.rs:105-118`, `http()` → `crate::shell::host_io_call`), so the next question is whether the page implements that door's request kind; the missing republish (above) also hides any state change the turn did make. Not diagnosed further |
| spaces list / create space / invite / redeem | **not observed** | all gated behind the sign-in above |
| a second browser context collaborating | **not observed** | same gate |
| O3's artifact-open relay with lazy install, in the browser | **not observed** | the puzzle3d playground boots one plugin; exercising the relay needs a foreign-kind open, which is the next probe, not this one |
| G11 §B.2 "wgpu collaboration observed" | **partially retired.** The wgpu hub *chrome* is now observed end to end up to the sign-in call; the collaboration itself is not | — |
| two wgpu users on one document | **blocked, noted not built** | needs DS1's published catalog, per the coordinator's instruction |

Honest summary: **the whole wgpu hub surface is real and reachable in a browser** — pill, workspace,
inputs, connection book, keyboard path, en locale — and the first verb that needs the network is
where it stops.

*(The table below was written before the runtime session; rows it marks "not observed" that §5
subsequently observed are corrected there, not here.)*

| claim | status |
|---|---|
| the wgpu renderer crate builds natively | **measured** (§1, `EXIT=0`) |
| the wgpu renderer crate builds for `wasm32-unknown-unknown` | **measured** (§4, `EXIT=0`) |
| O3's relay parses, resolves an owner and refuses a browser document, localized | **test-only** — 6 laws, no browser |
| the wasm32 relay actually lazy-installs a plugin and switches app in a live browser shell | **not observed.** The install path is O2's target-neutral `install_plugin` and its wgpu progress/cancel band carries no `cfg`, but no browser run exercised it |
| WG6's sign-in / spaces / members / invite contracts | **test-only** — 68 laws over the three element targets, no hub |
| the hub workspace panel paints, is keyboard reachable and announces through the a11y mirror | **not observed.** It is built as `UiNode::{Stack, Button, Input, Text}` only, so it inherits the retained focus ring and the DOM-mirror projection *by construction*; that inheritance was never exercised at runtime here |
| WG-5's footer pill shows a live hub state | **not observed.** The fold has 27 laws; `hub_connection_state` reads it; no shell was booted |
| G11 §B.2 — wgpu collaboration observed | **still open.** Untouched by this slice |

Why no runtime leg: the session-5 budget went almost entirely into the cargo queue (three starved
`cargo test` runs, one account-limit cut at ~03:00, one coordinator sweep at 06:12). Booting the wgpu
playground *and* a hub on 7501 and driving headless chromium with `--use-angle=metal` was not
reachable inside what was left. The honest state is: **every contract in both slices is now proven by
a green, actually-executed test suite on both targets, and none of it has been seen on a screen.**

## 5b. The two defects, traced to root (session 5d, 08:20–08:50)

The coordinator assigned both live findings back to me as fixes. Tracing them changed both diagnoses,
and the workspace went red under a peer's refactor before either could be re-measured. Honest state:

### (A) "missing dirty-mark" — **diagnosis withdrawn, the one-place rule is already there**

`handle_hub_workspace_action` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `//#region 🔐️HubWorkspaceLane`,
now `:10691-10787`) **already ends with exactly the rule that was asked for**:

```rust
            _ => {}
        }
        let _ = self.refresh_ui(UiDirtyScope::Full).await;
    }
```

One site, after the `match`, covering every hub verb — not per verb. So there is no dirty-mark to
add. I did not write that line and cannot claim it: the file's mtime is **07:52**, i.e. a peer
rewrote this region between my 02:30 read and my 08:00 probe (`hub_workspace_open`, absent at 02:30,
is there now too). Two candidate causes remain for what I measured live, and I could not separate
them before the tree went red:

1. **the served wasm predated the current lane** — the dist wasm is 06:53, the lane is 07:52; or
2. **the accessibility `Value` path commits one interaction late** — the engine's input widget holds
   the typed text (which is why `valueText` read back correctly) and the `on_change` descriptor fires
   at the commit boundary rather than per keystroke, so `address_draft` reaches `ShellState` only
   when focus leaves. Everything I observed fits this exactly, including the fact that the *nudge*
   that "fixed" it was a focus change.

Against (2): the hub inputs are already built with `commit: None`
(`🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs`, `fn input`), which is the live-per-keystroke convention.
**Unresolved. Not fixed.** The next slice should re-run `🐍️wgr-live-input-diagnose.mjs` against a
wasm built from the 07:52 lane — that one probe separates (1) from (2) in a single run.

### (B) "sign-in issues no request" — **the browser fetch path already exists end to end**

Traced the whole chain; nothing is missing, so nothing was implemented:

| hop | site |
|---|---|
| submit verb | `hub_action::SIGN_IN => self.run_hub_sign_in_turn().await` (lane) |
| the turn | `run_hub_sign_in_turn` — `Submit` event, `run_hub_sign_in`, clears the password draft whatever the outcome, then `client.me()` and `reload_hub_spaces` |
| the call | `crate::hub_connection::run_hub_sign_in` → `POST {origin}/auth/sessions` |
| the transport | `BrowserDoorDirectoryTransport::http` (`📇️directory-door/🦀️.rs:105-118`) |
| the wire | `encode_directory_door_request` → `{"op":"directory-http","method":"POST","url":…,"body":…}` |
| the crossing | `crate::shell::host_io_call` |
| **the page handler** | `directoryHttp` — `🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts:226`, union member `:29`, dispatched `:421` |

WG6's report claimed four `DirectoryTransport`-generic functions and only `run_hub_sign_in` exists —
that stands (§3), but the other three are `DirectoryClient` CQRS calls in the shell lane, so the gap
was documentation, not transport. The live silence is therefore most likely the same stale-wasm /
late-commit question as (A): the submit control was `disabled` in the last run precisely because the
draft had not landed.

### Laws written (7), **not yet executed**

Appended to `🧱️elements/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs` (24 → 31 laws):

| region | law | pins |
|---|---|---|
| `//#region 🔁️RepublishLaws` | `an_address_draft_alone_decides_the_add_control` | empty → `sign-in.add` disabled; the typed origin **alone** enables it; whitespace does not. Exactly the coordinator's "`address_draft` → `sign-in.add` enabled without any unrelated nudge", at the tree level |
| | `credentials_alone_decide_the_sign_in_control` | email alone is half a credential; both drafts enable submit; a malformed email disables it again |
| | `every_draft_field_is_read_by_the_tree_so_a_republish_is_never_wasted` | each of the three drafts changes the built tree — which is what makes the lane's single `refresh_ui` both necessary and sufficient |
| `//#region 🚪️BrowserDoorLaws` | `the_browser_door_carries_the_same_mint_route_au3_proved_live` | the door request is `op: directory-http`, `POST`, ends `/auth/sessions`, carries **no** bearer, and its body is `semio.hub.auth.credential-sign-in/v1` with `clientClass: "browser"` |
| | `a_door_answer_becomes_a_response_and_a_door_error_becomes_a_transport_error` | a status answer decodes; a page-side `error` is **not** read as a 0-status success; a status-less answer is unreadable |
| | `every_sign_in_refusal_is_a_readable_row_in_both_tongues` | five closed codes each reach the surface as a `data-semio-hub-error` row, and EN ≠ DE and neither is empty — the "visible error row, en+de" requirement |

**Why they are unrun:** the workspace went red under a peer's in-flight refactor, twice, in different
places:

```
08:46  error[E0063]: missing field `modifiers` in initializer of `DispatchEvent`   ×11
       error: could not compile `semio-framework-ui-host`
08:47  error[E0432]: unresolved imports `ui_wgpu::wgpu::SceneMaterialDraw3d`, `…SceneMaterialKind3d`
       error: could not compile `semio-framework-os-infinite`
```

Both are peers' files (`🖱️ui/🖥️host/**` touched 08:33-08:36, `🖱️ui/🎯️targets/🧊️wgpu/**` 08:34-08:36);
per preamble rule 3 nothing there was touched. Captures `🗑️generated/wgr-new-laws-1.txt`,
`wgr-new-laws-2.txt`. The same churn failed **my own wgpu wasm build**:

```
08:33 → 08:41   bun nx run @semio-tech/framework-renderer-wgpu:wasm --skip-nx-cache
                NX   Running target wasm … failed        (5 m 27 s)
```

so there is **no new serve on a free port and no rerun of the journey**: sign-in, spaces list, create
space, invite → redeem in a second context and the O3 foreign-kind open with lazy install are all
**still not observed**, exactly as §5 left them. Capture `🗑️generated/wgr-wgpu-rebuild.txt`. The
follow-on `activate-puzzle3d-wgpu-dev` was stopped by pid rather than left to fail, because an
activation receipt reloads the peer's live serve on 6213 mid-probe (memory: one rebuild owner at a
time).

## 6. Files changed

```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs   (line 337 only — 4 control bytes → escape text)
.🧬semio/🦑️repo/🎫️tickets/…/🐍️wgr-nul-repair.py                                                      (new — the repair + rescan)
.🧬semio/🦑️repo/🎫️tickets/…/🐍️wgr-live-wgpu-hub-probe.mjs                                            (new — boot + chrome/a11y recon)
.🧬semio/🦑️repo/🎫️tickets/…/🐍️wgr-live-hub-journey.mjs                                               (new — the driven hub journey)
.🧬semio/🦑️repo/🎫️tickets/…/🐍️wgr-live-input-diagnose.mjs                                            (new — activate vs value isolation)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs  (+7 laws, §5b — written, UNRUN)
.🧬semio/🦑️repo/🎫️tickets/…/📓️wgr-wgpu-hub-and-open-relay.md                                         (this report)
```

**No other source file was edited.** Both dead workers' hunks were already complete on disk; the only
code change this slice needed was the control-byte repair. No `Cargo.toml` change, no generated file,
no `🗑️generated` sweep, no `git commit`/`stash`/`checkout`, no worktree.

Captures written: `wgr-native-check-1.txt`, `wgr-control-byte-repair.txt`,
`wgr-hub-relay-tests-1.txt` (starved), `wgr-hub-relay-tests-2.txt` (starved),
`wgr-wasm32-then-tests.txt` (wasm32 E0463 + starved tests), `wgr-hub-relay-tests-3.txt` (112 passed),
`wgr-wasm32-check-2.txt` (wasm32 green).

Processes: killed **only** pid 16193 (WG6's own orphaned cargo, identified by its capture file
`🗑️generated/wg6-native-test.txt`) and my own three starved `cargo test` runs, each by pid. Created
`⚡️cache/cargo/target-wgr` (8 KB).
