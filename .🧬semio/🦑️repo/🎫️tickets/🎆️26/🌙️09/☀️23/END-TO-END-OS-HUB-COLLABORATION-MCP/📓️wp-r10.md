# WP-R10 — AGENTS.md Compliance, Taxonomy + Launch Generation, Harness Productization, Goal Gate

Slice: R10 (session 14; continues R9 + V1). Coordinator: `main`. Ports: hubs 8120–8129, serves 6620–6629.
Scripts/codemods/patches: `.tmp-ticket/wp-r10/`. Expendable captures: `.tmp-ticket/wp-r10/generated/`.
Durable data: `.🧬semio/🌐hub/s14-r10-*/`. Private cargo: `CARGO_TARGET_DIR=.tmp-ticket/wp-r10/target` (native lane, build-fleet-b).

## Session 14

### Session 14b

Successor agent (2026-09-28 12:0x). Guest freeze ON (chain launched 12:02:46); window 3 FROZEN — nothing kernel-derive lands before "WINDOW 3 OPEN".

| # | Item | State | Evidence |
|---|---|---|---|
| R1 | Reconcile predecessor in-flight: docstring census (orchestration module) | DONE — scanner fixed (206 oracle disagreements → 0), law 52/52, tsc clean for R10 files, live gate end to end | §14b log 12:1x |
| R2 | Reconcile predecessor in-flight: dead deprecated TS removals (20:3x) | DONE — no importer left, tsc shows no error from them, rule-20 boot PASS (Home, `ready:s`, 0 pageerrors) | §14b log, `s14-r10-logs/serve-boot-probe-s14b-1.txt` |
| R3 | `[DEBUG]`→`[TRACE]` retirement | open + decoupled part LANDED 12:4x (413 sites, 143 files); gate `verify debug-tags` (5.12) + plan check LANDED 13:2x; frozen/coupled part (1 940 lines, 645 files) = window-3 codemod `wp-r10/debug-trace.ts --scope all`, dry-run clean | §14b log |
| R4 | Window-3 input re-dry-run vs the live tree (peer edited ~1 870 files overnight) + missing relay specs | READY — every relay recorded (S20, G12, Z4, SH2, ST2, WG11, C13, F3, S18; pending: none); apply tool extended; dry runs on the live tree: targets 14 in 4 project.json + 1 edit, seed clean, plan valid (11 steps, 69 checks); `@emoji` 0 unresolved, comment-hoist 0 need emoji, `[DEBUG]` 0 stale; runbook below | §14b log 12:5x–13:2x |
| R5 | png/zip owned codecs (+ classify three, image, typescript/nx) | png/zip patches dry-run clean, overlay zip test queued (detached); classification done; typescript/nx policy row LANDED 13:2x; `image` surface-enum fix prepared (`surface-image-error.py`, dry-run + scratch apply idempotent); `three` → owner/renderer refactor (documented, not started) | §14b log 13:0x–13:2x |
| R6 | Goal gate runnable end to end once 7800 on ALL | zero-touch `hubAdmin` LANDED (dev-hub provider publishes the admin capability; live PASS); live plan 48 checks (docstrings ×2, debug-tags, G12 ×2 added), 69 after window 3; end-to-end run waits for 7800 on ALL (runbook step 13) | §14b log 13:1x–13:2x |
| R7 | `three` behind the ui module's interface (coordinator 13:3x) | LANDED 13:5x — 5 host importers routed, explicit re-exports + explicit scene-port bindings, gate `verify interface-owners` PASS (0 imports, oracle agrees), dependency policy row; manifests → window 3 | §14b log 13:3x–13:5x |

#### Window-3 runbook (14b — supersedes the session-14 list; strictly serial, each step gated, start on "WINDOW 3 OPEN")

Gate per step = `bun wp-coord/taxonomy-load-probe.ts` + (discovery/render) the registry launch laws + ONE boot
(`bun wp-r10/serve-boot-probe.ts --port 6620`); a red step → `bun wp-r10/window3-apply.ts revert <step> --apply` and stop.
All inputs: `wp-r10/window3-spec.json` (pending: none).
1. `zsh wp-r10/window3-run.sh refresh` (read-only kinds re-probe incl. every relayed dir) → 2. `taxonomy` (S18's
   `🗂️set-named-layout` FIRST — cross-shell preference loss until its Rust twin lands; then all passes) → S18 applies its twin.
3. `discovery` (R9 patch + law) → 4. `targets` (14 targets in 4 project.json incl. WG11 journey configurations, C13, SH2,
   F3 + the Z4 `test-diff` quote edit) → 5. `seed` (5 rows, 8 edits, input default, Z4 `bun x` inspector row) → 6. `render`.
7. `plan` (new step `frontend-journeys`; 69 checks, schema-valid in dry run).
8. ST2 `--part code` (ST2) → R10 `python3 wp-st2/st2-apply.py --dry-run --part r10` → `--write --part r10` → refresh + taxonomy
   + render (158 stdio playground rows return).
9. Z4 `b123-fresh-clone.py` (after the taxonomy pass) → native `cargo check -p semio-framework-os-infinite`; Z4
   `devcontainer-lifecycle/apply.py` → refresh + taxonomy for its `🐳️containers/*/🔁️lifecycle` dirs; tell Z4.
10. R10 guest patches: `owned-png-host.py --apply`, `owned-zip.py --apply`, `surface-image-error.py --apply` → native checks
    (deflate, os-kernel, os, surface `--lib --tests`) + `zip_archive` tests + host raster law + surface unit tests → refresh +
    taxonomy (`🎒️zip`, `🎒️zip-archive-cases`, `🔬️media-export-raster-unit`).
11. `[DEBUG]` retirement, frozen/coupled part: `bun wp-r10/debug-trace.ts --scope all` (dry, 0 stale) → `--apply` →
    `bun wp-r10/syntax-check.ts generated/debug-trace-all-files.txt` (baseline 646/646 clean: TS parser, rustfmt, gofmt,
    JSON) → native `cargo check --tests` per touched crate + wasm32 for guest crates (lanes) + plugin host law (asserts the
    guest trace sites) + demonstrator/play acceptance filters + boot. Then a `[DEBUG]` census gate (root `verify`) and its
    plan check (5.x).
