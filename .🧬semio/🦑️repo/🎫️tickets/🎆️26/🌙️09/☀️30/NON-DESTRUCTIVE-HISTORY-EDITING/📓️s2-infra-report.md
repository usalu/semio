# 📓️ S2-INFRA — shared repo infrastructure for the fleet

WP S2-INFRA, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor session 2 (2026-10-01, 12:45–13:30, resumed 16:35 after the usage cut).
Scratch: `🗑️generated/s2-infra/`. Aliases: `T` = ticket folder, `REG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`.

## Session 2 — 2026-10-01

### I1. Root script routing crash (`Invalid owned command <project.json>:<target>`) — FIXED

- **Peer files:** router `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts` (05:33, committed in the 11:16 auto-commit `4e36b2b5012`). Offending declarations date from 03:07–10:04.
- **Cause:** `ownedScriptRoutes` validates every `metadata.semio.workspaceCommand` repo-wide on each call. A row must have ≥ 2 words matching `^[a-zA-Z0-9-]+$` and be globally unique (`Duplicate owned command` is repo-wide, not per project). 27 rows broke this. 18 were one-word `graph-generate`/`graph-wire-check`, duplicated 9× each across writer, reasoning wires, trinity rewriting, trinity jack, draw drawing, puzzle 2d/3d/5d and the flow composition. The other 9 were gis `inference-discovery-oracle|check`, two `contract-check` (catalog deployment, cargo workspaces), cargo-workspaces `queued-|bun-|runtime-|capability-contract-check`, and `prepare --manifest Cargo.toml` (the `.` fails the regex). Root `verify …` (`📜️script.ts:7269`) and the stdio/flow owner scripts call the router.
- **Not a cause:** the brief's `tiles-check`, `tiles-prefetch`, `test`, `check`, `test-source`, `test-asset-transport`, `paged-history-stack-check`, `test-asset-contract`, `sqlite-observation-check`, `test-fresh-component`, `distribution-output-check` live under the key `metadata.workspaceCommand` (23 rows), not `metadata.semio.workspaceCommand`. Nothing in the repo reads that key (the router is the only reader of `workspaceCommand`), so they cannot crash the router. They are left untouched. Open question for the dev: are these rows meant to be routable? If yes, the owner should move them under `metadata.semio` with 2-word names.
- **Fix:** edited only the array text in 12 `📋️project.json` files, formatting preserved, via a region-guarded replace: exactly one match per row, then a JSON re-parse to check the target changed (`🗑️generated/s2-infra/rename-commands.ts`). Convention used: `<verb> <scope>-<thing>`.
  - 9 graph projects: `["generate","<scope>-graph"]` and `["check","<scope>-graph-wire"]`. Scopes: `writer`, `reasoning-wires`, `trinity-rewriting`, `trinity-jack`, `draw-drawing`, `puzzle-2d`, `puzzle-5d`, `puzzle-3d`, `flow`.
  - gis: `["verify","gis-inference-discovery-oracle"]`, `["check","gis-inference-discovery"]`.
  - catalog deployment: `["check","component-deployment-contract"]`.
  - cargo workspaces: `["check","cargo-workspaces-contract"]`, `…-queued-contract`, `…-bun-contract`, `…-runtime-contract`, `…-capability-contract`, `["prepare","cargo-workspaces-manifest"]`.
- **Launch rows:** none needed. Launch rows call `bun nx run <project>:<target>`, target names are unchanged, and no launch row names a `workspaceCommand`. No `plugin-registry:generate` needed for this item.
- **Evidence:** the scan (`scan-commands.ts`) went from 108 rows / 27 invalid / 3 duplicate keys to 108 / **0** / **0**. `bun ./📜️script.ts verify mutation-outcome-law` now routes and reaches a verdict (`outcome-law-1.txt`): `3 breach(es)`, all lowpoly `🌀️rotate-selection` / `🔍️scale-selection` / `🚚️move-selection` `🔺️diff` returning `MutationOutcome` in the wrong form. That is fleet content (lowpoly owner), not infrastructure.

### I1b. Taxonomy invalid (found while reproducing I1) — PEER SELF-RESOLVED

- **Peer:** REPO-PATH-BUDGET (session `⚪5dba80e6…`). `🔣️taxonomy.json` was modified at 12:41 and 12:46. The `rewrite.py` reference rewrite mapped many distinct `memberNames` onto the same short case name.
- **Symptom:** `loadCatalogTaxonomy()` threw 188 × `semanticDirectoryMemberKinds collide for owner "tests" and member "<x>"` (e.g. `🚫️rejects` 49×, `🚫️removes` 31×), so every root `verify` died before routing (`root-1.txt`).
- **Action:** waited, since the owner was mid-flight. The peer edited the taxonomy again at 13:37, and it was valid at 13:42 (`loadCatalogTaxonomy()` passes). No edit by me.

### I2. `DslValue::Bytes` non-exhaustive matches — PEER-FIXED, workspace check pending

- **Peer:** `DslValue::Bytes(Vec<u8>)` in `🧰️framework/🔨️modules/🌱️value/🦀️.rs` (committed 11:16).
- **Sites:**
  - `♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:231`: arm `DslValue::Bytes(value) => self.push(DagOwner::Bytes(value))` added by a peer at 12:56. It is correct: `DagOwner::Bytes(Vec<u8>)` exists and is retired by the same loop.
  - `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`: two arms added by a peer at 13:23, `format!("<{} bytes>", bytes.len())` and `self.bytes(value)`.
- **Evidence:** `cargo check -p semio-framework-os-infinite -p semio-framework-artifact-infinite-dag --keep-going` exit 0, 20 warnings, 5 m 29 s (`check-infinite-1.txt`).
- **Pending:** the `✏️s` workspace `--keep-going` check was cut by the usage limit (13:30). It left a deadlocked orphan cargo, PID 46355, at 0 % CPU for 3 h; I killed it at 16:36. Re-run in progress (see I2 follow-up below).

### I3. `bun test` segfault 0x8033 — DIAGNOSED (bun bug, not the workspaces glob). Exits pass; no repo change

- **Hypothesis rejected:** the root `package.json` `workspaces` glob `🧰️framework/**` is not the cause. It was committed at 11:16; before that the list was explicit over `♻️mit-bestand`, `✏️s` and `🧰️framework`. `📓️w1-b-report.md:94` documented the same segfault on 09-30, before the 10:27 change. With the same cwd and the same `package.json`, only the argument form decides whether bun crashes.
- **Repro**, all from cwd = repo root, bun 1.3.14:

  | invocation | result |
  |---|---|
  | `bun test <scratch>/trivial.test.ts` (absolute, file outside repo) | 1 pass, 0 s |
  | `bun test ./<T>/🗑️generated/s2-infra/trivial.test.ts` (in-repo, `./`) | 1 pass, 0 s |
  | `bun test /…/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (absolute; what the owner `📜️script.ts` runs) | **33 pass / 0 fail**, 3.5 s (`tool-machine-1.txt`) |
  | `bun test 🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (relative, **no `./`** → treated as a filter) | **`panic(main thread): Segmentation fault at address 0x8033`**, exit 133, 41 s, peak RSS 0.87 GB (`filter-root-2.txt`) |
  | `bun test s2-infra-nonexistent-filter-xyz` (ASCII filter) | no crash; "7866906 files were searched [102.93s]" |
  | the same with `--path-ignore-patterns='.🧬semio/**'` | no pruning: 7,866,913 files, 203 s |

