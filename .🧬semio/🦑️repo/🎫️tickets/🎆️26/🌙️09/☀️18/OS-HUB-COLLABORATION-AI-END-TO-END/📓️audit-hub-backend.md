# Audit — Server Hub Backend (`🌎️hub` + `🧰️framework/🛍️products/🖥️server`)

Read-only audit. All commands run from `/Users/ueli/Documents/semio`. Captures under
`🗑️generated/hub-*.txt` in this ticket folder. Cargo lock was free the whole session (no
"Blocking waiting for file lock" seen).

## 1. Architecture

**One Cargo crate for the hub.** `find 🌎️hub -name Cargo.toml` returns exactly one file:
`🌎️hub/📦️packages/🦀️rust/Cargo.toml`, package `semio-hub` (lib) + `[[bin]] name = "os-hub"` at
`🌎️hub/🏗️bootstrap/🦀️.rs` (8522 lines). Everything under `💡️inference`, `📇️directory`,
`🗿️artifact-authority`, `🚀️local-bootstrap`, `🚀️local-relay`, `🛰️lag-rebootstrap`, `🔐️auth` is source
tree for this one crate (path deps like `../../💡️inference/...` show up as warnings against
`semio-hub`, not separate crates).

Crate doc (`Cargo.toml` description): *"OS hub — the multi-tenant document/directory server: an
axum WS shell over `db::Database` (documents/blobs) and `HubDirectory` (identity/tenancy),
sqlite/postgres/neo4j backends selectable per Cargo feature."* Bootstrap file doc confirms: thin
axum shell speaking `protocol_wire`'s binary frames over WebSocket; command-lane persistence goes
through `db::Database`; presence/preview lanes are ephemeral, hub-owned `tokio::sync::broadcast`
fan-out (`🌎️hub/🏗️bootstrap/🦀️.rs:1-13`).

**Routes** (`🌎️hub/🏗️bootstrap/🦀️.rs:8086-8148`): documents (`/spaces/{id}/documents/...`, WS at
`/spaces/{space_id}/documents/{id}/socket/v1`), directory (`/directory/commands`, `/directory/spaces`,
`/directory/socket/v1`, invite redemption), admin (`/admin/api/*`, `/admin` SPA), inference jobs
(`/spaces/.../inference/gis-map/jobs*`), artifact creation (`/spaces/{id}/artifact-creations*`),
auth (`/auth/sessions/me`), blobs (`/spaces/{id}/blobs/{hash}`), health at `/readyz`
(`🌎️hub/🏗️bootstrap/🦀️.rs:8119`).

**DB wiring for local zero-touch dev** (AGENTS.md: zero-touch, cross-platform, no external
runtime deps) — two independent backend switches, both zero-touch by default:
- `connect_db` (`🌎️hub/🏗️bootstrap/🦀️.rs:8190-8223`): `OS_HUB_STORAGE_BACKEND` — **default `"fs"`**,
  rooted at `{data_dir}/db`, `db::Database::open_at`. `sqlite`/`postgres`/`neo4j` are opt-in
  (postgres needs `OS_HUB_DATABASE_URL`, neo4j needs `OS_HUB_NEO4J_URI`).
- `connect_directory` (`🌎️hub/🏗️bootstrap/🦀️.rs:8251-8288`): `OS_HUB_DIRECTORY_BACKEND` —
  **default `"sqlite"`**, `{data_dir}/directory.db`, `SqliteDirectory::connect` +
  `.seed()`. Doc comment on `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1-3`: *"the zero-touch default for
  local dev and single-user self-hosting; no external database service required"* — uses
  synchronous `rusqlite` behind `Arc<Mutex<Connection>>` deliberately (one native `sqlite3` link
  per workspace).
