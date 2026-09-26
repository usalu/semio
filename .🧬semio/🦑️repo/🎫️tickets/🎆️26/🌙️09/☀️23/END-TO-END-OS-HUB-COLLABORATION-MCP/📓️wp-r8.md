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
| 2 | workspace `cargo check --workspace --all-targets --keep-going --message-format=short` | run 1 (01:22, EXIT 101): 3 crates red. Run 2 (02:02, EXIT 101): 5 crates red. **Run 3 (06:45–06:57, EXIT 101): 2 crates red** — `semio-framework-os` lib test (my host-unit edit vs. the peer's `vcs.edits` → `ArtifactHistoryLedger`; fixed right after, `os-host-check-2.txt` EXIT 0) and `semio-framework-repo-cli` bin `repo` (**NEW**, see classification). Every plugin, hub, MCP and renderer crate compiled. **Run 4 (09-25 10:55–11:02): EXIT 0** — `Finished`, 0 `could not compile`, after the repo-cli restoration (round 2 §1) . **Session 12 (09-25 23:44): every member by explicit `-p` (264), `--all-targets`, EXIT 0; wasm-gated sets EXIT 0 on wasip2 (41) and unknown-unknown (30)** | `generated/ws-check-{1,2,3}-errors.txt`, `ws-check-4.txt`, `s12-r8-captures/ws-check-1.txt`, `wasm-check-1.txt` |
| 3 | framework/os crate lib tests (nextest `--profile quick`, private target) | kernel `--features sync,ureq` **1207/1207**; db `--all-features` **770/770** (+2 `#[ignore]` Docker lanes); replication+trace+pack+async+geometry+actor+hash 741/743 → actor fixed → **actor 126/126** (others green in that run); framework+os-run+plugin-host+os host 549/555: the 6 left are all **needs W2 rebuild** (staged guests predate H9's `codec.replay-envelopes` ABI); `semio-framework-os-flow` 37 → **1 red** (§Flow, 260/261 after the 2 held fixes landed 12:00; the 1 needs W2's flow bindings publish) | `kernel-nextest-1.txt`, `db-nextest-1.txt`, `batch1-nextest-1.txt`, `actor-nextest-2.txt`, `batch2-nextest-1.txt`, `batch2b-nextest-1.txt`, `flow-nextest-5.txt` |
| 3w | renderer-wgpu:test (audit P1-4) | **cargo 1386/1388 (7 skipped), vitest 399/399 (38 files)**. The audit's "~89 failures" is stale (R1: 1372/1372 on 09-24). The 2 reds are Shell source laws that pin `#[cfg(not(wasm32))]`/`#[cfg(wasm32)] … Detached` gating of the browser sync backbone and footer pill — exactly what WG7's browser-actor hub `connect` changes (Shell wgpu target, in flight) → **handed to WG7**: `💓️chrome-maintenance-pressure` `no_chrome_maintenance_lane_arms_itself_without_pressure`, `🌓️appearance-tour-and-footer-pills` `the_footer_pills_are_not_gated_off_the_browser_build` | `wgpu-nextest-1.txt`, `wgpu-vitest-1.txt` |
| 4 | root `test quick` (`bun nx run workspace:test-quick`, 869 tasks) green + exit code propagates cargo failures | **Round 3 (18:0x): repo-lib `test quick` 612 pass / 85 levelled / 2 fail (JCO staging → W2), typecheck EXIT 0; normalization family at `long` 69/69; repo-lib `test long` 691 pass / 1 levelled exhaustive / 7 fail in 508 s (JCO x2 → W2, discovery x3 + layering x2 → rename slice)** (`s11-r8-captures/repo-lib-test-quick-10.txt`, `repo-lib-typecheck-8.txt`, `norm-long-4.txt`, `repo-lib-test-long-2.txt`). Round 2 (13:16): 611 / 83 / 2 (§repo-lib). Unbailed root census deferred until W2's full publish (869 tasks would starve its builds). **Exit code: PROVEN.** A failing cargo run propagates: `nx run semio-framework-os-flow-core:test-quick` → nextest FAIL → script `process.exit` → nx **EXIT 1** (`flow-nx-test-quick-1.txt`); the root target fails on any failed dependency: `--parallel=1 --nxBail` run stopped on `@semio-tech/repo-lib:test-quick` with **EXIT 130** (bail interrupt) (`root-test-quick-1.txt`). **Green: NOT MET** — first red in fan-out order is `@semio-tech/repo-lib` (~30 red bun laws: repo tooling — JCO/flow compiler boundaries, package handoff, commit/micro-commit, command budgets, Nx transport), repo-tooling owner = **NEW**. A full unbailed census is not run: the fan-out includes `os-hub:test-quick` (hub mutex, H9) and `semio-tech-play:test-quick` (peer), which a single sweep must not run unguarded. Gate gap fixed: the flow crate's 261 laws were in no `test-quick` (project had only `test`) → `test-quick`/`test-long`/`test-exhaustive` added | `generated/root-test-quick-1.txt`, `flow-nx-test-quick-1.txt` |
| P1-6 | launch-registration resolver probe | **PASS**: launch.json 437 configs, 197 `nx run` pairs, 0 unresolved, 0 duplicates, 3/3 compounds resolve; committed launch.json byte-identical to a fresh render; seed 281 rows + 89 `@generated:` placeholders, 0 unresolved/duplicates | `launch-resolver-3.txt`, probe `wp-r8/r8-launch-resolver.ts` |
| P2-7 | K2 ownership-gate census (presentation-react, print, actor/cold-pair) | **all three collect and run green**: `@semio-tech/presentation-react` 147/147 (3 files; its default `test` ran at the 15 s fundamental budget and was killed → floored at `quick` like the O1b suites); `@semio-tech/print` `test` EXIT 0 (starts again; the command-boundary law counted Bun built-in `bun:sqlite` as an external package → `bun:` treated like `node:`, fixture updated); `🎭️actor/📥️cold-pair` in `@semio-tech/framework` 188/188 (4 files, suite listed by name) | `presentation-react-test-2.txt`, `print-test-2.txt`, `framework-vitest-1.txt` |

## Session 12

Session 12 (2026-09-25 22:50–). Captures: `.🧬semio/🌐hub/s12-r8-captures/` (durable). Private cargo: `.tmp-ticket/wp-r8/target`.

| # | Item | State | Evidence |
|---|---|---|---|
| S12-1 | Every TS project `typecheck` green, measured per project; framework + framework-os under Node strip-only | **DONE — 7/7 nx `typecheck` targets rc=0** (run 3 00:13 + framework-os run 4 00:20); strip-only law 3/3 | `tc/summary-3.txt`, `tc/framework-os-4.txt`, `node-strip-1.txt` |
| S12-2 | ui-contract retirement reds; root `test quick` unbailed census (count + exit code); root `test long` count | root cause corrected (item-metered backing, not `PagedList` accrual); patch set dry-run clean, measured 197/197 + ui-runtime 125/125 in isolation; lands after `--packages all`; test-only fixture truths landed (174/197 now). Root census: pending (load) | `ui-contract-1.txt`, `wp-r8/ui-retirement-item-metered.py` |
| S12-3 | repo-lib quick/long reds that needed the s restage (JCO physical matrix) — re-measure on restage4 | JCO fixed at the root (retired staging root); quick-file cost 475 s → 248 s (taxonomy parse memo), 0 fails unbudgeted; gate still load-bound at load 35–43 → re-measure at normal load | `quick-junit-2.xml`, `repo-lib-test-quick-4.txt` |
| S12-4 | workspace `cargo check` of every member (native) green; wasm32-only crate list checked through the wasm mutex | **DONE** — native EXIT 0 (264/264); 53 wasm-gated crates: wasip2 set (41) EXIT 0, wasm32-unknown-unknown set (30) EXIT 0 | `ws-check-1.txt`, `wasm-gated-crates-1.txt`, `wasm-check-1.txt` |
| S12-5 | flow bindings request for W2 (`wp-w1/requests/r8.txt`) | **DONE** — still owed after restage4, session-12 entry appended | `r8.txt` |
| S12-6 | rename-slice plan re-measured against today's tree (not started, by design) | **DONE** — discovery 647 (was 648), layering 254 files / 6 042 excess refs (was 255 / 6 698), 18 stale baseline entries | `discovery-census-1.txt`, `layering-census-1.txt` |
| S12-7 | Nx `inputs` narrowed to each plugin's real dependency closure; controlled hash experiment; prepared dry-run-clean, lands after `--packages all` | **prepared** — root cause = lone workspace negations (whole-repo filesets); simulated: one-plugin edit invalidates 59 → median 4 others, stdio-crate edit misses exactly its 51 Cargo dependents; patch `wp-r8/nx-narrowed-inputs.py` dry run clean | `nx-simulation-baseline-1.json`, `nx-simulation-narrowed-5.json` |

### Session 12 log

- 22:55 state: 0 R8 processes; W2's release batch b holds the wasm mutex (chain pid 83333); load 13; 161 GiB free. The 48 stray
  tracked `.js` (session-11 last step) are gone in `13163ff3ae` (19:34 auto-commit); the configurable-descendant node type is gone
  from discovery (`SemanticDescendantNode` = directory | kind-file | fixed-file) — verified by the typecheck/tests below.
- 23:00 **typecheck census run 1** (`tc/summary-1.txt`; the 7 nx `typecheck` targets, each target's own command
  `bun ./📜️script.ts typecheck`): framework rc=2 (17 lines / 12 distinct), framework-os rc=2 (1), os-hub-ts rc=2 (2), ui-react rc=2 (48),
  plugin-window-kits 0, renderer-react 0, repo-lib rc=2 (6). Fixed at the root:
  - repo-lib 6: my own session-11 removal of the configurable-descendant machinery dropped `referenceEdits` from
    `SemanticPathProjectionAuthority`; five laws still asserted it (workspace-contract ×4 incl. the golden type, draw-destination-observation) → removed
    (the golden fixture never carried the field).
  - framework: kernel label laws imported `LocalizedLabel` from the kernel, which only imports it → import from its owner (the generated
    ui-axes module); peer-overlay law cast a `PresenceViewKindInput` union → narrows on `kind`; styling suite built 4 partial
    `OwnedBuildServer`s → one complete `ownedServer(use)` helper; dock-axis / storybook-dock / Select / Tree / Window component laws
    (optional fixture fields, widened `rowExtent` literals, a raw string where `UiLabel` is required → `uiDataLabel`).
  - repo test module imported the Node-native `🕸️dependencies/🟨️.mjs` untyped → declaration `🟨️.d.mts` beside it (precedent
    `📚️library/🕸️dependencies/🧩️runtime/🟨️.d.mts`).
  - ui-react (48 → 0 of its own): its program re-checks the ui subtree incl. Bun laws and the tooling they import, but declared only
    DOM/React types → `types` gains `bun`, `dom-accessibility-api` mapped to its declarations (same as the framework program).
- 23:13 **run 2** (`tc/summary-2.txt`): **repo-lib 0, plugin-window-kits 0**; framework 1, framework-os 7, os-hub-ts 1, ui-react 1,
  renderer-react 4 — every remaining line but one is a **peer's in-flight edit** (files modified 23:05–23:13, uncommitted):
  (a) `BrowserActorUiMountedV1.windowKindId` added in os `🟦️.ts` (23:10) → `🧪️tests/🧪️backbone-envelope-io` L2389 not yet updated
  (framework, framework-os, os-hub-ts, ui-react); (b) `WgpuPluginHandle.codec` required (`🐚️plugin-bridge` 23:05, codec genesis for
  wasm32/React — WG8 lineage) → `🧩️package-integration` `fakeHandle` lacks a default `codec`; (c) `🎭️browser-actor-panels` law +
  fixture mid-rewrite (23:13, per-window stores). Re-measured after the peers settle.
  The one landed divergence: `🔗️hub-projection` law calls `hubConnectionSummaryV1(statuses, session)` without the `link` axis U5
  added (09-25 01:32). The React fold now has `local`/`online` and link-driven `reconnecting`/`connecting`; its Rust twin
  (`🔗️HubConnection/🎯️targets/🧊️wgpu` `hub_connection_summary`, used by the wgpu Shell's `shell_hub_connection_summary_v1`) and the
  target-neutral `🔗️hub-projection` fixture do not — no link value reproduces the fixture (`empty-is-offline` is `online` when
  reachable). Schema-first fix = link axis in the fixture + Rust fold + wgpu pill states (en/de) → wgpu Shell owner (WG8), routed.
- 23:14 repo-lib `test quick` run 1 (`repo-lib-test-quick-1.txt`, load 18–36, W2's 3-parallel release + peers, 19 rustc): budget-killed at
  300 s, EXIT 1 — 8 of 10 reds are 5 s default timeouts of taxonomy-loading laws (passed at load 15 in session 11) + the 2 JCO laws.
- 23:20 **JCO physical-matrix root cause** (not "needs a restage"): the taxonomy's 83 `dev-*-interfaces-*` directory contracts, 249
  companion contracts and 33 interface contracts describe the **retired second staging root** `🧑‍💻dev/🔌️plugin-modules` +
  `🧑‍💻dev/🧩️extension-modules` (last written 09-15; `🧑‍💻dev/🧫️fixtures/🔌️staging-root.json` lists it under `retiredRoots`) and the
  pre-codec interface roster. THE staging root is `pluginModulesRoot("dev")` = `🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules`
  (restage4, 60 components, extensions in the same root). Measured there: 60 component dirs, every one emits the same 28 interfaces
  (+ `semio-framework-codec` from H9's ABI), `wasi-random-random` in 35, `wasi-filesystem-*` only in layout; `plugin`/`contributor`/
  `host`/`byte-page` are no longer emitted; 1717 interface files. Fix (one-off `wp-r8/jco-matrix-rebuild.py`, dry run → apply,
  guarded against concurrent writes): the matrix is rebuilt from the staged tree — 60 directory contracts, 180 companion contracts,
  31 interface contracts, sets `dev-jco-all-interfaces` 60 / `dev-jco-random-interfaces` 35 / `dev-jco-filesystem-interfaces` 1
  (ids, reasons, authorities and dispositions kept where the component/interface still exists; +layout, energy, draw); law counts
  33/83/2027/249 → 31/60/1717/180. **Both JCO laws PASS** (targeted run, 50 pass incl. both; the 4 fails there are the same 5 s
  load timeouts). Same root cause fixed in every other reader of the retired root: Storybook `staticDirs` (served 09-15 guests) →
  `pluginModulesRoot("dev")`; the Storybook coordination manifest; the hub script's directory-Home browser attestation + runtime
  (read the 09-15 space guest) and the stdio catalog-root guard → `pluginModulesRootIn(repoRoot, "dev")`.
  Residuals: the retired directories still exist on disk (gitignored, nothing reads them now; the os tsconfig still excludes them) —
  pruning them is left to the user; on a fresh clone the two laws read a staging tree that does not exist yet (they belong behind the
  staging step — design note for the rename slice).
- 23:3x S12-5 flow bindings: still stale after restage4 (`🫀️core/🕸️bindings` 09-24 19:45 vs source 09-25 01:53, `cmp` differs at line 30);
  session-12 entry appended to `wp-w1/requests/r8.txt` (step + evidence + the two reds it clears).
- 23:44 **S12-4 native workspace check: EXIT 0** — `cargo check --keep-going --all-targets` with an explicit `-p` for each of the
  **264 members** (`wp-r8/s12-ws-check.sh`, niced, private target), 766 s at load 30–71, `Finished`, **0 `could not compile`**,
  3574 warning lines (proof of type-check) (`ws-check-1.txt`).
- 23:46 **wasm-gated crate list** (`wasm-gated-crates-1.txt`, derived from the native check's dep-info — every `.rs` each member
  actually compiled — grepped for `target_arch="wasm32"` / `target_family="wasm"` / `target_os="wasi"` / `target_env="p2"`):
  **53 members** carry wasm-target arms the native check cannot see. Split by the real closures (`cargo metadata --filter-platform`,
  normal+build edges; `wasm-check-sets-1.json`): **41** lie in the wasip2 closure of the 60 staged guest components (214 members),
  **30** in the wasm32-unknown-unknown closure of the browser renderer (`semio-framework-os-renderer-wgpu`, 85 members), **9** are
  native-only with wasm exclusions (os-mcp, os-run, plugin-host, plugin-describe, kernel-db, os-services, repo-dashboard, machine,
  os-scale-fixture). Queued `cargo check --lib --keep-going` for both sets through the wasm mutex
  (`wp-r8/s12-wasm-check.sh`, mutex pid 18855, FIFO behind WG7 + W2; capture `wasm-check-1.txt`).
- 23:5x–00:15 **S12-2 ui-contract retirement reds — root cause corrected, fix prepared.** Measured in an isolated copy of the crate
  closure (scratch cargo workspace of the 8 `🧰️framework/🔨️modules` crates ui-contract/ui-runtime need; own target, no shared
  build-dir, so W2's in-flight release sees no edit): baseline ui-contract **172/197**, ui-runtime **124/125**
  (`runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads`).
  - The session-11 plan (grant accrual inside `PagedList`) was built and measured, and **rejected**: +8 bytes per list moved every
    exact-size accounting law (6 416 → 6 448 per `UiNodeRecord`, resident permits, refresh sets) and it would recompile every guest
    from `semio-framework-replication`. The laws themselves decide the rule: typed retirement meters **payload** by bytes and
    **backing** by items — `UiFixedBytes` frees its emptied buffer as one item with zero bytes, and the typed-component, whole-patch,
    built-tree and document-alias laws all account logical payload bytes only. `UiFixedList` was the one owner that byte-gated its
    backing (`release_empty_page(bytes)`) and reported physical page bytes, so a sub-page grant stalled forever, and the document
    ladder's raise (`maximum_bytes.max(node table bytes)`, puzzle3d B52) then broke `released_bytes <= grant`.
  - Fix = patch set `wp-r8/ui-retirement-item-metered.py` (+ `-law.rs.txt`), **dry run clean**: the list frees its emptied backing
    one page per turn as an item (`release_empty_page(usize::MAX)`, zero bytes, documented), the document ladder drops its raise,
    and the one law that pinned byte-gating (`…obeys_actual_backing_grant`) becomes `…meters_backing_as_items` (a zero grant
    touches nothing, a one-byte grant finishes the list, items == backings). Measured with the patch: **ui-contract 197/197,
    ui-runtime 125/125, replication 292/292**. It touches the ui-contract lib every UI guest links → **lands in the window after
    W2's `--packages all`** (then: `cargo check -p semio-framework-ui-contract --all-targets` + wasm32 check + both nextests, W2
    restage for the guests).
  - Landed now (test-only, true on the current tree, no lib/guest compile): Table/TableRow (U5, 09-25) were appended to
    `Component` but three fixtures/schemas still pinned 19 variants (`🪞️copy`, `⚖️compare`, `⚖️compare/📃️document` → 21) and the
    typed wire fixture had no vectors for them (+2 sparse→normalized vectors, schema and law 21 → 23; the TS twin
    `RetainedUiTypedCursor` already decodes both); the resident-capacity law pinned the 09-12 measured static backing (566 352 B)
    while the 09-17 change `UI_BUILT_CHILDREN_MAX = UI_DOCUMENT_NODES` (a4cda597ea) grew the value arena to 2 077 760 B, so the
    byte ledger now admits **60** populated full documents and refuses the 61st (was 62/63; measured by the law itself) → fixture,
    schema consts, Rust law name and TS twin title updated; the capacity drop is that change's consequence (owner of the 09-17
    change to confirm). Real tree now: **ui-contract 174/197** (`ui-contract-1.txt`); the 23 reds are exactly the retirement-stall
    family the patch set fixes.
- 00:13 **typecheck run 3: framework 0, os-hub-ts 0, ui-react 0, plugin-window-kits 0, renderer-react 0, repo-lib 0**; framework-os 2
  (peer-settled lines): `🧩️package-integration` `fakeHandle` lacked the now-required `WgpuPluginHandle.codec` (WG8's 23:05 change,
  unowned for an hour) → one line `codec: async () => null` (preamble rule 12); `🔗️hub-projection` — the wgpu Shell owner landed the
  schema-first twin fix at 00:19 (fixture link axis + Rust fold). **framework-os run 4: rc=0.** → **7/7 targets green.**
  Node strip-only law (`✂️node-native-typescript`) **3/3** (framework + framework-os entries load under `node --experimental-strip-types`).
- 00:23–00:57 **S12-3 repo-lib quick.** JCO reds: fixed at the root above (both laws PASS). The gate itself is load-bound: run 2 (load 22–35)
  budget-killed at 300 s with 2 laws over their 5 s default. Per-law timing (junit, unbudgeted: `wp-r8/quick-law-timings.ts`,
  `quick-junit-1.xml`): 613 pass / 85 skip / 1 fail, **475 s** summed at load 35–40; the ten slowest were all `direct mutation
  ownership` (57/45/39/30/27/27/13/13/10 s). Root cause, profiled (`wp-r8/mutation-policy-stage-probe.ts`): every
  `policyMutationStructuralBreaches` call re-parses and re-validates the 1 MB taxonomy inside source admission (≈0.9 s of a 1.8–3.5 s
  admission), once per law vector; the rustc oracles cost ~0.1–0.3 s. **Fix (landed):** normalization's `loadTaxonomy` memoizes the
  parsed schema by content digest (bounded to 8 contents; the path only names errors, so the cached parse is exact) → admission
  **46–54 ms** after the first call; whole file **614 pass / 85 skip / 0 fail, 248 s** summed at load 25–40 (`quick-junit-2.xml`).
  The whole-repo `discoverBurndown … markerless manifests` law is levelled `long` (13:2x decision), four CPU-bound fixture laws got
  explicit 30 s timeouts (measured 5.6–6.6 s at load 35–42). Gate runs 3/4 (`repo-lib-test-quick-3/4.txt`, load 35–43): 610/85/4
  in 281 s, then 301 s budget-killed — the failing laws differ per run (5 s defaults, one 30 s law at 32.8 s): the machine is at
  3–4× its cores (W2's 3-parallel release + peers). **Re-measure at normal load**; the structural cost is fixed.
  repo-lib typecheck: my edits clean; the second program currently fails on a peer's in-flight `DirectoryCommand` edit in
  `💻️os/🧪️tests/🧪️backbone-envelope-io` (L2374/L2385, 00:4x).
- 00:58–01:07 **S12-6 rename-slice plan re-measured (not started, by design).** Discovery census (`wp-r8/repo-lib-discovery-census.ts`,
  `discovery-census-1.txt`, 19 s now vs 883 s in session 11): **647** problems (was 648) — `packaging-violation` 241, `manifest-without-marker`
  190, `package-implementation` 180 (was 181), `unknown-role` 26, `package-role-unresolved` 9, `target-inside-package-boundary` 1. Layering
  (`wp-r8/layering-census.ts`, `layering-census-1.txt`): **254 files over baseline, 6 042 excess references** (was 255 / 6 698), 44
  baseline entries of which **18 are stale** (their file is clean or gone → drop them in the slice); top: `🔣️schema-catalog.json` 3 067,
  CAD/Draw golden 480, root `Cargo.toml` 293, `🧼️remaining-package-purity-authority` fixture 252, root `📜️script.ts` 279/138, the `🧑‍💻dev`
  distribution bundle 128, `🔣️taxonomy.json` 118/7 (the JCO matrix now names the live staging root). The plan in "Decision 3" stands.
- 01:00–01:2x **S12-7 Nx inputs — measurement (design in progress, nothing landed).** Nx's own `HashPlanInspector` resolves every input
  a task hashes without running it (`wp-r8/nx-hash-plan-probe.ts`; graph 232 s, 60 plugin projects × describe/component-dev/materialize-dev
  = 180 tasks; `nx-hash-plan-1.json`, summary `nx-hash-plan-1-summary.txt`). Files hashed per task (min/median/max):
  **component-dev 700 / 8 468 / 10 649, describe 5 632 / 21 006 / 28 040, materialize-dev 73 142 / 73 142 / 73 146**; files of OTHER
  plugins inside a plugin's plan (median): component-dev 7 275, describe 14 802, materialize-dev 46 406. Example `note`: component-dev
  hashes 7 275 stdio files (note links 6 stdio codec crates, and each stdio artifact's `default` adds its whole owner tree
  `…/**/*.{json,semio,wit,…,ts,js}` — fixtures and examples included); materialize-dev hashes 73 146 files incl. 18 613 from
  `♻️mit-bestand` (png/md/jsonl research data) and `.cursor` plans → effectively the whole repository, so ANY edit anywhere misses every
  plugin's materialize — this alone explains the audit's 0–3 % hits. The per-crate `nativeSources` are already exact (Rust module graph);
  the leaks are the owner-tree globs in `default`/`production` (pulled in through `^production`/`^default`) and a still-unattributed
  whole-repo input in materialize (per-task attribution probe running: `wp-r8/nx-task-plan-probe.ts`).
- 01:3x Coordinator rule 18 (until lifted, ~B2 publish): every cargo/nx/bun test or build of mine runs under `nice -n 15`
  (my scripts `s12-wasm-check.sh` — still queued behind W2 on the wasm mutex —, `s12-ws-check.sh`, `s12-typecheck-all.sh` updated; probes
  launched with `nice -n 15`).
- 02:14–02:17 **wasm32 check of the wasm-gated sets (through the wasm mutex): `--target wasm32-wasip2` 41 crates EXIT 0 (83 s),
  `--target wasm32-unknown-unknown` 30 crates EXIT 0 (82 s)**, 0 `could not compile`, warnings present (`wasm-check-1.txt`). S12-4 done.
- 01:4x **S12-7 root cause found — lone workspace negations.** Attribution by re-planning note's materialize-dev one input at a time
  (`wp-r8/nx-input-attribution-probe.ts`, `nx-attribution-materialize-1.json`): every `!{workspaceRoot}/…` entry alone plans **~73–93 k
  files** (the whole repository), `^production` 89 889; the real positives are tiny (`browser-bundle/**/*.ts` 31, `default` 661).
  Controlled semantics (`wp-r8/nx-fileset-semantics-probe.ts`, `nx-fileset-semantics-1.json`): `[ws positive]` 31, `[ws positive,
  ws negation]` **24** (subtracts), `[ws negation]` **92 807** (everything else), project-rooted negations are harmless (7 → 6). A
  named input whose negations reach its positives only through a *reference* (`production = ["default", "!{workspaceRoot}/**/🧫️fixtures/**/*",
  "!{workspaceRoot}/<owner>/**/🧪️tests/**/*", …]`) therefore hashes the whole repository — in nx.json's global `production` AND in every
  plugin-generated `production` (`🟨️.mjs` `projectInputs`). Every task that reads `production`/`^production` — all materialize-*, and
  everything depending on them — misses on any edit anywhere. The per-crate `nativeSources` (exact Rust module graph) are fine; `describe`
  hashes `default`/`^default` (owner trees incl. fixtures) instead of the native closure.
  Design (prepared, not landed): (1) `production` inlines `default`'s entries and carries its negations in the same list (nx.json global:
  the lone `!{workspaceRoot}/**/🧫️fixtures/**/*` is dropped, the project-rooted exclusions stay); (2) the plugin sets `describe` inputs to
  the component's native closure (`nativeSources`, `^nativeSources`, its command sources) like `component-*`; (3) the cache-input
  boundary law uses Nx's real `HashPlanInspector` as authority instead of a flat pattern oracle (the flat oracle is why this passed).
  Proof by planning running (`wp-r8/nx-narrowed-inputs-simulation.ts`: the design applied in memory to the real graph, experiments
  (a) edit one plugin → other plugins that miss vs. Cargo dependents, (b) edit replication/ui-contract/hash/os-kernel → plugins that miss
  vs. Cargo dependents).
- 04:25 typecheck run 6 (load 40–71): framework, os-hub-ts, ui-react, plugin-window-kits, renderer-react, repo-lib **rc=0**;
  framework-os 1 — a peer added `suspend`/`resume` to `TaskManagerSourcesV1` (00:40) and updated its own component law but not
  `🧪️tests/🔄️shell-utility-leaves` → the two no-op members added (rule 12, one line); **framework-os run 7 rc=0 → 7/7 green again**.
- 03:4x–04:2x **S12-7 proof by planning (design applied in memory to the real graph, Nx `HashPlanInspector`, nothing run):**
  | | baseline (`nx-simulation-baseline-1.json`) | narrowed (`nx-simulation-narrowed-5.json`) |
  |---|---|---|
  | files hashed, materialize-dev median | 73 153 | **13 451** |
  | files hashed, describe median | 21 021 | **8 523** |
  | files hashed, component-dev median | 8 472 | 8 473 (already exact) |
  | (a) edit ONE plugin → other plugins that miss (median / max) | **59 / 59** (every plugin, every edit) | **4 / 51** (its Nx/Cargo dependents; 51 = stdio, linked by 51 plugins) |
  | (b) edit `stdio-dwg` / `stdio-png` → plugins that miss vs Cargo dependents | 60 vs 51 (9 extra) | **51 vs 51, 0 extra, 0 missing** |
  | (b) edit replication / ui-contract / mesh-engine / geometry | 60 = 60 | 60 = 60 (all plugins link them) |
  Residual non-dependent invalidations after the change: 5 plugin trees (procedural 14 plugins, flow 4, writer/draw/note 1) — real
  cross-plugin file reads (`include_str!` of a draw demo DSL in space, a flow editor fixture in sequence, flow-extension manifests in
  procedural), not globs. Known coarse inputs left as is: root `Cargo.lock`/`Cargo.toml`/`package.json`/`bun.lock` (toolchain contract).
  **Patch set `wp-r8/nx-narrowed-inputs.py`, dry run clean** (nx.json global `production` inlined without the lone workspace negation;
  `🟨️.mjs` `production` inlines `default`; component `describe` hashes the native closure; `🕸️graph` generator inputs → `generatorSources`).
  Lands after `--packages all` on the coordinator's word, then: re-plan without in-memory edits (must equal the simulation), the
  cache-input laws, and a new structural law (no named input may carry a `!{workspaceRoot}/…` negation whose positives come only
  through a reference — language-neutral vectors + Nx's planner as oracle).
- 04:3x–04:5x normalization family at `long` after the taxonomy parse memo (`norm-long-5.txt`, load 64): **68/70**; the two reds pass alone
  (`norm-prose-1.txt` 1/1 in 25 s, `norm-cad-draw-1.txt` 1/1 in 76 s — the batch failure was a load-starved `bun nx run
  @semio-tech/plugin-registry:generate` subprocess) → no regression from the memo.
- 04:5x **Blocked by load, not by code** (rule 18, W2's REST lanes on the critical path, load 40–71): the unbailed root `test quick`
  census (869 tasks incl. every crate's nextest compile), root `test long` count, and the repo-lib quick gate at normal load. They run
  when W2's `--packages all` is out and load is back near session-11 levels (≤ 15); nothing else in S12-1…7 is open.

### Session 12 files changed

- Typecheck: `🎠️kernel/🧪️tests/{🏷️history-entry-label,🏷️localized-label-fixture}/🟦️.ts`, `📡️replication/👕️peer-overlay/🧪️tests/🔬️unit/🟦️.ts`,
  `🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`, `🖱️ui/🧪️tests/{📐️dock-axis-geometry,📚️storybook-new-stories}/🟦️.ts`, `🖱️ui/🧱️elements/{🌳️Tree/📖️stories/🧪️.story.tsx,
  🌳️Tree/🧪️tests/🧩️component/🟦️.tsx,🔽️Select/🧪️tests/🧩️component/🟦️.tsx,🪟️Window/🧪️tests/🧩️component/🟦️.tsx}`, new `🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.d.mts`,
  `🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`, `📚️library/🧪️tests/{🔬️workspace-contract,📍️draw-destination-observation}/🟦️.ts`
  (`referenceEdits`), `📺️renderer/🧑‍🎨engine/🧪️tests/{🧩️package-integration,🔄️shell-utility-leaves}/🟦️.ts(x)` (one-line peer follow-ups).
- JCO matrix: `📚️library/🔣️taxonomy.json` (dev JCO section rebuilt), `🔬️workspace-contract/🟦️.ts` (counts); retired-root readers `.storybook/main.ts`,
  `.storybook/📖️stories/🧭️coordination/🟦️.ts`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`.
- ui-contract fixture truths: `🪞️copy`, `⚖️compare`, `⚖️compare/📃️document` fixtures + schemas (componentCount 21); `🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json`
  + schema (23 vectors); `🧪️tests/🔬️component-unit/🦀️.rs`; `🎟️resident/🔄️refresh/{🧫️fixtures,🧬️schema}/🔣️.json` + law name; renderer
  `🧪️tests/🎟️resident-refresh-budget/🟦️.ts` (title).
- repo-lib speed: `📚️library/🧹️normalization/🟦️.ts` (parsed-taxonomy memo); `🔬️workspace-contract/🟦️.ts` (one law → `long`, four explicit timeouts).
- Prepared, NOT applied: `wp-r8/ui-retirement-item-metered.py` (+ `-law.rs.txt`), `wp-r8/nx-narrowed-inputs.py`.
- Ticket inputs: `wp-r8/{s12-typecheck-all.sh,s12-ws-check.sh,s12-wasm-check.sh,jco-matrix-rebuild.py,typed-retire-accrual.py (rejected design),
  quick-law-timings.ts,mutation-policy-cost-probe.ts,mutation-policy-stage-probe.ts,layering-census.ts,nx-hash-plan-probe.ts,nx-named-inputs-probe.ts,
  nx-task-plan-probe.ts,nx-input-attribution-probe.ts,nx-fileset-semantics-probe.ts,nx-narrowed-inputs-simulation.ts}`.
- Processes: none running (every detached run of mine exited; the scratch workspace lives in the session scratchpad).

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

### Coordinator round 3 (15:2x): (a) Next typecheck, (b) Node strip-only framework, (c) normalization rebuild

**(a) DONE — second typecheck pass 26 → 0** (`bun ./📜️script.ts typecheck` EXIT 0, both programs; `s11-r8-captures/repo-lib-typecheck-6.txt`).
Root cause: the coordinator's Next tsconfig (`include: **/*.ts`) type-checked the package's tooling router `📜️script.ts`, pulling the whole
repo-lib under Next's globals (`ProcessEnv.NODE_ENV` required, DOM `ReadableStream` without async iteration). Fixes: Next program excludes
`📜️script.ts`, the repo program lists it in `files` (verified with `--listFilesOnly`: in repo 1×, in Next 0×); `.next/type/**` typo →
`.next/types/**`; env overlays typed `Partial<NodeJS.ProcessEnv>` (`devToolingEnv`, `orchestratorBudgetOpts`, `daemonBudgetOpts`,
`playPollingEnv`, `frameworkOsPlaygroundDevEnv`, transaction-v2 child), `repoToolCacheEnv` defaults its base to `process.env`; the Binaryen
download reads the body with a reader (cancelled on failure) instead of `for await`. The remaining path into the Next program is the
framework's in-source-test dynamic imports, which is by design.

**(b) DONE — `@semio-tech/framework` and `@semio-tech/framework-os` load under Node strip-only** (384 / 244 exports). The native surface is
what vite/vitest configs externalize (their bundler inlines relative imports and leaves bare specifiers to Node): measured over all 44 config
modules = exactly these two package entries. Their static runtime closures (61 / 80 files) had 15 TypeScript-only constructs (parameter
properties) in `⏯️tool-run`, `🎠️kernel` (2 classes), `💻️os/🟦️.ts` (2), `💡️inference/🚪️opening`, `📇️directory/🧬️schema`,
`📡️replication/…/local-interaction/📡️transport` → explicit fields. New law `🧪️tests/✂️node-native-typescript/🟦️.ts` (+ fixture/schema,
imported by workspace-contract, quick, ~20 s): 9 syntax vectors judged alike by Node's own `module.stripTypeScriptTypes` and TypeScript
`erasableSyntaxOnly`+`verbatimModuleSyntax`; the config-externalized entry set equals the declared set; every entry's live closure has no
strip-only finding and `node --experimental-strip-types` imports it (exit 0). The playground port module now loads under Node too (cad 6020).
Repo-wide census for the later slice: 1396 parameter properties outside the native surface (plugins 1254, os 83, repo 33, ui 17, hub 7,
replication 3) + 155 value imports of types + 3 enums + 1 value namespace (`🧊️3d`) — only loadable through a bundler; not needed today.
Real consumer check: `bun nx run @semio-tech/framework-os-dev:test-quick` loads its config and runs (159 pass / 4 fail). The 4 are peers':
`🚀️local-hub/🏃️execution` (O3/C10, 12:37) imports the repo-lib barrel into the vite config graph (60 modules > 40; denied module), and
ShellHost's new `?worker&url` import breaks the law's esbuild oracle → routed.

**(c) DONE — normalization family at `long`: 69/69 green** (was 35 red / 34 green; `s11-r8-captures/norm-long-4.txt`).
Captures: CAD/Draw subset `norm-draw-6.txt` (27/27), whole workspace-contract at `long` `wc-long-1.txt`, gate `repo-lib-test-long-2.txt`
(691 / 1 / 7 in 508 s; the 7 reds are JCO x2, discovery x3, layering x2).

| Area | Root cause | Fix |
|---|---|---|
| CAD golden (frozen mapping) | 09-05 path-budget renames (`a01.json`, `t-*.json`, `…-8a1d88.json`) and per-file emoji names (`🌙️arc.json`) broke the stem-identity binding; the tree now uses `🌉️` and `🕹️` | Rebuilt on the current names (`wp-r8/cad-projection-rebuild.py`): 229 authored `🔣️<stem>.json` sources, semantic stems recovered from rename history, `liveBindings` (source-root-relative `{source, live}`) bind every live carrier exactly; taxonomy interactions rule `🕹️interactions`; digest `b474…2d8f` |
| Draw golden and scenario | artifact renamed `🖍️draw` → `🖍️drawing`, projection applied | 9-member authored scenario, no live Draw reads; strict-union law checks the live applied destination tree (tests excluded) |
| Live CAD consumers in the fixture | consumer code names the live carriers; the fixture holds the authored sources | mounted content is rebound through `liveBindings`; the interaction spec's `🧪️tests/🔬️unit` child is mounted (the root join and 12 includes live there since 09-08) |
| Engine: Rust comment prose | recognizer hard-coded `🎬️interactions/*.json` | any declared category, and the directory form `…/*/🕹️interactions/`; form renamed `interaction-glob` → `category-glob` (taxonomy, discovery type, normalization) |
| Engine: Rust root join | rewrote a `CARGO_MANIFEST_DIR` join relative to the file directory (`../../../📚️examples/…`, one level off) | suffix substitution of the artifact-relative source root, base-agnostic (`../../📚️examples/🪆️1-any/🏗️models`) |
| Engine: kind-only findings | 09-12 added `taxonomy/kind-only-basename` to every leaf, so a plan that moves the leaf to a kind-only path stayed blocked | findings resolved by a planned kind-only move no longer block |
| Taxonomy | legacy `🦀️component.rs`/`🟦️component.ts` aliases in the CAD consumer contracts; `🔬️unit` test cases unregistered; `test-fixture-asset` inferred from a bare stem made every stem under a test case ambiguous | aliases removed; spec contract covers its unit child; `🔬️unit` added to `members-of-tests`; `test-fixture-asset` `inferWithoutEmoji: false` |
| Draw producer fixture | producer changed since the scenario was authored: nx runs through the bootstrap script, nx plugins import revisioned modules and read policy, the cargo command script and 26 stdio codec receipts; registry needs a deployment catalog row, `cdylib`, current devLauncher format, validator-clean routers | scenario declares `compilerRoots` = the package.json `nx` entry, `runtimeModules`, `runtimeData`, `runtimeReceiptCatalogs`, runtime packages `typescript`/`@iarna/toml`, an authored `deploymentCatalog`; host `cdylib`; seed with `wgpuOrder`/`env`; routers import `@semio-tech/repo-lib`; kind-only `🧪️tests/🧪️reference/🦀️.rs`; registry output 16 nodes. The launch law runs the generator from an isolated copy of its captured closure. `📏️field-parity` imports its type with `import type` |
| 13 laws with mangled fixture paths | the 09-02 kind-only rename (`21fbcd3538`) rewrote `"🧪️X/<k>.ext"` → `"<k>X.ext"` and `🧪️X/CACHEDIR.TAG` → `XCACHEDIR.TAG` in the law file | restored against `21fbcd3538^` (planner laws keep their deliberate legacy inputs; admission laws use canonical kind-only inputs); package-glue fixture made kind-only |
| Law truths that changed | scope below a gitlink is refused (source admission, 09-01); phases follow source admission (`tracked-enumeration` before `setup`, plus `source-observation`); no descendant contract has configurable entries any more | gitlink law renamed and asserts the refusal; phase and cancellation laws list the actual phases; the schema-boundary law checks closed descendant-node keys |
| Generator preview fixture | fixtures live in `tmpdir()` since 09-09, so `bun nx` found no nx; its script was not a command router | implementation in `🧪️generator/🟦️.ts`, router in `📜️script.ts`; nx and repo-lib linked |
| Apply rederivation | laws planned with an explicit source ticket and applied without it (empty directories dropped; the source-authority laws passed for the wrong reason) | apply passes `explicitTicketDir` equal to the plan's ticket |

Also: `discoverOwners` laws (whole-repo walk, timed out at 5 s under load) are levelled `long`; the census parity law
(`discoverPackageProblems` vs `buildSemanticCensus`, 883 s measured) is levelled `exhaustive` — alone it filled the 900 s `long`
budget and the gate killed the file (`repo-lib-test-long-1.txt`). Whole workspace-contract file at
`long`: the only reds are JCO x2 (W2 staging), discovery x3 and layering x2 (dedicated slice). A plain `bun test` run shows 21 more
reds with `SEMIO_TEST_ARTIFACT_DIR is required`; with the gate's environment they are 24/24.

### Decision 3 — tree-wide breakdown for the dedicated rename slice (after W2's final publish)

**648 discovery problems** (`s11-r8-captures/discovery-census-1.txt`, `discoverPackageProblems` on the live tree):

| Root cause | Count | Top offenders |
|---|---|---|
| `packaging-violation`: file has no exact fixed/configurable contract or file-kind identity (159); directory not an allowed package directory (82) | 241 | repo modules 85 (`🧪️test/📦️packages` 38, `🖥️server/🎛️coordinator` 18), stdio 40 (`📖️pdf` 11, `🧿️semio` 8), os modules 20, ui 17, animate 17, hub 9 |
| `manifest-without-marker`: package manifest without a semio role marker | 190 | stdio 74 (`📖️pdf` 12, `🧿️semio` 9), repo modules 29, os modules 15, cad 5, draw 5 |
| `package-implementation`: authored implementation inside `📦️packages/<lang>` | 181 | repo modules 93 (every module's Rust/Go twin: graphql, tree, providers, workspace, tickets, languages, test-runner 4 each; `⌨️cli` 9), os modules 12, ui 5, presentation 5 |
| `unknown-role` | 26 | role `library` x22 (repo modules' twin packages), `artifact` x2 (flow, dag), `test` x2 |
| `package-role-unresolved` | 9 | repo modules 5, ui dotnet styling 4 |
| `target-inside-package-boundary` | 1 | `🎤️presentation/📦️packages/🟦️typescript/🎯️targets` |

Suggested order: (1) decide the Go/Rust twin package convention (register role `library` or move twin implementations out
of `📦️packages`); this clears about 150; (2) add role markers mechanically (190); (3) stdio artifact packages (114) with T12;
(4) the rest per owner. Discovery also sees a new plugin owner `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react` (peer).

**Layering ratchet** (`🧅️layering.json`, 44 entries): 255 files carry 6698 references to implementation areas
(`✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`) beyond their baseline. The largest are `📚️library/🔣️schema-catalog.json` 3067,
the CAD/Draw golden 480 (it names CAD/Draw paths by nature; the owner-level fix moves golden and scenario into the plugins),
root `Cargo.toml` 293, root `📜️script.ts` 279 (baseline 138), `🧼️remaining-package-purity-authority` fixture 252 and the
`🧑‍💻dev` distribution bundle 128. By area: repo library 81 files, os dev 42, os plugin 17, renderer 17, mcp 12. `🌎️hub` being an
implementation area makes every framework reference to the hub count.

**Related tree-wide drift found this round:**
- `members-of-tests` registers 1311 test-case names, but the tree has 4560 `🧪️tests/🔬️*` directories, 308 `⛔️`, 296 `✅️`
  and 259 `🧩️`. Only `🔬️unit` is registered now (3285 dirs). A registry refresh belongs to the slice.
- The 09-02 rename mangled fixture paths in about 35 more workspace-contract laws that still pass, among them UI-host
  metadata, resident native metadata and capability facets. Restore them against `21fbcd3538^` in the same slice.

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
- **Coordinator decision (open)**: the repo module family's Go/Rust twins keep implementation inside `📦️packages/<lang>`
  (about 150 of the 648 census problems) — a taxonomy decision (Go cannot leave its module directory). The normalization
  family is rebuilt and green (round 3 (c)).
- **CAD owner**: the live CAD example tree matches neither the catalog's source contract (`🔣️<stem>.json`) nor its destination
  (`🕹️<stem>/🔣️.json`): per-file emojis (`🌙️arc.json`) and path-budget carriers (`a04.json`). A real CAD normalization run needs the
  authored names first; the rebuilt golden records them (`liveBindings`).
- **Repo tooling**: `configurableEntry` descendant nodes have no users since the Draw bundle became declaration-only, and the
  two validators disagree (normalization counts a configurable node twice in `realizedNodeCount`, discovery once): remove the
  machinery or reconcile it. Stale generator `inputPatterns` (tolerant, non-blocking): `dev-distribution-bundle`
  (`🚚️distribution/📐️schema.json`, `🔐️inputs.schema.json`, `🧬️manifest.schema.json`), `wgpu-frame-worker` (Trunk.toml,
  `🌐️.html`, tests, kernel seam).
- **T12/S15 (plugins)**: 252 plugin discovery problems (fem 31 …); stdio fixtures still carry codemod-split `🧪️test/🦀️s.rs` names
  (`📜️direct-mutation-contract`, jpg/bmp/png/tiff, txt unit test).
- Hygiene seen, not done: 48 tracked stale `.js` twins from a 09-19 auto-commit of a tsc emit (`03b1a41483`, 67 added, 65 next to
  their `.ts`; repo 21, os 11, demonstrator 19, ui/actor/assets 10; e.g. `🔍️discovery/🟦️.js` 1 MB, unreferenced); `[DEBUG]` lines in
  ui host source oracles and ~14 other library laws.
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
- Round 3 (15:20–18:1x), all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` unless noted:
  - (a) coordinator `…/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json`, `🦑️repo/tsconfig.json`; `🏃️process/🌿️environment/🟦️.ts`,
    `🏃️process/🟦️.ts`, `🟦️.ts` (env overlays), `🧪️tests/🔄️transaction-v2/🟦️.ts`, `⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts`.
  - (b) parameter properties → fields: `🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts`, `…/🎠️kernel/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🟦️.ts`,
    `💻️os/🔨️modules/💡️inference/🚪️opening/🟦️.ts`, `💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`,
    `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🟦️.ts`; new law `🧪️tests/✂️node-native-typescript/🟦️.ts`
    + `🧫️fixtures/✂️node-native-typescript/🔣️.json` + `🧬️schema/✂️node-native-typescript/🔣️.json`.
  - (c) `🧹️normalization/🟦️.ts` (category-glob prose, root join, kind-only move filter), `🔍️discovery/🟦️.ts` (form name),
    `🔣️taxonomy.json` (CAD interactions rule, plugin-registry inputs, CAD consumer contracts, `🔬️unit`, `test-fixture-asset`),
    `🧫️fixtures/📐️cad-draw-path-projection/🔣️.json`, `🧫️fixtures/🖍️draw-source-scenario/🔣️.json` + `🧬️schema/🖍️draw-source-scenario/🔣️.json`,
    `🧬️schema/🗿️artifact/⚖️laws/📏️field-parity/🟦️.ts`, `🧪️tests/🔬️workspace-contract/🟦️.ts`, `📦️packages/🟦️typescript/📜️script.ts`
    (source-residue/commit routes run at `long`), root `📜️script.ts` (13 dead re-exports removed, tool-job coverage self-tests
    loaded lazily), `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` (two restored artifact-source launchers).
  - Ticket inputs: `wp-r8/cad-projection-rebuild.py`, `draw-projection-rebuild.py`, `law-diff.py`, `law-diff2.py`,
    `law-corruption-scan.py`, `launch-closure-probe.ts`, `router-validator-probe.ts`.

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
