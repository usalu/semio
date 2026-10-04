# S4-BUMP — Final Channel-Bump Wave (design §20.7) + Store Description Wave

Agent: S4-BUMP (Opus executor), session 4, 2026-10-04. Brief: coordinator message (blocks A frame, B store description, C verify, D notify).
Scratch: `🗑️generated/s4-bump/`.

## Session 4 — 2026-10-04

### 0. Findings before editing (02:05)

- Channel file is now 3242 lines (the closure report's line numbers are stale); every anchor re-located from disk.
- `AppFrame::TransactionProposal.description` (tag 15): read by no host. Rust: only the `in_reply_to` arms in `🏃️run` and
  `🌉️mcp/🏠️workspace`; TS: only the codec in `💻️os/🟦️.ts`. The React `TransactionCoordinator` input type `TransactionProposal`
  (`🔌️PluginRuntime`) is a host-side type built by tests only, never from the frame. → deleted with `coalesce_key`.
- The agent `transaction_commit` label is NOT host-side state: it rides the frame `AppCommand::TransactionPrepare.label` (tag 17,
  host → guest; Rust host coordinator `🖥️host` `MemberDraft.label`, React `transactionPreparePlanned`, MCP workspace mapping), lands in
  the guest's `PendingTransaction.label`, becomes `Edit.description` AND the live command-log row label (`LocalizedLabel::data(label)`,
  a hand-written non-localized row label, §20.6 breach). Since block B deletes this path, the frame field rides THIS bump (§20.7: one
  bump for every frame-layout change) — otherwise B leaves a dead frame field that needs a second bump.
- `AppCommand::LoadDocument`: zero senders (MCP + `🏃️run` drive `DocumentArchiveLoadHost`); remaining = variant, paged decoder states
  `LoadDocumentPack/Spr`, field-stage arms, encoder/decoder, exhaustive `seq` arms (`🏃️run` `app_command_seq`, MCP `app_command_seq_mut`),
  guest arm, TS twin (`AppChannelClient.loadDocument` + document cache), tests, docs. `Effect::LoadDocument` (guest → host effect) and the
  `🎠️kernel` effect enum are a different thing and stay.
- Channel version authority: pin `💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json` + generator `channel-version generate --guest`
  (`🧑‍💻dev`), census `channel-version check` (baseline 02:00: pin 20, 36 consumers, 0 findings). Eight `derived` consumers (directory
  open-plan/browser-open/lease fixtures, hub trusted-catalog fixtures, gis frozen binding) are refused by the generator and must be
  re-derived (the v20 bump did this in `WGPU-RENDERER-REACT-PARITY/📓️sol-channel-v20-fixture-reconciliation.md`). The catalog-publication
  fixture declares `hostileValues: [19, 21]` — 21 becomes the pin, so its hostile row must move to 22.

### Progress

(milestones appended below)

#### Milestone 1 — wave A source + version bump (03:40)

Coordinator scope addition (02:4x, agreed with S4-LOAD): `AppCommand::ReadChildHeads { seq }` (tag 42) → `AppFrame::ChildHeads { in_reply_to,
entries: Vec<ChildHeadPackEntry> }` (tag 32, count bounded by `DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS`), `AppCommand::PureCommand { seq, command, head }`
(six lane fields deleted; `head` = HEAD snapshot pack, empty = live document, design §20.8).

Landed (all callers in the same wave, rule 39):
- Rust channel `📡️spr/🧵️channel/🦀️.rs`: `LoadDocument` variant/paged states/field stages/encoder/decoder removed (tag 6 unassigned, refused as
  unknown by both decoders), `TransactionPrepare.label` removed (route plan 17 → (3, ops, 2), field stages renumbered), `TransactionProposal.
  {description, coalesce_key}` removed, ReadChildHeads/ChildHeads added (paged state, rejected-fields arms; PureCommand now has a rejected-fields arm
  too — before, a PureCommand with trailing bytes hit `unreachable!`), `PURE_COMMAND_FIELDS` 7 → 2. Docs that named `LoadDocument` reworded.
