# Result lw2: WP2 plugins to framework/products/os

Executor `lw2`. No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command, `AGENTS.md` not edited. Tools (inputs, kept): `lw2-tools/`; generated output removed.

## 1. Totals (plugin source to `🧰️framework/🛍️products/💻️os/**`)

`lw2-tools/scan.ts` + `emulate.ts` re-implement the two WP2 rules exactly (reproduces the audit: 147 and 99 before).

| metric | before | after |
|---|---:|---:|
| plugin to os import pairs (`plugins-framework-sdk-only`, os targets) | 147 | **22** |
| `no-state-outside-os` (plugin to os `store`/`host`) | 99 | **0** |
| plugin files importing os | 137 | **18** |

Real dependency-cruiser (`dependency-cruise ✏️s/🔌️plugins --exclude node_modules --do-not-follow ^🧰️framework --config <lint config>`, 10,313 modules, `lw2-tools/cruise-files.sh` for file lists): violations with an os target, WP2 total **333 (audit) to 61**:

| rule (os targets) | before (audit) | after |
|---|---:|---:|
| plugins-framework-sdk-only | 147 | 22 |
| no-state-outside-os | 99 | 0 |
| no-cross-package-relative + no-cross-technology (rest of the 333) | 87 | 39 (25 + 14; 14 of the 25 are the moved puzzle stories, see 4.3) |

The whole plugin tree now has 109 violations (sdk-only 29, ncpr 44, cross-technology 17, plugin-no-extension 15, circular 4); the non-os share belongs to lw1/lay1.

## 2. What changed

### 2.1 Extension packaging port (26 plugin `📜️script.ts`)
Plugin extension scripts imported `runExtensionComponentPackage` from `os/plugin/store/packaging`. Now a script-provider port (same idiom as `inv`/`lw1`): port `🧰️framework/🔨️modules/🏃️process/🧩️extension-package/🟦️.ts` (`packageAdmittedExtensionComponentV1`), exposed through the existing SDK key `./process/artifact-publication` (facades `📦️packages/🟦️typescript/📦️artifacts/📤️publication` of the process package and SDK), implemented by `os/🔌️plugin/🏪️store/📦️packaging/🔌️provider/🟦️.ts`, designated in the repository composition root `📚️library/🏃️process/🔌️providers/🔣️.json` (`extension-package`) and in its test token list. The 26 scripts were rewritten with `lw2-tools/codemod-pack.ts`.

### 2.2 Stories and renderer hosts (29 files)
Plugin stories rendered os renderer hosts. They are dev composition, so they moved to `✏️s/🧑‍💻dev/<plugin>/📖️stories/**`: fem (6), block (5), remodel (whole tree: 6 stories + coordination, 14 files incl. json), puzzle (4, merged from the plugin and its 3 artifacts). Renderer imports became the bare `@semio-tech/framework-renderer-react`. Scope declarations updated: `.storybook/📖️stories/🧭️coordination` (puzzle, puzzle/2d|3d|5d, remodel), hub `Cargo.toml` storybook metadata of fem and block, the discovery fixture, the scope-resolution test.

### 2.3 Artifact child, link and blob schema promoted to the neutral layer (the 85 imports)
`ArtifactChild`, `ArtifactLink`/`LinkPin` and `BlobRef` are embedded in every plugin snapshot, so they are framework contract, not store state. JSON Schema, GraphQL, proto and TypeScript moved from `os/🔨️modules/🏪️store/{🪆️child,🔗️link,📦️blob}/🧬️schema` to `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/{🪆️child,🔗️link,📦️blob}`.
- Ids renamed: `os/store/child/schema.json` to `framework/schema/artifact-child.json`, link to `artifact-link.json`, blob to `blob-ref.json`; proto packages to `semio.framework.schema.artifact_{child,link}` / `blob_ref` and symbol references (`semio.os.store.ArtifactChild|ArtifactLink|LinkPin`) updated. 211 files text-replaced (schema `$ref`s, `.graphql` and `.proto` imports, fixtures, path literals) with `lw2-tools/replace-all.ts`.
- Public surface: existing SDK key `@semio-tech/framework/schema/artifact-reference` now also exports `parseArtifactChild`, `contentId`, `parseArtifactLink`, `parseLinkPin`, `parseBlobRef`, `artifactChildSchema`, `artifactLinkSchema`, `blobRefSchema` and the types (via `@semio-tech/schema-ts/artifact-reference`). 71 plugin/hub files re-pointed (AST codemod `codemod-import.ts`, imports merged into existing ones).
- Derive index `🗂️schema-document-index.json`: three entries moved to the `🧬️schema` root; Rust test-support `include_str!` paths updated (19 paths verified to exist); os contract tests now import from the neutral entry.
- Kept in the os (Rust twin is a cargo-owned crate, I did not run cargo): `os/store/{link,blob}/🧬️schema/{🦀️.rs,📦️codec,📦️packages,🧪️tests}`. The four os glue files `📦️packages/🟦️typescript/🔨️modules/🏪️store/{🪆️child,🔗️link,📦️blob}/🧬️schema/🟦️.ts` have no importer left; they only keep the frozen export keys non-dangling (removal list in `exports-needed.json`).