- `semio-hub`'s Cargo feature `default = ["sqlite", "native-artifact-execution"]`
  (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:15`) compiles the sqlite driver in by default; postgres/neo4j
  features exist and now pull real drivers (`sqlx-postgres`, `neo4rs`) both in `semio-hub`'s
  Cargo.toml and in `db`'s own Cargo.toml (`🧰️framework/…/🛢️db/📦️packages/🦀️rust/Cargo.toml:28-30`:
  `sqlite = ["dep:rusqlite"]`, `postgres = ["dep:sqlx"]`, `neo4j = ["dep:neo4rs"]`) — this reads as
  **fixed** relative to the 2026-08-17 ticket finding "postgres/neo4j directory backends... never
  compiled, db declares them as empty features" (`FINISH-HUB-SPACES-COLLABORATION-END-TO-END/📓️final-summary.md`).
  Not reverified by compiling with those features in this pass (time-boxed; see §3).
- Artifact CAS (`connect_artifact_cas`, `🌎️hub/🏗️bootstrap/🦀️.rs:8225-8249`) has its own backend
  switch mirroring the same four backends, filesystem default.

**Schema-first source**: yes. Every hub subsystem carries a `🧬️schema/` folder with a `🔣️.json`
(JSON Schema) plus `🦀️.rs`/`🟦️.ts` twins generated/checked against it — e.g.
`🌎️hub/💡️inference/🧬️schema/{🔣️.json,🦀️.rs,🟦️.ts}`, `🌎️hub/🔐️auth/🧬️schema/{🔣️.json,🟦️.ts}`,
`🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json`, `🌎️hub/🛰️lag-rebootstrap/🧬️schema/🔣️.json`,
`🌎️hub/🗿️artifact-authority/{🌱️creation,🔏️trusted-catalog}/🧬️schema/{🔣️.json,🦀️.rs}`. The nx
`foundation-source-check` / `socket-grant-command-source-check` targets in
`🌎️hub/📦️packages/🦀️rust/📋️project.json` exist specifically to prove Rust/TS twins agree with the
JSON schema (`HubFoundationSourceScript`, `HubSocketGrantCommandSourceScript` in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`).