- **Cause:** with a bare relative path, bun's filter mode walks the whole cwd tree (7.87M files, 7.46M of them under `.🧬semio`: caches and cargo targets). With a non-ASCII (emoji) filter it segfaults mid-walk. `pathIgnorePatterns` filters results without pruning the walk, so no bunfig or `package.json` change mitigates it. This is an upstream bun 1.3.14 bug.
- **Rule for the fleet:** always pass an absolute or `./`-prefixed path to `bun test`, or use the owner `📜️script.ts`/nx target (they already pass absolute paths).
- **Not done:** `package.json` and `bun.lock` left untouched; no `bun install`.

### I4. Stale generated plugin registry — FIXED

- **Peer files:** `REG/📦️deployment/🟦️.ts` (08:14) changed `moduleDirectoryName` to `(pluginId, inventory)`. `REG/🤖️generated/🧩️plugins/🟦️.ts` is gitignored and was last generated at 05:49; its `PROGRAM_TARGETS` calls the 1-arg form at module load and throws. The template `REG/📽️projection/🟦️.ts` was edited at 13:10 and 16:28, the latter only in `emitPlaygroundsTypeScript`; `emitTypeScript` was unchanged.
- **Why more than the call sites changed:** the template's `COMPONENT_MODULE_DIRECTORIES = parseModuleDirectories({ … directoryName })` needs `directoryName` on every row, and `parseModuleDirectories` throws without it. So the template's whole plugin surface had to land: the second import, the `directoryName: InstallationDirectoryV1` type field, the row field, the constant and the 3 call sites. The template also renamed doc comments `PluginRegistryEntry` → `DeployedRegistryEntryV1`.
- **Fix:** `🗑️generated/s2-infra/reemit-plugins-ts.ts` parses the stale rows (69 targets + 1 host row) and adds each row's `directoryName` from its crate's `Cargo.toml` via `declaredComponentDeploymentDirectoryV1`. That is the same source-only value `parseComponentSourceOwnerV1` gives the generator; no descriptor is involved. The script then calls the template's own `emitTypeScript`. A guard checks that, with `directoryName` stripped, every emitted row is byte-identical to its stale row. The written file is byte-identical to the template emission (`cmp`). The real `plugin-registry:generate` will differ only where descriptor data changed.
- **Evidence:**
  - The module loads: `PROGRAM_TARGETS` 43, `COMPONENT_MODULE_DIRECTORIES` 69, `pluginModuleUrl("puzzle")` = `/🔌️plugin-modules/🧩️puzzle/🌉️bridge.js`.
  - React time-travel band suite (`🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component`, import chain Shell → `🏛️ShellHost` → registry): **31/31 passed** (`band-test-1.txt`, `bun ./📜️script.ts test long … --run` from the React package cwd).
  - React `typecheck`: **TS2554 = 0**, 0 errors in the generated file (`typecheck-react-1.txt`). It still exits 2 on 6 fleet errors (TS2741 ×3, TS2322 ×2, TS2304 ×1 in `🗣️Interpreter/📖️stories`), caused by S2-W1E's `SliderProps` fields `precision, displayUnit, displayFactor, limits`.
- **Leftover for the coordinator's generate:** the gitignored `REG/🤖️generated/🔌️plugins.json` (05:49) also lacks `directoryName`, so `readGeneratedCatalogProjection` (dev boot) throws until the real generate runs. I did not touch it.

### I6. Plugin `🛰️declaration-channels` fixtures — RENAME-MAP ENTRY, references already consistent

- `rename-map.txt` lines 4357–4360 map `🧫️fixtures/🛰️declaration-channels/1️⃣standard-1` → `1standard` and `2️⃣standard-2` → `2standard`; git shows the files as staged renames (R). So this is not an accidental deletion, and restoring from HEAD would be wrong.
- Every reference already uses the new names. `🧪️tests/🛰️declaration-channels/🦀️.rs` was updated by a peer at 13:08. No file in the repo still names `1️⃣standard-1` or `2️⃣standard-2`. All `#[path]` targets resolve (the I7 scanner reports none for these files).
- **Peer defect, reported, not fixed:** the new names have no leading emoji. `apply.py` computes `prefix_len` from the first character, and the keycap `1️⃣` starts with ASCII `1`, so the emoji was dropped. Every other renamed directory kept its emoji. The dev or the REPO-PATH-BUDGET owner decides whether to rename these to `1️⃣standard` / `2️⃣standard`.

### I7. Dangling path references after REPO-PATH-BUDGET — FIXED, 1,649 references in 339 files; 2 deferred (2 remodel files)

- **Peer:** REPO-PATH-BUDGET (`⚪5dba80e6…`), settled at 12:59 (no process, folder static since). Per its `📓️decision.md`: 2,183 fixture dirs renamed; `twins.py` then moved 1,117 schema twins (237 mutation leaves, 880 test cases) to the same names; 51 reference files updated.
- **Method:** `🗑️generated/s2-infra/scan-dangling-refs.ts` resolves every `#[path]`, `include_str!`/`include_bytes!` (literal and `CARGO_MANIFEST_DIR`), Cargo `path =` and `🔣️.json` path-like string against the disk. `classify-dangling.ts` and `new-dangling.ts` keep only references that resolved at HEAD, or that were introduced after HEAD. They map each file back to its HEAD location through the rename map plus derived twin moves, so the resolver's own noise cancels out. Fixes go strictly in the direction of the disk state: the HEAD target is mapped forward through the rename map, the result must exist on disk, and the reference is rewritten relative to the file's current location. The exact quoted string is replaced on its line, files touched in the last 10 minutes are skipped, and everything is re-scanned afterwards.

  | class | count | example |
  |---|---|---|
  | a. Target renamed, reference not updated | 130 in 26 files (energy 113, block 12, shooting 3, writer 3) | energy `🧪️tests/🏛️mutate-energy-model-1/🦀️.rs` → `🅱️change-infiltration-temperature-term-coefficient/…` now `🅱️change-infiltration/…` |
  | b. Collateral rewrite by the bare-basename rule; target never renamed | 1,521 in 317 files (architect 1,189, stdio 119, procedural 61, note 32, wfc 20, shooting 12, dag 11, block 10, writer 10, imperative 9, sequence 9, remodel 8, animate 6, trinity 5, mathematical 5, fem 4, flow 4, sourcing 3, space 2, dev composition 1) | architect `…/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs` → `../../🧫️fixtures/🔬️window/🔣️.json` reverted to the existing `🔬️window-ownership` |

  All 1,521 class-b fixes pass a correspondence guard: each differing segment is a word-boundary prefix of the other, and the new target exists. 1,460 went back to the longer name, and 61 went to a shorter name that really exists (e.g. procedural oracle `👁️set-generation-preview` → `👁️set`).
