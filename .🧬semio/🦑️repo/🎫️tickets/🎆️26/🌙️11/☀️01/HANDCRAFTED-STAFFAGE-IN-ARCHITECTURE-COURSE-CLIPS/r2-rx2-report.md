# R2 rx2 report: React overview, appliances category, i18n and facets (2026-11-01)

Scope: `S/🎯️targets/⚛️react` overview, i18n and catalog-loading, after the appliances category and about 30 new assets (agent mv moved plate-heat-exchanger and sorption-wheel to machinery).

## Findings and fixes

1. **i18n keys: no gaps.** The React overview takes category titles and descriptions from the manifest (`🗂️catalog/🔣️.json`, `{en, de}`), not from the bundles. No per-category key exists in `🌐️i18n/🔠️english` or `🔡️german`, so nothing is missing. The German bundle is still type-checked against the English key set.
2. **Category registration: consistent.** `CATEGORIES` (`🧬️schema/🟦️.ts:38`) lists nine ids in the same order as `categories` in the manifest: humans, animals, vegetation, furniture, appliances, objects, vehicles, machinery, components. `appliances` has en and de title and description, and folder `🔌️appliances` exists with 24 asset folders, matching 24 manifest entries. `CATEGORY_ICONS` (`🏠️overview/🟦️.tsx:114`) is a `Record<CategoryId, IconName>`, so it covers every category. Its `appliances: "plug"` icon is valid, because `plug` is already used by `NOTICE_ICONS` in `🪟️chrome/🟦️.tsx:204`. Typecheck confirms the icon names.
3. **Category emoji missing variation selector (fixed).** In `🗂️catalog/🔣️.json`, `furniture` had `"🛋"` and `machinery` had `"🏗"`, while the other categories and folder names use VS16. Edited to `"🛋️"` (line 46) and `"🏗️"` (line 98). The schema has no pattern for `emoji`, so this is cosmetic consistency only.
4. **Stale feature count (fixed).** `🧪️tests/📚️catalog-loading/🥒️.feature` said "one hundred seven rows are ready". Now reads "every manifest entry is one ready row and the manifest check is empty". The test already checks against `manifest.entries.length`.
5. **Transient catalog failure (resolved, no change needed).** The first React vitest run failed `catalog-loading` with `{ ready: 145, invalid: 2 }` against 147 rows, and the check tool printed `✗ manifest manifest-mismatch entries[136].analytics` for `sorption-wheel`. Both came from concurrent edits to the moved assets. On rerun, `catalog-loading` passes 7 of 7. `sorption-wheel` analytics now agree between manifest and asset. A direct `validateAsset` pass over all 147 asset documents (`🧰️rx2/find-invalid.ts`) found 0 failing assets.
6. **Search and facets.** Facets are categories, analytics, height range, root motion and completion (`🔎️search/🟦️.ts`, `filtersFromParams`). There is no material or function facet. Free-text search matches manifest titles and categories, and asset tags and summaries once loaded. New materials and functions are findable only through tags, which the existing search-filter-sort test covers ("finds tags and summaries once the asset is loaded"). Not added: a material or function facet is a feature, not a fix.
7. **Dev preview.** Not run. The brief allowed relying on tests when a preview is not feasible without `repo dashboard`. The en and de overview was not inspected visually.

## Test output

- React vitest, ticket-local alias config `q2-vitest-react-alias.config.ts` (run from `🎯️targets/⚛️react/📦️packages/🟦️typescript`): **17 of 17 files, 107 passed, 1 skipped**. Same counts as Q2.
- `catalog-loading` alone: 7 of 7 passed after the transient resolved.
- `tsc --noEmit` on the React package: 12 errors, all in `🧰️framework/...` (missing `@semio-tech/framework-transient` link and implicit any in framework UI). None in staffage React files, so the icon and i18n typing is clean.
- Catalog check (`🩺️check --no-physics`): timed out at 500 s before finishing (147 assets). First 57 lines were all `✓`. No further `✗` was visible before the timeout, and the manifest mismatch was no longer present on the rerun.

## Files

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json` (emoji edits, lines 46 and 98)
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/📚️catalog-loading/🥒️.feature` (scenario wording)
- Ticket: `🧰️rx2/find-invalid.ts` (validation script, kept). `🗑️generated/rx2/` deleted.

## Still open

- Visual check of the overview in en and de with a dev preview (not done).
- Material and function facets do not exist. Decide whether the brief requires them.
- Catalog check over all 147 assets with physics on was not finished in this run.