**`🧰️framework/🛍️products/🖥️server`** is a *second*, generic "authoritative server" product —
crate `semio-framework-server` — described as *"the fourth framework product... hub and zentrale
compose"* per `🎫️tickets/…/SERVER-FRAMEWORK-PRODUCT/📋️summary.md`. It has five real modules
(`🧬️contract` 2 traits, `🗄️storage` 15 impls across 4 trait families — `AuthorityStore`,
`ProjectionStore`, `BlobStore`, `SessionStore` — `🛡️policy` 6, `🎭️authority` 10, `📡️gateway` 21;
line counts confirm these are substantive, not stub files) intended to let `hub` become "instance
#1" of a generic server host instead of hand-rolling axum directly. **`grep -rln
"semio-framework-server" --include=Cargo.toml .` returns only the root `Cargo.toml` and the
crate's own `Cargo.toml` — `semio-hub` does not depend on it.** The 2026-08-18 ticket's own "Next"
section says Wave 3 ("hub as instance #1") was designed but not started; that is still true today.
This is architecturally a **duplicate/parallel implementation risk**: two server codebases exist
(hub's bespoke axum app; server's generic contract/gateway/authority), and only one is wired to
anything.

## 2. Presence

`grep -rn "presence\|Presence"`:
- **Hub**: `🌎️hub/🏗️bootstrap/🦀️.rs` (session registry + per-document `broadcast` presence roster,
  admission/rejection of presence pages, `PeerPresenceRoot`/`PresenceRosterAdmission` types used at
  the WS boundary), `🌎️hub/📇️directory/🦀️.rs` (`DirectoryPresenceActor`), directory socket-grant
  decision logic (`🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧭️decision/🟦️.ts`), admin-intent
  tests (`presence-lease-check`, `presence-normalization-check`,
  `admin-presence-target-recovery-check` targets exist in `🌎️hub/📦️packages/🦀️rust/📋️project.json`).
- **`🧰️framework/🔨️modules/📡️replication`** (the wire contract hub speaks): presence wire codec
  (`📡️wire/🦀️.rs`, tests `🔬️presence-codec`, `🔬️assemble-presence-interaction`), fixture
  `🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json`.
- **`🧰️framework/🔨️modules/🎭️actor`**: `presence` appears in `🦀️.rs` and `🖼️wire-turn/🟦️.ts` — the
  generic actor-turn layer is presence-aware, but there is no dedicated `presence` submodule (no
  directory named for it) the way hub has its own presence roster.
- **`🖥️platform`** (`🧰️framework/🔨️modules/🖥️platform`): no presence hits at all.
- **`🧰️framework/🛍️products/🖥️server`** (the generic product): no presence hits — the
  "authoritative server" contract/storage/authority/gateway modules carry no presence concept yet;
  presence today lives only in hub + the replication wire layer, not in the product framework
  that's meant to generalize hub.

**What exists (per the 2026-08-16/17 tickets, function names confirm code is real, not stub)**:
session registry + heartbeats via the document WS presence roster, scoped `(space, document,
surface)`, proven by `presence_roster_is_scoped_per_surface` and a real three-client integration
test (two peers see each other, a third on another surface does not); an admin "Connections" page
with live updates over `/directory/ws`; kick/rebuild actions. **What's stubbed/thin**: the wgpu
native shell's presence wiring is compiler-verified/unit-tested but was "never observed running" as
of 2026-08-17; the generic `server` product has no presence abstraction at all yet (see §1).

## 3. Build state

`cargo metadata --no-deps` filtered for hub/server (`🗑️generated/hub-cargo-metadata.txt`):
```
semio-framework-repo-coordinator  🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/Cargo.toml   (unrelated repo tool, name collision on "server", not part of this audit's scope)
semio-framework-server            🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml
semio-hub                         🌎️hub/📦️packages/🦀️rust/Cargo.toml
```

**`cargo check -p semio-hub` (default features `sqlite,native-artifact-execution`) — FAILS.**
2 errors (`🗑️generated/hub-check-semio-hub-default.txt`):
```
🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:64:13: error[E0560]: struct `SpaceArtifactCreationReadyV1` has no field named `document_id`
🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:249:13: error[E0560]: struct `SpaceArtifactCreationReadyV1` has no field named `document_id`
```
Root cause confirmed by reading the struct: `SpaceArtifactCreationReadyV1` is defined at
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:161-165`
with fields `{artifact_id, kind_id, artifact_schema, parent_dialect}` — **no `document_id` field**.
Both hub call sites (`materialize_selected_genesis` and `ArtifactCreationIntentV1::ready`)
construct it with a `document_id: …` field instead of `artifact_id: …`. No uncommitted diff on
either file (`git status --porcelain` empty) — this is a committed schema/caller drift, not
concurrent churn.

**`cargo check -p semio-framework-server` — FAILS.** 28 errors, all `E0277`
(`🗑️generated/hub-check-semio-framework-server.txt`):
```
🧰️framework/🛍️products/🖥️server/🔨️modules/🧬️contract/🦀️.rs:90,102,146,149,150,180,186,206,209,210,211: 
  error[E0277]: the trait bound `FrontierSummary: serde::Serialize`/`Deserialize` is not satisfied
```
Root cause: `FrontierSummary` (`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:544-551`) derives
only `Clone, Debug, PartialEq` — no `Serialize`/`Deserialize`. `server`'s `🧬️contract/🦀️.rs` embeds it
inside `CommandOutcome`, `QueryConsistency::AtFrontier`, and `QueryResult` variants that themselves
derive `Serialize, Deserialize` (lines 89, 149, 186, 206-211), so the derive can't be satisfied
transitively. Since nothing depends on `semio-framework-server` except its own Cargo.toml and the
root workspace list (§1), this break is currently invisible to any other crate's build.

**`bun tsc --noEmit -p 🌎️hub/📦️packages/🟦️typescript/tsconfig.json` — 151 errors, 0 in hub-owned
files** (`🗑️generated/hub-tsc.txt`; `grep "error TS" | grep 🌎️hub` → 0 hits). All 151 errors are in
transitively-included framework/repo library files (`…/🦑️repo/🔨️modules/📚️library/…`,
`…/💻️os/🧪️tests/🧪️backbone-envelope-io/…`) — missing `@types/bun`, a `Taxonomy` property rename
(`testsDirName`/`testFixturesDirName` no longer exist), an untyped `DirectoryClient` used as a
type, a broken relative import to `./🔨️modules/📇️directory/🧬️schema/🟦️.ts`. This reads as **pre-existing
repo-wide TS debt pulled in by the hub tsconfig's broad root**, not a hub-specific defect — flagged
for completeness per "zero errors ≠ working," but not counted toward hub's own P0/P1 list below.

## 4. Tests

Only one Cargo package (`semio-hub`) exists under `🌎️hub`, so "fastest hub unit test crate" is
`semio-hub` itself; nx `test`/`test-quick`/`test-long`/`test-exhaustive` targets in
`🌎️hub/📦️packages/🦀️rust/📋️project.json` all point at it (plus ~90 other nx targets that are
integration/fixture checks, e.g. `presence-lease-check`, `checkpoint-publication-check`,
`gis-inference-ledger-check` — script-level checks in `📜️script.ts`, not `[[test]]` entries).
`🧪️tests` dirs exist per subsystem (`🌎️hub/🧪️tests/*`, `🌎️hub/💡️inference/*/🧪️tests/🔬️unit/🦀️.rs`,
`🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs`, etc.) — all `#[cfg(test)]` modules inside the one crate.

**`cargo test -p semio-hub --lib` — FAILS to compile, 0 tests run**
(`🗑️generated/hub-test-semio-hub-lib.txt`), same 2 `E0560` errors as §3 (test build recompiles the
same lib). This directly blocks all hub unit tests, not just the binary.

For context, the 2026-08-17 ticket recorded `cargo test -p semio-hub --lib`: 11/0 and
`--bin os-hub`: 18/0 passing — so this is a **regression** introduced since that ticket closed, not
a permanent gap.

## 5. Local dev start attempt

Documented start path: `.vscode/launch.json` entry `"🛠️dev🗄️os-hub"` runs `bun nx run os-hub:dev`
with `OS_HUB_PORT=8787`, `OS_HUB_DATA=${workspaceFolder}/.🧬semio/🌐hub/hub-dev/`
(`.vscode/launch.json:3206-3213`). `DevScript.run` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:11934+`)
builds the admin SPA, materializes a trusted stdio+GIS bundle if missing, `cargo build`s the hub
binary, then calls `startLocalHub` which binds the port and polls readiness.

**Attempted**: `nohup bun nx run os-hub:dev > 🗑️generated/hub-serve.txt 2>&1 &`, polled for ~8s until
the process exited on its own (well under the 60s budget). **It never reached `cargo build` or a
`curl` against `/readyz` — it failed earlier**, at the `os-hub:build-dev` nx task
(`🗑️generated/hub-serve.txt`):
```
error: Cannot find module '../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts'
  from '/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts'
```
Root cause found by counting path depth: the target file (`🧭️routing/🟦️.ts`) **does exist** on disk
and its bytes match the import string exactly (verified byte-for-byte — not a Unicode/variant-
selector mismatch). The importing file
`🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts:2` is 4 directories below the repo
root (`🏃️execution → 🧱️socket-grant-command-source → 🧪️tests → 🌎️hub → root`) but its import uses
**5** `../` segments — one too many, walking one level above the repo root
(`/Users/ueli/Documents/`) where `🧰️framework` doesn't exist. A sibling file at the correct deeper
nesting, `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🏃️execution/🟦️.ts:2`, imports the
*same* module with the *correct* 6 `../` for its own (deeper) location — confirming the bug is an
off-by-one in the shallower file, not a moved target.

**Result: hub did not serve.** `/readyz` was never reached; no `curl` was attempted since no
listener ever bound. No process needed killing (it exited on its own by the time of the first
poll).

## 6. Known gaps

**Code-level markers**: `grep -rn "TODO\|FIXME\|todo!(\|unimplemented!(\|\bstub\b" 🌎️hub` returns
exactly one hit, and it's descriptive prose in a doc comment (`🌎️hub/🏗️bootstrap/🦀️.rs:1607`,
"...the actual `/admin` file-serving handler (and its 503-if-missing stub)..."), not a marker of
unfinished work. **The hub codebase is not carrying an inventory of known-incomplete code paths in
comments — its real gaps are structural/regressions, found only by compiling (§3) and booting
(§5).**

**Ticket history** (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/**` folders matching HUB/SERVER/PRESENCE/AUTH/DB/RELAY):

