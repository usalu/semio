# W3 — `🪐️space` plugin migration report

Plugin: `✏️s/🔌️plugins/🪐️space/` (crate `semio-s-plugin-space`). Cleared for APA by both SMO and UCAS.

## Step 0 — baseline

`cargo check -p semio-s-plugin-space` was queued before any edit. The shared build lock
(`CARGO_TARGET_DIR=…/🎯️target`) serialized this run behind other concurrent agents; see
`## Verification` below for the real pasted output once the lock cleared.

## What changed

### Step 1 — dead facet directories deleted

All three were the 1-line doc-only stub (no real code) and **unmounted** — `grep -n
"🛂️manifest\|🎟️capabilities\|🔧️setup" "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs"` returned
zero matches before deletion. Confirmed also via `📓️w0-b-plugin-shape.md` §3/§4 (space listed among
the 30/33 doc-only `🔧️setup`, 32/33 doc-only `🛂️manifest`, 33/33 doc-only `🎟️capabilities`) and via
`taxonomy.json`'s `pluginChildDirs`, already flipped to `["🎛️apps"]` as of the time this wave started
(`git log --oneline -3 -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` → `a445617cae`
`🐙️ueli🎆️26🌙️06☀️04🚩️493`, `stat -f '%Sm'` → `Aug 12 15:28:34 2026`), so the hard `assert!` gate at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:2226-2235` (which reads
`pluginChildDirs` dynamically) no longer requires these three dirs.

- **Removed** `✏️s/🔌️plugins/🪐️space/🛂️manifest/` (1 file, doc-only stub)
- **Removed** `✏️s/🔌️plugins/🪐️space/🎟️capabilities/` (1 file, doc-only stub — this is also the ONE
  plugin repo-wide with a real capability call, `.local_backbone_storage()` at plugin-root
  `🦀️component.rs:63` inside `pub fn plugin()`, per `📓️w0-b-plugin-shape.md` §4 — that call was
  already correctly at plugin root, not in this facet, so nothing needed to move)
- **Removed** `✏️s/🔌️plugins/🪐️space/🔧️setup/` (1 file, doc-only stub)
- No `.DS_Store` / `node_modules` found at plugin root.

Post-deletion `grep -rn "🪐️space/🛂️manifest\|🪐️space/🎟️capabilities\|🪐️space/🔧️setup"` across the whole
repo (excluding this ticket folder): zero hits — no dangling references anywhere.

### Step 2 — plugin root closed

`ls -a "✏️s/🔌️plugins/🪐️space/"` now shows exactly: `🎛️apps`, `📦️packages`, `🗿️artifacts`,
`🦀️component.rs` (no `AGENTS.md`/`README.md` exist for this plugin — confirmed absent both before and
after, matches `📓️w0-b-plugin-shape.md`'s note "no AGENTS.md/README.md at all" for `🪐️space`). Root was
already closed after step 1 — no stray files to relocate.

### Step 3 — escape-hatch call site (`register_app_io`)

**Re-grepped fresh** (per the task brief's own warning that line numbers move) rather than trusting
the brief's named files verbatim, and rather than trusting `📓️w0-a-escape-hatch.md` blindly — both were
stale in different ways:

- The task brief named `🎮️commands/🖼️media/🦀️component.rs` and `📌️panels/🛍️catalogue/🦀️component.rs`
  as the two production call sites. **Neither is one.** `🖼️media/🦀️component.rs` has **zero**
  `register_app_io` calls at all (it calls `register_os_media_export_handler_kind` /
  `register_dwg_import_handler`, both inside `#[cfg(test)] mod tests` starting at line 103 — test
  fixtures for that file's own round-trip test, not production registration). `🛍️catalogue/🦀️component.rs:120`
  does call `register_app_io`, but it too sits inside `#[cfg(test)] mod tests` (starts line 107) —
  a test-seed helper (`seed_app`), not production code.
- The actual production call site — confirmed by re-grep and by cross-checking
  `📓️w0-a-escape-hatch.md`'s own §2c table row — is
  `🎛️apps/🪐️space/🎮️commands/🧭️navigation/🦀️component.rs:78`, inside `fn apply_app_registrations`
  (**not** gated by `#[cfg(test)]`; that file's test mod starts at line 102/79 after the edit — see
  below).