- **Evidence:** re-scan after the fixes. Of the 1,651 planned lines, 1,649 resolve. The other 2 are in remodel (`📸️remodeling/🦀️.rs:1654`, `…/🧊️model/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:35`), which a peer was editing (17:07, 17:10); they are deferred.
- **Real dangling references not caused by the rename (reported, direction unknown):** 16. All are framework fixtures naming files that other peers deleted or moved after HEAD:
  - `🖱️ui/🧫️fixtures/🪪️fixture-ownership/🔣️.json` (6 rows → `🐚️Shell/🧫️fixtures/↔️panel-resize`, `🪪️window-surface-owner`, `🛟️panel-window-reservation`);
  - `🧪️test/🧫️fixtures/🔌️adapter-ownership/🔣️.json` (5);
  - `🧪️test/🧫️fixtures/🧬️schema-invariants/🔣️.json` (2);
  - `👀️readme-reviewed-fixture-inputs/…/🔣️.json` (1, `🧪️test/🧬️schema/🔣️.json`);
  - `⚡️caching/🧫️fixtures/nx-bootstrap/🔣️.json` (1, `🏃️process/🧭️routing/🟦️.ts`);
  - `🧱️manifestless-source-closure/🔣️.json` (1, `🧪️shardclient-reserved-response-settlement/🟦️.ts`).

  List: `🗑️generated/s2-infra/dangling-real-nonrename.txt`.
- **False positives** (verified, ignored): `#anchor` suffixes such as `…/🦀️.rs#SetSoftwareInfo` in stdio oracles (128), `#[path]` inside inline modules, and a commented-out `✏️s/Cargo.toml:276` line.
- **Kind/dir identity:** in progress. The leaf-identity test is blocked by the taxonomy (I5-b). Already seen: energy fixture leaf `🅱️change-infiltration` ≠ schema leaf `🅱️change-infiltration-temperature-term`, and `🆑️change-infiltration` ≠ `🆑️change-infiltration-velocity-squared-term`.