- `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` and its continuation
  `26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END`: largest and most informative. At close:
  hub persistence (event-sourced directory, restart-survival test), scoped presence, home/space
  tables, sharing, admin page, vcs check-in were all **"done and verified"** at the hub/unit level.
  Explicitly **not done**: browser end-to-end only 2/8 steps passing, blocked on a `plugin instance
  busy`/`missing HistorySnapshot frame` retry storm in the React `PluginRuntime` component (owned by
  a different, concurrently-rewriting session at the time — correctly left alone rather than
  fought); wgpu shell collaboration wiring never observed running; postgres/neo4j directory backends
  "written to parity but never compiled" (appears fixed now per §1, not reverified by compiling);
  `bun nx run os-hub:test*` was "unusable" because it hardcodes `--all-features`.
- `26/08/18/SERVER-FRAMEWORK-PRODUCT`: built the generic `server` product (server core "complete",
  73 tests passing at the time) but explicitly left **Wave 3 ("hub as instance #1")** and a db/os
  decoupling decision (B3) undone — matches §1's finding that `semio-hub` still doesn't depend on
  `semio-framework-server` today, and (new, this audit) that crate no longer even compiles (§3).
- `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY` and `26/08/06/HUB-PRODUCT-CRATE-CONSOLIDATION-DRESS-REHEARSAL`:
  large, unrelated infrastructure-migration tickets (plugin/renderer/kernel churn); no hub-backend-
  specific open items surfaced beyond what's already listed.
