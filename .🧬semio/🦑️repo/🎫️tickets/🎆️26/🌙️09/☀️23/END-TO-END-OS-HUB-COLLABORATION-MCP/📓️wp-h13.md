# WP-H13 — Hub Backend Correctness and Security (successor of H11 + DB1)

Slice: H13 (session 14, 2026-09-27 18:2x). Coordinator = main chat. Rules: `📓️session-14-preamble.md`. Ports: hubs 8010–8019,
serves 6510–6519. Private cargo target: `.tmp-ticket/wp-h13/target` (build-dir `build-fleet-b`, native lane only). Captures:
`wp-h13/generated/` (expendable). Durable data/logs: `.🧬semio/🌐hub/s14-h13-*` (logs `s14-h13-logs/`). Handovers:
[📓️wp-h11.md](📓️wp-h11.md) (Session 13 table + B3 wave), [📓️wp-db1.md](📓️wp-db1.md) (Session 13 log),
[📓️audit-s13-hub.md](📓️audit-s13-hub.md), [📓️acceptance-s13.md](📓️acceptance-s13.md) §2.

## Session 14

| # | Item | Status |
|---|------|--------|
| 1 | H11's P0 agent ceiling + agent roles + check-in cause + interpreter cancellation: semio-hub `--all-features --lib --tests`, os-hub bin, os-mcp green on the current tree; new laws run | H11 compiled + ran them 16:02–16:19 (unreported; captures below). TS oracles re-run today **9/9 PASS**. Re-proof on the current tree QUEUED (native lane, hold 1). **Live finding: the P0 fix broke every read-audience semio-MCP agent binding** → root-fixed in os-mcp 🔗️remote (item 2) |
| 2 | Live P0 proof: `wp-g11/g11-refused-relay-probe.ts` against my own current-tree hub (fresh root) | **Hub side PROVEN LIVE 19:2x: new permanent check `os-hub-ts agent-ceiling-check` 16/16 PASS on the current-tree hub 8010** (read agent's edit Rejected `no grant allows Write`, head 0→0; edit agent accepted 0→1; neither agent lists/reads the foreign space (404), renames/adds a member/mints an invite (403 ×6), reads the admin console (401 ×2); author + admin controls 202/200) — and **5/16 on the pre-P0 B3 binary (= 7800 today)**: read agent edits, agents see the foreign space and administer the space. MCP leg: the P0 fix broke the gateway's read-agent binding → os-mcp 🔗️remote fix (TS oracle 4/4; native check queued); G11 probe re-run after the rebuilt gateway |
| 3 | Hub suite on the current tree (`os-hub:test`, `os-hub:test-all-features`; rows 2.1–2.3) | QUEUED (hold 1: full all-features suite) |
| 4 | C11/C12 routed defect: Check In refused `codec-refused` → root cause + fix | **ROOT CAUSE FOUND + FIX LANDED 19:15** (guest-linked, pre-freeze): guest `print_mirror` dropped a populated envelope's owners → vcs ledger Drop witness panic. **Native kernel + plugin `--lib --tests` EXIT 0 20:04**; 2 laws running (hold 1); wasm32 = chain. Live re-proof needs a rebuilt writer guest (7800 ALL) |
| 5 | pg/neo4j live gates (row 2.4): backend-up → two-client-e2e + document-growth-e2e pg/neo4j → backend-down | TODO |
| 6 | DB1 greeting-storm / storm-ratio (row 2.6) + permanent hostile-input/fuzz harness (row 2.13) | storm: DB1's redesigned laws are already permanent (`@semio-tech/framework-os-kernel:reopen-storm-check`, record `hub-reopen-storm`) → run queued (hold 2). Fuzz: NEW permanent `os-hub hostile-input-check` (Ajv oracle + 3 enumerated laws + generative law per seed, `--seed-range` for longer fuzz, record `hub-hostile-input`); tsc 0; runs queued (hold 2) |
| 7 | C12 P1 (coordinator 20:0x): writers' post-cut batch refused `DB I/O aggregate admission exhausted` → all typing lost | (b) DONE hub-side: transient refusals carry `HubTransientApplyRefusalMessageV1` (`hub.unavailable`, warning) — schema + Rust + fixture + bin law + Ajv 6/6; native check in hold 1. (a) db per-operation credit vs declared batch maxima → routed to H14 via main |

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