11b. `python3 wp-r10/three-manifests.py` (dry) → `--apply` (r3f + renderer-react: `three` → devDependencies in package.json and
    their `bun.lock` blocks, textual) → `bun install --frozen-lockfile --dry-run` clean → `verify dependencies literal-external`.
12. LAST: `comment-hoist.ts --apply` wave 1 (`🔌️plugin`, `🏪️store`, `📺️renderer/🧑‍🎨engine`, `♾️infinite`; dry run today
    1 338 blocks, 0 need an emoji) → checks → `at-emoji-strip.ts --apply` (818 files, 9 108 tokens, 11 picks, 0 unresolved;
    census fixture now excluded) → `verify docstrings at-emoji` must PASS → laws → boot.
13. Goal gate end to end once 7800 is on ALL: `bun nx run @semio-tech/repo-test-domain:acceptance-goal` (zero-touch
    providers; 7800 variant with `--hub-admin-capability .🧬semio/🌐hub/s13-w3-state-7800/admin-capability.json`).

#### Session 14b log

- 12:1x read preamble 14 (rules 1–21 + 14b), AGENTS.md, fleet tail, this report. Predecessor's last in-flight step = the docstring
  census: `docstringHitsOfText` / `runDocstringCensus` (orchestration module), fixture + law (source-census), root `📜️script.ts`
  verb `verify docstrings <at-emoji|emoji-first>` (landed 20:39; live run 20:41 `generated/verify-docstrings-1.txt`: oracle
  DISAGREES). Re-measured on today's tree (`wp-r10/docstring-census-probe.ts`): 206 disagreements, all oracle-only — Rust
  `/** … */` block docstrings were never scanned, and `@emoji` paragraphs inside a `///` run (not the opener) were missed.
  Fix (orchestration module only; exported API unchanged): `at-emoji` = every doc line whose opener (`///`, `//!`, `/**`) or
  block continuation (` * `) is followed by the token (the oracle's definition, incl. generator-emitted doc lines);
  `no-emoji` = openers only, `/** */` blocks scanned in Rust too, `/**/` + `/***` banners and `/**` behind `//` or a quote are
  not docstrings. Fixture: 1 case corrected (mid-run residue), 3 cases added. Results: probe **0 disagreements**, at-emoji 9 089,
  no-emoji 4 619 over 30 923 sources in 33 s (`generated/docstring-census-probe-2.txt`); laws source-census + goal-gate +
  hub-freshness **52/52** (`generated/acceptance-laws-s14b-1.txt`); tsc (`tsconfig-r10.json`: orchestration, 3 laws, os-dev
  script) → only `🌎️hub/🤝️integration-harness/🟦️.ts(463)` `BunServerWebSocket` unknown (Z4's forwarding proxy, not R10;
  relayed) (`generated/tsc-r10-s14b-1.txt`); live gate `bun ./📜️script.ts verify docstrings emoji-first` → FAIL as expected,
  oracle agrees, record published (`generated/verify-docstrings-s14b-1.txt`, 32 s).
- 12:1x goal plan (`🎯️acceptance/🎚️config/🔣️.json`, not a derive input): compliance step += `docstring-at-emoji`,
  `docstring-emoji-first` (criterion **5.11** = "every docstring starts with its emoji", AGENTS.md; `workspace:verify` +
  args, no new nx target). `readGoalPlan` accepts it (45 checks, 9 steps); `plan-targets-check.ts`: 46/48 declared, only the
  two `serve-hold` providers wait for window 3.
- 12:1x predecessor's deprecation removals (20:3x): `git grep` finds no importer of any removed symbol outside `.cursor/plans`
  prose; `s14-r10-logs/tc-deprecations-1.txt` (framework, framework-os, ui-react, renderer-react, repo-lib) lists only peers'
  errors (layout `FramePatch`, `UiLabel`, hub harness), none from a removal.


- 12:2x rule-20 boot for the predecessor's removals: PASS (`s14-r10-logs/serve-boot-probe-s14b-1.txt`: Home seated, `ready:s`,
  0 pageerrors, 32 s, 6620 freed). Landing rows written for the predecessor's three landed sets (goal gate 19:1x, removals 20:3x,
  docstring census) — none had one.
- 12:2x relays handled: S20 (io-matrix now ONE record; plan criteria 1.7+1.8(+5.2) in the spec), Z4 (row texts, dirs; fixed the
  `BunServerWebSocket` tsc error), G12 (`mcp-plugin-coverage-hub` landed in the goal plan now — existing target; durability check
  kept out: the gate has no catalog-root/free-port token), SH2 (`verify-home` target + 3 plan checks), ST2 (its `--part r10`
  becomes one serialized window-3 step after its part code), WG11 (target with 7 journey configurations, self-provisioning its
  serves → checks need only hub (+localServe); en ×7 + de for wasm32). All in `wp-r10/window3-spec.json`.
- 12:3x–12:4x **`[DEBUG]` retirement** (decision GO). Census on the tree: 2 356 tagged lines (1 868 in test files). Codemod
  `wp-r10/debug-trace.ts` classifies every line: `error` (thrown / expect-message → prefix dropped), `test-print` (a one-line
  print in a test file → deleted), `test-note` (such a print whose deletion would orphan a binding it reads — a Go compile error,
  a Rust warning → tag dropped), `script-print` (`📜️script.ts` status line → tag dropped), `trace` (runtime diagnostics, their
  consumers — WASI classifier, acceptance filters, asserted lines, comments, fixtures → `[TRACE]`), `manual` (multi-statement /
  multi-line format strings → tag dropped in tests, `[TRACE]` elsewhere, window 3 only). Coupling: every tagged line matching the
  literal skeleton of an error/print site follows its action (e.g. the retired-instance fixture quoting PluginRuntime's
  `program … no actor for instance …` error, which is thrown by the frozen plugin bridge too → the whole group waits).
  Safety guards: a deletion never leaves its predecessor dangling (arrow, brace-less if/else, open call) or its successor
  continuing (`.then`, operators); no deleted print had a side effect (`await`, mutation calls: 0); deleted print texts are quoted
  nowhere else (357 keys, 0 real foreign quotes).
  **Landed now (open during the freeze + decoupled): 413 sites in 143 files** (29 error prefixes, 383 test prints, 1 test-note;
  TS 140 files incl. `🔌️PluginRuntime/🟦️.tsx`, Go 3). Backups `generated/debug-trace-backup-now-*`. Go: the first pass deleted
  two `t.Logf` breach lines inside `for _, v := range` → `v` unused (Go compile error) → restored untagged by hand; that case
  is what the `test-note` class now catches. Proof: TypeScript parser 0 syntactic diagnostics over all 143 files + gofmt;
  `go vet` codebase package clean (the two cli test files are emoji-path packages staged by their script; change there = one
  untagged restored line + one deleted `t.Logf` whose operands stay used); renderer-react `typecheck` 1 error, a peer's
  (`🎬️MediaTransportHost/🧪️tests/♻️lifecycle` 04:59, untouched by me) (`generated/tsc-renderer-react-s14b-1.txt`); vitest
  engine-contract, the two tests whose error text changed: 2/2 pass (`generated/vitest-engine-contract-s14b-1.txt`); rule-20
  boot PASS (`s14-r10-logs/serve-boot-probe-s14b-2.txt`, 14 s, 0 faults).
  **Window 3 (frozen or coupled):** `bun debug-trace.ts --scope all --apply` = 1 953 sites / 646 files (47 error, 1 310 test
  prints, 36 test-notes, 57 script lines, 413 trace, 90 manual), dry run 0 stale (`generated/debug-trace-all.{json,diff}`); then
  per-crate native + wasm32 checks, the plugin host law (asserts guest trace sites) and the acceptance filters move with it.