- `26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`: heavy presence-adjacent
  work (session colors, artifact creation) landed against os-kernel/hub; consistent with §1/§2, no
  new hub-backend gap beyond the schema drift found in §3 (this ticket is a plausible origin of the
  `SpaceArtifactCreationReadyV1` field rename that broke hub's two call sites).

## 7. Prioritized gap list

**P0 — does not build / does not start**
1. `cargo check -p semio-hub` fails (E0560 ×2): update `SpaceArtifactCreationReadyV1 { document_id, … }`
   construction to the real field name at
   `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:64` and
   `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:249` (struct is `{artifact_id, kind_id,
   artifact_schema, parent_dialect}`, defined at
   `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:161`).
   This alone blocks `cargo check`, `cargo build`, and `cargo test` for the whole hub crate.
2. `bun nx run os-hub:dev` fails before reaching `cargo build`: fix the off-by-one `../` in the
   import at `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts:2` (drop one `../`,
   4 levels not 5, matching the sibling import pattern at
   `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🏃️execution/🟦️.ts:2`). Local zero-touch
   dev boot is currently impossible via the documented command.
3. `cargo check -p semio-framework-server` fails (E0277 ×28): add `#[derive(Serialize, Deserialize)]`
   to `FrontierSummary` at `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:544`, or stop
   deriving `Serialize`/`Deserialize` transitively on the `server` contract types that embed it
   (`🧰️framework/🛍️products/🖥️server/🔨️modules/🧬️contract/🦀️.rs` — `CommandOutcome`,
   `QueryConsistency`, `QueryResult`). Lower urgency than #1/#2 only because nothing currently
   depends on this crate, but it means the "generic server" story is currently dead code that
   doesn't even compile.

**P1 — missing capability**
4. `semio-hub` does not depend on `semio-framework-server` (§1); Wave 3 of `SERVER-FRAMEWORK-PRODUCT`
   ("hub as instance #1") was designed but never executed, so the repo carries two parallel,
   unreconciled server implementations (hub's bespoke axum app vs. the generic
   contract/storage/policy/authority/gateway product). Decide whether to finish that migration or
   retire the `server` product; leaving both live and only one wired risks drift like finding #3.
5. `bun nx run os-hub:test*` hardcodes `--all-features` (per 2026-08-17 ticket), meaning it always
   requires postgres/neo4j drivers to link even for a sqlite-only dev loop — worth re-verifying
   whether `cargo check -p semio-hub --features postgres,neo4j` (or `--all-features`) still compiles
   now that `db`'s features pull real drivers (§1); not verified in this pass.
6. Presence has no representation in the generic `server` product (§2) — if Wave 3 proceeds, the
   gateway/authority layer needs a presence concept, since today it lives only in hub's own
   broadcast roster and the replication wire codec.
7. wgpu native-shell collaboration/presence wiring is unit-tested but has never been run end to end
   (per 2026-08-17 ticket); still true as of this audit (no new evidence either way was gathered
   here — flagged as carried-over, not re-verified).

**P2 — polish**
8. `bun tsc --noEmit` on the hub tsconfig surfaces 151 errors, none in hub's own files, all in
   transitively-included framework/repo library code (missing `@types/bun`, a renamed `Taxonomy`
   field, etc.) — not hub's bug to fix, but the hub tsconfig's broad include means hub's own
   type-check signal is currently drowned out by unrelated repo debt; consider narrowing the
   tsconfig's `include` so a genuinely clean hub check is possible.
9. 25+86 compiler warnings on `semio-hub` (`unnecessary qualification`, `unused import`, one
   `deprecated method` — `Atomic::<u64>::fetch_update` → `try_update`,
   `🌎️hub/🏗️bootstrap/🦀️.rs:964` and `💡️inference/🏃️runtime/🦀️.rs:964`-ish) — harmless today but
   worth a cleanup pass once P0s are fixed, per the repo's "warnings are proof of type-check" norm.
