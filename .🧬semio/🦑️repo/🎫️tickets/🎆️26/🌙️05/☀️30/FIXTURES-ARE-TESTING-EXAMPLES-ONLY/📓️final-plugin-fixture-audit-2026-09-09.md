# Final Plugin Fixture Audit — 2026-09-09

## Result

The bounded plugin-source audit found four live stale readers that already have a canonical `🧫️fixtures` replacement and eight present test corpora outside their canonical owner roots. No inspected plugin runtime, component build, descriptor, manifest, or catalog writer copies fixture bytes into a published product. The Hub GIS cold-start gate is a qualification dependency, but it is not a production-data dependency: the fixture is compiled into a Cargo test executable and never becomes catalog or component content.

This report supplements `📓️plugins-fixture-separation-2026-09-09.md` and `📓️final-repo-production-fixture-audit-2026-09-09.md`. It deliberately does not repeat the root 114,306-reference scan or the 9,057-case plugin resolver run.

## Live Stale Fixture Readers

| Priority | Reader and evidence | Actual canonical owner | Observed behavior | Required correction |
| --- | --- | --- | --- | --- |
| P1 | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:380-381` reads `🧫️fixture/🔣️.json`; `:443` reads `🧫️fixture/♻️lifecycle/🔣️.json`. The route is registered at `:1010`. | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🧫️fixtures/🔣️.json` and `🧫️fixtures/♻️lifecycle/🔣️.json`. | `bun ./📜️script.ts flow-retained-decode-check --oracle-only` failed with `ENOENT` at line 381 for the singular directory. | Change both reader segments to plural `🧫️fixtures`; do not restore a singular compatibility alias. |
| P1 | `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:241-243` reads a JSON fixture from `🧪️tests/🌉️component-cold-map-patch`; `:288` then reads Rust source from that same root. Check routes are `:336-344` and `:511-512`. | `✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🌉️component-cold-map-patch/🔣️.json`; the Rust test remains `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`. | `bun ./📜️script.ts component-cold-map-patch-check` failed with `ENOENT` at line 243. | Split the two roots: load JSON from the canonical fixture owner and Rust source from the test-case directory. Changing only `fixtureRoot` would make line 288 seek a Rust file in `🧫️fixtures`. |
| P1 | `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:125-130` reads `🎚️config/🧪️tests/🔣️.json`; the explicit check route is `:276`. | `✏️s/🔌️plugins/📕️norm/🎚️config/🧫️fixtures/🔣️.json`. | `bun ./📜️script.ts config-mutation-source` failed with `ENOENT` at line 129. | Read the canonical config-owner fixture path. |
| P1 | `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📜️script.ts:128-133` reads `🗿️artifacts/🧊️process3d/🧪️tests/⚖️retained-route-laws.json`; `:175` makes it part of default `test`. | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🧫️fixtures/⚖️retained-route-laws.json`. | The two example tests passed, then the default command failed with `ENOENT` at line 133. | Read the artifact-owner `🧫️fixtures` path. |

Each fault is test/check-only. None is a component build input or a product catalog member; the correction is a direct reader update, with no fixture promotion to `🖼️assets`.

## Noncanonical Test Corpora That Still Need an Owner Move

The following files exist and are read as semantic test oracles. Their consumers are either `test` or an explicit audit/check route, not a production component or descriptor route.

| Current test corpus | Reader evidence | Canonical destination shape |
| --- | --- | --- |
| `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:70`, default test at `:135` | `💠️lowpoly/🧫️fixtures/🧪️interactive-job/🔣️.json` |
| `✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:77-87`, routes `:102` | `🧱️block/🧫️fixtures/🧪️publication-authority/🔣️.json` |
| `✏️s/🔌️plugins/🖍️draw/🧪️publication-authority/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:82-92`, routes `:107` | `🖍️draw/🧫️fixtures/🧪️publication-authority/🔣️.json` |
| `✏️s/🔌️plugins/➗️mathematical/📣️publication-authority/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:32-41`, routes `:50` | `➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json` |
| `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:295-343`, routes `:349` | `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json` | `📦️packages/🟦️typescript/📜️script.ts:80-85`; explicit `retained-audit` at `:147` | the same artifact owner's `🧫️fixtures/🗄️retained-jobs/🔣️.json` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🧭️wiring-fixture/🗂️subset-directory-wiring/🔣️.json` | `📜️script.ts:591-608`; check/generate route at `:1010` | the package owner's `🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring/🔣️.json` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📇️mutation-leaf-taxonomy-v1.json` | generated at `📜️script.ts:238-242`, read as a fixture at `:246-267`, routes `:279-280` | the Norm plugin's `🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` |

