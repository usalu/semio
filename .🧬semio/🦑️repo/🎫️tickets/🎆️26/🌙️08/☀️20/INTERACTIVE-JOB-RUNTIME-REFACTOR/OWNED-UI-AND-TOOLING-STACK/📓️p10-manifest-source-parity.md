# P10 Manifest–Source Dependency Parity Audit

Generated from each manifest directory's static source/config/script evidence; `compose/` is excluded. Full per-row evidence is in `📊️p10-manifest-source-parity.json`.

## Totals

| Manifests | Direct rows | External rows | No package-scope evidence |
| ---: | ---: | ---: | ---: |
| 64 | 575 | 263 | 74 |

## High-Confidence Candidate Rule

A row is a candidate only when its manifest directory has no static import/require, recognized config reference, or package-script reference. Dynamic loading and code outside the declared package directory need a package-local allowlist before deletion.

## Proposed Gates

1. `bun ./📜️script.ts verify dependencies parity js --format json` regenerates this data and fails on an undeclared external import/config command.
2. `bun ./📜️script.ts verify dependencies parity js --no-unowned-rows` fails any row with no evidence unless `dependency-audit.allow.json` at that package root names it with a reason and expiry.
3. `bun nx run-many -t test --projects=<affected-projects> --skip-nx-cache` validates each manifest deletion; CI then runs `bun ./📜️script.ts verify dependencies list js` as the freeze ratchet.

## Largest No-Evidence External Groups

- `@types/react`: 10 rows
- `@types/react-dom`: 10 rows
- `@types/node`: 5 rows
- `react-dom`: 4 rows
- `@types/three`: 4 rows
- `@react-three/drei`: 3 rows
- `@react-three/fiber`: 3 rows
- `chevrotain`: 2 rows
- `xstate`: 2 rows
- `@tailwindcss/vite`: 2 rows
- `@tailwindcss/typography`: 2 rows
- `reveal.js`: 1 rows
- `vitest`: 1 rows
- `brepjs`: 1 rows
- `brepjs-opencascade`: 1 rows
- `three`: 1 rows
- `@vitejs/plugin-react`: 1 rows
- `tailwindcss`: 1 rows
- `dagre`: 1 rows
- `@mdx-js/rollup`: 1 rows
- `@vitest/coverage-v8`: 1 rows
- `rehype-autolink-headings`: 1 rows
- `rehype-slug`: 1 rows
- `remark-frontmatter`: 1 rows
- `remark-gfm`: 1 rows
- `remark-mdx-frontmatter`: 1 rows
- `katex`: 1 rows
- `typescript`: 1 rows
- `@bytecodealliance/jco`: 1 rows
- `@napi-rs/canvas`: 1 rows
