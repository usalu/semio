# Layout Playground Examples Audit (read-only)

Scope: `🎬️demo`, `🎬️demo-session`, `🔗️linked-asset` for the layout playground. No files were modified except this report.

Layout examples root (standards subset): `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/`

## 1. Files and resolution

Catalog entry (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json`, variant `layout`): app `s.layout.layout@1/*#editor`, ports 6079 (react) / 6179 (wgpu), examples exactly `🎬️demo`, `🎬️demo-session`, `🔗️linked-asset`. `cratePath` and `compositionConfigPath` exist.

| Example | Source folder | Primary text | Status |
|---|---|---|---|
| `demo` | `📚️examples/🎬️demo/` (`🦀️.rs`, `🟦️.ts`, `🧪️tests/🧩️example/`) | `🖼️assets/🎬️demo/🗣️.dsl.semio` | Files exist |
| `demo-session` | `✏️editor/📚️examples/🎬️demo-session/` (`🦀️.rs`, `🟦️.ts`, tests, `🖼️assets/🎮️.cmd.semio`) | `🎮️.cmd.semio` (`action=demo`) | Files exist, editor dialect only |
| `linked-asset` | `📚️examples/🔗️linked-asset/🦀️.rs` only | `🖼️assets/🔗️linked-asset/🗣️.dsl.semio` | Exists, but no `🟦️.ts` or tests |

Resolution: `?example=` is parsed by `resolveBootQueryExampleId` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts`, then validated by `examplesForApp` in `🧰️framework/🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts` (dialect-keyed). Unknown ids are dropped silently. The wgpu server (`…/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`) only reads `SEMIO_DEFAULT_EXAMPLE` / `SEMIO_LOCKED_EXAMPLE`. I did not confirm the wgpu browser-boot call site (`…/🚀️browser-boot/🟦️.ts`) consumes the resolver.

Stale artifact: `🗿️artifacts/📏️layout/📚️examples/🛂️manifest.json` declares id `layout-sample` and fixture `📏️sample.layout`. That fixture exists nowhere in the repo, and no code references the manifest. It should be removed or replaced.

## 2. Schema conformance

Examples are DSL text (`layout.layout.dsl v1`), not JSON, so the JSON Schema (`🧬️schema/🔣️.json`, `LayoutArtifact`) cannot be validated directly. Required keys: `schema, name, grid, paragraphStyles, characterStyles, stories, links, parentPages, spreads, pages, printTarget`.

- `demo` and `linked-asset` DSL both contain schema, name, grid, paragraph-styles, character-styles, stories, links, parent-pages, spreads, pages.
- `print-target` is absent from both. That is a required-key gap, unless the parser defaults it (unverified).
- `🧪️tests/🧫️fixtures/🛍️canvas-catalogue/linked-asset.json` has keys camera, catalogueBoundary, drop, expected, refusals, source. It is a canvas-catalogue fixture, not a LayoutArtifact, so it is not checked against the artifact schema.
- No validator was run (python `jsonschema` not installed). This is a key spot-check only.

## 3. Tests

- Language-agnostic `🥒️.feature`: only one, `🧪️tests/📐️mutate-layout-1/🥒️.feature` (139 lines).
- Language-agnostic `🧫️cases.json`: 14 under the layout subset (`✏️editor/⚙️engine/…`), none at the plugin root.
- Third-party validation: `🧪️tests/🧫️fixtures/🛍️canvas-catalogue/linked-asset-oracle.py` (PIL/jsonschema), `…/✏️editor/🖼️canvas/🧪️tests/rotated-proxy-oracle.py` (affine). TS tests use three, lodash, fast-xml-parser, fast-json-patch, @napi-rs/canvas.
- Cross-language oracle: `✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/portable-oracles.json`.

Gaps:
- No `.feature` or `cases.json` covering the three example ids or example-picker behavior.
- No `.feature` at the plugin level.
- `linked-asset` has no test folder and no TS twin.
- `demo-session` has Rust and TS example tests but no language-agnostic case file.

## 4. Root session inputs

- `🛂️.descriptor.semio` (78 KB) and `🔣️.json` (255 KB) exist at `🌎️hub/🧩️compositions/📏️layout/🧩️component/`.
- They do NOT exist at the owner source root `✏️s/🔌️plugins/📏️layout/`. Only `AGENTS.md`, `🗿️artifacts`, `🏭️bridge`, `🎮️commands`, `🔮️oracles`, `📦️packages`, `🧬️schema`, `🧪️tests` are there.
