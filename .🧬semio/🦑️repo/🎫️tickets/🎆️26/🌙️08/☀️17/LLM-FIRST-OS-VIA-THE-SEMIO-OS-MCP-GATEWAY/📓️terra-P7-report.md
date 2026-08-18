# 📓️ terra report — packet P7-headless-workspace

## 1. Preconditions

- Baseline `git rev-parse HEAD`: `abd29c08d0a04dd72d3b9c3fabe818c300c125c8`.
- SHA-256 of every file this packet created or edited (`shasum -a 256`, taken after the final edit):

| file | lines | sha256 |
|---|---:|---|
| `🌉️mcp/🏠️workspace/🦀️component.rs` (new) | 881 | `334f49025774db12ab1620f6c60e49839fcf27d8d1b952263897e0a7be1ced1d` |
| `🌉️mcp/📦️bin.rs` (edited) | 132 | `992fc132f8515a7e744fc6eb3444a632ca96957a7378f31b64b960af64337c1b` |
| `🌉️mcp/🦀️component.rs` (edited, mount + workspace-aware entrypoints) | 803 | `d60de838bcd16164954c2fe1631247aa6e17073e8043ab3f41e551332c546b91` |
| `🌉️mcp/📦️packages/🦀️rust/Cargo.toml` (edited) | 81 | `46dae55ddee695b44090a3e7b1b784ca129705d8701871558eca2c1aa88eeae9` |
| `🌉️mcp/📦️packages/🦀️rust/📦️glue.rs` (edited) | 63 | `a2f6ba1bcb00bb47b5fe78d641331ee8bd3882dbcd844b42f58541fd30488696` |

Nothing outside `path_scope` was edited. No git-modifying command was run at any point.

## 2. What was built

### `🏠️workspace/🦀️component.rs` (new, 881 lines, 6 regions)

- **`🔖️PluginPaths`** — `find_repo_root()`, `load_plugin_registry()` (reads the real generated
  `🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json`), `find_plugin_entry()`,
  `resolve_plugin_wasm_path()`. A documented **temporary seam** (see §5 leases) reimplementing the
  narrow ~30-line algorithm `🏃️run/📦️bin.rs`'s own `find_repo_root`/`resolve_plugin_paths` use —
  those are private to that binary's own separate `[[bin]]` crate root (confirmed by reading that
  file's own module doc: a `[[bin]]` target does not inherit the lib's `extern crate … as X`
  aliases), so nothing outside it could call them even if marked `pub`. Reads the SAME generated
  source of truth (`plugin-registry:generate`'s JSON mirror of the `.rs` `PLUGIN_WASM_ARTIFACTS`
  table `🏃️run` itself reads), not a guessed naming convention.
- **`🔖️Descriptor`** — `load_package_descriptor(owner_root)` reads a plugin's real, committed
  `🔣️descriptor.json` (E2's proof artifact) directly into `semio_framework::PackageDescriptor`,
  deliberately **not** by instantiating the wasm and calling `describe()` at runtime the way
  `semio-framework-plugin-describe` does at build time — that path is exactly what's blocked
  upstream in `🏃️run`'s own `WasmtimeNodeHost::load_runtime_recursive` (see §4).
- **`🔖️ProbeDocument`** — `ProbeSnapshot`/`ProbeDiff`/`ProbeMutation` (schema `os.agent.probe/v1`):
  a minimal, real, non-test `ArtifactDsl`/`ArtifactPack`/`Mutation`/`OpText`/`OpBinary` implementor
  this crate owns outright, used ONLY to exercise the real VCS + backbone pipeline
  (`store::ArtifactStore`, `store::sync::ArtifactHost`) end to end without pretending to understand
  any plugin's actual document schema (host-opaque by design — this file's own module doc explains
  why). `ensure_probe_codec_registered()` registers its `ArtifactCodec` once per process.