### 2.4 Wasm board session contract (puzzle production code)
`createWasmModuleLoader`, `Board2dWasmSession`, `AppSurfaceSessionFactory` moved from the renderer's `WasmSessionLoader` to `🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/🪪️wasm-session/🟦️.ts`, exported through the existing SDK key `canvas/react-renderer`. 12 renderer files plus the puzzle `🌉️wasm` module re-pointed (`codemod-split.ts`); the renderer package index no longer re-exports them.

### 2.5 Service presentation contract (gis production code)
`ServiceMountedViewV1`, `InferencePortUiAction`, `InstalledServicePresentationV1<Rendered>` and the read-only `MountedServiceUiReaderV1` port (replaces the leaked `UiDocumentStore` class) now live in `🧰️framework/🔨️modules/💡️inference/🔌️service/🪟️presentation/🟦️.ts`, exported via SDK key `inference/service`. ShellHost (shell-model, host-bootstrap, directory-route, shell-core-state, identity-binding, index) and the gis presentation import them from there.

## 3. Verification (exact commands, outcomes)
All from the repo root, foreground.
- Type-check (`lw2-tools/tc.ts`, strict TypeScript program over the files, fails on unresolved modules): packaging 29 files 0; schema promotion 81 files 0; session contract 19 files 4 (2 DOM-iterable lib flags of the checker on an untouched test, 2 `@semio-tech/puzzle-wasm` = absent wasm build output); presentation contract 10 files 0; all 56 ShellHost sources 0; moved stories 30 files 3 (all absent wasm pkg).
- `bun test` of the 7 changed bun-test files (sqlite snapshot + schema suites): 87 pass 0 fail; link and blob contract tests (Ajv + TOML oracles): 2 pass; `testContentIdOracle` (Python hashlib parity) passes through the moved TypeScript; repo provider designation suite: 4 pass; extension-package provider loads end to end (`probe-extension-package.ts`).
- `lw2-tools/check-refs.ts`: of 5,247 schemas with `$id`, every semio `$ref` resolves except 3 that were missing before (stdio gltf x2, a reactor job schema); 52 schemas reference the new child id, 0 reference the old one. Derive index: 4,693 entries, every path exists, every `$id` matches.
- Storybook: `check-story-scopes.ts` (fast-glob over the real scope globs) MATCH for puzzle 4, fem 6, block 5, remodel 6; `check-story-csf.ts` (Storybook's own CSF parser) title and id identity ok for all 21 moved stories.
- Taxonomy naming law (`law-shape.ts`): banned stems 0. The emoji-uniqueness law lists the `🎭️*` sibling story folders; the same siblings breached in the plugin dirs (4,377 repo-wide), no new class.
- Not run: any `bun repo run` target, cargo, vitest renderer suites (shared lease queue / disk budget).

## 4. Still open (22 import pairs, all renderer or world coupling)
1. `cad` editor renderer imports `@semio-tech/infinite-world-r3f` (7 production files). The library depends only on neutral packages but lives in `os/♾️infinite/🌍️world/🎨️r3f`: promote it to a framework module and expose an SDK key (manifest batch).
2. Host-contract suites that run plugin data against the real renderer (`test-renderer-contract` of layout 4, puzzle 4, gis 3, procedural, raster, draw, stdio brush mesh): they belong in the dev composition projects (`✏️s/🧑‍💻dev/<plugin>`), together with their nx targets and a regenerated `launch.json`; not done because of the launch-registration churn. gis `📜️script.ts` is a cross-owner source-text conformance probe of the same kind.
3. Manifest batch (`lw2-tools/exports-needed.json`): asset/package entries needed by the moved puzzle stories (5 packages, 10 entries; today 14 `no-cross-package-relative` hits there replace the former `plugins-framework-sdk-only` hits, net zero) and 3 export keys of `@semio-tech/framework-os` to delete with their glue files.
4. `📓️schema-catalog.md` is generated and stale (scope `os.store.child` etc.); regenerate with the schema generator. Scope ids in `STORE_SCHEMA_EXPORTS` still say `os.store.{child,link,blob}`.
5. Complete the schema promotion on the Rust side (move `os-store-link-schema` and `os-store-blob-identity` crates next to the neutral schema) with a cargo-capable executor.
