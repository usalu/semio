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

## Session 5 — 2026-10-05

Agent: S5-CHANNEL (successor of S4-BUMP + S4-PACKFIX). Scratch: `🗑️generated/s5-channel/`. Kernel test-target work is in
`📓️s4-packfix-report.md` § Session 5. Decision in force: design §22.4 — wave B lands with `CHANNEL_VERSION` 22, only after the
coordinator's "WAVE B GO".

### S5.0 State at launch (00:18–00:25, landing + serve locks HELD by COORDINATOR-ACTIVATION)

- Repair-first: no half-edit of S4-BUMP on disk. Pin 21 (`💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json`), const 21
  (`📡️spr/🧵️channel/🦀️.rs:26`). Wave B is not on disk (store still declares the `description` fields). Owned trees changed after
  the last S4 section only by the 18:27 stash-pop (mtime reset) and by a peer's 22:56 sqlite files in the host fixture component.
- Wave-B dry run on today's tree (`python3 🧪️s4-bump-waveb.py`, read-only, log `s5-channel/waveb-dry-1.txt`): **exit 0, every
  exact/after anchor asserted, 180 files + 2 hot files** — the same file set as the 10-04 13:14 dry run. Drift = 3 new literals the
  sweep already covers (`🏪️store/🧪️tests/🔬️unit` 199 → 201 removals, `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract`
  11 → 12); 82 `REMAIN` lines, identical to the reviewed set; 0 warnings.
- Reseal oracle dry run (`bun 🧪️s4-bump-reseal-edit-description.ts`, no `--write`, log `s5-channel/reseal-dry-1.txt`): exit 0, all
  7 committed digest chains + sealer + borrowed map reproduced before the re-derivation (reader 4976 → 4944 bytes, unchanged).

### S5.1 Wave B + channel 22 prepared for one lock hold (00:25–00:45, nothing saved under `🧰️framework/**`)

Inputs (ticket root, kept): `🧪️s4-bump-waveb.py` (hardened), `🧪️s5-channel-waveb-hot-plan.py` (new),
`🧪️s5-channel-reseal-channel-version-fixtures.ts` (new), `🧪️s4-bump-reseal-edit-description.ts` (unchanged).

- **Driver hardened** (the session-4 runaway lesson): `--apply` now REQUIRES `--files-from <list>` (written by a dry run with
  `--list`); it refuses an empty/missing list, a repo root without `.git`/`Cargo.toml`, a broken candidate search, and any
  write set that differs from the list; it re-reads every file right before writing (a file changed in between → nothing is
  written), keeps the originals under `s5-channel/waveb-backup/` + `waveb-manifest.json` (sha-256 before/after) and offers
  `--restore` (puts back only files that still hold the applied bytes; edited ones are listed as CONFLICT). RAN: the three
  fail-closed probes (no list, empty list, 3-of-180 list) each refused with nothing written (store still 20× `description: Option<String>`).
  Explicit list: `s5-channel/waveb-files.txt` (180 paths: 51 `💻️os`, 20 puzzle, 4 stdio, 4 hub, 1 replication = 80 in the
  landing closure; 100 in plugin trees outside it). Mirror of the result: `s5-channel/waveb-preview/` (`--preview`, incl. hot files).
- **Gap found in the staged wave** (a reader the driver cannot see, found by `s5-channel/unwritten-description-reads.py` over
  every Rust file the wave does not write): `🔌️plugin/⏪️time-travel/🦀️.rs` `MemberEditHistory.description` copies
  `protocol::Edit.description` (L1739) and `member_backfill` labels a composed-member row from it first (L1834). Without a
  fix the plugin crate would not compile after wave B. Added to the wave (design §20.6: a row label comes from the leaves): the
  field, its reader, the label arm (first leaf → first printed operation → edit id), the Rust law field, the TS twin, the
  fixture (`🧫️composed-child-history`: 8 `description` properties; case `e1` is now labelled `Set snapshot` /
  `Schnappschuss setzen` instead of the hand-written `Imported`) and its schema. Region owner of the file: S5-RUNTIME (told via `main`).
