# Phase 10 Owned `eslint-plugin-react-hooks` Retirement Implementation

Date: 2026-08-23  
Status: Implementation complete; independent audit pending

## Scope

This narrow Terra implementation retired only the unused direct root
`eslint-plugin-react-hooks` declaration and the lock entries that became
unreachable from that removal.

Authorized implementation surfaces:

- `package.json`
- `bun.lock`
- this dated ticket report

No React Hooks disable comment, ESLint configuration, lint rule, Compose file,
other dependency declaration, coordinator artifact, Phase 3 artifact, or Phase
8 artifact was edited by this packet. No substitute plugin was introduced. No
ticket or goal lifecycle operation and no modifying Git operation was performed.

## Pre-Edit Evidence

The pre-edit dependency boundary contained 67 JavaScript dependencies and 63
Rust dependencies. The direct root manifest declaration and its Bun root
workspace tuple were the only dependency declarations for
`eslint-plugin-react-hooks`.

Exhaustive static imports, dynamic imports, `require(...)`, configuration,
`.storybook`, owned React UI, Compose algorithm, Nx project configuration, and
repository script scans found no runtime or tooling consumer. `bun pm why
eslint-plugin-react-hooks` identified only the root development workspace.

The source tree contained exactly nine existing
`react-hooks/exhaustive-deps` disable comments. They were not backed by a loaded
plugin or configured rule. Before the edit, the resolved root and representative
UI lint configurations both had this plugin inventory:

```text
@, @typescript-eslint:@typescript-eslint/eslint-plugin@8.66.0, storybook
```

Both configurations had no `react-hooks/*` rules, and their complete normalized
configuration SHA-256 values were identical:

```text
4349c703608be75bbe58026df316f94685faaa9872af6aa705cce7d117ade7af
```

Directly linting the representative `PanelTabBar` file exited 1 with the same
19 diagnostics expected by the scout: 18 existing unused-variable diagnostics
and the existing line 479 diagnostic that the definition for
`react-hooks/exhaustive-deps` was not found. The pre-edit owned React UI lint
target passed. The prescribed root lint baseline was already red on unrelated
repository, Compose, TypeScript-policy, Clippy, and async-drift failures; it did
not report a React Hooks dependency or configuration failure.

The pre-edit Storybook production build passed. Its guard observed 170 stories,
61 docs, 61 TypeScript/TSX inputs, and zero MDX files. Independent index parsing
observed 231 entries, 170 stories, 61 docs, 61 inputs, 61 autodocs, zero MDX
entries, and zero unsupported entries. The generated index SHA-256 was:

```text
72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4
```

## Implementation

The root `devDependencies` row for `eslint-plugin-react-hooks` was removed and
`bun install` reconciled the lockfile. Bun reported one package removed.

The lockfile delta is exactly the root workspace tuple row plus these now
unreachable resolutions:

- `eslint-plugin-react-hooks@7.1.1`
- `hermes-parser@0.25.1`
- `hermes-estree@0.25.1`
- `zod-validation-error@4.0.2`

The focused implementation diff is ten deleted lines: one in `package.json`
and nine in `bun.lock`.

The shared lock still retains the independently reachable resolutions for
ESLint, Babel core, Babel parser, and Zod. Post-edit `bun pm why` shows Babel
core and parser retained through Nx and Vite React tooling and Zod retained
through the MCP framework and Compose. No package matching
`eslint-plugin-react-hooks` remains.

## Post-Edit Parity

The post-edit root and representative UI resolved ESLint configurations retain
the exact pre-edit plugin inventory, no React Hooks rule, and the exact pre-edit
configuration SHA-256. The representative direct lint run retains the exact
same 19 diagnostics, including the pre-existing unknown-rule diagnostic. The
same nine disable comments remain byte-for-byte in place.

The owned React UI lint, typecheck, quick test, and uncached Storybook production
build targets passed. Quick tests passed 724 of 724 tests in 21 files. The
Storybook guard counts, independent index parse counts, and index SHA-256 are
identical to the pre-edit baseline.

Dependency verification reports baseline 238, current 129, removed 109, and no
new dependency. The current boundary is exactly 66 JavaScript dependencies and
63 Rust dependencies. JavaScript parity reports 83 manifests, 245 external
rows, 103 evidenced rows, 142 unowned rows, zero undeclared imports, 44 lock
workspaces, zero lock mismatches, and five lock fixtures.

## Verification Results

| Verification                                              | Result                                                   |
| --------------------------------------------------------- | -------------------------------------------------------- |
| Exhaustive import, require, config, Nx, and script scans  | Passed; no consumer remains                              |
| `bun pm why eslint-plugin-react-hooks`                    | Expected no-match exit 1                                 |
| Resolved ESLint configuration equality                    | Passed; exact SHA and inventory preserved                |
| Representative direct ESLint diagnostic equality          | Passed; exact 19 diagnostics preserved                   |
| Owned React UI lint                                       | Passed                                                   |
| Owned React UI typecheck                                  | Passed                                                   |
| Owned React UI quick tests                                | Passed, 724/724                                          |
| Uncached owned React UI build and index parse             | Passed; exact baseline parity                            |
| Root script syntax build                                  | Passed                                                   |
| Dependency verification, lists, and JavaScript parity     | Passed                                                   |
| Frozen Bun install                                        | Passed; 1,945 installs across 1,993 packages, no changes |
| Lock removal and retention assertions                     | Passed                                                   |
| Focused `package.json` Prettier check                     | Passed                                                   |
| Focused and whole working, staged, and `HEAD` diff checks | Passed                                                   |

The scout's literal multi-file Prettier command exits 2 because Prettier has no
parser for `bun.lock`. Its parseable subset exits 1 only for pre-existing,
unchanged formatting drift in root `eslint.config.mjs` and the owned React UI
`📜️script.ts`; those files are outside this narrow retirement and were not
formatted. The changed parseable manifest passes its focused Prettier check.

The prescribed post-edit root lint remains red on the same broad out-of-scope
classes already captured before the edit and does not introduce a React Hooks
dependency or configuration failure. Concurrent work changed some individual
Rust diagnostics during the packet, so root-wide diagnostic text is not claimed
to be byte-identical.

## Handoff

The exact focused retirement is complete. Independent audit and coordinator
acceptance remain separate steps; this report does not alter their records or
claim their approval.
