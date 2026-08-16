# Wave 3 Print Pipeline Semantic SCC Lease

## Selection And Isolation

`🧰️framework/🛍️products/📓️print` is the only clean framework product boundary: one authored TypeScript runner (800 lines), zero dirty paths, unchanged existing Nx targets, and no active stdio, repository-CLI, framework-plugin-capability, kernel, machine, platform, renderer, root-registrar, or protected repository-library-index overlap.

The only active source is:

```text
78a803d32c4ca2c964d75714287789daafec06fcc40190055e00d15a9936da04  🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts
```

Rehash it and confirm the product remains clean before editing. The sole non-print implementation import is the repository-owned process/script interface; it is read-only in this lease. The only `buildPrintDocument`/`fetchPrintFonts` external source referrer is under structurally excluded `♻️mit-bestand`; it neither justifies a production module nor permits a compatibility export.

## Why This Is One Atomic Product SCC

The permanent runner currently intermixes five independently executable actions with three shared implementations:

| Responsibility | Current runner implementation | Production action consumers | Disposition |
| --- | --- | --- |
| LaTeX design-token stylesheet generation | token JSON parsing, paint calculation, LaTeX serialization, output write | `generate`; template build; template watch | product-local module |
| Print font catalog | font descriptors and deterministic font search paths | font provisioning; template build; template watch | product-local module |
| Light/dark Tectonic template compilation | Tectonic provisioning, input environment, temporary dark representation, two-pass panel glass rendering, PDF verification | template build; template watch | product-local module |
| Token stylesheet generation action | current `GenerateScript` | one action | command |
| Font provisioning action | current `FontsScript` | one action | command |
| PDF template build action | current `BuildScript` | one action | command |
| PDF template watch action | current `WatchScript` | one action | command |
| Print pipeline verification action | current `TestScript`, including quick/long partition | one verification action | command |

Splitting only one function would either leave shared implementation in the permanent runner or force a one-consumer module. The three modules become legal together: their reverse terminal closures contain the independently mounted named commands below. The permanent `📜️script.ts` remains mandatory but becomes mechanical router/assembly only.

## Semantic Layout

```text
📓️print/
  🎮️commands/
    🎨latex-token-stylesheet-generation/🟦️component.ts
    🔤print-font-provisioning/🟦️component.ts
    🖨️template-pdf-build/🟦️component.ts
    👁️template-pdf-watch/🟦️component.ts
    🧪print-pipeline-verification/🟦️component.ts
    🔣️component.json
  🔨️modules/
    🎨print-design-token-paints/🟦️component.ts
    🔤print-font-catalog/🟦️component.ts
    🖨️tectonic-template-compilation/🟦️component.ts
    🔣️component.json
```

### Command Manifest Members

| ID | Directory | Responsibility |
| --- | --- | --- |
| `framework.print.command.latex-token-stylesheet-generation` | `🎨latex-token-stylesheet-generation` | Writes the canonical LaTeX stylesheet from the framework design-token document. |
| `framework.print.command.print-font-provisioning` | `🔤print-font-provisioning` | Ensures the exact print TTF catalog is present locally. |
| `framework.print.command.template-pdf-build` | `🖨️template-pdf-build` | Builds requested registered print templates as light and dark PDFs. |
| `framework.print.command.template-pdf-watch` | `👁️template-pdf-watch` | Watches print inputs and rebuilds requested template PDFs. |
| `framework.print.command.print-pipeline-verification` | `🧪print-pipeline-verification` | Verifies pure print transformations and, at long level, every template PDF output. |

### Module Manifest Members And Proven Consumers

| ID | Directory | Exact responsibility | Declared terminal production consumers |
| --- | --- | --- | --- |
| `framework.print.module.print-design-token-paints` | `🎨print-design-token-paints` | Resolves design-token paints and renders the canonical LaTeX design-token stylesheet text. | token stylesheet generation; template PDF build; template PDF watch |
| `framework.print.module.print-font-catalog` | `🔤print-font-catalog` | Defines the canonical print-font descriptors and their deterministic search paths. | font provisioning; template PDF build; template PDF watch |
| `framework.print.module.tectonic-template-compilation` | `🖨️tectonic-template-compilation` | Compiles one registered TeX template's light and dark representations to verified PDFs, including its panel-glass pass. | template PDF build; template PDF watch |

The compiler imports the first two modules. Its terminal consumers remain the build and watch commands; intermediary modules and the verification command do not inflate the declared consumer sets. Module public contracts use only repository-owned structural options and primitive values—no Node, Tectonic, PDFJS, Canvas, or Sharp type is exposed.

## Exact Terra Write Lease

| Path | Operation | Required result |
| --- | --- | --- |
| `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts` | Modify | Retain only imports, local `BundleScript` router adapters, and `import.meta.main` execution. Each adapter delegates directly to one named command component. Remove all implementation, data contracts, tests, and legacy-facing exports; do not create aliases. |
| `🧰️framework/🛍️products/📓️print/🎮️commands/🔣️component.json` | Create | Canonical `x-semio` command collection with exactly the five members above. |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔣️component.json` | Create | Canonical `x-semio` module collection with exactly the three members and production-consumer IDs above. |
| five command component paths in the layout | Create | Move each action plus its owning tests without duplication. |
| three module component paths in the layout | Create | Move only the proven shared implementation. |

Do not alter `package.json`, `📋️project.json`, root `package.json`, root `📜️script.ts`, `.vscode/launch.json`, Cargo files/lock, framework UI styling sources/tokens, generated outputs, template sources, fonts, or any path outside the print product. Existing target names (`generate`, `fonts`, `build`, `watch`, `test`, and test levels) remain unchanged, so no root registrar is needed.

`semio-tokens.sty` and `.semio-dark/*.tex` are generator outputs. Do not hand-edit them; generate via the existing Nx targets, check drift afterward, and record their generator provenance in the relevant command/manifests. The test action is a command because Nx invokes it, but its source must not count toward any module production-consumer threshold.

## Required Execution Order

1. Create module interfaces first, preserving pure token, font-catalog, and compilation behavior exactly.
2. Move the five command bodies and co-locate their tests; update module consumer manifests only after imports are direct and terminal closures are resolved.
3. Reduce `📜️script.ts` to mechanical command registration using the existing `ScriptRouter`; it must contain no algorithms, contracts, I/O implementation, or test assertions.
4. Regenerate only through Nx. There is no external-legacy adapter: remove the former exports instead of forwarding from the runner.

## Validation And Runtime Evidence

```text
bun ./📜️script.ts verify taxonomy report --scope framework.print
bun ./📜️script.ts verify taxonomy enforce --scope framework.print
bun nx run @semio-tech/print:generate --skip-nx-cache
bun nx run @semio-tech/print:fonts --skip-nx-cache
bun nx run @semio-tech/print:test-quick --skip-nx-cache
bun nx run @semio-tech/print:build --skip-nx-cache -- report
bun nx run @semio-tech/print:test-long --skip-nx-cache
git diff --check -- 🧰️framework/🛍️products/📓️print
git status --short -- 🧰️framework/🛍️products/📓️print
```

The `generate` run must leave no unaccounted `semio-tokens.sty` drift. The `fonts` run may report zero downloads when the tracked catalog is complete. Capture the real `build -- report` PDF and `test-long` results; do not claim success if Tectonic/toolchain availability blocks them. `watch` is not a finite test target; preserve its existing registered target and verify its imports/mounts statically after the successful build runtime.
