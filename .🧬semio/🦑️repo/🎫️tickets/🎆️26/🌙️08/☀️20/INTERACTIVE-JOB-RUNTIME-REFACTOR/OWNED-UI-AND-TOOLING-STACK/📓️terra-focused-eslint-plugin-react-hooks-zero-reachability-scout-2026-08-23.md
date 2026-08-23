# Terra Focused Direct Dependency Scout — `eslint-plugin-react-hooks` — 2026-08-23

## Decision

**ACCEPTED candidate: `eslint-plugin-react-hooks`.** This uses the clarified zero-reachability criterion: the package is a direct root tooling declaration with no active binding to replace. Expected dependency boundary: **130 = 67 JavaScript + 63 Rust** to **129 = 66 JavaScript + 63 Rust**.

This is one narrow Phase 10 dependency wave only, not Phase 10 acceptance. It changes no lint policy, runtime behavior, UI source, Storybook behavior, Compose package, Rust path, Cargo input, or Dagre hold.

## Exhaustive Reachability Review

The only non-inventory declaration is root `package.json:185`, `devDependencies["eslint-plugin-react-hooks"] = "^7.1.1"`. No UI React, other workspace, or Compose `package.json` declares it. The root Bun workspace tuple is at `bun.lock:26`; its only package resolution is `eslint-plugin-react-hooks@7.1.1` at `bun.lock:2677`. `bun pm why eslint-plugin-react-hooks` has exactly one path: the root development workspace.

The exhaustive non-generated, non-ticket source/config census found no static import, dynamic import, CommonJS `require`, package-export/name-string use, flat-config registration, script, test, or Nx executor use of `eslint-plugin-react-hooks`. Root `eslint.config.mjs` imports only `@eslint/js`, `typescript-eslint`, and `eslint-plugin-storybook`; its flat composition contains only their configurations. `.storybook/🟦️lint-tooling.ts`, used by UI React's `🟦️eslint.config.ts`, imports only `typescript-eslint` and `eslint-plugin-storybook`. UI React's `📜️script.ts` invokes ESLint with that explicit flat config. `nx.json`, the discovered `project.json` files, and the only other live ESLint config (`compose/dev/algorithm/js/eslint.config.ts`) have no target reference.

There are exactly nine source comments that spell `react-hooks/exhaustive-deps`: two UI element files and seven OS-renderer element lines. They are stale rule-disable comments, not package loading, configuration, or behavior. This distinction was verified against ESLint's resolved configuration while the package remains installed:

```text
UI React entry and comment-bearing PanelTabBar both resolve plugins:
["@", "@typescript-eslint:@typescript-eslint/eslint-plugin@8.66.0", "storybook"]
resolved react-hooks/* rules: []
```

Directly linting the comment-bearing `PanelTabBar` file emits `Definition for rule 'react-hooks/exhaustive-deps' was not found` today. ESLint flat configuration does not auto-register installed packages, so removal preserves this current result. The comments must not be edited in this declaration-only wave; they are a separate lint-source cleanup question.

No in-memory manifest-unavailable probe is needed: both actual resolved configurations omit the package and the direct lint result proves its rule is already unavailable despite installation. A probe that mutated `node_modules` would be less representative and outside this read-only scout.

## Lock Reachability And Exact Boundary

The target resolution depends on `@babel/core`, `@babel/parser`, `hermes-parser`, `zod`, and `zod-validation-error`.

- `@babel/core` and `@babel/parser` are retained by `@nx/js` and `@vitejs/plugin-react`, including the UI React workspace.
- `zod` is retained by `@modelcontextprotocol/sdk` under the framework MCP workspace and by Compose. Compose is not touched or needed for this proof.
- `hermes-parser@0.25.1` is target-only; its `hermes-estree@0.25.1` child is therefore target-only.
- `zod-validation-error@4.0.2` is target-only (with `zod` as its peer).

After deleting only the root declaration and running Bun install, the lock must remove the root workspace declaration plus exactly these otherwise-unretained resolutions: `eslint-plugin-react-hooks@7.1.1`, `hermes-parser@0.25.1`, `hermes-estree@0.25.1`, and `zod-validation-error@4.0.2`. It must retain `eslint`, `@babel/core`, `@babel/parser`, and `zod`. No Compose tuple or resolution is involved. The dependency inventory `🔒️dependencies.json` is the ratchet baseline and is not a live owner or implementation-file target.

## Executor Packet

1. Record the pre-edit dependency list (`130 = 67 JavaScript + 63 Rust`), both resolved `--print-config` plugin lists, the nine-comment census, and the current UI Storybook index/hash evidence: SHA-256 `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`.
2. Delete only root `package.json`'s `eslint-plugin-react-hooks` row. Do not add a substitute plugin, local rule, facade, externalization, compatibility behavior, configuration change, or comment cleanup.
3. Run `bun install`, accepting only the root workspace tuple and four resolution removals described above. Do not change UI/Compose manifests, `eslint.config.mjs`, `.storybook/🟦️lint-tooling.ts`, UI `🟦️eslint.config.ts`, UI `📜️script.ts`, Storybook, Dagre, P3/P8, or Cargo paths.
4. Prove post-install that both actual printed configs retain the exact three-plugin list and no `react-hooks/*` rule; then require pre/post lint parity, the full UI quality suite, frozen install, the 129 dependency ratchet/list/parity boundary, exact absence/retention scans, and clean formatting/diff checks.

No permanent assertion is useful here. The permanent policy is the declarative flat config itself: it never imports or registers this plugin, and the dependency ratchet prevents an unreviewed addition. A second bespoke scanner would duplicate those checks without guarding runtime behavior.

## Independent Audit Gates

```text
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun ./📜️script.ts lint
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
bunx eslint --print-config '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx'
bunx eslint --print-config '🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️PanelTabBar/🟦️component.tsx'
! rg --hidden -n -F 'eslint-plugin-react-hooks' --glob 'package.json' --glob '!node_modules/**' --glob '!.🧬semio/**' --glob '!.git/**' .
! rg -n -F 'eslint-plugin-react-hooks' bun.lock
rg --hidden -n -P "(?:from\\s+|import\\s*\\(|require\\s*\\()[\"']eslint-plugin-react-hooks[\"']" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -F 'react-hooks/exhaustive-deps' --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!.🧬semio/**' --glob '!.git/**' .
bun pm why @babel/core
bun pm why @babel/parser
bun pm why zod
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check package.json bun.lock eslint.config.mjs .storybook/🟦️lint-tooling.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️eslint.config.ts' '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts'
git diff --check
git diff --cached --check
git diff HEAD --check
```

Acceptance requires `129 = 66 JavaScript + 63 Rust`, no root or workspace manifest declaration and no target lock resolution, the four expected orphan-resolution removals only, retained Babel/Zod/ESLint resolutions, no import/config/externalization substitute, preserved nine-comment census, identical resolved ESLint plugin lists, and clean scoped plus whole working/staged/HEAD diff checks. The UI Storybook build must retain its established index/hash evidence and exact `231 = 170 stories + 61 docs`, 61 TS/TSX inputs, 61 Autodocs, and zero MDX boundary.

The closest competing direct tooling leaves remain active: `eslint-plugin-storybook`, `@eslint/js`, `typescript-eslint`, and `eslint` are imported by flat configs; Storybook/addon-docs supplies active docs behavior; and Dagre remains held for the real Rust/Wasm/OffscreenCanvas lane. No production, manifest, lockfile, Git, ticket-lifecycle, or Cargo input was changed by this read-only scout.
