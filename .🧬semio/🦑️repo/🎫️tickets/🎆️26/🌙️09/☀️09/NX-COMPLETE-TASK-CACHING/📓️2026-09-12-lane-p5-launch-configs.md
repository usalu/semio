# Lane P5 — Launch Configs Through Nx (2026-09-12)

Scope: `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (schema-owned seed the launch validator
diffs against), `.claude/launch.json`. Goal: every launch entry that builds/tests/checks/serves repo
code goes through a cached Nx target — no raw `cargo`, no direct `bun ./…/📜️script.ts <cmd>` bypassing
Nx (ticket-scoped ad-hoc probes excepted).

Note on file ownership: `.vscode/launch.json`/`🧩️launch.seed.jsonc` are live, shared, concurrently-edited
files (other lanes/sessions add entries while this lane worked — three unrelated new `🛠️dev…` configs
appeared mid-session, see Verification). Only the rows in the tables below were touched by this lane;
everything else in these files is untouched.

## `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` — raw `cargo` → Nx (10 entries, identical edit in both files)

| config name | before | after | target |
| --- | --- | --- | --- |
| `🛠️dev🔺️trinity🃏️jack🦀️shell` | `cargo run -p trinity_jack_shell -- trinity/fixture/nakagin-capsule-tower.trinity.json "MATCH (a:Piece) RETURN a.name"` | `bun nx run @semio-tech/trinity-jack-shell:run -- trinity/fixture/nakagin-capsule-tower.trinity.json "MATCH (a:Piece) RETURN a.name"` | new `run` target (depends on the crate's existing auto-generated cached `build` target) |
| `⚖️gate📐️cad🧠️brep-invoke🦀️check` | `cargo check -p semio-framework-os-flow --lib --message-format short` | `bun nx run semio-framework-os-flow-core:check --args="-p semio-framework-os-flow --lib --message-format short"` | existing cached `check` target (needed `forwardAllArgs: true` added) |
| `🏛️bestest🔋️energy♻️regenerate` | `SEMIO_ENERGY_BESTEST_REGENERATE=1 cargo test -p semio-s-plugin-energy --lib bestest::tests::regenerate_bestest_fixtures -- --nocapture` | `bun nx run @semio-tech/energy-model-rs:test-bestest-regenerate` | new dedicated generator target, declared outputs, `cache: true` |
| `🧪️test⏱️brep🧊️tessellation-jobs` | `cargo test -p semio-s-artifact-stdio-semio --features conversion-brep --test brep_tessellation_jobs -- --nocapture --test-threads 1` | `bun nx run @semio-tech/stdio-semio-rs:test --args="--features conversion-brep --test brep_tessellation_jobs -- --nocapture"` | existing cached `test` target |
| `🧪️test📦️brep🧊️extrude-orientation` | `cargo test -p semio-s-artifact-stdio-semio --test brep_extrude_orientation -- --nocapture --test-threads 1` | `bun nx run @semio-tech/stdio-semio-rs:test --args="--test brep_extrude_orientation -- --nocapture"` | existing cached `test` target |
| `🧪️test🎨️brep🧊️analytic-blend` | `cargo test -p semio-s-artifact-stdio-semio --test brep_analytic_blend -- --nocapture --test-threads 1` | `bun nx run @semio-tech/stdio-semio-rs:test --args="--test brep_analytic_blend -- --nocapture"` | existing cached `test` target |
| `🧪️test⏱️procedural🧊️boot-deadline` | `cargo test -p semio-s-plugin-procedural --test boot_deadline -- --nocapture --test-threads 1` | `bun nx run @semio-tech/procedural-plugin:test --args="--test boot_deadline -- --nocapture"` | existing cached `test` target |
| `🧪️test🚪️procedural🧊️close-ladder` | `cargo test -p semio-s-plugin-procedural --test close_ladder -- --nocapture --test-threads 1` | `bun nx run @semio-tech/procedural-plugin:test --args="--test close_ladder -- --nocapture"` | existing cached `test` target |
| `🧪️test🎒️flow🧊️mesh-pack-wire` | `cargo test -p semio-framework-os-flow --test flow_mesh_pack_wire -- --nocapture --test-threads 1` | `bun nx run semio-framework-os-flow-core:test --args="--test flow_mesh_pack_wire -- --nocapture"` | existing cached `test` target (needed `forwardAllArgs: true` added) |
| `🏛️bestest🔋️energy🧪️laws` | `cargo test -p semio-s-plugin-energy --lib bestest:: -- --nocapture` | `bun nx run @semio-tech/energy-model-rs:test --args="--lib bestest:: -- --nocapture"` | existing cached `test` target |

Two corrections made while wiring these (both are launch-config bugs, not caching design choices):

- **Wrong package.** The two `🏛️bestest🔋️energy…` entries pointed at `-p semio-s-plugin-energy`, whose
  crate has no `bestest` module — the real ASHRAE BESTEST test tree (`mod bestest`, `#[path]`-included)
  lives in `semio-s-artifact-energy-model` (project `@semio-tech/energy-model-rs`). Verified: `cargo test
  -p semio-s-plugin-energy --lib bestest::` runs **0 tests** (4 filtered out, none matched); the same
  filter against `semio-s-artifact-energy-model` runs **19** (confirms the crate). Both entries now target
  `@semio-tech/energy-model-rs`. Surfaces a pre-existing, unrelated red: 5 of those 19 tests currently fail
  with `"energy simulation fault"` (`annual_run_is_deterministic`, `committed_example_assets_match_the_builders`,
  `controlled_case_holds_its_setpoints`, `free_float_case_delivers_no_hvac_energy`,
  `shading_never_increases_cooling`) — a physics/domain bug, out of this lane's scope; not fixed here.
