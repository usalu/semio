# Result: hub directory capability traits (slice HUB-DIRECTORY-CAPABILITIES, prefix `hdc`)

Executor: Sonnet, foreground, no sub-agents, no ticket tool, no modifying git command. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools, snapshot and installed-file list: `F/hdc-tools/`. Logs: `F/🗑️generated/hdc-*`. Date 2026-10-16.

## 1. Verdict

The `HubDirectory` port is gone. It is replaced by 15 backend capability traits plus one extension trait, `PrincipalAuthority`. Every section child of the three backends (`🪶️sqlite`, `🐘️postgres`, `🌐️neo4j`) now implements its traits directly, and the 91 / 86 / 86 forwarders in the three backend roots are deleted. The closed-enum dispatch, every hub consumer, the shared conformance corpus, the laws and testkit crates, the manifests and the TS source reader were changed in the same pass. No alias, no deprecated trait and no compatibility shim remain.

**Nothing was compiled** (Wave 6, launcher refused). The evidence is a new stub-layer typeck oracle that type-checks the trait impls against the real trait definitions, seeded-sensitivity runs, a module-aware trait-in-scope proof for every consumer, definition scans, rustfmt hunk counts, path checks and a census. Section 6 lists every command and its real result. Section 9 holds the real-check commands for the compile queue.

## 2. Design

### 2.1 The partition

The partition follows the 12 sections that the three backends share. A section that held two audiences became two traits. A trait's methods are never spread over two section children, so one `impl Trait for Backend` block sits in one file. Region markers (`//#region Users`, `AuthSessions`, …) stay in the backend children and now wrap the trait impl blocks of that region.

| Trait | Section child (port and backends) | Methods | Notes |
| --- | --- | ---: | --- |
| `ShareTokenDirectory` | `🎟️capabilities` | 9 | `issue_share_token` and `revoke_share_token` keep their trait-level default bodies |
| `InviteDirectory` | `🎟️capabilities` | 10 | `issue_invite`, `revoke_invite` default bodies |
| `UserDirectory` | `👤️accounts` | 5 | |
| `AuthSessionDirectory` | `👤️accounts` | 8 | |
| `CredentialDirectory` | `👤️accounts` | 2 | `append_credential_audit`, `set_password_credential` |
| `AgentDelegationDirectory` | `👤️accounts` | 5 | the only optional capability, see 2.3 |
| `SpaceDirectory` | `🏘️spaces` | 9 | |
| `DocumentDirectory` | `🏘️spaces` | 9 | descriptors and published checkpoint reads |
| `ArtifactCreationDirectory` | `🏭️artifact-creation` | 6 | |
| `ArtifactCasDirectory` | `🧺️artifact-cas` | 10 | |
| `AdminOperationDirectory` | `🛡️administration` | 6 | gained `admin_overview_counts` (2.4) |
| `ReceiptDirectory` | `🛡️administration` | 5 | command receipts and checkpoint publication claims |
| `SyncSessionDirectory` | `🔄️sync-sessions` | 6 | `list_active_sync_sessions` default body |
| `EventLog` | `📜️event-log` | 5 | `append_events` default body |
| `ProjectionRebuild` | `📽️projections` | 2 | moved out of the event-log child (2.4) |
| `PrincipalAuthority` (extension) | port `🏘️spaces` | 2 | not implemented by backends |

The 99 methods of the old port are all accounted for: the inventory of `async fn` names in the old port equals the one of the new port children (99 = 97 + the 2 `PrincipalAuthority` methods; `verify_final.py`). Each backend implements exactly the required methods of every trait (sqlite 91, postgres 86, neo4j 86, the same counts as the old forwarders). The dispatch enum implements 91.

### 2.2 Where the traits live

`📇️directory/🔌️port/🦀️.rs` is now only the mount list and the re-exports. Each section has a port child `🔌️port/<emoji>️<kebab>/🦀️.rs` with the same name as the backend section it mirrors, holding that section's traits. The nine new files start with a `//!` docstring whose emoji is unique inside the crate. The crate root exports the traits explicitly: `pub use port::{AdminOperationDirectory, …, UserDirectory}`. The lib closure of `semio-hub-directory` is 12 files, 989 active lines (limit 32 files / 4,000 lines).

