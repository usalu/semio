# 📶️ Wave W0-F: presence summary of a peer's tool run

Contract: `📋️tool-run-contract.md` §3.4 (presence), §2.2 (state spelling), §2.3 (stage/completed/total shapes).

## What changed

### Replication kernel (`semio-framework-replication`)

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
  - `PresencePeer.tool_run: Option<PresenceToolRun>` (:1377).
  - `PresenceToolRunState` (:1385) with `ALL`, `wire_name`, `from_wire_name`; `PresenceToolRun` (:1426) with
    hand-written `ToValue`/`FromValue` (camelCase `toolId,state,stage,completed,total?`).
  - `PresencePeer` `ToValue`/`FromValue` carry `toolRun`.
  - Binary codec: flag **bit 10** = `tool_id str | state u8 tag | stage varint | completed varint | total bool + varint?`
    (`encode_presence_tool_run` :1672, `PresencePeerReader::tool_run` :1845). Unknown-flag guard widened to
    `flags >> 11` (:1870).
  - `PresencePeerWireLimitsV1.maximum_tool_run_units = 2^53-1` (:1704). Decoder rejects: unknown state tag,
    stage > u16, completed/total > 2^53-1, invalid total boolean, truncation, and `completed > total`.
- `📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs`: every literal gets `tool_run`; every-field-present now includes a tool run;
  unknown-flag test moved to bit 11; new `presence_peer_tool_run_round_trips_every_state_with_wire_spelling`
  (all 9 states × determinate/indeterminate, binary + value round trip, tag order, wire spelling, `idle` rejected).
- `🧬️schema/🔣️.json`: `definitions.toolRun` (state enum = contract §2.2), `peer.toolRun`, `limits.maximumToolRunUnits`.
- `🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json` (language-agnostic corpus, Rust + TS): every-field-present extended
  (flags `ff0f`, tool run appended); new accepted `tool-run-indeterminate`, `tool-run-faulted-boundaries`
  (tag 8, stage 65535, total 0); new rejected `tool-run-unknown-state`, `tool-run-stage-over-u16`,
  `tool-run-completed-over-exact-range`, `tool-run-total-over-exact-range`, `tool-run-completed-over-total`,
  `tool-run-invalid-total-boolean`, `tool-run-truncated`; `unknown-flag` moved to bit 11. 27 cases.
- `🟦️.ts`: `ArtifactPresencePeer.toolRun?` (:104), `PRESENCE_TOOL_RUN_STATES` (:109),
  `ArtifactPresenceToolRunState` (:112), `ArtifactPresenceToolRun` (:115), `writePresenceToolRun` (:436),
  `PresencePeerReader.toolRun()` (:592), flag guard `0x7ff` (:617), `PRESENCE_PEER_WIRE_LIMITS_V1.maximumToolRunUnits`.
- `📦️packages/🦀️rust/📜️script.ts` (`presence-peer-codec-check`): the source-law assertion read `📡️wire/🦀️.rs`, but
  the laws live in `📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs` since an earlier test split, so the oracle always
  failed after the vectors. It now reads the test file.

### OS store sync

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:500-507`: `artifact_actor_message_bytes` credits
  the tool run (id text, state + stage + completed + flag, optional total) so the mailbox byte cap stays exact.
- `…/🔄️sync/🧪️tests/🔬️unit/🦀️.rs`: literals get `tool_run: None` (the committed `🙋️client-presence`/`👥️server-presence`
  `.bin` specimens are unchanged); new `presence_tool_run_summary_is_byte_credited_and_last_writer_wins` (:1005):
  byte credit grows bare < indeterminate < determinate, decode(encode) identity via `presence_to_bytes`, and
  `PresenceHeartbeatProducer` coalesces starting → running → finalized to the newest (last-writer-wins unchanged).

### Hub (the only other language mirror found by `rg "drag_ghost_json|dragGhostJson" 🌎️hub`)

- `🌎️hub/🏗️bootstrap/🦀️.rs:1744`: `refresh_document_presence` passes `tool_run: input.tool_run` through
  normalization (ephemeral, client-owned like `ui`/`views`).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:5081`: socket normalization test asserts `tool_run` survives.
- `🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json`: new `tool-run-summary-passes-through` (accepted, normalized
  hex with flags `cd09`) and `tool-run-completed-over-total` (rejected); `unknown-peer-flag` moved to bit 11. 19 vectors.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `provePresenceNormalizationFixture`: independent LEB128 encoder handles
  index 10, output passes `toolRun`, source fence requires `tool_run: input.tool_run`; fixture path fixed from
  the non-existent `🧪️fixture/🔣️.json` to `🔣️.json`.

## Public API as landed

```rust
pub struct PresencePeer { /* … */ pub ui: Option<PresenceUi>, pub tool_run: Option<PresenceToolRun> }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceToolRunState { Starting, Running, Paused, Complete, Finalizing, Finalized, Aborting, Aborted, Faulted }
impl PresenceToolRunState {
    pub const ALL: [PresenceToolRunState; 9];
    pub fn wire_name(self) -> &'static str;
    pub fn from_wire_name(name: &str) -> Option<Self>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceToolRun { pub tool_id: String, pub state: PresenceToolRunState, pub stage: u16, pub completed: u64, pub total: Option<u64> }
pub struct PresencePeerWireLimitsV1 { /* … */ pub maximum_tool_run_units: u64 }
```

```ts
export const PRESENCE_TOOL_RUN_STATES: readonly ["starting", "running", "paused", "complete", "finalizing", "finalized", "aborting", "aborted", "faulted"];
export type ArtifactPresenceToolRunState = (typeof PRESENCE_TOOL_RUN_STATES)[number];
export type ArtifactPresenceToolRun = { readonly toolId: string; readonly state: ArtifactPresenceToolRunState; readonly stage: number; readonly completed: number; readonly total?: number };
// ArtifactPresencePeer.toolRun?: ArtifactPresenceToolRun
```