These are test examples and check baselines, so `🖼️assets` would misclassify them as shipped static data. Move each atomically with its reader/generator, keeping the schema where it is. The move should not leave aliases at the old paths.

`✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/🔣️.json` is a further physical noncanonical corpus. No reader was found in this bounded source-role pass, so it is not counted as a live source edge; its semantic owner should nevertheless either move it under `🧫️fixtures` or remove it after a separate ownership decision.

## Hub GIS Cold Bootstrap: Qualification Gate, Not Production Fixture Data

The prior audit correctly observed that a cold Hub bootstrap cannot publish until the GIS test succeeds. Calling that a production *data* dependency conflates the gate with a catalog-content edge.

1. `🌎️hub/📦️packages/🦀️rust/📜️script.ts:12570-12578` materializes a missing trusted catalog, then calls candidate validation before it starts the developer Hub.
2. Materialization builds fresh GIS and Stdio components (`:9981-10035`), then serializes only component, descriptor, browser-actor, codec, dependency, and target metadata into `trusted-catalog.json` (`:10097-10118`). It verifies and renames that staged generation at `:10123-10146`.
3. Validation invokes `proveTrustedGisColdMapComponentV1` before staging and starting the candidate Hub (`:10896-10914`). That function validates the retained component and descriptor paths/hashes (`:10183-10218`) and runs Cargo test target `component_cold_map_patch` against those exact component bytes via environment variables (`:10219-10247`).
4. Cargo declares that test as a test target, not a component source, in `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:66-68`; its host/scene support is under `dev-dependencies` at `:90-97`.
5. The test's `include_str!` embeds the canonical fixture only into the test executable (`✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs:26-28`). It turns `document.before` into an ephemeral pack/SPR input, sends that to the tested component, and dispatches `document.after` (`:255-270`). The stale-authority test likewise derives only an ephemeral cold page (`:272-283`). No test code writes fixture bytes to the staged generation.
6. The candidate test must pass before `publishTrustedBootstrapCurrent` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:10908-10914`). The production catalog therefore contains the prebuilt, receipt-verified component, never the fixture JSON.

The correct classification is **test qualification input with publication-gate influence**, not **production static data**. This is legitimate fixture use: qualification tests may decide whether a candidate is safe to publish. Promoting the JSON to `🖼️assets` would falsely describe it as shipped content and would not remove the gate. If policy intends to prohibit even a test gate from using fixtures, that is a separate release-policy decision requiring removal of the qualification test; it cannot be fixed by relocating its bytes.

## Cleared Source Roles and Production Edges

- Canonical, test-only readers were confirmed in Norm (`📕️norm/📦️packages/🟦️typescript/📜️script.ts:24-109`), Flow (`🌊️flow/📦️packages/🦀️rust/📜️script.ts:81-216`), Space (`🪐️space/📦️packages/🦀️rust/📜️script.ts:29-34`, `:37-108`, `:435-568`), VCS (`🌿️vcs/📦️packages/🦀️rust/📜️script.ts:114-154`), and the canonical Stdio generators/checks.
- The Stdio document generator reads and writes fixture manifests as fixture authoring (`🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/📜️script.ts:528-573`); it does not publish a component or add data to a runtime catalog.
- Lowpoly's optional `cad-fixtures` feature is only compiled inside `#[cfg(all(test, feature = "cad-fixtures"))]` tests (`💠️lowpoly/📦️packages/🦀️rust/Cargo.toml:51-63`; `🧬️schema/🦀️.rs:438-447`). It is neither a fixture-byte loader nor a production feature edge.
- The bounded direct-loader and manifest/config inspection found no selected plugin build, descriptor, catalog, or runtime code that copies, embeds, serves, or declares a `🧫️fixtures` file as a production asset. The Hub trace above is deliberately separated from that content-flow conclusion.

## Method and Limits

I inspected plugin TypeScript/Rust script readers, direct fixture-bearing paths, router reachability, selected Cargo/config declarations, canonical destination existence, and the Hub/GIS bootstrap path. I also ran four targeted Bun commands: the Stdio, GIS, Norm, and Process stale-reader paths fail as recorded above. Initial Nx invocations were attempted with `--skip-nx-cache`; their caching bootstrap produced no completed result within the 30-second command window, so they are not treated as evidence.

This is a read-only source and filesystem audit. It excludes the already-completed full repository path scan and resolver scan, does not prove dynamic behavior of every plugin command, and does not evaluate framework or library fixture ownership. No production files, Git state, ticket lifecycle, or generated report output were changed.