### 2.3 Decisions

- **No aggregate supertrait.** The task allowed a thin supertrait for callers that need everything; no caller does. `HubState` holds the concrete `Arc<HubDirectories>`, and the two generic functions (`capped_socket_session_binding`, the two backend-corpus assertions) need a handful of capabilities. An aggregate would only have been the old name back.
- **`PrincipalAuthority` is an extension trait.** `principal_ceiling` and `principal_role` combine two capabilities (`AgentDelegationDirectory::agent_delegation` and `SpaceDirectory::get_role`). They are default methods of `PrincipalAuthority: SpaceDirectory + AgentDelegationDirectory`, with `impl<T: SpaceDirectory + AgentDelegationDirectory> PrincipalAuthority for T {}`. Method call syntax stays; no backend can override it (none did).
- **Dead default bodies removed.** The old port had 22 default bodies that answered `… is unavailable for this backend` (CAS ledger 10, credentials 2, delegations 5, receipts 5). Only the agent delegations are really unsupported (postgres and neo4j). The other 17 became required methods; all three backends implement them. `AgentDelegationDirectory` keeps its five defaults, and postgres and neo4j opt out with an explicit empty impl (`impl AgentDelegationDirectory for PostgresDirectory {}`). The 17 deleted `DirectoryError::Backend` default bodies are the only difference the oracle reports (section 6).
- **Dispatch keeps its match delegation.** `HubDirectories` is a closed enum whose methods choose a variant; that is its job, not the forwarding layer that was removed. It is regrouped into one `impl <Trait> for HubDirectories` per capability, in the same file, and needs all 15 traits.
- **`?Sized` kept** on the two corpus functions; every capability trait has the supertraits `Send + Sync + 'static`, as the old port had.

### 2.4 Two relocations

Both moves are verbatim method bodies, done identically in all three backends (`xform_backend.py` `RELOCATE`).

- `admin_overview_counts` (spaces, users and connections counts for the administrator dashboard) moved from the `👤️accounts` child to `🛡️administration`, so it sits in `AdminOperationDirectory`.
- `rebuild_projections` and `rebuild_projections_controlled` moved from `📜️event-log` to `📽️projections` into `ProjectionRebuild`, next to `project`, which they replay through. The event-log children shrink (sqlite 189 to 83 lines, postgres 296 to 171, neo4j 309 to 211).

## 3. What changed

### 3.1 Backends

For each of the three backends: 9 section children rewritten, the root lost its adapter block and its `use semio_hub_directory::{…, HubDirectory, …}` token became the 15 trait names (ASCII-sorted into the list), module docs and the three `🔌️connection` docs lost the `HubDirectory` name, and the crate `Cargo.toml` description was reworded. Inherent helpers (`*_attempt` of neo4j, `revoke_auth_sessions_matching`, `project`, the cas helpers) stay in separate `impl Backend { }` blocks. One signature that was multi-line only because of the `pub(super) ` prefix (`revoke_auth_sessions_for_user_with_admin_effect`) fits on one line again, as rustfmt wants.

Size effect (active lines, lib closure, `dm2-tools/closure.ts`): sqlite 3,471 to 3,184, postgres 3,771 to 3,500, neo4j 3,273 to 3,002. Bytes: sqlite 229,181 to 208,462, postgres 233,748 to 214,243, neo4j 218,721 to 199,262.

### 3.2 Consumers (all changed in the same pass)

