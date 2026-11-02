# Storybook Asset Preparation

## Result

The existing generators work without dependency-store publication. Ran the real generators against authored data: **8 styling artifacts and 262 asset artifacts**, including both missing browser imports. Both full freshness checks pass. **250 catalog icons** match an independent filesystem scan and parse as SVG. No fallback content or stubs were introduced.

No implementation edits are necessary inside either generator. The missing piece is the Storybook prerequisite edge, outside this agent's write ownership. Root and shared routing files were not changed.

## Existing Ownership and Root Cause

- Styling renderer: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts`. Reads authored `🔣️.json`, font catalog and adapter manifest; emits CSS/TS/C#/Rust/Python.
- Its canonical target is `@semio-tech/ui-styling-tokens:generate`, implemented by `styling/📦️packages/🦀️rust/📜️script.ts` using `generateStylingArtifacts(writeGeneratedFileIfChanged)`. Despite the package directory name, this generation is TypeScript and needs no Rust compiler.
- `@semio-tech/ui-styling:generate` is intentionally an aggregate/noop depending on the token target. Its TypeScript script's logging-only generate handler is not the canonical generation target.
- Icon renderer: `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts`. Reads authored SVGs and shortcode data.
- Canonical target `@semio-tech/assets:build` uses `publishAssetArtifacts()` from `assets/🏗️builder/📦️publication/🟦️.ts`; this owns catalog plus metabolism outputs and exact freshness/membership checks.
- Root `prepare` already depends on both correct targets. Root `dev-storybook*` targets have no preparation prerequisite, and `build-storybook`/`test-storybook` have empty `dependsOn`. `setup-storybook` is a separate manual setup command, not an automatic dev/build prerequisite.
- The previously observed `repo run` block is unpublished dependency-store staging, not either generator. No repeated store waits or dependency-store changes were attempted here.

## Requested Lead Wiring

Add `@semio-tech/ui-styling-tokens:generate` and `@semio-tech/assets:build` prerequisites to every canonical Storybook dev/build/test path, or route them through one shared Storybook preparation aggregate. Reuse these existing deterministic targets instead of new generators. Root `prepare` is broader and includes schema/graph/plugin generation; choose it only if those other outputs are also necessary for the full catalog.

If direct `repo ./📜️script.ts dev storybook ...` routing remains a supported entry, shared routing must ensure the same preparation, not rely solely on project target edges. This routing decision remains with the lead.

## Reproduction Without Published Store

A retained ticket input script invokes the exact existing generator functions and both freshness checks, not task dispatch:

```sh
PATH="$HOME/.local/share/semio/launcher/bin:$PATH" repo tools exec --light bun -- './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/prepare-storybook-assets.ts'
```

This prints `Prepared and verified 8 styling and 262 asset artifacts from authored sources.` It is bootstrap evidence, not a new canonical Storybook runtime dependency. No new packages were needed.

## Validation

- Pure render before publication succeeded: 8 styling outputs, 262 asset outputs.
- Real publication and `checkStylingArtifacts()` / `checkAssetArtifacts()` succeeded.
- Replayed retained preparation script successfully, proving repeatable invocation.
- Independent existing third-party libraries: `fast-glob` source discovery matched all 250 generated icon IDs; `jsdom` XML parser accepted every generated SVG; `postcss` parsed all 4 emitted CSS outputs, each matching on-disk bytes.
- An initial independent test stripped emoji with an ASCII prefix regex and failed on keycap digits. Replaced only that ad-hoc oracle with standard `Intl.Segmenter` grapheme splitting; the corrected oracle passes. No generator defect was inferred from the faulty test.
- `git diff --stat` after generation was empty: generated artifacts are ignored or identical to tracked authored outputs. No tracked generator/component/Storybook source was changed by this assignment.
- Final shared-worktree status contains concurrent Storybook/package/story changes by other executors. They are outside this assignment and were left untouched; no generator-owned tracked diff was introduced here.
- Browser validation remains with lead; asset preparation alone does not certify the entire catalog.

## Files and Remaining Work

Created ticket inputs/report: `prepare-storybook-assets.ts`, `preparation.md`. Generated canonical owner outputs in existing generator-owned locations; not added to version control. No ticket temporary output/log folder was created.

Remaining: lead applies shared/root prerequisite wiring, tests the fresh-checkout preparation entry and reruns browser probes. MCP goals/ticket lifecycle remain unavailable as stated in supplied handoff; no ticket state was fabricated.
