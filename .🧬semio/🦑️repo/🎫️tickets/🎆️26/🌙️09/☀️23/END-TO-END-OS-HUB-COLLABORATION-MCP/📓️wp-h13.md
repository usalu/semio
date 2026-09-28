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
| 3 | Hub suite on the current tree (`os-hub:test`, `os-hub:test-all-features`; rows 2.1–2.3) | **os-hub:test-all-features EXIT 0 14:09** (lib 247 pass / 19 ignored, bin 176 pass; `hold4b-all-features.txt`); default-feature suite = hold 5 |
| 4 | C11/C12 routed defect: Check In refused `codec-refused` → root cause + fix | Fix landed 19:15; kernel laws **2/2 PASS 20:11**; guest law red once on its own non-server-minted genesis id (fixed 20:3x), re-run blocked by peers' plugin lib-test compile errors (6) → window 3. Docstring misplacement → prepared patch `h13-docstring-restore.py` (window 3). Live re-proof = 7800 on ALL (C12 STEP 6) |
| 5 | pg/neo4j live gates (row 2.4): backend-up → two-client-e2e + document-growth-e2e pg/neo4j → backend-down | blocked until ALL: needs a channel-19 catalog (B3 refused by today's binary); all-driver binary ready (`s14-h13-bin/os-hub-all-drivers-1404`) |
| 6 | DB1 greeting-storm / storm-ratio (row 2.6) + permanent hostile-input/fuzz harness (row 2.13) | hostile-input-check (fixture seeds + 100..109) + reopen-storm-check all = hold 5 (queued) |
| 7 | C12 P1 (coordinator 20:0x): writers' post-cut batch refused `DB I/O aggregate admission exhausted` → all typing lost | (b) **DONE**: bin law + unit law PASS (hold 4b); live proof waits for ALL. (a) H14. G12 relay (transport wedge) root-caused → prepared patch `h13-transport-refill-patch.py` (window 3) |

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