- Guest `🔌️plugin/🦀️.rs`: proposal literal, LoadDocument arm + `plugin_load_document_pack` (+ re-export) deleted, TransactionPrepare arm and
  `transaction_prepare` trait/impl without label, `PendingTransaction.label` deleted, `transaction_commit` records the row label from its first leaf
  (`SemanticMutation::label`, the reload rule) and dispatches `description: None` (store field goes in wave B), testkit transaction helpers lost
  `label`; ReadChildHeads arm (→ `child_head_packs()`), PureCommand arm bridged to `hydrate_document_lane(&head, &[])` (S4-LOAD then replaced it with
  `hydrate_pure_head`).
- Host `🖥️host/🦀️.rs`: `MemberDraft.label`, `run_transaction(description)` deleted; `🏃️run` `run_transaction(description)`, seq/in_reply_to arms;
  MCP: gateway `AppCommand::TransactionPrepare.label`, `transaction_prepare_with_retry(label)` (the hand-written `"agent invoke …"`/`"saga …"` labels),
  shell-channel JSON `label` (Rust encoder, TS decoder + type, shared fixture `🗿️app-payloads.json`), workspace mapping, seq/frame/tag arms, sender
  `head: Vec::new()`.
- TS `💻️os/🟦️.ts`: all of the above in the twin, `AppChannelClient.loadDocument` + the LoadDocument document-cache candidate path deleted,
  `loadDocumentArchive` snapshots the root pair at admission (a caller mutating its archive mid-load can no longer reach the cache),
  `readChildHeads()`; React `🔌️PluginRuntime` (`TransactionPrepareRequest.label`, coordinator `TransactionProposal.description`); docs in ShellHost
  and the energy model.
- Tests/goldens re-sealed by scripts (each anchor asserted): `🧪️s4-bump-channel-tests.py`, `🧪️s4-bump-channel-heads-tests.py`,
  `🧪️s4-bump-plugin-tests.py`, `🧪️s4-bump-host-tests.py`, `🧪️s4-bump-mcp-tests.py`, `🧪️s4-bump-react-tests.py`, `🧪️s4-bump-backbone-tests.py`
  (document-cache acceptance law moved onto `loadDocumentArchive`, same fixture + schema), `🧪️s4-bump-backbone-heads-tests.py`,
  `🧪️s4-bump-ts-twin.py`, `🧪️s4-bump-ts-twin-heads.py`, `🧪️s4-bump-channel-heads.py`; shared vectors `🧾️app-command-transaction.json` /
  `📨️app-frame-transaction.json` re-sealed.
- CHANNEL_VERSION 20 → 21: pin edited, `channel-version generate --guest` wrote 20 consumers; catalog-publication hostile vectors moved to
  20/22 (+ `📇️consumers.json`); the 8 refused derived consumers re-derived by `🧪️s4-bump-reseal-channel-v21-fixtures.ts` (old values re-verified
  under v20 first): directory plan (`expectedHex`/generation), browser open, execution-target lease (descriptor Pack f64 byte, descriptor SHA-256,
  catalog generation), compiled-dependencies (4 Pack f64), two-package / generation-stage (literals; generations are computed at run time), GIS
  frozen binding (its committed digest was ALREADY stale: a peer renamed `nativeExecutable` on 10-01 without a reseal; re-derived over the current
  binding, quoted by gis-inference-job / proposal-approval). `channel-version check`: **pin 21, 36 consumers, 0 findings**.
