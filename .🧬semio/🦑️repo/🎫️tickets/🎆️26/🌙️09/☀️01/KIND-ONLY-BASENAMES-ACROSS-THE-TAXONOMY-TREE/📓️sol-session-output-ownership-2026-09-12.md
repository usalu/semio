# Session Output Ownership Correction — 2026-09-12

## Result

The canonical generated playground session now has one writer and one identity. `@semio-tech/framework-os-dev:generate-playground-session` writes only `DEFAULT_HOST_VARIANT`, currently `s`, to:

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts`

`ensurePluginRegistry(filterPlugin?)` still regenerates the registry and synchronizes the selected plugin descriptors, but no longer chooses a session variant or writes that canonical source. Variant-specific sessions are produced only by the registry's `session <variant>` route and are isolated at:

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions/<variant>/🎮️playground-session/🟦️.ts`

The registry exports `PLAYGROUND_SESSION_ARTIFACT_KEY` and `stagePlaygroundSession`. The helper validates the variant, renders from the generated catalog projection, and publishes the exact `🎮️playground-session/🟦️.ts` key through `stageArtifacts`. The existing Nx output root remains `dist/sessions/<variant>` and the existing repository `dist` ignore covers the nested source.

## Exact source and contract files

Added:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🎮️playground-session/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🎮️playground-session/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎮️playground-session/🟦️.ts`

Updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — added the semantic artifact key and staging helper; the registered session route now uses them.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` — removed the selected-variant canonical writer and updated preparation and activation imports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts` — the virtual session alias uses the staged semantic owner.
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` — runtime preparation imports the staged semantic owner.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — the boot-planner contract now describes defensive handling of a caller-injected mismatched session instead of a concurrently overwritten source.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts` — the compiler interception reaches anonymous TypeScript leaves and guards the exact canonical path; a positive control proves one Bun and one esbuild interception before the WGPU zero-read assertion.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — the canonical playground-session generator contract now includes the registry renderer source as an ordered input and records exclusive default ownership.

No executable target or dependency was added, so launch registration and package manifests did not need changes.

## Exact generated staging correction

The twelve extant variant outputs were regenerated through the corrected producer. Each old named leaf was removed and replaced as follows; every `.nx-artifact.json` now contains exactly `["🎮️playground-session/🟦️.ts"]` with its existing `playground-session:<variant>` owner.

```text
dist/sessions/aggregator/🟦️session.ts -> dist/sessions/aggregator/🎮️playground-session/🟦️.ts
dist/sessions/aussuchen/🟦️session.ts -> dist/sessions/aussuchen/🎮️playground-session/🟦️.ts
dist/sessions/bearbeiten/🟦️session.ts -> dist/sessions/bearbeiten/🎮️playground-session/🟦️.ts
dist/sessions/draw/🟦️session.ts -> dist/sessions/draw/🎮️playground-session/🟦️.ts
dist/sessions/generation3d/🟦️session.ts -> dist/sessions/generation3d/🎮️playground-session/🟦️.ts
dist/sessions/generator/🟦️session.ts -> dist/sessions/generator/🎮️playground-session/🟦️.ts
dist/sessions/koordinator/🟦️session.ts -> dist/sessions/koordinator/🎮️playground-session/🟦️.ts
dist/sessions/note/🟦️session.ts -> dist/sessions/note/🎮️playground-session/🟦️.ts
dist/sessions/puzzle3d/🟦️session.ts -> dist/sessions/puzzle3d/🎮️playground-session/🟦️.ts
dist/sessions/puzzle5d/🟦️session.ts -> dist/sessions/puzzle5d/🎮️playground-session/🟦️.ts
dist/sessions/s/🟦️session.ts -> dist/sessions/s/🎮️playground-session/🟦️.ts
dist/sessions/verfolgen/🟦️session.ts -> dist/sessions/verfolgen/🎮️playground-session/🟦️.ts
```

Post-generation identity evidence:

| Identity | Bytes | SHA-256 | Variant | Registry plugin | Host mode |
|---|---:|---|---|---|---|
| canonical default | 12,139 | `4a785c5f4288f2874e86540a27efb9bde741ed3bfd97ed121d193a643fce0043` | `s` | `space` | true |
| staged note | 965 | `13044f6c089b27f71f975719adea6694fc2499ba20d498f4e9c44f6f4b46a216` | `note` | `note` | false |
| staged puzzle5d | 996 | `ec096d23233c7a2c0f1fc2f5fa82f38a5a064ecc902a028c8607823757a5f8d0` | `puzzle5d` | `puzzle` | false |

## Regression contract

The language-neutral JSON fixture declares the default source and producer, the staged root, semantic directory, anonymous source leaf, artifact key, Vite specifier, and two explicit identity rows. The Draft 2020-12 schema rejects undeclared fields and fixes the output contract to schema version 1.

The focused Vitest regression validates the fixture through Ajv, proves the dev registry-preparation function has no session writer, establishes the default bytes, stages `note` and `puzzle5d`, and verifies:

- both variant directories coexist;
- the old named leaf is absent;
- each receipt owns the semantic artifact key;
- direct native imports report the expected variant and registry plugin;
- Bun and esbuild produce native-importable ESM with the same identities;
- the actual Vite config resolves `virtual:semio-playground-session` to each exact staged source;
- the canonical default hash remains unchanged after both explicit stages;
- canonical preview and check still agree after the stages.

The WGPU cache-input oracle now has a positive compiler control. Bun and esbuild each intercept the exact canonical source once through the same predicate and path guard. Bundling the real WGPU browser-boot entry then intercepts it zero times for two ambient session values, and the resulting bundles remain byte-identical.

## Verification

- `bunx vitest run '🧪️tests/🎮️playground-session/🟦️.ts' --config vitest.config.ts`: passed, 1 file and 2 tests; 4.63 seconds. This used the ticket-scoped `SEMIO_SESSION_OUTPUT_TEST_ROOT` and installed Ajv, Bun, esbuild and Vite code paths.
- `bun nx run @semio-tech/plugin-registry:session-note --skip-nx-cache`: passed with `repo:generator-inputs` and registry generation; 59 plugin crates, 61 playgrounds and 49 framework packages; staged `note`; Nx duration 17.5 seconds.
- `bun nx run @semio-tech/plugin-registry:session-puzzle5d --skip-nx-cache`: passed with the same registered prerequisites; staged `puzzle5d`; Nx duration 20.9 seconds.
- `bun nx run @semio-tech/framework-os-dev:check-playground-session --skip-nx-cache`: passed with `repo:generator-inputs` and `@semio-tech/plugin-registry:check-generated`; reported `playground session generated source is fresh`; Nx duration 18.8 seconds.
- `bun nx run @semio-tech/framework-os-dev:preview-playground-session --skip-nx-cache`: passed; emitted one semantic directory, one canonical anonymous source and zero stale removals; Nx duration 1.1 seconds.
- Direct `testWgpuBootInputs(...)` Bun run: passed. It reported that the Bun/esbuild positive controls read the canonical default session while the WGPU bundles remained independent, and that missing/stale browser-boot checks preserve artifact bytes and mtime.
- Direct strict `loadTaxonomy()` plus `validateTaxonomy(...)`: zero problems.
- Physical/reference closure: zero `🟦️session.ts` files remain under the OS and Demonstrator trees; twelve staged `🎮️playground-session/🟦️.ts` files exist; the only authored old-leaf string is the regression's absence assertion.
- `git check-ignore -v` confirms `.gitignore:300` (`dist`) owns the staged generated source disposition.

The registered canonical check emitted Bun's existing `NO_COLOR`/`FORCE_COLOR` warning after succeeding; it did not affect the target result. No broader OS or repository suite was run because this correction is limited to session ownership, staging, its direct consumers, and the WGPU dependency oracle.
