# Phase 10 Next Dependency Scout After VS Code Rollback — 2026-08-23

## Decision

**NO SAFE CANDIDATE.** The accepted coordinator boundary remains exactly **129 = 66 JavaScript + 63 Rust**. This is a bounded Phase 10 dependency-wave scout, not Phase 10 acceptance.

`@vscode/test-electron` is explicitly excluded: the restored direct declaration is active. Installed `@vscode/test-cli/out/cli/platform/desktop.mjs:61` calls `mustResolve(this.config.dir, "@vscode/test-electron")`, then dynamically imports it and calls `runTests`. The preceding rollback audit records the restored 129 boundary and safe runner probes.

## Method And Census

I read `AGENTS.md`, the current coordinator inventory, the preceding no-safe scout, the accepted eslint-hooks material, the VS Code retirement/rollback reports, and installed package implementations. I ran:

```sh
bun ./📜️script.ts verify dependencies list js --format json
rg -l -F "<identity>" --glob '*.{ts,tsx,js,mjs,cjs,json}' \
  --glob '!node_modules/**' --glob '!**/.git/**' --glob '!**/.🧬semio/**' .
bun pm why <identity>
```

The first command reports 66 direct JavaScript identities. The name census produced only eight apparent zero-name candidates: `@nx/devkit`, the three root Storybook packages, and four `@types/*` packages. The other 58 identities have an owned static, dynamic, config, test, or runner binding, or belong to an excluded lane. Literal name absence is only a shortlist generator: the installed implementation and resolver behavior below is the deciding evidence.

## Rejected Shortlist

| Identity or group                                                           | Installed-source / resolver evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Why it is not a wave                                                                                                                                                                                                                                |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@vscode/test-electron`                                                     | Installed test-cli desktop runner resolves the identity at line 61, dynamically imports it, and invokes `runTests`.                                                                                                                                                                                                                                                                                                                                                                               | Active runner prerequisite; expressly excluded.                                                                                                                                                                                                     |
| `@storybook/addon-docs`, `@storybook/addon-vitest`, `@storybook/react-vite` | `.storybook/main.ts:13` imports `StorybookConfig` from `@storybook/react-vite`; lines 63–66 resolve both addons and configure the React/Vite framework. Story files and preview/setup import its types.                                                                                                                                                                                                                                                                                           | Live root config and generated-doc/test path; not zero reachability. No Storybook-stack bundling.                                                                                                                                                   |
| `@nx/devkit`                                                                | There are two direct owners: root `package.json` (`21.6.11`) and `framework/.../repo/library/.../package.json` (`21.4.1`). `bun pm why` also shows installed consumers `@nx/js`, `@nx/workspace`, and `@nxlv/python`.                                                                                                                                                                                                                                                                             | Removing only the root row does not retire the identity; removing both is neither a single isolated wave nor zero reachability.                                                                                                                     |
| `@types/node`                                                               | Ten direct manifest owners; project `tsconfig` files explicitly request `"node"` (for example `hub/.../tsconfig.json:18`). Node itself supplies no package metadata containing TypeScript declarations; installed `@types/node` supplies `index.d.ts`.                                                                                                                                                                                                                                            | Live compiler-resolver dependency.                                                                                                                                                                                                                  |
| `@types/react` and `@types/react-dom`                                       | Sixteen and fifteen direct manifest owners, respectively; multiple project `tsconfig` files explicitly list `react` and `react-dom`. There are 135 owned React module edges. Installed `react` and `react-dom` have no `types`/`typings`; both installed `@types` packages expose `index.d.ts`.                                                                                                                                                                                                   | Live compiler-resolver dependencies across many owners.                                                                                                                                                                                             |
| `@types/three`                                                              | Four direct owners: CAD, puzzle, infinite R3F, and renderer React. `three@0.182.0` has no `types`/`typings`; installed `@types/three@0.182.0` exposes `index.d.ts`. Owned code uses `three` types and runtime imports: renderer React `index.tsx:524`, its test `index.test.ts:2422`, and infinite R3F `component.tsx:70,3883`. `bun pm why @types/three` additionally finds active `@react-three/drei → maath` peer (`>=0.134.0`) and `@react-three/drei → stats-gl` dependency (`*`) retainers. | Not zero reachability. Removing all four direct rows would still retain the lock identity through active R3F/Drei internals, would leave four coupled owner edits, and risks type-resolution behavior while not truthfully retiring the dependency. |

The relevant lock evidence for the last row is `bun.lock:2056` (`@types/three@0.182.0`), `:3166` (`maath` peer), and `:3846` (`stats-gl` dependency), plus the workspace tuples at `:370`, `:589`, and `:648`.

## Exclusions And Non-goals

- Compose retainers, the Dagre/Rust/Wasm/OffscreenCanvas lane, Cargo, and all P3/P8-touched files were excluded without proposing changes.
- The root MDX chain, Storybook core/addon stack, and transitive-only identities were not bundled.
- No production source, manifest, lockfile, coordinator artifact, or ticket lifecycle state was changed. This report is the sole scout artifact.
- No runtime replacement is proposed: the remaining positive-use leaves require their own behavior differential and are not a safe zero-reachability retirement.

## Gate State

No pre/post implementation gate applies because no candidate is accepted and no package data was edited. Before writing this report, all three whitespace checks returned no output:

```sh
git diff --check
git diff --cached --check
git diff HEAD --check
```

The shared worktree remains intentionally dirty with concurrent P3/P8, coordinator, prompt, Storybook, manifest, lockfile, and ticket work; that status is not attributed to this scout. A future accepted wave must specify a single direct identity, actual installed-source reachability proof, exact lock closure, an expected coordinator delta, appropriate narrow runtime differential, frozen install, dependency verify/list/parity, and scoped plus whole-tree diff checks.