- 12:5x–13:0x window-3 input kept current: `window3-apply.ts` learned `configurations` (WG11 journeys), exact-once
  `projectJsonEdits` (Z4 process3d `test-diff` quotes) and `seedTextEdits` (Z4 MCP inspector `bun x`), skips plan checks that
  already landed; the S20 + SH2 checks moved into a new plan step `frontend-journeys` (the frontend step hit the schema's
  16-check ceiling). Dry runs on the live tree: targets 13 new in 4 project.json + 1 edit; seed 5 rows + 8 edits + 1 input;
  plan **valid, 11 steps, 67 checks**. Landed directly in the goal plan (existing targets): G12 `mcp-plugin-coverage-hub` and
  `mcp-hub-edit-durability` (self-provisioning, criteria 3.12 + 4.4) → live plan 47 checks, `plan-targets-check` 48/50
  declared (only the `serve-hold` providers wait). New relays recorded: C13 `two-human-viewer` / `two-human-cross-undo` (need
  `hubAdmin`, which the gate only takes from `--hub-admin-capability` → zero-touch needs the provider to name the launcher's
  `admin-capability.json`; asked), Z4 `b123-fresh-clone.py` + `devcontainer-lifecycle/apply.py` and ST2 `--part r10` as
  serialized window-3 steps.
- 13:0x owned png/zip: both prepared patches still dry-run clean on the live tree; the zipcheck copy = live deflate + the one
  `mod` line. The overlay run queued at 12:48 (8 waiters ahead) was relaunched detached with its own queue stamp
  (`FLEET_TICKET_STAMP=20260928124843`, pid 27394, log `.🧬semio/🌐hub/s14-r10-logs/zipcheck-2.txt`).
- 13:0x **dependency classification (item 2, measured by `git grep`):**
  - `three` — NOT compliant as is. Owner = ui React target; it re-exports the whole namespace (`export * as THREE`, not an
    explicit re-export) plus named types, and public functions take `THREE.Object3D`/`THREE.Camera`. Four production modules
    import `"three"` directly instead of through the owner (renderer React target, `🌐️World3dHost` ×2, `🗺️WorldTerrainLayer`,
    infinite `🎨️r3f`), and the fem TS package declares it as a production dependency for probes/generators only. Its oracle
    use (fem 2d/3d, stdio step/obj/gltf loaders vs our Rust parsers) is independent of the React rendering, but the gate
    flags any overlap. Fix: named re-exports only in the owner, the four modules import from the owner, drop their direct
    declarations, fem → devDependencies; gate policy row "production declarations = the registered interface owner only".
  - `image` — partly compliant. Dev-only oracle in intrinsic-size and pixels (correct). Private use in infinite (world
    reference images) and the wgpu renderer (Scenes/Interpreter). **Violation:** surface's public error enums
    `FrameworkSurfacePaintError::Image(image::ImageError)` and `FrameworkSurfaceTiledMapError::Image(image::ImageError)`
    expose the external type (same shape as the `ZipError` fix; their `Json(serde_json::Error)` arms are T14's `serde_json`
    scope). Root fix = decode through the first-party stdio codecs behind the existing interfaces (as png/zip), making
    `image` dev-only and dissolving the oracle conflict (stdio jpg/bmp/tiff oracles). Guest-linked → window 3+ (owner:
    surface/infinite/renderer).
  - `typescript` / `nx` / `@nx/js` — declared as `dependencies` of the bootstrap tools manifest
    (`⚡️caching/🚀️bootstrap/🛠️tools/package.json`), which sits outside the root `bun.lock` workspaces → the truth module
    counts them unauthorized (`lock-owned=false`) and `typescript` as an oracle conflict (it is the `🗣️languages` oracle).
    Policy row needed in `🕸️dependencies/⚖️truth`: the bootstrap manifest is an authorized toolchain manifest owned by its own
    lock, its rows `repository-tooling`. Not landed (library code imported by chain scripts) → window 3.

