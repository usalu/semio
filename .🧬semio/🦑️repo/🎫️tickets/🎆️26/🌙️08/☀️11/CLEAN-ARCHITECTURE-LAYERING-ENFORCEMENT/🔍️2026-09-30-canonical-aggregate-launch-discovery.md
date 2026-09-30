# Canonical Aggregate And Launch Discovery Review

Read-only current-routing audit. No targets executed, descriptors regenerated, broad builds run, or source changed. Native counts reported by other owners are not independently verified here.

## Aggregate Scope And Omissions

Root `📜️script.ts:8958–8959` runs `bun nx run-many -t canonical-architecture --all --exclude workspace`. Its registration is `📋️project.json:1622`, and launch `🏛️check🧩️canonical-architecture` is `.vscode/launch.json:5090`. Discovery depends on the actual project target, not merely a test file or script registration.

Repo library `📦️packages/🟦️typescript/📋️project.json:164` contributes canonical-architecture through `test dependency-direction`. That script at lines 79–85 executes the hostile/oracle TS test and the live complete TS/JS dependency graph. The separately authored strict Cargo `lint-cargo-dependency-direction` and `test-cargo-dependency-direction` targets are present near the project tail, and launched explicitly at launch lines 17286 and 17330. Neither is currently in that canonical contribution. The aggregate therefore does not establish Cargo direction through this owner.

Session project `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📋️project.json` contributes check, test, wasm, source-check, and browser-test; no canonical-architecture target exists. Framework 3d Rust project's manifest likewise has no canonical-architecture target, so retained modeling jobs/Parry3d tests are outside the aggregate. Their ordinary test routes remain available; missing canonical contribution is not a missing native test.

S composition project `✏️s/🧑‍💻dev/🧩️composition-laws/📦️packages/🦀️rust/📋️project.json:43` names canonical-architecture but invokes source-check. Its script lines 5–10 run only testCompositionOwnership. The separate TestScript lines 12–21 uses compositionLawGroups plus runExactCargoLaws. Thus the claimed 355 native composition laws cannot be attributed to the canonical aggregate. Existing native test-specific launch entries occur at launch lines 19376–19423, and the generic test picker also exists.

React owner contributes real translation-totality Vitest through its canonical script lines 51–56. Wgpu owner maps canonical-architecture to MediaSlotContractTestScript; script lines 112 onward run neutral presented-media-slots Vitest plus seven exact native laws. These scopes are honest and executable. Framework OS native canonical script lines 1736–1770 executes its explicit shared policy/publication laws; it does not implicitly discover newly added Session/modeling tests.

The canonical project picker at `.vscode/launch.json:20352–20376` includes repo-lib, composition-laws, UI React, wgpu, and existing owners. It correctly omits Session/3d while they lack targets. If those owners gain canonical contributions, update this picker alongside targets. Session already has dedicated source-check/browser-test commands at launch lines 5214 and 15012 and appears in generic check/test/wasm pickers; that does not add it to canonical execution.

## Descriptor And Catalog Regeneration Routes

`@semio-tech/framework-os-dev:plugin` is a real Nx route (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:220–228`). Its router lines 110–119 forwards ordinary arguments to PluginBuildScript; build execution lines 194–196 accept a plugin filter. Existing deployment catalog explicitly identifies sequence, playbook, playbook-module-procedural, and flow-extension-brep.

Relevant executable commands are:

- `bun nx run @semio-tech/framework-os-dev:plugin -- sequence`
- `bun nx run @semio-tech/framework-os-dev:plugin -- playbook`
- `bun nx run @semio-tech/framework-os-dev:plugin -- playbook-module-procedural`
- `bun nx run @semio-tech/framework-os-dev:plugin -- flow-extension-brep`
- `bun nx run @semio-tech/plugin-registry:generate`
- `bun nx run @semio-tech/plugin-registry:check-generated`

These are descriptor-producing builds, not cheap metadata-only rewrites. `🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts:115–127` probes the freshly materialized component with JSPI, finalizes descriptor hashes, and writes the owner's `🛂️.descriptor.semio` and `🔣️.json`. Build materialization stages that fresh descriptor; registry generate only refreshes registry/catalog projections and must not be described as repairing stale concrete compiled descriptors.

Sequence/playbook package `registerPlaygroundSiteBuildCommands` registers build <variant|all>, but this delegates a complete release distribution rather than a descriptor-only task (`📚️library/🎮️playground/🌐️site/🟦️.ts:22–40`). The Flow BREP `@semio-tech/flow-extension-brep-rust:package` route builds a wasip2 extension and emits .sxt through runExtensionComponentPackage; it is distinct from the browser component descriptor build above. Sequence artifact io-descriptor-parity has explicit launches at lines 18672–18686 and verifies artifact I/O descriptor equality, not plugin/browser descriptor regeneration. The existing plugin-registry rebuild-all launcher at line 4782 performs broader catalog rebuilding; use the filtered plugin build for the concrete edited owners.

The coordinator received aggregate omissions and the precise runtime-scope distinction. No aggregate green claim follows from this routing review.