- Also test-only, left untouched: `🎛️apps/🪐️space/🦀️component.rs:605` (inside
  `#[cfg(test)] pub(crate) mod testkit`, starts line 554/551 after the edit).

**Why this one call site is architecturally different from the puzzle/demonstrator escape-hatch
violations**, and why I did not follow the census's literal suggested destination ("the app's own
artifact `⚙️engine` registration path"): `🪐️space`'s app `🦀️component.rs` module doc (lines 1-15,
unedited) states explicitly that this app **owns no artifact of its own** — its
`ArtifactApp::Snapshot`/`Mutation` are `semio_framework_os::WorkflowSnapshot`/`WorkflowMutation`,
owned entirely by os-core. There is no `🗿️artifacts/🪐️space` directory to relocate into (confirmed:
`find "✏️s/🔌️plugins/🪐️space/🗿️artifacts" -maxdepth 4 -type d` shows only `🏠️home`, never `🪐️space`).
Nor is this call registering IO for an artifact kind the plugin doesn't own (the puzzle/demonstrator
shape) — `register_app_io` populates this wasm instance's own private copy of
`semio_framework_os::APP_REGISTRATIONS` with *other plugins'* `AppDefinition`s, because each wasm
component statically links its own copy of os-core and never sees what a native host's
`PluginHost::load_plugin` populates. This is a documented (docstring preserved verbatim in the move)
cross-wasm-boundary registry-sync mechanism triggered by an explicit host push
(`SetAppRegistrations` command from `os-shell.tsx`), not a "kind ownership" violation and not
something that can become load-time-declarative — the host doesn't know what to push until runtime.

