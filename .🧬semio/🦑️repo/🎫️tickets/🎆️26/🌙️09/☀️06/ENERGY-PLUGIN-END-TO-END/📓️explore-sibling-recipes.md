# 📓️ Explore: sibling-ticket recipes for the 🔋️energy plugin end-to-end

Read-only mining of `26/09/05/{BLOCK,DRAW,RASTER}-PLUGIN-END-TO-END`, `26/09/06/REMODEL-PLUGIN-END-TO-END`,
`26/09/05/S-END-TO-END`, `26/09/01/{PROCESS,SOURCING}-END-TO-END`,
`26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`, `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`,
plus a skim of `26/09/02/COMPLETE-SEMIO-END-TO-END`. All four plugin-shaped siblings (BLOCK/DRAW/RASTER/REMODEL)
were **still open, not closed**, at read time — treat every recipe below as "worked at least once", not "this
class of plugin is done".

No energy-specific prep exists anywhere in the ticket tree (`grep -ril energy` under COMPLETE-SEMIO hits only
unrelated filenames — none is about the 🔋️energy plugin). We are first.

**Energy's own current compile state, per `S-END-TO-END/📓️explore-per-plugin-blockers.md` and
`📓️explore-catalog-build-state.md`** (read by a background research pass on this same ticket, 2026-09-05/06):
energy does **not** compile today — `semio_framework` import unresolved, 279 errors, first at
`🔋️model/…/🧵️simulation-session/🦀️.rs:7`; it has a staged descriptor but **no core wasm and no JS module**
yet (a false-positive trap: a present descriptor does not imply a present build). It is listed in that
ticket's dispatch table as "remaining plugins with 0 actions or non-compiling crates — must compile first."
Treat native-check-green as the true first milestone, before any dispatch/oracle work.

---

## 1. Recipes (command blocks)

### 1.1 Native `cargo check`/`cargo test`, private target dir, sccache bypass
```bash
cd /Users/ueli/Documents/semio
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-energy-e2e \
CARGO_BUILD_JOBS=3 RUSTFLAGS=-Awarnings \
cargo check -p semio-s-plugin-energy --lib --message-format=short
# then, once check is green:
cargo test -p semio-s-plugin-energy --lib
```
Sources: BLOCK `📓️w7a-block2d-compile.md:152-160`, DRAW `📓️w2-compile.md:4-5`, RASTER `📓️status.md:18`
(`nice 10, -j 2, sccache bypassed`), PROCESS `📓️batch-only-migration.md` verification table
(`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-process` → `Finished dev profile in 40m 34s`).
`CARGO_BUILD_JOBS=2-4` was used everywhere under load; never left at cargo's default under concurrent peers.
`RUSTFLAGS=-Awarnings` is optional (used by BLOCK only, to shorten log noise for a 60-error triage pass).

### 1.2 wasm32-wasip2 build/check
```bash
export CARGO_PROFILE_WASM_DEV_DEBUG=false   # cuts stdio's rustc RSS from 8.6GB to 165MB — see pitfalls
export SEMIO_BUILD_BUDGET_MS=3600000        # 1h, default is 20min and WILL SIGKILL under lock contention
cargo build -p semio-s-plugin-energy --target wasm32-wasip2 --profile wasm-dev --features component-guest
```
Source: RASTER `📓️explore-raster-dev-boot-ts.md:397-422` (mirrors `pluginCargoArgs`/`PLUGIN_WASM_TARGET` in the
dev-package `📜️script.ts`). Root `Cargo.toml` already sets `[profile.dev] debug=false` /
`[profile.wasm-dev] inherits="dev", codegen-units=1` — exporting the env var anyway is "cheap insurance"
against a peer reverting it (BLOCK hit exactly this: a stalled stdio wasm build at 8.6GB RSS/45GB swap was
killed and relaunched with the override, RSS fell to 165MB — `📓️status.md` 2026-09-05 13:45/18:25).

### 1.3 taskpolicy / nohup background-build pattern
```bash
nohup <build command> > <scratchpad>/log.txt 2>&1 & disown
```
Then, if the build is throttled at ~5% CPU (macOS background-QoS on anything spawned from a background tool
call), lift it and keep re-lifting its children (a fresh rustc child restarts throttled):
```bash
# from DEMONSTRATOR's 🚀️unthrottle.sh — adapt MINE to energy's own private target dir name
MINE="target-energy-e2e"
while true; do
  for c in $(pgrep -f "bin/cargo" 2>/dev/null); do
    env_line=$(ps eww -o command= -p "$c" 2>/dev/null | tr ' ' '\n' | grep '^CARGO_TARGET_DIR=' | head -1)
    case "$env_line" in *"$MINE"*) ;; *) continue ;; esac
    taskpolicy -B -p "$c" 2>/dev/null
    for d in $(pgrep -P "$c" 2>/dev/null); do
      taskpolicy -B -p "$d" 2>/dev/null
      for g in $(pgrep -P "$d" 2>/dev/null); do taskpolicy -B -p "$g" 2>/dev/null; done
    done
  done
  sleep 30
done
```
Full script: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🚀️unthrottle.sh`. Poll a
real HTTP port for readiness, never the log file — it can vanish if a peer/subagent sweeps `🗑️generated/`
mid-run (see Pitfalls).

### 1.4 `describe` regeneration
```bash
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust
bun ./📜️script.ts describe
# or, from repo root:
bun nx run @semio-tech/energy-plugin:describe
```
Rebuilds the wasm32-wasip2 component, extracts the core, and calls the `semio-framework-plugin-describe`
binary's `describe` subcommand, writing `🔣️.json`/`🛂️.descriptor.semio` straight onto the owner root (never
into a generated/ dir). Emitter code path: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
(`describePluginComponent` → `emitOwnerDescriptorPairV1`, S-END-TO-END `📓️opus-descriptor-producer.md` §2.1) —
stages into a scratch dir *inside the cargo target root* (not `tmpdir()` — that's now a containment
violation), verifies both hashes, then does two `renameSync` calls onto the owner root; a failure never
half-publishes. Do this **after** the crate builds — every sibling ticket found the committed descriptor
schema-stale/content-divergent and needing a fresh `describe` run (see §6).

### 1.5 registry `check` / `generate`
```bash
bun 📇️registry/📜️script.ts generate   # hard gate before any dev boot; reads 🔣️taxonomy.json
bun 📇️registry/📜️script.ts check      # descriptor pair validation — now FAIL-CLOSED (S-END-TO-END lane B)
```
`generate` stays permissive (never calls `validateDescriptors`) — `dev energy` still boots against a partially
described catalog. `check` is where the descriptor-pair contract, packageId/pluginId/role/host match, canonical
pack re-encoding, self-hash, placeholder-identity rejection, and the **classification-drift audit**
(committed `interactiveJob` vs what the owner's own Rust declares) all live —
`🔌️plugin/📇️registry/📜️script.ts:1989-2090` (S-END-TO-END `📓️opus-descriptor-producer.md` §2.2-2.3).

### 1.6 `test quick|long|exhaustive`
```bash
bun ./📜️script.ts test [quick|long|exhaustive]     # level read from env SEMIO_TEST_LEVEL
nx run @semio-tech/energy-plugin:test[-quick|-long|-exhaustive]
```
`test quick` has a 30s budget and used to be silently killed with zero results for slow crates — S-END-TO-END
lane A fixed this repo-wide by level-gating the ~26 slowest cases with `atTestLevel(it, "long")`
(`🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`, ⏱️Budget region) — energy's own TS suite should
follow that convention from the start rather than retrofitting it later.

### 1.7 One 🥒 feature test through the repo test platform
```bash
python3 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py \
  --plan <plan.json> --out <results.jsonl> --adapter <component.py>