| Consumer | Change |
| --- | --- |
| `📇️directory/🔀️dispatch` | regrouped per trait, imports the 15 traits |
| `📇️directory/🛠️service`, `🧭️decider` | the traits they use, merged into the existing `semio_hub_directory` use line (they had no import at all) |
| `📇️directory/🔤️wire` | `capped_socket_session_binding<D: PrincipalAuthority>` |
| `📇️directory/🧪️tests/🔮️backend-corpus` (shared conformance corpus) | bounds `D: EventLog + ShareTokenDirectory + ?Sized` and `D: DocumentDirectory + EventLog + InviteDirectory + SpaceDirectory + UserDirectory + ?Sized` |
| `📇️directory/🧪️tests/🏛️laws` root and the six backend test files (`🪶️sqlite`/`🐘️postgres`/`🌐️neo4j` `🧪️tests/🔬️unit`, postgres/neo4j `🌱️creation-v1` standalone) | `use semio_hub_directory::{…}` with exactly the traits they call |
| the hub bootstrap, inference, auth, stores, artifact-authority, local-bootstrap and lag-rebootstrap files (`changed-files.txt`) | `HubDirectory` import replaced by exact traits |
| the route-law and testkit unit files under `🧪️tests/🔬️bin-unit` | new imports |
| docs and comments naming `HubDirectory::method` | rewritten to the owning trait (9 places, tool `refs_fix.py`); `HubDirectory::head_seq(self)` and `::events_since(self, …)` in `directory-events/🧱️types` became `EventLog::…` |
| `🌎️hub/🧪️tests/🤖️agent-ceiling/🟦️.ts` | one comment names `PrincipalAuthority::principal_ceiling` |
| hub bin `Cargo.toml` | description text only |

The gateway (`🖥️server/…/📡️gateway`) does not reference the directory port: `git grep` over the whole repo finds `semio_hub_directory` only under `🌎️hub`.

### 3.3 Latent consumer defects fixed on the way

A scan of the module tree shows that about two dozen consumer files called directory methods without any trait import (carve leftovers from wave 4, never compiled): `service`, `decider`, the two `🛡️guard`/`🪞️pair-blobs`/`🌱️artifact-creation` route files, `📌️check-in/🧱️types`, all route-law unit files, the testkit and the backend test files. They now import exactly what they call. Seven test crates had no `semio-hub-directory` dependency at all, so the call was impossible even with the import. Added `semio-hub-directory = { workspace = true }` to `semio-hub-hostile-input-laws`, `-presence-route-laws`, `-directory-route-laws`, `-session-route-laws`, `-boot-laws`, `-open-plan-laws` and `semio-hub-route-testkit`, and the matching seven `dependencies` arrays in `🌎️hub/Cargo.lock` by hand (seven inserted lines, no other line changed). The compile window must still run the offline lock update.

### 3.4 TS readers and the `hss` slice

- `📜️script.ts` was **not edited** (no re-point needed: `hss` had already moved all readers out of it).
- One edit in the `hss` product `🌎️hub/🧪️testing/📖️source-readers/🟦️.ts`: `readDirectoryBackendSource` no longer cuts the root at `impl HubDirectory for …` (there is no adapter any more). It reads the root up to the testkit mount, then the children with `pub(super) ` stripped, then the neo4j rows, exactly as before; the throw `lost its port adapter` is gone, and the doc comment was rewritten. This is a one-hunk re-point, recorded here for `hss`. `readNeo4jDirectorySource` and `readHubDirectorySource` need no change (the latter follows the module tree, so it now also reads the port children).
- Gate-text equivalence: see section 6, row "Gate slices".

### 3.5 Not touched

Peers' unrelated edits were left alone. During the slice a peer renamed `🧑‍🏭️service-v1` to `🧑‍🏭service-v1` under `🗿️artifact-authority/🌱️creation`; my import edit travelled with the folder (verified). A peer also changed `🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` since my snapshot (not mine).

## 4. Found state

No half-applied batch. `RESULT-REPO-HUB-RUST-FILE-SPLITS.md` describes the starting layout; the live tree matched its description (91 / 86 / 86 forwarders, 12 sections). Pre-edit snapshot of all 649 hub `.rs`/`Cargo.toml` files: `hdc-tools/orig-hub-rs.tar` (+ `.SHA256`). The install tool refuses to overwrite a file that changed since the snapshot.

## 5. Census rows

