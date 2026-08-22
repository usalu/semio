# Phase 10 Storybook Dependency Parity

## Scope

This packet owns the live JavaScript dependency-parity census and the root `.storybook` source cohort only. It does not alter manifests, the lockfile, renderer Rust, Puzzle sources, dependency allowlists, or gate severity.

## Census Analysis

The permanent command was run and its exact JSON output regenerated at `📊️p10-manifest-source-parity.json`:

```text
bun ./📜️script.ts verify dependencies parity js --format json
```

Before this source cleanup the live totals were **83 manifests, 308 external rows, 142 evidenced rows, 166 unowned rows, and 240 undeclared imports**.

The red rows separate into three materially different classes:

1. **Scanner evidence gaps, not deletion proof.** Sixty `typescript` rows are attached to packages with owned `tsconfig` inputs and Nx-inferred compiler work, but the scanner only recognizes imports, config package strings, and `scripts`; it does not yet treat a package-owned TypeScript project as compiler evidence. Ambient `@types/*` packages have the same structural gap because correct consumers do not import them. Root executable dependencies invoked by `📜️script.ts` command strings are likewise invisible to the current import-only source pass. These rows must not be deleted merely to satisfy `--no-unowned-rows`.
2. **Genuine undeclared imports.** The largest residual is 113 package-owned test files importing workspace-hoisted `vitest`. Co-located UI stories account for 35 imports of `@storybook/react` and four imports of `@storybook/react-vite` while their deepest owning UI manifest declares neither Storybook package. These are real ownership mismatches, not scanner mistakes.
3. **High-confidence removal candidates requiring their own package gates.** Global source/config/script searches find no non-manifest evidence for the single-owner rows `@types/reveal.js`, `its-fine`, `jose`, and `jsonc-parser`. They are credible later manifest-removal packets, but were not mixed into this source-only cohort because each belongs to a different package and requires lock regeneration plus its package-specific gates.

## Implemented Cohort

Forty-two root-owned `.storybook/stories/**/*.stories.tsx` files imported `Meta` and `StoryObj` from undeclared transitive `@storybook/react`. They now import those types from the directly declared and configured `@storybook/react-vite` renderer package. Its installed declaration entry point explicitly re-exports `@storybook/react`, so the public type contract is identical. All imports are type-only; runtime output is unchanged.

No manifest changed, so `bun install` was intentionally not run.

## Verification

```text
bun nx run workspace:build-storybook --skip-nx-cache
```

Storybook loaded its presets and transformed 68 modules, including the changed story cohort. It then failed on the existing unresolved internal export `@semio-tech/coda-desktop/renderer` in `.storybook/stories/ui/🌳OntologyTree.stories.tsx`; no `@storybook/react-vite` resolution or type-export failure occurred.

```text
bun nx run workspace:lint --skip-nx-cache
```

The workspace lint fan-out remained red on unrelated live-tree failures, including missing repo-script modules, obsolete Compose config paths, and Rust clippy/de-async diagnostics. The `@semio-tech/ui-react:lint` task completed successfully within that fan-out. No changed Storybook import diagnostic occurred.

```text
bun nx run workspace:verify-dependencies-freeze --skip-nx-cache
```

Exit `0`: **185 current identities** against the 238-entry baseline, with 53 removals and no additions.

```text
bun ./📜️script.ts verify dependencies parity js
```

The expected red Phase 10 gate now reports **83 manifests, 308 external rows, 142 evidenced rows, 166 unowned rows, and 198 undeclared imports**. The cleanup removed exactly 42 undeclared imports and did not hide or allowlist any residual.

Static postcondition: root `.storybook` contains zero exact imports from `@storybook/react`; 55 Storybook source files import the configured `@storybook/react-vite` entry point.