- 13:0x–13:1x **zero-touch `hubAdmin` (coordinator: "make the hub provider zero-touch, law included"; C13's viewer /
  cross-undo checks and `mcp-security` need it).** Hub side (`🌎️hub/🚀️local-bootstrap`, open): schema-first
  `LocalAdminCapabilityV1` (`🧬️schema/🔣️.json`, `semio.hub.local-admin-capability/v1`: loopback origin, session-token
  capability, sessionId, expiresAt) + parser `parseLocalAdminCapabilityV1`; `startLocalSessionBroker(…, adminProfileId?)`
  keeps `admin-capability.json` `0600` in the data root — issued through the broker's own serialized pipe sequence at start
  and again whenever a caller creates `admin-request` (consumed; 5 s poll), removed on `stop`; exported
  `LOCAL_HUB_ADMINISTRATOR_PROFILE` / `_SUBJECT` (the os-hub script's secure-admin path now uses them instead of its inline
  literal). os-dev `local-hub` declares the administrator + admin subject and starts the broker with it. Gate: a hub
  provider's optional `adminCapability` path (schema + type); when the checks need `hubAdmin`, the command line names none
  and the gate's own hub provider is the hub, it waits ≤ min(readyBound, 60 s) for the file and hands it to
  `{hubAdminCapability}`; otherwise only the `hubAdmin` checks are blocked with the reason. Plan: hub provider
  `adminCapability: .🧬semio/🌐hub/hub-dev/admin-capability.json`; for 7800 runs pass
  `--hub-admin-capability .🧬semio/🌐hub/s13-w3-state-7800/admin-capability.json`.
  Proof: fixture `🎫️session-broker-v1` +6 admin cases, Ajv (hub schema) and the parsers agree on all 20
  (`wp-r10/broker-fixture-agreement.ts`); goal-gate law +2 runs (capability handed over / missing → only hubAdmin blocked)
  → acceptance laws **54/54** (`generated/acceptance-laws-s14b-2.txt`); tsc (gate, 3 laws, os-dev script, os-hub script,
  broker): only 6 pre-existing os-hub script errors at l.14433+ / lodash typings, none from these edits
  (`generated/tsc-r10-admin-1.txt`); **live** (`wp-r10/admin-capability-probe.ts`, hub on 8120 = B3 binary over an APFS
  clone of the B3 root, removed after): ready 12 s, file issued `600`, `/admin/api/documents` 200 with it vs 401 without,
  `admin-request` → fresh session + request consumed, developer broker session still issued, file removed on stop —
  PASS (`.🧬semio/🌐hub/s14-r10-logs/admin-capability-probe-1.txt`); rule-20 boot PASS (`serve-boot-probe-s14b-3.txt`).
  A dev hub on 8787 started before this change has no file → its hubAdmin checks stay blocked until it restarts.

- 13:2x **`[DEBUG]` gate (criterion 5.12 "no temporary-log tag in tracked sources")**: `debugTagHitsOfText` /
  `runDebugTagCensus` (orchestration; scope = tracked text files minus ticket tree, Markdown, `.cursor` and
  `DEBUG_TAG_CENSUS_EXEMPT` = the census's own module + the source-census fixture, which name the tag), cross-checked per
  file against `git grep -c`; root `📜️script.ts` verb `verify debug-tags` (no tag literal in the script); plan compliance
  step += `debug-tags` (live plan 48 checks; window-3 preview 69). The window-3 codemod excludes the same exempt files, so
  its site set = the census (1 940 lines / 645 files, 0 stale). Proof: source-census law +2 cases, acceptance laws **56/56**
  (`generated/acceptance-laws-s14b-3.txt`); live gate FAIL as expected, oracle agrees, 13 s
  (`generated/verify-debug-tags-1.txt`); tsc of the root script + census law: only the peers' layout `FramePatch` error
  (`generated/tsc-r10-root-1.txt`). The `@emoji` codemod now also excludes the source-census fixture (its `@emoji` openers
  are law inputs): dry run 818 files / 9 108 tokens / 11 picks / **0 unresolved**; comment-hoist wave 1 dry run 1 338
  blocks, 0 need an emoji.

- 13:2x **typescript / nx bootstrap policy row LANDED** (`📚️library/🕸️dependencies`, only root `📜️script.ts` imports it — no
  chain path): `DEPENDENCY_TOOLCHAIN_RECIPE_MANIFESTS` (inventory) = the Nx bootstrap recipe
  `⚡️caching/🚀️bootstrap/🛠️tools/package.json` → its own `bun.lock`; every section of the recipe is `repository-tooling`
  (so `typescript`, the `🗣️languages` oracle, is no longer a production declaration → its oracle conflict dissolves), the
  recipe is an authorized toolchain manifest, and its rows are audited against its OWN lock (`dependencies` +
  `devDependencies` of its root workspace) → nx/@nx/js become mandated-toolchain instead of the 2 toolchain-owner conflicts.
  Proof: 2 new self-test cases (recipe row excepted against its lock; a row its lock does not own fails the audit), `verify
  dependencies self-test` clean, direct check `{okMandated:[nx], okConflicts:0, staleFailures:1}`; tsc root script + truth:
  only the peers' layout error (`generated/tsc-r10-root-2.txt`). Live `literal-external` re-measure (~20 min scan) runs in
  the goal gate's compliance step (expected: oracle conflicts 6 → 5 now, → 3 after png/zip; toolchain conflicts 2 → 0).

- 13:2x `image` violation prepared (guest-linked → window 3): `wp-r10/surface-image-error.py` — both surface error enums
  carry the decoder message (`Image(String)`), `Display` prints it, `source` ends there, the `From<image::ImageError>`
  conversions stay (they only feed `?`); the one consumer (`matches!(…, Image(_))` in the paint unit test) is unchanged.
  Dry run clean on the live tree; applied to a scratch copy: exactly the intended 8 hunks, second run "unchanged".

- 13:3x–13:5x **`three` behind the ui module (coordinator: route the direct importers, no external types, law, rule 20).**
  Census first: 67 tracked files import `three`; outside the ui module every one is test domain (tests, oracles, probes,
  generators) except **5 host modules, all open** (no `🔌️plugin/`): infinite `🎨️r3f`, renderer React target,
  `🌐️World3dHost`, its `⏯️tool-run-trace`, `🗺️WorldTerrainLayer` (+ `import("three").Camera/Texture` type references in
  World3dHost ×10 and r3f ×1). Changes: ui React target — `export * as THREE` replaced by named re-exports of exactly the 31
  classes/constants the host modules use + `GLTFLoader`, `OBJLoader`, `ThreeOrbitControls` (the three addon) and the types
  `ThreeCamera`, `ThreeScene`, `ThreeTexture`, `NormalBufferAttributes`, `Ray`; the scene port's `three` member is no longer
  `typeof THREE` but `SCENE_THREE_BINDINGS` (`🔌️Ports`), the 36 named constructors/constants its consumers read (cad
  renderer, r3f, two tests); the ui icon-camera test imports three itself (oracle inside the owner) instead of the removed
  namespace export. Consumers: codemod `wp-r10/three-route.py` moved every three specifier into the file's existing
  `@semio-tech/ui-react` import; the type references became `ThreeCamera` / `ThreeTexture`. The cad plugin (frozen) needed no
  edit: it reads `sceneHostPort.three`, whose explicit bindings cover all 16 members it uses.
  Law/lint: `INTERFACE_OWNED_PACKAGES` ← `DEPENDENCY_INTERFACE_OWNERS` (dependency policy, `📇️inventory`: three → ui
  directory + its manifest), `interfaceImportHitsOfText` / `runInterfaceImportCensus` (orchestration; value, type, addon,
  dynamic and `require` imports; test domain from the taxonomy) cross-checked against `git grep`; fixture +3 cases; root
  `verify interface-owners` + plan check `interface-owned-imports` (1.10, 5.9) → live plan 50 checks. Policy: the interface
  owner's own production declaration is no oracle conflict (self-test +2: owner-only → none; owner + bypass manifest →
  only the bypass counts).
  Proof: census before the type-reference fix 11 hits (the lint catches), after **0, oracle agrees** (`verify
  interface-owners` PASS, `generated/verify-interface-owners-1.txt`); tsc ui-react 1 / renderer-react 1 / r3f+cad 319 /
  root+orchestration+deps 1 error = exactly the pre-existing peer errors (layout `FramePatch`, MediaTransportHost, cad
  spatial-kernel test typings), none in a touched file (`generated/tsc-three-*`, `tsc-r10-root-4.txt`); vitest ui icon-camera
  9/9 (`vitest-ui-icon-camera-2.txt`); acceptance laws 59/59 (`acceptance-laws-s14b-5.txt`); dependency self-test clean;
  **rule 20: `serve s react dev` on 6620 + program matrix `--only lowpoly,cad`: boot `ready:s`, 60/60 plugins loaded, both
  3D editors PASS (rendered, verb, edit/undo/redo, 0 faults)** — lowpoly = World3dHost, cad = the scene port
  (`.🧬semio/🌐hub/s14-r10-logs/matrix-three-1.txt`, screenshots `generated/matrix/r10-three/`), serve stopped.
  Window 3: `wp-r10/three-manifests.py` (r3f + renderer-react keep `three` for tests only → devDependencies in package.json
  and their `bun.lock` blocks, textual so node_modules stays untouched): dry run clean; scratch apply = exactly 3 files, JSON
  valid, lock parses, second run "nothing to do". Not in scope: `three-mesh-bvh` (r3f, production) — same pattern later.

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | Kernel-derive inputs (window 3, serial): V1 taxonomy directory registration, R9 launch-inputs discovery patch, launch rows regenerated by the generator | in progress | §log |
| 2 | Row 1.10 external-dependency violations (`✏️editor/🦀️.rs:108`, `optional_json_to_dsl`) | in progress | §log |
| 3 | Harness productization (S18, C12, WG11, G12, H13/H14) + ONE goal gate verdict table | in progress | §log |
| 4 | Comment-hoist codemod (61 emoji picks) + `@emoji` codemod: prepared + dry-run, land LAST in window 3 | in progress | §log |
| 5 | `[DEBUG]` leftovers, comments inside definitions, missing emoji docstrings, CRUD/compat shims (last 4 days) | in progress | §log |