- **`--test-threads 1` breaks the wrapped test runner.** All 6 rewritten `test` commands originally
  carried `--test-threads 1`. The repo's `runCargoTestBudgeted` prefers `cargo-nextest` when installed
  (it is), and nextest rejects that flag post-`--`: `error: failed to parse test binary arguments
  "--test-threads": arguments are unsupported`, which made the Nx task fail every time (and, since Nx
  never caches a failed task, made it look permanently uncached). Dropped `--test-threads 1` from all 6
  (the wrapper already manages its own thread count via `SEMIO_TEST_LEVEL`); confirmed both rewired tests
  now pass. This one bug would have silently defeated every one of these 6 conversions had it shipped.

**Cache-hash gotcha found and applied:** `bun nx run <project>:<target> -- <args>` (bare `--`) forwards
args to the underlying command correctly but is **not** reliably included in Nx's task hash for this
repo's `nx:run-commands` targets — two back-to-back runs with identical bare-`--` args did not converge
on a cache hit even with zero repo churn in between. `--args="<args>"` (the form already used elsewhere
in this same file, e.g. `@semio-tech/cad-js:test --args='-- -t spatial-kernel/semio'`) does hash
correctly and hits cache. All 8 args-carrying conversions above use `--args="…"`; the one entry left with
bare `--` (`trinity-jack-shell:run`) is a `cache: false` interactive one-shot, so this doesn't apply.

## `.claude/launch.json` — direct `bun ./📜️script.ts dev <x>` → Nx (9 entries)