# plan.json is generated by the TS coordinator, not hand-written:
bun ./📜️script.ts contract     # or nx target `test-contract` — runs validateAllContracts,
                                # writes .🧬semio/🦑️repo/⚡️cache/breaches/testing.json, exits 1 on any breach
```
Mechanism traced in full in REMODEL `📓️explore-tests-oracles.md` §1 (schema v2,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`). `asset://` in a feature file resolves
against the **subset owner root**, never a dedicated fixtures dir (`resolveFixtures`, same file). No sibling
ticket actually ran this end-to-end (all were memory-blocked) — energy will be the first live run if attempted.

### 1.8 Dev boot (react, then wgpu wasm)
```bash
# React
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-energy-e2e \
SEMIO_RENDERER=react \
SEMIO_BUILD_BUDGET_MS=3600000 \
SKIP_ENGINE_BUILD=1 \
bun ./📜️script.ts dev energy
# poll http://127.0.0.1:<react-port>/ for the readiness beacon

# wgpu wasm (bare default — no SEMIO_RENDERER needed)
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-energy-e2e \
SEMIO_BUILD_BUDGET_MS=3600000 \
bun ./📜️script.ts dev energy
# poll http://127.0.0.1:<wgpu-port>/?plugin=energy

# native wgpu window / headless smoke
bun 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native energy
# add --smoke for a headless boot that dumps the widget tree as JSON instead of opening a window
```
Every sibling ticket's dev-boot explore report converges on the same five facts — treat all five as true
for energy without re-deriving them:
1. **Default renderer is wgpu, not react.** Bare `bun run dev:energy` boots native `trunk serve`. Must export
   `SEMIO_RENDERER=react` (or use the `served` launch-json segment) to reach `ShellHost`.
2. **`SEMIO_PLUGIN_ONLY=energy` never isolates the browser-side load closure** — only narrows which crates get
   cargo-built. The registry's `{energy, stdio}` closure (whatever energy's actual dependency edge is — check
   with `filterPlugin`) is fixed independently; used cold it risks cascading both to
   `plugin.descriptor-unavailable`.
3. **First boot of a session must be a full, un-narrowed build** (no `SEMIO_PLUGIN_ONLY`) — do not reach for a
   narrowed build on a cold tree (BLOCK `📓️explore-dev-boot-path.md` §5 step 1).
4. **`SEMIO_BUILD_BUDGET_MS` default is 1,200,000 ms (20 min)** — a build queued behind another session's
   shared `target/debug/.cargo-lock` will be SIGKILL'd. Always isolate `CARGO_TARGET_DIR` and raise this under
   concurrent load.
5. **Poll a real HTTP port, not a log file**, for readiness — background dev-server logs can vanish if
   `🗑️generated/` gets swept mid-run (project memory `feedback-subagents-sweep-ticket-generated-folder`).

Also: an explicit `served` launch-json segment can serve 404 for every document while `/@vite/client` still
answers 200 (dev server up, root not served) — S-END-TO-END lane A hit this live; drop `served` if it happens
(`📓️opus-catalog-smoke-harness.md` §4).