### Window-3 runbook (R10; strictly serial, one step at a time, each gated; start only on the coordinator's "WINDOW 3 OPEN")

Tools: `wp-r10/window3-run.sh <step>` = `window3-apply.ts <step> --apply` (keeps every touched file's previous bytes in
`wp-r10/generated/window3-backups/<step>-<time>/`) → `wp-coord/taxonomy-load-probe.ts` → (discovery/render) registry launch
laws → `serve-boot-probe.ts` (one `serve s react dev` boot to Home on 6620). Any red → `bun window3-apply.ts revert <step>
--apply` and stop. Log per step: `.🧬semio/🌐hub/s14-r10-logs/window3-<step>-<time>.txt`; one landing row per step.

1. `zsh window3-run.sh refresh` (read-only, ~10 min): re-probe the scopes against the current candidate for directories
   created since → `generated/tax-kinds-window3.json`.
2. `zsh window3-run.sh taxonomy` — register (all passes + refresh), validator loop drops refused names, write, probe, boot.
3. `zsh window3-run.sh discovery` — R9's registry catalog content-input patch + the re-derived law; probe, launch laws, boot.
4. `zsh window3-run.sh targets` — nx targets from `window3-spec.json` (os-dev ×4, os-hub-ts ×2 + later relays); probe, boot.
5. `zsh window3-run.sh seed` — curated rows + row edits + input default; probe, boot.
6. `zsh window3-run.sh render` — launch.json from the generator; probe, launch laws (byte-identical), boot.
7. `zsh window3-run.sh plan` — goal-plan checks (not a derive input; any time after 4).
8. Guest patches of R10 (compile-atomic, native lane + wasm32 per the landing rules): `owned-png-host.py --apply`,
   `owned-zip.py --apply` → `cargo check -p semio-framework-deflate -p semio-framework-os-kernel -p semio-framework-os
   --lib --tests` + `cargo test -p semio-framework-deflate zip_archive` + os host `media_export_raster` law; then
   `refresh` + `taxonomy` once more (their new directories `🗜️deflate/🎒️zip`, `🎒️zip-archive-cases`,
   `🔬️media-export-raster-unit`).
9. LAST, on the coordinator's word: `comment-hoist.ts --apply` wave 1 (plugin SDK → kernel store → renderer engine →
   infinite; native + wasm32 check per family) then `at-emoji-strip.ts --apply` (all 809 files; anchors + targets in one
   pass), each followed by the per-crate checks, the launch/goal-gate laws and one boot.

### Session 14 log

- 18:26 slice start; read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED line yet), `📓️wp-r9.md`,
  `📓️wp-v1.md`, `📓️audit-s13-rules.md`, `📓️acceptance-s13.md` (1.10/1.11, NO HARNESS rows), `📓️fleet-13-agents.md` 14:00→.
- 18:3x state measured (read-only): launch.json **fresh** — `bun wp-r9/launch-render-probe.ts` (no `--write`): 439 projects,
  2 419 declared targets, 1 244 configurations, 0 uncovered, 0 duplicate names, VS Code parser 0 errors, identical to the
  committed file (`wp-r10/generated/launch-render-1.txt`, 139 s). R9's discovery patch `git apply --check` **clean**; R9's law
  patch **does not apply** (its hunk 1 — `path` in the rendered-projects type — is already in the tree; only the new law is
  missing) → re-derive. `🔣️taxonomy.json` = HEAD (git diff empty).
