# WP-H4 — Agent Edit Durability On The Hub Document Ledger

Slice H4 · session 10 · 2026-09-24. Ports: hubs 7850–7859. Captures: `wp-h4/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Reproduce G4 (head stays 1, catch-up shows 1 edit) | DONE: reproduced on :7850 (`h4-repro-3.txt`) |
| 2. Root cause | DONE (§Root cause) |
| 3. Root fix (Store replica identity, runtimes' entropy/clock, ledger collision refusal) | LANDED in source; kernel/db laws green; guests need W1 rebuild |
| 4. Regression laws (Store fixture, db fixture, live hub-edit-durability gate) | Store and db laws GREEN. The live gate is written and registered, and on the old catalog guest it reproduces the defect (head 1→1). A green live run is still pending a catalog carrying the rebuilt note |
| 5. Gates: hub-agent-participant 17/17 ✔, hub test quick 327/327 ✔, kernel sync 61/61 (flaky, shared with R2), live durability/G4 repro BLOCKED on a catalog carrying the rebuilt note (P5 GIS codec receipt) | PARTIAL |

## Root cause

Measured on hub :7850 (a clone of `g4-boot`, H2 00:16 binary). The agent's relayed envelope carries `mutation_id = edit-2bb425fa800a3b7f`
(`h4-agent-edit-human-sees.json`). That is exactly the document's FIRST committed edit, authored hours earlier by
a different agent session (`hub.v1.205b…`, WAL `segment-0` @ 23:56). The WAL file has not been written since, and the
`head_seq` stays 1 across all later runs.

The chain:
1. **Store (every guest, browser and native):** `apply_command` names an edit `mint_edit_id(actor, sequence, forwards)`.
   `actor` is `"local"` (the mutation's author default), `sequence` restarts at 1 in a fresh Store, and `forwards` is the
   gesture's content. That content includes a deterministic block id for `addBlock`. So every fresh process that performs
   the same first gesture mints the same id. Multi-op edits are no better: they name each op by `mint_mutation_id(bytes)`,
   a pure content hash. Clock and randomness cannot rescue this. The id takes no clock input, the owned interpreter
   answers `wall-clock.now` and `insecure-seed` with zeroes, and `wasm32-wasip2` had no entropy arm (`🪪️identity`).
2. **Kernel sync** stamps the socket actor and a fresh HLC on the wire copy but keeps the Store's `mutation_id`.
3. **db ledger (`🗿️artifact` `submit`/`apply_one`)** deduplicates by id. A known id is treated as an idempotent replay
   and answered with an `Ack` `Accepted`. Nothing is written to the WAL, and the frontier and index stay unchanged. So
   status (`head_seq`) and catch-up (WAL replay) are *correct*: the edit never became durable.
4. The live relay G4 saw came from the 00:16 hub, which relayed even non-advancing receipts. H2 has since gated the relay
   on `commit_seq` advancing, so on current source the human would not even see it live.

The same defect hits humans too. Two browser tabs doing the same first gesture collide, and the second tab's edit is
silently lost.

## Fix (landed in source)

| layer | change |
|---|---|
| `🪪️identity` | `fill_entropy` for `wasm32-wasip2` via `wasi:random/random@0.2.9#get-random-u64` (the interface version libstd's `wasip2` embeds, so the component linker resolves it). New `entropy_u64()`. The catch-all arm no longer includes p2. |
| `🏪️store` | Every Store instance gets a replica clock `HybridLogicalTimestamp::new(entropy, now)` (`fresh_replica_clock`) at construction and runtime initialization. A reload keeps its replica (`seed_runtime_state(envelope, replica)`). A Store refuses to construct without entropy instead of colliding. |
| `🌿️vcs` | `mint_edit_id(replica, sequence, fingerprint)`; `mint_mutation_id(bytes, (replica, physical_ms, logical))`. The edit and op ids are namespaced by the replica, and op ids also by the replica's strictly advancing tick. Transition ids already hash the (now replica-bearing) timestamp. |
| `🛢️db` `🗿️artifact` | `submit` refuses a committed `mutation_id` whose diff, inverse or dependencies differ: `DbError::Conflict("mutation id … already committed with different content")`. The hub turns this into a Rejected ack, and kernel sync rolls it back. An identical resend (also under a new socket actor) stays an idempotent replay. |
| owned interpreter (`🔌️plugin/🖥️host`) | Real `wall-clock.now` (datetime record), real `insecure-seed`, new `wasi:random/random@0.2.9 get-random-u64`. Wasmtime already links full WASI p2 (`WasiCtx`). |
| describe host (`🧠️interpreter`) | `get-random-u64` answers 0. Describe stays the deterministic profile, the same way `now-ms` is 0 there. |
| browser WASI profile (`🌐️browser-bundle/🌐️wasi`) | `wasi:random/random@0.2.9` (`getRandomBytes` ≤ 64 KiB, `getRandomU64`) from `crypto.getRandomValues`. The admitted interface lists move 18 → 19 in lockstep: TS profile, `DOCUMENT_BROWSER_ACTOR_INTERFACES` (Rust + TS), directory schema `DocumentBrowserActorInterfaces`, and the hub independent oracle in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`. |

### Catalog bootstrap bind rule (found on the way, root-fixed, agreed with W1)

`trusted-catalog-bootstrap --packages stdio,gis,note` into a fresh root printed
`trusted-catalog-bind: source=.🧬semio/🌐hub/hub-dev/trusted-catalog … wasm=skipped` (`catalog-bootstrap-1-stale-bind.txt`).
`resolveTrustedCatalogBindSource` auto-bound ANY published root under `.🧬semio/🌐hub` whose bundle covered the
selection, and never compared it with the sources. So a rebuilt guest could never reach a freshly bootstrapped
catalog. Fix (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`): no discovery. A bind happens only from `OS_HUB_TRUSTED_CATALOG_SOURCE`,
verified against that root's own bundle and generation hashes. Otherwise every selected package is built fresh.
`trustedCatalogPublishedRoots` is deleted.