### 1.9 Verifying dispatch/windows in the browser
```bash
bun ./📜️script.ts verify catalog          # spawns every declared program in one session,
                                            # asserts non-empty bounding box + ≥1 descendant per window
bunx playwright test .storybook/os-plugins.spec.ts -g "energy"
```
`verify catalog`'s mechanism (S-END-TO-END `📓️opus-catalog-smoke-harness.md` §3): a dev-only
`window.__semioOsCatalogProbe` (guarded by `import.meta.env.DEV`) exposes `{shellPluginId, ready, plugins:
[{pluginId,status}], programs: [{pluginId,appId,label}], spawned}`; the runner navigates with
`waitUntil:"commit"`, polls the readiness beacon on its own deadline, enumerates programs from the probe
(never hardcoded), spawns each through the real command-palette item
`[data-slot="command-item"][data-command-item-id="spawn.<pluginId>"]`, diffs
`[id^='framework.window.']` before/after, asserts non-zero bounding box **and** ≥1 descendant, and attributes
console/page errors per program. A shell that never reaches `ready:` is reported as a `boot.failure` row with
the last 20 console errors + a 1000-char body excerpt — never a thrown navigation error. A vacuous green (0
programs rendered) is itself scored a failure.
For manual/console verification: check for `data-semio-window-fault`/`-code`/`-origin` attributes on any
window body (the six-class fault taxonomy — `abi-mismatch`/`interactive-ceiling`/`clock`/`plugin-internal`/
`install-failed`/`unknown` — S-END-TO-END lane C, `📓️opus-fault-discriminators.md` §3) and grep the console for
`[DEBUG] render failed [<class>] <code>`. Console must show no `turn failed`/`render failed`/
`readConflicts failed` for the energy actor (PROCESS `🧪️runtime-verification.md` §"What the checklist is").

---

## 2. Reusable scripts (paths + what they do)

- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/🐍️w3-io-leaves.py`** — generates typed
  `Serializer<Snapshot>`/`Deserializer<Snapshot>` leaf files per format per subset (real json/txt/zip vs.
  honest-`Err` stubs for formats the schema can't reach). Re-runnable, writes files in full. Directly
  reusable as a template for energy's own `🚪️io` leaf generation once the model/zones/simulation subsets are
  known.
- **`…/BLOCK-PLUGIN-END-TO-END/🐍️w3-io-roots.py`** — generates the `io() -> IoDeclaration` roots wiring every
  leaf onto the `io_mechanism` channel, and the subset-root `🦀️.rs` binding (`io: io::io()`). Deletes any old
  `ComposerEntry`/`io_registry` channel outright.
- **`…/BLOCK-PLUGIN-END-TO-END/🐍️w3-io-typescript.py`** / **`🐍️w3-io-typescript-txt.py`** — generate TS mirrors
  (field tables + writer/reader, and a real `.semio` DSL reader) of the Rust io leaves, for cross-language
  fixture parity.
- **`…/BLOCK-PLUGIN-END-TO-END/🟦️w3-fixture.ts`** — renders shared `🔣️json` parity fixtures from each
  subset's own DSL example asset using the TS reader+writer, so Rust and TS assert against the same
  committed bytes.
- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🐍️fix-fixture-include-paths.py`** —
  repoints stale `include_str!`/`include_bytes!` literals to renamed (hash-suffixed) fixture directories;
  dry-run by default, does a unique dash-prefix match on disk, `--apply` writes. **Directly reusable if
  energy's own fixture dirs get hit by the repo-wide emoji/hash rename sweep** (see Pitfalls — this hit 3 of
  4 sibling plugins independently).
- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🐍️fixture-audit.py`** — parses a
  `.feature` file's Scenario Outline steps + Examples tables, substitutes `<dir>`/`<fixture>` placeholders,
  and checks every resulting `asset://…` path against disk — mirrors the TS coordinator's
  `fixtureUrisIn`+`resolveFixtures` exactly. Use this on energy's own feature file(s) before assuming they
  resolve.
- **`…/REMODEL-PLUGIN-END-TO-END/🐍️mount-check.py`** and **RASTER's `🐍️w2-check-mounts.py`** — read-only
  `#[path]` mount resolution checkers (confirm every `#[path]`/`include_str!` literal in a crate resolves on
  disk before trusting a check run).
- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🔎️scan-dangling-path-refs.py`**
  — scans for dangling `#[path]`/`include_str!` references repo-wide; same class of check as above, more
  general.
- **`…/LOWPOLY-…/🔬️validate-lowpoly-fixtures.ts`** — cross-language fixture validator (ajv schema validation
  over recorded mutation fixtures).
- **`…/LOWPOLY-…/📜️script.ts`** — the ticket-local nx-style command dispatcher used for `test discover` etc.
  during that ticket.
- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🔨️build-components.sh`** —
  serial per-plugin wasm component build with a retry-on-known-transient-error loop (see full quoted script
  in §1.3 context / Pitfalls — retries only on "peer renamed a path mid-build" signatures, never on a real
  compile error), logs to `LOG_DIR` (scratchpad, never the ticket's `🗑️generated`).
- **`…/DEMONSTRATOR-…/🚀️unthrottle.sh`** — the taskpolicy re-application loop quoted in full in §1.3.
- **`…/DEMONSTRATOR-…/🔁️heal-loop.sh`** — repairs paths broken by a peer's in-flight rename within ~45s (the
  reason `build-components.sh` retries rather than treats a "couldn't read <path>" as fatal).
- **`…/DEMONSTRATOR-…/🩺️heal-paths.py`** — the path-healing implementation `heal-loop.sh` invokes.
- **`…/DEMONSTRATOR-…/🔨️guard.sh`**, **`🔨️run-e2e.sh`** — acceptance-test harness scaffolding for that
  ticket's six-app demonstrator; less directly reusable for a single-plugin ticket like energy, but
  `run-e2e.sh`'s pattern (boot once, walk every window, assert non-empty, screenshot) is the same shape as
  `verify catalog` in §1.9.
- **`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/🔨️classification-drift-probe.py`** — measures the
  interactive-job classification-drift audit's bidirectional-vs-narrow-rule false-positive rate (742 vs 206
  findings across 4 owners) — reference implementation if you need to re-run that audit against energy
  specifically after landing dispatch classification.
- **`…/S-END-TO-END/🔨️register-extension-describe.ts`** — mechanically wires a `describe` nx target +
  `DescribeScript` into N owner `📜️script.ts`/`📋️project.json` pairs at once (used for 26 extension owners,
  not plugins — not directly applicable to energy, which already has a plugin-shaped `describe` target, but
  useful as a model if energy turns out to need the same wiring repaired).
- **`…/S-END-TO-END/🔨️lane-g-boot-probe.ts`**, **`🔨️shard-liveness-fault-probe.ts`** — boot-resilience/shard
  liveness diagnostic probes (framework-level, not plugin-specific — useful if energy's dev boot exhibits the
  "anonymous shard worker error" symptom described in Pitfalls).

---

## 3. Adding new mutation kinds + fixtures — the LOWPOLY playbook

LOWPOLY is the closest precedent (energy's `📓️status.md` already notes only one mutation kind exists today,
`♻️replace-model`, and the goal implies adding more). LOWPOLY went from 19/47 dispatchable commands to 47/47,
adding 11 new mutation-kind fixture cases and 2 new dispositions along the way. The reusable procedure:

1. **Classify every command against `AppActionRegistry`** — read the handler's *actual* `Emit` construction
   (never infer disposition from the command's name). LOWPOLY needed two new disposition/lane shapes beyond
   the original six because a real handler (`addPrimitive`) emitted to two lanes at once and also touched
   session-transient state:
   - `ArtifactConfig` — emits both an artifact mutation (`CreateObject`) and a config mutation
     (`SetActiveObject`) from one command.
   - `ArtifactConfigTransient` — additionally reads+writes a session-scoped scratch struct
     (`LowpolyScratch`/`build_doc`/`set_mesh_workspace_map`); without a `Transient` lane the handler silently
     no-ops against a blank scratch after any prior edit.
2. **Per command, wire exactly five things**: exact admission check, exact Store-publication-authority
   coverage (LOWPOLY's `lowpoly_artifact_mutation_retained_bytes` went from admitting 4 of 17 mutation
   variants to being exhaustive over all 17 — this kind of helper must be **exhaustive**, not just
   "covers what's tested"), a disposition, an `ArtifactToolPublicationContract`, a `ToolExecutionContract`,
   a reduce arm, a route-table entry, and the manifest classification flip to `Migrated`.
3. **io format honesty**: for each of the plugin's formats (energy will likely have IDF/gbXML/OSM-adjacent
   or a native JSON-only set — check what the model actually needs), decide real vs. honest-stub per
   format. LOWPOLY's stub formats (STL/gltf/dwg/las) fail loudly with an explicit `Err` and a reason —
   **never silently emit an empty-but-schema-valid file**. The `Option<ArtifactChild<S>>` content-addressed
   handle pattern means a synchronous io serializer structurally cannot reach composed-child geometry; if
   energy's model has a similar composed-child shape, budget for the same limitation rather than trying to
   force a real serializer through it.
4. **Schema truth check across all four representations** (json-schema / rust / typescript / protobuf /
   graphql if energy uses graphql) — LOWPOLY found the non-Rust representations held a verbatim wrong copy
   of the artifact-lane schema (a stale field, byte fields typed as int arrays instead of base64 strings, a
   misnested field, a dangling `$ref`). Run ajv over every recorded fixture as a cheap catch-all.
5. **TypeScript package.json**: every sibling ticket found the plugin's `package.json` was a verbatim
   copy-paste of another plugin's (cad-js in BLOCK/DRAW/RASTER's cases) with wrong `scripts`/`description`.
   **Check energy's TS `package.json` early** and rewrite it against its actual import graph
   (`dependencies: {}` if it only imports node/bun builtins, per BLOCK's fixed shape).

### Traps hit adding/renaming fixtures (recurring across BLOCK/DRAW/RASTER/REMODEL independently)
- **Emoji-filename hash-rename drift**: a repo-wide "normalize semantic emoji path identities" sweep renames
  friendly-slug fixture dirs to `<slug>-<6-hex>` (e.g. `➕️appends-shape-b-at-the-root` →
  `➕️appends-shape-b-0b0435`) without touching the `#[path]`/`include_str!` literals or `.feature` Examples
  tables that name them by exact string. This independently broke DRAW (12 mounts + 72 `include_str!`),
  RASTER (12 mounts), and REMODEL (34 `#[path]` + 99 `include_str!` + 102 `asset://` URIs, **100% of that
  ticket's fixture references**). **Before trusting any fixture path in energy, run a mount-check script
  (§2) first.**
- **`#[path]` depth is often correct-but-scary**: a 5-`..` literal path shared verbatim by several plugins'
  test-host adapters resolves correctly because those files are re-mounted into a cache-local
  generated-test-host crate at test-run time — the path is relative to the generated location, not the
  source tree. Don't "fix" a deep relative path just because it looks wrong under naive on-disk resolution.
- **Compile-time (`include_str!`) vs runtime (`asset://` read at test time) fixture wiring**: REMODEL's
  harness reads kind+paths at compile time via 3 separate Rust literals per kind — fragile, a rename breaks
  all 3 at once. Puzzle's harness reads kind+all 5 file paths at **runtime** out of the `.feature` scenario's
  own doc-string JSON (`ctx.doc_json()`/`ctx.fixture_json(asset://…)`) — a rename there only touches the
  `.feature` file's data. **Use puzzle's runtime pattern for energy's harness from the start**, not the
  compile-time-literal pattern.
- **`🔣️oracle.json` catalog is authoritative over physical layout**: mutation ownership is declared by the
  subset's `🔮️oracle/🔣️.json` `mutationCatalogs[]`/`mutationManifests[]`; a manifest's `subset` field wins
  even when leaf code physically sits elsewhere. Confirmed identically in all four sibling plugins.
- **Discovery is filename-exact**: the taxonomy's `testContributionDirName` is `🔮️oracle`, not `🧪️oracle` —
  BLOCK found this rename was a real, necessary discovery fix, not cosmetic.
- **`🎯️outcome` schema tends toward trivial**: REMODEL's fixtures were `{"status":"applied"}` for all 34
  happy-path kinds, zero refusal-path fixtures except one deliberate error vector. Budget explicit
  refusal/error vectors for energy's mutations from the start if the DoD implies more than happy-path
  coverage.
- **`🥒️.feature` tag convention** (RASTER): `@capability-<id>-1-mutate @oracle-<id>-<lang>-independent
  @comparison-ordered-json-v1 @mutations-<id>-1-any`, three Scenario Outlines (`@id-mutate`/`@id-inverse`/
  `@id-spec-vector`) + one plain `@id-identity-round-trip` scenario.
- **`nativeSecondImplementationBreaches` discharges by capability, not by mutation kind** (REMODEL's most
  important finding, `📓️explore-tests-oracles.md` §2): if energy declares one blanket capability across
  every mutation kind, a kind with zero actual oracle vector still passes the automated contract gate
  "by construction". If any of energy's kinds is structurally hard to oracle (reads global state, etc.),
  either give it its own narrower capability or record an explicit `noOracleDecisions` exception — don't
  rely on the coarse gate to catch the gap.

---

## 4. Third-party oracle patterns

Energy's own DoD explicitly names Honeybee-energy → OpenStudio → EnergyPlus as the oracle chain, with
lightweight Python/TS oracles where E+ isn't the right instrument — none of these binaries exist on this host
(`📓️status.md`: "No EnergyPlus/OpenStudio/honeybee on host; Docker 29 available; Python 3.14 venv via uv").
The sibling tickets never had that particular toolchain, but their *oracle-writing methodology* transfers:

- **`verified-native-second-implementation`** is the earned (not asserted) discharge path when no real third
  party reads your native format — BLOCK and REMODEL both used it: a from-scratch Python (or other-language)
  re-implementation written directly from schema/mutation JSON + grammar, driven through the repo's cucumber
  adapter. Requires (per `🟦️.ts:2841 QUALIFYING_ORACLE_KINDS` / `:4868-4995`): a real
  `nativeSecondImplementation` evidence object, format = `isSemioNativeArtifact`, 100%-**capability**
  coverage, a credible `noThirdPartySurvey` (named ecosystems considered + ≥1 declined candidate with a
  ≥10-char structural reason), subject-language ≠ second-implementation-language, a non-empty
  `specificationSource`, and `fixtureCoverage.vectors > 0`.
- **A real third party wrapped behind an owned interface** (DRAW, RASTER): DRAW's `quick-xml`-based
  `🔬️probes/🦀️oracle-probe/🦀️.rs` reads the subject's own exported output independently and writes its own
  output directly with the third-party writer — **never calling the subject's own serializer**, to keep the
  oracle honestly independent. RASTER split codec-layer oracle (image-rs crates: `png`/`gif`/`image`) from
  mutation-document oracle (hand-written Python cross-language differential) and explicitly declined Pillow
  for the document layer with a recorded reason ("Pillow reads pixel files, has no authority over a layer
  tree"). **For energy: if EnergyPlus/OpenStudio genuinely aren't installable in this environment, this
  "declined with a recorded structural reason" pattern is the legitimate fallback — don't silently skip the
  oracle requirement, write the decline into `noThirdPartySurvey`/`noOracleDecisions` with real named
  candidates and reasons.**
- **Proving an oracle is real, not vacuous, by fail-injection** (LOWPOLY's PNG↔Pillow oracle): corrupt the
  pixel-equality check deliberately, confirm a real `AssertionError` fires, then restore and confirm
  byte-identical output (MD5). Do this for whichever oracle energy ends up using — a green oracle run alone is
  not proof it can fail.
- **Cross-language fixture parity via a shared committed fixture** (BLOCK): render one canonical fixture from
  Rust and from TS off the same DSL source asset, assert byte-identity in both languages against the
  committed bytes — cheap and catches drift at compile-fixture level without needing a heavyweight external
  binary at all.
- **Comparison tag**: `@comparison-ordered-json-v1` is the repo convention tag/profile for "compare these two
  JSON documents order-sensitively" — used by RASTER and referenced generically elsewhere.
- **Existing third-party format oracles already in the repo, reusable supplementally**: `🗄️stdio` ships real
  PLY (`ply-rs`), OBJ (`tobj`), LAS (`las`), STL (`stl_io`) oracles with backing crates already declared
  (feature-gated, optional) in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`. If energy's
  model or simulation output can be exported to any of these already-oracled formats, round-tripping through
  them is a legitimate **supplemental** (not discharging on its own, but strengthening) cross-check —
  REMODEL flagged this as a real, currently-untaken opportunity for its own point/mesh outputs; the same
  logic may apply to energy's geometry (zones/surfaces) if it can be exported to OBJ/glTF.

---

## 5. Fixing `interactive-job.*` / bounded-factory dispatch faults

This is the single most consistent finding across every sibling ticket — expect energy's editor actions to
be dead on arrival unless already wired, and expect the fix to look identical every time:

1. **Classification flip**: every dispatchable action must read
   `.action_interactive_job(id, InteractiveJobClassification::Migrated)` in the `create_<app>()` manifest
   builder — OR, per S-END-TO-END lane D's cleaner convention, a single blanket
   `.interactive_jobs(InteractiveJobClassification::Migrated)` sweep if energy's action ids are all genuinely
   ready (careful: `action_interactive_job` only touches `self.actions`, silently no-opping for ids declared
   as *commands* rather than actions — PROCESS hit this exact silent-no-op bug). Never leave an id
   `Unclassified` — `EditorBuilder::try_build_definition` **panics** at manifest-assembly time for any
   `Unclassified` action (BLOCK found this the hard way in block3d).
2. **Factory apparatus** (only if `bounded_first_step_tool_proofs!` doesn't exist yet): add
   `<APP>_RETAINED_TOOL_IDS`, a `<App>RetainedCommandJobFactory` implementing `ToolJobFactory` (classification
   = `Migrated`) + `ArtifactOwnedToolJobFactory` (`owner`, `TOOL_IDS`, `DOCUMENT_SCHEMA`, and **critically**
   `const PUBLICATION_CONTRACTS` — an empty/omitted one makes `register_tool_job_factories`
   unconditionally fault `interactive-job.publication-contract` at app-construction, DRAW's exact bug),
   `register_tool_job_factories`, `build_tool_job`, and a `build_{artifact,config}_store_one_item_preparation_factory`
   override per lane actually used.
3. **`factory_type:` inside `bounded_first_step_tool_proofs!` must name the real factory type** — its absence
   (a "bare" proof) turns dispatch into `interactive-job.missing-owned-reducer`. This is the exact
   block5d-vs-block2d/3d precedent and the root cause first identified in the PROCEDURAL-3D ticket.
4. **Read the lane off the handler's actual `Emit`, never off the action's name.** An action declared with a
   lane that has no matching `build_{artifact,config}_store_one_item_preparation_factory` faults
   `interactive-job.publication-authority-missing` at dispatch (DRAW). A handler that emits to two lanes at
   once (both `Artifact` and `Config`, or additionally reads/writes session-transient state) needs the
   widened `ArtifactConfig`/`ArtifactConfigTransient` dispositions (LOWPOLY, §3 above) or a multi-lane
   `lanes: Lane[]` schema (BLOCK's block3d widening from single `lane` string to an array).
5. **If many apps in the plugin share the identical action shape**, consider S-END-TO-END lane D's shared
   generic factory pattern instead of N hand-copies: one `NormBoundedCommandJobFactory<A>` generic over the
   app type, with each app supplying only a 3-line `trait NormRetainedEditor { dispatch_retained }` impl and
   a one-line macro invocation (`norm_owned_tool_job_factory!(<Stem>BoundedCommandJobFactory, <Stem>PlayApp)`)
   — collapsed what would have been per-app hand-written store-preparation code into one generic
   `NormOneItemPreparationFactory<P, M>`. Relevant if energy ends up with several near-identical
   apps/windows (e.g. structure/zones/simulation sharing a command shape).
   **Gotcha for this pattern**: `ArtifactBoundedFirstStepProof` matches the `factory:` literal against
   `type_name::<T>().rsplit("::").next()` — a bare generic instantiation's `type_name` ends in
   `…SomeGenericFactory<…::ConcretePlayApp>`, so each concrete app needs its own thin newtype wrapper, not a
   literal generic reference.
6. **`testkit::new_app()` (registry-less) breaks the moment proofs are declared** — it builds an app with no
   `AppActionRegistry`, so `migrated_tool_ids()` is empty and `validate_tool_job_rows` fails closed with
   `interactive-job.catalog-authority`. Every test must use `testkit::new_app_with_registry()`/
   `app_with_registry()` instead once any proof exists (S-END-TO-END lane D changed 180 call sites across 90
   files; PROCESS/SOURCING hit the same thing under a different name).
7. **Watch for the `BatchOnlyPendingRewrite`-vs-silently-`Unclassified` distinction** — a command explicitly
   marked `BatchOnlyPendingRewrite` (not UI-safe, but classified) is a strictly *better* state than one that
   was simply never classified (defaults to `Unclassified`, which panics at construction, not just at
   dispatch).
8. **Verification pattern once fixed** (PROCESS's `route audit` / independent-oracle convention): a
   TypeScript-side test that re-derives `routes=N; migrated=N; bounded=X; resumable=Y; batchOnly=0` from the
   built app definition and asserts `batchOnly=0`/`migrated=N` — run it both before and after any refactor of
   the classification call form (PROCESS's audit had to be updated mid-ticket to read *both* the per-id
   `.action_interactive_job` form and a later peer's blanket `.interactive_jobs(...)` sweep, or it would have
   silently reported green over a dead surface).

---

## 6. Descriptor regeneration + registry check + example staging

- **What `check` complains about**: a *missing* descriptor pair is only a **warning**; a *present-but-wrong*
  one (stale `packageId`, stale app ids, wrong `interactiveJob` classification vs source) is now a **hard
  error** under the fail-closed `validateCatalogDescriptorPair`. Every sibling plugin's committed descriptor
  was found stale in exactly this way: DRAW's said `s.draw.draw@1/*#editor` vs source `s.draw.drawing`;
  RASTER's served wasm sha256 didn't even match its own registry-recorded hash (3+ days stale); REMODEL's
  still carried pre-rename `3d.remodel`/`s.remodel.remodel@1/*#…` vs current source `3d.remodeling`. **Expect
  energy's own committed descriptor to need a fresh `describe` run after the crate builds — don't trust it
  as-is even if it looks present.**
- **Example staging/switching at runtime**: the react shell's example picker reads `PluginManifest.examples`
  **from the descriptor**, not from the registry's generated catalog scan
  (`ShellHost/🟦️.tsx: activePluginManifest?.examples ?? []`). This means even after fixing example
  *discovery* on disk (BLOCK's coordinator fix: `registryExampleCatalog` now also scans
  `🪆️subsets/<s>/📚️examples/` and `<subset>/{editor,viewer}/📚️examples/`, landed 2026-09-05 13:30), the
  react example switcher shows **zero** examples until `describe` is re-run and repopulates
  `manifest.apps[].examples` in the descriptor itself. Both RASTER and REMODEL flagged this as an open DoD-4
  blocker at read time — **for energy, run `describe` once after every example addition, not just once at
  the end.**
- **Classification-drift audit narrow-vs-bidirectional**: S-END-TO-END lane B measured 742 false positives
  from a bidirectional committed-vs-source classification check vs. 206 genuine findings from a narrow one
  (only flag when the committed value contradicts what the owner's *own* Rust actually declares for that
  exact id). If energy's committed descriptor predates any dispatch work, expect this gate to be silent
  until `describe` is re-run (S-END-TO-END lane D found norm's committed descriptor predated the
  `interactiveJob` field entirely — the drift check was "reported but vacuous" until regeneration).

---

## 7. Pitfalls (bulleted, source ticket cited)

- **Host memory/load saturation is the default state, not an edge case.** All four plugin-shaped sibling
  tickets opened with swap in the 60-67 GB range and load average 100+, from 10-40 concurrent peer
  rustc/cargo processes — and all four deliberately deferred every cargo invocation until the host recovered
  (BLOCK/RASTER/REMODEL/DRAW `📓️status.md`, all four). **Check `vm_stat`/`uptime` before launching any
  cargo command for energy; if swap is critically high, wait or coordinate with peers rather than launching.**
- **`CARGO_PROFILE_WASM_DEV_DEBUG=false` is not optional under load** — BLOCK's stdio wasm pre-warm without it
  stalled 4h18m at 3% CPU / 8.6 GB RSS; with it, RSS fell to 165 MB (BLOCK `📓️status.md`).
- **A queued/stale cargo check can read a tree that no longer exists** — BLOCK's baseline check (exit 101)
  failed on a file a peer renamed *after* the check started; not a real break (BLOCK `📓️status.md`).
- **Served plugin wasm can be weeks stale and sha256-mismatched against its own registry entry** — RASTER's
  served `.core.wasm` was 3+ weeks old; booting cold with `SKIP_PLUGIN_BUILD=1` would silently serve stale
  behavior (RASTER `📓️explore-raster-dev-boot-ts.md` §2).
- **A repo-wide emoji/hash fixture-directory rename breaks `#[path]`/`include_str!`/`asset://` independently
  in any plugin** — hit DRAW, RASTER, and REMODEL separately; REMODEL's was total (102/102 URIs
  unresolvable). Check fixture-path resolution as step zero before trusting any pre-existing test.
- **External/shared framework gates can block every s-plugin wasm build at once**, unrelated to your own
  plugin — BLOCK hit an uncommitted `semio-framework-os-kernel` directory-schema rewrite (E0432) that
  blocked block's build for ~20 min until a peer session fixed it (BLOCK `📓️status.md` 04:10→04:29). If
  energy's wasm build fails with an error in `framework-os-kernel` or `stdio` rather than in energy's own
  crate, **attribute per-crate before assuming it's your bug** (see LOWPOLY's whole §10-16 methodology below).
- **Attribute compile errors by crate, always** (LOWPOLY's converged methodology, `📓️summary.md` §10-16): run
  `cargo check -p <your-crate> --all-targets --message-format short`, then `grep` the error output for which
  crate each error actually originates in. LOWPOLY tracked its own error count across 7+ independent build
  configurations and correctly reported "zero lowpoly-owned errors" for most of the ticket even while the
  overall command exited non-zero, because every single error was in `stdio`'s mid-flight migration. Do the
  same for energy — don't report "energy is broken" from an aggregate exit code without checking which crate
  each error line names.
- **Async/sync trait-impl drift from a blanket codemod hit some plugins, not others** — BLOCK (2125 async
  fns against sync framework traits), DRAW, and REMODEL (123+6 async/sync mismatches) all needed a de-async
  pass; puzzle/procedural/stdio had zero such instances. If energy shows the same `async fn diff/inverse` on
  `MutationKind` or `async fn parse_dsl/print_dsl` on `ArtifactDsl`, treat it as this same repo-wide codemod
  artifact, not a new bug — check the trait definitions are actually sync first.
- **A framework-API drift wave** (`render` must return `Result<ComponentTree>`, `command_from_action`
  DslValue vs JsonValue, `Config`/`Presence` missing `DESCRIPTORS`/`descriptor`, `AppIo::with_ports` now
  async, `ActionDescriptor` helper signature, prelude `Label: From<…>` ambiguity, mutation `ToValue`/E0080) —
  BLOCK's W7a/b/c catalogued this precisely with puzzle as the compiling oracle (60 errors → fixed). Use
  puzzle as your own oracle crate if energy shows the same error shapes.
- **A killed Opus-fleet mid-flight leaves placeholder verification text on disk** — BLOCK lost its entire
  implementer fleet to session limits twice; surviving wave reports had `PLACEHOLDER_CARGO_*`/empty
  verification sections that were never actually compiler-checked. **Never trust a wave report's "PASS"
  claim without a fresh, present-tense command output in the same report.**
- **`bun test <glob>` silently skips `.ts` files without `.test`/`.spec` in the name** — use an explicit
  `./`-prefixed path list instead (BLOCK `📓️w3-io.md` §5.1).
- **A stale test-result/breach cache can predate the very change it claims to cover** — REMODEL's
  `⚡️cache/breaches/testing.json` and case-result cache both predated the commits that created/broke its own
  feature file; "this cache tells you nothing about the current state" (REMODEL
  `📓️explore-tests-oracles.md` §3). Check cache mtimes against your own commits before trusting a green
  cache.
- **`SEMIO_PLUGIN_ONLY` used cold cascades to `plugin.descriptor-unavailable`** for both the target plugin and
  its dependency (stdio) — documented identically in BLOCK/RASTER/DRAW/REMODEL.
- **A registered `describe` nx target does not itself register a `.vscode/launch.json` entry** — `grep -c
  describe .vscode/launch.json` was 0 across all 33 existing plugin `describe` targets at S-END-TO-END read
  time; don't assume one exists.
- **Ticket-folder `🗑️generated/` can be swept mid-run by a peer or by a background subagent** — always log
  live builds to the session scratchpad, copy into `🗑️generated/` only at ticket close (project memory
  `feedback-subagents-sweep-ticket-generated-folder`; DEMONSTRATOR's `build-components.sh` comment states
  this explicitly as the reason it never logs there directly).
- **An "anonymous shard worker error"** (`shard N worker error Event` with no message) can mean the shard
  worker URL points at a dead/stale route serving an HTML SPA fallback instead of JS — check
  content-type on the worker URL if energy's dev boot ever shows this symptom (S-END-TO-END lane G,
  `📓️opus-shard-liveness.md`).
- **macOS background-task QoS throttles builds launched from background tool calls to ~5% CPU** — use the
  `taskpolicy -B` re-application loop in §1.3/§2 if a build looks starved despite the host otherwise having
  headroom.
- **A wedged `sccache` server deadlocks the whole native queue**, and `RUSTC_WRAPPER=""` on your own command
  does not protect you from a peer's still-wedged sccache holding the lock — check `sccache -s` actually
  returns output before trusting a "fast" build (PROCESS/SOURCING `🧪️runtime-verification.md`/`📓️day3-run.md`;
  this was traced as the root cause of that session's `repo`/`semio` MCP CONNECT_TIMEOUT too).
- **0% CPU on a build's parent process is not proof it's stuck** — check for a live `rustc` **child** (still
  working) vs. an idle/defunct `sccache` child or no child at all (genuinely stuck); a killed wrapper process
  can also orphan its `cargo`/`sccache` children onto launchd instead of actually stopping them (same source).
- **Swap-thrash can look like slow progress for hours** — a build with 44 min of real CPU time over 10 hours,
  40 MB RSS against a 431 GB VSZ, is page-thrashing, not compiling; `CARGO_PROFILE_WASM_DEV_DEBUG=false` is
  the fix, not more patience (SOURCING `📓️day3-run.md`).
- **Zero compiler errors + zero dead-code/reachability warnings together means the crate never got past
  macro expansion** — a clean-looking log with no `never constructed`/`unreachable expression` lint output at
  all is a red flag, not a green one; both PROCESS and SOURCING independently rediscovered this (confirms
  project memory `feedback-require-warnings-as-proof-of-typecheck`).
- **A bound port answering `curl /` with HTTP 200 can still mean "no real modules served"** — Vite mid-restart
  serves a 200 with no usable module graph; probe the actual entry module path (e.g. `curl '/🟦️.ts'`), not
  just `/` (PROCESS `🧪️runtime-verification.md`).
- **A `dev` child process has no `--` separator in its argv**, so a naive `pkill -f "…:dev"` can miss it and
  it survives a parent kill, keeping the plugin-build lease held (same source).
- **`error instanceof Error ? error.message : String(error)` on a lifted wasm-guest fault object prints
  `[object Object]`** in the console instead of the real fault — if energy's dispatch failures show up as
  `[object Object]`, that's this bug pattern (`replyError`), not a mysteriously opaque fault; the fix is
  `JSON.stringify` on the non-`Error` branch (SOURCING `🧪️runtime-verification.md`).

---

## 8. Suggested wave plan for energy

Given the DoD (native+wasm check, exhaustive per-mutation fixture+oracle, descriptor+registry, playground
dispatch) and that energy's status.md already records a baseline (`one mutation kind`, `~19-26k lines / 50
modules`, an 8×Sonnet read-only explore fleet already launched covering: mutation vocabulary, mutation
scaffolding recipe, test+oracle infra, engine API+validation, editor UI+boot, build gates+history, oracle
toolchain research, and this sibling-recipes report), the sibling tickets' converged shape suggests:

1. **Explore fleet (already launched, read-only)** — confirm findings converge before dispatching writers.
   BLOCK/RASTER/REMODEL all ran 4-8 parallel Sonnet explorers before any implementer; none deadlocked.
2. **Baseline native + wasm check, private target dir, gated on host memory** (§1.1/§1.2) — run in parallel
   with wave 1, not blocking on it. Every sibling ticket ran this concurrently with exploration.
3. **Dispatch classification wave** (§5) — almost certainly needed; every sibling plugin without exception
   had some or all editor actions un-dispatchable at ticket-open. Do this as one focused wave per app/window
   (structure/zones/simulation), or — if the three apps share a command shape — consider the shared-generic-
   factory pattern (§5.6) instead of three hand-copies.
4. **Mutation-kind + fixture wave(s)**, following the LOWPOLY playbook (§3) — likely the largest wave, since
   energy's DoD explicitly wants "every mutation of the model artifact" exhaustively fixture-tested. Split
   by mutation-kind group if there end up being many (LOWPOLY split 17 mutations across 3 disjoint subtree
   waves — schema, editor, viewer/io — once the underlying value_derive migration blocker cleared).
5. **Oracle wave** — given no EnergyPlus/OpenStudio/Honeybee binary exists on this host, decide early whether
   to (a) install via the available Docker daemon (currently down) or a Python venv, or (b) record a
   structured decline (§4) for the physics oracle while still building a `verified-native-second-implementation`
   Python oracle for the document-level mutations. Don't let this wave block the dispatch/fixture waves.
6. **io honesty wave** — audit every declared import/export format against LOWPOLY's real-vs-honest-stub
   pattern (§3 point 3); likely small if energy's model has few external formats.
7. **Descriptor + registry wave** — run `describe` once dispatch+mutations are stable, then again after any
   later example addition (§6). Cheap; do this last per sub-wave rather than once at the very end, since
   descriptor regeneration is cheap and the example-picker gap (§6) means staleness is directly visible.
8. **Playground/dispatch verification wave** — `verify catalog` + manual console-log walk (§1.9) as the final
   gate, mirroring SOURCING's click-by-click checklist (`✅️end-to-end-checklist.md`) as a template for a
   per-window, per-command verification script specific to energy's three windows.

Sizing note: every sibling ticket that ran a 5-agent-or-larger Opus implementer fleet in parallel got hit by
an Opus session-limit kill at least once (BLOCK twice). Prefer smaller (2-3 concurrent) implementer waves with
frequent, real (not placeholder) verification checkpoints over one large fleet, or explicitly budget for a
mid-wave resume via `SendMessage` after a session-limit reset.
