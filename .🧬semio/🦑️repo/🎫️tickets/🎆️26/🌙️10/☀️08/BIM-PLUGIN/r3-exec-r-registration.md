# r3 exec: r-registration (Wave F, registration part)

Scope: hub composition `🏙️bim`, hub workspace + registration rows, bridge, taxonomy, launch registration, plugin oracle manifest.
All cargo calls went through `🚦️gate.sh r-registration`. Logs: `🗑️generated/r-registration/`.

## Created

- `🌎️hub/🧩️compositions/🏙️bim/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,🦀️.rs}`
  - `semio-hub-bim`, `package = "semio:bim"`, `deployment-directory = "🏙️bim"`, component-kind `plugin`, role `hub`, playground `bim` / `s.bim.model@1/*#editor`, ports react 6302 / wgpu 6402.
  - Ports: 6271/6371 (brief) are taken by `stdio` office extension; highest used react port was 6300 (wgpu 6400). 6302/6402 appear in no composition, no launch file; only unrelated test fixtures mention 6301/6302 as fake URLs.
  - Nx: `@semio-tech/bim-plugin` with `test`, `test-quick`, `test-long`, `test-exhaustive` (no `package.json`: note/shooting/architect compositions have none).
  - Deliberately no `semio-framework-os` dependency: the capability lint (`KNOWN_CAPABILITY_VIOLATIONS`) forbids it for new plugins.
- `🌎️hub/🧩️compositions/🏙️bim/🦀️.rs`: `Plugin::<BimApps>::builder("bim")`, label "BIM" (en and de identical), `package_id semio:bim`, shooting-style `.artifact(model::declaration())` + `.editor::<BimModelApp>(create_bim_app())` + `.viewer::<BimModelViewer>(create_bim_viewer())` (f1's crate exports `declaration()`, not note's `artifact<PA>()`), mutation rosters, `OnArtifactKind`, `Isolated`, `artifacts.write` request.
- `🌎️hub/🧩️compositions/🏙️bim/🧪️tests/🔬️surface/🦀️.rs` (viewer never mutates, editor/viewer dialect, manifest not the assembly-failed stub, 2 apps) and `🧪️tests/🛂️committed-descriptor/🦀️.rs` + `🧫️fixtures/🛂️committed-descriptor/🔣️.json` (copied from note, adapted).
- GENERATED through the describe pipeline (not hand-written): `🌎️hub/🧩️compositions/🏙️bim/🔣️.json`, `🛂️.descriptor.semio` (wasm sha256 ec1a4cb0..., descriptor sha256 9509edc1...). `dist/component-dev/semio_hub_bim.wasm` is built (gitignored).
- `✏️s/🔌️plugins/🏙️bim/🏭️bridge/{Cargo.toml,🦀️.rs,📜️script.ts,Cargo.lock}`: bin `semio-bim-mutation-bridge`; AGGREGATES = `ModelMutation` only (editor and viewer use `NoConfig`/`NoPresence`, there is no config or presence aggregate yet; re-add rows when f1/UI wave adds them); COORDINATES = `s.bim.model 1 any ""`. `📜️script.ts` adds `maxBuffer: 1 << 28` (the shooting variant dies with `status null` once cargo replays more than 1 MB of warnings).
- `✏️s/🔌️plugins/🏙️bim/🔮️oracles/🔣️.json`: created as schemaVersion 2 mirror of gis/draw; the o-oracles agent has since filled `oracleHostPackages` (shapely 2.1.2, ifcopenshell 0.8.4.post1). Left as found.

## Updated (surgical edits)

- `🌎️hub/Cargo.toml`: member `🧩️compositions/🏙️bim/📦️packages/🦀️rust`, `[workspace.dependencies] semio-s-artifact-bim-model`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `🏙️bim` in `members-of-plugins` and in `deployed-module-members` (dev plugin-modules output dirs), `🏢️model` in `members-of-artifacts`.
- Further registration sites found by following `semio-hub-architect`/`note` through `git grep`:
  - `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: `TRUSTED_BOOTSTRAP_ALL_PACKAGES` (+`bim`) and the trusted plugin row `bim`.
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` `PACKAGE_TEST_BUDGET_MS` `semio-hub-bim` quick 1_800_000, mirrored in `🧫️fixtures/⏱️test-level-budgets/🔣️.json`.
  - `🌎️hub/.config/nextest.toml`: quick-profile slow-timeout override for `semio-hub-bim` (1800 s).
  - `🌎️hub/🧩️compositions/🧫️fixtures/📇️ownership/🔣️.json`: `semio-hub-bim` ownership row (requires the generated `🛂️.descriptor.semio`, now present).
- Peer breakage fixed because it blocked the describe-tool rebuild (trivial, 6 one-word edits): `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/{🌗️set-appearance,📐️set-layout,📖️set-terminology,🕹️set-driver,🖼️set-theme,🗣️set-locale}/🦀️.rs` inverse built `UiPreferencesConfigMutation::Appearance(..)` etc. but the enum variants are `SetAppearance(..)` etc. (E0599 x6 in a standalone build of `semio-framework-os-config`). Renamed to `Set*`.

