# Global Dependency-Cruiser Architecture Audit

## Scope

Read-only audit performed 2026-08-22. No Cargo command, source edit, manifest edit, configuration edit, or full multi-stage `verify` invocation was run.

The precise first-stage command in `VerifyScript.runGate()` is:

```text
bunx dependency-cruiser compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type err
```

It completed in 23 seconds and found **1,005 violations** across **11,360 modules** and **9,996 dependencies**: **760 errors** and **245 warnings**. This is the current blocker before any later `verify` stage can run.

## Exact Current Rule Census

| Severity | Rule | Findings | Repair classification |
| --- | --- | ---: | --- |
| error | `no-circular` | 155 | real dependency cycles |
| error | `no-core-path` | 84 | real taxonomy/name debt |
| error | `no-cross-technology-compose-to-🧰️framework` | 32 | 24 tooling edges; 8 runtime/composition edges |
| error | `no-cross-technology-♻️mit-bestand-to-✏️s` | 1 | real cross-technology edge |
| error | `no-cross-technology-♻️mit-bestand-to-🧰️framework` | 21 | 7 tooling/generated edges; 14 runtime/composition edges |
| error | `no-cross-technology-✏️s-to-🧰️framework` | 160 | 135 `📜️script.ts` tooling edges; 25 runtime edges |
| error | `no-cross-technology-🌎️hub-to-🧰️framework` | 22 | 3 tooling edges; 19 runtime edges |
| error | `no-cross-technology-🧰️framework-to-♻️mit-bestand` | 3 | real reverse-layer edges |
| error | `no-state-outside-os` | 1 | real OS-authority leak |
| error | `not-to-unlisted` | 89 | package-manifest declaration debt |
| error | `renderer-hosts-only-ui` | 147 | mixed resolver-policy mismatch and real host coupling |
| error | `s-modules-no-plugins` | 9 | real inverted spatial-kernel dependency |
| error | `ui-no-framework-packages` | 36 | real UI ownership breaches, with tooling subset |
| warn | `no-cross-package-relative` | 228 | package-boundary migration backlog |
| warn | `no-impl-segment` | 2 | taxonomy/generated-source cleanup |
| warn | `no-plugin-to-extension-📐️cad` | 4 | documented real CAD composition-root inversion |
| warn | `plugins-framework-sdk-only` | 11 | real plugin-to-framework leakage |

## Evidence-Based Clusters

### 1. CAD / Spatial Kernel Knot — repair first

This is the densest shared graph, so clearing it reduces several rule families together:

- 28 of the 155 circular findings originate in `✏️s/🔌️plugins/📐️cad`; 8 more originate in `✏️s/🔨️modules/🌐️spatial-kernel`. The CAD↔spatial-kernel pair contributes 17 directed cycle findings, in addition to 16 internal-CAD cycles.
- All 9 `s-modules-no-plugins` findings are spatial-kernel importing CAD code (six from `🧱️brepjs`, two `📐️geometry`, one `🗺️spatial`).
- The documented 4 `no-plugin-to-extension-📐️cad` warnings are all the CAD runtime composition root importing its four extension package entries.
- CAD also supplies most of the 25 non-tooling `✏️s → 🧰️framework` edges and most plugin-SDK-only warnings.

Repair packet: introduce an owned spatial-kernel port/contracts layer; move CAD-specific registrations and extension assembly to a composition boundary; make extensions register through that boundary rather than have plugin core import extension entries. Then remove dependency directions from spatial-kernel to CAD and break the action/runtime/artifact/typology cycles.

### 2. Renderer Host and UI Ownership — split policy mismatch from genuine coupling

`renderer-hosts-only-ui` has 147 errors, but the rule currently matches **resolved paths**, while its allowlist primarily names **specifier forms**. Confirmed policy-shaped mismatches include:

- 28 imports resolving to `node_modules/react/index.js` and one to `react-dom/client.js`, although the rule explicitly intends to allow React.
- 38 imports resolving under the framework UI/styling implementation roots (26 UI React package entries, 8 styling package entries, and 4 UI test-render entries), although the rule comment explicitly permits UI/styling.
- Node built-ins and test-only Vite/Vitest paths are also resolved without their `node:`/package-specifier spelling.

These are not vendor code failures; they are rule matching drift. Do **not** blanket-exclude `node_modules`: the same rule also reports genuine host imports of OS shell, kernel, replication, registry, renderer-specific, 3-D, and WASM package paths.

The real coupling packet remains large: renderer cycles account for 62 of 155 `no-circular` findings, and UI accounts for another 31. `ui-no-framework-packages` adds 36 errors (17 targeting framework SDK glue, 9 repo-library, 7 assets; eight come from tooling files). Repair by extracting renderer-facing protocol/port types and UI presentation contracts, then remove host→OS/kernel/registry/data imports from element components. In a separate small config packet, make the rule recognize resolved React and UI/styling paths, while retaining the forbidden OS/product paths.

