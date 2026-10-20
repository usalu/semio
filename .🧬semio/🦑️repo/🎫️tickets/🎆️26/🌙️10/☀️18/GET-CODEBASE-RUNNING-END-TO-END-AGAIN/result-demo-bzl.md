# result-demo-bzl: mit-bestand demonstrator and report/deck on Bazel (2026-10-20, wave 1020c, stage only)

`C` = this ticket. Tools: `C/demo-bzl-tools/`. Static work only: no `bun repo`, no Bazel, no cargo, no repo write outside `C/` (scratch overlay `/private/tmp/dbz`, recreated by `overlay.sh`). **Status: staged, waiting for GO. After GO: apply, re-verify, then report "ready for cold run" and wait for the coordinator's heavy-run slot.**

## 0. Verdict

1. Under the applied adapter the demonstrator chain is a graph of `repo_task` actions; a task is Bazel-cached iff its manifest says `cache: true` with a deterministic decision and declared outputs (`bazelCacheDecisionV1`). Four demonstrator verifiers were uncached only because cache-fix gave them the `driver-owned` class (exact rows in `L/⚡️caching/🔣️policy.json`); they are pure functions of their dependency outputs. Staged: `prepare-prototype`/`prepare-release` cached checks, `activate-prototype` cached union, `activate-lanes-prototype` a process-free `repo:noop`.
2. Found a restore defect for interactive use: a launcher (`bazel run` of `serve`, `dev`, `serve-e2e`) and `prepare-e2e` restore only the snapshots of their DIRECT producers unless their inputs declare `dependentTasksOutputFiles` with `transitive: true` (`bazelActionsV1`: `restore = transitive(task) ? closure(task) : direct(task)`; continuous dependencies are never followed). On a checkout whose store lacks the modules (remote-cache-only machine, wiped `.cache`) the served app would find no plugin modules, lanes or sessions. Staged: transitive restore on those four targets.
3. Found a wrong output declaration: the three report builds (`build-zwischenbericht|forschungsbericht|kompaktbericht`) declared `{projectRoot}/dist/documents/<id>`, but the script publishes the PDFs into the products store slot `.cache/b/p/m/print/<mode>/documents/report/<id>` (`publishPrintArtifact` -> `printArtifactLayoutV1`). A cache hit restored nothing and the `build` aggregate (`complete`) then failed on a clean store. Staged: the slot, checked against the repository's own `printArtifactNxOutputV1` (cross-check equal).
4. The heavy producing steps of the activation (per-component cargo/wasm, describe, jco, wasm-opt, `support`, lane `prepare`, lane `activate` inputs) are owned by `pg-chain` (agreed split below). I touch none of their files.
5. `build` (release site) cannot be a pure action: `prefetchDemonstratorMapTiles` downloads OSM/OpenFreeMap tiles at build time into a store no declared input pins. It stays uncached (reason recorded in the README); a pinned tile snapshot is the follow-up. Not on the e2e path.
6. Hermetic TeX decision (section 4): Tectonic with per-platform sha256 pins plus a per-file pinned TeX bundle, offline at compile time, no system TeX, no Docker. Already hermetic by construction; verified statically.

## 1. Split with the other executors

| Concern | Owner |
|---|---|
| `component` targets cached with exact inputs, lane `prepare-<lane>` as verdict, `activate-<lane>` owner-root inputs, `support`, wasm fingerprint, descriptor pair, policy/target-policy, os-dev scripts | `pg-chain` (a80e2ba1e2dfdac31); I sent the demonstrator facts (9 lanes, 6 sharing one hub crate and one 28-component closure; existing test `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` asserts the opposite and must be rewritten; owner root is `componentOwnerRootV1` = path minus 3 parts, not `join(crate,'..','..')`) |
| play manifest, `s-dev-composition:generate` inputs, `repo:generator-inputs` | `play-bzl` (its message: manifest decisions win over the exact `driver-owned` rows; same mechanism used here) |
| demonstrator manifest and runtime, report/deck manifests, e2e ladder, TeX decision | this report |

## 2. The chain as Bazel actions (target -> action, cache, restore)

`kind`: task = `repo_task` (no-sandbox, runs in the checkout), noop = elided ordering node, launch = `bazel run`. `restore` = snapshots restored inside the action when it runs. Prototype profile; release is the same with `prepare-release` and `build`.

