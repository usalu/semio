# All-App Discovery and Launch Gate Implementation

Date: 2026-08-26

## Result

The bounded descriptor and launch parsers are implemented. The stricter plugin-descriptor startup boundary is GREEN: it joins each descriptor app context through the generated playground registry to an owner-qualified React, WGPU Wasm, and WGPU native launch triple instead of accepting the presence of unrelated dev configurations.

All 101 descriptor app contexts now have an exact complete route. The launch catalog additionally discovers 68 launch-only product surfaces, including 11 Compose surfaces, for 169 total production app/product surfaces without a hand-maintained allowlist. Canonical playground identities were repaired for Mathematical, Procedural 2D/3D, Process 3D, and Sourcing; Energy and Demonstrator gained their missing canonical playgrounds; and the registry generator now synthesizes any missing renderer surface for every discovered playground. This covers all 15 Norm variants and the Demonstrator variants that previously had React-only launchers without a manual allowlist.

The live result is stored in `📊️all-app-discovery-gate-live-2026-08-26.json`:

- 32 descriptor files;
- 28 parent app descriptors and 4 parent-activated CAD extension descriptors;
- 101 declared app surfaces representing 94 unique app identities;
- 68 launch-only product surfaces, 11 of them Compose, for 169 total discovered surfaces;
- 248 `🛠️dev` launch surfaces;
- 0 discovery or registration failures;
- 25 hostile, capacity, localization, launch-only product discovery, owner-qualified launch-join, command/renderer reachability, action-disposition, JSONC, and TypeScript-oracle self-tests.

## Implementation

`📜️script.ts` now:

- recursively discovers every `✏️s/🔌️plugins/**/🔣️descriptor.json` within a fixed 256-descriptor capacity;
- derives launch-only framework/product surfaces from the generated launch catalog and rejects duplicate/ambiguous product identities without a product allowlist;
- caps each app descriptor at 64 app declarations and the launch catalog at 512 configurations;
- validates descriptor version, role, owned identifiers, app identities, and English/German labels without a default language;
- classifies nested extension descriptors from their owned `EXTENSION_ID` and `.extends(...)` source declarations, then proves the parent app is discovered;
- reports repeated app ids as explicit aggregation evidence rather than silently deduplicating them;
- discovers every `🛠️dev` launch configuration from `.vscode/launch.json` and validates the canonical `.vscode/🧩️launch.seed.jsonc` source as the same fail-closed boundary;
- proves the exact `verify interactivity`, `verify interactivity tool-jobs`, focused `verify interactivity apps`, descriptor-action acceptance, dependency-ratchet, and zero-target dependency launch registrations remain in `4_gate`;
- compares owned JSON/JSONC parsing with the test-only TypeScript parser through an owned string boundary;
- runs this discovery contract as part of the root `verify interactivity` gate and as the focused `verify interactivity apps` command.

`🖥️launch.ts` now treats the registry as the zero-touch renderer matrix: for each sorted playground it preserves curated seed launchers and adds only missing React, WGPU Wasm, or WGPU native entries. The generated entries carry exact plugin/app ownership and per-renderer ports. The app verifier independently reparses the generated playground catalog and proves the exact owner-qualified triple, command/cwd route, and browser renderer selection, including hostile missing-native, wrong-owner, wrong-command, and wrong-renderer fixtures. That stronger check exposed Draw's native launcher still targeting `s`; the canonical seed now targets `draw` and regeneration preserves it.

The invalid clean-task nesting was repaired at its canonical source in `.vscode/🧩️launch.seed.jsonc`, rather than only in generated `.vscode/launch.json`. The root, tool-job, discovery, descriptor-action, dependency-ratchet, and red-until-zero dependency gates are registered in the existing order in both files. The launch registry generator at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts` now parses the JSONC seed and generated JSON before returning, so an invalid canonical seed or invalid generated catalog fails closed instead of overwriting the launch surface silently.

## Verification

```text
bun ./📜️script.ts verify interactivity apps
exit 0
descriptors=32
extensions=4
apps=101
launchOnlyProducts=68
surfaces=169
actions=4754
migratedActions=1973
missingActions=2781
launchCoveredApps=101
launchMissingApps=0
launches=248
failures=0
selfTests=25
```

The independently launchable descriptor-action acceptance gate is deliberately RED and reports the exact live backlog without relabeling it:

```text
bun ./📜️script.ts verify interactivity apps --actions
exit 1
actions=4754
migratedActions=1973
missingActions=2781
dispositionFailures=2781
productionJoinFailures=1372
acceptedCommands=165
acceptedSharedRoutes=8
4153 descriptor action disposition or production-reachability join failures
```

The action gate caps each app at 512 action declaration rows, validates owned action ids and equivalent English/German labels, and covers migrated, missing, maximum-plus-one, owner-local production join, wrong-owner, and accepted shared-route hostile fixtures. It joins every migrated descriptor row to the live tool-job census; a metadata-only migration now fails closed. The 99 newly accepted Puzzle command rows belong to launch-only products and therefore increase `acceptedCommands` without changing the 4,153 descriptor-row failures. The separate tool-job gate additionally requires the entire runtime ledger to be clean.

The focused discovery/launch gate and the latest coordinator rerun of the full static gate are GREEN after reconciling Puzzle fill with the mounted-session universal contract:

```text
bun ./📜️script.ts verify interactivity
exit 0
[verify interactivity apps] descriptors=32 extensions=4 apps=101 launchOnlyProducts=68 surfaces=169 actions=4754 launchCoveredApps=101 launchMissingApps=0 launches=248 failures=0 selfTests=25
[verify interactivity] severity=deny
1 recorded allowlisted blocking-bridge finding
DENY mode — clean
```

The conflicting former Puzzle-specific direct-driver predicate was replaced by an exact mounted-session predicate: production has no direct `drive_step` or `StepBudget`, one `pump_one` opportunity owns the checked-out outcome, and hostile fixtures reject direct-driver substitution and duplicate mounted opportunities. `bun nx run @semio-tech/plugin-registry:check` reparsed and generated the 60-playground catalog and launch file, then exited 1 on broader pre-existing taxonomy, packaging, descriptor-id, and stale-Wasm-hash violations. Therefore this report claims the focused all-app launch gate and root static interactivity gate only; it does not relabel the repository-wide registry check as green.

This proves the live plugin-descriptor plus launch-derived product discovery, launch-registration, and root static interactivity contracts only. The 68 launch-only product surfaces are now members of the 169-surface universe, but they do not yet contribute an interaction-action ledger equivalent to the 4,754 descriptor rows; that remains a fail-closed acceptance gap. It does not claim the native, Wasm, replay, timing, browser, accessibility, device, or dependency-zero matrices are complete; those remain explicit final acceptance gates.

The seven repeated declaration rows are intentional host-context evidence in the Demonstrator descriptor: CAD editor, GIS Map editor, Procedural 3D editor, Process 3D editor/viewer, and Sourcing Curate editor/viewer also belong to their owning descriptors. The final matrix must execute all 101 declaration contexts while comparing shared semantic results by their 94 unique app identities.