- 18:3x taxonomy: new read-only probe `wp-r10/taxonomy-unresolved.ts` (normalization's `inventoryTaxonomy` per scope, optional
  `--taxonomy <candidate>` so a candidate is proven without touching the live file). `🌎️hub/🧪️tests`: **22**
  `directory-kind-unresolved` (V1 counted 13; R9 21 at 16:02; +`🪞️pair-content` H12 16:00) — `generated/tax-unresolved-1.txt`.
  Cause read from `matchDirectoryKind` (`🧹️normalization/🟦️.ts:2269`): `🌎️hub/🧪️tests` resolves to the global kind `tests`
  (exact-id rule wins before the contextual `hub-tests`), so the hub's two registered child kinds (`hub-foundation-source-test`,
  `hub-socket-grant-command-source-test`, parent `hub-tests`) never match, and no member kind owned by `tests` lists the
  other 20 names. Wider scopes (hub, MCP, os-dev, repo test, plugin registry) running detached (pid 52390).
- 18:3x item 2: `✏️editor/🦀️.rs:108` (reasoning wires) is **already fixed** in the tree (T13, session 13: `os_pack::json`,
  `serde_json` in `[dev-dependencies]`). Remaining: `action-bus::optional_json_to_dsl(Option<serde_json::Value>)` —
  49 files / 73 call sites (`git grep`), guest-linked; T14 holds "5b dsl_value!" in its roster scope → coordinate, no fork.
  Gate `bun ./📜️script.ts verify dependencies literal-external` running detached (pid 41646).
- 18:4x coordinator relays: S18 asked for a shared serve fixture → answered (none exists; S18 writes the ONE
  `ensureDevServe` in os-dev, the goal gate consumes it as a `serve` pre-step). Harness contract sent to main → preamble
  rule 17 (slices port their harness as a 📜️script.ts verb writing an acceptance record; R10 owns project.json targets,
  generated launch rows, goal-plan checks and the verdict table).