| # | Target | Kind | Cached after this set | Inputs (exact) | Outputs | Owner |
|---|---|---|---|---|---|---|
| 1 | `workspace:deps-wasm-opt` | task | no (`side-effecting/mutates-host-store`), accepted probe | pinned Binaryen identity | none | pg-chain adds the tool fingerprint to dependents |
| 2 | `@semio-tech/os-plugin-describe-rs:build`, engines `<surface|editor|node-graph|puzzle>:wasm`, canvas/infinite fonts | task | yes | cargo module closure, receipts | emitter dir, `.🧾️store/*.json` (store receipt verified after the run, repaired alone when the store lost it) | existing |
| 3 | `framework-plugin-web:support`, per-component driver, `plugin-registry:session-<variant>` x9, `prepare-<variant>-react-prototype` x9, `activate-<variant>-react-prototype` x3 (`energy`, `fem3d`, `generator`) | task | pg-chain | see pg-chain | module dirs, sessions, lane runtime dirs | pg-chain |
| 4 | demonstrator `prepare-prototype` | task | **yes** `deterministic/check-without-outputs` | `activationSources` + transitive dependency outputs | none | here |
| 5 | demonstrator `activate-lanes-prototype` | **noop** (was a process that only logged) | n/a | none | none | here |
| 6 | demonstrator `activate-prototype` | task | **yes** `deterministic/pure-function-of-inputs` | `activationSources` + transitive dependency outputs | `dist/♻️activation/prototype` | here |
| 7 | `serve` / `dev` (:6029) | launch | interactive, decision `continuous/serves-until-stopped` | transitive restore of the whole closure | none | here |
| 8 | `prepare-e2e` | task | no (`side-effecting/mutates-host-store`: opens the invocation-scoped service session) | transitive restore | none | here |
| 9 | `serve-e2e` | launch | interactive (decision recorded), port 0, readiness through `ready.json` (budget fixed by zt-e2e-fix) | transitive restore via `prepare-e2e` | none | here |
| 10 | `test-e2e` | task | no (`live/drives-running-process`), Playwright vs the running server | none | reports under `SEMIO_TICKET_DIR` | existing |
| 11 | `build` | task | no (`managed-output`; network tiles, section 0.5) | | `.🧾️store/build.json` | existing |

Expected warm re-activation once pg-chain lands: only `deps-wasm-opt` and `generator-inputs` (probes) execute; a one-component edit re-runs that component, the lane verdicts of lanes whose boot closure contains it (cut off when the bytes are equal), the lane activate only on changed bytes or owner source, then rows 4 and 6.

## 3. Staged set (`C/demo-bzl-tools/apply.py`)

Dry run is the default (`python3 apply.py`), `--write` applies all-or-nothing, a file whose sha256 differs from `bases.json` is refused unless `--accept-drift` (every anchor must still occur exactly once; rerun `--bases` to re-record), `--status`, `--group demonstrator|bericht|deck`. Idempotent. No source copies (path limit), only anchored block edits.