| Crate (lib closure) | Files | Bytes | Active lines |
| --- | ---: | ---: | ---: |
| `semio-hub-directory` | 12 | 66,997 | 989 |
| `semio-hub-directory-sqlite` | 13 | 208,462 | 3,184 |
| `semio-hub-directory-postgres` | 13 | 214,243 | 3,500 |
| `semio-hub-directory-neo4j` | 11 | 199,262 | 3,002 |
| `semio-hub-directory-dispatch` | 1 | 50,024 | 839 |
| `semio-hub-directory-service` | 2 | 67,668 | 1,051 |

All under 32 files / 262,144 bytes / 4,000 lines. Longest new path: 107 bytes from the repo root (limit 240).

## 6. Verification actually run

| Check | Command or method | Result |
| --- | --- | --- |
| Stub-layer typeck oracle, 19 configurations | `hdc-tools/hdc_oracle.py <crate>`: baseline = pre-edit snapshot with the old single trait, current = live tree with the capability traits. Both sandboxes get the `frs-tools/stubgen.py` stub layer **plus the real trait definitions** injected as `crate::__ext::semio_hub_directory::*`, so rustc reaches typeck and checks impl completeness (E0046), signatures (E0053), membership (E0407), trait names in scope (E0405) and bounds (E0599). `directory` (lib, test, testkit, test+testkit), `sqlite` (lib, test), `postgres` (lib, test, testkit, test+testkit), `neo4j` (same four), `dispatch` (lib). Log `hdc-oracle-final.log`. | In all 19 configurations the only difference is **17 baseline errors that are gone**: `E0599 no associated function Backend` from the 17 deleted default bodies. **Zero errors added.** No E0046, E0053, E0407 (on our traits), E0405, E0412, E0425, E0432, E0603, E0616 or E0624 in the current trees. The one E0407 in postgres (`sqlx FromRow::from_row`) is stub noise, identical in the baseline. |
| Seeded sensitivity (mutation applied to the current sandbox only) | `hdc-tools/mutations.py`, log `hdc-oracle-mut.log` | M1 `get_role` impl renamed: E0046 missing item plus E0407. M2 `list_spaces` parameter `i64` to `i32`: E0053. M3 `SpaceDirectory` removed from the backend root import: E0405. M4 dispatch `head_seq` renamed: E0046 plus E0407. M5 foreign method inside `impl DocumentDirectory`: E0407. M6 postgres `rebuild_projections_controlled` renamed: E0046 plus E0407. M7 `PrincipalAuthority` bound replaced in `wire`: E0599 unsatisfied bounds. M8 backend-corpus bound too small (testkit configuration): E0599 `no method append_events`. All 8 caught. |
| Unused imports | `hdc_oracle.py --warn` (`-W unused-imports -W unused-qualifications`, no lint cap), 15 configuration runs | baseline and current counts equal everywhere (directory 7, sqlite 4, postgres 3, neo4j 1, dispatch 0), **0 new, 0 gone** (`hdc-oracle-warn.log`). |
| Trait-in-scope for all consumers | `hdc-tools/scope_check.py` (module tree from `#[path]` mounts, `use super::*` inheritance, receiver filter that excludes `DirectoryService` and `HubAuthorityStore` inherent methods of the same name) | 43 files call directory capability methods; **0 missing imports**, **0 unused imported traits**, 0 remaining `HubDirectory` tokens (`hdc-final-scope.log`). Files that inherit through `use super::*` get their traits from the parent import, which is the union of its glob children. |
| Definition scan, no forwarders | `hdc-tools/verify_final.py` over the live tree | no `impl HubDirectory`; every trait implemented once per backend, in the expected section file; per-trait method sets equal the required sets (sqlite 91, postgres 86, neo4j 86, dispatch 91); no trait-impl method whose body is `self.<same name>(…).await`; no `pub(super)` inherent method with a trait method name; port inventory old 99 = new 99; no missing `#[path]` target; no `[DEBUG]` (`hdc-final-defscan.log`). The same single-line forwarder scan run on the snapshot finds 257 forwarders (89 / 84 / 84; the other 6 of the 263 have multi-line signatures). |
| Losslessness | `hdc-tools/verify_stage.py`, multiset of stripped non-blank lines (`pub(super) ` undone), old children plus old root without the adapter against the staged new children and root | per backend, the only differences are the `impl Backend {` wrapper lines, 15 `impl Trait for Backend {` lines with their closing braces, the merged `use semio_hub_directory` line, doc fixes (`HubDirectory::x` to the owning trait, neo4j: eight doc lines) and the opt-out line plus its doc. No code line was lost or added. The later single-line join of one signature per backend is the only edit after that proof. |
| Gate slices | `hdc-tools/gate_slices.ts` (old reader text rebuilt from the snapshot, new text from the edited reader) | all share, invite, auth, share-schema and event-page slices cut by the TS gates read identically for the three backends; the event-page order predicate is unchanged; whole-text normalized line multiset differs only by the docs and wrapper lines named above. The two slices that differ (`rebuild_projections_controlled`, neo4j `append_document_genesis`) are tail differences of a slice that runs to the next `async fn`; no gate reads them (`git grep` finds no gate naming the moved methods). The gates themselves were **not run** (`share-issuance-atomicity-check` etc. need the launcher and the foreign `startLocalSessionBroker` fix). |
| rustfmt | `hdc-tools/fmtcheck.py`: `rustfmt +nightly-2026-07-20 --check` with the repo `rustfmt.toml` and `skip_children`, old snapshot against live per changed file | 92 changed files: hunks 225 old, 227 new in total, **no file got worse**; the nine new port children have 0 hunks, the port root was formatted with rustfmt itself; the extra hunks that the first install produced (joined signatures, the port root's import block, two leading blank lines) were fixed (`hdc-final-fmt.log`). |
| Paths and docstrings | `verify_final.py`, `verify_layout`-style check | all `#[path]` targets of the family exist, longest path 107 bytes; each port child starts with a `//!` line whose emoji (`👤️ 🏘️ 🎟️ 🏭️ 🧺️ 🛡️ 🔄️ 📜️ 📽️`) is unique among the crate's files. |
| Descriptor-digest fixture | token count of `descriptor_digest` per changed file, before and after | unchanged in all 92 files, so the `rustReceivers` list in the descriptor-digest fixture stays valid. |
| Reader parse | `bun gate_slices.ts` imports and runs `readDirectoryBackendSource` | runs; output as above. |

## 7. Not verified, and why

- **No cargo or full rustc build.** The oracle checks the capability partition, impls, signatures and the in-crate bounds against the real trait text, but external types are stubs. Method calls on stub-typed receivers (almost every consumer: `state.directory…`) are invisible, so consumer trait-in-scope rests on `scope_check.py`, not on rustc. Orphan/coherence of the blanket impl `impl<T: SpaceDirectory + AgentDelegationDirectory> PrincipalAuthority for T` (no other impl of `PrincipalAuthority` exists), auto-trait (Send) leakage through the `async fn` in traits at the axum call sites, and macro hygiene of `admin_effect_try!` in trait impl bodies are the review points for the first real check.
- **The `os-hub` gates and all Rust laws** were not run (launcher refused; `startLocalSessionBroker` import defect is foreign).
- **Generic-bound audit of the hub crates**: I found no other generic use of the old trait (`D: HubDirectory`) than the three listed; a real check will confirm.

## 8. Hand-offs for the compile window

1. Offline lock update for the hub workspace first (`update --workspace --offline --manifest-path 🌎️hub/Cargo.toml`): seven new dependency edges on `semio-hub-directory` were added to `Cargo.lock` by hand.
2. Expected review points: `unused_qualifications` warnings (sqlite/postgres bodies use a few fully qualified `semio_hub_directory_contract::…` names that were already there), a possible `async_fn_in_trait` warning per public trait (the old port had it as well), and `private_interfaces` on `pub(super)` helper signatures.
3. `🔒️dependencies.json` lists external dependency users only; no entry is affected.

## 9. Real-check commands for the compile queue

Run one launcher call at a time under the fleet compile mutex. Prefix: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`

1. Lock: `update --workspace --offline --manifest-path 🌎️hub/Cargo.toml`.
2. Port, backends, dispatch, service: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory -p semio-hub-directory-contract -p semio-hub-directory-sqlite -p semio-hub-directory-postgres -p semio-hub-directory-neo4j -p semio-hub-directory-dispatch -p semio-hub-directory-service --lib --keep-going --message-format=short`, plus the feature lane `--features semio-hub-directory-postgres/testkit,semio-hub-directory-neo4j/testkit`.
3. Consumers (lib): `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub -p semio-hub-document-routes -p semio-hub-session-routes -p semio-hub-route-models -p semio-hub-ledgers -p semio-hub-inference-runtime -p semio-hub-inference-catalog -p semio-hub-stores -p semio-hub-artifact-creation-service -p semio-hub-local-bootstrap -p semio-hub-lag-rebootstrap --lib --keep-going --message-format=short`.
4. Tests: `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory -p semio-hub-directory-laws -p semio-hub-directory-backend-laws -p semio-hub-route-testkit -p semio-hub-directory-route-laws -p semio-hub-presence-route-laws -p semio-hub-session-route-laws -p semio-hub-boot-laws -p semio-hub-open-plan-laws -p semio-hub-hostile-input-laws --tests --keep-going --message-format=short`.
5. After the foreign `startLocalSessionBroker` defect is fixed, from `🌎️hub/📦️packages/🦀️rust`: `bun ./📜️script.ts share-issuance-atomicity-check`, `retained-short-admin-check`, `directory-event-page-v1-check`, `directory-command-receipt-check`, `invite-redemption-transaction-check`, `space-administration-check`.

## 10. Files

Created (nine, under `🌎️hub/📇️directory/🔌️port/`): `🎟️capabilities`, `👤️accounts`, `🏘️spaces`, `🏭️artifact-creation`, `🧺️artifact-cas`, `🛡️administration`, `🔄️sync-sessions`, `📜️event-log`, `📽️projections`, each `🦀️.rs`.

Rewritten: `🔌️port/🦀️.rs`; per backend the root and the nine section children (`🎟️capabilities`, `👤️accounts`, `🏘️spaces`, `🏭️artifact-creation`, `🧺️artifact-cas`, `🛡️administration`, `🔄️sync-sessions`, `📜️event-log`, `📽️projections`); `🔀️dispatch/🦀️.rs`.

Edited: `📇️directory/🦀️.rs`, `🔤️wire`, `📜️contract`, `🕰️events`, `🧾️model`, `🛠️service`, `🧭️decider`, `🧪️tests/🔮️backend-corpus`, `🧪️tests/🏛️laws`; the three `🔌️connection` children; the six backend test files; the bootstrap, inference, auth, stores, authority, local-bootstrap and lag-rebootstrap files and the `🧪️tests/🔬️bin-unit/**` unit files; the four backend `Cargo.toml`s (descriptions) and seven laws/testkit `Cargo.toml`s (dependency); `🌎️hub/Cargo.lock` (seven lines); `🌎️hub/🧪️testing/📖️source-readers/🟦️.ts`; `🌎️hub/🧪️tests/🤖️agent-ceiling/🟦️.ts` (comment). The full list with new/modified flags: `hdc-tools/changed-files.txt` (it also shows two peer-side entries, the renamed `service-v1` folder and the trusted-catalog unit file, which are not mine).

Tools (`F/hdc-tools/`): `hdclib.py` (partition table), `portparse.py`, `gen_port.py`, `xform_backend.py`, `gen_dispatch.py`, `install.py` (hash-guarded), `verify_stage.py`, `verify_final.py`, `scope.py`, `callers_plan.py`, `callers_apply.py`, `crates.py`, `refs_fix.py`, `hdc_oracle.py`, `mutations.py`, `fmtcheck.py`, `gate_slices.ts`, `orig-hub-rs.tar` (+ SHA256), `installed-files.json`, `changed-files.txt`. Rollback: extract the tar over the tree and remove the nine port children.