- **`🔖️Binding`** — `WorkspaceOrigin::{Folder, Hub}` wrapping `store::sync::PersistenceBinding`.
- **`🔖️PluginActivation`** — `activate_plugin_instance()`: real `WasmtimeRuntime::compile` →
  `instantiate` → `execute_turn(Event::InstanceOpen{..})`, returning the real `TurnResult`
  (status/effects/fuel) or an error. Every capability the descriptor requests is granted (narrowest
  grant that lets `InstanceOpen` proceed — real policy is P6's).
- **`🔖️ArtifactChannel`** — `PluginArtifactChannel` implementing `crate::actions::ArtifactChannel`,
  the exact seam **P6-actions-policy already defined** while landing in parallel
  (`🎬️actions/🦀️component.rs`'s own module doc literally names this as "P7's job when it implements
  `ArtifactChannel` for real"). `ReadHistory` is real: `Event::AppCommandEvent` in →
  `GuestRuntime::execute_turn` → scan `TurnResult.effects` for `Effect::Respond{req, result}` where
  `req.0 == seq` → `store::decode_app_frame` (the real channel's hand-rolled binary codec —
  `encode_app_command`/`decode_app_frame`, NOT `to_dsl_value`, which the compiler correctly rejected
  the first time I tried it since `AppCommand`/`AppFrame` derive only `Clone, Debug, PartialEq`, no
  serde). `PureCommand`/`TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo` return a well-formed,
  typed `"channel.not-wired"` `Fault` — see §4 for exactly why, and why fabricating an
  `ActionAddress{mode_id, window_kind_id, window_instance_id}` for a headless session would not be
  honest engineering.
- **`🔖️HeadlessWorkspace`** — the struct: `open_folder`/`open_hub` constructors,
  `workspace_artifact_ids()` (real `FolderSqliteStorage::document_ids()`),
  `read_artifact_bytes()` (real cold pack+spr read), `ensure_probe_artifact()` (real commit through
  `ArtifactStore` + `ArtifactHost::send`, idempotent), `attempt_plugin_activation()`,
  `open_artifact_channel()`. Implements `GatewayBackend` for real: `resolve_context` (real session +
  real active-artifact id + real catalog hash from a passed-in `Arc<Catalog>`),
  `search_capabilities`/`describe_capabilities` (delegate to the crate's own `search`/`Catalog::get`
  — reused, not reimplemented), `read_resource`/`list_resources` (real for `semio://workspace`,
  `semio://workspace/artifacts`, `semio://artifact/{id}[/history|/validation]`; `/schema` and
  `prepare_action`/`invoke_action` are typed `PLUGIN_UNAVAILABLE` — P6's mutation-protocol territory,
  explicitly not duplicated here). `impl GatewayBackend for Arc<HeadlessWorkspace>` (legal: the trait
  is local even though `Arc` is foreign) lets one live `Arc<HeadlessWorkspace>` serve as both the
  `GatewayBackend` `McpServer::new` takes and the object `context_resolve`'s override closure holds —
  never two divergent workspace instances.
- **`🧪️Tests`** — `mod quick` (8 tests: folder creation, empty listing, idempotent seeding, context
  resolution, real resource bytes, not-fabricated 404, `PLUGIN_UNAVAILABLE` shape, base64/registry
  edge cases) + `mod long` (2 tests: real cross-host backbone propagation over a real folder;
  real note-wasm activation attempt, self-skipping with a clear message if the registry/wasm aren't
  present rather than fabricating a pass).

### `🌉️mcp/🦀️component.rs` (root, edited)

Deliberately touched beyond pure "mount the facet" — the brief's own §2/§3.1 requires "wire
`--folder`/`--hub` flags to a real workspace", and the wiring has to live somewhere between `bin.rs`'s
argv parsing and the existing `build_server_with_principal`/`run_stdio`/`run_http` this file already
owns. What changed, precisely:
- `pub use crate::workspace::*;` (the actual "mount").
- `build_server_with_workspace(principal, audit, workspace: Arc<HeadlessWorkspace>, channel)` — an
  **additive twin** of `build_server_with_principal`, not a modification of it (that function's
  3-argument shape has live callers in P6's own in-flight tests; I did not touch it). Reuses
  `build_catalog`/`build_tool_registry`/`ActionAdapter::new` unchanged, then overrides just the
  `context_resolve` tool entry (`InMemoryToolRegistry::register` overwrites by name — verified by
  reading its `HashMap::insert`-based implementation) to answer from the live workspace instead of
  the backend-independent default, and passes the workspace as the real `GatewayBackend`.
- `server_for_workspace_options()` — `--folder` → `HeadlessWorkspace::open_folder` + a real
  `PluginArtifactChannel` targeting `note` (falls back to `MockArtifactChannel` with a clear stderr
  diagnostic if the registry/wasm aren't resolvable — never a silent downgrade); `--hub` → the hub
  binding equivalent; neither given → byte-identical old behavior
  (`build_server_with_principal`/`NullBackend`/`MockArtifactChannel`), so every pre-existing
  P1a/P1b/P2/P6 test is unaffected.
- `StdioOptions`/`HttpOptions` gained a `hub: Option<HubOptions>` field; `run_stdio`/`run_http` now
  call `server_for_workspace_options` instead of hardcoding `MockArtifactChannel::new()`.

### `📦️bin.rs` (edited)

`--hub <url> --space <id> [--token <t>]` argv parsing for both `stdio` and `http` modes (mutually
exclusive with `--folder`, checked and rejected with a clear message). `http`'s own required bearer
`--token` is disambiguated from the hub's own token by parse order (documented in the file's own
module doc): the first `--token` seen binds the bearer, any later one binds the hub.

### `Cargo.toml` / `📦️glue.rs` (edited)

Added `semio-framework-os-kernel` (`sync` feature — mounts `store::sync::ArtifactHost` etc.),
`semio-framework-actor` (`ActorId`/`RuntimeActorId` — not re-exported by `semio-framework-plugin-host`,
verified), `framework_hash`, and (native-only, mirroring `🏃️run`'s own
`[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` gate) `semio-framework-plugin-host`. All
four use the SAME relative paths `🏃️run/📦️packages/🦀️rust/Cargo.toml` uses (this crate sits at the
identical directory depth). `glue.rs` gained `extern crate semio_framework_os_kernel as store;`
(a single alias, not also `dsl`/`protocol` the way `🏃️run` does — this crate already owns the name
`protocol` for its own P1a JSON-RPC facet, so a second `protocol` alias would collide; every item
`🏠️workspace` needs is reachable through the one `store` alias via that crate's own root glob
re-exports, verified by reading its `📦️glue.rs`) and `pub mod workspace`.

## 3. Real runtime APIs consumed (with paths)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` — `ArtifactHost::{new, open,
  subscribe, send, open_artifacts}`, `PersistenceBinding::{Folder, Hub}`, `ArtifactActorConfig`,
  `ArtifactActorMsg::LocalMutations`, `ArtifactEvent::RemoteMutations`, `FolderSqliteStorage::{new,
  read, document_ids}`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `ArtifactStore::{new, dispatch,
  attach_backbone, snapshot, applied_edit_ids, tick}`, `create_document_envelope`, `ArtifactDsl`,
  `ArtifactPack`, `ArtifactCodec::of`, `register_document_codec`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` — `Mutation<P>`,
  `MutationDiff<P>`, `MutationOutcome::new`, `OpText`, `OpBinary`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` — `AppCommand::{ReadHistory}`,
  `AppFrame::HistorySnapshot`, `encode_app_command`, `decode_app_frame` (the real hand-rolled binary
  wire codec — not serde).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust` (`semio-framework-plugin-host`)
  — `GuestRuntime::{compile, instantiate, execute_turn, drop_instance}`, `WasmtimeRuntime::new`,
  `SharedEngineConfig`, `PackageRef`/`PackageId`/`PackageHash`.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (via `semio_framework::kernel`) — `Event::{InstanceOpen,
  AppCommandEvent}`, `Effect::Respond`, `RequestOutcome`, `Budget`, `BrokerCapabilityGrant`,
  `PluginInstanceId`, `AppInstanceId`, `HistoryPatch`.
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` — `ActorId::new` (bit-packed
  `plugin_ordinal:kind:ordinal:generation`).
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — `PackageDescriptor` (read directly from the
  committed `🔣️descriptor.json`, not decoded from wasm), `PluginManifest`, `AppDefinition`, `AppRef`,
  `AppRole`, `ArtifactDialect`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎬️actions/🦀️component.rs` (P6, landed in parallel) —
  `ArtifactChannel` trait, `AppCommand`/`AppFrame`/`Fault`/`PreparedOps`/`MutationOrigin` (P6's own
  narrow port types — implemented, not modified).

## 4. Live transcript — what actually happened, honestly

**The official acceptance commands (§5 of the brief) could not be run to completion.** Both
`cargo test -p semio-framework-os-mcp` and `cargo build -p semio-framework-os-mcp --bin semio-os-mcp`
fail with the SAME 4 `error[E0308]` in `🌉️mcp/🗂️catalog/🦀️component.rs:615/618/621/624` — verbatim
transcripts at `🧪️p7-acceptance-test2.txt` and `🧪️p7-acceptance-build.txt` in this ticket folder.

**This is not this packet's bug.** `git log --date=iso` shows `🛂️manifest/🦀️component.rs`'s most
recent commit is AFTER `🗂️catalog/🦀️component.rs`'s own last commit: the peer ticket's E1/E2 packets
retyped `ContributionSet.{inference_services,mutation_services,io_entries,composer_entries}` from
the old placeholder `Vec<manifest::DescriptorEntry>` to real
`Vec<ContributedInferenceMetadata>`/`Vec<ContributedMutationMetadata>`/`Vec<IoEntryDescriptor>`/
`Vec<ComposerEntryDescriptor>`, and `🗂️catalog`'s own `capability_from_contribution` (P2, closed) was
never updated to match. **Two other packets in this same ticket independently hit and leased the
exact same defect before I did** — `.../📓️terra-P8-report.md` §4 (first to flag it) and
`.../📓️lease-P6-catalog-contribution-types.md` — confirmed by reading both. I filed a third,
cross-referencing lease (`📓️lease-P7-catalog-contribution-types.md`) with the exact per-type fix
(field-level, not guessed — verified against the real struct definitions) since mine is the only one
that also lands the precise recipe. Re-checked repeatedly through the session (`🧪️p7-check1.txt`
through `🧪️p7-check4.txt`, plus two more unlogged re-runs) — still unresolved as of this report.

**What IS verified, compiler-checked, real:** every error introduced by this packet's own code was
found and fixed through 4 rounds of `cargo check -p semio-framework-os-mcp --lib`
(`🧪️p7-check1.txt` → `🧪️p7-check4.txt`) until the ONLY remaining errors were the 4 pre-existing
`🗂️catalog` ones and the ONLY remaining warnings were pre-existing ones in `🧭️protocol` (dispatcher,
unrelated `unnecessary qualification`s already there) and the peer-owned `📡️spr/📡️wire` one the brief
itself names as not mine. `🏠️workspace/🦀️component.rs` and this packet's edits to `🦀️component.rs`/
`bin.rs`/`Cargo.toml`/`glue.rs` compile with **zero errors and zero warnings** of their own — the
last clean checkpoint is captured at the top of `🧪️p7-acceptance-test2.txt` (4 pre-existing
`unnecessary qualification` warnings in `🧭️protocol`, then the 4 pre-existing `🗂️catalog` errors,
nothing from `🏠️workspace`).

**Consequence for the demo.** I cannot paste a live `context_resolve` JSON-RPC transcript against a
running `semio-os-mcp stdio --folder …` process, because the binary cannot be built right now — not
because the workspace/channel code is unproven, but because the crate it links into cannot compile
until a DIFFERENT packet's already-leased, already-triple-confirmed bug is fixed. This is exactly the
brief's own explicit escape hatch ("if the wasm cannot be produced in reasonable time, say so
explicitly, show how far you got, and land the workspace with its unit tests rather than claiming an
unproven end-to-end") — except the actual blocker here is one layer up from wasm production: a sibling
source file, not owned by this packet, not fixable by this packet under `📌️important.md` rule 3.

What I DO have, real and re-checked:
- A real `.wasm` build exists on disk right now: `target/wasm32-wasip2/debug/semio_s_plugin_note.wasm`
  (43 096 083 bytes, built by another session, `stat` confirms — not mine to have built or claim
  credit for producing).
- `🏠️workspace/🦀️component.rs`'s own `mod quick`/`mod long` tests exercise the REAL `ArtifactHost`,
  `ArtifactStore`, `FolderSqliteStorage`, and (in `attempt_plugin_activation_against_a_real_note_wasm_when_available`)
  the real `WasmtimeRuntime` against that real wasm file — these compiled cleanly through every
  `cargo check` run in §above; they could not be *executed* (`cargo test`) because the crate itself
  cannot link until the catalog.rs fix lands. Once it does, re-running
  `CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp` is the single command
  that turns this from "compiler-verified" to "test-run-verified" — nothing else in this packet's
  own code needs to change first.

## 5. Leases filed

### (a) `🏃️run` `🔖️PluginPaths` extraction — `…/📓️lease-P7-plugin-paths.md`

```markdown
# 📓️ lease-request — P7-headless-workspace → 🏃️run `🔖️PluginPaths` extraction

**Ask**: extract `🏃️run/📦️bin.rs`'s private `find_repo_root()`/`resolve_plugin_paths(repo_root,
plugin_ids)` into a new `//#region 🔖️PluginPaths` in `🏃️run/🦀️component.rs` (the LIBRARY, not
`📦️bin.rs` — a `[[bin]]` target is its own crate, nothing outside it can call these even if `pub`),
`pub fn`, so every consumer (this crate included) calls ONE function instead of each maintaining its
own copy.

**What we did in the meantime**: `🌉️mcp/🏠️workspace/🦀️component.rs`'s own `//#region 🔖️PluginPaths`
reimplements the SAME algorithm independently, resolving the plugin-id → wasm-path mapping through
the JSON MIRROR of the data `🏃️run/📦️bin.rs` itself reads
(`🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json` — same `plugin-registry:generate` nx target's
output as the `.rs` `PLUGIN_WASM_ARTIFACTS` table `🏃️run` reads via `include!`), not a second copy of
the `.rs` table. Once this lease lands, our own `find_repo_root`/`load_plugin_registry`/
`find_plugin_entry`/`resolve_plugin_wasm_path` should be deleted and replaced with direct calls into
`🏃️run`'s new public API.

**Status**: pending.
```

### (b) `🗂️catalog` contribution-type break — `…/📓️lease-P7-catalog-contribution-types.md`

```markdown
# 📓️ lease-request — P7-headless-workspace → P2-catalog `capability_from_contribution` typing break

Cross-references `.../📓️terra-P8-report.md` §4 (first to flag) and
`.../📓️lease-P6-catalog-contribution-types.md` (second) — same defect, independently confirmed a
third time.

**The exact fix needed** (verified field shapes): `capability_from_contribution`'s 4 call sites
(`🗂️catalog/🦀️component.rs:615/618/621/624`) pass `&ContributedInferenceMetadata` /
`&ContributedMutationMetadata` / `&IoEntryDescriptor` / `&ComposerEntryDescriptor` where the function
still expects `&manifest::DescriptorEntry`. None of the four real types has a `DescriptorEntry`-shaped
`.id`. Per type: `ContributedInferenceMetadata` → `format!("{}#{}", contributor, inference_schema)`;
`ContributedMutationMetadata.mutation_id` → use directly (already the right id shape);
`IoEntryDescriptor` → `format!("{}->{}", owner.to_coordinate(), counterpart.to_coordinate())`;
`ComposerEntryDescriptor` → `writes.to_coordinate()` (a composer entry's identity is what it writes).

**Status**: pending as of this report — blocks `cargo test -p semio-framework-os-mcp` for every
packet in this ticket, not just P7 (confirmed: P6, P7, P8 all independently hit it).
```

## 6. What is still stubbed, and why

| area | status | why |
|---|---|---|
| `prepare_action`/`invoke_action` (`GatewayBackend` trait methods) | typed `PLUGIN_UNAVAILABLE`, retryable | The real 2-phase transaction protocol is `P6-actions-policy`'s owned territory (§2 of the brief). P6 built its own, better-scoped seam (`ArtifactChannel`) for exactly this — implementing it twice, differently, in the SAME packet would be duplicated/conflicting logic, forbidden by CLAUDE.md. |
| `PluginArtifactChannel::exchange` — `PureCommand`/`TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo` | typed `"channel.not-wired"` `Fault` | Real construction needs `manifest::ActionInvocation{address: ActionAddress{mode_id, window_kind_id, window_instance_id, ..}, arguments}` — every one of those three address fields beyond `plugin_id`/`app_id` is normally supplied by a LIVE window/mode a headless gateway has none of. Inventing placeholder values is a real, undecided design question (what IS a headless agent's "window"/"mode"?), not something this packet is positioned to settle unilaterally. `ReadHistory` needed no such invention (no `ActionAddress` in its shape) and is real. |
| `read_resource("semio://artifact/{id}/schema")` | typed `PLUGIN_UNAVAILABLE`, retryable | Needs a live plugin instance's `describe()`/manifest resolved per-artifact-kind; not reachable generically without the same `ActionAddress`-shaped gap above. |
| Hub binding (`workspace_artifact_ids`, `read_artifact_bytes` for `WorkspaceOrigin::Hub`) | honestly returns empty/`None`, not fabricated | Enumerating a remote hub space's documents needs a REST listing endpoint P4/hub territory owns; `PersistenceBinding::Hub` itself is wired and real (`ArtifactHost::open` accepts it identically to `Folder`), only the "list what's there" convenience is unimplemented for that binding. |
| Live end-to-end JSON-RPC transcript | not captured | Blocked by the pre-existing, triple-confirmed, leased `🗂️catalog` breakage (§4) — not a gap in this packet's own code, which compiles clean. |
| `read_artifact_resource(.../schema)` for the folder-listed generic artifact bytes | typed `PLUGIN_UNAVAILABLE` | Same class as the `/schema` row above. |

## 7. Files touched (for `ticket_close`, sol's own call — not mine)

Created: `🌉️mcp/🏠️workspace/🦀️component.rs`; `📓️sol-P7-headless-packet.md`,
`📓️lease-P7-plugin-paths.md`, `📓️lease-P7-catalog-contribution-types.md`, this report, and scratch
`.txt` evidence files (`🧪️p7-check1.txt` … `🧪️p7-check4.txt`, `🧪️p7-acceptance-test.txt`,
`🧪️p7-acceptance-test2.txt`, `🧪️p7-acceptance-build.txt`) in this ticket folder.
Edited: `🌉️mcp/📦️bin.rs`, `🌉️mcp/🦀️component.rs`, `🌉️mcp/📦️packages/🦀️rust/Cargo.toml`,
`🌉️mcp/📦️packages/🦀️rust/📦️glue.rs`. Nothing outside `path_scope` was touched; no git-modifying
command was run; no `[DEBUG] ` markers left in owned paths (grep-verified, §4-adjacent check in this
session).

## post-unblock fixes

sol fixed `🗂️catalog/🦀️component.rs` (replaced the untyped `DescriptorEntry` helper with a
`ContributionRow` trait) and the crate compiled for the first time. `cargo test -p
semio-framework-os-mcp` then ran and reported 151 passed / 5 failed (3 mine, 2 P6's — P6's fixed in
parallel, not touched here). All 3 of mine were real, first-ever-executed signal — investigated and
fixed for real, not papered over.

### 1. `resolve_context_reports_the_open_probe_artifact_as_active` — `left: None, right: Some("probe-b")`

**Root cause**: `workspace_artifact_ids()`/`read_artifact_bytes()` only ever did a COLD read through
`FolderSqliteStorage` (disk). `ensure_probe_artifact()` — the only writer — populates
`self.open_probes` (an in-memory map) and wakes `ArtifactHost`'s actor via
`ArtifactActorMsg::LocalMutations`, but that actor persists to `<folder>/.semio/documents.db`
**asynchronously on its own thread**. `ensure_probe_artifact` returns as soon as the LOCAL
`ArtifactStore::dispatch` call applied the mutation — before the actor thread has necessarily
flushed anything to disk. The reader (cold disk read) and the writer (in-memory map + async actor)
were never the same data source, exactly as sol's diagnosis predicted: "the registry the writer
populates and the one the reader consults are not the same."

**Fix**: `workspace_artifact_ids()` now unions `self.open_probes`' keys (the SAME map the writer
populates, read under the same lock, zero cross-thread hop) with whatever is already durably
persisted on disk. `resolve_context`'s `active_artifact_id` derives from this union, so it now sees
`"probe-b"` immediately, synchronously, with no race window at all.

### 2. `read_resource_artifact_returns_real_bytes_after_a_commit` — `NotFound("no such artifact: probe-c")`

**Same root cause as (1)**, same fix's other half: `read_artifact_bytes()` now checks
`self.open_probes` first — if a live `ProbeStore` is open for `artifact_id`, it returns REAL
in-memory bytes via `ArtifactStore::snapshot_pack()` (no cross-thread wait, no disk round trip at
all) — and only falls back to the cold `FolderSqliteStorage` read when nothing is open in-process.
Both fixes are additive reads (union / try-live-first-then-cold), not new writes — the on-disk
persistence path (and the backbone attachment that drives it) is completely unchanged.

### 3. `a_headless_commit_propagates_to_a_second_host_on_the_same_folder` — `Ok(Err(Closed))`

**Two distinct bugs, found in sequence — sol's hint ("very likely a dropped handle/guard... find
what closes") was exactly right for the first one.**

**Bug 3a — the actual `Closed`, a bug in the TEST, not in propagation.** The test called
`shell_host.subscribe("shared-doc")` **before** `shell_host.open(...)`. `ArtifactHost::subscribe`'s
own doc states the exact behavior verbatim: "If the document is not open the receiver's sender is
dropped, so it simply reports closed." Subscribing to a not-yet-open id hands back a receiver paired
with a sender (`let (_tx, rx) = broadcast::channel(1);`) that is dropped in the very same statement —
permanently closed from the first poll. `open()` then mints a **brand-new** `broadcast::channel` for
the same id; a receiver taken out beforehand can never see it. Fix: reordered `subscribe` to happen
**after** `open` (one line moved, `🏠️workspace/🦀️component.rs`'s `long` test module). This alone
turned `Ok(Err(Closed))` into a genuine `Err(Elapsed(()))` — i.e., the channel now stays open and the
only remaining question was propagation *timing*, not propagation *existence*.

**Bug 3b — real, benign timing, not a wiring gap.** With 3a fixed, the property largely worked but
occasionally still missed a 5s window. Instrumented with temporary `[DEBUG]` tracing (removed before
this report) across 10 runs: the disk-write wait **never once** timed out (the agent's commit always
reached `.semio/documents.db` well under 1s); of 10 runs waiting on `shell_events.recv()` after a
deterministic `ArtifactActorMsg::ExternalChanged` poke, 9 delivered `RemoteMutations` in well under
1s and 1 hit the 5s deadline. `ps aux` at the time showed several dozen concurrent `cargo`/`rustc`
processes from the sibling `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket's W3/W4 plugin fan-out —
real OS-thread scheduling contention on a heavily shared box, not a dropped/never-fired event. This
is the SAME reason `🏪️store/🔄️sync/🦀️component.rs`'s own reference test
(`folder_external_edit_delivers_remote_operations`) explicitly pokes `ExternalChanged` rather than
trusting the `notify` watcher's timing ("notify also wired, but timing-independent here" — that
test's own comment) — I adopted the identical, already-established pattern rather than inventing a
new one. Fix: added the same deterministic poke (after confirming the write really landed on disk,
polling `FolderSqliteStorage::read` the same way the reference test does) and widened the
post-poke wait from 5s to 20s, with the investigation numbers above recorded in the code comment so
the reasoning survives — not a blind bump to hide a `Closed`, which was already fixed by 3a and is a
categorically different failure mode (`Closed` fires instantly, on the very first poll; `Elapsed`
only after the full deadline, and only under measured external load).

**The property itself is real and proven, not merely tested-around**: a headless agent's commit
(`HeadlessWorkspace::ensure_probe_artifact`, going through the real `ArtifactStore` →
`ArtifactHost::send` → folder-persisted pack+spr path) reaches a second, independent
`ArtifactHost`/`ArtifactStore` pair watching the same folder, delivered as a real
`ArtifactEvent::RemoteMutations`, ingested via `store.tick()`, visible in the second store's own
`snapshot()`. This is exactly §6 of the packet brief.

### Verification — verbatim, exit codes

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp workspace::
running 11 tests
test workspace::quick::find_plugin_entry_reports_a_typed_not_found_for_an_unknown_plugin ... ok
test workspace::quick::base64_encode_matches_a_known_vector ... ok
test workspace::quick::open_folder_creates_the_directory_if_missing ... ok
test workspace::quick::prepare_action_and_invoke_action_are_a_well_formed_plugin_unavailable_not_a_panic ... ok
test workspace::quick::read_resource_on_an_unknown_artifact_is_not_found_not_fabricated ... ok
test workspace::quick::a_fresh_folder_workspace_lists_zero_artifacts ... ok
test workspace::quick::resolve_context_reports_the_open_probe_artifact_as_active ... ok
test workspace::quick::read_resource_artifact_returns_real_bytes_after_a_commit ... ok
test workspace::quick::ensure_probe_artifact_seeds_a_real_revision_and_is_idempotent ... ok
test workspace::long::a_headless_commit_propagates_to_a_second_host_on_the_same_folder ... ok
test workspace::long::attempt_plugin_activation_against_a_real_note_wasm_when_available ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 149 filtered out; finished in 4.98s
```
Full transcript: `🧪️p7-final-workspace2.txt`.

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
test result: ok. 160 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 56.07s
$ echo $?
0
```
Full transcript: `🧪️p7-final-full.txt` (includes `bin.rs` unit tests — 0, none declared — and doc-tests —
0). Every pre-existing P1a/P1b/P2/P6 test plus every `🏠️workspace` test is green; nothing in this
ticket's `semio-framework-os-mcp` crate is failing or ignored.

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp
$ echo $?
0
$ grep -c "^warning" <build output>
2   (both lines belong to ONE warning — `📡️spr/📡️wire`'s pre-existing `pos` assignment, named in
     §6 of the brief as not mine — plus its own "generated 1 warning" summary line)
```
Full transcript: `🧪️p7-final-build.txt`.

### Files touched by this fix pass

`🏠️workspace/🦀️component.rs` only (the 2 read-path fixes + the `long` test's subscribe-order and
poke fixes). No other file in `path_scope` needed a change. `🧪️p7-postunblock-1.txt` through
`🧪️p7-postunblock-3.txt` and `🧪️p7-final-workspace.txt`/`🧪️p7-final-workspace2.txt`/
`🧪️p7-final-full.txt`/`🧪️p7-final-build.txt` are the scratch evidence trail for this pass.

## P7b — wasm linker

### What was driven, live, verbatim

```
$ BIN=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target/debug/semio-os-mcp
$ WORK=$(mktemp -d /tmp/semio-agent-space.XXXX)
$ printf '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"action_prepare","arguments":{"capabilityId":"cad.editor.translateSelection","input":{"dx":1.0}},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | "$BIN" stdio --folder "$WORK" --scopes artifact.read,artifact.write
{"jsonrpc":"2.0","id":1,"result":{"content":[{"text":"Internal: instantiating `note`: wasmtime: component imports instance `wasi:io/poll@0.2.9`, but a matching implementation was not found in the linker","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INTERNAL","details":null,"message":"Internal: instantiating `note`: wasmtime: component imports instance `wasi:io/poll@0.2.9`, but a matching implementation was not found in the linker","retryable":false}}}
```
Reproduced exactly as sol reported — identical error text, identical shape.

### Question 1 — is the fix ours to add, or an existing plugin-host entry point?

**Neither, precisely: there is no existing entry point that wires WASI, and the fix cannot be made
from `🏠️workspace/**` at all — it has to land in `semio-framework-plugin-host` itself.**

Verified, not assumed:
- `world actor`'s own WIT (`🔌️plugin/🧬️schema/📜️component.wit:816`) declares exactly one import
  (`pure`) — no WASI import anywhere in our own world. The `wasi:io/poll@0.2.9` requirement comes
  transitively from the Rust `wasm32-wasip2` target's own runtime — every real component built for
  that target needs a full WASI Preview 2 linker regardless of its own WIT world, a well-known
  wasmtime/wasm32-wasip2 characteristic, not a defect in the descriptor or the WIT.
- `wasmtime-wasi = "22.0.1"` **is already a declared dependency** of `semio-framework-plugin-host`
  (and, unused, of `semio-framework-plugin-describe` too) — but a repo-wide grep for
  `wasmtime_wasi::|WasiCtx|WasiView|add_to_linker` across the entire `🧰️framework` tree returns ZERO
  matches outside those two `Cargo.toml` dependency lines. Nobody has ever wired it. `ActorHostState`
  (`🔌️plugin/🖥️host/🦀️component.rs:692`) has no `WasiCtx`/`ResourceTable` fields and no `WasiView`
  impl; `WasmtimeRuntime::new`'s linker only calls
  `actor_bindings::semio::framework::pure::add_to_linker`.
- `🏠️workspace/🦀️component.rs` already calls the ONE shared entry point that exists
  (`semio_framework_plugin_host::WasmtimeRuntime::new`/`instantiate`/`execute_turn`) rather than
  constructing a second `Linker` — that half of the architecture was already right; there was nothing
  to "reuse" that isn't already being reused, because the WASI wiring itself has never been written
  by anyone, anywhere in this tree.
- `WasmtimeRuntime`'s `linker: Linker<ActorHostState>` field and `ActorHostState` struct are both
  **private** to `🔌️plugin/🖥️host/🦀️component.rs` — there is structurally no way to inject additional
  linker imports from outside that file. The fix cannot be made from `🏠️workspace/**` regardless of
  how it's written, and that file is explicitly listed in `📌️important.md`'s collision table as B1's
  (peer ticket) territory, not this packet's.

**Action taken**: filed `…/📓️lease-P7b-wasi-linker.md` (this ticket folder) with the exact fix,
version-verified against the pinned `wasmtime-wasi = "22.0.1"` source directly
(`~/.cargo/registry/src/…/wasmtime-wasi-22.0.1/src/{lib,ctx}.rs` — the real `add_to_linker_sync::<T:
WasiView>` signature and `WasiView { ctx, table }` shape, not generic docs that drift across
versions) — `ActorHostState` gains `wasi_ctx: WasiCtx` + `resource_table: ResourceTable`, a
`WasiView` impl, `wasmtime_wasi::add_to_linker_sync(&mut linker)` alongside the existing `pure`
linker call, and a sandboxed-default `WasiCtxBuilder::new().build()` (no inherited stdio/fs/network —
matches the plugin host's own "capability-gated imports" security stance; none of `pure`'s three
functions need real WASI access). Cross-posted a copy to the peer ticket's own folder
(`…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️lease-from-LLM-FIRST-OS-P7b-wasi-linker.md`) per
`📌️important.md`'s own instruction that a peer-owned-file change "is a lease-request posted to their
ticket folder."

### Question 2 — does a real turn complete once instantiation succeeds?

**Cannot be answered yet, and I am saying so plainly rather than claiming otherwise: instantiation
itself does not succeed today**, blocked entirely on the lease above. Nothing downstream of
`WasmtimeRuntime::instantiate` (the `Event::InstanceOpen` turn, and — the actual target of
`action_prepare` — a real `PureCommand` round trip) can be exercised until it lands. Recorded here so
this is re-checked, not forgotten, the moment the lease is applied:
- `activate_plugin_instance` (`🏠️workspace/🦀️component.rs`) already submits a real
  `Event::InstanceOpen` turn immediately after `instantiate` succeeds — that half is written and
  compiler-verified, just unexercised until instantiation itself works.
- `PluginArtifactChannel::exchange`'s `PureCommand` arm (what `action_prepare`/`action_invoke`
  actually need for a real preview) is **still** the honestly-stubbed `"channel.not-wired"` gap this
  report's §6 already named — constructing a real `manifest::ActionInvocation{address:
  ActionAddress{mode_id, window_kind_id, window_instance_id, ..}}` for a headless session with no
  live window/mode remains undecided design work this packet has not invented an answer for. So even
  once instantiation is fixed, `action_prepare` will not return a genuine `PreparedActionReport` from
  this path yet — it will still surface `PLUGIN_UNAVAILABLE`/`"channel.not-wired"`, now for a
  different, more specific reason than "no backend wired." This is the honest, current ceiling of
  what P7 can prove end-to-end, and it is unchanged by this packet — only the instantiation floor
  moved.

### Test suite

`cargo test -p semio-framework-os-mcp` re-run after the WASI investigation; verbatim output and exit
code recorded once the shared build lock (contended with P1c's own concurrent
`cargo test -p semio-framework-os-mcp` run against the same `CARGO_TARGET_DIR`, confirmed via `ps
aux` — both our sessions legitimately share this ticket's target dir) clears. No file in
`🏠️workspace/**`/`📦️bin.rs`/root `🦀️component.rs`/`Cargo.toml`/`📦️glue.rs` was touched for P7b — this
was a pure diagnosis-and-lease packet, so the 160/0 baseline from the prior section is not expected
to regress; confirming that with a fresh run is the only remaining step.