- 18:5x **launch.json delta vs HEAD (A14-tree #3) — verdict: INTENDED, no row lost.** Name diff (`generated/launch-delta-1.json`):
  HEAD 1 403 → tree 1 244 configurations; 159 only in HEAD, 0 only in the tree; compounds 3/3, inputs 40/40. The 159 =
  **158** playground rows `🛠️dev🗄️stdio<variant>⚛️react` + `…🧊️wgpu🌐️wasm` for **79 stdio variants** (avi, bcf, docx*,
  dwg*, ifc*, pdf*, pptx*, semio-*, step*, xlsx*, zip*, …) that the playground registry no longer lists (the staged stdio
  `🔣️.json` ships only the text families — LB's shipped-fleet restore; ST2's 88-subset components bring them back through the
  same generator) + **1** `▶️directory-live-lanes🌎️hub🦀️`, whose target `os-hub:directory-live-lanes` is registered by the
  curated seed rows `⚖️gate🗂️directory-live-lanes🐘️postgres/🕸️neo4j` (generator rule: a curated row running `nx run P:T`
  registers that target; HEAD's file predates that rule). Proof the tree file IS the generator output: render probe 2
  (`generated/launch-render-2.txt`, after the file's 18:46:33 mtime, sha256 `2b0a4111…` unchanged across the run):
  439 projects, 2 419 declared targets, **0 uncovered**, 0 duplicate names, VS Code parser 0 errors,
  `identicalToCommitted: true`. Nothing to regenerate for window 3 beyond the planned passes.
- 19:0x coordinator decisions: item 2 split — T14 keeps 5b `dsl_value!` + renderer `serde_json`; R10 takes `png`, `zip`, the
  `typescript`/nx bootstrap-tools policy row, and classifies `three` + `image` (compliant-behind-interface or root fix).
  Gate measured (`generated/deps-literal-external-1.txt`, 18:35→18:5x): literal-external 247 (zero target counts every
  third-party row), production-reachable 84, oracle conflicts 6 (three, typescript, image, png, serde_json, zip),
  toolchain-owner conflicts 2 (nx, @nx/js in the bootstrap tools manifest).
- 19:1x **goal gate (item 3) extended** (repo test module, not frozen): schema-first `outcomes` (1–5 with en/de titles,
  required in every plan; a check serves outcome N through any criterion N.x — `readGoalPlan` refuses a criterion of an
  undeclared outcome) and `providers` (hub / serve / localServe: an Nx target held for the run, ready url + bound, `{hub}`
  / `{runDir}` tokens); the gate stands up every missing requirement zero-touch before the steps (dependency order, a serve
  provider needing an absent hub is not started), blocks only dependent checks with the provider's reason (en + de), and
  stops what it stood up in a `finally`; `goalSummary.outcomes` + a per-outcome verdict table heads `summary.{en,de}.md`
  and the console (`[goal-gate] outcome N PASS|FAIL|BLOCKED|SKIPPED`). Plan: 5 outcomes; providers hub = os-dev
  `local-hub` (8787, reuses a running dev hub), serve = `serve-hold --serve :6071 --hub {hub}`, localServe =
  `serve-hold --serve :6070`; program-matrix / tool-run / hub-sweep args moved to S18's `--serve` (+`--hub` for the sweep;
  de matrix now editor+viewer). New os-dev verb `serve-hold --serve <url> [--hub <url>] [--variant <v>]` (holds S18's
  `ensureDevServe` until SIGINT/SIGTERM; nx target in window 3). Laws: goal-gate **26/26** (was 16; +6 schema records incl.
  plan without outcomes / remote provider / backends provider / summary outcome 6, +4 runs: outcome verdicts, provider
  stands up + stops, failed provider blocks only dependents with its reason, serve provider without hub not started),
  source-census + hub-freshness **18/18** (`generated/goal-gate-law-2.txt`, `acceptance-laws-other-1.txt`); tsc over the
  orchestration + law **rc 0** (`generated/tsc-r10-2.txt`; run 1 had 1 narrowing error, fixed), os-dev script with the new
  verb 0 errors (`tsc-r10-1.txt`). Live compliance run detached (pid 37583,
  `.🧬semio/🌐hub/s14-r10-logs/goal-gate-compliance-1.txt`).
- 19:2x **item 4 prepared (dry runs, nothing applied — lands LAST in window 3 on the coordinator's word):**
  - `@emoji` codemod `wp-r10/at-emoji-strip.ts` (git-tracked files with the token, minus Markdown prose, the ticket tree and
    the taxonomy-pinned frozen asset `📽️nested-cargo-package-projection` whose sha256 the taxonomy checks): removes the token
    after every doc opener (`///`, `//!`, `/**`, ` * `, `#`, `"""`, incl. the generators' emitted lines and both `indexOf`
    anchors, so anchors and targets stay consistent); symbol markers (`⊕`, `√`, `⛶️`) are kept; 11 docstrings with no marker
    (bullet `•`, `ˆ`, plain text, and one mojibake `­ƒ╝️´©Å` in the CAD renderer) get hand-picked emoji checked unused in
    their file (🔘️ ×2, 🎩️, 📥️, 🌊️, 🔗️, 🪝️, 🏷️, 🔖️, 🫥️, 🪟️). Dry run **809 files, 9 086 tokens, 11 picks, 0 unresolved**
    (`generated/at-emoji-dry-2.{txt,diff}`, 32 s). Measured census before: `/**` 4 674, `///` 4 155, ` * ` 168, `//!` 100.
  - comment hoist `wp-r10/comment-hoist.ts` (R9's codemod; picks moved to data `comment-hoist-emoji.json` and **keyed by the
    definition's text + ordinal, not its line number** — the first line-keyed pass lost 9 picks to peers' edits between two
    dry runs 10 min apart). Wave 1 (plugin SDK, kernel store, renderer engine, infinite — R9's guest scope) on today's tree:
    **1 343 blocks, 72 new docstrings** (was 61 at 10:55) → 72 picks written by `wp-r10/comment-hoist-pick.py` from
    per-row hand-chosen candidates, first one not already opening a docstring in that file (18 took a 2nd/3rd choice;
    `plugin/🦀️.rs` alone has 1 568 docstrings, 199 distinct openers, 135 repeated); dry run 4: **1 343 blocks, 0 need an
    emoji** (`generated/comment-hoist-wave1-dry-4.txt`); preview of `⚛️reactor/🦀️.rs` reviewed (SAFETY notes become fn
    docstring paragraphs, in-body `//#region` markers dropped). Rest of the tree (R9 11:07: 10 296 blocks / 1 215 picks in
    1 919 files) = later waves, same tooling.
- 20:0x **goal gate live run 1** (`acceptance goal --only compliance`, pid 37583, log `.🧬semio/🌐hub/s14-r10-logs/goal-gate-compliance-1.txt`,
  records in `…/goal-gate-compliance-1/`): `dependencies-literal-external` FAIL in 1 261 s (same numbers as above);
  `production-placeholders` then sat **31 min at 0 % CPU in "Calculating the project graph on the Nx Daemon"** — TWO nx
  daemons are running for the workspace (pids 29609 since 19:07 and 37782 since 19:15, both ppid 1; not killed: not
  provably mine) → I SIGTERM'd my gate (its cancellation path ran: running check `skipped` "cancelled while running",
  the rest "cancelled before start", `summary.json` + en/de summaries written) and the stuck nx client 58070 (my child).
  The run proves the new **per-outcome verdict table** end to end in en + de (outcome 1 FAIL, 5 FAIL, 2–4 SKIPPED — only
  compliance selected) and the console lines `[goal-gate] outcome N …`.
- 20:0x window-3 input assembled (`wp-r10/window3-spec.json`, applied only by `wp-r10/window3-apply.ts <step> [--apply]`,
  one kernel-derive input per invocation; every step dry-run clean now): **targets** — os-dev `io-matrix` (S20),
  `channel-version-check` / `-generate` (G12), `serve-hold` (R10); os-hub-ts `agent-ceiling-check` (H13),
  `shutdown-drill` (H14) — inserted textually after the last target so each file keeps its formatting (previews in
  `generated/preview-*.json`, both re-parse); **seed** — 7 curated rows (G12's two channel-version rows at 206.171/.172,
  `⚖️gate🎯️repo-goal🔗️hub` (the prompted variant), `⚖️gate🚪️io-matrix⚛️react(+🌐️de)`, `⚖️gate🛑️hub-graceful-shutdown`,
  `⚖️gate🤖️hub-agent-ceiling`), 7 row edits (`⚖️gate🎯️repo-goal` becomes zero-touch — no prompts, providers stand the hub
  and serves up; S18's matrix/tool-run/hub-sweep rows move to `--serve` literal urls, de matrix editor+viewer), input
  `acceptanceHubUrl` default 7800 → the dev hub 8787 (`generated/preview-launch.seed.jsonc`); **plan** — `hub-agent-ceiling`
  (2.13), `hub-graceful-shutdown` (2.10), `io-matrix-en/-de` (1.7/5.2); **discovery** — R9's patch applies cleanly, the
  law is re-derived as an insertion (its type hunk already landed); **render** — generator only.
- 20:1x **item 1 taxonomy registration prepared (not landed — window 3):** `wp-r10/taxonomy-kinds.ts` (inventory through a
  ticket-local copy of the live normalization module whose ONLY change is `kindId` on directory entries,
  `wp-r10/instrumented/normalization.ts`) gives each unresolved directory's parent kind; `wp-r10/taxonomy-register.py`
  adds the name to the member kind that parent kind owns (`members-of-tests`, `-fixtures`, `-schema`,
  `members-of-members-of-modules`, …; creates `members-of-<kind>` where none exists), refusing names without an emoji,
  names without their variation selector and names the validator refuses. Four passes over hub, MCP, os-dev, repo test
  module, plugin registry (every pass re-inventoried against the previous candidate via `taxonomyPath`):
  **unresolved 322 → 28 → 10 → 4**; the 4 left are not registrations: `🌎️hub/📦️packages/🦀️rust/.semio/hub/db(+/catalog)`
  = hub runtime data TRACKED in git since 08-17 (`root.bin`, `directory.db`), `🚚️distribution/🏁completion` (emoji
  without VS16) and `🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1` (validator: invalid exact member) = renames by their
  owners. Candidate 3 (`wp-r10/taxonomy.candidate-3.json`): discovery `validateTaxonomy` **0 problems**; vs live: **290
  names added, 13 new member kinds, 0 same-owner conflicts (no name in two member kinds sharing an owner → no new
  ambiguity anywhere in the repo), nothing else changed** (static diff; the whole-repo census was stopped after 59 min
  at 9 % of its file pass — too slow under this load). Covers V1's 13 hub/MCP test siblings and every harness directory
  handed over so far (`🚪️io-matrix`, `🤝️hub-collaboration`, `🤖️agent-ceiling`, `🐳️docker-image`, `🔀️forwarding-proxy`,
  V1's `🎯️acceptance` tree, `🧑‍💻dev/🧪️tests/*`). In window 3 `window3-apply.ts taxonomy` re-derives it on the live file
  (all kinds passes + one fresh pass for directories created meanwhile), validates, then writes.
- 20:1x **item 2 prepared:** `wp-r10/owned-png-host.py` (os host `encode_rgba_png` → `semio_framework_pixels::encode_png`;
  `png` moves to `[dev-dependencies]` as the oracle of a new native law `🧪️tests/🔬️media-export-raster-unit`: 4×2 red
  SVG → rasterize → the `png` crate decodes RGBA8 4×2 all (255,0,0,255)); dry run clean. `wp-r10/owned-zip.py` + staged
  files `wp-r10/owned-zip/…`: new first-party ZIP container `semio_framework_deflate::zip_archive` (`🗜️deflate/🎒️zip/🦀️.rs`:
  deterministic writer — deflate, 1980-01-01 timestamp, UTF-8 flag, no extras; central-directory reader of stored/deflated
  entries, CRC-32 checked, per-entry size bound, zip64/encryption/multi-disk refused) + laws (fixture
  `🧫️fixtures/🎒️zip-archive-cases/🔣️.json`: CRC vectors incl. `123456789`→`cbf43926`, 3 archives, 5 hostile mutations; the
  `zip` crate as dev-dependency oracle in BOTH directions, deflated and stored) replacing the `zip` crate in the kernel
  `.sxt` extension packages and the os host space collection export/import — and removing the external type
  `zip::result::ZipError` from the public enums `ExtensionPackageError` / `SpaceZipError`; dry run clean (6 files + 3 new).
  Compile/law proof = overlay lane (next).
- 20:2x **window-3 validation harness measured:** `wp-r10/serve-boot-probe.ts` (S18's `ensureDevServe` local-only on 6620 +
  ONE headless Chromium; passes when the ready beacon is set and exactly `s-home-main` is seated; stops what it started,
  closes the browser) — baseline run on today's tree: **PASS** `{"reused":false,"seconds":75,"home":true,"beacon":"ready:s",
  "windows":["s-home-main"],"faults":[]}`, serve ready after 27 s, stopped, 6620 free
  (`.🧬semio/🌐hub/s14-r10-logs/serve-boot-probe-1.txt`). This + `wp-coord/taxonomy-load-probe.ts` is the per-step gate.
- 20:2x plan-target audit (`wp-r10/plan-targets-check.ts`): all 44 plan checks + the hub provider name DECLARED nx targets;
  only the two serve providers wait for `serve-hold` (window 3). Standalone zip-container test queued in the OVERLAY lane
  (13 waiters ahead; pid 25310, `wp-r10/zipcheck.sh`, log `.🧬semio/🌐hub/s14-r10-logs/zipcheck-1.txt`; private
  target/build dirs, no repo build-dir).
- 20:3x **item 5 — deprecations/compat (code changed in the last 4 days; census `git grep` over tracked sources):**
  - REMOVED now (dead: 0 callers outside their own definition; non-frozen TS): `uiAssetsVitePlugin` +
    `presentationRendererVitestStripPlugin` (ui styling vite builder), `NavbarTrailingFullscreenSlot` (ui React target),
    `documentFromEnvelopeJson` + `wrapArtifactEnvelope` (throw-only stubs, os `🟦️.ts`), `DEFAULT_TEST_BUDGET_MS` (repo
    library), `parseSpaceShellPath` + its `SpaceShellPath` type + both re-exports (renderer React target) — its only user
    was a redundant engine-contract case; the `parseShellRoute` case gained the one assertion only the removed case had
    (`…/instances/inst-1/extra` → `notFound`). Rule 20 proof running: typecheck of framework, framework-os, ui-react,
    renderer-react, repo-lib (`s14-r10-logs/tc-deprecations-1.txt`) then the serve boot probe.
  - ROUTED (live compat in other owners' code): ShellHost `@deprecated … kept as a thin URI-parsing adapter only for the
    existing sync-card UI` (S18: sync-card should call `openDocument`); `🔌️plugin/🦀️.rs:6071` per-app fallback wrappers
    accepting singular `id`/`nodeId` keys (puzzle, sequence, trinity, procedural, mindmap — input-shape compat; frozen);
    `trinity/🔌️jack/🧠️lsp` "compatibility shim for existing launch targets until callers migrate" (frozen);
    `🛢️db/🗿️artifact/🦀️.rs:1415` "Deprecated-in-spirit extension seam, kept defined" (frozen); norm `En1994Artifact`
    `@deprecated` alias with 2 users in the en1994 schema TS (S19/norm owner).
  - `[DEBUG]` census (files changed in 4 days, `generated/debug-hits-1.txt`): **1 181** tags — Rust prints 530, TS console
    310, `*_debug_log` helpers 133, other 161, comments 42; 366 of them in production (non-test) files, 47 files. The tag
    is used as a PERMANENT channel today (WASI console classifier `line.startsWith("[DEBUG]")` in the browser bundle, the
    plugin host law asserting guest `[DEBUG]` trace sites, `dag_debug_log`/`document_debug_log`/`engine_canvas_debug_log`,
    the renderer's runtime-armed per-frame dumps, shard-client's "deliberate, permanent" line) — which contradicts
    AGENTS.md ("`[DEBUG]` marks temporary logs"). Root fix needs one naming decision (proposal: runtime-armed permanent
    diagnostics → `[TRACE]` incl. the WASI classifier and the host law; 📜️script.ts status lines drop the tag; thrown
    errors drop the prefix; explicitly temporary lines — "temporary watchdog widening" ×38, "temporary" streak traces,
    test prints — are deleted by their owners). Almost all of it is frozen guest/renderer code → window-3 codemod after
    the decision; asked main.