Re-exported explicitly next to `PresenceUi` in `os_spr` (`📡️spr/🦀️.rs:47`), `🎠️kernel/🦀️.rs:1114` and
`🧰️framework/📦️packages/🦀️rust/🦀️.rs:179`.

## Tests run (outputs under `T/🗑️generated/W0-F/`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-replication` | 273 passed, 0 failed (incl. 16 presence laws) |
| `cargo check -p semio-framework-replication --target wasm32-wasip2` | ok |
| `cd 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust && bun ./📜️script.ts presence-peer-codec-check --oracle-only` | 27 Rust/TS vectors, 23 hostile rejected (ajv schema + TS codec); red before the implementation (limits mismatch) |
| same with `SEMIO_TEST_ARTIFACT_DIR=<T>/🗑️generated/W0-F/cargo-laws bun ./📜️script.ts presence-peer-codec-check` | exit 0 (exact cargo laws receipts asserted) |
| `cd …/📡️replication/📦️packages/🟦️typescript && bun ./📜️script.ts test` | 6/6 passed (vitest; no presence tests live there) |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- presence artifact_mailbox actor_stamps` | 31 passed, 1 failed: `os_store::component::presence_retirement::tests::retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open` (mounted-worker `terminal_is_empty` in the generic `Value` presence store; no `PresencePeer` involved; fails identically when run alone, so it is not caused by this lane). All `os_store::sync` tests pass, incl. the new one |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- os_store::sync` (broader) | Aborts with failures in VCS and bootstrap tests, e.g. `receive_materializes_remote_envelope_into_the_edit_timeline`: `Vcs("validation failed: edit history insertion requires its exact mutation retirement factory")`, then a destructor panic in `📡️replication/🔗️causal/🦀️.rs:359`. No presence code is involved: that is a peer's mutation-retirement work in progress. The targeted presence, mailbox and heartbeat subset above passes. |
| `bun T/🐍️w0f-hub-presence-normalization-probe.ts` | 19/19 hub normalization vectors reproduced by the TS codec |
| `cargo test -p semio-hub --bin os-hub --no-default-features --features sqlite -- presence_normalization` | **Not run: the test binary does not compile, but the failures are not in this lane.** Attempts 1–4 failed in `semio-framework-ui` during W0-C's in-flight edits (`…/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:923` Progress arm; `…/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:242` stale MeasureProgress re-export). After W0-C fixed those, attempt 5 reached `semio-hub` (63 warnings) and failed on another peer's directory-session change: `E0432 unresolved imports directory::os_directory::DirectorySessionAuthorityV1, DirectorySessionKindV1` at `🌎️hub/🏗️bootstrap/🦀️.rs:31`, plus `E0308` at `bootstrap/🦀️.rs:3789` and `🧪️tests/🔬️bin-unit/🦀️.rs:705,3423,4110,4125,4210`. None of these errors is on the lines this lane changed (`bootstrap:1744`, `bin-unit:5081`). |
| `cargo check -p semio-framework-plugin --features artifact-app-testing --tests` | Type-checks to the end (243 warnings). The one error is a peer's: `E0599 no variant Progress for WindowMeasure` at `🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs:94`. No error on the builder-contract literal edit. 
`tsc --strict` over `📡️replication/🟦️.ts` shows only the pre-existing `import.meta.dir` environment errors, none on
the new lines.

The hub TS oracle `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts presence-normalization-check source` cannot load: a
peer file `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts` imports with one `../` too many.
The ticket probe above replaces it for this lane.

## Commands to register in launch.json

None new. Existing targets cover the lane: `@semio-tech/framework-replication-rs:presence-peer-codec-check`,
`…:presence-peer-codec-native-check`, and the hub `presence-normalization-check`.

## Deviations and notes for the coordinator

- **State enum duplicated on purpose.** `PresenceToolRunState` (Rust) and `PRESENCE_TOOL_RUN_STATES` (TS) repeat
  `ToolRunState` from W0-A because replication layers below `semio-framework-tool-run`. The binary tag is the
  declaration order of contract §2.2. When W0-A lands, add a conversion in the tool-run crate or the plugin runtime
  (`From<ToolRunState> for PresenceToolRunState`), plus a test that pins both spelling lists against each other.
- `stage`, `completed` and `total` follow §2.3 (`u16`, `u64`, `Option<u64>`). Values above 2^53-1 are rejected so
  the TS decoder stays exact.
- The decoder rejects `completed > total`. The producer (W0-D runtime) must clamp, for example when a reconfigure
  lowers the total.
- **Nothing fills `tool_run` yet.** The heartbeat peer is built in the plugin runtime (`P`, owned by W0-D), and
  the "Alice · Fill · 42 %" footer row belongs to the renderer. Both are open items for W0-D and the renderer lanes.

## Foreign edits

- Re-export lines: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs:47`, `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1114`,
  `🧰️framework/📦️packages/🦀️rust/🦀️.rs:179`.
- `tool_run: None` in struct literals: `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12079`,
  `…/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs:182`,
  `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3063`.

## Open items

- Rerun `cargo test -p semio-hub --bin os-hub --no-default-features --features sqlite -- presence_normalization` once the hub directory-session churn compiles. The TS probe already reproduces all 19 hub vectors byte-exactly. The Shell wgpu literal edits (`tool_run: None`) have not been compiled by this lane.
- The broader `os_store::sync` VCS/bootstrap failures and the presence_retirement mounted-worker failure (see the tests table) needs an owner.