Root `📜️script.ts`'s `DevScript` resolves any bare `dev <tokens>` into `bun nx run
@semio-tech/framework-os-dev:dev -- <plugin> …rest` plus an env computed by
`frameworkOsPlaygroundDevEnv` (`SEMIO_RENDERER` defaults differently per plugin/mode) — and the root
`📜️script.ts` router itself is auto-exposed as an Nx target (`rootCommandTargets` in the caching `.mjs`
plugin turns every `.register("name", …)` on the root router, including `.register("dev", DevScript)`,
into `workspace:<name>` with `forwardAllArgs: true`; confirmed via `bunx nx show project workspace
--json` → `dev: { command: "bun ./📜️script.ts dev", forwardAllArgs: true, cache: false, continuous: true
}`). Routing through `workspace:dev -- <tokens>` is therefore the exact, byte-identical equivalent of the
old direct invocation (same code path, same env resolution) rather than a guess at which renderer/env a
given plugin defaults to — safer than reimplementing that logic against the `@semio-tech/framework-os-dev`
project's own specialized `dev-<variant>-react-dev`/`serve-<variant>-react-dev` targets, which exist for
some but not all of these plugins and could silently change renderer behavior.

| config | before (`runtimeArgs`) | after (`runtimeArgs`) |
| --- | --- | --- |
| `cad-react` | `["./📜️script.ts","dev","cad"]` | `["nx","run","workspace:dev","--","cad"]` |
| `s-react` | `["./📜️script.ts","dev","s"]` | `["nx","run","workspace:dev","--","s"]` |
| `s-react-served` | `["./📜️script.ts","dev","s","served"]` | `["nx","run","workspace:dev","--","s","served"]` |
| `dag-react` | `["./📜️script.ts","dev","dag"]` | `["nx","run","workspace:dev","--","dag"]` |
| `storybook-framework-hosts` | `["./📜️script.ts","dev","storybook","framework","hosts"]` | `["nx","run","workspace:dev","--","storybook","framework","hosts"]` |
| `gis2d-wgpu` | `["./📜️script.ts","dev","gis","2d"]` | `["nx","run","workspace:dev","--","gis","2d"]` |
| `puzzle2d-react` | `["./📜️script.ts","dev","2d"]` | `["nx","run","workspace:dev","--","2d"]` |
| `storybook-static` | `["./📜️script.ts","dev","storybook-static"]` | `["nx","run","workspace:dev","--","storybook-static"]` |
| `storybook-framework-os` | `["./📜️script.ts","dev","storybook","framework","os"]` | `["nx","run","workspace:dev","--","storybook","framework","os"]` |

These are continuous dev servers (`cache: false`, `continuous: true` by design) — going through
`workspace:dev` does not itself get cached (correct; a live dev server has no cacheable output), but the
command now runs through the Nx graph rather than bypassing it entirely, and the underlying build/activate
steps `DevScript` triggers are already Nx-cached targets (`prepare-*`/`activate-*`, owned by lane P2/P3).

The other 11 `.claude/launch.json` entries were left untouched: 7 already ran `bun nx run …`
(`procedural3d-react/wgpu`, `puzzle3d-react/wgpu`, the 3 `mit-bestand-demonstrator*` variants), 2 are
plain `url` attach configs with no command, and 2 are orphaned scripts outside any Nx project — see below.

## Nx targets added / changed (project.json)

| project | file | change |
| --- | --- | --- |
| `@semio-tech/trinity-jack-shell` | `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📋️project.json` | new `run` target: `cache: false`, `dependsOn: ["build"]` (the crate's existing auto-generated cached `build` target — see `cargoTargets()` in the caching `.mjs` plugin, which gives every `[package]`-bearing `Cargo.toml` a free cached `build`/`check`/`test` unless the project overrides it), `command: "bun ./📜️script.ts run"`, `forwardAllArgs: true` |
| (same) | `…/📦️packages/🦀️rust/📜️script.ts` | new `RunScript`: execs the `build` target's staged binary (`{projectRoot}/dist/build/semio-s-plugin-trinity-jack-shell[.exe]`) via `Bun.spawnSync` with inherited stdio — never `cargo run` (no wrapper process to chase), matching the `os-hub`/`buildHubBinary` convention already used elsewhere in this repo |
| `@semio-tech/energy-model-rs` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📋️project.json` | new `test-bestest-regenerate` target: `cache: true`, `env: { SEMIO_ENERGY_BESTEST_REGENERATE: "1" }`, `outputs` = the two directories the generator writes (`…/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧫️fixtures,🖼️assets}`) |
| `semio-framework-os-flow-core` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json` | added `"forwardAllArgs": true` to the existing `test` target (was missing; needed so the `flow_mesh_pack_wire` launch entry's filter reaches `cargo test`) |

`@semio-tech/stdio-semio-rs`, `@semio-tech/procedural-plugin`, `@semio-tech/energy-model-rs`'s plain
`test` target already had `forwardAllArgs: true` — no project.json change needed for those 4 conversions.

## Found (not fixed — out of lane P5's scope): `trinity_jack_shell` crate is currently broken

Building the crate underlying the trinity dev-shell entry (`cargo build -p
semio-s-plugin-trinity-jack-shell` — note the launch entry's original `-p trinity_jack_shell` never
matched any real Cargo package name either) fails: `error[E0432]: unresolved import trinity::executor` at
`📦️bin.rs:10` — no `executor` module exists in the `trinity` crate. The entry's example args also
reference a `trinity/fixture/nakagin-capsule-tower.trinity.json` fixture that does not exist anywhere in
the repo (searched for any `*.trinity.json` file — none). This entry was already fully non-functional
before this lane touched it; the new `build`/`run` wiring is correct and will work once the crate compiles
and a real fixture path is supplied, but neither of those pre-existing bugs is caching-related, so they
were not fixed here. Flagged as a follow-up task (`task_082e929e`).

## Left as-is (with reasons)

- **69 ticket-scoped `bun nx exec --projects=workspace[…] -- bun "<ticket>/…/📜️script.ts" …` probes** in
  `.vscode/launch.json` — one-off investigations under `.🧬semio/🦑️repo/🎫️tickets/**`, not package steps.
  Grouped by owning ticket:
  - `SEMANTIC-MUTATIONS-OVERHAUL` (2026/08/12): 57 entries
  - `END-TO-END-TAXONOMY-NORMALIZATION` (2026/08/17): 9 entries
  - `ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` (2026/08/17): 2 entries (`laws`, `laws-fleet`)
  - `FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG` (2026/08/17): 1 entry (`--projects=@semio-tech/framework-actor`)
  These already go through `nx exec` (not a raw script call) and are explicitly out of scope per the brief.
- **12 `bun -e 'const m = await import("./📜️script.ts"); const b = m.policy…Breaches(...)…'` policy
  debug launchers** in `.vscode/launch.json` (`policyArtifactBuilderBreaches`,
  `policyArtifactDecomposerBreaches`, `policySchemaRepresentationBreaches`,
  `policyIoSerializerMatrixBreaches`, `policyIoTerminalityBreaches`, `policyCodecFidelityBreaches`,
  `policyStandardsCoverageBreaches`, `policyArtifactAnalyzerBreaches`, `policyArtifactComposerBreaches`,
  `policyArtifactBuilderMigratedBreaches`, `policyPluginDependencyParityBreaches`,
  `policyContributionTargetBreaches`). Root `📜️script.ts` only exposes the *whole* policy sweep as a CLI
  command (`bun ./📜️script.ts policy`, dispatched by the shared `dispatchPolicyArgv`) — there is no
  existing per-rule CLI surface to route an individual breach function through Nx. Exposing one would mean
  adding a new subcommand/dispatch branch to root `📜️script.ts`, which is outside `.vscode/**`/
  `.claude/launch.json`/"add a missing target to a project.json" — it's a change to the shared root script
  that every other phase-2 lane also touches. Left as-is; flagged here for a follow-up (root-script owner)
  rather than fixed by this lane.
- **3 `bun ./📜️script.ts render` / `preview` / `flush-cache` entries** (`🎥️render🎬️animate-video`,
  `👁️preview🎬️animate-video`, `🧹️flush-cache🎬️animate-video`) with `cwd:
  "${workspaceFolder}/animate/video/rs"` — that directory does not exist in the repo at all (`find` finds
  nothing under `animate/`), and root `📜️script.ts`'s router has no `render`/`preview`/`flush-cache`
  registration either (`rootCommandTargets` would have exposed them as `workspace:render` etc. if it did —
  confirmed absent via `bunx nx show project workspace --json`). Dead/orphaned entries from a removed
  feature; nothing to route through Nx since there is no source. Left as-is rather than fabricated.
- **`.claude/launch.json` `terra-jco-spike-static`** — runs a package-local
  `🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/📜️script.ts` directly; no `📋️project.json`
  exists anywhere from that directory up to `🧰️framework/🛍️products/💻️os` (confirmed). Its own name
  ("spike") and the taxonomy's separate `🧫️fixtures/🔌️jcoprobe` path (vs. this `🧪️testkit/🧩️jcoprobe`)
  suggest an ungoverned experimental harness outside the Nx project graph. Creating a whole new Nx project
  for a one-off spike is scaffolding, not the "small additive edit" this lane's file ownership allows —
  left as-is, listed here.
- **`.claude/launch.json` `map-harness`** — runs
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/📜️harnessscript.ts` directly: a
  ticket-scoped script, same category as the 69 `nx exec` probes above (one-off investigation, not a
  package step). Left as-is, listed here.

## Verification

- **JSONC/JSON validity + config counts**, comments/trailing commas stripped via `Bun.JSONC.parse`
  (matches the S6 report's method):
  - `.vscode/launch.json`: parses; 2482 configs at the ticket's baseline (`git show HEAD:…`) →
    2484 now. The +2 are **not this lane's**: two new `🛠️dev🔧️procedural🏙️3d👁️viewer…` configs and one
    `⚖️gate🖱️world3d-interaction🌐️renderer` config appeared mid-session from other concurrent
    work — confirmed via `git diff` (their whole config blocks are pure additions, not touching any line
    this lane edited) and cross-checked against `.vscode/🧩️launch.seed.jsonc`, which picked up the same
    two `🛠️dev` additions (1392 → 1394) but not the gate one (seed doesn't carry `⚖️gate…` entries).
  - `.claude/launch.json`: parses (strict `JSON.parse`); 20 configs, unchanged.
- **Every rewritten command's Nx target exists**, confirmed via `NX_DAEMON=false bunx nx show project
  <p> --json` for `workspace`, `@semio-tech/trinity-jack-shell`, `@semio-tech/stdio-semio-rs`,
  `@semio-tech/procedural-plugin`, `@semio-tech/energy-model-rs`, `semio-framework-os-flow-core`,
  `@semio-tech/framework-os-dev` (target list also confirmed the `dev-<variant>-react-dev` /
  `serve-<variant>-react-dev` naming already used by the two pre-existing `.claude/launch.json` react
  entries, per `playgroundPreparationTargets()` in the caching `.mjs` plugin).
- **Two rewritten entries run twice, `[local cache]`-equivalent hit recorded on the second run**
  (`nx:run-commands` prints "Nx read the output from the cache instead of running the command" rather
  than a literal `[local cache]` tag, which is what the `[local cache]` tag on *dependency* tasks in the
  same run also means):
  - `bun nx run @semio-tech/stdio-semio-rs:test --args="--test brep_extrude_orientation -- --nocapture"`
    — 1st run: 1m 47s, 1/5 tasks cached, 12/12 tests pass. 2nd run: 1.2s, **5/5 tasks cached (100%)**,
    "Nx read the output from the cache instead of running the command for 5 out of 5 tasks."
  - `bun nx run @semio-tech/procedural-plugin:test --args="--test boot_deadline -- --nocapture"` — 1st
    run: 1m 31s, 4/5 cached, 1/1 test passes. 2nd run: 885ms, **5/5 tasks cached (100%)**.
  - (`semio-framework-os-flow-core:check`/`:test` were also exercised repeatedly during debugging but
    never converged on a cache hit — that project's `^default` transitive-input closure includes
    `🌊️flow/🖥️host`, `🌊️flow/📔️registry`, `🌊️flow/🌉️bridge` etc., which another concurrent lane was
    actively editing throughout this session (confirmed: `git status` shows those exact files modified,
    live, by someone else). Not a defect in this lane's wiring — verified separately that the *same*
    target caches cleanly with zero args when nothing concurrent touches its inputs.)
- **Launch validator** (`bun ./📜️script.ts verify interactivity apps`, found via
  `INTERACTIVITY_ALL_APP_LAUNCH_FILE`/`INTERACTIVITY_ALL_APP_LAUNCH_SEED_FILE` in `📜️script.ts`) — ran
  it; it currently fails with 782 pre-existing, unrelated findings: a stale `512`-configuration capacity
  constant (both files have carried 2000+ configs for a long time per this same ticket's own inventory —
  not something this lane's edits changed the count class of), 6 missing exact `⚖️gate⚡️interactivity*`
  registrations, and hundreds of malformed plugin descriptor files (e.g. `✏️s/🔌️plugins/✒️writer/**`
  missing `descriptorVersion`/`manifest`/`role`). None of the 782 failure lines reference any launch name
  or file this lane touched. Cannot claim a pass — reporting the failure honestly rather than treating it
  as green; it was already red before this lane started and needs its own (unrelated) cleanup lane.
- **`rg` sweep**: `rg '"command":\s*"[^"]*\bcargo (run|check|test)\b"' .vscode/launch.json
  .vscode/🧩️launch.seed.jsonc` → 0 matches (was 10 in each). The three hub headless-Stdio gate entries
  that keep a private `CARGO_TARGET_DIR` (review fix 5) were left untouched — they already run through
  `bun nx run os-hub:…`, never called raw `cargo`.

## Files touched by this lane

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — 10 raw-`cargo` command rewrites each (identical).
- `.claude/launch.json` — 9 direct-`dev` command rewrites.
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📋️project.json` — new `run` target.
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📜️script.ts` — new `RunScript`.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📋️project.json` — new
  `test-bestest-regenerate` target.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json` — added
  `forwardAllArgs: true` to `test`.

Not deleted (input scripts kept per ticket rules): `/private/tmp/…/scratchpad` working files used during
this lane's investigation are scratch, not ticket-folder inputs, and were not copied into
`🗑️generated/p5/` since no generated/log artifact needed retention beyond this report (all verification
output is quoted inline above).
