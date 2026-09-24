# WP-R8 — Non-Plugin Tree Health: TS Suites, Workspace Check, Framework/OS Lib Tests, Root Test Quick

Slice: R8 (session 11). Captures: `.tmp-ticket/wp-r8/generated/`. Private cargo: `.tmp-ticket/wp-r8/target`. Ports 8070–8079 / 6570–6579.
Inherits: R1–R7, O1/O1b/O1c, G18 compile health, build-infra audit. Coordinator add-ons: audit-s11-build-health §5 P1-4, P1-6, P2-7.

## Gate table

| # | Gate | State | Evidence |
|---|------|-------|----------|
| 1a | framework-os vitest (`bun ./📜️script.ts test`) | PASS 372/372 (7 files), 9 s | `framework-os-test-1.txt` |
| 1b | plugin-registry vitest | 59/61 + 1 fail → **PASS 60/60 + 1 level-gated long** after fixture fix | `plugin-registry-test-1.txt` → `-2.txt` |
| 1c | os-host-rs vitest (host-rs `bun 📜️script.ts test`, same OS config) | PASS 372/372 | `host-rs-vitest-1.txt` |
| 1d | os-dev `test quick` | PASS 163 + 28 level-gated long | `os-dev-test-quick-1.txt` |
| 1e | renderer-react typecheck | PASS, 0 errors over 2554 files | `renderer-react-typecheck-1.txt` |
| 1f | renderer-react tests: fundamental / quick | PASS 1 (+4 name-filtered) / 5 of 5 | `renderer-react-test-1.txt`, `renderer-react-test-quick-1.txt` |
| 1g | renderer-react tests: long (the real corpus) | run 1: 10 fail / 1976 pass, 3 files not loading → run 2 hung (flow pump starved timers under Bun) → run 3: 3 fail (two peer in-flight edits) → **run 4: PASS 2036/2036, 104/104 files, EXIT 0, 231 s** | `renderer-react-test-long-1..4.txt`, `rr-*.txt` |
| 1h | flow host JS (`🌊️flow/🫀️core` `test-browser`, `test-source`, `test-browser-clock`) | test-browser PASS (incl. new pump law + the pre-existing `pending-close-yields-to-user-events`, red under Bun before), test-source PASS (was red since 09-20), clock PASS. `test-browser-ownership` red until the flow `wasm` step republishes the gitignored bindings copy (request filed `wp-w1/requests/r8.txt`) | `flow-test-*.txt` |
| 2 | workspace `cargo check --workspace --all-targets --keep-going --message-format=short` | run 1 (01:22, EXIT 101): 3 crates red. Run 2 (02:02, EXIT 101): 5 crates red. **Run 3 (06:45–06:57, EXIT 101): 2 crates red** — `semio-framework-os` lib test (my host-unit edit vs. the peer's `vcs.edits` → `ArtifactHistoryLedger`; fixed right after, `os-host-check-2.txt` EXIT 0) and `semio-framework-repo-cli` bin `repo` (**NEW**, see classification). Every plugin, hub, MCP and renderer crate compiled. **Run 4 (09-25 10:55–11:02): EXIT 0** — `Finished`, 0 `could not compile`, after the repo-cli restoration (round 2 §1) | `generated/ws-check-{1,2,3}-errors.txt`, `ws-check-4.txt` |
| 3 | framework/os crate lib tests (nextest `--profile quick`, private target) | kernel `--features sync,ureq` **1207/1207**; db `--all-features` **770/770** (+2 `#[ignore]` Docker lanes); replication+trace+pack+async+geometry+actor+hash 741/743 → actor fixed → **actor 126/126** (others green in that run); framework+os-run+plugin-host+os host 549/555: the 6 left are all **needs W2 rebuild** (staged guests predate H9's `codec.replay-envelopes` ABI); `semio-framework-os-flow` 37 → **1 red** (§Flow, 260/261 after the 2 held fixes landed 12:00; the 1 needs W2's flow bindings publish) | `kernel-nextest-1.txt`, `db-nextest-1.txt`, `batch1-nextest-1.txt`, `actor-nextest-2.txt`, `batch2-nextest-1.txt`, `batch2b-nextest-1.txt`, `flow-nextest-5.txt` |
| 3w | renderer-wgpu:test (audit P1-4) | **cargo 1386/1388 (7 skipped), vitest 399/399 (38 files)**. The audit's "~89 failures" is stale (R1: 1372/1372 on 09-24). The 2 reds are Shell source laws that pin `#[cfg(not(wasm32))]`/`#[cfg(wasm32)] … Detached` gating of the browser sync backbone and footer pill — exactly what WG7's browser-actor hub `connect` changes (Shell wgpu target, in flight) → **handed to WG7**: `💓️chrome-maintenance-pressure` `no_chrome_maintenance_lane_arms_itself_without_pressure`, `🌓️appearance-tour-and-footer-pills` `the_footer_pills_are_not_gated_off_the_browser_build` | `wgpu-nextest-1.txt`, `wgpu-vitest-1.txt` |
| 4 | root `test quick` (`bun nx run workspace:test-quick`, 869 tasks) green + exit code propagates cargo failures | **Round 2 (13:16): repo-lib `test quick` 611 pass / 83 levelled / 2 fail (JCO staging → W2) in 155 s** (§repo-lib). Unbailed root census deferred until W2's full publish (869 tasks would starve its builds). **Exit code: PROVEN.** A failing cargo run propagates: `nx run semio-framework-os-flow-core:test-quick` → nextest FAIL → script `process.exit` → nx **EXIT 1** (`flow-nx-test-quick-1.txt`); the root target fails on any failed dependency: `--parallel=1 --nxBail` run stopped on `@semio-tech/repo-lib:test-quick` with **EXIT 130** (bail interrupt) (`root-test-quick-1.txt`). **Green: NOT MET** — first red in fan-out order is `@semio-tech/repo-lib` (~30 red bun laws: repo tooling — JCO/flow compiler boundaries, package handoff, commit/micro-commit, command budgets, Nx transport), repo-tooling owner = **NEW**. A full unbailed census is not run: the fan-out includes `os-hub:test-quick` (hub mutex, H9) and `semio-tech-play:test-quick` (peer), which a single sweep must not run unguarded. Gate gap fixed: the flow crate's 261 laws were in no `test-quick` (project had only `test`) → `test-quick`/`test-long`/`test-exhaustive` added | `generated/root-test-quick-1.txt`, `flow-nx-test-quick-1.txt` |
| P1-6 | launch-registration resolver probe | **PASS**: launch.json 437 configs, 197 `nx run` pairs, 0 unresolved, 0 duplicates, 3/3 compounds resolve; committed launch.json byte-identical to a fresh render; seed 281 rows + 89 `@generated:` placeholders, 0 unresolved/duplicates | `launch-resolver-3.txt`, probe `wp-r8/r8-launch-resolver.ts` |
| P2-7 | K2 ownership-gate census (presentation-react, print, actor/cold-pair) | **all three collect and run green**: `@semio-tech/presentation-react` 147/147 (3 files; its default `test` ran at the 15 s fundamental budget and was killed → floored at `quick` like the O1b suites); `@semio-tech/print` `test` EXIT 0 (starts again; the command-boundary law counted Bun built-in `bun:sqlite` as an external package → `bun:` treated like `node:`, fixture updated); `🎭️actor/📥️cold-pair` in `@semio-tech/framework` 188/188 (4 files, suite listed by name) | `presentation-react-test-2.txt`, `print-test-2.txt`, `framework-vitest-1.txt` |

## Error classification (task 2)

Every error of both workspace runs, by owner. "Fixed" rows were re-checked with `cargo check -p <crate> --all-targets` (captures named).

| Crate (target) | Errors | Root cause | Owner | State |
|---|---|---|---|---|
| `semio-framework-plugin-host` (lib, lib test) | 1 (E0432 `os_store::ComponentDocumentCodec`) | transient half-edit: `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` added 01:27, the store trait landed 01:34 while run 1 compiled in between | peer (TC3b/WG7 lineage) | **resolved by its owner**; `plugin-host-check-1.txt` EXIT 0 (02:0x). Its red lib masked all dependents in run 1 → run 2 |
| `semio-framework-geometry` (lib test) | 49 (E0277 `.await` on non-futures) | `🎲️random` API is sync (async-convention fix of the callee), its unit tests still `.await`ed every call | **R8** | **fixed**: 30 tests → `#[test] fn`, unused `semio-framework-async-macros` dev-dep dropped (Cargo.lock −1 line); `geometry-check-1.txt` EXIT 0 |
| `semio-framework-os-run` (bin, bin test) | 2 (E0277 `Media: Serialize/Deserialize`) | `Media` moved to first-party `ToValue`/`FromValue` (runtime-dependency elimination); the run binary's disk media cache still used `serde_json` | **R8** | **fixed**: cache reads/writes `protocol::os_pack::json::{from_json_str,to_json_string}`; `os-host-run-check-1.txt` EXIT 0 |
| `semio-framework-os` (host-rs, lib test) | 17 | `AppDefinition` gained `actions` (6 initializers); `BackboneDocument` dropped its `cursor` for the `transitions` event log (11 reads) | **R8** | **fixed**: `actions: vec![]`; laws compare the complete folded history (`store::fold_event_log` → `HistoryFold`: applied/redo/checkpoint/alternative) across binary, text and store round trips instead of the removed field; `os-host-run-check-1.txt` EXIT 0 |
| `semio-framework-repo-cli` (bin `repo`, bin test) | 1 (E0433 `mcp_verb`) | the Rust repo MCP production `Repository` (`mcp_verb::real_repository`) was removed from the cli crate before 2026-09-14; `📦️mcp-main.rs` still references it; the `🔌️mcp-verb-handshake` Rust adapter also imports a missing `repo_cli`. Red for ≥ 11 days. The live repo MCP is the Go server (`dev mcp stdio client`) | **NEW** (repo tooling, outside the four outcomes) | decision needed: restore the Rust `Repository` twin (multi-implementation) or retire bin `repo` + the Rust handshake adapter |
| `semio-hub` (lib, lib test) | 1 + 9 (`TrustedCatalogAsset`, `browser_actor_bytes`, `Arc<TrustedCatalogGenerationRoot>`) | in-flight hub edit (`🔏️trusted-catalog/🦀️.rs` 02:13, its unit test 02:18) | H9 / W2 | **resolved by its owners**: green in run 3 |
| `semio-framework-os-dsl-fixture-sweep` (test `fixture_sweep`) | 10 (E0432/E0433 `crate::os_dsl`, `crate::os_store`) | the package's `[[test]]` root pointed at the kernel-only module the kernel itself mounts (`os_dsl::fixture_sweep`); the fleet example laws the package exists for (`🧪️tests/🦀️.rs`, git `9869c6e99b`) were overwritten by the 09-08 test-layout codemod (`de617a7c17`). Red since 2026-09-08 | **R8** | **fixed (compile)**: fleet laws restored as the package's own root `🧪️tests/🔬️fleet-example-sweep/🦀️.rs` (renamed types `FlowHostSnapshot`/`SequenceSnapshot`/`WorkflowSnapshot`, + workflow artifact dep), `[[test]]` repointed; `fixture-sweep-check-2.txt` EXIT 0. Run 1: DSL sweep hit the 300 s quick timeout — its repo walk followed symlinks (`Path::is_dir`); now `DirEntry::file_type` (no follow): 61 s. Run 2: 1 hard failure — `💻️os/📚️examples/♻️reuse` (legacy layout, pre-rename `document-ref`), an unreferenced stale duplicate of `🪐️space/📚️examples/♻️reuse` → deleted. **Run 3: 2/2 PASS** (`fixture-sweep-nextest-3.txt`). Residual, reported not failed by design: 88 unmapped fixtures (11 empty `.semio`, placeholder/`hello` bodies, `procedural.generation3d`/`wfc.*`/`stdio.*` envelopes without a registered `ArtifactDsl` in the sweep registry) → plugin owners (T12/S15) |
| `semio-hub` dependents (bin `os-hub`) | — | masked by the hub lib in run 2 | H9 | green in run 3 |

Not errors but relevant: run 2 emitted warnings only for plugin/artifact crates beyond the above; no T12/G10/WG7 crate failed in either run.

## Fixes

- **plugin-registry `WASI codegen profile policy`**: `Cargo.toml` gained two documented `[profile.wasm-dev.package]` overrides (G5,
  `semio-s-plugin-wfc-engine`, `semio-s-artifact-wfc-bitmap` at `opt-level = 2`); the language-neutral fixture
  `🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` `developmentPackageOverrides` now lists them (the fixture is the contract both
  the registry and the os-dev staging suites read). Also removed 4 `[DEBUG]` console leftovers from registry tests.
- **renderer-react**, each against a deliberate HEAD change:
  - 3 suites could not load: they read `🧑‍🎨engine/🧪️fixtures/…`; the fixture root is `🧫️fixtures` (taxonomy `testFixturesDirName`).
    `🌐️settings-locale-panel-refresh` 2/2, `🖋️ink-canvas-domain-interaction` 5/5, `🕸️node-graph-domain-interaction` 5/5 now.
  - `PluginOperationCompletion.revision` is `bigint` (declared type); the completion law expected `3`, now `3n`.
  - `🛑️scene-pointer-cancellation` producer-boundary law anchored on `className="absolute inset-0 z-30"`; the NodeGraph pointer
    surface became `… z-30 touch-none` (pinch gestures, commit fe0033d12a) → anchor updated.
  - `🚪️opening` scope law: `resolveDocumentOpeningBindings` returns classified bindings (`dataClass`, C6). Schema-first:
    `DocumentOpeningTargetV1` (renderer `🧬️schema/🔣️.json`) arms now require `dataClass` const `persistedShared` / `persistedLocalOnly`
    (matching store `ClassifiedPersistenceBinding`), TS mirror updated, fixture `📍️scope/🔣️.json` updated; `[DEBUG]` log removed.
  - `FlowGraphCanvasHost` gained a required `keyboardPort` ref (fe0033d12a); 8 engine-contract renders omitted it → pass `{ current: null }`.

### Flow crate lib laws (`semio-framework-os-flow`, 249 laws + os host 18)

**37 → 6 red** (`batch2-nextest-1.txt` → `flow-nextest-2.txt` → `flow-nextest-5.txt`: 261/267 incl. os host 18/18).

Fixed (test side; every flow owner — `FlowHostSnapshot`/`Widget`/`Tree` hold `OrderedMap`/`OrderedSet`/`Dictionary` roots that
refuse a bare drop — must be retired, and several laws predate product changes):
- 22 laws dropped live `FlowRetainedVcs` sessions / `FlowDomainAdapter`s / hosts. New helpers `close_to_terminal`
  (cancel or acknowledge + close each open operation, then `begin_close` + `close_retired_step` to terminal-empty),
  `retire_widget_source`, `retire_snapshot_source` (flow-vcs) and `close_domain` (domain laws, bounded by the session-close
  fixture's `maximumTurns`), placed block-accurately by the one-off codemod `wp-r8/flow-close-laws.py`; hosts in the two
  extrude laws retire at the end.
- `Dictionary` retirement: extension evaluate law retires `input`/`out`; the cluster round-trip law uses the store's
  existing `assert_dsl_round_trip_cold`/`assert_dsl_pack_equivalence_cold` twins.
- `command_envelope_round_trip…`: store now installs `member_store_owners()` (the owner catalog every mutating command
  needs) and retires via `retire_flow_store_cold`.
- `connect_ports_replaces…`: port typing now refuses text→number; the law replaces the incoming wire with a second
  number slider (same intent, type-compatible).
- registry projection: `ChannelSpec` gained `valueTypes` → fixture input carries `"valueTypes": []`.
- drawing registry is content-addressed (a `HashSet` of handles since 09-08): deriving the same node twice yields one
  handle and one entry → law expects 1.
- draw-list laws read the frame's `hostDocument` key (renamed from `hostSnapshot` in `render_frame`).
- synchronized-document law gives the document a full layout (the host lays out missing positions on resync, which the
  law's intent — "the synced document is exactly what is retained" — must not depend on) and retires both snapshots.
- whole-action source law: `mem::replace` is now allowed only as the O(1) publication of the persistent layout root the
  bounded `LayoutUpdate` cursor built (`…layout, layout)`); every other replace still fails the law.
- oracle-extraction source law read `../../🦀️.rs` (production, since the 09-08 test-layout move) instead of its own laws
  file; it now reads its own file up to its own definition (no self-satisfaction).
- mushroom law: solid handles are now the kernel's Blake3 content digest (no `solid-` prefix) → checks the digest shape.
- Gate gap: the crate's laws were in no `test-quick` → targets added to `semio-framework-os-flow-core`.

Round 2 as flow owner (`flow-nextest-9.txt`: **264/267**, os host 18/18):
- **Synapse to a missing port** — decided by the neighbour law written with the same change (`synapse_to_a_missing_port_does_not_create_an_engine_edge`:
  "an unresolved port stays in the document until it can be joined"): the dag host snapshot keeps the synapse unjoined
  (`result@missing`) so `sync_from_dag` never deletes document content (neuron ports resolve only once kind infos load — dropping
  unresolved edges erased live wires, measured: 30 host laws went red), and the engine gets no edge. The stale half of the law
  now asserts exactly that. No product change.
- **Kernel scoping of the evaluated solid** — root cause: a packaged flow extension links its own copy of
  `semio-framework-os-flow`, so the solid lives in that copy's `KERNEL`; `crate::tessellate_geometry` read the law crate's copy
  ("not live in this kernel"), exactly as a guest-hosted extension owns its geometry in production. The law now asks the
  minting extension's own `brep.io.exportStl` through the registry the evaluation used and checks a non-empty binary STL
  (triangle count vs byte length). No product change.
- **Lifecycle fixture cursor phases** — (a) layout replacement is now one O(1) root swap (`ReplaceLayout` → `TransferHistory`), so
  the per-relation boundary `afterLayoutRelation` no longer exists → replaced by `afterLayoutRoot`
  (`TransferHistory`, `ReplaceDocument`, mutated, `candidateLayout` 2); (b) a rollback's first close turns now retire the
  `LayoutUpdate` cursor (8 turns for this fixture) → rollback boundaries 1/2/3/6/7 → 9/10/11/14/15, measured per boundary for
  cancel and fault; (c) **product fix** `🌿️vcs/🦀️.rs` `close_operation_step`: those retirement turns overwrote the operation's
  control stage with `Closing` mid-rollback, while every rollback turn restores it; the stage is now set once per turn
  (`rolling_back ? stage : Closing`) and the five per-branch restores are gone. Fixture re-signed with its own FNV-1a digest
  (`wp-r8/flow-lifecycle-resign.py`, verified against the unchanged entries first). Both laws gained boundary diagnostics.

**Held fixes landed 09-25 ~12:00** (coordinator: catalog B live on 7800 at 11:50), compile-atomic: `impl ArtifactDsl for FlowHostSnapshot`
now answers `envelope_id()` from its DSL twin (`flow.flow`; its pack/DSL bytes already went through `FlowHostSnapshotDsl`, so encoded
headers are unchanged — only the trait's identity answer moved off the `EXTENSION` default `flow`); `FlowRetainedVcs::close_phase`
replaced `document.as_ref().filter(|_| self.closing)` with an early `!self.closing → Complete` guard (behaviour-neutral, satisfies the
scan-free token law). `cargo check -p semio-framework-os-flow -p semio-framework-artifact-flow-flow --all-targets` EXIT 0 right after
the edit (`flow-held-check-1.txt`, warnings present = type-checked); nextest **260/261** (`flow-nextest-10.txt`). The 1 left is the
bindings copy → W2's flow `wasm` publish (request appended to `wp-w1/requests/r8.txt`, with the flow guests for the full catalog).

### Framework/os lib-test fixes (task 3)

- `semio-framework-actor` (2 SIGABRT): `JobPayloadProjection::step` closed each payload page with a grant of the page's
  *logical* length. Since the job crate prices a page as its whole physical 16 KiB block (grants accrue until the block is
  paid, `charge_payload_page`), a short page answered `Pending{0, n}`, the assertion failed and the unwinding drop of the
  still-owned `RetainedJobPayload` aborted. The projection now pays one whole page per turn (`JOB_PAYLOAD_PAGE_BYTES`).
- `semio-framework` action-bus wire-dispatch law: same pricing change (`released_bytes == JOB_PAYLOAD_PAGE_BYTES`).
- `semio-framework` tutorial event law: `DslValue` objects from `json!` are key-ordered; the expected inverse literal is
  now written in canonical key order.
- `semio-framework` kernel arena law asserted `!needs_drop::<FixedCommandPage>()`, but the page now owns its 4 KiB block
  behind a `Box` on purpose (no contiguous spine allocation, `📡️spr/🧵️channel`). The law now pins what the design
  promises: a pointer-sized spine slot, one released block per close step (renamed accordingly).
- `semio-framework-os-run` cancellation law called the now-async `CancelToken::cancel()` without awaiting it (a
  must-use warning), so nothing was cancelled → `.await`.
- plugin-host schema-parity law: the actor export set gained `codec.replay-envelopes` (H9's ABI) → pinned.

## Processes (pids)

None running. Every detached run I started has exited (workspace checks 90546/20787/40340, renderer-react long runs,
root test-quick 49021, wgpu census 56831). Three of my own hung vitest runs (engine-contract, pids 1633/1639/6541/6543/
8798/92613–92618) were killed by pid after the flow pump starvation was diagnosed. `wp-r8/target` holds only 4 KiB.

## Owner round 2 (coordinator 07:2x): repo-cli bin `repo`, repo-lib laws, flow residuals, held flow fixes

### `semio-framework-repo-cli` bin `repo` — evidence and decision

- `git log -S real_repository` / `-S mcp_verb` (all refs): the only commits are `bb961413d4` (2026-09-14, **adds**
  `📦️mcp-main.rs` + the 09-06 ticket report `📓️opus-cli.md`) and `82c0bdf59a` (a later report). No revision ever contained
  `mod mcp_verb`, `fn real_repository` or `repo_cli`; `git log --all -S'fn real_repository'` is empty and none of the
  27 617 unreachable blobs contains it (read-only `git cat-file --batch` scan).
- `📓️opus-cli.md` (ticket 26/09/06 REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE) documents a complete Rust port of the
  repo CLI — regions `Ansi … Dispatch2`, `McpVerb` (`RepoRepository`, `real_repository`, `serve`) — "appended to
  `🦀️.rs`" of `⌨️cli/📦️packages/🦀️rust/`, beside a concurrent dashboard executor rewriting the same file. That file's history
  (`8add1df147^`: 910 lines, orchestrator regions only; deleted/moved in `8add1df147`) never held those regions, while
  the port's other outputs (bin `repo`, 8 Rust adapters under `⌨️cli/🧪️tests/*/🦀️.rs`, the README's "binaries `semio`,
  `repo`", the oracle manifest's "reference Go implementation the vectors were read off") did land.
- **Decision: accidental loss, not a retirement.** The port's working-tree regions were overwritten before any
  auto-commit captured them; nothing states an intent to drop the Rust twin, and AGENTS.md requires multi-implementation.
  Restoring properly = re-porting from the Go reference (`⌨️cli/📦️packages/🐹️go`: `🐹️.go` 6 892 + `🖨️render.go` 724 +
  `🔌️mcp.go` 2 502 lines) with `📓️opus-cli.md` as the design, over the existing Rust domain crates, until bin `repo`
  and the 8 Rust adapters compile and the cli parity cases run the Rust subject.

#### Restoration (landed 09-25 ~10:45)

| Piece | Where | Twin of |
| --- | --- | --- |
| `FsRepoContext` — the production `RepoContext` (all 49 methods) over `🎫️tickets` `TicketService`, `🎯️goals` `Goals`, `📝️todos`, `🧑️contributors`, `📜️statutes` (breach cache + autofix), `🗂️codebase`, `🗣️languages`, `🚚️move` plans through `FileSystemExecutor`, `🧩️providers` (GitHub management port adapter, `syncManagement`) | `🔗️graphql` crate, region `🗄️FsRepoContext` (+11 domain deps; only `⌨️cli` depends on `🔗️graphql`, so the DAG stays acyclic) | Go `graphqlpkg.NewRepoContext` |
| `repo_cli`: event stream + engine, NDJSON/human/markdown renderers with Go-exact number/duration/escape spelling, the immutable command framework, the whole declared tree (38 root verbs, every flag), one handler per verb, `FsTreeSource`, stream filters, harness entries (`projection`, `usage`, `command_paths`, `render_stream_json`, `graphql_roundtrip_json`, `export_records_json`, `test_verb_lines_json`, `hook_verb_dispatch_json`, `mcp_conversation`, `run`) | `⌨️cli/🦀️.rs` region `🧭️RepoCli` | Go `⌨️cli` package (`🐹️.go`, `🖨️render.go`) |
| `mcp_verb::RepoRepository` / `real_repository` — the 9 tools, 8 resources (GraphQL → YAML), 4 prompts | `⌨️cli/🦀️.rs` region `🔌️McpVerb` | Go `🔌️mcp.go` handlers |
| bin `semio-repo` (`📦️cli-main.rs`, new) = the repo CLI; bin `repo` = MCP-only server (profile from env) | `⌨️cli/📦️packages/🦀️rust/Cargo.toml` | Go `semio-repo` / `💻️client` MCP binaries |
| `AGPL_LICENSE_TEXT`, `git_author` | `🏠️workspace` crate, region `🪪️Authorship` | Go `AGPLLicenseText`, `GetGitAuthor` |

Decisions: the repo verbs get their own binary instead of re-entering the `semio` orchestrator (whose `test` and
`micro-commit` already delegate to the root script, so the 09-06 dispatch arm would have changed their meaning); the
handshake adapter serves the recorded conversation in process through `repo_cli::mcp_conversation`, exactly as the Go
adapter does, instead of spawning a binary from a stale `target/` path; `autofix` runs `FsRepoContext::autofix`
instead of the Go `mutation Fix` document that the schema never declared; the cli/mcp `📜️script.ts` binaries resolve
through `cargoTargetDirectory`. Remaining named gap: `technology <name> generate requirements|docs|todos` refuses
(no Rust twin of `GenerateTechnology*` in `🏃️test-runner`).

Evidence: `cargo check -p semio-framework-repo-cli --all-targets` success, zero warnings; `semio-repo --help`,
`ticket list --json`, `goal list` (md/text), `ticket show`, `mcp --dry-run`, arity refusal verified by hand; bin `repo`
stdio probe (`generated/repo-mcp-probe.txt`): initialize (unknown member tolerated), 9 tools, `repo://goals` YAML,
`prompts/get`, a refused `section_move`, and a real `section_move` + `file_integrate` + `section_extract` round trip on
scratch files. **Language-agnostic cases: `⌨️cli` parity quick — 8 cases, 50/50 passed (25 Rust + 25 Go),
parity 25/25** (`generated/cli-parity-1.txt`); `🔗️graphql` parity quick 34/34, parity 29/29 (`graphql-parity-1.txt`);
graphql + workspace crate unit tests green; **workspace check run 4 EXIT 0** (`ws-check-4.txt`).

### `@semio-tech/repo-lib` workspace-contract laws (round 2 §2) — root fixes

Root `test quick` for repo-lib runs one bun file, `🔬️workspace-contract/🟦️.ts` (696 laws incl. 7 imported suites), under the
300 s quick budget. Runs: 4 → 76 fail (313.8 s); 5 → 73 fail / 623 pass (347 s); 7 (load 40–58) → 53 fail, 1609 s (real-repo
walks: `discoverPackageProblems` 883 s, layering area policy 205 s, `computeWorkspaces` 126 s); **8 = the real gate
`bun ./📜️script.ts test quick` (13:16, load 15): 611 pass, 83 skip (levelled `long`), 2 fail, 155 s, inside the 300 s budget**
(`.🧬semio/🌐hub/s11-r8-captures/repo-lib-test-quick-8.txt`; runs ≤ 6 were deleted by the 12:16–12:35 low-disk sweep).
The 2 reds are the JCO physical-matrix laws (routed below). Typecheck (`bun ./📜️script.ts typecheck`): repo program 0 errors
after the fixes; the second (coordinator/Next) program surfaces 26 pre-existing errors that were masked while the first failed
(Next's `ProcessEnv` requires `NODE_ENV`, library env helpers build plain objects; one DOM `ReadableStream` iterator) → open.

Fixed (each rerun green in isolation):
| Law(s) | Root cause | Fix |
|---|---|---|
| projected scenarios single-emoji identities | 09-09 decoupled logical scenario id from physical directory (592 real catalog rows differ); fixture/oracle still required equality | oracle checks a kebab physical identity; fixture row `🔌️disconnect` expected true |
| OS semantic-stem canonical leaves | `flow-family-package-manifest` contract removed 09-13 with the family manifest | law expects no contract for the family path |
| JCO companion / interface / Flow output specifiers | 09-08 test-move codemod rewrote `"./x.js"` module specifiers into `"../../📦️packages/🟦️typescript/x.js"` | restored `./` specifiers (3 laws + 3 synthetic literals) |
| JCO destination vendored shim | preview2-shim bumped to 0.25.0 (jco 1.34); jcoprobe fixture still vendored the 0.2x set | re-vendored all 15 `dist/browser/*.js`; law derives the file set from the installed tool; jcoprobe bun harness S1–S4 ALL PASS on the new shim |
| JCO counts | wfc interfaces registered 09-18 (82→83 parents, 246→249 components) | counts = taxonomy truth; physical matrix routed (below) |
| cargo provider manifest projection | codemod turned `…/provider/Cargo.toml` into `…/providerCargo.toml` | fixture locator restored |
| TypeScript declaration oracle | oracle moved to `../🔮️typescript-declaration-facts-oracle/` | asset path |
| glTF generator / TSV schemas / mutation catalogs | `🔮️oracle` collection renamed `🔮️oracles` (taxonomy `testOraclesDirName`); `mutationVectorRegistryBreaches` moved to `🧪️test/🟦️.ts` | paths |
| captured structural schema checks | functions moved from root `📜️script.ts` to `📐️structural-reachability` as exports | law reads the module |
| mutation metadata facts | aliases carry `restricted: true` for non-`pub` items since 09-02; schema/fixture lacked it | schema `restricted: {const: true}`, fixture rows |
| STDIO TXT/GLTF metadata origins | stdio composes independently compiled artifact crates; txt/gltf leaves derive `value_derive::ToValue/FromValue` besides `dsl::MutationLeaf` | law uses each artifact crate as consumer; **library fix**: derive routes consider only `MutationLeaf` terminals (unrelated derives no longer void the proof); 2 new fixture vectors (unrelated derives accepted, double genuine derive rejected) |
| semantic collection census (4) | codemod-corrupted import specifiers + non-canonical `🧪️test/🟦️s.ts` leaf in the synthetic fixture | specifiers `../../🔨️modules/📏measure/🟦️`, `#[path]` depth, canonical `🧪️tests/🟦️.ts` |
| package handoff: classifier | `canonicalDirectory` calls `targetInsidePackageBoundaryFinding`/`mutationDomainOwnerLocation` since 09-12; the extracted-source harness did not inject them | injected + `discoverySchema`; explicit 30 s timeout (per-scenario `structuredClone` of the taxonomy) |
| package handoff: UI-host (2) + resident | contract pinned the 09-09 package: admission oracle moved into `🧪️tests/🔬️input-admission`, second oracle `🎡️ordered-scroll`, browser-host run before cargo, lib at `../../🦀️.rs`, `🖌️render` rename, wasm target cfg, project.json, rust-version 1.95 | schema-first: `sourceOracles`/`sourceCases`/`browserHost`/`cargo.libSourcePath` in schema + fixture; laws iterate the declared oracles and prove each is imported and called exactly per scope |
| UI-host source oracle itself (`bun 📜️script.ts test source` was EXIT 1) | value schema lost its `U64` export on 09-09; two admission schemas still `$ref` it (13 refs) | restored `U64` in `🌱️value/🧬️schema/🔣️.json` (single decimal-string u64 owner); oracles EXIT 0 |
| taxonomy: `🖥️host` under any module | kind `repo-test-host` (parent `members-of-modules`) shadowed the UI host member, leaving its whole subtree unclassified | removed the two `repo-test-host*` kinds; `🏗️materialization` and `🎡️ordered-scroll` registered as module members |
| discoverBurndown message law | `unknown-lang` message lacked its path; scratch `🗑️generated` under `📦️packages` was scanned as a language | message path-qualified; discovery-skip dirs skipped |
| playground ports (33–48 s!) | port table walked the whole repo reading every `package.json` | reads the declared root `workspaces` (new `declaredWorkspaces`), 47.7 s → 38 ms, identical table |
| Nx Unicode describe graph | spawns a cold isolated-plugin project graph: 71 s measured | levelled `long` (`test.if(testLevelAtLeast("long"))`, 180 s) |
| generator preview protocol + 10 normalization laws | planning evaluated generator contracts whose owner is absent (synthetic repos) and demanded the registry script | generator contracts with an absent `ownerPath` are inert in planning |

| rollback-at-stage laws (4) | `🟦️subject.ts` + `🟦️consumer.ts` in one test case both collapse to `🟦️.ts` (semantic-stem ambiguity) → no moves | consumer moved to `🧪️consumer/🟦️.ts`: 1 move + 1 edit, 4/4 green |
| direct mutation ownership: orphaned folder | the captured source view is git-admitted; an empty `➕️insert-page` directory is invisible | orphan carries its component leaf `🦀️.rs` |
| direct mutation ownership: test presence | the 09-02 codemod split `🧪️tests/🦀️.rs` into `🧪️test/🦀️s.rs` and `🧪️outside/🦀️.rs` into `🦀️outside.rs` | fixture restored; byte-identical to the 08-27 original; rustc oracle 25/25 |
| git/rustc-spawning mutation laws | 5 s default timeout killed their git child under load | explicit 30 s timeouts (8 laws) |
| `bun:test` ambient types | `test.if` / `describe.if` undeclared | `🏃️process/🌿️environment/🟦️.d.ts` declares both |
| cache-contracts / mcp binary-sources test typing | root rows now `{ id, appScoped }`; literal schema widened | typed oracle roots; `CargoBinarySourcesV1` annotation; explicit guard |

Removed `[DEBUG]` console lines in the handoff laws (7).

Levelled `long` by cost (each walks the real repository or builds git fixture repositories; they stay red/green at `long`,
where `bun ./📜️script.ts test long` now passes a per-law timeout equal to the level budget):
| Laws | Cost (run 7) | State at long |
|---|---|---|
| real-repo discovery census: `discoverPackages` ×5, `discoverBurndown` ×4, `computeWorkspaces` real repo | 4–883 s | **red** by repo state: 648 discovery problems (252 plugin, 396 non-plugin; top: 🧪️test 45, 🖥️server 27, 🖱️ui 26, 🌎️hub 16, os plugin 13, ⌨️cli 13; the repo module family has implementation inside `📦️packages/<lang>` for every Rust/Go twin + role `library` undeclared) |
| layering ratchet ×4 | 7–205 s | **red**: `🔣️schema-catalog.json` 3067 implementation refs (baseline 0), test fixture `🌐️html-source-pair` 1 |
| glue classification of actual root scripts | 63 s | green |
| Nx isolated describe graph | 71 s | green |
| normalization engine family: `taxonomy normalization`, `artifact path projection authority`, `taxonomy transaction dispositions v2`, generator preview apply | 131 s | **35 red / 34 green** (measured with `--timeout 120000`): 15 = frozen CAD/Draw projection authority names pre-09-05 CAD example files (`Missing or duplicate CAD scenario source identity …constructOneWayReinforcedConcreteSlabFrom2PointsAndHeight.json`); Draw live-source moves (`✏️editor/🎮️commands/…` ENOENT, launch-seed `devLaunchers` marker); ~12 planner expectations predating the semantic-stem/source-admission rules; singles (symlink leaf, gitlink boundary, `compatibilityAlias` TypeError) |

Still red in quick (2): **JCO physical matrix** (interface filename boundaries, companion triples) — they read the gitignored
`🧑‍💻dev/🔌️plugin-modules` staging, last staged 09-15 (wfc never staged; reasoning/remodel/stdio wasm missing; 112 stale
interface files). Needs W2's os-dev restage; on a fresh clone the tree does not exist, so these belong behind the staging step.

### ui-contract retirement laws (U5 §7 hand-off) — diagnosis, fix scheduled after W2's handoff

`semio-framework-ui-contract --lib`: **172/197, 25 red** (`ui-contract-1.txt`; same set with U5's edits reverted, per U5).
Root cause (measured on `retained_binding_copy_separates_allocation_clone_and_placement`: after 5 turns every close step
answers `progressed:false, released 0` for 100 000 turns): `PagedList::release_empty_page` (`🌱️value/📋️list`) frees a page
whole or not at all, so a byte grant narrower than one page can never free it. Two later patches treated the symptom in
opposite directions — the typed list retirement passes the caller's grant (stalls forever), while the document ladder
raises the grant to the node table's footprint (puzzle3d B52: "a grant narrower than one page is a ceiling that NO amount of
further retirement can meet"), which releases more than granted and breaks the laws' `released_bytes <= grant`.
The repository already decided this class once, in the job crate (`charge_payload_page`: grants **accrue** until the
physical page is paid; each turn spends at most its grant; bounded turns, never refused). Fix = the same accrual in
`PagedList` (a paid ledger + `paid_bytes` in `PagedListProgress`; callers that pay ≥ the demand are unchanged), typed
retirement reports paid bytes, the document ladder drops its raise; list laws move from "exact−1 refuses" to "exact−1
pays and frees next turn". It touches `semio-framework-replication` (every guest links it), so it would force W2's in-flight
34-package release build to recompile from the root: **scheduled for right after W2's Hub Handoff**, together with the
ui runtime `runtime_tree_retirement_*` twins.

## Routed to owners

- **W2**: os-dev `🔌️plugin-modules` restage (JCO physical-matrix laws; wfc never staged) + flow guests and flow_core bindings in the full
  catalog (request appended 12:0x). Hub data of W2/S15/C10 under the ticket folder is untracked and not ignored (Windows 260-unit
  checkout law flagged 60+ paths at 11:30; green again at 13:16).
- **Coordinator decision**: the normalization-engine family (35 red at `long`) pins a completed CAD/Draw migration authority; re-derive
  it against the renamed CAD examples or retire it with the migration. The repo module family's Go/Rust twins keep implementation inside
  `📦️packages/<lang>` (396 non-plugin census problems) — a taxonomy decision (Go cannot leave its module directory).
- **T12/S15 (plugins)**: 252 plugin discovery problems (fem 31 …); stdio fixtures still carry codemod-split `🧪️test/🦀️s.rs` names
  (`📜️direct-mutation-contract`, jpg/bmp/png/tiff, txt unit test).
- Hygiene seen, not done: tracked stale `🟦️.js` twins of repo-lib modules (09-19 auto-commit of a tsc emit, 1 MB each); `[DEBUG]` lines in
  ui host source oracles and ~14 other library laws.
- Seen, not mine to change now: `@semio-tech/framework` no longer loads under Node `--experimental-strip-types` (TypeScript parameter
  properties, e.g. `📡️replication/🎮️mutation/🗂️map/🧬️schema/🟦️.ts:11`, `💻️os/🟦️.ts:3602`), so any strip-only consumer of the playground
  port module fails before reaching it; Bun and esbuild-bundled Vite configs are unaffected.
- Held until W2's full publish (soft freeze): ui-contract `PagedList` accrual fix (13 `release_empty_page` callers incl. pack
  retained catalogs, ui reconcile, flow retained, ui-contract retirement) + ui runtime `runtime_tree_retirement_*`; the unbailed root
  `test quick` census.

- **WG7**: renderer-wgpu Shell source laws `no_chrome_maintenance_lane_arms_itself_without_pressure`,
  `the_footer_pills_are_not_gated_off_the_browser_build` (browser sync backbone gating, their Shell wgpu change).
- **W2**: flow `wasm` publish (`wp-w1/requests/r8.txt`) so the gitignored bindings copy of the flow host runtime matches source.
- **NEW (repo tooling)**: `semio-framework-repo-cli` bin `repo` references the removed `mcp_verb::real_repository`
  (red ≥ 11 days; Rust repo-MCP twin vs Go server decision); `@semio-tech/repo-lib` ~30 red bun laws block root `test quick`.
- **Flow owner / NEW**: the 3 non-deferred residual flow laws above. **R8 after W2 handoff**: the 2 product-side flow fixes.
- **T12/S15**: 88 unmapped DSL example fixtures reported by the fleet sweep (empty/placeholder `.semio`, envelopes without a
  registered `ArtifactDsl`).
- Hygiene, not done: ~220 `[DEBUG]` console lines remain in renderer engine tests (some are asserted by laws).

## Files changed (R8)

- Round 2 repo-lib (09-25 11:30–13:25): `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`, `🧪️tests/🔏️path-emoji-statutes/🟦️.ts`,
  `🧪️tests/📣️typescript-declaration-facts/🟦️.ts`, `🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`,
  `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`; library `🔍️discovery/🟦️.ts` (derive routes, unknown-lang), `🧹️normalization/🟦️.ts`
  (absent generator owners inert), `🎮️playground/🟦️.ts` + `🗂️workspaces/🟦️.ts` (`declaredWorkspaces`), `🏃️process/🌿️environment/🟦️.d.ts`,
  `📦️packages/🟦️typescript/📜️script.ts` (long per-law timeout), `🔣️taxonomy.json` (−`repo-test-host*` kinds, +`🏗️materialization`,
  +`🎡️ordered-scroll` members); fixtures/schemas: `🔏️path-emoji-statutes`, `📽️cargo-provider-projection`, `🪪️mutation-metadata` (+schema),
  `🏷️metadata-source-provider` (+2 vectors), `✅️mutation-test-presence`, `🤝️package-language-kind-handoff/{🖥️ui-host-package,💾️resident-package}`
  (+schemas); `🌱️value/🧬️schema/🔣️.json` (`U64`); `💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/*.js` (15, re-vendored 0.25.0);
  `🌉️mcp/🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts`; removed an empty stray `$PWD/` tree under `🔌️plugin/📇️registry/`.
- Held flow fixes (12:00): `🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`, `🌊️flow/🌿️vcs/🦀️.rs`.

- TS/fixtures: `🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json`; registry tests `🚀️launch`, `✅️catalog-complete`,
  `🎮️playground-session` (`[DEBUG]` removal); renderer `🧬️schema/{🔣️.json,🟦️.ts}` + `🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope/🔣️.json`;
  renderer engine tests `🌐️settings-locale-panel-refresh`, `🖋️ink-canvas-domain-interaction`, `🕸️node-graph-domain-interaction`,
  `🔌️plugin-runtime`, `🛑️scene-pointer-cancellation`, `🚪️opening`, `🔬️engine-contract`, `🔄️shell-utility-leaves`;
  taxonomy `generatorContracts.wgpu-frame-worker` (check-in + browser-actor modules); presentation-react `📜️script.ts`;
  print `🧪️print-pipeline-verification/{🧪️tests/🖨️pipeline/🟦️.ts,🧫️fixtures/🧫️command-boundaries.json}`.
- Flow JS: `🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js` (pump alternates message/timer rounds), `🕸️wasm/🧪️tests/🖥️host/🟨️.js`
  (fairness law), `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts`.
- Flow ABI admission of 2608 (before W2's freeze notice): `🕸️wasm/📡️protocol/🦀️.rs`, `🧬️schema/🔣️.json` (108→109),
  `🧫️fixtures/🔣️.json`, `🧪️tests/{🔬️protocol-unit,🔬️component-domain-laws}/🦀️.rs`, `🧪️tests/🧬️schema-oracle/🟨️.js`.
- Rust: `📐️geometry/🎲️random/🧪️tests/🔬️unit/🦀️.rs` + `📐️geometry/📦️packages/🦀️rust/Cargo.toml` (Cargo.lock −1);
  `🏃️run/🏗️bootstrap/🦀️.rs`, `🏃️run/🧪️tests/🔬️unit/🦀️.rs`; `🖥️host/🧪️tests/{🔬️host-unit,🔬️registry-unit}/🦀️.rs`;
  `🎭️actor/🦀️.rs` (test projection); `🎯️action-bus/🧪️tests/🔬️unit/🦀️.rs`; `🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs`;
  `🎠️kernel/🧪️tests/🔬️extension-activation/🦀️.rs`; `🔌️plugin/🖥️host/🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs`;
  flow tests `🌿️vcs/🧪️tests/🔬️flow-vcs`, `🕸️wasm/🧪️tests/{🔬️component-domain-laws,🎬️draw-list}`, `🖥️host/🧪️tests/🔬️unit`,
  `🧩️extensions/🕸️wasm/🧪️tests/🔬️unit`, `🖍️drawing/🧪️tests/🔬️drawing-kernel`, `📔️registry/🧫️fixtures/🔣️.json`;
  flow core `📋️project.json` (test-quick/long/exhaustive).
- DSL fixture sweep: new `🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs`, package `Cargo.toml` (test root +
  workflow artifact dep; Cargo.lock +1). Deleted the unreferenced stale duplicate `💻️os/📚️examples/♻️reuse/` (4 files).
- Ticket inputs kept: `wp-r8/r8-launch-resolver.ts`, `flow-close-laws.py`, `mc-starvation-probe.ts`,
  `print-boundaries-probe.ts`, `fixture-sweep-fleet-9869c6e99b.rs.txt` (recovered source).
