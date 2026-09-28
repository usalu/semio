# WP-H13 — Hub Backend Correctness and Security (successor of H11 + DB1)

Slice: H13 (session 14, 2026-09-27 18:2x). Coordinator = main chat. Rules: `📓️session-14-preamble.md`. Ports: hubs 8010–8019,
serves 6510–6519. Private cargo target: `.tmp-ticket/wp-h13/target` (build-dir `build-fleet-b`, native lane only). Captures:
`wp-h13/generated/` (expendable). Durable data/logs: `.🧬semio/🌐hub/s14-h13-*` (logs `s14-h13-logs/`). Handovers:
[📓️wp-h11.md](📓️wp-h11.md) (Session 13 table + B3 wave), [📓️wp-db1.md](📓️wp-db1.md) (Session 13 log),
[📓️audit-s13-hub.md](📓️audit-s13-hub.md), [📓️acceptance-s13.md](📓️acceptance-s13.md) §2.

## Session 14

| # | Item | Status |
|---|------|--------|
| 1 | H11's P0 agent ceiling + agent roles + check-in cause + interpreter cancellation: semio-hub `--all-features --lib --tests`, os-hub bin, os-mcp green on the current tree; new laws run | **DONE (14b):** semio-hub compiles on today's tree (H14 check 13:16; my bin/lib suites below); os-mcp check EXIT 0 12:40 + gateway 12:41; P0 bin laws 8/8, lib laws 7/7, os-mcp laws 8/8 (`hold4b-*.txt`) |
| 2 | Live P0 proof: `wp-g11/g11-refused-relay-probe.ts` against my own current-tree hub (fresh root) | Hub side proven 19:2x (16/16 vs 5/16 pre-P0). **14b MCP leg LIVE 12:15:** read agent binds with the 🔗️remote fix; edit agent relay-acknowledged; revoke → PERMISSION_DENIED; read agent's edit never reaches the hub, but the gateway answers SUCCEEDED on a local `plugin:note` session → G12. `agent-ceiling-check` on a current-tree hub **waits for ALL** (a channel-19 hub refuses the channel-18 B3 catalog) |
| 3 | Hub suite on the current tree (`os-hub:test`, `os-hub:test-all-features`; rows 2.1–2.3) | **DONE:** all-features EXIT 0 14:09 (lib 247 / 19 ignored, bin 176; `hold4b-all-features.txt`); **default features EXIT 0 14:35** (lib 247, bin 164; `hold5-hub-default.txt`) |
| 4 | C11/C12 routed defect: Check In refused `codec-refused` → root cause + fix | Fix landed 19:15; kernel laws 2/2 PASS 20:11; docstring restored 13:1x (rule 22); **guest law + restored viewer law 2/2 PASS 14:32** (`hold5-plugin-codec-law.txt`). Live re-proof = 7800 on ALL (C12 STEP 6) |
| 5 | pg/neo4j live gates (row 2.4): backend-up → two-client-e2e + document-growth-e2e pg/neo4j → backend-down | blocked until ALL: needs a channel-19 catalog (B3 refused by today's binary); all-driver binary ready (`s14-h13-bin/os-hub-all-drivers-1404`) |
| 6 | DB1 greeting-storm / storm-ratio (row 2.6) + permanent hostile-input/fuzz harness (row 2.13) | hold 5: every hostile law green in cargo (3 enumerated + 13 generative seeds, 0 findings) but both verbs published false `fail` (chunk-split line reader) → **harness fixed 17:0x** (`runLawProcess`), re-run = hold 6; storm: unit + fs green (fs storm/serial 0.39), **sqlite RED 0.64 > 0.5** (real) → see 14c log |
| 7 | C12 P1 (coordinator 20:0x): writers' post-cut batch refused `DB I/O aggregate admission exhausted` → all typing lost | (b) **DONE**: bin law + unit law PASS (hold 4b); live proof waits for ALL. (a) H14. G12 relay (transport wedge) root-caused → prepared patch `h13-transport-refill-patch.py` (window 3) |
| 8 | 14c: SQLite backend serializes every DB I/O step on one connection and writes (stage + WAL sync) for every read | **LANDED 17:31** WAL reader connections; law reads-beside-a-held-write-lock-and-never-write PASS; kernel-db lib 734/736 (reds = wall-ratio timing only); `os-hub:test-all-features` EXIT 0 18:18; storm turns ~700 → 513; wall ratio needs an idle machine |
| 9 | 14c: db shutdown hang (`PoolUse` deadline, stranded retirement cursor) | **LANDED 17:44** root fix `enter_terminal` wakes the retirement; law PASS, mutant reproduces the hang |
| 10 | 14c: close ring hands a backend's cleanup fault to unrelated callers | **LANDED 18:04** per-backend routing; law PASS 18:08; hub suite EXIT 0 18:18 |
| 11 | 14c: hostile-input / reopen-storm verbs misread chunk-split output | **LANDED 17:04** shared `runLawProcess`; hostile-input-check PASS 17:20 (row 2.13 green) |

### Session 14 log

- 18:2x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet → no freeze yet; hub
  crates stay open during it anyway), `📓️wp-h11.md`, `📓️wp-db1.md`, `📓️audit-s13-hub.md`, `📓️acceptance-s13.md` §2,
  `📓️fleet-13-agents.md` from 14:00. Load 8.5, 0 rustc.