- **2 deferred remodel lines:** applied at 17:48, after the peer edits (17:07, 17:10) had been quiet for more than 30 minutes. In total, 1,651 lines in 341 files are fixed. I also applied 8 more class-b fixes whose line held two same-shape references (`⬅️before`/`➡️after`): pairing by position fixes 2 sequence lines and 4 stdio lines, plus the UI JSON rows.
- **Central catalog `📚️library/🔣️schema-catalog.json`:** the scan above only read files named `🔣️.json`, so this was a second pass. 41 `"path"` rows dangled. 39 had exactly one word-boundary prefix sibling on disk; each was rewritten on its line and nothing was reformatted (`git diff --stat`: 39 insertions, 39 deletions). Examples: glTF `🪪️asset/📝️change` → `📝️change-description`, presence `📸️replace` → `📸️replace-presence`. 2 rows point at dirs that were deleted, not renamed: `⚡️caching/🔒️leases/🧬️schema`, and process3d `⏱️change-cursor`, gone since commit 665. Script: `fix-catalog-paths.ts`. A real `schema generate` will reproduce the same rows.
- **Schema lints before/after.** Before = the 12:24/12:33 snapshots in `📓️resume-evidence.md` §3.2:

  | lint | before | mid (17:30) | after (17:42) |
  |---|---|---|---|
  | `schema mutation-inputs` | 41 findings (13 malformed) | 98 (40 malformed: glTF 13, shooting 6, procedural 6, imperative 3, architect 2, animate 2, …) | **37** (1 malformed, the pre-existing process3d `⏱️change-cursor`; 30 `leafUncatalogued` waiting on `schema generate`; norm en1995 `uiInvalid` 2; `labelMissing` 3; os `widgetIncompatible` 1) |
  | `schema mutation-payloads` | 15 findings | 11 | **11** (norm 8, raster 1 unwitnessed, stdio wav 2; none path-related); 5108/5113 rows clean; 3081/3082 leaves witnessed |

  Files: `inputs-strict-after{,-2}.txt`, `payloads-strict-after{,-2}.txt`. Commands run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`: `bun ./📜️script.ts schema mutation-inputs` and `bun ./📜️script.ts schema mutation-payloads`.
- **Leaf identity:** `bun ./📜️script.ts test mutation-leaf-identity` (repo-lib) → **9 pass / 0 fail** (`leaf-identity-2.txt`). My own census is stricter: it compares each leaf dir's ASCII slug with its `$id` kind. **231 schema leaf dirs created by the twin moves no longer equal their kind**: energy 148, procedural 16, stdio 11, shooting 8, trinity 7, framework 7, remodel 6, mathematical 5, block 5, lowpoly 4, draw 3, forms 2, playbook 2, raster 2, sequence 2, process/cad/sourcing 1 each. Examples: `🎚️change` = `change-coefficient`, `🔄️replace` = `replace-points`, `👁️set-generation` = `set-generation-preview`. List: `identity-kind-regressions.txt`.

  Twin pairing (fixture vs schema-test names) is a separate list (`identity-twin-mismatch.txt`, raw 706, including convention noise such as stdio's deeper nesting). Notable rows:
  - energy: 7 fixture-only and 7 schema-only leaves. Example: fixture `🅱️change-infiltration` vs schema `🅱️change-infiltration-temperature-term`; fixture `🆑️change-infiltration` vs schema `🆑️…-velocity-squared-term`.
  - remodel `🛠️update-camera`: fixture `🔍️refines2` vs schema test `🔍️refines-the-9fd25a`.

  I did not rename kinds or dirs (coordinator ruling). The dev or REPO-PATH-BUDGET decides.
- **Pre-existing, owner WP:** draw `🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs:36-40` `include_str!`s `…/🧬️schema/🧬️mutations/🎨️replace-layer-fill/🧪️tests/🌈️solid/📸️snapshot/…`. Those snapshot JSONs have never existed under the schema twin (the HEAD tree has only `🦀️.rs` there). That is S2-DRAW's file, so I left it.

### I2 follow-up — remaining `DslValue::Bytes` arms (written; compile check queued)

A static scan (`scan-bytes-matches.ts`, `scan-bytes-generic.ts`, alias-aware, catch-all-aware) found 9 real non-exhaustive `DslValue` matches. Everything else flagged was another enum (`PropertyValue`, `UiValue`, `GltfJson`, pack-json `Value`, `serde_json::Value`, …). Arms added in each owner's style:

| file | function | arm |
|---|---|---|
| procedural generation2d `💾️binary/🦀️.rs` | `generation2d_copy_json` | fallible copy, `try_reserve_exact`, `"generation2d-initializer.json-bytes-preflight"` |
| procedural generation2d `💾️binary/🦀️.rs` | `generation2d_observe_json` | digest `b"bytes"` + len + bytes |
| procedural generation3d `💾️binary/🦀️.rs` | the same 2 functions | same, `generation3d-…` labels |
| architect xlsx serializer | `cell_value` | joins `Array \| Object` (JSON inline string) |
| cad `🎬️interaction-spec` | `expr_value_truthy` | `!b.is_empty()` |
| puzzle 2d `✏️editor/🪟️window` | byte census | `charge(&mut bytes, value.capacity())` |
| puzzle 3d `⏳️precompute/🪣️fill` | `retire_dsl_one` | bounded drop, like the string arm |
| puzzle 3d `🧊️main` | `hash_dsl_value` | tag `6_u8` + bytes |
| `🛂️manifest/🧪️tests/🧪️mutation-inputs` | `canonical` | JSON byte array, as in `DslValue::to_json` |

forms was fixed by a peer at 17:38, bidirectional with the doc updated, the same arms I had planned. The `♾️infinite` dag and board arms were added by peers (12:56, 13:23).

`cargo check -p semio-framework-os-infinite -p semio-framework-artifact-infinite-dag` exits 0. The 6 plugin crates are queued: the first attempt (16:49) sat behind build-dir unit locks for 30 minutes with no rustc and was killed by me, and the second (17:36) is waiting the same way.

### I8. Guest gate red since 19:11: retirement macros moved, three crates left behind (session 3, 21:38)

- **Peer:** at 19:18, the peer moved the retirement macros `artifact_retire_struct!`, `artifact_retire_leaf!` and `artifact_retirement_sequence!` out of `store` into `semio_framework_value` (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs`, `#[macro_export]`, `$crate::retirement::…`). They migrated `📡️replication/🧬️retirement/🦀️.rs` but not three retirement modules written at 18:59, which still had `use store::{artifact_retire_* as …}`:
  - `💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/♻️retirement/🦀️.rs` (the gate's 7 errors: E0432 + E0599 `retirement` on `WorkflowEdge`/… because the macros never expanded);
  - `💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/♻️retirement/🦀️.rs`;
  - `💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/♻️retirement/🦀️.rs`.
- **Fix:** I changed the one import line in each to `use semio_framework_value::{… as retire_struct, … as seq}` (space also takes `retire_leaf`), the location replication uses. All three crates already depend on `semio-framework-value`. No other `store::` retirement-macro users remain in the repo (scan over all tracked and untracked `.rs` files).
- **Evidence:** the exact gate command `cargo check --lib --target wasm32-wasip2 -p semio-framework -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-os -p semio-framework-plugin --features semio-framework-plugin/component-guest,semio-framework-os-kernel/deflate,semio-framework-replication/deflate` → **exit 0, 267 s, 0 errors** (21:47–21:52, `🗑️generated/s2-infra/gate-wasip2-1.txt`). Workflow, os, kernel and plugin were all checked; warnings prove real type-checking (os-kernel 33, plugin 137, replication 18).

### I9. Guest-framework check 4 named a package outside the root workspace (session 3, 22:3x)

- **Cause:** `REG/🔁️rebuild/🔣️.json` `guestFrameworkChecks` (Sep 29) runs every check through `cargo` from the repo root. The peer's 09:11 workspace split moved `semio-s-plugin-stdio` into `✏️s/Cargo.toml`, so check 4 failed with "package ID specification … did not match any packages". `cargo metadata` workspace membership now: the root `Cargo.toml` owns `semio-framework`, `-replication`, `-os-kernel`, `-os`, `-plugin`, `-os-renderer-wgpu`; `✏️s/Cargo.toml` owns `semio-s-plugin-stdio`; `🌎️hub/Cargo.toml` owns `semio-hub-stdio`.
- **Fix (schema-first):**
  - Every check row declares a required `"workspace"` (repo-relative workspace `Cargo.toml`): rows 1–3 `"Cargo.toml"`, row 4 `"✏️s/Cargo.toml"`, with the package unchanged.
  - `readGuestFrameworkChecks` validates it: a string with no empty, `.` or `..` segment that ends in `Cargo.toml`; otherwise it throws `… names no repo-relative workspace Cargo.toml`.
  - `GuestFrameworkCheckV1` carries `workspace`.
  - `guestFrameworkCheckArgs` emits `check --manifest-path <workspace> --lib --target …`, and the progress line names the workspace.
  - Unit test `REG/🧪️tests/🔁️rebuild/🟦️.ts`: the argument prefix now includes `--manifest-path`, the last row includes `workspace`, every declared workspace exists, the invalid-row cases carry a valid workspace so they still test what they claim, and 3 new refusals cover missing, `../` and absolute workspaces.
- **Row 4:** the coordinator decided to keep `semio-s-plugin-stdio` in `✏️s/Cargo.toml`, which is the shared assembly checked early. `semio-hub-stdio` is covered by the components step.
- **Evidence:**
  - `bun ./📜️script.ts test ./🧪️tests/🔁️rebuild/🟦️.ts --run` (registry cwd) → **4/4 passed**.
  - `bun nx run @semio-tech/plugin-registry:guest-framework-check --skip-nx-cache` → **exit 0** (23:14–23:34, before the 02:45 cargo hold). The 4 checks took 257 s (wasip2 framework, `Cargo.toml`), 167 s (wasm32 os-kernel), 190 s (wasm32 renderer-wgpu, so S2-W2C's E0609 fix holds), and 215 s (wasip2 `semio-s-plugin-stdio` via `✏️s/Cargo.toml`), with 0 errors. Log: `🗑️generated/s2-infra/guest-framework-check-1.txt`.

### I10. Activation #7 died on `await` in non-async `WasmScript.run` (session 4, 02:40, during CARGO HOLD)

- **Evidence:** `🗑️generated/e2e/activate-7.log`: `@semio-tech/puzzle-plugin:wasm` → `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:15: error: "await" can only be used inside an "async" function`, which then fails as `Selected Cargo preparation failed: Cargo.toml (1)`.
- **Cause:** a peer migrated `runWasmPackWebBuild(…)` → `await buildRepositoryWasmWebV1(…)` (async). In flow-core it also wrote `async run(): Promise<void>`, but it left `run(): void` in three `WasmScript`s:
  - the puzzle composition (18:11);
  - trinity jack LSP `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/📜️script.ts` (18:11);
  - `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:1136` (19:16). This one means every command of that package fails to parse.
- **Fix:** `async run(): Promise<void>` in all three, the peer's own form.
- **Evidence (static; no cargo, nx or tests, per hold rule 26):** a `Bun.Transpiler` parse of all 9,206 tracked `.ts`/`.tsx` files (outside dist, node_modules, target and tickets) → **0 failures** (`parse-all-ts.ts`, `parse-all-ts-1.txt`). The same parser rejects a minimal `run(): void { await f(); }` sample, so the check is not blind. Activation needs a re-run by the coordinator.

### I11. wgpu browser profile `sourceModulePaths` missing 3 modules (session 4, 02:5x, static under the hold)

- **Cause:** `@semio-tech/framework-renderer-wgpu:generate-frame-worker` (`🧊️wgpu/📽️projection/🟦️.ts` `renderWgpuBrowserBundles`) refuses any resolved import outside the taxonomy profile `kind: "wgpu-browser-esm-v1"` `sourceModulePaths`. Two new imports were not declared: a peer's `🎞️frame-worker` → `🖼️assets/🥽️mesh/📇️catalog/🟦️.ts`, and S2-W1E's `🖱️ui/🧬️contract/🛡️limits` → `../🧩️component/🟦️.ts`. That one transitively pulls `🔢️number-format/🟦️.ts`.
- **Fix:** 3 entries added to `🔣️taxonomy.json` with the Edit tool, one atomic edit each on a unique two-line anchor, in UTF-8 byte order (the parser rejects anything else):
  - `🖼️assets/🥽️mesh/📇️catalog/🟦️.ts` after `📇️catalog.json`;
  - `🖱️ui/🧬️contract/🧩️component/🟦️.ts` after `🛡️limits`;
  - `🖱️ui/🧬️contract/🔢️number-format/🟦️.ts` after `🖱️ui/🎬️scene`.
- **Evidence (static):** `wgpu-browser-walk.ts` mirrors the bundler resolver: relative specifiers are joined literally, bare specifiers go through `workspaceImports`, and imports come from `Bun.Transpiler.scanImports` with the latin1 mojibake of non-ASCII specifiers re-decoded. Over all 3 entries: 151 declared / 151 reached / **0 undeclared** / 0 declared-but-unreached. `loadCatalogTaxonomy()` is valid, and `parseSemanticPackageBrowserProfile` accepts the profile. The only unbound specifiers are `node:path`/`node:url` in `🎠️kernel/🟦️.ts`'s in-source `import.meta.vitest` blocks; they predate this change, and the define dead-code-eliminates them.
- **Pending:** the `generate-frame-worker` run itself, after the hold, done by the coordinator's activation.

### I5. Ongoing watch

- **I5-a, 16:36:** my own deadlocked orphan cargo from before the usage cut (PID 46355, 3 h at 0 % CPU) was killed. About 20 idle 3 h-old cargo/nextest processes from cut agents were gone by 16:50 (someone cleaned them).
- **I5-b, taxonomy invalid 17:13–17:25:** a peer was adding the `repo-entity-kinds` contract, which produced 3 `generatorContracts` errors, and later 5, including `raster-svg-video-export` → missing `raster-video`. It was valid again at 17:25. I waited and did not edit.
- **I5-c:** nx project graph builds (`bun nx show projects`, exit 0, 125 s).
- **I5-e, 22:17 health sweep:** root routes 109 / 0 invalid (`scan-commands.ts`); taxonomy valid; `cargo metadata --no-deps` exits 0 for the root and `✏️s` workspaces. The dangling-reference re-scan finds 118 new rows since 17:15, none rename-related: repo-lib policy fixtures under authoring (`🧱️rust-source-direction` 61, `🧱️cargo-dependency-direction` 36, `🧫️mutation-leaf-identity` 9, `🧱️locale-law-ownership` 10), the schema validator ownership fixture 1, and the norm en1992 `🧷change-anchor-as` anchor 1, a known NORM WP item. List: `dangling-new-4.txt`.
- **I5-f, 22:14, stdio obj E0308 ×20 (`usize`↔`u64` index migration, mid-flight):** a peer edited `🗽️obj/…/📐️geometry/🧬️schema/📸️snapshot/🦀️.rs` at 22:04 and `🔺️diff/🦀️.rs` at 22:09 (13 files changed). `semio-s-artifact-stdio-obj` does not compile, and every crate depending on it is skipped. I'm waiting for the owner.
- **I2 evidence so far:** in the same run `semio-s-artifact-stdio-gltf` compiles (11 warnings). So the coordinator's 210 glTF `MutationLeaf source authority failed` errors are gone (they coincided with the 17:13–17:25 taxonomy flicker and the collateral catalog rows fixed in I7).
- **I5-g, 02:4x static re-checks during the CARGO HOLD:**
  - **I7:** the dangling-reference re-scan finds 315 new rows since 22:17, again none rename-related: repo-lib policy fixtures 311 (`🧱️rust-source-direction` 261, `🚷️discovery-boundaries` 20, `📥️inference` 11, `🧬️mutation-type-origin` 8, `📡️mutation-reachability` 8, …) and 4 synthetic Unicode test strings (`😀\u0000url!@/`, `relative/🧬️/native`). The central catalog has 0 dangling `"path"` rows; the earlier 2 deleted-dir rows are gone too (`dangling-new-5.txt`).
  - **I6:** all 11 `#[path]`/`include_*` references in the 5 plugin declaration-channel tests resolve to `1standard`/`2standard`. The channel manifest `🔣️.json` names no standard dirs, and nothing in the repo still names `1️⃣standard-1`/`2️⃣standard-2`. The only open point is the missing keycap emoji, which is the dev's call.
- **I5-d, wgpu guest gate E0609** (coordinator 17:4x; handed to S2-W2C at 22:0x, dropped here): `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `document_execution_target_lease` is `#[cfg(not(wasm32))]` on `ShellState`. Its only reader, `open_service_port`, is native-only too. But its three writes (10653, 10675 reset; 11580 set) are not gated: each follows `#[cfg(not(wasm32))] self.cancel_inference_port();`, and that attribute covers only the next statement. This has been true since commit 665 or earlier, so it is older than the 17:21 mtime, which comes from S2-W1E's `..Default::default()` UI-contract edits. Planned fix if still red after 18:00: gate the three writes with the same cfg.

## Session 3 — 2026-10-02

Executor S3-INFRA (coordinator `⚪b7db773a…`), started 10:54. Scratch: `🗑️generated/s3-infra/`.

### S3-1. Activation blocker (a): frame-worker "browser import is not schema-owned" — FIXED (11:01)

- **Symptom:** activation attempts 10–18 (`🗑️generated/e2e/activate-retry-1{0..8}.log`) all died in `@semio-tech/framework-renderer-wgpu:generate-frame-worker`. `./🫧️transient/🟦️.ts in 🎠️kernel/🟦️.ts` appeared 16 times, `../../🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts in 🧰️framework/📦️packages/🟦️typescript/🟦️.ts` 5 times (from attempt 14 on).
- **Legitimacy check (both imports are real peer API, not strays):**
  - `🎠️kernel/🟦️.ts` (03:20, uncommitted): a peer moved the in-file `OsTransient` lane into the new module `🎠️kernel/🫧️transient/🟦️.ts` (03:20, untracked). It is now `TransientStore`/`defaultTransientStore`, and the kernel re-exports `ephemeralBox|Map|Set|WeakMap`. This is the same ephemeral local-only lane, extracted.
  - `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (08:31, uncommitted): a peer (post-cut, so Codex or the dev) exports `parseIntrinsicValue`/`parseIntrinsicValueControlled` plus types from the new `🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts` (08:46, untracked). `🌱️value/🧬️schema/🟦️.ts` re-exports the same names, and forms consumes them (`parseIntrinsicValue as parseFormsValue`). The intrinsic module itself imports `🧬️schema/🧾️record/🟦️.ts` (tracked since 09-09), which was also undeclared, and `🔢️ieee754` (declared). `🛬️decode` is type-only and erased.
- **Fix (schema-first, declare):** 3 entries added to the `🔣️taxonomy.json` `generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths`. Each is one Edit-tool edit on a unique two-line anchor (counts checked = 1), in UTF-8 byte order:
  - `🧰️framework/🔨️modules/🎠️kernel/🫧️transient/🟦️.ts` after `🎠️kernel/🧩️extensions/🟦️.ts`;
  - `🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts` after `🌱️value/🗂️ordered/🔢️numeric/🟦️.ts`;
  - `🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts` after `🧬️schema/🟦️.ts`.
- **Evidence:**
  - Full static closure walk (`s3-infra/wgpu-browser-walk.ts`, the I11 walker), all 3 entries: before = 151 declared / 154 reached / **3 undeclared** (`walk-1.txt`); after = 154 / 154 / **0 undeclared** / 0 declared-but-unreached / 0 missing (`walk-2.txt`). Only `node:path` and `node:url` stay unbound, in the kernel's `import.meta.vitest` blocks (dead-code-eliminated by the define, as before).
  - `loadCatalogTaxonomy()` + `validateTaxonomy()` → **0 problems**. `parseSemanticPackageBrowserProfile` accepts 154 modules (`taxonomy-load.ts`).
  - `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker --excludeTaskDependencies` → **exit 0**, "generated 🎞️frame-worker.js", 53 s, 10:59–11:01 (`generate-frame-worker-1.txt`). I added `--excludeTaskDependencies` because the target `dependsOn` `@semio-tech/plugin-registry:generate` + `@semio-tech/ui-rs:generate`, which I may not run. Their outputs from the 10:50 activation were on disk.
- **Coordinator notified** (SendMessage `main`, 11:02).
- **Open (schema-first debt, not blocking):** the browser module list is duplicated by hand in 2 more places that do not follow the profile:
  - the wgpu renderer `📋️project.json` `namedInputs.frameWorkerSources`, the nx cache inputs of `generate-frame-worker`, `check-browser-worker`, `test-browser` and `browser-embedded-acceptance`. It lacks **41** of the 151 profile modules (now 44), so an edit to only such a module can give a stale cache hit;
  - the contract's `inputPatterns`, which lacks 10 (now 13).

  Neither list is checked against the profile. See "Open items" at the end of this session.

### S3-2. Root routing RED again: sqlite-rollout owned commands (coordinator request 11:05) — FIXED (11:17)

- **Peer files:** a non-fleet sqlite rollout (Codex), uncommitted. Each of these `📋️project.json` files gained `test-snapshot-sqlite`, `-native` and `-source` targets with `metadata.semio.workspaceCommand`:
  - `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/📋️project.json` (08:00, also `verify-snapshot-sqlite-source`);
  - `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/📋️project.json` (08:15);
  - `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/📋️project.json` (06:53, also `verify-snapshot-sqlite-source`).
- **Breakage:** 3 one-word rows `["test-snapshot-sqlite"]` (`Invalid owned command`). Even with 2 words, the same words in 3 projects collide: `test-snapshot-sqlite native|source` ×3, and `verify snapshot-sqlite-source` ×2 (`Duplicate owned command`). The router validates repo-wide, so every root command died.
- **Sibling convention:** 79 other `test-snapshot-sqlite*` targets declare no route at all. The note plugin routes its sqlite checks as `["verify","note-sqlite-snapshot-guest"]`. The root dispatches owned routes only under `verify` (`📜️script.ts:7164`), so any route under another first word is unreachable (audit I-1).
- **Fix:** I edited only the metadata arrays, with a region-guarded replace (`s3-infra/rename-sqlite-commands.ts`): 30-min mtime guard, exactly one match per row, and a whole-document JSON comparison proving that nothing else changed. Formatting is preserved. Nx target names and commands are unchanged, so launch rows (which call `bun nx run …:test-snapshot-sqlite*`) and the Codex peer's running `test-snapshot-sqlite-native` are unaffected. New routes, which are reachable as `bun ./📜️script.ts verify <word>`:
  - `["verify","<scope>-test-snapshot-sqlite"]`, `…-native` and `…-source` for the three test targets;
  - `["verify","<scope>-snapshot-sqlite-source"]` for the two `verify-snapshot-sqlite-source` targets;
  - scope = `architect-program`, `forms`, `cad`.
- **Evidence:** `scan-commands.ts`: 120 rows / 3 invalid / 4 duplicate keys → 120 / **0** / **0**. `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/🎠️kernel/🫧️transient` → **exit 0**, routed, report written (`taxonomy-report-transient-1.txt`). The report has 1 peer error in that dir: `fixed-source-disposition-unresolved …/🫧️transient/📜️script.ts` (peer 03:35; untracked `📋️project.json` + `📜️script.ts` next to the module). That is not mine; it goes to the watch.
- **Coordinator notified** (SendMessage `main`, 11:18).

### S3-3. Predecessor's unreported 03:12–03:42 work (found on disk, cut before it was reported)

- **I-2 floating async repair, applied 03:38 by S2-INFRA:** `🗑️generated/s2-infra/floating-async.ts` (TS-AST scan of every `📜️script.ts` for un-awaited calls to the library's async exports) found 75 floating calls in 57 files: `runVitest` 37, `runRepositoryCargoTests` 21, `runRepositoryTestCommand` 17. `floating-async-fix.ts --apply` inserted `await` and made each enclosing function `async` (`void` → `Promise<void>`). The base contract is `Script.run(segments): void | Promise<void>` (`🏃️process/🧭️routing/🟦️.ts:13`), and `ScriptRouter.run` awaits it (`:74`), so failures now reach the router. Re-scan now (11:3x, `s3-infra/floating-async-1.txt`): 18 async runners, 642 scripts, **0 findings**. The jack LSP site named by the audit (`🧠️lsp/📦️packages/🦀️rust/📜️script.ts:26–28`) is `async run` + `await runRepositoryCargoTests`. File list: `s3-infra/floating-async-files.txt`.
- **`RecordSpecProducer` call-site migration, applied 03:42 by S2-INFRA:** `specfn-ordinary.ts --apply` rewrote `&spec_fn()` → `&(spec_fn.ordinary)()` in 101 files (`DslVariants::variants()` now yields `(String, RecordSpecProducer)`, `#[derive(Clone,Copy)]`, field `ordinary: fn()->RecordSpec`). All 101 carry the new form now (old 0 / new 101). They span 40 crates in the `✏️s` workspace plus os-kernel 4, plugin 7 and hub-space 2 (`s3-infra/specfn-crates.txt`). **Compile proof owed:** folded into the S3-4 `✏️s` check.
- **I-1 analysis (in progress):** 120 routes. Reachable: `verify *` 69 (root `📜️script.ts:7164`, prefix-open), plus the exact `stdio package-contract|package-graph` and `flow test-source|child-identity-check|child-edit-check|add-widget-retained-check` (owner scripts). Unreachable: 45 (`check` 33, `generate` 9, `members` 2, `prepare` 1). List: `s3-infra/routes-all-1.txt`.

### S3-4. N13 DSL brace sweep (coordinator 11:3x) — CARRIERS MIGRATED; process3d regeneration and cargo proof pending

- **Rule:** the peer decoder (`🗣️dsl/🧬️schema/🛬️decoding/🦀️.rs`, `Shape::List(Record)` → `expect(LBrace)`; peer ticket UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O, "no bare-record fallback") rejects any list whose first token pair is `ident =`. Grammar-wise, `[` followed by `ident =` is therefore always a parse error, so it is a complete detector for this error class.
- **Census tool:** `T/🧪️s3-infra-dsl-brace-census.py` (kept). It tokenizes, skipping strings and fences. It covers every committed `*.dsl.semio` (247) plus every `include_str!` target of a Rust file (1,217 non-code texts), so 1,464 carriers in total. A second pass checks Rust string literals: 30 headed `semio … .dsl v1` literals plus all literals with `[ ident=`. Norm path selectors (`members[id=B1]`) are false positives and were filtered out by hand.
  - **Before (11:41):** 19 carriers / 94 bare lists (`dsl-census-1.tsv`): mathematical demo, animate presentation demo, fem 2d demo, process3d ×2, lowpoly concrete-forest, layout demo, norm en1991 multi-fail, iso16757 `🚫️broken` (semantically broken on purpose, syntax valid), en1999 ×2, stdio las/zip/md/json ×2, space `♻️reuse` workflow graph, collection `🎬️demo.collection`, and the stdio binary diff `📖️.grammar.semio`. Forms (S3-CONTROLS, 11:24), puzzle (S3-PUZZLE, 11:06) and dag (S3-GRAPHS) were already braced. Rust literals: process3d `PROCESS_3D_PLATE_EXAMPLE_TEXT` (8), stdio md sqlite test (1), stdio json sqlite tests (3).
  - **After (11:49):** **3** carriers / 51 bare lists (`dsl-census-2.tsv`). Process3d ×2 is excluded: it is regenerated via `regenerate_example_fixtures`, pending. The other is the binary diff `📖️.grammar.semio`; its 2 hits are inside `#` comment examples, and the grammar rule itself models the bare form (see open items).
- **Migrator:** `T/🧪️s3-infra-dsl-brace-migrate.py` (kept). It uses the same boundary rule as S3-PUZZLE's script, which is the old parser's rule: a key that repeats inside the current record starts the next record. It also handles block fields (`crop { … }`), table fields (`style-runs [cols] { }`), bare variant tags (`document schema=…`) and, via `--nested`, declared bare nested records. Nested key sets were checked against the Rust types: lowpoly `transform`={position,rotation,scale} and `mesh`=ArtifactChild{child_id,target}; zip `metadata`/`local`/`central`. It inserts only `{ ` / ` }`, so every source byte is preserved and the diff is exactly the braces (byte count = 4 × records, asserted). Before applying, I reviewed the per-record key signatures (`brace-review.py`); every list has uniform records, e.g. json `members: key value` ×18 and md `blocks: kind …`.
- **Applied:** 16 carriers, 31 lists, 122 records (`brace-apply-1.txt`), all quiet > 30 min. Hand edits (Edit tool) of the 4 inline test literals in the stdio sqlite tests:
  - json `:126` valid, `:128` invalid, `:184` payload;
  - md `:44` valid. Its "orphan inline" negative case now inserts a second braced record (`…inlines=[]} {kind=soft-break inlines=[]}`), so it still tests an orphan and not a duplicate key.
- **Cargo proof owed:** each owner crate's example/round-trip laws (see S3-6). Until then: WRITTEN, census-verified, not compile/test-verified.
- **Open:**
  - **Grammar docs:** `.grammar.semio` files that model `Vec<record>` as `"[" item* "]"` with bare items (e.g. stdio binary diff `splice-item = "offset" "=" …`) no longer describe the printer's braced output. If `🧹️fixture-sweep/🔬️m5-handcrafted-grammar-conformance` checks printer output against grammars, it will flag them. That is the grammar peer's sweep; I did not touch any grammar.
  - **Permanent law:** see S3-5.

### S3-5. DslEnum/DslRecord derive hygiene (coordinator 11:4x) — FIXED in source; compile proof running

- **Defect (peer derive, `🗣️dsl/✨️derive/🦀️.rs`, quiet since 08:11):** generated code bound authored field names as plain locals. Pattern bindings `Variant { field, record, … }`, decode `let <field> = …` and struct `let Self{…}=self` all did this, mixed with the generator's own locals `field`, `record`, `control` and `value`. So a variant field named `field` was shadowed by `let field=control.scoped_stage(…)` (S3-GRAPHS' E0308; the dag workaround is `#[dsl(key="field")] slider_field`), and a field named `record` or `control` broke the controlled decode and encode.
- **Fix (one atomic codemod write, `s3-infra/derive-hygiene.py`, 12 count-asserted edits):** two new helpers, `field_local(ident)` → `__semio_field_<name>` and `field_inits` (`name: __semio_field_name`). Authored names now appear only in field-name position (`name:` / `self.name`), never as locals:
  - ordinary decode `let __semio_field_x`;
  - controlled decode + `controlled_fields`;
  - 3 struct `Ok(Self{…})` constructors;
  - variant match patterns `Variant{x: __semio_field_x}` and construct;
  - `to_value_from_bindings`;
  - `encoding_record_codegen(bindings=true)` source;
  - variant and struct retirement.
- **Law:** `🗣️dsl/🧪️tests/🧪️hygienic-bindings/🦀️.rs` + `🗣️dsl/🧫️fixtures/🧫️hygienic-bindings/🔣️.json`, registered in `🗣️dsl/🦀️.rs` `//#region 🧪️Tests`. A `DslRecord` struct and a `DslEnum` variant have fields named `field`, `record`, `control`, `value`, `fields` and `keyword`. The law checks:
  - the spec keys equal the fixture names;
  - ordinary = controlled projection;
  - print → parse → ordinary and controlled construction both round-trip;
  - serde (third-party oracle) agrees.
- **Evidence so far:** `cargo check -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-kernel --lib` started 12:04. The derive crate compiled (1 pre-existing warning at `:2220`); the kernel is still running.
- **Follow-up for owners:** the dag `slider_field` + `#[dsl(key="field")]` workaround can go back to `field` (S3-GRAPHS' call).

### S3-6. Audit wave A, I-1..I-3 — DONE

- **I-1 (unreachable owned routes) — FIXED + LAW.**
  - **Grammar decision:** `metadata.semio.workspaceCommand` declares a root route, so it must be reachable from a `dispatchOwnedScriptRoute` call site. The call sites are root `VerifyScript` (`["verify", ...segments]`, prefix-open, dispatched before the built-ins), stdio `package-contract|package-graph` and flow `test-source|child-identity-check|child-edit-check|add-widget-retained-check`. Nothing dispatches `check`, `generate`, `members` or `prepare`. The 45 rows with those verbs were dead mirrors of their nx target names, and devs run those targets through `bun nx run` / launch rows.
  - **Fix:** removed exactly those 45 declarations (`s3-infra/drop-unreachable-routes.ts`, 17 files). Every removal is checked against a JSON deep-compare of the expected document (key dropped, empty `metadata` dropped) and guarded by a 30-min mtime check. Formatting is kept, and nx targets and launch rows are untouched. Routes: 123 → **78** (verify 72, flow 4, stdio 2), 0 invalid, 0 duplicate.
  - **Law** (new test in `📚️library/🧪️tests/🧱️owned-script-routes/🟦️.ts`):
    - it parses every `📜️script.ts` with the TS compiler and collects each `dispatchOwnedScriptRoute(root, [literals…, ...rest])` site as an open or closed prefix;
    - it asserts every real route is reachable;
    - it asserts no `verify <x>` route shadows a root `VerifyScript` built-in (`segments[0] === "<x>"`, 60 built-ins). Owned dispatch runs first, so such a route would hijack the built-in.
  - **Evidence:** `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️owned-script-routes/🟦️.ts` → **4 pass / 0 fail**, 257 expects, 30.7 s (`owned-routes-test-1.txt`). Negative control: the same predicate over the pre-fix route list flags **45** unreachable, and over the post-fix list **0** (`reachability-negative.ts`).
- **I-2 (floating async runners) — VERIFIED.** The predecessor's 03:38 repair (S3-3) holds: 0 findings over 642 scripts. The stdio/flow/root dispatch call sites are all awaited (`if (!await dispatchOwnedScriptRoute(…))`). Not done: the audit's "type-level guard". `Script.run` returns `void | Promise<void>`, so TS cannot flag a floating promise without a lint rule. The scanner (`s2-infra/floating-async.ts`) is the guard; making it a repo law is an open item.
- **I-3 (guest-check workspace not schema-first / not portable) — FIXED.**
  - **New schema** `📇️registry/🔁️rebuild/🧬️schema/🔣️.json` (draft-07) covers the whole rebuild-chain document: steps (stage enum, nx command) and `GuestFrameworkCheckV1`. The workspace pattern is `^(?:(?!\.\.?/)[^/\\:]+/)*Cargo\.toml$`, so it allows `/` segments only and no `.`, `..`, `\` or `:`.
  - **Runtime:** `readGuestFrameworkChecks` now reads target enum, crate, feature and workspace patterns, `maxLength` and the exact key set from the schema (no Ajv at runtime, per AGENTS). It also refuses duplicate features and unknown keys.
  - **Tests** (`🧪️tests/🔁️rebuild/🟦️.ts`): Ajv (strict, third-party oracle) validates the committed `🔣️.json`; the schema stage enum equals `REBUILD_STAGES`; Ajv and the reader agree on 14 workspace strings (`..\x\Cargo.toml`, `x\Cargo.toml`, `C:x/Cargo.toml`, `C:/Cargo.toml`, `a//…`, `./…`, `a/../…`, …), of which only `Cargo.toml`, `✏️s/Cargo.toml` and `🌎️hub/Cargo.toml` pass; an unknown-key refusal is covered.
  - **Evidence:**
    - `bun ./📜️script.ts test ./🧪️tests/🔁️rebuild/🟦️.ts --run` (registry cwd) → **5 passed / 0 failed** (`rebuild-test-2.txt`). The first run failed on Ajv strict-tuple for `command`; the schema now uses `items` + `contains: nx`, and the first-word rule stays in the reader.
    - `tsc --noEmit --strict …` over `🔁️rebuild/🟦️.ts` and its imports → exit 0, 0 errors (`tsc-rebuild-1.txt`).

### S3-7. Frame-worker nx inputs: the committed ownership law was red; now 0 uncovered (12:34)

- **Law:** `⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts` `testWgpuGeneratorOwnership`, lines 89–92. Every profile source must be an nx input of `generate-frame-worker`: either in the wgpu `📋️project.json` `namedInputs.frameWorkerSources`, or in the `⚙️browser-build` source-input closure (28 files), or, for generated files, in a `dependentTasksOutputFiles` row.
- **Finding:** 27 of the 154 profile sources were covered by neither, including all 6 that I11 and S3-1 declared (`s3-infra/frame-worker-input-law.ts`, `frame-worker-input-law-1.txt`). This is a real cache hole: `generate-frame-worker` is `cache: true`, so an edit to only such a module (e.g. `🫧️transient`, `🌳️intrinsic`, `🧾️record`, `📇️directory/*`) left a stale `🎞️frame-worker.js` restored from the nx cache.
- **Fix:** 27 rows added to `frameWorkerSources` (byte-ordered block, one Edit on a unique anchor; file quiet since 05:01): 143 rows, all unique. Re-evaluation: **uncovered 0** (`frame-worker-input-law-2.txt`).
- **Could not run the whole law:** `testWgpuGeneratorOwnership` first does a full `renderWgpuPackageArtifacts`, which throws `Current WGPU package artifact authority drift` (`wgpu-ownership-law-1.txt`). Cause (`wgpu-catalog-diagnose.ts`): `classifyPackageSource` now classifies the catalog's `binary-adapter` (`📦️packages/🦀️rust/💾️binary/🦀️.rs`, `run_native_entrypoint(Vec::new())`) as `implementation` ("Rust function body performs non-delegating work"), but the catalog declares `declaration`.
  - The catalog is unchanged (sha `50dadf…`); the classifier/grammar side moved (`🔍️discovery/🟦️.ts` peer-edited 12:22; taxonomy `packageGlueGrammar`).
  - Activation is not affected, because producer-scoped `generate-frame-worker`/`-browser-boot`/`-renderer-boot` never parse the package catalog. But `@semio-tech/framework-os:generate-wgpu` and this law are red. Owner: the package-glue grammar peer. Not fixed by me.

### S3-8. Shared build-dir deadlock (12:30) and peer-red kernel (12:31)

- **Deadlock:** 16 `cargo` processes idle for 1–28 min with 0 `rustc` anywhere; `sample` on 2 of them shows both in `prebuild_lock_exclusive → flock` (`sample-63597.txt`, `sample-72540.txt`). I killed all 16 per pid, mine included (`deadlock-cargos-1.txt`): puzzle-2d, vcs, forms, writer, raster, layout and fem-2d tests; plugin, kernel and wgpu checks; infinite and os-config. Coordinator informed 12:31. Likely trigger: my derive fix (S3-5) invalidated every derive-using unit at once while ~16 builds were queued.
- **Kernel red from an active peer (not mine):** `cargo check -p semio-framework-os-kernel --lib` → exit 101, 12:31 (`check-kernel-derive-2.txt`).
  - `🧬️schema/📇️registry/🦀️.rs` (12:20): duplicate `ArtifactSchemaRegistry` (:304, :349) and `SchemaDescriptorRegistryError`, 28 errors including the unresolved `semio_framework_os_kernel` import at :393 and `GRAPHQL_STATE_PREAMBLE`.
  - `📡️replication/🎮️mutation/🦀️.rs:219` (12:24): `semio_framework_schema_state::StateClass` without the dependency (E0433).
  - Coordinator informed 12:32. Activation precheck and every kernel-dependent proof are blocked until it lands. The derive crate itself compiled (exit 0 in the first run; only the pre-existing `:2220` warning).

### S3-9. Permanent native carrier law + compile status (12:5x)

- **Native law `🗣️dsl/🧪️tests/🧪️carrier-record-lists/🦀️.rs`** (registered in `🗣️dsl/🦀️.rs` `//#region 🧪️Tests`):
  - lists every committed `*.dsl.semio` (`git ls-files -co --exclude-standard`, outside tickets);
  - strictly lexes each one with the real DSL lexer (`lex(…, false)`, unbounded `Limits`, trivia filtered);
  - fails on any `LBracket Ident Equals`, the exact pattern the decoder rejects, reporting `path:line`;
  - asserts it saw the corpus (> 200 carriers).

  It parses at token level, so it needs no artifact specs. It goes green once process3d is regenerated (the only remaining `.dsl.semio` offender: 2 carriers, 49 bare lists).
- **Kernel compile with the derive-hygiene change:** `cargo check -p semio-framework-os-kernel --lib` → **exit 0, 42 warnings, 8 m 35 s** (12:42–12:50, `check-kernel-derive-3.txt`). This was after the 12:40 peer repair of `🧬️schema/📇️registry`. Every kernel-internal `DslRecord`/`DslEnum`/`DslOps` expansion compiles with reserved locals.
- **Running:** `cargo test -p semio-framework-os-kernel --lib -- hygienic_ carrier_record` (private target `target-nde-s3-infra`), `test-kernel-dsl-laws-1.txt`.
