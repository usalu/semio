# Terra Independent Audit — `eslint-plugin-react-hooks` Retirement — 2026-08-23

## Verdict

**ACCEPT.** The root-only zero-reachability retirement is a correct, narrow dependency wave: **130 = 67 JavaScript + 63 Rust** became **129 = 66 JavaScript + 63 Rust**. This accepts the dependency wave only; it is not Phase 10 acceptance.

No blocker was found. The live target diff is exactly one root `package.json` deletion and nine `bun.lock` deletions. It introduces no source/configuration change, replacement plugin, facade, compatibility behavior, externalization, Storybook change, Compose change, Dagre change, P3/P8 overlap, or Cargo action.

## Independent Reachability And Lint Parity

The exact non-ticket/non-`node_modules` census finds no static import, dynamic import, CommonJS `require`, package-name/export reference, flat-ESLint registration, Nx registration, script, or test binding for `eslint-plugin-react-hooks`. The only remaining non-ticket occurrences are the intentional ratchet inventory row and nine `react-hooks/exhaustive-deps` disable comments: two UI element comments and seven OS-renderer comments. There is no current package-manifest declaration.

`bun pm why eslint-plugin-react-hooks` exits `1` with `No packages matching ... found in lockfile`; no target retainer remains. Root `eslint.config.mjs` and `.storybook/🟦️lint-tooling.ts` import only their active ESLint/TypeScript/Storybook implementations. UI React's `🟦️eslint.config.ts` delegates to that tooling config, and its `📜️script.ts` passes it explicitly to ESLint.

Both the UI React entry and comment-bearing `PanelTabBar` resolve to the exact reliable pre-image configuration:

| Check                               | Current result                                                                                                           |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Normalized `--print-config` SHA-256 | `4349c703608be75bbe58026df316f94685faaa9872af6aa705cce7d117ade7af`                                                       |
| Plugins                             | `@`, `@typescript-eslint:@typescript-eslint/eslint-plugin@8.66.0`, `storybook`                                           |
| `react-hooks/*` rules               | none                                                                                                                     |
| Disable comments                    | 9, unchanged                                                                                                             |
| Representative direct lint          | 19 errors: 18 `@typescript-eslint/no-unused-vars` plus `react-hooks/exhaustive-deps` unknown-rule at `PanelTabBar:479:5` |

The package was never registered in the flat config. The unknown-rule diagnostic therefore already occurred while it was installed and remains identical after removal; the nine comments are intentionally outside this declaration/lock wave.

## Exact Lock And Manifest Boundary

The live target-only unstaged diff against the already-staged predecessor is **10 deletions**:

- root `package.json`: `eslint-plugin-react-hooks` declaration;
- root Bun workspace tuple;
- `eslint-plugin-react-hooks@7.1.1` resolution;
- target-only `hermes-parser@0.25.1` and `hermes-estree@0.25.1` resolutions;
- target-only `zod-validation-error@4.0.2` resolution.

The target and all three orphan resolutions are absent from both manifest and lock. `eslint@10.8.0`, `@babel/core@7.29.7`, `@babel/parser@7.29.8`, and `zod@4.4.3` remain as top-level resolutions. `bun pm why` independently retains Babel core/parser through `@nx/js` and `@vitejs/plugin-react`, and Zod through `@modelcontextprotocol/sdk` (and separately Compose). The staged/HEAD combined diff also contains the prior accepted `@mdx-js/rollup` retirement and its Bun lock re-homing; it is not attributed to this target. The target-only diff has no additions or unrelated lock churn.

## Independently Run Gates

| Command / inspection                                                      | Result                                                                                                                                                             |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                  | Passed                                                                                                                                                             |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`             | Passed                                                                                                                                                             |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`            | Passed, 724/724 tests in 21 files                                                                                                                                  |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache`                 | Passed, uncached, exit 0                                                                                                                                           |
| Parsed `storybook-static/index.json`                                      | 231 entries = 170 stories + 61 docs; 61 unique TS/TSX inputs; 61 Autodocs; 0 MDX; exact SHA-256 `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4` |
| `bun install --frozen-lockfile`                                           | Passed; 1,945 installs across 1,993 packages, no changes                                                                                                           |
| `bun ./📜️script.ts verify dependencies`                                   | Passed; current 129, no new dependency                                                                                                                             |
| JS/Rust dependency lists                                                  | Exactly 66 JS and 63 Rust                                                                                                                                          |
| `bun ./📜️script.ts verify dependencies parity js`                         | Passed: 83 manifests, 245 external rows, 103 evidenced, 142 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures                       |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'` | Passed                                                                                                                                                             |
| `bunx prettier --check package.json`                                      | Passed                                                                                                                                                             |
| `bunx prettier --check bun.lock`                                          | Not applicable: exit 2, no Prettier parser for Bun lockfiles                                                                                                       |
| `git diff --check`; `git diff --cached --check`; `git diff HEAD --check`  | All passed                                                                                                                                                         |

The successful Storybook build still emits its existing CSS-selector, asset-resolution, docgen, browser-externalization, and chunk-size warnings. They are unrelated to this deletion, and the exact index/hash parity shows no changed Storybook discovery result.

## Broad Root Lint Context

The implementation report records the pre/post root `bun ./📜️script.ts lint` as red on broad unrelated repository/Compose/TypeScript-policy/Clippy/async-drift classes. This audit does **not** count that red result as a pass or as evidence for this wave. I did not rerun the root orchestration: its `nx run-many -t lint --all` includes `@semio-tech/framework-rs:lint`, whose own `📜️script.ts` invokes Cargo, prohibited by this audit. Concurrent Rust edits also make a byte-identical root diagnostic comparison unreliable.

The direct UI lint target, resolved root/UI lint configurations, representative existing diagnostic, frozen install, exact lock delta, and Storybook parity were independently executed instead. They establish the only behavior affected by this root tooling declaration. This broad-root limitation is not a blocker for the narrow acceptance and must not be promoted into a claim that global lint is green.

## Scope And Handoff

Reviewed reports:

- `📓️terra-focused-eslint-plugin-react-hooks-zero-reachability-scout-2026-08-23.md`
- `📓️p10-owned-eslint-plugin-react-hooks-retirement-2026-08-23.md`

This read-only audit created only this report. No production, manifest, lockfile, Git, lifecycle, temporary-outside-ticket, or Cargo input was changed by the audit.