## Not done, with reason

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`: do NOT exist in the working tree (deleted, `D` in git status) by the DASHBOARD-LAUNCH-COCKPIT migration, which makes the registry (Cargo `playground` metadata + discovered Nx targets) the launch source. Recreating them would resurrect files the goal deletes, so no `🛠️dev🏙️bim🏢️model*`, test/check rows and no `bim-react` preview entry were written. The playground block in the hub `Cargo.toml` is the declaration the dashboard reads. No `metadata.semio.dashboard` is needed: the migration declares only special parameters/ready ports on a few manifests, never on plugin composition manifests (plugin test targets are "discovered Nx targets").
  AGENTS.md still says "register in launch.json"; the owner must reconcile (AGENTS.md may not be edited by agents).
- Root `📜️script.ts` / `📋️project.json`: shooting/note/architect have no per-plugin rows there that every plugin needs; nothing added.
- `🔒️dependencies.json`: not regenerated (hub deps are all already-used third-party crates; the registry file lists manifest users, regeneration is a repo-wide write-baseline owned by the lead).
- `bun nx run @semio-tech/plugin-registry:check` / `:generate`: Nx cannot build its project graph at the moment (`Duplicate Nx project workspace: . and 🧰️framework/.../🎛️dashboard/🧫️fixtures/🧭️journeys/🏗️workspace`, plus a generator-output-without-producer error; not mine). Ran the same script directly (`bun ./📜️script.ts check` in `.../🔌️plugin/📇️registry`): it fails BEFORE and AFTER bim for an unrelated reason, `stale-channel descriptors refused (host app channel 23)` for 67 plugins (every committed descriptor except bim speaks an older channel). `generate` was not run (it would rewrite all registry outputs and throws on the stale set).
  Proof that bim is admitted: `r3-r-registration-registry-row.ts` (`staleChannel: "exclude"`) prints the `bim` row (pluginId bim, packageId semio:bim, cratePath hub/compositions/bim/packages/rust, activation `on-artifact-kind:s.bim.model` and `...3d.bim-model`, `artifacts.write`, isolated, hashes) and `admitted=3 stale=67`.

## Verification (commands and results)

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --message-format=short` | PASS (3m18s, f1's state at 05:40) |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --message-format=short` | PASS |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --lib --target wasm32-wasip2` | PASS |
| `bun .../cargo/📜️script.ts native component dev --manifest <hub bim Cargo.toml>` (inferred `component-dev`) | PASS, `dist/component-dev/semio_hub_bim.wasm` |
| describe tool rebuild (`bun ./📜️script.ts build` in `🖨️describe/📦️packages/🦀️rust`) | PASS after the os-config fix above (failed before: E0599 x6) |
| `bun .../🖨️describe/📦️packages/🦀️rust/📜️script.ts component --manifest <hub bim Cargo.toml>` | PASS; first attempt failed `plugin.channel-mismatch: guest 23, host 21` (stale describe binary), passes with the rebuilt tool |
| `bun ✏️s/🔌️plugins/🏙️bim/🏭️bridge/📜️script.ts list-mutations s.bim.model 1 any` | PASS, 12 mutations: create/delete-site, create/delete-building, create-storey, rename-storey, set-storey-height, set-storey-level, delete-storey, create/delete-wall, set-wall-top (matches `KINDS` of f1) |
| `cargo test --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --lib` | NOT RUN TO COMPLETION: the dev-dependency graph (`semio-framework-os-run` -> stdio) fails in `semio-s-artifact-stdio-contract` (E0061: `MutationDiff::apply` now needs `ApplyCapability`; the diff-only stdio agent is mid-conversion). The surface and committed-descriptor tests are therefore WRITTEN BUT UNVERIFIED; re-run when stdio compiles. |
| `bun ./📜️script.ts verify taxonomy report` | see "Taxonomy scan" below |

## Taxonomy scan

`verify taxonomy report` (repo-wide, 94k directories, 138k files, then a 112k-reference plan phase) was started and stopped by me in the plan phase after about 40 minutes under heavy machine load: it had inventoried all bim paths (844 progress lines name `🏙️bim`/`🏢️model`) and printed no finding, but it did NOT finish, so taxonomy conformance of the bim tree is UNVERIFIED by the gate. Log: `🗑️generated/r-registration/verify-taxonomy-2.txt`.

## Open issues for the lead

1. Hub unit tests of `semio-hub-bim` need the stdio contract crate to compile again (peer work); rerun `cargo test ... -p semio-hub-bim --lib` and `bun nx run @semio-tech/bim-plugin:test-quick` when Nx is healthy.
2. `🏙️` is also used by `deployment-directory = "🏙️process-extension-concrete"` (process concrete extension). They are only siblings inside the gitignored `dist/dev/🔌️plugin-modules/`, so the git-visible emoji uniqueness policy is not violated, but the two deployment names share an emoji.
3. When the editor/viewer gain config or presence aggregates, add them to the bridge `AGGREGATES` and the `✏️editor/🎚️config`, `✏️editor/👥️presence` COORDINATES.
4. If the stdio export/import codecs that f1/IO wave links need linking into the hub (note links dwg/dxf/pdf/png/semio/svg), add them to the hub `Cargo.toml` then and regenerate the descriptor (`component dev` + `describe component`).
5. Launch registration is moot until the owner decides between AGENTS.md (launch.json) and the DASHBOARD-LAUNCH-COCKPIT deletion.