| File | Change |
|---|---|
| `♻️mit-bestand/🧺️demonstrator/📋️project.json` | rows 4-9 above; also `cache` decisions `continuous/serves-until-stopped` on `serve`, `dev`, `serve-e2e` |
| `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` | removes the dead `complete` command (`CompleteScript`, its `checkScriptInvocation` import) |
| `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts` | asserts `activate-prototype` cached, `activate-lanes-prototype` `repo:noop`, transitive restore on the four launch/e2e targets (this suite already asserted `prepare-<profile>.cache === true`, which the cache-fix demotion had broken) |
| `♻️mit-bestand/🧺️demonstrator/README.md` | EN+DE: lane gate is an ordering node; `build` is uncached because of network tiles |
| `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` | the three `build-*` outputs = products-store report slots |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json` | `dev` records `continuous/serves-until-stopped` (the deck `build` is already a cached receipt target, `test` and `canonical-architecture` are cached) |

Interaction with `pg-chain`: none of these files; the test they rewrite is theirs.

## 4. Hermetic TeX toolchain: decision (recorded)

Decision: **keep Tectonic with its pinned material as the only TeX toolchain; do not adopt a system TeX Live, `rules_latex`, or a Docker image.** Reasons, each verified statically by `verify-independent.py`:
- Tectonic 0.16.9 is pinned per platform by archive sha256 and byte size (7 platforms, `🔧️toolchain/🔣️.json`); the extracted binary sha256 is stored in `.toolchain.json` and re-checked on every use.
- The TeX bundle is not Tectonic's default network bundle: 500 files, each pinned by sha256 and byte range in `📚️bundle/🔒️dependencies.json` (19.9 MB), fetched by HTTP range requests, then used as a local file URL, so compilation is offline.
- Fonts come from the repository (`🖼️assets/🔤️font/**/*.ttf`), hash-compared to the prepared copy.
- The compile environment is closed: `SOURCE_DATE_EPOCH`, `TZ=UTC`, `HOME`/caches in the per-run scratch, no `NODE_OPTIONS`, `TECTONIC_CACHE_DIR` in scratch.
- The report/print `build-*` actions list the toolchain pin, bundle lock, LaTeX tree, fonts, `bun --version` and platform as inputs, so a cache hit never depends on what is installed on the machine; a restore needs no TeX at all.
Consequences: the document actions are normal cached Bazel actions with exact inputs and the store slot as output. `deps-tectonic`, `deps-tex` and `fonts` stay host-state provisioning tasks (they write the tools/products store behind custody checks and cannot be restored by tree copy: `printArtifactNxOutputV1` refuses the tools store by design); under Bazel they run on every invocation as cheap verification probes (pinned-hash reads, ~20 MB). Open design for later, once measured: provision inside the consuming action on a miss (lazy), so a fully warm document build executes nothing.

## 5. Verification (static; all run, outputs below)

- `python3 apply.py` on the live tree: 6 files pending, no refusal (before/after bytes 10728->11003, 8725->8256, 8399->8930, 19952->20225, 24165->24327, 3008->3102).
- Overlay (copies in `/private/tmp/dbz/ov`, recreate with `overlay.sh`): `apply.py --write` then a second run `0 file(s) would change` (idempotent); `bun build --no-bundle` of the edited runtime script and test: syntax ok.
- `bun law-check.ts`: the live `cacheCorrectnessFindingsV1` plus the closed project schema (Ajv) over 20 demonstrator/report/deck targets: all three manifests valid, 0 findings (the unedited `serve`, `dev`, `serve-e2e`, deck `dev` produced `uncached-without-reason`, now fixed).
- `python3 -I verify-independent.py <root>`: 41 ok / 0 FAIL on the overlay; on the live tree it fails exactly the staged items (negative control).
- Cross-check against the repository's own function: `bun layout.ts` (`printArtifactNxOutputV1`) equals the path derived in Python from the schema constants for all three reports.
- Not run: `bun repo`, `show projects`, inference, any Bazel plan or build, the runtime suite (`bun test` of `🧪️tests/🟦️.ts` needs the full workspace and an envelope), Playwright.

## 6. After GO (commands; heavy ones one at a time, coordinator's slot)

1. `python3 C/demo-bzl-tools/apply.py` (dry run), then `--write` (re-check drift; `--bases` then if peers moved a file).
2. Run `python3 -I C/demo-bzl-tools/verify-independent.py` (expect 0 FAIL) and `bun test ./♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts` under the usual envelope.
3. `C/demo-bzl-tools/cold-run.sh preflight`, then with `DEMO_BZL_GO=1`: `cold` (expected order of 41 min, memory), `warm` (expect only the two probes executed), `probe comment|code` (only the edited component chain, then disk hits), `probe brand` (no Rust or wasm action), `e2e` (`run-e2e.sh demonstrator --tool=bazel --skip-activate`; check teardown with `lsof -nP -iTCP:6029 -sTCP:LISTEN`), `report cold|warm|probe` and `deck`.
4. Report "ready for cold run" to the coordinator before step 3.

## 7. Risks and open items

1. Everything in row 3 depends on `pg-chain`; without it the nine lane `prepare` tasks stay `driver-owned` and uncached, so the only cached win from this set is rows 4-9 and the report outputs.
2. A cached check with no outputs (rows 4) hits without restoring its dependencies; the restore happens in the consumer that runs (activate, launchers). That is why rows 7-9 now declare transitive restore.
3. Store content is not in the Bazel cache for receipt-only producers (wasm, fonts); the neutral script repairs a hit whose receipt names missing content by re-executing that producer alone (`withStoreReceiptsV1`), costing a rebuild on a fresh machine. Store-aware restore is the adapter owner's item (also in result-play-bzl).
4. `stdio` and other components that cannot link would fail their lane exactly as today; run Bazel with `--keep_going`.
5. The network: component crates fetch registry crates (locked), the release `build` fetches tiles, `deps-*` download pinned archives. Remote execution workers need egress or a mirror.
6. `fonts` could become a cached target (kind `fonts` is a supported Nx projection of `printArtifactNxOutputV1`); left as authored until a clean-store restore is measured.
7. `.vscode` launch rows are still absent (AGENTS conflict, not mine).

## 8. Files

Created: `C/result-demo-bzl.md`, `C/demo-bzl-tools/{apply.py,bases.json,overlay.sh,verify.sh,verify-independent.py,law-check.ts,layout.ts,cold-run.sh}`. Repo files outside `C/`: none written. Scratch `/private/tmp/dbz` kept for `verify.sh` (recreate with `overlay.sh`).