- Not re-derivable here: `stdio-gis-bootstrap` profile `generationId` (literals are 21). Its only consumer is the hub oracle
  `trusted-stdio-gis-bundle-check --source`, which is red before the bump (central schema catalog lacks `🪐️space/🏛️ownership/🧮️compute`, and it
  captures the codec closure natively: committed stdio receipts are 30 rows with new hashes vs the oracle's exact 29). The lifted-oracle attempt
  (`🧪️s4-bump-bootstrap-generation.ts`) cannot reproduce the committed v20 value from committed receipts → owner/coordinator action.

Verification so far: `cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` **Finished 03:37** (lib 267 warnings,
lib test 1024, 0 errors).

#### Milestone 2 — resume after the 04:15 cut: versionless wire fixtures + channel handshake (06:45–07:30)

State at resume: wave A complete and consistent (no half-edit of mine; the cut hit while reading host code for the handshake design).
Coordinator scope (rule 41 ABI freeze holder): runtime guest↔host channel handshake, then wave B, then BUMP DONE.

- **Versionless wire fixtures** (`🧪️s4-bump-versionless-wire-fixtures.py`): `🧬️fixtures/🪪️document-identity-wire-v20` → `…/🪪️document-identity-wire`,
  `🎬️media-export-wire-v19` → `…/🎬️media-export-wire` (plain `mv`, schema `$id` versionless), both registered channel-version consumers
  (+ the media schema's `const`); the census learned the `"channelVersion"` key (pattern + `git grep` alternative); both Rust laws assert
  equality with `CHANNEL_VERSION` (renamed without `v20`/`v19`); TS readers re-pointed. `channel-version generate`: pin 21, 39 consumers, 0 findings.
- **Handshake design** (no extra round trip, one read per instantiation, before any frame): new export `reactor.channel-version: async func() -> u32`
  (WIT) answered by the guest's compiled `CHANNEL_VERSION`; owned-ABI twin `semio_owned_channel_version_v1() -> u64` (JSON integer) because
  the native wgpu renderer, `🏃️run` and the hub run guests on the owned interpreter. Every host admits through ONE function:
  - kernel `admit_guest_channel_version(guest, host) -> Result<(), Fault>` + `CHANNEL_MISMATCH_CODE` (`📡️spr/🧵️channel`, re-exported);
    refusal = `plugin.channel-mismatch` (Framework) with params `guest`/`host`;
  - TS twin `admitGuestChannelVersion(guest, host): Fault | null` + `CHANNEL_MISMATCH_CODE` (`💻️os/🟦️.ts`), self-contained so the jco bridge
    embeds it via `toString()`;
  - hosts: wasmtime `WasmtimeRuntime::instantiate` and the pooled `⏳️runtime` spawn call `call_channel_version` right after
    `instantiate_async`; `OwnedRuntime::instantiate_actor` runs `OwnedOperation::ChannelVersion` (1 M fuel, 5 s) before handing the instance out;
    the jco bridge (`createActorApi`) awaits `reactor.channelVersion()` before returning the API and throws an `Error` carrying `.fault`;
    refusals surface as the new `PluginHostError::Refused(Box<Fault>)` (Rust). A pre-v21 component lacks the export: wasmtime/jco instantiation
    and owned admission (`OwnedSemioExport::ALL` now 18) refuse it outright, so it can never be misread either.
  - Notice: framework table `plugin.channel-mismatch` en/de with `{guest}`/`{host}` (kernel Rust + TS twin + fixture; schema pattern admits
    the `plugin` namespace); laws: kernel Rust/TS notice tests (filled placeholders, unfilled → no notice).
  - Corpus `📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake/🔣️.json` (+ schema `🧬️schema/🔣️channel-handshake`): cases by `guestOffset`
    (0 admitted; −1, +1, −20 refused), replayed by the Rust channel law `a_host_admits_only_a_guest_of_its_own_channel_version` and the TS law in
    `🧪️backbone-envelope-io`.
  - Guests: `__semio_actor_exports!` (`channel_version`), `__semio_owned_core_exports!` (`semio_owned_channel_version_v1`),
    `component::wasip2::channel_version()`; scale fixture implements it (its `CHANNEL_VERSION` literal is a registered guest consumer).
  - Fallout fixed: owned-ABI law count 13 → 18, hub synthetic owned module types `ChannelVersion` like `Describe`, browser actor-exports
    fixture lists `reactor.channelVersion`.

Handshake verification (08:20, rule 43: checks only):
- `cargo check -p semio-framework-os-kernel -p semio-framework-plugin-host -p semio-framework-plugin -p semio-framework --lib --tests`: kernel lib +
  lib test and framework lib + lib test compiled (0 errors in them); the run was then SIGKILLed by the deadlock breaker while queued (exit 137);
  the only red was the peer integration test target `sqlite_snapshot_native_admission` (S4-INFRA sqlite snapshot migration, E0432).
- `cargo check -p semio-framework-plugin-host -p semio-framework-plugin --lib` **Finished 07:36** (host 4 warnings, plugin 258, 0 errors).
- wasm32-wasip2: `semio-hub-puzzle` / `semio-hub-vcs` compositions are blocked by the peer stdio sqlite crates (`stl` ValueError, `ply`/`wav`
  sqlite E0308 — rule 41c S4-INFRA); the guest macros are proven on a stdio-free component instead:
  `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-plugin-imperative-math --target wasm32-wasip2 --lib` **Finished 08:18** (expands
  `extension_exports!` → `__semio_actor_exports!` (`channel_version`) + `__semio_owned_core_exports!` (`semio_owned_channel_version_v1`)).
- TS: `💻️os` in-source suite **258/258 PASS** (incl. the handshake corpus law); kernel `framework-notices` law **4/4 PASS** (explicit
  `SEMIO_VITEST_POLICY`); `channel-version` census law **4/4 PASS**; `channel-version check` pin 21, 39 consumers, 0 findings.
- OWED (rule 43): Rust `a_host_admits_only_a_guest_of_its_own_channel_version`, the channel unit suite, the renamed wire-fixture laws, kernel
  `a_channel_mismatch_names_both_versions_in_both_locales`, owned-ABI law (18 exports), hub synthetic owned-guest test.

#### Milestone 3 — audit F1/F2/F7/F19 (`📓️audit-s4-core.md`), 11:35–13:05 (cut ~08:55–11:35; repair check: all handshake host edits were on disk, wave B not started)

- **F2** (owned codec origin un-admitted): new `admit_owned_channel(state)` in `🔌️plugin/🖥️host/🦀️.rs` (reads `semio_owned_channel_version_v1`
  once, admits through `protocol::admit_guest_channel_version`, refuses as `PluginHostError::Refused`); both owned paths call it:
  `OwnedRuntime::instantiate_actor` and `assemble_codec_origin` (before `pack-schema-hash`, so the hub Check In fold, SQLite export/import
  and the wgpu `OwnedComponentDocumentCodec` never serve a guest of another channel). The hub synthetic owned guest
  (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`) now answers its channel (data segment at 3072).
- **F1 host half** (structured refusal flattened): `activation::ActivationRefusal { reason, fault: Option<Fault> }`
  (`🔌️plugin/🖥️host/🎠️activation/🦀️.rs`, `ActivationRefusal::instantiation(PluginHostError)` keeps the `Refused` fault) is what
  `install_actor`, `NativeKernelRuntime::activate` (`💻️os/🖥️host/🎠️activation`) and the wgpu `ParallelRuntime::activate` return. Callers
  owned by other slices keep `.reason` until they carry the fault: `🏃️run` (`RunError::Host(refusal.reason)`, S4-LOAD), wgpu `create_app`
  and the env probe (`.map_err(|refusal| refusal.reason)`, S4-WGPU), the extension activation log (Display).
- **F7** laws (compile-checked, OWED rule 43): `owned_hosts_admit_a_guest_channel_before_any_frame` (`🖥️host/🧪️tests/🔬️owned-runtime`:
  a byte-built owned guest whose every other export traps; own channel → admitted and reaches its first codec frame; ±1 → actor and codec
  call refused with `plugin.channel-mismatch` + params before any frame) and `a_guest_refused_at_admission_reaches_the_host_as_its_fault`
  (`🎠️activation` tests; `MockGuestRuntime::script_instantiate_refusal` → `ActivationRefusal.fault` carries the params, reservation retired,
  no instance dropped).
- **F19**: `🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🔣️.json` registered as a derived channel-version consumer → `channel-version check`:
  pin 21, 41 consumers, **1 finding (intended)** until the describe wave regenerates that fixture component (+ its `🛂️.descriptor.semio`).
- Checks: `cargo check -p semio-framework-plugin-host --lib --tests` Finished 12:52; `cargo check -p semio-framework-os -p semio-framework-os-run
  -p semio-framework-os-renderer-wgpu --lib` Finished 12:59; `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`
  Finished 13:03 — 0 errors each. Reported "RENDERER GREEN 13:03".
- F18 (describe before wave B) acknowledged: wave B held staged until the coordinator's "ACTIVATION DONE". `🔗️causal/🔀️transition`
  `TransitionCheckpoint.description` / `FoldChange.description` are checkpoint/change descriptions, not edit ones — they stay.

#### Wave B staging (13:05)

Driver `🧪️s4-bump-waveb.py` (dry run / `--preview` mirror / `--apply`), library `🧪️s4-bump-rust-fields.py` (regex code mask, bounded head
path, span sweeps). Dry run: 180 files, 2 hot files reported for the Edit tool (`🎮️mutation/🦀️.rs`, `🔌️plugin/🦀️.rs`), every remaining
`description` token reviewed (all are checkpoint/change, UI, puzzle meta, MCP tool descriptions). Store anchors asserted exactly; binary
command presence keeps the `0b10` transaction bit and refuses every other bit; `.spr` REC_EDIT refuses bits 2–3; owned SPR edit field 6
removed; canonical-edit field paths renumbered. Fixture reseal `🧪️s4-bump-reseal-edit-description.ts` (oracles reproduce all committed
digests first): 7 digest chains, sealer + borrowed map (`expectedJson`/`expectedDigest`), canonical reader (4976 → 4944 bytes).

#### Describe crate repair (13:26, coordinator routing)

- Puzzle describe s4-2 red: `semio-framework-plugin-describe` 2× E0433 at `🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:537` — `store::json` no
  longer exists (the kernel stopped re-exporting the pack-json helpers). Fix: direct dependency `semio-framework-pack-json` in
  `🖨️describe/📦️packages/🦀️rust/Cargo.toml`, call site `semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&final_value))`.
- `cargo check -p semio-framework-plugin-describe --lib` **Finished 13:26** (0 errors, 2 warnings).
- PARKED per coordinator (usage limit): wave B stays staged (nothing on disk). Landing order when resumed after ACTIVATION DONE:
  1. `bun 🧪️s4-bump-reseal-edit-description.ts --write` + canonical-edit schema (`"description"` out of both required lists/properties) + TS twin
     (`🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts`: drop `...text(edit, "description")`);
  2. `python3 🧪️s4-bump-waveb.py --apply` (re-run the dry run first: anchors are asserted);
  3. Edit tool on the hot files — `🎮️mutation/🦀️.rs` (Edit field, ToValue/FromValue) and `🔌️plugin/🦀️.rs` (6× `description: None, transaction: None`,
     `description: None, lane:`, genesis description, BoundedConfigPreparation field/literal/preflight/close/terminal, `commit_transaction_group` +
     `dispatch_emit_group` params and their 4 calls, the GroupMeta literal, 4 `begin_*apply_batch` `None` args, 3 `into_owners` triples, docs);
  4. gated checks: replication/kernel/plugin/plugin-host/framework native + wasip2, then the touched ✏️s plugin and hub crates; TS twin vitest.
