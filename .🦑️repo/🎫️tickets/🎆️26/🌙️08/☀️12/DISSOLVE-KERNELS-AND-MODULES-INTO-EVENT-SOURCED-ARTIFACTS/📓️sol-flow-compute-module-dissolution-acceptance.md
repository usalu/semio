# Flow Compute Module Dissolution Acceptance

## Baseline

- `✏️s/🔌️plugins/🌊️flow/🔨️modules/🧮️compute/🟦️component.ts` was clean at SHA-256 `4cae46be8e6f30501cffcd751e2ef5899334754f6836fedb0a5c7b2d359d8c7d`.
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️vitest.config.ts` was clean at SHA-256 `3841164880ea235b9ca33ba9296df0e09d24aecf900c6c3809b6f9bf33da7f41`.
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📋️project.json` was clean at SHA-256 `886599e45c9d8095d7c02e27e12762082b3acebdf52f09b11a14b81ac57b0e56`.
- The production TypeScript barrel did not export or import the compute module. Compute symbols were confined to the source leaf and its in-source tests; the only active file reference was Vitest configuration.
- A protected root-script migration breadcrumb mentions the old path as text only. It is neither an import nor executable dependency and is outside this lease.

## Implementation

- Deleted `✏️s/🔌️plugins/🌊️flow/🔨️modules/🧮️compute/🟦️component.ts` and its now-empty `🧮️compute` and `🔨️modules` directories.
- Updated `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️vitest.config.ts`: removed the deleted source from `include`, removed `includeSource`, removed its coverage inclusion, and enabled Vitest’s standard `passWithNoTests` behavior.
- Updated `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📋️project.json`: removed the deleted modules glob from the default named input.
- The first package-local quick test exposed an independent stale assembly fault: the production barrel still targeted pre-standard Flow artifact paths, so importing `📦️index.ts` failed before testing the deleted module.
- Revalidated the expanded clean barrel baseline, then updated only `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📦️index.ts`: retained the existing schema, snapshot, diff, mutation, and IO namespace names while targeting their existing canonical `🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any` leaves. Removed the absent and unconsumed `flow_decomposer` namespace.

## Post-Edit Evidence

- Deleted compute leaf: absent.
- Post-edit SHA-256:
  - `🧪️vitest.config.ts`: `42e346582bebd711983596988d9e3067c156897563fb2c0b12f92203995aa6fa`.
  - `📋️project.json`: `559196c224536e52a1fe0e1e2f6cc5505f851fafe3e5c9c0596fbb5189797b6e`.
  - `📦️index.ts`: `513022a3aa593288d08638e3eb56fdba7f53bf1f31e74e530830aa3c8ce61428`.
- `rg` found no live Flow reference to `initFlowThreadPool`, `FlowThreadPoolInit`, or the deleted compute path; the final protected-root breadcrumb check was also empty.
- `rg` found no pre-standard Flow schema/IO target and no `flow_decomposer` export in the production barrel.
- `git diff --check` and `git diff --cached --check` on the leased source/config paths completed cleanly.

## Validation

- Initial `bun nx run @semio-tech/flow-js:test-quick --skip-nx-cache` exited `1` because `📦️index.ts` imported the absent pre-standard schema leaf. This was the stale barrel assembly defect described above, not a compute-module reference.
- After the authorized barrel correction, the same Nx target exited `0`: one barrel file passed, with no tests, and its imports resolved successfully. The coordinator independently repeated this exact command with the same successful result.
- `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🌊️flow` exited `0` with `75` components, `128` errors, and `0` warnings. The exact remaining findings are unrelated Flow app, command-collection, artifact-standard/schema/serializer, and extension manifest/leaf issues; none mentions the dissolved compute module or a stale compute reference.
- The required taxonomy enforce command was run before the barrel correction and exited `1` for the same `128` existing scope errors. A repeated broad enforce was intentionally skipped after concurrent external Flow app/schema/Rust/Cargo changes began, per coordinator direction, to avoid validating a moving contaminated graph.

## Concurrent Index State

- All originally leased paths were clean before editing. After the authorized `apply_patch` edits, the compute deletion, Vitest config, and project manifest unexpectedly appeared index-staged while no Git-modifying command was run in this lease. Their index state was preserved unchanged.
- Final leased status also showed the authorized barrel edit as unstaged. No `git add`, commit, stash, reset, checkout, restore, or other index-mutating command was executed.