- **Hand-edit plan** `🧪️s5-channel-waveb-hot-plan.py` = every Edit-tool row with its expected count: 6 rows
  `🎮️mutation/🦀️.rs` (struct field, doc, ToValue, FromValue ×3), 23 rows `🔌️plugin/🦀️.rs` (driver's 27 removals + the
  `commit_transaction_group`/`dispatch_emit_group` definitions, the caller, 2 docs), 17 rows in 7 further files (time-travel,
  composed-child-history law/twin/fixture/schema, canonical-edit schema + TS twin). RAN (simulation on today's tree): **46/46
  anchors found with their exact counts; closure proof passes** (after the rows the driver's own `transform` removes nothing
  more in the hot files and no `description` token survives outside the UI/label allowlist); `--verify` on the unedited tree
  reports NOT closed (negative control). After the Edit pass the same script with `--verify` is the gate.
- **Channel 22 fixtures.** `channel-version check` today: pin 21, 41 consumers, 1 finding (the intended F19 one). Derived
  consumers: `bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from 21 --to 22` dry run **exit 0**: every committed v21
  generation/digest re-verifies, 10 files would change (plan, browser open, lease descriptor + generation, compiled-dependencies
  ×4 Pack f64, two-package, generation-stage, stdio-gis-bootstrap literals, GIS frozen binding + its two quoting fixtures).
- **Latent S4 defect found (red today, not caused by wave B).** `🔌️plugin/📇️registry/🧫️fixtures/🧬️catalog-publication/🔣️.json`
  still declares `executionProtocol.appChannelVersion: 20`: the v21 bump moved its hostile rows to 20/22, and the census
  treats ANY literal that equals a declared hostile value as hostile, so the stale pin literal became invisible (a +1 bump
  always turns the old pin into the new "previous" hostile). RAN `SEMIO_TEST_ARTIFACT_DIR=… bun ./📜️script.ts test-catalog-contract`
  (cwd `🔌️plugin/📇️registry`): **27 pass / 18 fail**, all 18 = `expected 20 to be 21` (owner descriptor admission). Fix = one
  literal (→ 21 now, hand-set to 22 with hostile 21/23 at the bump); census hardening (`hostileOccurrences`) listed in S5 open items.
- Peer finding (not mine, no action taken): `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️surface/🦀️.rs:73,241` reads
  `emit.description`, a field `Emit` no longer has (deleted by §20.6) → that hub test target cannot compile; owner S5-STROKES-NORM.
- Observed 00:34 while the landing lock was held by the activation: `🔌️plugin/🦀️.rs` gained `document_codec_bindings` and
  `🎒️pack/🌱️value`, `📡️replication/🎮️mutation/📦️bytes` changed (the Codex peer's snapshot/op-pages work, outside our locks).

### S5.2 Census hardening staged (`🧪️s5-channel-census-hostile-occurrences.py`, 00:45)

Schema-first fix of the blind spot above: a consumer that declares `hostileValues` also declares `hostileOccurrences`
(contribution schema: both or neither, `anyOf` + `propertyNames` so the first-party subset validator and Ajv agree); the
census reports a `hostile` finding when a file holds another number of hostile literals, and `generate` refuses to rewrite
such a file. 8 files: schema, type, census + generator, portable corpus (+2 census cases `stale-pin-equal-to-a-hostile-value-refuses`
/ `hostile-rows-beside-the-pin-are-admitted`, +2 refused admission cases), both laws, the os registry (4 hostile consumers) and
the hub registry (1). RAN: dry run, **19/19 anchors ok, 8 files would change**. Lands in my first lock window; RED → GREEN proof
= `channel-version check` must name `🧬️catalog-publication` as `hostile` (3 held, 2 declared) before the fixture literal is fixed.

### S5.3 Landing runbook for "WAVE B GO" (one `landing` hold, then `serve` for the two served TS files)

Pre-acquire (read-only, repeat right before): `python3 🧪️s4-bump-waveb.py --list s5-channel/waveb-files.txt` (anchors assert),
`python3 🧪️s5-channel-waveb-hot-plan.py` (46 rows ok + closed), `bun 🧪️s4-bump-reseal-edit-description.ts`,
`bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from 21 --to 22`, plus the relayed S5-RUNTIME (§22.2 `nextProblem`) and
S5-STORE (§22.3 viewed alternative) wire/persisted fields if they are not on disk yet.
1. `zsh 🔐️lock.sh acquire landing S5-CHANNEL`; `python3 🧪️s5-channel-waveb-hot-plan.py --snapshot`; copy the 4 canonical-edit
   fixtures, the 10 derived channel fixtures, the pin and the two registries to `s5-channel/waveb-backup/fixtures/`.
2. Fixtures: `bun 🧪️s4-bump-reseal-edit-description.ts --write` (4 canonical-edit fixtures).
3. `python3 🧪️s4-bump-waveb.py --apply --files-from s5-channel/waveb-files.txt` (180 files, backup + manifest).
4. Edit tool, rows of `🧪️s5-channel-waveb-hot-plan.py` in order (46 rows, 9 files); then `--verify` must print `closed`.
5. Channel 22: pin `🔖️channel-version.json` 21 → 22; catalog-publication hostile rows 20/22 → 21/23 and its pin literal → 22;
   `hostileValues` `[21, 23]` in `📇️consumers.json`; `acquire serve S5-CHANNEL`; `bun ./📜️script.ts channel-version generate --guest`
   (cwd `🧑‍💻dev/📦️packages/🟦️typescript`; writes `📡️spr/🧵️channel/🦀️.rs` const and `💻️os/🟦️.ts` `APP_CHANNEL_VERSION` among ~22 literal
   consumers); `bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from 21 --to 22 --write`; channel docs name the v22 change.
6. Gated checks (v3 gate before each): `cargo check -p semio-framework-replication -p semio-framework-os-kernel
   -p semio-framework-plugin -p semio-framework-plugin-host -p semio-framework -p semio-framework-os -p semio-framework-os-run --lib`;
   the same kernel/plugin set `--target wasm32-wasip2`; `cargo check PM -p <puzzle 2d/3d/5d + stdio crates touched> --lib`;
   `cargo check HM -p semio-hub-puzzle --target wasm32-wasip2 --lib` (the activation canary); kernel + plugin `--lib --tests`.
7. TS: `💻️os` in-source suite (channel twin), canonical-edit twin, composed-child-history twin, `channel-version check`
   (expected: pin 22, findings = the host fixture component only), `test-catalog-contract`, `test-contributions`.
8. `release serve`, `release landing`; then outside the lock the 100 plugin-tree files: batched `cargo check PM -p … --lib`.
Fallback (rule 51, not green within the hold): `python3 🧪️s5-channel-waveb-hot-plan.py --restore` (the 9 hand-edited files go
back byte-identically to the `--snapshot` taken right after step 1; a file that differs by anything but plan rows is left and
listed), `python3 🧪️s4-bump-waveb.py --restore` (180 files from `waveb-backup/` by sha), the 4 + 10 + 2 fixture/pin files from
the copies taken before step 2 under `s5-channel/waveb-backup/fixtures/`, pin back to 21 + `generate --guest`, release, report.
Coordinator actions after the landing: re-describe + re-activate puzzle (every component is rebuilt at 22; 69 committed
descriptors stay withheld by the dev registry until their describe), regenerate `🔌️plugin/🖥️host/🧫️fixtures/🧩️component` (F19),
`trusted-stdio-gis-bundle-check --source` for the `stdio-gis-bootstrap` `profile.generationId` (not re-derivable here).

### S5.5 Landed 01:48–01:52 (hub + serve locks, cargo-free): census rule + the stale fixture

- `python3 🧪️s5-channel-census-hostile-occurrences.py --apply` (8 files) + one schema follow-up (Ajv strict mode wants the
  `anyOf` branch to declare its own `type`/`properties`; the script carries the fix).
- RED → GREEN, all RAN: `channel-version check` right after the rule landed = **2 findings**, the new one
  `hostile …/🧬️catalog-publication/🔣️.json: declares 2 hostile literal(s), holds 3` (the blind spot is closed); after the
  literal fix (`executionProtocol.appChannelVersion` 20 → 21, Edit tool) = **1 finding** (the intended F19 one).
  `channel-version test-contributions`: **29 vectors passed** (was 25: +2 census cases, +2 refused admission cases);
  `test-catalog-contract`: **45 pass / 0 fail** (was 27 / 18); kernel census law (vitest) **4 / 4**.
- Re-verified after the 02:40 usage cut (04:25): the same four commands, same counts.

### S5.6 Wave B grew two riders; everything dry-run clean again at 04:47 ("WAVE B READY" sent)

- **§22.22 rider** `🧪️s5-channel-merge-archive.py` (codec only; S5-LOAD/S5-STORE land the handlers):
  `AppCommand::MergeDocumentArchive { seq, archive }` = tag 43, the last command, body byte-identical to
  `LoadDocumentArchive` (one shared encoder arm, one shared paged decode state with a `merge` flag, one shared retirement);
  `DocumentArchiveLoadStatus.ahead` (varint after `total`, before `fault`). Rust: channel + its unit laws (round trip, paged
  route, "the load encoding under its own tag", shared vectors), `🏃️run` + MCP `seq` arms, run test status; guest: 3 Edit-tool
  rows in `🔌️plugin/🦀️.rs` (`ahead: 0` at both status constructors, an `app.command.unsupported` arm) and the guest law
  `a_merge_archive_command_is_refused_as_unsupported_until_its_handler_lands` (encoded + decoded route) in the live-dispatch
  tests. TS twin + `🧪️backbone-envelope-io` (round trip, tag 43, shared vectors), host/runtime test status literals. New shared
  golden vectors `💻️os/🧫️fixtures/📡️channel/🧲️document-archive-merge.json` (hand-derived bytes, proven by both languages).
  Dry run: **41/41 rows, 10 files + 1 new, 3 hand rows**; `--verify` on the unlanded tree fails (negative control).
- **Driver** re-derived 04:46: exit 0, **181 files** (+ S5-LOAD's new `🔌️plugin/🧪️tests/🧪️folder-reload-route`, one more
  literal in the time-travel tests) + 2 hot; found and added one more stale reference the removal leaves behind
  (`matches!(field_id, 2 | 6 | 10 | 11 | 12)` in the owned SPR edit decoder still named the removed field id 6).
  Hand-edit plan **46/46**, closed. Reseal oracles exit 0. Lock set per rule 58: landing → stdio → puzzle → hub → serve.
- The hold cannot be 20 min after the 04:22 build-unit prune (cold verifying set at 3 jobs): told the coordinator; rollback is
  scripted (`🧪️s4-bump-waveb.py --restore`, `🧪️s5-channel-waveb-hot-plan.py --restore`).

### S5.7 Design §22.19 — descriptor packs canonical by the descriptor layer (staged 04:45, lands on my `landing` ticket)

Finding that led to it: the kernel value twin sorts intrinsic object keys (the coordinator's 10-04 20:41 fix), the framework
twin keeps authored order by the pack peer's own law; `describe` relied on the kernel twin's sort, so the next activation's
`describe` would die with "emitted descriptor pack is not canonical" the moment the peer unifies the twins. Decision (a):
the general encoder follows the peer; `describe` canonicalizes.
- `🧪️s5-channel-describe-canonical.py` (dry run **7/7 rows, 4 files**): `CanonicalDescriptorValue` (members of every object
  in UTF-8 key-byte order, recursively, stable) is the ONLY input of `descriptor_pack`; `describe_component` hashes and writes
  through it (the JSON mirror keeps its authored member order; verifiers compare it deep-equal). Laws in the describe crate:
  `a_descriptor_pack_is_canonical_whatever_order_its_members_were_authored_in` (authored / canonical / reversed member order
  all pack to the bytes the TS encoder sealed; fixture parsed with the first-party order-preserving JSON reader, never
  `serde_json`, whose map would sort the "authored" form) and `the_emitter_encodes_descriptors_only_through_the_canonical_pack`
  (the emitter source names `encode_wire_value(` exactly once). New verb + nx target `test-canonical-descriptor-pack`.
- On disk already (new files): fixture `🖨️describe/🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json` (2 cases: nested
  reverse-declaration order; UTF-8 vs UTF-16 order `Z a ～ 😀`), its schema, the TS oracle
  `🧪️tests/🧪️canonical-descriptor-pack/🟦️.ts`; sealed by `🧪️s5-channel-seal-canonical-descriptor-pack.ts`
  (TS side only). RAN: oracle **2/2 cases**. Taxonomy report for the describe scope: the two new directories add no finding.
- OWED after landing: `cargo check -p semio-framework-plugin-describe --lib --tests`, the two laws as targeted tests,
  `bun 🧪️s4-describe-pack-diff.ts` (byte-exact round trip of the real puzzle descriptor).
- Coordinator action: launch row for `test-canonical-descriptor-pack` at the next seed/launch regeneration.

### S5.8 §22.19 landed 05:20–05:30 (landing + serve) and proven; owed handshake laws run

- Landed: `python3 🧪️s5-channel-describe-canonical.py --apply` (4 files) → `--verify` 7/7 rows on disk; verifying check
  `cargo check -p semio-framework-plugin-describe -p semio-framework-os-kernel --lib --tests --keep-going` **exit 0** (05:30).
- RAN `bun ./📜️script.ts test-canonical-descriptor-pack` (cwd `🖨️describe/📦️packages/🦀️rust`): `cases=2`.
- RAN `bun 🧪️s4-describe-pack-diff.ts` (05:36–05:43, real `semio_hub_puzzle.wasm`, new emitter): **emitted 306822 bytes,
  re-encoded 306822 bytes, first difference at 306822 (byte-exact), 0 JSON/pack disagreements**; the descriptor equals the
  committed one except its three hashes (the component was rebuilt after the 00:32 describe); the JSON mirror keeps its
  authored member order.
- RAN targeted tests (uplift dir `target-nde-s5-channel`, `CARGO_INCREMENTAL=0`, gate v5, 3 jobs):

| Time | Command | Result |
| --- | --- | --- |
| 05:45–05:46 | `cargo test -p semio-framework-plugin-describe --lib -- a_descriptor_pack_is_canonical the_emitter_encodes_descriptors owned_core_exports_are_defined_once` | **3 passed / 0 failed** (both §22.19 laws + the owned-ABI law, 18 exports) |
| 05:50–05:54 | `cargo test -p semio-framework-plugin-host --lib -- owned_hosts_admit_a_guest_channel_before_any_frame a_guest_refused_at_admission_reaches_the_host_as_its_fault` | **2 passed / 0 failed** (audit F7; F2 proven by the first) |
| 05:54–05:55 | `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` | **91 passed / 0 failed** (channel unit suite incl. `a_host_admits_only_a_guest_of_its_own_channel_version`, wire-fixture laws) |
| 05:56–05:58 | `cargo test -p semio-framework --lib -- a_channel_mismatch_names_both_versions_in_both_locales` | **1 passed / 0 failed** |

- Wave-B dry runs re-derived 05:47 after PUZZLE codes, RUNTIME D+E, STORE RB+P2, UI r2, LOAD attach: no anchor moved; driver
  183 files (+ S5-STORE's `🧪️supersede-law`, `🧪️viewer-head`), plan 46/46 closed, rider 41/41 + 1 + 3, reseals exit 0.
- Still OWED from the S4 list: the hub synthetic owned-guest test (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit`).

### S5.9 WAVE B LANDED — channel 22 (all five locks 06:12:26–06:47:53, 35 min of the granted 50)

On disk: the store `description` deletion (driver **183 files** + **9 files** found by the positional pass = 192 in the restore
manifest `s5-channel/waveb-manifest.json`), the hand rows (6 `🎮️mutation/🦀️.rs`, 23 + 3 `🔌️plugin/🦀️.rs` by the Edit tool,
17 rows in 7 further files by `🧪️s5-channel-waveb-hot-plan.py --apply-extra`), the resealed canonical-edit fixtures, the §22.22
rider (`🧪️s5-channel-merge-archive.py --apply`: 10 files + the new vector fixture), `CHANNEL_VERSION` 22 (pin, 24 generated
literals incl. the Rust const and TS `APP_CHANNEL_VERSION`, 10 derived fixtures, catalog-publication hostile rows 21/23).

Three defects of the staged wave surfaced only in the hold (each fixed, the scripts carry the fix):
1. the driver's own rewrite `read_command_transaction(reader, reader.read_u8()?)` borrowed `reader` twice (2× E0499) → two statements;
2. **driver blind spot**: its candidate search is the word `description`, so a file that only passes the argument positionally
   was never seen — 21 call sites in 10 files (`begin_*apply_batch(…, None, …)`, `.preflight(&m, None, lane)`,
   `apply_one(…, None, …)`, `DurableOwnedMapMemberAdmissionV1::new(…, Some(..))` in `🌎️hub/💡️inference/🏃️runtime`). New input
   `🧪️s5-channel-waveb-positional.py` (`--scan` over the whole tree with the driver's call table, `--apply --files-from` on an
   explicit list, same backup + manifest): 9 files written (the tenth, the hub space footprint test, was fixed by its owner
   at 06:25), re-scan = 0;
3. two stub guests of the OS twin suite (`🔌️plugin/🧪️tests/🧬️component-codec-reply`, `🧪️shardclient-reserved-response-settlement`)
   exported `reactor = {}` and failed the v21 handshake since 10-04 (`reactor.channelVersion is not a function`; S4's "258/258"
   was a narrower run) → they answer `APP_CHANNEL_VERSION`.

Interference during the hold (not mine): S5-LOAD's scratch mirror wrote through symlinks into the channel test file 06:17–06:25
(a call to a not yet existing `DocumentArchiveLoadHost::merging`); restored by LOAD at 06:25:21, my rider `--verify` passes after it.

| Time | Check (gate v5, `CARGO_BUILD_JOBS=3`, `--keep-going`) | Result |
| --- | --- | --- |
| 06:18 | `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-plugin-host --lib` | exit 0 (kernel 424, framework 154, plugin 283, host 4 warnings) |
| 06:20 | `… -p semio-framework-os -p semio-framework-os-flow -p semio-framework-os-infinite -p semio-framework-os-mcp -p semio-framework-os-run -p semio-framework-os-renderer-wgpu --lib` | exit 0 |
| 06:22 | `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-puzzle --target wasm32-wasip2 --lib` (activation canary = guest closure) | exit 0 |
| 06:27 | `… -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin-host -p semio-framework-os -p semio-framework-os-run -p semio-framework-os-mcp --lib --tests` | exit 0 |
| 06:29 | `cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` | exit 0 (lib-test 1120 warnings) |
| 06:33 | `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub -p semio-hub-space --lib --bins --tests` | `semio-hub-space` lib + lib-test ok; **`semio-hub` not reachable**: pre-existing peer reds `semio-framework-os-kernel-db` 23 errors (pack-error API, `pack::write_atomic` arity; no file of the wave) and `semio-hub-gis` 1 (`protocol::ToValue` private) |
| 06:38 | `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-{2d,3d,5d} -p semio-s-artifact-stdio-{contract,docx,semio,wav,zip} --lib --tests` | all ok except puzzle-2d lib-test: 51× `semio_framework_async` unresolved = the known feature gap, no wave error |
| 06:41 | `… -p semio-s-artifact-puzzle-2d --lib --tests --features component-app-assembly` | exit 0 |
| 06:42–06:44 | TS: OS twin suite `bun ./📜️script.ts test` (cwd `💻️os/📦️packages/🟦️typescript`) | **510 passed / 0 failed** (one file `🧪️backbone-envelope-io` is reported "No test suite found": it only exports the registered tests — pre-existing, exit 1 for that alone); merge-vector law 1/1 by name |
| 06:42 | `bun ./📜️script.ts test-channel-oracles` / `test-store-oracles` | 9 / 0 and 22 / 0 |
| 06:41 | composed-child-history oracle, canonical-edit fixtures + sealer self tests (`s5-channel/ts-oracles.ts`) | cases=4; ok; 37 reader checks |
| 06:44 | `channel-version check` / `test-contributions` / `test-catalog-contract` / census vitest law | pin 22, 41 consumers, **1 finding (intended F19)** / 29 vectors / 45 / 0 / 4 / 0 |
| 06:44–06:47 | `cargo test -p semio-framework-os-kernel --lib -- os_spr:: os_store::` (wave-B tree) | **849 passed / 5 failed**: 2 mine (below), `viewer_head_tests::the_viewer_head_corpus_matches_two_stores` (S5-STORE's own open defect: "incremental applied revision records … first stale record Some(1)"), `sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags` (pack peer), one env-only (`SEMIO_TEST_ARTIFACT_DIR`; passes with it). Channel suite incl. the three rider laws green. |

Open after the landing:
- **Mine, wave-B caused, fixture only**: kernel laws `durable_group::tests::durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds`
  and `…durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes` — the durable-owned-group corpus pins
  Store-owned outcome packs that embed an edit JSON with `description`. Staged: `🧪️s5-channel-reseal-durable-owned-group.py`
  (re-derives member hash, unsigned JSON, decision hash as the TS twin composes them; proves first that it reproduces the corpus
  as it is; dry run on the parent member: pack 917 → 899 bytes). Lands at "LOCKS OPEN": law run → `--from-log … --write` → repeat
  for the three members (one kernel lib-test rebuild each, the corpus is `include_str!`).
- **40 plugin crates outside the closure are WRITTEN BUT UNVERIFIED**: owner sheet `📓️s5-channel-waveb-plugin-owners.md`
  (crate → change shape → owner WP, owed command at its end).
- `semio-hub` (5 files of the wave): unverifiable until the two peer reds above are fixed.
- Rule 55 slip, recorded: the 06:44 kernel test build started at 22 GiB free (limit 25); it reused the 05:55 binary's units.

Coordinator actions after wave B:
- every component is rebuilt and re-described at channel 22 (a guest of 21 is refused at admission);
- documents persisted by an earlier build are refused on read (`.spr` REC_EDIT presence bits 2–3 are unassigned and refused; the
  binary command presence byte admits only the transaction bit) → fresh probe folders / browser storage. The 31 committed
  `.spr.semio` examples and the 78 `.op.semio` / `.cmd.semio` files hold no description (every file checked);
- F19: rebuild + re-describe the host fixture component — crate `semio-framework-plugin-host-fixture`
  (`🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust`, verb `component-dev`), then `describe` its wasm into
  `🧩️component/` (`🔣️.json` + `🛂️.descriptor.semio`); clears the one census finding (20 vs 22);
- `stdio-gis-bootstrap` `profile.generationId` (hub oracle `trusted-stdio-gis-bundle-check --source`), unchanged from S4;
- launch seed row for `test-canonical-descriptor-pack`.

### S5.4 Audit residue (P3)

- **F1** (structured `plugin.channel-mismatch` flattened on native paths). Host half is on disk (`ActivationRefusal { reason, fault }`,
  `ActivationRefusal::instantiation`). Three callers still drop the fault (verified by grep today):
  `🏃️run/🦀️.rs:2133` `.map_err(|refusal| RunError::Host(refusal.reason))` → S5-LOAD: `RunError` needs a variant that carries
  `semio_framework::Fault` (e.g. `RunError::Refused(Box<Fault>)` when `refusal.fault` is `Some`, else `Host(reason)`) and its MCP
  projection must pass the fault through unchanged; wgpu `🧊️renderer/🦀️.rs:8267` (`create_app`) and `:9988` (extension activation)
  `.map_err(|refusal| refusal.reason)` → S5-WGPU: the kernel outcome carries `ProgramFault { text: refusal.reason, fault: refusal.fault }`
  so the native ProgramBridge arm stops building `fault: None` (the Shell already consumes `refusal.fault` at
  `🐚️Shell/…/🧊️wgpu/🦀️.rs:24500/24509`). Law per path: a guest answering another channel reaches the notice funnel with `guest`/`host`.
- **F2** (owned codec origin un-admitted): on disk — `assemble_codec_origin` calls `admit_owned_channel(&mut state)?`
  (`🖥️host/🦀️.rs:1695`) before `PackSchemaHash`, `instantiate_actor` at `:1639`. Proof = law
  `owned_hosts_admit_a_guest_channel_before_any_frame` (OWED until the test window, command in S5 owed list).
- **F19**: unchanged, 1 intended census finding until the describe wave regenerates the host fixture component
  (coordinator action; after the v22 bump the finding reads 20 vs 22).
- **`TransportFailure` → `InvariantViolated` (recommendation).** Keep the mapping for this ticket and make the state
  unrepresentable later, in the pack-error owner's wave: `ArtifactPack::decode_pack_with(&[u8], …)` is a pure in-memory codec,
  so by the peer's own rule ("a pure parser cannot invent transport ownership", applied by PACKFIX to every `*_controlled`
  codec) it should return `PackRefusal`; `PackError` with its `TransportFailure` arm then exists only on entry points that own
  a `PackTransportContext` (file/HTTP sources), and `serializer_entry` loses the arm instead of classifying it. Until that
  trait narrowing (≈ every `ArtifactPack` impl, peer-owned and in flight), `InvariantViolated` is the right class: the failure
  can only come from an implementor breaking the contract, it is neither user input (`InvalidValue`) nor retryable transport
  (`Io`), and the message keeps the transport display for diagnosis. No change to `IoError` (it must not grow a transport source).

### S5.10 After "LOCKS OPEN" (07:27 → 10:30): durable corpus, F9, launch row

- **Durable-owned-group corpus LANDED** 07:27–07:38 (`🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json`: three member packs /
  hashes, unsigned JSON, decision hash `58742cc4e823…`), from the law's own derived bytes. RAN: kernel `durable_group` 20 / 0, TS twin
  ok. Process slip, recorded: I held `serve` 07:28–07:38 through the three lib-test rebuilds and blocked the live probe — a `serve`
  hold never spans a cargo run.
- **F9 (folder attach dead on B1) — cause found, NOT wave B.** The pack peer's encoder stopped sorting intrinsic
  `DslValue::Object` members by key bytes (`💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:566-572`, uncommitted against HEAD `5c7f51ee643`
  which sorted; framework twin `🔨️modules/🎒️pack/🌱️value/🦀️.rs:585-591` the same). B0 (00:14) sorted, B1 (06:48) keeps authored
  order. The guest's control receipt is `encode_wire_value(&DocumentBackboneBindingReceiptWireV1{schema, operation, instance_id,
  binding_generation, uri, code}.to_value())` (`🔌️plugin/📡️backbone/🔗️binding/🦀️.rs:92-103`) and the derive keeps field order
  (`🌱️value/✨️derive/⚙️expansion/🦀️.rs:491/496/1595`), so B1 writes schema, operation, instanceId, bindingGeneration, uri; the TS
  grammar re-encodes sorted and refuses (`🔗️binding/🟦️.ts:102-103`, `actor-document-control.noncanonical`). RAN
  `🗑️generated/s5-channel/f9-receipt-order.ts`: sorted bytes "accepted bound", the same entries in struct order "refused
  actor-document-control.noncanonical". Wave B's 192-file manifest holds no file of the binding / reactor / shard path.
  Side effect found: the Rust checks `binding-noncanonical` / `receipt-noncanonical` (`encode(decode(x)) != x`) were vacuous for
  order under the new encoder. Contract agreed with S5-LOAD and landed by LOAD (train line 10:18:03, wave `f9`): the control
  codec owns its canonical form (`canonical_control_bytes`, both encoders and both decode checks), TS grammar unchanged, golden
  rows in the binding fixture — the same rule as §22.19 for describe.
- **Same drift, other readers.** Checked not exposed: backbone message (`💻️os/🟦️.ts:486`, fields by id), envelope batch
  (`🔨️modules/📡️replication/🟦️.ts:1515`, hand binary), channel frames (tagged codecs), canonical-edit (document-order JSON, never
  the sorting TS encoder), describe (`🧾️describe/🟦️.ts:30`, guarded by `CanonicalDescriptorValue`), browser-actor intent /
  command (`🏪️store/👷️worker/🟦️.ts:1963/1993`, TS-produced). EXPOSED, not verified live: `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/
  📤️publication/🟦️.ts:138` (history patch pack) and `:230` (service-operation payload) re-encode a guest pack with the sorting TS
  encoder; they refuse wherever a guest builds an object out of key-byte order. Each needs a golden row and a canonicalizing
  owner on the Rust side.
- **Launch row**: `⚖️test-canonical-descriptor-pack🖨️describe🦀️` is in `.vscode/launch.json` (group `4_gate`), rendered from the
  declared project target; `reconcile-launch-seed --check` does not list it among its 28 hand-made rows, so the seed needs no
  row for it. Those 28 rows are other owners' — not moved by me.
- **Wave-B fallout census: NO VERDICT yet.** Batch 01 (animate-presentation, block-2d, block-3d, `--lib --tests`) was stopped by
  the harness at its 30-minute background limit while still compiling third-party test dependencies, 0 errors. Decision: ONE
  gated `--lib --keep-going` cargo over my 33 crates (`🗑️generated/s5-channel/plugin-crates-mine.txt`, helper `plugin-census.sh`)
  after wave C lands — wave C changes a kernel struct, so a census before it would be repeated in full.

### S5.11 Wave C = channel 23, staged (10:10 → 10:30), lands on "WAVE C GO"

- **One script, three row groups**: `🧪️s5-channel-wave-c.py` — PIN (pin 23, hostile rows 22 / 24, handshake corpus + law in both
  directions via `hostOffset`: this guest at the previous host, this guest at the next host, previous guest at the previous
  host), FRAMES (N2 field list of S5-NESTED: `ChildPackEntry.owner` after `envelope_pack`, `ChildHeadPackEntry.owner` after
  `head_pack`, text like `slot`; Rust codec incl. the streaming `LoadChildren` arm, TS twin, golden rows `owner: ""` → trailing
  `00` and `…OfAMember` rows with `owner: "content/forms-1"` in both suites, the two MCP test literals), DIGEST (§22.28: the
  two TS oracles digest the edit's revision value, schema text). 13 files / 61 rows; `--without-frames` / `--without-digest` drop
  a half; all four combinations dry-run clean at 10:20.
- **Proven on a scratch mirror of the 13 files** (no live write): apply → `--verify` 61 / 61 → second apply refused → restore
  beside a regenerated version literal → 13 / 13 byte-equal to the live tree. Patched files parse: 10 TS / JSON (bun), 3 Rust
  (rustfmt), 0 errors. NOT compiled.
- **§22.28 fixtures**: `🧪️s5-channel-reseal-canonical-edit.ts` first proves its oracle on the four fixtures as they are, then
  derives the new ones (sealer + borrowed map `expectedJson` −19 bytes and `expectedDigest`, reader `expectedByteLength` 4944 →
  4925 and `expectedJsonSha256`, seven chain digests). Rule from S5-STORE (10:20): single-operation = `record("edit", [id, JSON
  without sequenceNumber])`, chained = without the 4-byte sequence part, the member leaves the canonical JSON, the `edit` input
  keeps it; no wire byte changes. Preview RAN: the patched oracle refuses the old fixtures, accepts the resealed ones, the reseal
  is idempotent.
- **Landing order** (coordinator's five-lock hold, apply only): STORE `🧪️s5-store-revision-digest.py` (6 files) → NESTED
  `🧪️s5-nested-n2-wire.py land` (4 files, 23 replacements) → mine: `wave-c.py --apply`, `reseal-canonical-edit.ts --write`,
  `channel-version generate --guest`, `reseal-channel-version-fixtures.ts --from 22 --to 23 --write`, `channel-version check`,
  `wave-c.py --verify`. Restore: `reseal-canonical-edit.ts --restore` → `wave-c.py --restore` (rows backwards) →
  `generate --guest` → `reseal-channel-version-fixtures.ts --from 23 --to 22 --write`.
- **Owed after the landing**: kernel `--lib --tests` (channel suite incl. the new golden rows and the handshake corpus,
  canonical-edit, `durable_group` — reseal that corpus only if its law goes red), plugin + hub-puzzle wasip2, the TS oracles,
  census (`channel-version check`: pin 23, one intended finding), then the 33-crate plugin census.
- **FUNNEL (opt-in `--with-funnel`, staged 10:32)**: second site of the F9 class, confirmed by code and repro — a B1 guest sends
  every `HistoryPatch` in struct order (`cursor, upserts, canUndo, canRedo, …`, `🔨️modules/🎠️kernel/🦀️.rs:2119-2131`) through
  `encode_wire_serialized` (`🔌️plugin/🦀️.rs:41703`, 47 uses) and `historyPatchBytes` (`📤️publication/🟦️.ts:134-138`) refuses what
  does not survive its decode → sorting re-encode (RAN `🗑️generated/s5-channel/f9-class-history-patch.ts`: key-byte order equal,
  struct order not equal; 0 hits of the fault in the run-6 probe logs, so the lane was not exercised with a non-null patch).
  The rows make the funnel order members by key bytes at every depth (`key_ordered_wire_value`) and add the law
  `typed_wire_values_leave_in_key_byte_order_whatever_their_types_declare` (dispatch tests): 2 files / 2 rows, 15 files / 63 rows
  with every group; dry-run clean, parses, NOT compiled. Lands only on the coordinator's "with funnel".
- Disk at 10:33: 16 GiB free (55 GiB at 09:39) — below the 25 GiB floor for `--tests`; the owed kernel test runs after wave C
  need the floor back. My scratch holds 0 build output (private uplift dir empty).

### S5.12 WAVE C LANDED — channel 23 (10:53:56 → 10:58:37, inside COORDINATOR-WAVE-C's `landing` + `serve` hold, apply only)

"WAVE C GO — WITH FUNNEL" at 10:53. Every dry run re-run on the live tree first (STORE `--check` 13 pending, NESTED `check` clean,
mine 15 files / 63 rows, both reseals) and mine again after the two peers' halves: no anchor had moved.

| Step | Command | Result |
| --- | --- | --- |
| 1 STORE | `python3 🧪️s5-store-revision-digest.py` | `applied 13 hunks in 6 files`, then `pending: none` |
| 2 NESTED | `python3 🧪️s5-nested-n2-wire.py land` | 4 files / 23 replacements, with frames |
| 3a | `python3 🧪️s5-channel-wave-c.py --apply --with-funnel` | 15 files / 63 rows written |
| 3b | `bun 🧪️s5-channel-reseal-canonical-edit.ts --write` | 4 fixtures written |
| 3c | `channel-version generate --guest` | 24 literals at 23 (2 min 24 s: its `git grep --untracked` under load 130) |
| 3d | `bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from 22 --to 23 --write` | 10 files written |
| 3e | `channel-version check` | pin 23, 41 consumers, 1 finding (intended: host fixture component at 20) |
| 3f | `python3 🧪️s5-channel-wave-c.py --verify --with-funnel` | 63 / 63 |

59 files in all. Train line 10:58:37 with the restore sequence (`reseal-canonical-edit.ts --restore` → `wave-c.py --restore` →
`generate --guest` → `reseal-channel-version-fixtures.ts --from 23 --to 22 --write` → `s5-nested-n2-wire.py restore` →
`s5-store-revision-digest.py --revert`; NESTED's restore writes whole pre-images of `🔌️plugin/🦀️.rs` and `🏪️store/🦀️.rs`).
"WAVE C ON DISK" sent 10:58.

RAN after the landing, cargo-free:

| Check | Result |
| --- | --- |
| `bun 🗑️generated/s5-channel/ts-oracles.ts` (canonical-edit fixtures + sealer self tests + composed-child oracle) | ok — 9 schema hostiles, 6 + 6 source hostiles, 37 reader checks, 4 composed cases |
| `bun ./📜️script.ts test-channel-oracles` (cwd `💻️os/📦️packages/🟦️typescript`) | 23 pass / 0 fail |
| `bun ./📜️script.ts test` (OS twin suite, same cwd) | 511 passed / 0 failed; the pre-existing "No test suite found" file aside (exit 1 for that alone) |

NOT yet seen by a compiler when this was written. The FRAMEWORK RED 10:56:33 in `train.status` is a mid-landing snapshot (kernel
compiled from the pre-landing tree, plugin runtime from the tree after step 2): its 12 errors name `owner` on the two entry
structs and the four-argument `send_member_mutations` / four-tuple `take_member_inbound`, all on disk since 10:54:06.

### S5.13 Wave C verdicts and what is owed (11:00 → 11:25; activation B2 building since 11:21, rule 68: no cargo started)

| When | Check | Result |
| --- | --- | --- |
| 11:10:31 | train FRAMEWORK check through 11:07:50 (past the wave-C line 10:58:37) | GREEN |
| 11:02:31–11:12:44 | `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-puzzle --target wasm32-wasip2 --lib` (gate v6, shared dir) | exit 0, Finished in 10 m 11 s, 0 errors; kernel + plugin recompiled on the channel-23 tree |
| 11:13:53–11:18:59 | 33-crate plugin census, `GATE_PATIENCE=300 zsh 🗑️generated/s5-channel/plugin-census.sh lib --lib` | gate closed (exit 5), NO cargo started |
| 11:19–11:21 | kernel laws in the private target + build dir | gate closed by `activation.flag`, NO cargo started (time file empty) |

The PUZZLE RED 11:11:27 (`🔌️plugin/🦀️.rs:29520/29531/29535`, `ChildDispatch`) was the Codex peer's `ChildDispatch<'wire>` wave
mid-save, not wave C (coordinator 11:22).

OWED after "SERVE UP (B2)", in this order (rule 68: test-build floor 18 GiB, ONE test build per owner):

1. Kernel laws, private target + build dir:
   `zsh T/🚦️gate.sh 3 18 && CARGO_TARGET_DIR=<repo>/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s5-channel CARGO_BUILD_BUILD_DIR=<repo>/.🧬semio/🦑️repo/⚡️cache/cargo/build-nde-s5-channel CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 cargo test -p semio-framework-os-kernel --lib --message-format=short -- os_spr:: os_store::`
   (wave-B tree: 849 passed / 5 failed, 3 of them peers'). Expect: channel suite incl. the three `…OfAMember` golden rows and the
   handshake corpus with `hostOffset`; canonical-edit unit / borrowed / reader laws on the resealed fixtures; `durable_group` 20
   (reseal its corpus only if the law goes red); then `--tests` check for the integration target (MCP test literals).
2. Funnel law `typed_wire_values_leave_in_key_byte_order_whatever_their_types_declare` on S5-RUNTIME's shared plugin test
   binary (`cargo test -p semio-framework-plugin --lib --features artifact-app-testing`), filter on the law's name — no build
   of my own.
3. 33-crate plugin census: `zsh 🗑️generated/s5-channel/plugin-census.sh lib --lib`.

Open items recorded:
- NEXT bump only (coordinator 11:22): delete `AppCommand::CommandText` — nothing sends it, a "not yet wired" stub.
- F19: the committed describe output of the host fixture component (`🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🔣️.json:3346`
  `appChannelVersion: 20` and `🛂️.descriptor.semio`, both of 10-01) is the one census finding. It needs the component rebuilt
  on the channel-23 tree (`@semio-tech/framework-plugin-host-fixture:component-dev`, wasm32) and described into that folder —
  build + describe are coordinator verbs; I verify after (`channel-version check` 0 findings, plugin-host `owned-instance-open`).
- F9 class, third reader (`🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts:230`, service-operation payload): LATENT,
  not live. The payload is an author-built `DslValue` encoded by the reactor in authored order
  (`🔌️plugin/⚛️reactor/🦀️.rs:1738`, `encode_wire_value(&payload)`, not through the funnel); its one producer sends an empty object
  (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/🎮️commands/💡️inference/🦀️.rs:26`), which no order can break. The first producer with two
  members out of key-byte order is refused by the TS reader; the fix is one call at the reactor line (order the payload's members
  like the funnel does) plus a golden row — for the next wave that touches the reactor, not for B2.