## Regression laws

| law | kind | result |
|---|---|---|
| `🏪️store/🧫️fixtures/🪪️replica-edit-identity.json` → `fresh_replicas_authoring_identical_gestures_never_share_an_edit_or_operation_id` | fixture + Rust | PASS (`kernel-replica-law.txt`) |
| `🛢️db/🗿️artifact/🧫️fixtures/🪪️mutation-id-collision/🔣️.json` → `a_committed_mutation_id_replays_idempotently_but_refuses_colliding_content` | fixture + Rust | PASS (`db-artifact-targeted.txt`) |
| `🌿️vcs` mint law (replica/tick distinguish) | Rust | PASS |
| browser WASI activation (random interface, lists, closed refusal) | TS | PASS (`browser-wasi-activation.txt`) |
| `🌉️mcp/🧫️fixtures/🤝️hub-edit-durability/🔣️.json` → `hub-edit-durability-check` (live: 2 fresh MCP agent lanes running the SAME gesture + 1 human socket lane. Per lane: relay to observer, head +1; then distinct ids, late-joiner catch-up, hub restart keeps head and catch-up) | fixture + live gate (Nx target, launch.json `⚖️gate🌉️os-mcp🤝️hub-edit-durability`) | pending rebuilt guests + hub |

## Evidence

