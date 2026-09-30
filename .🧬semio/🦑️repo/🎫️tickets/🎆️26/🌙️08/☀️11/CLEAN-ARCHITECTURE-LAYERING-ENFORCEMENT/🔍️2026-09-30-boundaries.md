# Current Boundary Audit — 2026-09-30

Read-only source inspection and successful `cargo metadata --format-version 1 --no-deps` probe. No test suite or full dependency-cruiser sweep was run. No source changes.

## Verified Current Coupling

Cargo metadata confirms 14 normal dependency edges from framework packages/modules into products:

- `semio-framework`, schema, tool-run, ui, compiler, 2d, graph-layout-run → `semio-framework-os-kernel`.
- graph → os-kernel and os-kernel-neural-engine.
- editor → os-kernel and os-infinite.
- surface → os-kernel, os-infinite, framework-artifact-infinite-dag.

Exact manifests follow the package taxonomy `🧰️framework/🔨️modules/{🧬️schema,⏯️tool-run,🖱️ui,📚️compiler,◻️2d,🕸️graph,🕸️graph/⏯️layout-run,✍️editor,🗺️surface}/📦️packages/🦀️rust/Cargo.toml`, plus `🧰️framework/📦️packages/🦀️rust/Cargo.toml:36`. The graph manifest lines 23–24, editor lines 35–37, and surface lines 43–44 expose real inverse dependencies. UI's kernel dependency is optional behind `wgpu`; other edges still exist in Cargo metadata independently of selected activation. `🌱️value/✨️derive/.../Cargo.toml:28` additionally has an OS dependency, but it is dev-only and excluded from the normal-edge count.

The framework facade `🧰️framework/📦️packages/🦀️rust/🦀️.rs:3–9,32` aliases the OS kernel as dsl/protocol/protocol_core/store and exports Locale/LocalizedLabel/Terminology/io_schema. Consequently the apparently neutral SDK still exposes product-owned contracts.

Fresh literal import inspection found real framework→implementation edges:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:98` imports inference reconcile parsers directly from `🌎️hub/💡️inference/🧬️schema/🟦️.ts` (runtime).
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts:5,12` imports puzzle and cad tests directly from s.
- `🧰️framework/🛍️products/💻️os/🧪️tests/🔑️directory-access-changed/🟦️.ts:12` and `🧪️tests/⏳️transient-apply-refusal/🟦️.ts:11–12` import hub JSON fixtures/schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts:19` and `🚀️local-hub/🏃️execution/🟦️.ts:23,34` import implementation-owned bootstrap services.

These are current source observations, not a claim that a full resolver sweep has reported them. Some test/bootstrap coupling is composition work, but currently resides beneath the neutral framework root.

## Enforcement Holes

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`:

- Cross-technology rules lines 137–153 only forbid `local` dependencies. `framework-no-s` lines 252–261 handles s package names, but no equivalent framework-no-hub package rule exists. No rule distinguishes neutral framework modules from framework products.
- Plugin SDK, cross-package relative, implementation-segment policies retain WARN severity; plugin core→extension retains procedural/cad WARN exceptions. Verify command arguments in root script lines 730,8967 omit `compose` although config lists it.
- Strict rules already exist for renderer presentation ownership and framework→s. Do not redundantly replace them with another policy system.

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️layering-policy/🟦️.ts:43–47,107–145` considers only framework/s-module/plugin/extension metadata roles; product/hub/testkit/tool are deliberately excluded. Unknown or missing roles silently disappear. All framework→product Cargo edges above therefore pass its policy. Dependency alias resolution uses Cargo dependency canonical names correctly, but role extraction is an unbounded regex that may capture a later TOML table's role.

The same file's `KNOWN_LAYERING_VIOLATIONS` includes procedural→flow extensions and renderer-wgpu→puzzle; freshness of those exceptions was not revalidated. It does not assert that every exception remains present.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:718–755,783–797` implements the layering ratchet by counting literal `<implementation-area>/` substrings. This includes comments but misses package aliases, crate identifiers, relative edges without area names, and product boundaries. Per-file totals allow an old coupling to be exchanged for a different coupling at the same count. Generated banners in the first 512 bytes exempt the entire file. `🧅️layering.json` measures debt, not a dependency graph invariant.

## Coherent Implementation Slice

Make neutral protocol/value/state contracts framework modules and reverse product dependencies onto them. Start with schema/value contracts and SDK facade: kernel owns behavior, framework owns neutral public types. Preserve actual CQRS/event contracts; avoid adapters or compatibility aliases.

Change schema/value owning source files and their language-agnostic fixtures; framework facade and OS kernel public exports; affected manifests listed above. Extend existing taxonomy role policy, dependency-cruiser config, library boundary engine, and workspace-contract tests together. Move implementation integration tests/bootstrap composition to implementation or designated integration owners rather than exempting all tests.

Required policy fixture cases: framework→framework allowed; framework-module→product/hub/plugin denied; product→framework allowed; implementation composition→product allowed; alias and Cargo workspace dependencies classified; missing roles fail; dev/build edge classification explicit; stale exceptions fail. Validate fixture verdicts against dependency-cruiser for TS and cargo metadata for Rust as third-party comparators. Add schema validation for fixture format.

Run through existing Nx/script targets: root verify layering; root boundary/verify gate target; framework-os-dev layer-lint; repo library workspace-contract target; impacted crate checks/tests through existing bundle scripts. Inspect project registration before selecting exact target spellings; standalone documented Cargo policy invocation is `bun nx run @semio-tech/framework-os-dev:layer-lint`. Register any genuinely new executable policy command in `.vscode/launch.json` through the existing ordering. Do not claim success until these actually run.