### 3. Cross-Technology Runtime vs Tooling

The cross-technology rules correctly find runtime coupling but are also applied to bootstrap tooling:

- `✏️s → 🧰️framework`: 135 of 160 are `📜️script.ts` edges to repo library/descriptor scripts; 25 are runtime code.
- `compose → 🧰️framework`: 24 of 32 are `📜️script.ts`/Vite tooling; 8 are application/composition code.
- `🌎️hub → 🧰️framework`: 3 of 22 are tooling; 19 are application code.
- `♻️mit-bestand → 🧰️framework`: 7 of 21 are tooling/generated; 14 are application/demo code. The 3 framework→mit-bestand and 1 mit-bestand→s findings are runtime edges.

Repair packet A (small, policy): decide and encode one explicit bootstrap-tooling boundary rather than treating all scripts as runtime technology dependencies. This must be narrow (`📜️script.ts` and approved config entry points only), evidence-backed, and must not hide product code.

Repair packet B (medium/large, runtime): replace remaining relative cross-technology runtime imports with owned package/port APIs. Prioritize the 25 non-tooling `✏️s → 🧰️framework` edges alongside the CAD/spatial packet, then compose, hub, and mit-bestand consumers.

### 4. Taxonomy and Manifest Integrity

`no-core-path` is genuine naming debt, not vendored code:

- 38 hits target GLTF `🔨️geometry-core`.
- 29 hits target `@semio-tech/animate-present-core`.
- 10 target `@/lib`; the remaining hits include old `🫀️core` DSL paths and Node aliases whose source path still contains a banned stem.

Repair packet: rename/move the named API roots and their import specifiers together; update generated artifacts at the generator source rather than editing emitted files.

`not-to-unlisted` is 89 external imports resolving under `node_modules`, but they are **not vendor false positives**: the rule specifically reports a missing nearest-package declaration. Top resolved targets include Vite (6), Tailwind (5), `postcss-load-config` (5), Three (5), and `@tailwindcss/vite` (4). Repair manifests or remove the imports, package-by-package; do not suppress `node_modules` in the cruiser scope.

### 5. Remaining Warning Backlog

- `no-cross-package-relative` (228): 7 touch generated sources, 7 touch WASM `pkg/` outputs, and 5 are tooling; the remainder are real cross-family relative imports. Generated paths must be corrected through their generator; `pkg/` imports are project-owned build artifacts and need owned façade APIs, not a vendor exclusion.
- `plugins-framework-sdk-only` (11): real plugin use of framework 3-D/UI/OS targets; replace with plugin SDK contracts or a plugin-owned adapter.
- `no-impl-segment` (2): one generated catalog path and one UI icon resolver still expose the retired shape. Fix the generator/input taxonomy, then regenerate.

## Prioritized Independent Repair Packets

1. **P10s-a: Verifier semantics alignment (small).** Correct only the resolver-form mismatch in `renderer-hosts-only-ui` and establish the explicit bootstrap-tooling treatment for cross-technology rules. Re-run the narrow cruiser command and prove actual runtime findings remain visible. This removes false positives without weakening product boundaries.
2. **P10s-b: CAD spatial ownership inversion (large).** Untangle CAD extensions, spatial-kernel imports, and CAD action/runtime cycles. Expected to reduce `no-circular`, `s-modules-no-plugins`, `no-plugin-to-extension-📐️cad`, `plugins-framework-sdk-only`, and runtime `✏️s → 🧰️framework` together.
3. **P10s-c: Renderer/UI port boundary (large).** Remove renderer host’s OS/kernel/registry/data imports and UI’s framework business imports; separately collapse renderer/UI SCCs. This is the biggest product-layer gate after CAD.
4. **P10s-d: Package declaration integrity (medium).** Resolve all 89 `not-to-unlisted` findings through per-package manifest ownership or import retirement. Keep it serialized with dependency-removal packets so the manifest ratchet stays valid.
5. **P10s-e: Core-stem taxonomy purge (medium).** Rename GLTF geometry-core, Animate present core, `@/lib`, and residual DSL core paths at the source/generator level.
6. **P10s-f: Runtime cross-technology consumers (medium).** Migrate the remaining runtime edges in compose, hub, mit-bestand, and framework through owned packages/ports after packet a has isolated tooling noise.
7. **P10s-g: Warning ratchet (large, finalization).** Convert cross-family relative imports to owned package APIs; regenerate the two stale generated references; remove plugin framework leakage. Promote warnings only once the source inventory is empty.

## Verification Handoff

Run only the narrow dependency-cruiser command above after each packet. Do not use `bun ./📜️script.ts verify` as a development loop until this command is green; the full verb subsequently invokes unrelated Nx and Cargo stages.