| command | result | capture |
|---|---|---|
| G4 repro on :7850 (H2 00:16 binary, g4-boot clone) | 6/7, head 1→1, relayed id = first committed id | `h4-repro-3.txt`, `h4-agent-edit-human-sees.json` |
| `cargo check -p semio-framework-os-kernel --features sync` | EXIT 0 (4 pre-existing warnings) | `check-kernel-1.txt` |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2` (wasm mutex) | Finished | `check-kernel-wasip2.txt` |
| `cargo check -p semio-framework-plugin` | Finished | console |
| kernel `--lib --features sync,ureq` (all) | 1188/1189. The 1 red is `document_codec_of_round_trips_dsl_and_pack_and_edit_text`: its edit-text line-count assertion fails on a hand-built edit, and no minting is involved. Not mine; it is red in isolation too. | `kernel-lib-all.txt` |
| db `artifact::` targeted (new law + dedupe/persist/replay neighbours) | 5/5 | `db-artifact-targeted.txt` |
| db `artifact::` full, parallel | 34/51: 17 red at `storage()` (MemoryStorage test pool, line 705) plus pool-capacity asserts, i.e. the test pool under parallel load; the new law and neighbours are green in isolation. The serial run hangs >20 min (killed, my pid). Not caused by the submit change (the reds sit in storage construction) | `db-artifact-tests.txt` |
| kernel sync `--lib --features sync,ureq -- sync` | 61/61 once (`kernel-sync.txt`); repeated ×4 in parallel: 2/4 red on `detach_drains_pending_outbound_operations` (5 s deadline). R2 reported the same flake independently (lost wake, idle pool); it passes serially | `kernel-sync-repeat.txt` |
| os-hub build (current tree incl. H2/H3/H5 + db collision refusal) | EXIT 0 | `build-os-hub-2.txt`, binary `wp-h4/bin/os-hub-h4` |
| hub `test quick` (`bun nx run os-hub:test-quick`, hub mutex) | **327/327** passed, 10 skipped, EXIT 0 | `build-os-hub-2.txt` |
| os-mcp build (`semio-os-mcp`, private target) | EXIT 0 | `build-os-mcp.txt`, binary `wp-h4/bin/semio-os-mcp` |
| hub-agent-participant on :7850 (new hub + new MCP, old catalog guest) | **17/17** | `hub-agent-participant-1.txt` |
| hub-edit-durability, new hub + MCP, OLD catalog note guest (g4-boot catalog) | 6/11 then 9/17: every agent lane SUCCEEDED but relayed nothing and head stayed 1→1. This is the defect, reproduced by the law. The MCP `--hub` loads the guest from the hub's trusted catalog (`execution-target/component`), so the proof needs a catalog with the rebuilt note | `durability-old-guests.txt`, `durability-2.txt` |
| W1 note rebuild | sha256 `6cd7dcc5…5cb7`; component imports `wasi:random/random@0.2.9` | `wp-w1/requests/h4.txt` |
| fresh 3-package catalog `trusted-catalog-bootstrap --packages stdio,gis,note` into `.🧬semio/🌐hub/h4-cat` (after the bind fix) | run 2: the candidate GIS creation probe stayed `accepted` past 120 s. Load ~30; a manual probe on :7850 and on the OLD 00:16 binary needed 515 s to reach Ready, i.e. a cold wasmtime compile. So this is a budget issue, not a regression. Run 4: the fresh generation `3142b907…` was built, but the candidate hub exits before readiness with `ArtifactAuthority(Catalog("gis/native-codecs/v1: exact private receipt rejected"))`, which is P5's structural `schema_hash` change vs the hub's linked GIS native codec receipts. Reported to P5 | `catalog-bootstrap-2.txt`, `catalog-bootstrap-4.txt`, `creation-probe-*.txt`, `hub-7854-status-capture.txt` |

### Live proof attempts (catalog with the rebuilt note)

| run | outcome |
|---|---|
| bootstrap 1 | bound the stale `hub-dev` catalog (the bind bug above, now fixed) |
| bootstrap 2 | fresh build; the candidate GIS creation probe timed out at 120 s (cold wasmtime compile at load ~30; measured 515 s even on the old binary) |
| bootstrap 3 | killed by me after it held the wasm lock ~40 min while waiting on C8's wedged hub lock |
| bootstrap 4 | generation `3142b907…` built; the candidate exits `gis/native-codecs/v1: exact private receipt rejected`. Rebuilding os-hub on the current tree changes this to `native codec schema hash is zero or mismatched`: the catalog's GIS guest was built before P5's structural `schema_hash` change, so it is stale against the new hub |
| bootstrap 5 | peer compile break mid-edit (`dsl::tagged_value_binary` missing in stdio) |
| bootstrap 6 (×2) | twice the detached job died silently mid-build (after `gis derive 8/8`, no error in its capture). The process was killed externally; I did not kill it |

The G4 repro/durability gate therefore still has to be re-run once a stdio+gis+note catalog is built from the current
tree. The command:
`OS_MCP_HUB_BINARY=<current os-hub> OS_MCP_HUB_DATA_DIR=<root with that catalog + user1 space> SEMIO_OS_MCP_BIN=<current semio-os-mcp> bun nx run @semio-tech/framework-os-mcp-rs:hub-edit-durability-check`
(launch.json `⚖️gate🌉️os-mcp🤝️hub-edit-durability`). The gate creates its own fresh note via the hub creation
transaction and allows 30 min for the cold compile.

## Pids

| pid | what |
|---|---|
| 74942 / 74946 | hub hold / os-hub :7850 (`h4-boot`, `wp-h4/bin/os-hub-h4`), left running for the coordinator |
| (none) | the :7852/:7853/:7854 gate/probe hubs exited or were killed by me; no H4 build or mutex waiter is running |

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🪪️identity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🪪️replica-edit-identity.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🪪️mutation-id-collision/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`, `🧠️interpreter/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts`, `🧪️tests/🌐️wasi-activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`, `🌐️browser-actor/🦀️.rs`, `🌐️browser-actor/🟦️.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (browser actor interface oracle)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤝️hub-edit-durability/🟦️.ts`, `🏃️execution/🟦️.ts`, `🧫️fixtures/🤝️hub-edit-durability/🔣️.json`, `📦️packages/🦀️rust/📜️script.ts`, `📋️project.json`, `🧪️tests/🎚️config/🟦️.ts`
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`
- ticket: `wp-h4/h4-hub-hold.ts`, `wp-h4/h4-agent-edit-human-sees.ts`, `wp-w1/requests/h4.txt`

## Gaps

- **Live proof of the fix is not green yet** (see "Live proof attempts"). The laws prove it at the Store and db layers.
  The live gate reproduces the defect on old guests but has not yet run on rebuilt guests. That needs a
  current-tree stdio+gis+note catalog; W1's catalog pass or a re-run of `trusted-catalog-bootstrap` into
  `.🧬semio/🌐hub/h4-cat` provides it.
- G4's `h4-agent-edit-human-sees.ts` repro was not re-run on rebuilt guests, for the same reason.
- Kernel sync parallel flake (`detach_drains_pending_outbound_operations`, `fixtures_replay_matches_expected_events`):
  not mine. R7 root-caused a lost timer re-arm in `⏳️async/🔔️worker-parking` and owns it.

- Guests must be rebuilt (W1 request `wp-w1/requests/h4.txt`) before any live proof. Until then the running guests still
  mint colliding ids. The hub refuses a collision whose content differs, but an identical-content collision (G4's exact
  case: the same `addBlock` on the same base) is indistinguishable from a resend and stays a replay until the guests
  carry the replica identity.
- Domain entity ids are a separate issue (flagged as a task). For example, note block ids
  (`NoteIdOwner::for_document_child` = doc id + command + block count) still collide when two replicas add a block to the
  SAME base concurrently. Now that the edit ids are distinct, both edits commit, so this has become a domain-level merge
  question and no longer loses a ledger write.