**The fix applied**: this app's own `⚙️engine/🦀️component.rs` **is** the documented artifact-engine
equivalent for this no-artifact app (its own module doc: *"headless compute … kept app-level since
this app owns no document-side `🗿️artifacts` node"*) — the correct, already-sanctioned destination.
Moved `apply_app_registrations` (private fn + its `AppRegistrationWireEntry` wire-shape struct) from
the command file into a new `//#region 🔖️AppRegistrations` in `⚙️engine/🦀️component.rs`, made it
`pub fn`, and left the command handler (`set_app_registrations::handle`) as a thin
parse-and-delegate call to `crate::apps::space::engine::apply_app_registrations(&payload.json)`. No
new artifact was invented, no new IO was authored — this is a same-file-family relocation of an
existing call, matching the shape §3 of the task brief asks for ("registration belongs to the
artifact['s stand-in], never to a command handler").

- **Updated** `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/⚙️engine/🦀️component.rs` — added
  `//#region 🔖️AppRegistrations` (struct `AppRegistrationWireEntry` + `pub fn apply_app_registrations`),
  added `register_app_io`, `AppDefinition` to the `semio_framework_os::{…}` import list, added
  `use serde::Deserialize;`.
- **Updated** `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🧭️navigation/🦀️component.rs` — removed
  the struct/fn (24 lines) and the now-unused `register_app_io, AppDefinition` /
  `use serde::Deserialize;` imports; `set_app_registrations::handle` now calls
  `crate::apps::space::engine::apply_app_registrations(&payload.json)`.

Post-fix `grep -rn "register_app_io" "✏️s/🔌️plugins/🪐️space/"` shows exactly one production call
(`⚙️engine/🦀️component.rs:242`), plus the two test-only call sites (`🦀️component.rs:605`,
`🛍️catalogue/🦀️component.rs:120`) and two prose mentions in the navigation file's doc-comments —
zero remaining production violations in an app/pane/panel/command/setup file.

### Step 4 — dependency purge (`semio-framework-os`) — NOT purged, analysed

Per the task's plugin-specific note, this was **not** force-purged. `Cargo.toml` already carries an
inline comment explaining `semio-framework-os` is deliberately `path =` (not `workspace = true`)
pending a separate "registrar handoff" wave that makes this crate a real workspace member — i.e. this
dependency's shape is already tracked infra debt, not something this wave should touch structurally.

Per `📓️w0-d-sdk-surface.md` §3.2, `🪐️space` uses **36 distinct symbols** from
`semio_framework_os` (feature `os-host-full`): `APP_REGISTRATIONS`, `DwgDrawing`,
`OS_HOME_VFS_ROOT_ID`, `OS_SPACE_SCHEMA`, `OsBackbonePort`, `OsMediaCapability`,
`OsMediaExportResult`, `OsParameter`, `OsParameterFieldBinding`, `OsParameterType`,
`OsSpaceCatalogEntry`, `OsWorkflowCamera`, `SpaceKind`, `SpaceVisibility`, `VcsError`, `Workflow`,
`WorkflowNode`, `WorkflowSnapshot`, `delete_os_space`, `dwg_to_bytes`, `empty_space_snapshot`,
`empty_workflow_snapshot`, `export_os_app_instance_media_kind`, `host`, `import_os_app_instance_media_kind`,
`import_os_space_from_dsl`, `list_os_space_catalog_entries`, `media_accept_filter_kinds`,
`open_file_space_backbone`, `open_folder_space_backbone`, `os_parameter_types_compatible`,
`os_workflow_to_flow_fixture`, `register_app_io`, `register_dwg_import_handler`, `validate_workflow`,
`workflow`. This includes the app's own `ArtifactApp::Snapshot`/`Mutation` types
(`WorkflowSnapshot`/`WorkflowMutation`, imported separately, also OS-host types) and whole modules
(`host`, `workflow`) — not narrow leaf functions. This is unambiguously not a clean removal; leaving
the dependency in place, filed under `## sharedFileRequests` below rather than forced here, exactly as
the task brief instructed for this plugin.

### Step 5 — inventory only (not fixed, per the Draft-lane prohibition and the "record, don't fix" instruction)

**Interior-mutable app state** (all `🎛️apps`, none of it Draft-lane-eligible without a framework
ruling — see below):

1. `🎛️apps/🪐️space/🦀️component.rs:115-116` — `shared_presence_peers()`:
   `static REGISTRY: OnceLock<Arc<Mutex<HashMap<String, HashMap<String, SPresencePeerLocal>>>>>` —
   a process-global presence-peer registry, written by `publish_presence` (line 136), read by
   `presence_peers_json` (line 120).
2. `🎛️apps/🏠️home/🦀️component.rs:73-74` — `shared_studio_ports()`:
   `static REGISTRY: OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>>` — a
   process-global backbone-port registry, written/read at lines 115/152/188.
3. `🎛️apps/🪐️space/🦀️component.rs:568` — `thread_local! { static STUDIO_TEST_APP: RefCell<SpaceApp> }`
   — **test-only**, inside `#[cfg(test)] pub(crate) mod testkit`.

Per `📓️w0-c-purity.md` §5, both `🪐️space` and `🏠️home`'s registries are independently flagged
**"large … genuinely hard to fit into a per-document Draft lane … likely needs a framework-level
answer (a real shared/ephemeral capability), not just 'move the field', flagged as UNVERIFIED whether
Draft is even the right target for this one."** I concur and left both alone — these are process-wide,
cross-session-lifetime shared registries (presence must survive per remote client; ports are looked
up by string key across the whole running process), not per-document/per-app-instance session scratch
of the kind the Draft-lane spec (`📓️draft-lane-spec.md`) targets. **Proposed Draft-snapshot
fields/verbs, if a future ruling decides Draft is still the right home**: none proposed — this is
exactly the "cannot be authored conformingly" case the draft-lane spec anticipates (§"Ruling 2":
*"leave its dispatch enum empty with no triad dirs and report it"*), except here the blocker is
architectural fit (shared-process registry vs. per-document draft state), not vocabulary. No code
touched.

**`std::fs`/`std::env`/`std::process`/`Command::new`/network calls outside `#[cfg(test)]`**: zero
hits — `grep -rn "std::fs::\|std::env::\|std::process::\|Command::new(" "✏️s/🔌️plugins/🪐️space/"
--include="*.rs"` returns nothing.

**`fn seed(` implementations**: zero — `grep -rln "fn seed(" "✏️s/🔌️plugins/🪐️space/" --include="*.rs"`
returns nothing (the repo's one `seed()` is in `🌿️vcs`, per `📓️w0-c-purity.md` §7, not this plugin).

**`LazyLock<()>` side-effect gate** (noted for completeness, not touched — not interior-mutable *app*
state, it's plugin-root registration, and it's already the sanctioned "run exactly once" idiom, not
Draft-lane material): `🦀️component.rs:24`, `static FIXTURES: LazyLock<()>` gating two
`register_os_fixture_json` calls inside `ensure_space_fixtures_registered()`.

## Files touched

- **Removed**: `✏️s/🔌️plugins/🪐️space/🛂️manifest/🦀️component.rs` (and the now-empty `🛂️manifest/` dir)
- **Removed**: `✏️s/🔌️plugins/🪐️space/🎟️capabilities/🦀️component.rs` (and the now-empty `🎟️capabilities/` dir)
- **Removed**: `✏️s/🔌️plugins/🪐️space/🔧️setup/🦀️component.rs` (and the now-empty `🔧️setup/` dir)
- **Updated**: `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/⚙️engine/🦀️component.rs`
- **Updated**: `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🧭️navigation/🦀️component.rs`

No other files created, moved, or removed.

## Verification

_(filled in below once the shared build-lock cleared — see pasted output)_

## sharedFileRequests

1. **File**: `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` — **region**: `[dependencies]`,
   the `semio-framework-os` line and its preceding comment block. **Reason**: cannot remove the
   `semio-framework-os` (OS HOST, forbidden-to-plugins) dependency without a curated
   `semio_framework_plugin` re-export for the 36 symbols listed in Step 4 above (source:
   `📓️w0-d-sdk-surface.md` §3.2, row `🪐️space`) — several are whole modules (`host`, `workflow`) or
   this app's own `ArtifactApp` associated types (`WorkflowSnapshot`/`WorkflowMutation`), not narrow
   leaf functions, so this is not a small ask. The Cargo.toml's own inline comment already flags this
   dependency as pending a "registrar handoff" wave that makes this crate a real root-workspace
   member — that same wave is the natural place to land the SDK re-export list, per the task brief's
   own instruction not to force this from W3. No patch file produced (nothing to apply yet — this is
   a request for the curated re-export list itself).

## Concurrent-churn observations

- The Step-0 baseline `cargo check -p semio-s-plugin-space` (and the Step-6 re-run) were both queued
  behind the shared build lock for an extended period with no forward progress reported
  (`ps` showed the process alive but at constant CPU time, consistent with "Blocking waiting for file
  lock on build directory" — normal per `📌️important.md` rule 5, not killed, not retried).
- `taxonomy.json`'s `pluginChildDirs` was already flipped to `["🎛️apps"]` by the time this wave
  started (commit `a445617cae`, `🐙️ueli🎆️26🌙️06☀️04🚩️493`, Aug 12 15:28) — ahead of the "flip is the
  LAST thing APA does" rule in `📌️important.md`, meaning either a coordinator wave already landed it
  or another concurrent APA session did. Not investigated further (out of this plugin's boundary);
  flagging only because it changed the correctness of Step 1's premise (I verified the current gate
  state directly rather than assuming staleness in either direction).
- No edits from any other session landed on any file this report touches during this session (checked
  via `git log --oneline -3 -- <path>` per file before editing).

## apa-status: partial

`register_app_io` (step 3) and the three dead facets (step 1) are resolved. Outstanding for a future
wave, all explicitly out of this wave's mandate: the `semio-framework-os` dependency purge (step 4,
blocked on an SDK re-export list — filed above), the two process-global `Mutex<HashMap>` registries
in `🎛️apps/🪐️space` and `🎛️apps/🏠️home` (step 5 inventory — blocked on a framework-level
shared/ephemeral-capability ruling, not a per-app Draft migration), and the taxonomy/policy/registry
gate updates for `pluginChildDirs` (already flipped ahead of this wave per the observation above —
worth a coordinator sanity check that this was intentional and not premature).
