# Window Config Pack Inner Identity Law

## Concrete Contract

The source review in `store-owner-capability-level-review.md` found that the window registry's Pack loader validates the schema but does not compare the decoded envelope id against `window-config:{kind}:{window_id}`. A matching outer window kind therefore does not itself establish exact inner partition ownership.

The new schema-first corpus covers a valid same-window load, a foreign same-kind window, and two distinct colon-containing instance IDs. The TypeScript oracle uses strict Ajv2020 and independent JSON Patch to validate the declared admission/unchanged-target policy. It does not call the native loader and cannot prove its behavior.

The authored native test registers a real `WindowConfigOwnerRegistry` with the shared NoConfig lifecycle, initializes source and target partitions, prints the actual source Pack/SPR through `registry.packs`, changes only the outer addressed window id, and calls the real `registry.load`. A refused foreign inner identity must preserve target generation, revision and exact snapshot backing. Every registry is explicitly closed before result assertions. The zero-state fixture keeps the counterexample focused on envelope identity; it does not prove large-candidate retirement.

## Executed Evidence

`abstraction-ownership-validation:window-config-pack-identity` passed in3.0s through Bun/Nx with all3 strict neutral cases and strict TypeScript. Log: `🗑️generated/window-config-pack-identity-neutral-1.log`. Rustfmt parsed the native source and the scoped diff check passed.

Native execution is pending after the Pack/FEM queue. No foreign-inner-id runtime failure or production correction is claimed yet. The test is expected to expose the inspected missing check. A future correction must retain/release rejected decoded candidates correctly; do not add an unbounded synchronous rejection drop merely to satisfy this small fixture.

## Files And Commands

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️schema/🪪️pack-identity/🔣️.json`
- Same owner: `🧫️fixtures/🪪️pack-identity/🔣️.json`, `🧪️tests/🪪️pack-identity/🟦️.ts`, `🧪️tests/🪪️pack-identity/🦀️.rs`, and the test module registration in `🦀️.rs`.
- Root `📜️script.ts`, `📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`; ticket `validation/📜️script.ts` and `validation/project.json`.

Canonical commands are `workspace:window-config-pack-identity` and `workspace:window-config-pack-identity-native`, with launch orders311.221 and311.222. The canonical route is independent of this ticket. The ticket facade delegates it with explicit repository-root environment when run from the isolated Nx workspace.