- 18:3x **reconcile H11 (item 1):** H11's report stops at 15:5x ("NOT compiled yet"), but its capture dir shows the proof ran
  after: `s13-h11-logs/hubcheck-1.txt` `cargo check -p semio-hub --all-features --tests` **EXIT 0 16:09:57** (coordinator's
  out-of-lane exception), then `hold10-*` (outside the lane — `hold10-lane.txt`: the relative mutex path failed): bin laws
  **8/8 PASS 16:16** (`an_agent_session_holds_at_most_its_delegations_audience_in_its_delegations_space`,
  `a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order`, `an_agent_can_never_widen_…`,
  `only_an_author_of_the_space_can_delegate_to_an_agent`, `an_agent_delegation_mints_a_session_…`,
  `revoking_a_delegation_closes_…`, `credential_optional_routes_…`, `a_peer_beat_reaches_…`), lib laws **10/10 PASS 16:16**
  (both access-policy laws incl. `declared_access_policy_matches_the_language_neutral_truth_table`, the two cancellation laws,
  4 trusted-catalog neighbours, sqlite binding law), `build -p semio-hub --bin os-hub` EXIT 0 16:19, `check -p
  semio-framework-plugin-host --lib --tests` EXIT 0 16:19. Last hub Rust edit 15:56 (bin-unit); no `.rs`/`Cargo.toml` in
  `🧰️framework`/`🌎️hub`/`✏️s` changed after 16:19 (find -newermt). The session-13 final chain's hub-prewarm built
  `dist/build-dev/os-hub` 16:08 and os-mcp `dist/build/semio-os-mcp` 16:29 from this tree. Nothing half-applied → no edits
  needed; re-proof on today's tree queued: hold 1 (pid 45693, queue position 7) = check hub `--all-features --lib --tests
  --bins` + os-mcp `--lib --tests` + the bin/lib laws above + full `cargo test -p semio-hub --all-features` + os-hub build
  (`wp-h13/h13-hold-1.sh`, captures `s14-h13-logs/hold1-*.txt`).
- DB1 reconcile: last entry 05:01 (check-4); `s13-db1-logs/check-5.txt` db all drivers + async `--lib --tests` **EXIT 0
  06:33** → the storm/serial throughput-law redesign compiles; it was never run (item 6).
- 18:43 **TS oracles on today's tree** (no cargo): `🛡️access-policy` (valid HubAccessPolicyV1, every truth-table vector decided
  as declared — 160 vectors), `🚧️hostile-input` (5 cases incl. credential refusal + generative draws) and `📌️document-check-in`
  (2) — **9/9 PASS** (`s14-h13-logs/vitest-policy-2.txt`; `bunx vitest run --config 🌎️hub/🧪️tests/🎚️config/🟦️.ts …`).
- 18:47 **hub 8010** (`wp-h13/h13-hub.sh`, copy of H11's recipe): fresh root `s14-h13-hub-8010` = clone of catalog B3
  (generation `e3c0c98e…`) + users 1–3; binary `.🧬semio/🌐hub/s14-h13-bin/os-hub-h11-1619` = H11's 16:19 private-target
  build (built after the last hub Rust edit 15:56; no hub/framework `.rs` changed since → current tree), pid 55219, `/readyz`
  **200 in 34.1 s**, background verification 9/9 packages 146 s (log `s14-h13-logs/s14-h13-hub-8010-8010-1.log`). NB: the
  chain's `dist/build-dev/os-hub` has `builtAtMs` 15:50 (before H11's 15:54–15:56 edits) → NOT the P0 build.
- 18:5x **item 2, probe run 1** (`SEMIO_OS_MCP_BIN=<dist/build/semio-os-mcp 16:29> bun wp-g11/g11-refused-relay-probe.ts
  http://127.0.0.1:8010`, capture `wp-h13/generated/p0-refused-relay-1.txt`): space + note created, `read` delegation + agent
  session minted (hub spans ok), then the GATEWAY exits: `PermissionDenied: current hub space membership is required`.
  **Root cause (regression of the P0 fix):** the hub now answers an agent session's space page with the capped role
  (`read` → spectator) while the member rows stay the humans' (the delegating author = author); os-mcp
  `🔗️remote::validate_snapshot` required `space.role == membership.role` → every read agent (and any capped agent) is
  refused before its first call. The hub side is right (the role IS capped); the MCP rule was human-only.
  **Fix (os-mcp host only, not linked into guests):** `principal_role_admitted(session_kind, principal, membership)` — a human
  session holds exactly its member row's role, an agent session at most it (spectator below author), no principal ever more;
  `validate_snapshot` takes the session kind. Schema-first: 3 new cases in `🔗️remote/🧬️schema/🔣️.json` (required) +
  fixture `🔣️authenticated-hub-descriptor-index.json` (`agentBelowMembership` → ready, `humanBelowMembership` and
  `principalAboveMembership` → revoked/PERMISSION_DENIED; pages with receipts = sha256 of the unsigned canonical page,
  generated by `wp-h13/h13-mcp-role-cases.py`, idempotent). Rust law
  `an_agent_binds_at_its_capped_role_below_its_account_but_no_principal_differs_upward_or_as_a_human` (`🔗️remote/🧪️tests/🔬️unit`);
  independent TS oracle (`🌉️mcp/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts`: recomputes every page receipt + derives the
  admission rule itself) **4/4 PASS** (`s14-h13-logs/vitest-mcp-roles-1.txt`). Native check + law + gateway rebuild queued
  (hold 1). G12 told (their os-mcp edits are elsewhere), H14 told (no overlap with OwnedRuntime / residency).
- 19:0x **item 4 reproduced on my own hub** (serve **6510** = `s react dev` → 8010, `wp-h13/h13-serve.sh`, pid 85489;
  probe `wp-h13/h13-checkin-probe.mjs` (copies of C12's helpers as `h13-lib.mjs`/`h13-journey.mjs`), one human, fresh space,
  `text.document` (= `writer.document`), 3 typed rounds; capture `wp-h13/generated/h13checkin1-*`). The shell committed a
  checkpoint and its Check In was refused after 43 s; H11's cause-carrying span (`s14-h13-hub-8010-8010-1.log` line 187):
  `codec-refused: trusted artifact codec Output failed: guest trapped: wasm trap: unreachable executed — [semio-plugin panic]
  panicked at 🌿️vcs/🦀️.rs:622:9: artifact history ledger reached Drop before every exact entry owner was retired`.
  Ledger captured (`wp-h13/h13-ledger-capture.ts` → `.🧬semio/🌐hub/s14-h13-checkin-capture/writer-1/`: genesis pair 341 +
  248 B, 14 edits + 1 commit transition, 4190-B `encode_envelopes` stream).
  **Root cause:** `materialize_check_in` validates the folded pair with the codec's `print_mirror` (stage Output). The guest
  twin (`🔌️plugin/🦀️.rs` `artifact_codec_table::print_mirror`) parsed the pair and then `drop(envelope.into_owners())`: an
  unadopted envelope's history ledgers carry a terminal-empty Drop witness, so a pair with ≥ 1 edit aborts the guest (the
  genesis Input validation has empty ledgers, which is why creation always passed). The native twin
  (`🏪️store` `print_mirror_impl`) already retires with `ArtifactEnvelope::retire_unadopted()` — hub-native GIS check-ins pass.
  Same latent abort in the zero-op `apply_ops` pass-through of both twins (plugin `artifact_app_apply_ops`, store
  `apply_ops_binary_impl`) for any populated pair.
  **Fix LANDED 19:15** (pre-freeze; guest-linked; `wp-h13/h13-checkin-retire-patch.py`, idempotent, dry-run clean after):
  the 3 sites call `envelope.retire_unadopted()`; laws: store `document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store`
  extended (a populated pair passes a zero-op batch + mirrors) and new plugin
  `the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting` (guest twin, `EditorApp<SurfaceEditorFixture>`:
  genesis → one op → populated pair → mirror + zero-op batch) — both red without the fix by construction. Native check +
  laws = first steps of hold 1; wasm32 = the chain's fast gate. Main + C12 told. **Same bug class, NOT fixed (routed):**
  `💻️os/🖥️host/🦀️.rs:522` `with_backbone_envelope` drops a backbone document's populated envelope the same way.
