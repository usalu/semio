# Result: Hub `📜️script.ts` split into domain modules

Slice HUB-SCRIPT-SPLIT (prefix `hss`), Sonnet executor, 2026-10-16. `F` = this folder (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`). `H` = `🌎️hub`. Target: `H/📦️packages/🦀️rust/📜️script.ts`. Tools, logs and the original copy: `F/🗑️generated/hss-*` (tools in `hss-tools/`, original in `hss-original/`).

## Verdict

DONE. `📜️script.ts` went from 17,556 to 80 lines. It keeps the CLI parse and dispatch: shebang, docstring, three imports, the `router` and the `import.meta.main` entry. All command logic lives in 36 plain `🟦️.ts` modules (largest 1,074 lines, sum 18,107) under the hub's owning folders, plus one hand-written reader-support module. Nothing imports `📜️script.ts` any more: `🚀️local-bootstrap/👷️dev-owner`, `🏗️bootstrap/🎭️actors` and four hub tests import the owning module, so the old cycle `dev-owner` <-> `script.ts` is gone. Evidence (section 7): the CLI list and usage are byte-identical, 79 old-versus-new command runs (47 argument and refusal paths, 32 source-phase runs) have identical output and exit code, `tsc` is clean before and after (0 diagnostics), the bun test pass/fail set of the six reader suites is identical, the declaration set is lossless (442 of 442), the module graph has no cycle, and no non-test module imports a test module. Caveats: section 9.

## 1. What was split

`📜️script.ts` (live text of 14:03, SHA-1 `6b7c8386bb48`, 17,556 lines, 1.28 MB) held 541 top-level statements: 98 imports, 69 classes (68 command classes registered with the router plus the helper class `TrustedBootstrapPublicationUnconfirmed`), 279 functions, 60 type aliases, 34 constants, the `router` and the entry `if`. Most of it is the proof drivers (`prove*Fixture`, `prove*Process`) of the 71 hub gates, the local relay servers, the MCP child-process helpers, the trusted bootstrap materialization and publication (about 3,200 lines) and the GIS map process driver.

Now the script registers every command with `ScriptRouter.registerLazy(name, async () => (await import(<module>)).<Class>)`: the same 71 names in the same order, so `usage()` is unchanged. 48 routes go to a `🧪️tests/<x>` law module, 22 to `🎛️commands`, 1 to `👷️dev-owner` (`SocketGrantCheckScript` and `LocalRelayRoutingScript` stay in their existing owners and are counted among the 48). Lazy loading is what keeps the script free of test imports (the old script statically imported `SocketGrantCheckScript`, `LocalRelayRoutingScript`, `proveGisComponentColdMapPatch` and others from test folders), and a broken module now fails only the commands that load it (seen once, section 7).

## 2. Baseline (before the first edit)

- CLI. `bun ./📜️script.ts` alone fails at the entry with `Original parent Script process invocation required`. Every run therefore goes through `hss-tools/envrun.ts`, which builds the same process envelope the launcher builds (`createScriptProcessEnvelope`, owner `repository:cargo:caller`, 3,600 s) and spawns `bun <script>`. There is no `--help`: it exits 1 with `unknown command "--help"; usage: bun ./📜️script.ts <runtime-fixture-graph|setup|hub-document-sweep|…|secure-local-smoke> [args…]`, the 71 command names in `hss-tools/commands.txt`.
- Type-check: `tsc -p hss-tools/tsconfig.hss.before.json` (the script plus its six importers, 64 s): 0 diagnostics (`hss-tsc.before.log`).
- bun tests that read or import the script (`hss-tests.before.summary`, run as `bun test ./<path>`): foundation-source 13 pass / 3 fail, socket-grant-command-source 6 / 1, provenance cargo tests 17 / 0, browser-host 2 / 1, carried-phase 3 / 0, consumption 24 / 3. The three vitest files (linked-codec-ownership, catalog-selection-preflight, dev `local-hub`) did not start within 8 minutes at fleet load 40 and were killed (`hss-tests.before-attempt1.log`); they are covered by text equivalence (section 7). A first attempt with `bun test <path>` without `./` searched 7.5 million files and matched nothing.
- Existing breakage that is not the split's: the three failing foundation-source tests (stale `consumer-contracts` fixture, credential source order, read-only actors need the original invocation), the failing socket-grant test (native law planning), the browser-host test, the three consumption failures, the `os-dev-composition-ownership` entry `hub-browser-host-staging` (its tokens are gone from the script), the `app-verification-source-ownership` entry for the script (it lists four owners the script does not import) and the dev `local-hub` vitest marker `materializeTrustedCatalogBundle` (renamed long ago). Also: while I worked a peer changed `HubLiveSignInScript` (14:03) and, after the split, one line of `canonical-architecture` in `💡️inference-fixtures` (`proveLocalSessionBrokerContract(this.invocation, control)`); both are theirs.

## 3. Taxonomy tree

Placement rule: production code goes to its owner folder (`🚀️local-relay`, `🚀️local-bootstrap`, `🏗️bootstrap/🧾️provenance`, `🧪️testing`); law drivers and fixtures go to `🧪️tests/<emoji>️<kebab>/🟦️.ts` (the hub's established home of `io-matrix`, `dev-collaboration` and `document-sweep`, which the script already ran lazily); the thin command shells that have no owner go to a new hub-level `🎛️commands`. Every new folder is `<emoji>️<kebab>` with U+FE0F; every emoji is already used as a folder prefix in the repo (`hss-tools/folder-emojis.tsv`, 771 prefixes) and none repeats among the new siblings or the existing ones (the siblings that share an emoji in `🧪️tests`, `🧪️testing`/`🧪️tests` and `🚀️local-*` predate this). The longest new path is 85 UTF-8 bytes (limit 240). Modules at `🌎️hub/<a>/<b>` have the same depth as the script folder, so their relative specifiers and `import.meta.url`-relative paths are unchanged; the five modules under `🧾️provenance` are one level deeper and were re-relativised by the generator (and the one `import.meta.url` use, `trustedCatalogPublicationReceiptContract`, got `"../../../.."`).

| Module | Lines | Holds |
|---|---:|---|
| `H/🧪️testing/📖️source-readers` | 126 | Hub Rust source readers shared by the hub gates: the `os-hub` binary closure, the test package list, the directory backend and trusted-catalog source trees. |
| `H/🎛️commands/🚦️gate-commands` | 255 | Thin hub gate commands: source-gate runners, setup, the cargo test lanes and the hostile-input and CAS checks. |
| `H/🚀️local-relay/🛰️servers` | 498 | The local browser and admin relay servers the live hub gates drive. |
| `H/🚀️local-bootstrap/🧰️mcp-process-support` | 179 | Child-process support for the live hub gates: browser readiness, MCP workspaces, JSON-RPC waits and the admin SPA build. |
| `H/🧪️tests/🪪️check-in-process` | 424 | The check-in process gate: genesis, edit commits and heads over a real MCP workspace child. |
| `H/🧪️tests/🔐️secure-local-smoke` | 620 | The secure local smoke and native document-open gates: credential envelope delivery and the native socket grant actor. |
| `H/🧪️tests/🌐️relay-checks` | 573 | Browser broker, execution target and inference relay gates. |
| `H/🧪️tests/📄️browser-document-open` | 354 | The browser document-open fixture and runtime gate. |
| `H/🧪️tests/🛡️admin-relay-checks` | 457 | Admin relay boundary, backend and live-journey gates. |
| `H/🧪️tests/🔑️directory-authority-checks` | 685 | Directory authority gates: admin authority, share issuance, retained short admin, message authority and scoped sockets. |
| `H/🧪️tests/🗺️open-plan-checks` | 605 | Document open-plan fixtures and the open-plan gates. |
| `H/🧪️tests/📇️native-catalog-checks` | 644 | Native openable catalog provider, headless stdio and catalog selection gates. |
| `H/🧪️tests/🔗️execution-target-lease-checks` | 453 | Execution target lease corpus and the lease gates. |
| `H/🧪️tests/👤️browser-actor-checks` | 484 | Browser actor child worker containment, GIS describe and document reservation gates. |
| `H/🧪️tests/💡️inference-fixtures` | 903 | Inference WAL, catalog, reconcile, approval and command fixtures and the architecture and ledger oracles. |
| `H/🧪️tests/🌍️gis-map-fixtures` | 926 | GIS map approval, retained runtime, checkpoint, proposal and two-author composition fixtures. |
| `H/🧪️tests/🏛️trusted-catalog-fixtures` | 1073 | Trusted catalog and browser actor identity fixtures and the opened-root and identity gates. |
| `H/🧪️tests/📮️trusted-publication-fixtures` | 438 | Trusted bootstrap codec capture, generation stage and publication fixtures. |
| `H/🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection` | 345 | Trusted bootstrap package selection: package specs, profile ids, declared and hosted kinds and the selection preflight. |
| `H/🏗️bootstrap/🧾️provenance/🌍️gis-browser-proof` | 274 | The retained GIS browser actor evidence and the child proof the trusted bootstrap publication runs before it publishes. |
| `H/🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims` | 326 | Trusted bootstrap descriptor claims, dependency closure, encodings and selection preflight. |
| `H/🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle` | 512 | Trusted bootstrap bundle types, codec capture, generation receipts, plugin module records and package selection. |
| `H/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication` | 1074 | Trusted bootstrap materialization, rotation and publication of the current trusted catalog. |
| `H/🧪️tests/🎮️gis-map-process-support` | 447 | GIS map process support: document sockets, MCP clients and checkpoint control shared by the GIS map gates. |
| `H/🧪️tests/🕹️gis-map-process` | 882 | The GIS map process gate: Shell peers and the two-author proposal process. |
| `H/🎛️commands/🏷️trusted-catalog` | 47 | Trusted catalog commands: preflight and bootstrap of the selected packages. |
| `H/🧪️tests/🎁️trusted-stdio-gis-bundle-check` | 222 | The trusted stdio and GIS bundle gate: source, native, process, browser and two-author modes over the published trusted catalog. |
| `H/🎛️commands/🏗️build-and-dev` | 313 | Build, build-dev, publish and dev commands and the local bootstrap launch gate. |
| `H/🧪️tests/📣️directory-live-support` | 112 | Live directory event page and command helpers shared by the directory process gates. |
| `H/🧪️tests/📰️event-page-checks` | 442 | Directory event page route and process gates. |
| `H/🧪️tests/🏠️home-browser-checks` | 775 | Directory Home browser process and scoped presence browser serve gates. |
| `H/🧪️tests/🧾️command-receipt-checks` | 401 | Directory command receipt and authority gates. |
| `H/🧪️tests/🎫️invite-redemption-checks` | 433 | Invite redemption transaction and ordered publication gates. |
| `H/🧪️tests/🛂️space-administration-checks` | 807 | Space administration, public boundary, presence lease and presence normalization gates. |
| `H/🧪️tests/🧬️space-artifact-creation-checks` | 243 | Space artifact creation contract gate. |
| `H/🧪️tests/🧭️space-journey-checks` | 754 | Directory space journey fixture and process gates. |
| `H/🧪️testing/📜️script-sources` | 60 | Hand-written reader support: `HUB_SCRIPT_MODULES`, `readHubScriptModule`, `hubScriptSourceFiles`, `readHubScriptSource`. |
Sizes include the header docstring and the imports.

## 4. Method

1. Parsed the live script with the TypeScript compiler into one unit per top-level statement, with a checker-backed reference graph (`gen.ts` reuses the `rss-tools` idea), and planned the modules by contiguous ranges plus name overrides (`hss-tools/plan.ts`). The plan is keyed by declaration names, not line numbers: `gen.ts` assigns the live units through a frozen snapshot (`plan-snapshot.ts.txt`), so a peer edit inside a body does not invalidate it.
2. The planner reports module cycles (Tarjan) and production-to-test edges. The first plan had two cycles (the trusted bootstrap family, and GIS process against GIS fixtures) and ten production-to-test edges; they were removed by moving `trustedBootstrapReadRegular`, the package selection and the retained GIS browser proof into their own production modules, splitting the GIS process support from the process gate, and moving four small helpers (`documentOpenNeutralObject`, `documentOpenNeutralText`, `liveDirectoryCommandRequestId`, `postLiveDirectoryCommand`) and `TrustedStdioGisBundleCheckScript` to the right side.
3. The generator writes every module from the live text: verbatim unit text, `export` only where another module or the router needs a name, imports rebuilt from the identifiers a module really uses (checker-backed), a header docstring with an emoji unique in the file, `//#region` markers kept. It refuses to apply when the script changed since the dry run.
4. In-memory check before applying: a TypeScript program with the generated tree laid over the real tree (`hss-tools/check.ts`): 0 diagnostics. A seeded type error in one module is reported as the only diagnostic (`hss-check.seeded.log`), so the check is sensitive. Applied once with the sha pin `6b7c8386bb48`, then the six importers were patched.
5. The second plan (the production-to-test fixes above) was generated from the frozen original into a second sandbox and merged into the live files with a three-way `git merge-file` against the first generated tree, because a peer had already edited one module: 13 files replaced, 1 new, 2 merged cleanly (the script and the peer-edited module), 0 conflicts.

## 5. Wrong-direction imports

- Before: the script statically imported test code (`SocketGrantCheckScript` and `LocalRelayRoutingScript` from `🧪️tests` folders, `proveGisComponentColdMapPatch`) and was imported back by production code (`dev-owner`, `actors`).
- Now: `hss-tools/graph-check.ts` over the 37 files: 0 static import cycles, and 0 edges from a non-test module to a `🧪️tests` module (the script itself has no static project import at all). The edges that remain are law modules importing production helpers, which is the allowed direction.

## 6. Readers, importers and registrations repaired

Importers (patched by `hss-tools/patch-importers.ts`, anchored): `🚀️local-bootstrap/👷️dev-owner`, `🏗️bootstrap/🎭️actors` (the namespace import became two named imports from `bootstrap-selection`), and the hub tests `backup-restore`, `document-growth`, `dev-collaboration`, `two-client-document`. Exports needed from outside: `withMaterializedTrustedCatalogBundleV1`, `validateAndPublishTrustedStdioGisCandidate` (publication), `trustedBootstrapSelectPackages`, `trustedBootstrapProfileIdV1` (selection).

Source-text readers:

| Reader | Change |
|---|---|
| the script's own self-reads (`readFileSync(import.meta.path)` twice, `readFileSync(join(repoRoot, "…/📜️script.ts"))` five times) | read the module that now holds the region through `readHubScriptModule` (strips `export `, so the markers keep working); the ten region slices of `TrustedStdioGisBundleCheckScript` use per-module `body(module, start, end)`; the event page route law reads the live-support and event-page modules together |
| `🧪️testing/📜️script-sources` (new, hand-written) | `HUB_SCRIPT_MODULES`, `readHubScriptModule`, `hubScriptSourceFiles`, `readHubScriptSource` (script plus all modules concatenated) |
| `🏗️bootstrap/🧾️provenance/📦️cargo/🧪️tests` (5 sites) | parse `readHubScriptSource(process.cwd())`, so declaration lookups and whole-script call counts (`withMaterializedTrustedCatalogBundleV1` called 4 times, no `retainTrustedCargoInvocationV1`) keep their meaning |
| `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🧪️tests` (2 sites) | read the `🕹️gis-map-process` module |
| `🧪️tests/⛓️linked-codec-ownership`, `🧪️tests/🛫️catalog-selection-preflight` (vitest) | read the `🧭️bootstrap-selection` module |
| `🧪️tests/🎟️carried-phase` | reads the `🚦️gate-commands` module |
| `🧰️…/🧑‍💻dev/🧪️tests/🚀️local-hub` | reads the publication module (its marker was already stale) |
| `🧰️…/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/📥️consumption` | parses `readHubScriptSource` |
| `🧪️tests/🧱️foundation-source`, `🧪️tests/🧱️socket-grant-command-source` | `rootImportsForOwner` unions the imports of all script modules; the router assertions expect `registerLazy` plus the module path; the declared-name scan covers all modules; the fixtures lost five dead imports the old script never used (`LOCAL_BOOTSTRAP_FRAME_MAX`, `LocalFrameReader`, `LocalClientClass`, `verifyAuthenticatedFrame`, `directChildEnvironment`) and `SocketGrantCheckScript` (now lazy) |

Registrations and ledgers:

- Nx inputs. The default input of `H/📦️packages/🦀️rust/📋️project.json` and its two named source inputs gained one brace glob over the 37 module files (the files no longer sit under the project root); the same glob follows the script entry in the library `📦️packages/🟦️typescript/📋️project.json` (5 places) and in the foundation-source and socket-grant fixtures (the tests compare them with `project.json`).
- Consumer ledgers: `plugin-publication-source-ownership` (1 entry became 9), `cargo-transaction-command-source` (1 became 6), `tool-configuration-ownership` (1 became 2), `os-dev-composition-ownership` (`hub-hash` now points at `bootstrap-bundle`), `🔖️channel-version/📇️consumers.json` (both literals are in `trusted-publication-fixtures`), `build-surfaces` `commandSources` (the 36 modules are scanned for forbidden `nx` commands like the script was), the print and languages oracle `reachableFrom` lists (6 `ajv` modules, 1 `typescript` module). Test constants moved with them: consumers 3 to 11, 18 to 23, 22 to 23.
- Docs and comments naming the old locations: hub README, the MCP script docstring, the boot-descriptor comment, two Rust doc comments, one dev schema description.
- Launch: no command was added or renamed, so `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` are unchanged (both parse, as do `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json` and `.devcontainer/devcontainer.json`). `project.json` still calls `bun ./📜️script.ts <command>` for every target.
- Not touched: the `windows-command-paths` entry point list (its closure walk follows literal dynamic imports, so it still reaches the modules), and `hub_module_tree` (a Rust helper that reads Rust module trees, not this script; the script's own Rust readers moved to `📖️source-readers` unchanged).

## 7. Verification

All comparisons are old versus new. `old` is `hss-original/orig-script-cli.ts`, the 14:03 text with absolute imports and the two `import.meta` reads pinned to the real script directory (`mk-orig-cli.ts`); it runs next to the new script through the same envelope. Normalisation strips ANSI, stack frames, source excerpts, durations and the vitest start/duration lines.

| Check | How | Result |
|---|---|---|
| Command list and usage | `--help`, `bogus`, no argument | identical text (71 names, same order), exit 1 |
| Argument and refusal paths | `hss-tools/cli-fast.txt`: 47 invocations (one wrong or extra argument for every command that validates, plus `directory-live-lanes`, `build x`, `build-dev x`, `publish x`) | 47 of 47 identical output and exit code (`hss-dual.fast.summary.log`; an earlier run `hss-dual.fast-run1.summary.log` also 47 of 47) |
| Source-phase runs | `cli-source.txt`: 32 commands that read fixtures and sources (`admin-directory-authority-check` and `directory-ordered-publication-check` pass; `browser-actor-document-reservation-check` runs its vitest; `trusted-catalog-preflight --packages all` prints the same 34-line selection findings) | 32 of 32 identical (`hss-dual.source.summary.log`): 2 pass, 3 run moved logic to the same assertion failure (`directory-message-authority-check`, `open-plan-check`, the preflight findings), 27 stop at the same existing repository defect (16 times `hub schema module … is absent from schema-catalog.json`, 5 missing fixture files, 6 missing roots, Vitest policy, budget or ticket artifacts) |
| Peer breakage | the first fast run happened while a peer had the local-hub schema half edited | the OLD script failed at load for every command, the new one did not (lazy loading); the run is kept as `hss-dual.fast-attempt1-peer-broken.summary.log` and was repeated once the old script loaded |
| Type-check | `tsc -p hss-tools/tsconfig.hss.after.json` (script, 36 modules, `script-sources`, six importers) and `tsconfig.hss.readers.json` (the nine reader tests I edited) | 0 diagnostics in both; before the split the script closure had 0 as well |
| Module load | `hss-tools/load-all.ts` imports the 36 modules and `script-sources` in one process | 37 loaded, 0 failed |
| Module graph | `graph-check.ts` (static imports, 37 files) | 0 cycles, 0 production-to-test edges |
| bun tests, six reader suites | `hss-tests.before.summary` against `hss-tests.after3.summary` | identical (`diff` exit 0): foundation-source 13/3, socket-grant 6/1, provenance cargo 17/0, browser-host 2/1, carried-phase 3/0, consumption 24/3 |
| bun tests, ledger suites | `hss-tests.ledgers.log` (the consumer ledgers I rewrote; no baseline is possible because they read the script) | three suites cannot start in this tree (plugin-publication, app-verification and os-dev-composition read their fixture from `library/🧫️fixtures`, which does not exist), tool-configuration needs the missing package `postcss-load-config`, build-surfaces passes the nx-scan test that now covers the modules (11 pass, 1 unrelated failure), cargo-transaction reaches the six new consumer entries and fails on the same binding as the old script did (`buildRepositoryCargoArtifacts` is missing from the fixture's declarations; its two other failures are unrelated). The new entries were therefore recomputed with the tests' own rule (`ledger-plan.ts`, `fix-ledgers2.ts`) rather than proven by those suites |
| vitest readers | `hss-tools/lifted-equiv.ts`: the three functions the vitest files lift (`trustedBootstrapLinkedUnownedKindsV1`, `trustedBootstrapSelectionFindingsV1`, `trustedBootstrapDescriptorKindsV1`) | transpiled text identical in the old script and in `bootstrap-selection`; the six functions the AST readers look up (`proveGisMapTwoAuthorShellProcess`, `startGisMapShellPeerV1`, `runTrustedPublicationChild`, `publishTrustedBootstrapCurrent`, `withMaterializedTrustedCatalogBundleV1`, `proveCheckInMcpProcess`) are identical too |
| Self-source regions | `hss-tools/regions-equiv.ts`: the ten text regions that `TrustedStdioGisBundleCheckScript` asserts on, taken from the old text with the old markers and from the new modules with the new markers; every one of the 173 string literals in the check is looked up in both | 0 presence differences in the nine regions that map one to one; the tenth (`DevScript`, which used to include three following units) differs only in the literal `"utf8"`, which the check never asserts on that region |
| Docstring emoji | `verify docstrings emoji-unique --files-from` over the 38 files | the only repeat in a header was in `script-sources` (fixed); 14 files report repeats carried verbatim from the original (`hss-docstrings.after.log`) |
| JSON | every edited JSON file parses (`JSON.parse` after each patch); launch and MCP files parse | ok |

## 8. Losslessness

Method: `hss-tools/itemset.ts` (declaration set) and `lossless2.ts` (line multiset); original = `hss-original/script.ts.txt` (SHA-1 `6b7c8386bb48`).

- Items. 442 top-level non-import declarations before and 442 after, 0 lost, 0 gained, 0 duplicated across the 37 files (the `router` and the entry stay in the script; the four helpers of the hand-written `script-sources` are not part of the comparison).
- Lines (`hss-lossless.final.log`). 17,557 original lines against 18,225 in the script and the modules. Only in the original: 154 lines = 57 import lines + 97 others. Only in the new files: 822 = 579 import lines + 37 blank + 206 others. The "others" are explained: the 71 `.register(...)` lines against 71 `.registerLazy(...)` lines, 36 header docstrings of 3 lines (a docstring and a `@see`), the replaced router docstring, the reader lines of section 6 (self-source reads and the `body(...)` calls), the `import.meta.url` path, and the peer's one-line change to `canonical-architecture`. Everything else is byte-identical text moved verbatim.
- Edits inside moved code (all of them): `export ` on the declarations other modules, the router or an importer need; the self-source reads and the ten `body(...)` calls above; `"../../.."` to `"../../../.."` in `trustedCatalogPublicationReceiptContract`; relative specifiers of the five deeper modules.
- Region markers: both original `//#region`/`//#endregion` pairs (`SourceGateRunners`, `SpaceJourney`) are intact inside one module each; no synthetic marker was needed.

## 9. Not done, risks, hand-offs

- Not run: the commands with side effects or long runs (`build`, `build-dev`, `publish`, `dev`, `setup`, `test`, `secure-local-smoke`, `trusted-catalog-bootstrap`, `local-hub-owner`, the native and process phases of the checks); they are covered by the type-check, the load check, the identical argument paths and the text-equivalence checks. The three vitest readers were not run before or after.
- Taxonomy registration is not done (same unresolved class as the siblings, see `RESULT-TAXONOMY-SWEEP.md`). Names to register: `H/🎛️commands/{🚦️gate-commands,🏷️trusted-catalog,🏗️build-and-dev}`, `H/🚀️local-relay/🛰️servers`, `H/🚀️local-bootstrap/🧰️mcp-process-support`, `H/🧪️testing/{📖️source-readers,📜️script-sources}`, `H/🏗️bootstrap/🧾️provenance/{🧭️bootstrap-selection,🌍️gis-browser-proof,🧮️bootstrap-claims,🗃️bootstrap-bundle,🚚️bootstrap-publication}` and the 25 new folders under `H/🧪️tests` listed in section 3 (names as in the table).
- `HUB_SCRIPT_MODULES` is an explicit list; a new module has to be added there (the foundation-source test fails if a listed module lacks a header docstring, and the Nx glob in `project.json` must follow). A scan-based variant was rejected because it needs a recursive walk of `🌎️hub`.
- Concurrent editors. A peer who holds the old 17,556-line script in a buffer and saves it would bring the whole monolith back next to the modules. Two peers already adapted to the new layout (one edited `💡️inference-fixtures` right after the split). The script's SHA-1 at the end of this work is `642aab0f26c8` (80 lines); the last `tsc` over the script, the 36 modules, `script-sources` and the six importers (`hss-tsc.final.log`) printed nothing.
- Stale ledger entries that the split did not create and did not fix: `app-verification-source-ownership` (script entry), `os-dev-composition-ownership` `hub-browser-host-staging`, the dev `local-hub` vitest marker.
- 27 of the 32 source-phase runs stop on existing repository defects before they reach moved code (section 7), so for those the evidence is the type-check, the load check and the text-equivalence checks, not a behavioural run.
- The scratch trees `hss-out` and `hss-out2` (generated modules, 1.4 to 1.7 MB each, deepest path 245 bytes) stay in `🗑️generated`; they are reproducible with `ORIGINAL=<F>/🗑️generated/hss-original/script.ts.txt bun hss-tools/gen.ts <dir>`.

## 10. Tools

`F/🗑️generated/hss-tools/`: `units.ts`, `deps.ts` (analysis), `plan.ts` (module plan), `check-plan.ts` (cycles, sizes), `prod-test-edges.ts`, `gen.ts` (generator: dry run, `APPLY`, `ORIGINAL=` frozen input), `merge-in.sh` (three-way merge into live files), `patch-importers.ts`, `patch-readers.ts`, `patch-contracts.ts`, `patch-inputs.ts` and `glob.ts`, `fix-ledgers.ts`, `fix-ledgers2.ts`, `fix-rootimports.ts`, `ledger-plan.ts`, `ledger-old.ts`, `envrun.ts`, `mk-orig-cli.ts`, `dual-cli.sh`, `cli-fast.txt`, `cli-source.txt`, `run-tests.sh`, `run-ledger-tests.sh`, `test-summary.sh`, `check.ts` (overlay type-check), `load-all.ts`, `graph-check.ts`, `lossless2.ts`, `itemset.ts`, `lifted-equiv.ts`, `regions-equiv.ts`, `paths-check.ts`. Logs: `hss-tsc.*`, `hss-check.*`, `hss-tests.*`, `hss-dual.*`, `hss-lossless.final.log`, `hss-docstrings.after.log`, `hss-callers.*`, `hss-cls/` (a classification run I stopped), `hss-wait.log`.