- 19:2x **item 2, hub side — permanent check (rule 17):** `bun ./📜️script.ts agent-ceiling-check --hub <url> [--kind …]
  [--locale en|de]` in os-hub-ts (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts` `AgentCeilingCheckScript`, record
  `hub-agent-ceiling` via `withAcceptanceRecord` + `publishAcceptanceCheckResult`, en + de summary, `blocked` without hub or
  password; credentials only from env `OS_HUB_PROBE_EMAIL`/`_PASSWORD`/`_MEMBER_EMAIL`), logic in the NEW
  `🌎️hub/🧪️tests/🤖️agent-ceiling/🟦️.ts` (taxonomy registration → R10 relayed). It drives the hub directly (no gateway):
  one human, two private spaces, one note, a `read` and an `edit` delegation exchanged for agent sessions exactly like the
  semio MCP (`POST /auth/agent-sessions`), then 16 boundaries. Runs: `agent-ceiling-8010-1.txt` (harness: delegation answers
  201), `-2.txt` (harness: Bun fires `close` synchronously inside `socket.close()`, so the verdict was overwritten — fixed
  with a settled flag), **`-3.txt` (de) 16/16 PASS, rc 0**. Contrast on a pre-P0 hub (**8011** = 7800's B3 binary
  `s13-w3-bin/s13-w3-hub-7800-b3/os-hub`, fresh root, ready 67 s, stopped after): **`agent-ceiling-8011-pre-p0-1.txt` 5/16,
  rc 1** — the check detects the P0 (read agent edit accepted head 0→1, foreign space listed + readable 200, rename /
  upsert-member / create-invite 202 for both agents). ⇒ 7800 keeps the P0 open until the chain moves it to a current-tree
  binary. R10 spec relayed (target `agent-ceiling-check`, criteria 2.13).
- 19:4x **item 6:** DB1's storm/serial redesign is already a permanent verb (`reopen-storm-check [unit|fs|sqlite|all]` in the os
  kernel `📜️script.ts`, record `hub-reopen-storm`, pg/neo4j entries under `os-hub-ts backend run`) → run in hold 2
  (`wp-h13/h13-hold-2.sh`). Fuzz (row 2.13): H12's generative law exists (`🎲️hostile-generative`, fixture seeds 1–3, env
  `SEMIO_HUB_HOSTILE_SEED`), H14 confirms fuzz is mine → NEW permanent verb `hostile-input-check` in the os-hub
  `📜️script.ts`: the Ajv oracle, the draw-vector / fixture-coverage / typed-refusal laws and the generative law once per seed
  (`--seeds a,b` or `--seed-range a..b` for a longer fuzz run), record `hub-hostile-input` with requests, socket sequences
  and failing seeds (en + de). tsc over 🌎️hub 0 errors (`tsc-os-hub-ts-2.txt`). Queued: fixture seeds + `--seed-range
  100..109` (hold 2, pid 65301). Found in passing (not mine, routed via this report): three `[DEBUG]`-tagged
  `console.log` status lines in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` (84, 187, 2145).
- 20:0x **coordinator relay C12 P1** (writers' whole outbox after a link cut → `Rejected{reason:"unavailable: DB I/O aggregate
  admission exhausted", messages:[]}` → client rolls back → every keystroke of the cut lost): split — (a) the db engine must
  admit a declared-legal batch (per-operation credit 64 pages / 16 controls in `🛢️db/🗄️storage/🦀️.rs:262`
  `db_io_operation_add`) → routed to H14 (db engine, frozen soon, my lane queue full); (b) mine, hub-only, schema-first:
  `🚧️refusal/🧬️schema` gains `HubTransientApplyRefusalCodeV1` (const `hub.unavailable`) and
  `HubTransientApplyRefusalMessageV1` (level warning, code, message ≤ 1024 chars, no target / opIndex); Rust
  `semio_hub::refusal::{HUB_TRANSIENT_APPLY_REFUSAL_CODE, hub_transient_apply_refusal_message}` + unit law
  `the_transient_apply_refusal_is_the_declared_schema_message`; bootstrap `messages_for_error` answers `DbError::Unavailable`
  with that message and the agent rate-limit Ack carries it too (`transient_apply_refusal_messages`); fixture
  `🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1` (causes, permanent causes, 5 near misses); bin law
  `a_transiently_refused_batch_names_the_declared_resend_code_and_a_permanent_one_does_not`; Ajv oracle case in
  `🧪️tests/🚧️hostile-input/🟦️.ts` → **6/6 PASS** (`vitest-transient-1.txt`). C12 told the code (`hub.unavailable`).
- 20:04 **hold 1 step 1: `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib --tests` EXIT 0** (16 min,
  warnings = type-checked; `hold1-check-codec.txt`) → the Check In fix compiles natively. Kernel law build running.

### Session 14b

Successor agent (2026-09-28 12:0x, after the usage cut + app restart; guest freeze ON since 12:02:46).

- 12:1x **reconcile.** Every predecessor edit is complete and was auto-committed (`5bcb2da23da` 21:54): `🌎️hub` has no
  diff vs HEAD and no overnight change; the transient refusal (schema `🚧️refusal/🧬️schema` 20:07, `🚧️refusal/🦀️.rs` +
  unit law 20:07, fixture `⏳️transient-apply-refusal-v1` 20:08, bootstrap `transient_apply_refusal_messages` + bin law
  `a_transiently_refused_batch_names_…`, Ajv case 20:10) is whole; os-mcp `🔗️remote/🦀️.rs` unchanged since 18:51. No `[DEBUG]`
  in `🌎️hub` (`git grep`). Hold captures after the last log line (all `s14-h13-logs/`): `hold1-check-mcp` **EXIT 0 20:48**,
  `hold1-os-mcp` gateway build **EXIT 0 20:59**; `hold1-check-hub`/`-bin-laws`/`-all-features`/`-os-hub` and `hold2-*` hub
  steps **EXIT 101 on the db crate** (`semio-framework-os-kernel-db`: `🛢️db/🗿️artifact/🦀️.rs` 20:41 uses an undefined
  `PlannedEntries` and a removed `ArtifactEngine::apply_one` — H14's cut WAL edit; still red now, the chain's hub-prewarm
  failed on it 12:03) → **no hub proof on today's tree yet**; `hold1-mcp-laws` EXIT 101 on a peer's `semio-framework-ui`
  `colors::DIFF_ADDED`; `hold1-plugin-codec-law` **FAILED** (my law's genesis id `surface-codec` is not a server-minted
  artifact id — test construction, not the fix); the predecessor corrected the id (`artifact-5c0dec0de…`, 32 hex) and
  re-ran it in hold 2, which did not compile (a peer's broken `include_str!` in `🔬️app-window-kits`, since fixed). Hostile /
  storm runs in hold 2 were red only because their cargo hit the db crate.
- **Docstring defect of my 19:15 landing:** the law was inserted between the `👁️🔒` docstring of
  `viewer_rejects_every_contract_mutating_verb` and its function (patch anchor = the function head) → that docstring now heads the
  codec law and the viewer law has none. Plugin crate frozen → prepared patch `wp-h13/h13-docstring-restore.py` (idempotent,
  `--dry-run` clean: "would move"). `h13-checkin-retire-patch.py` realigned to the live tree (server-minted id, docstring-safe
  anchor): it now refuses instead of inserting a second copy, and reports "applied already" after the restore (simulated).
- 12:1x db blocker relayed to main (owner H14, confirmed by the coordinator). Hold 3 (db-independent: os-mcp check + role law,
  gateway build, plugin codec law) queued 12:14 (`h13-hold-3.sh`, pid 51877).
- 12:15 **hub 8010 restarted** (P0 binary `s14-h13-bin/os-hub-h11-1619`, root `s14-h13-hub-8010`, pid 52780, ready 13.0 s).
  **G11 probe with the 🔗️remote-fixed gateway** (`wp-h13/target/debug/semio-os-mcp` 20:59; capture
  `wp-h13/generated/p0-refused-relay-2.txt`, rc 0): the read agent now **binds** (yesterday: gateway exit `PermissionDenied`) ✓;
  edit agent `relay:acknowledged` head 0→1 ✓; after revoke `PERMISSION_DENIED hub session is unauthorized` ✓; read agent's edit
  never reaches the hub (head 0) ✓ — but the gateway answers it `SUCCEEDED` on its local `plugin:note` session
  (`stamped_artifact_id` fallback) → MCP truthfulness defect, relayed to G12 via main (not hub).
- 12:40 **hold 3** (`hold3-*.txt`): os-mcp `check --lib --tests` **EXIT 0 12:40:01** (incl. a peer's 12:14 🏠️workspace edit), gateway
  build **EXIT 0 12:41:57** → "os-mcp green 12:40" sent to main. Laws 7/8: my role law **PASS**; neighbour
  `authenticated_hub_catalog_hydrates_exact_selected_descriptor…` FAILED on the lease corpus' `appChannelVersion` 18 vs the pin 19
  (fixed by the channel-19 re-derivation below; re-run pending). Plugin guest codec law: compile blocked by 6 peer errors in plugin lib
  tests (`🔬️app-window-kits`, `🔬️plugin-runtime-plugin-builder-contract`) → window 3. Hub 8010 stopped 12:4x (no current-tree binary yet).
- 12:45 hold 4 queued (`h13-hold-4.sh`: semio-hub all-features check first, all-driver os-hub build, full all-features suite).
- 12:5x–13:0x **channel pin 19 (coordinator, rule 22):** `wp-h13/h13-channel19.py` re-derived my 4 channel-derived fixtures with the
  17→18 oracles, each first proven equal at 18 (plan generation `97f393c7…`→`28ebc3e4…`, frozen binding `25a58ad6…`→`1643c1dd…` + its
  2 quotes + the inference identity chain in 5 files, lease/browser literals). `channel-version check`: 7 → 3 findings (H14's).
  TS oracles all rc 0 after fixing 3 stale hub-script oracles (lease status vocabulary lacked the contract's `retrying`; GIS approval-undo
  source oracle still named the pre-09-25 MCP `HubGisMapApproval`; ingress oracle predated the 09-26 retained-guard redesign). Hub-script
  tsc: 0 errors in os-hub-ts (G12's 6 = compiling without `🦀️rust/📐️ambient.d.ts`; lodash-es/leb128 are test-only oracles).
- 13:1x docstring restore applied (rule 22, test-only): `h13-docstring-restore.py` ("already applied" after), check-in patch "0 pending".
- 13:2x **G12 relay — hub-lane MCP "transport" wedge: ROOT CAUSE.** The gateway's `HttpPool` byte bucket never refills:
  `NativeDirectoryTransport::with_new_http_pool_now` never starts `HttpPool::spawn_refill_driver` (only services tests call it;
  `HTTP_BUCKET_REFILL_INTERVAL_MS` is dead code) → the 80 MiB "per minute" budget is a lifetime budget → after 3 components + the
  catalog refreshes every request fails `ByteBudgetExhausted` → `TransportError::Io(detail)` → mapped to a detail-less
  `HubUnavailableCause::Transport` → permanent. Same latent bug in the wgpu shell's directory client and the renderer probe (same
  constructor). Run 2's "4 registered plugins" = `🏠️workspace::open_hub` builds the Catalog once → G12 owns that. **Prepared patch
  (window 3)** `wp-h13/h13-transport-refill-patch.py` (dry-run: 21 pending, 0 problems): services `TokioHostRuntime::worker_pool()` +
  pub `HTTP_BUCKET_REFILL_INTERVAL_MS`; kernel constructor starts the refill driver + law
  `an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn` + fixture `🔁️byte-budget-refill.json`; os-mcp
  🔗️remote `Transport { detail }` (≤ 512 chars) + refusal `details.cause`/`summary` (en + de), schema `HubUnavailableCauseV1` /
  `HubUnavailableRefusalDetailsV1` / corpus def, generated fixture `🔣️hub-unavailable-refusal.json` (6 cases), laws
  `a_hub_unavailable_refusal_names_its_typed_cause_in_english_and_german` + `a_transport_fault_keeps_its_cause_and_the_next_refresh_recovers_the_binding`,
  Ajv/independent-derivation oracle case. Proven now: patched schema + corpus under Ajv strict (corpus valid, index fixture still valid,
  overlong detail and missing `de` refused; scratch check). Rust compile + laws: window 3, native lane.
- 13:31 hold 4 re-queued with the coordinator's priority stamp (`FLEET_TICKET_STAMP=20260928120004`, hold 4b; old waiter stopped).
  **Results (`hold4b-*.txt`):** hub bin laws **8/8 PASS 13:45** (the 7 P0 agent/delegation/revocation/credential laws +
  `a_transiently_refused_batch_names_the_declared_resend_code_and_a_permanent_one_does_not`), hub lib laws **7/7 PASS 13:47**
  (`refusal::` ×3 incl. `the_transient_apply_refusal_is_the_declared_schema_message`, both access-policy laws incl. the 160-vector truth
  table, both interpretation-cancellation laws), os-mcp laws **8/8 PASS 13:48** (role law + the catalog-hydration law, green again
  after the channel-19 lease fix), all-driver `os-hub` (sqlite + postgres + neo4j) **EXIT 0 14:04** → `s14-h13-bin/os-hub-all-drivers-1404`;
  full all-features suite running. (H14's semio-hub lib+bins+tests check EXIT 0 13:16 already covered this code.)
- 13:5x lease corpus `descriptorHex` re-sealed at 19 with its own generator (coordinator relay from H14): census **29 consumers, 0 findings**;
  lease / actor-identity / open-plan TS oracles rc 0.
- 14:05 **live proofs on a current-tree hub are blocked until ALL is published:** hub 8010 on the 14:04 binary + fresh B3 clone
  (`s14-h13-hub-8010-b`) refuses to boot: `package semio:animate was published for app channel 18 but this hub speaks app channel 19`
  (log `s14-h13-hub-8010-b-8010-1.log`; process exited, nothing left running). ⇒ `agent-ceiling-check`, the transient-refusal live proof
  and the pg/neo4j e2e gates run against a clone of the chain's ALL catalog after `final-publish.rc`.
- 14:0x T14: plugin lib tests compile again → the guest codec law + os-mcp lease readers are the first steps of hold 5 (queued 13:3x).

### Session 14c

Successor agent (2026-09-28 16:5x, after the 14:37 usage cut + app restart; chain relaunched 16:55:46, guest freeze ON).

- 17:0x **reconcile.** No half-applied hunk: `git status` clean for `🌎️hub` and os-mcp `🌉️mcp`, everything auto-committed
  (`dfe2687f7db` 16:29); newest hub files 13:52–13:58 (lease corpus re-seal, H14's trusted-catalog/two-author fixtures, hub
  script). The predecessor's hold 5 (`h13-hold-5.sh`, native lane) had **finished 14:35:39 rc 0** before the cut; results
  (captures `s14-h13-logs/hold5-*.txt`):
  - guest codec law `the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting` + the restored
    `viewer_rejects_every_contract_mutating_verb` **2/2 PASS** (EXIT 0 14:32) → Check In fix proven on the guest twin.
  - os-mcp `authenticated_hub*` lease readers **9/9 PASS** (EXIT 0 14:33; lease corpus at channel 19).
  - `os-hub:test` (default features) **EXIT 0 14:35**: lib 247 pass, bin 164 pass.
  - `hostile-input-check` (fixture seeds) and `--seed-range 100..109 --locale de`: **every cargo law run printed
    `test result: ok. 1 passed`** (13/13 in the fuzz capture; each generative seed `396 requests, 8 socket sequences,
    0 findings`), yet the verb published `FAIL` ("enumerated laws 2/3", "792 requests" for 3 seeds; fuzz "findings at
    seeds 103, 108" with no finding printed). **Harness defect:** the law runner split every output CHUNK on `\n`, so a
    line cut between two pipe chunks (or interleaved with stderr) never matched the verdict / count regexes.
  - `reopen-storm-check all`: unit PASS (24 grown docs, storm 258 ms, welcome max 217 ms); fs **PASS in cargo** (storm/serial
    0.39) but published failing — same chunk-split reader in the os kernel verb; **sqlite RED for real**: `24 greetings at
    once took 680 ms, one after another 1061 ms: the storm serializes` (ratio 0.64, bound `stormToSerialRatioMax` 0.5;
    storm census 456 tasks / 726 turns vs serial 456 / 465; IndexRead turns 243 vs fs 84, IndexList 152 vs 96).
- 17:0x **harness fix (TS only, hub + os-kernel script + acceptance module; not guest-linked):** one shared law runner
  `runLawProcess(command, args, {cwd, env}, onInterrupt?)` in the acceptance orchestration module (UTF-8 decoding per stream
  across chunk boundaries, per-stream unfinished tail, SIGINT forwarding) replaces both hand-rolled collectors
  (`HostileInputCheckScript` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `ReopenStormCheckScript` in the os kernel
  `📜️script.ts`). tsc: os-hub-ts project **0 errors** (`tsc-os-hub-ts-6.txt`, covers the hub script + the acceptance module);
  os kernel script scoped (`wp-h13/tsc/tsconfig-os-kernel-script.json`) 54 pre-existing errors (53 TS18046 + 1 TS7016 in a
  pack test), none in the edited region (`tsc-os-kernel-script-1.txt`). Transport-refill patch **dry-run clean 17:02**
  (21 pending, 0 problems).
- 17:06 hold 6 (`h13-hold-6.sh`, native lane, pid 84058): hostile-input-check fixture seeds, reopen-storm fs+sqlite, sqlite ×2.
- 17:20–17:23 **hold 6 results** (`hold6-*.txt`, load 47–52 with the chain's hub-prewarm): `hostile-input-check` **PASS**
  (oracle, enumerated 3/3, generative 3/3 seeds, 1188 requests / 24 socket sequences — the fixed reader now counts all 3 seeds)
  → row 2.13 green on today's tree. Storm laws RED under load for BOTH backends: fs storm/serial **0.88** (was 0.39 at 14:34),
  sqlite 0.78 and **1.41** → the ratio is not load-independent: with every core taken by the chain a storm gains no
  parallelism; no verdict on row 2.6 until an idle machine. Structural sqlite serialization regardless of load
  (`🛢️db/🗄️storage/🪶️sqlite/🦀️.rs`): ONE `Connection` behind a `Mutex` serves every DB I/O step of every worker, and every
  read (WAL/snapshot/index/payload) stages through `db_io_stage` = an INSERT + DELETE write per read (WAL `synchronous=FULL`
  → a WAL sync per read). Proposed (asked main 17:2x, kernel-db is hub-closure-only): WAL reader connections — single-statement
  reads on any free reader, multi-step blob reads pinned to one reader inside one read transaction (one snapshot, no staging
  write), writes + catalog stay on the writer; in-memory databases unchanged.
- **NEW db defect (routed H14 via main 17:2x):** sqlite run 3 died in the throughput law's post-serial `database.shutdown` —
  `database shutdown deadline elapsed in phase Some(PoolUse): … retained pool-use owners 2 … artifact_retirement: 1` after
  120 s (`hold6-reopen-storm-sqlite-3.txt`, `⏱️throughput/🦀️.rs:245`); 1 in 4 sqlite runs today.
- 17:3x **coordinator GO** for the SQLite readers (kernel-db is hub-closure-only; land now, green before `final-publish.rc`, else
  revert + window 3) and **H13 owns the shutdown hang** (H14 wave B).
- 17:31 **SQLite WAL readers LANDED** (`wp-h13/h13-sqlite-readers-patch.py`, idempotent; chain hub-prewarm had ended 17:13):
  `🛢️db/🗄️storage/🪶️sqlite/🦀️.rs` — `SQLITE_READERS` = 16 read-only WAL connections (lazy, `cache_size` 1 MiB); one-statement and
  listing reads (`query_step`, shared with the writer path) on any free reader; blob reads (WAL range, snapshot, index run,
  payload) pinned to one reader inside one read transaction, page by page with SQLite incremental blob I/O on the located rowid
  (`snapshot_blob_step`/`blob_page_step`: no `db_io_stage` write, no WAL sync, one snapshot); a read that already progressed on
  the writer stays there; every reader held → the writer's staged path; catalog root + leases + in-memory databases unchanged;
  operation close ends/rolls back a pinned read; backend close drops the readers. rusqlite `blob` feature (kernel-db
  `Cargo.toml`, features only — no lock change). Law `file_reads_run_on_wal_readers_beside_a_held_write_lock_and_never_write`
  (page-lifecycle lengths; an independent rusqlite connection holds `BEGIN IMMEDIATE` while 12 concurrent reads run; bytes equal
  the oracle's own reads; oracle `data_version` + `-wal` length unchanged).
  - hold 7: `check -p kernel-db --features sqlite --lib --tests` **EXIT 0 17:32** (warnings = type-checked). sqlite laws **3/8**:
    every failure `database is locked` — **root cause:** operation cleanup (`close_operation_step`) ALWAYS ran
    `DELETE FROM db_io_stage` on the writer, so even a read that never staged took the write lock at close and waited behind the
    oracle's lock (5 s busy timeout ×N → 405 s); and the lane's process-global `db_io_maintenance_step` (close ring) returned that
    foreign backend's cleanup error to UNRELATED in-memory tests' `close()`/`len()` (see finding below). **Fix 17:42:** cleanup
    deletes only when a stage row exists (`SELECT EXISTS` first — a read on the writer, no lock).
  - **Finding (lane isolation, db storage, not fixed — route H14):** `db_io_maintenance_step()` (`🗄️storage/🦀️.rs:5353`) drives
    the global close ring; `close_db_io_backend` (3519) and `db_io_wait_task_retirement` (5328) return ANY backend's cleanup error
    to their unrelated caller (observed: in-memory storage `close()` → `Io("database is locked")` of another file backend).
- 17:44 **shutdown hang ROOT CAUSE + fix LANDED** (`wp-h13/h13-retirement-wake-patch.py`): the census (`runner_handoff: 0`,
  `artifact_retirement: 1`) says the runner HAD gone terminal (the handoff's pool use is surrendered only in the terminal
  transition) while its retirement cursor still held a `WorkerPoolUse`. Sequence: the retirement hook's turn ran `close_one`
  while a normal turn was `Polling` → not terminal → `Idle`; that normal turn then completed the close inside
  `ArtifactRunner::finish`, whose terminal transition stored `Terminal` without requesting the hook, and the turn guard returns on
  `Terminal` without a wake → cursor stranded, shutdown spins in `PoolUse` to its 120 s deadline. Fix: the transition moved into
  `ArtifactRunnerHandoff::enter_terminal` (surrender pool use → publish `Terminal` → `request_retirement_maintenance`). Law
  `a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement` (commits a real retirement cursor on a
  2-worker pool, waits until its first turn went idle, calls `enter_terminal`, requires the slot + pool use released within 30 s;
  red without the wake by construction).
- 17:48–17:53 **results.** hold 7: semio-hub `--all-features --lib --bins --tests` check EXIT 0 17:47 (may predate the two
  later edits → re-run in hold 8); sqlite storm law (load 40): storm/serial **0.52** (was 0.78 / 1.41 under the same load
  before), storm turns 513 (was 682–726; IndexRead turns 110 vs 243, WalRead 81 vs 163) — but the law failed earlier on
  `late commits ack p50 364.1 ms vs early 134.9 ms` (write-side timing, load-bound); kernel-db full lib **734/735** (EXIT 101
  only on the sqlite throughput timing law; every sqlite storage law incl. the new one PASS, the new retirement law PASS)
  (`hold7-db-lib.txt`). hold 8: retirement law **mutant FAILS** (`driver=9 terminal=true turns=1 maintenance=true`, the exact
  hang) → restored → **PASS**, reader law PASS (`hold8-retirement-law*.txt`). Timing laws (storm/serial, late/early) need an
  idle machine: re-run after the chain.
- 17:55 hold 8: semio-hub `--all-features --lib --bins --tests` check **EXIT 0 17:55** after every db edit (`hold8-check-hub.txt`);
  full `os-hub:test-all-features` on the reader path running.
- 17:56 **coordinator: the close-ring finding is mine** (H14 wave B). 18:04 **LANDED** (`wp-h13/h13-cleanup-fault-routing-patch.py`,
  applied after hold 8's kernel-db compile had finished): root cause — `db_io_task_close_step` (maintenance class 5, the
  process-global close ring) returned a backend's operation-cleanup error (`close_operation_step`, `db_io_backend_return_operation`)
  to whichever caller drove the maintenance step AND left the failing task at the ring head, so every other backend's
  `DbIoTaskOperation::finish` (task-retirement wait) and `close_db_io_backend` failed with the foreign fault and could not retire
  behind it. Now: the ring records the fault on the failing backend's registry slot (`cleanup_fault: Option<DbIoFault>`, fixed
  authority, kind preserved via `db_io_task_fault`/`into_db_error`), rotates the task and retries it; a successful cleanup of that
  backend clears it; only that backend's task waiters (`db_io_wait_task_retirement` via the task's `backend`) and its
  `close_db_io_backend` take and report it (`db_io_note_backend_cleanup` / `db_io_take_backend_cleanup_fault`). Law
  `a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close` (`🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs`, fixture
  executor `CleanupFaultLawExecutor`): faulty + healthy backend on one pool; the faulty op's own `finish` = the injected `Io`;
  the neighbour's `finish` = `Ok(Unit)`, drain + close `Ok`; the faulty close = the injected `Io`; after the fault clears its drain +
  close `Ok`; ledger witness unchanged (red on the old ring by construction: the stuck head hands the neighbour's retirement wait
  the foreign error). kernel-db `--features sqlite --lib --tests` check **EXIT 0 18:05** (`hold9pre-check-db.txt`, lane slot 2,
  private throwaway target deleted after). Hold 9 (queued behind hold 8): routing law, full kernel-db lib, semio-hub check.
- 18:07 hold 8: **`os-hub:test-all-features` EXIT 0 18:07** on the SQLite readers + retirement wake (lib 247 / 19 ignored, bin 176;
  `hold8-hub-all-features.txt`). hold 9: kernel-db check EXIT 0, **routing law PASS 18:08**, full kernel-db lib 734/736, semio-hub
  all-features check **EXIT 0 18:10**. hold 10 (`hold10-reopen-storm-fs-sqlite.txt`, load 43): the 2 reds are the WALL-clock
  storm/serial ratio only — fs 0.83, sqlite 0.91 (fs was 0.88 at 17:21 before any db edit); every census bound before it holds
  (late/early, ack/durable, storm tasks = serial tasks 456). ⇒ functional db suite green; the wall ratio is load-bound by design
  (with every core taken a storm cannot overlap) — **recommendation:** bound the storm by a load-free measure (census turns on the
  storm's critical path / peak concurrent DB I/O steps) and keep wall time as a report; until then judge row 2.6 on an idle machine.
- 18:18 hold 11: **`os-hub:test-all-features` EXIT 0 18:18 on the final kernel-db** (readers + retirement wake + routing; lib 247 /
  19 ignored, bin 176; `hold11-hub-all-features.txt`). 18:25 hold 12: all-driver `os-hub` (sqlite + postgres + neo4j) from today's
  tree **EXIT 0** → `.🧬semio/🌐hub/s14-h13-bin/os-hub-all-drivers-1818` (rm + cp + codesign) — the binary for the live gates.
- **State at 18:3x / next:** every H13 edit is landed and compile/test-green (landing rows 17:04, 17:31–17:42, 17:44, 18:04); no H13
  process running; transport-refill patch dry-run clean (window 3). Blocked on the chain: after `final-publish.rc` = 0, clone the ALL
  catalog (path from W4/main) → hub 8010 on `os-hub-all-drivers-1818` → `agent-ceiling-check` (en + de), transient-refusal live proof
  (C12 writers' cut), `two-client-e2e` + `document-growth-e2e` on **sqlite** (`OS_HUB_STORAGE_BACKEND=sqlite` = the new reader path
  live), then pg + neo4j (`backend-up` / `backend-down`); storm law on an idle machine.
- 21:10 **LIVE PROOFS on p24 (WINDOW 3 open).** Hub 8010 = clone of `s14-w4-catalog-p24` (generation d1099ba9…) on
  `os-hub-all-drivers-1818` (root `s14-h13-hub-8010-p24`, ready 13.5 s). **`agent-ceiling-check` 16/16 PASS en + de**
  (`agent-ceiling-8010-p24-{en,de}.txt`, `s.note.note`).
- 21:12 **transient refusal live** (new probe `wp-h13/h13-transient-probe.ts`: one writer socket sends its outbox as ONE `Commands`
  batch per round, resends on `hub.unavailable`, an observer socket must receive every envelope): run 1 (600-envelope batches) →
  **P1 FOUND:** refused `unavailable: limit exceeded: artifact submit batch item credit` + `hub.unavailable` — the engine capped a
  submit at 256 envelopes (the wire declares 8 192 / 256 KiB) and `SubmitFuture::submit` wrapped that size-permanent refusal as
  `Unavailable`, so the client resends forever (C12's lost typing, second half). Run 2 (256-envelope batches, debug build): 23
  batches commit, 1.0 → 4.3 s each as the doc grows; batch 24 committed (head 6 144) but its Ack never came within 60 s → H14
  (fsync barriers per index-run replacement; 30 s socket frame deadline drops the Ack — H14 fixing).
- 21:24 **P1 root fix LANDED** (coordinator GO; landing row): declared credits, kind-preserving admission error, permanent typed
  `hub.batch-limit` refusal (schema-first) for undeclared batches and every engine `LimitExceeded`. Hold 13: checks EXIT 0,
  hub/refusal laws PASS, `os-hub:test-all-features` **EXIT 0 21:55** (lib 248 / 19 ignored, bin 178), TS oracle 7/7; engine law's
  first run hit the TEST profile's 64-command cap (`LimitExceeded("db_artifact::batch_commands")`) → law now opens the hub's
  `Profile::Prod`; isolated re-run queued. Binary `os-hub-all-drivers-2155`.
- 22:06 **P1 proven live** (hub 8010 on a fresh p24 clone + binary 2155, `transient-probe-8010-3.txt`): 12 batches × 1 100 envelopes
  all commit (~300 ms each); from batch 7 the socket budget is empty → `hub.unavailable` ("dos budget exceeded"), resent every 2 s,
  commits on attempt 5/10 — nothing lost; an over-declared 1 400-envelope batch → **permanent `hub.batch-limit`** (batch-bytes).
  **NEW P1 → H14:** reopening that 13 201-edit document fails `limit exceeded: database sync hello cumulative envelope backing`
  (🔄️sync hello decodes the whole tail since the last snapshot, ≤ 65 536 items ≈ 8–9k envelopes).
- 21:17 two-client e2e sqlite (binary 1818): 1/2 — the directory-page step got an empty body (`JSON Parse error`, test line 207);
  a 60-space burst on a sqlite hub is clean (`h13-space-burst.ts`, 0 refused, 2.0 s) → re-run pending on binary 2155.
- 22:10–22:29 **e2e on sqlite (binary 2155):** `two-client-e2e sqlite` red ×3 on the SAME cause — the hub's directory-command rate
  limit (burst 60, 10/s) refused the fixture's 100-space burst with a body-less 429 and the test JSON-parsed the empty body (found
  by making the test print the hub output on failure) → harness honors 429 + Retry-After (landing row) → **2/2 PASS 22:13**.
  `document-growth-e2e sqlite` **2/2 PASS 22:29** (5.5 min). (Both run the SQLite WAL reader path.)
- 22:13 hub 8010 for C12's STEP 15: W4 hold (`wp-w4/w4-hub-hold.ts`) on a fresh p24 clone + binary 2155, state
  `.🧬semio/🌐hub/s14-h13-state-8010/` (admin capability there), ready + admin issued 22:19.
- 22:22 **STEP 8 duplicate-id (C12 relay):** read-only capture of 7800's artifact-6e93e221… (`generated/c12-step8-bootstrap-7800.json`):
  bootstrap "Tail", 17 envelopes (4 edits → Check In transition → 11 two-author edits with `observed` → 2nd check-in transition),
  no id repeated or delivered twice by the hub. Transition payloads name committed ops by their mutation ids WITH the op suffix
  (`edit-…#0`); writer's SeedHistory seeds entry ids AND forward mutation ids → a client fold that records a transition's committed
  ops as forwards seeds `…#0` twice. Relayed to C12 with a reproduction recipe (base → Check In → two-author typing → hard reload).
- 22:2x transport-refill set (L1 T1a GREEN 22:18): os-mcp TS oracle **5/5 PASS** (`vitest-mcp-refill-1.txt`); Rust laws = hold 14
  (queued in the native lane).
- 22:42 **machine rebooted** (every process died: hub 8010 hold, lane waiters incl. hold 14). 22:46 reconcile: all five H13 patch
  scripts dry-run "0 pending", refusal schema + fixture parse, file tails intact (no torn write); my landing rows present.
- 22:50 hub 8010 restarted after the reboot (same root + binary 2155, W4 hold, state `s14-h13-state-8010/`, admin issued 22:50) → C12.
- 22:4x **typed 429 body LANDED** (coordinator item; `wp-h13/h13-rate-limit-refusal-patch.py`): schema-first `RateLimitRefusalV1` +
  `RateLimitRefusalMessageV1` in `🔐️auth/🧬️schema` (schema, code `rate-limited`, the class, `retryAfterMs` ≥ 1, en + de notice), Rust
  twin + `RateLimitRefusalV1::new` in `🔐️auth/🚦️rate-limit`, the rate-limit middleware answers every non-auth family's 429 with it
  (+ `no-store`, `retry-after`; sign-in keeps `AuthErrorV1`); fixture `🔐️auth/🧫️fixtures/🚦️rate-limit-refusal-v1` (4 valid, 6 near
  misses) registered in the taxonomy (`members-of-fixtures`); laws: auth unit `the_rate_limit_refusal_is_the_declared_schema_body`
  (owned draft-07 validator + fixture), bin `a_directory_command_burst_is_refused_with_a_typed_rate_limit_body`, TS Ajv oracle case
  → **8/8 PASS** (`vitest-rate-limit-1.txt`). Rust = hold 15 (queued with hold 14's laws, which died in the reboot).
- 23:1x **STEP 8 root cause (guest):** `🏪️store` `edit_from_operation_envelope` names a remote edit after its op's wire id AND puts the
  same id in `mutation_meta[0]`; the plugin initializers' SeedHistory seeded lane 0 (`entry.id`) and lane 1 (each op id) → the id
  twice → `MutationDagError::Duplicate` (only with foreign edits in the restored history = two-author sessions). Relayed to C12/L1;
  coordinator: C12's `wp-c12/seed/c12-seed-history-patch.py` (one store rule `seed_edit_operation`, 9 initializers + 2 hydrations)
  is the set — I reviewed it (approve; amendment: the law should initialize the MIXED shape — local + remote edits + transitions)
  and wrote no competing patch.
