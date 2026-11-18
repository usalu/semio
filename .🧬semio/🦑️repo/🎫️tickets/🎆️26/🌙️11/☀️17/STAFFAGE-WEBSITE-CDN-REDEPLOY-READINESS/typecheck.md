# Staffage Typecheck

Run natively: `repo tools exec tsc -- -p <config> --noEmit` (logs in `🗑️generated/`).

| Project | Config | Errors |
|---|---|---|
| staffage/site | tsconfig.json | 24 (0 in staffage files) |
| staffage/site | tsconfig.site.json | 0 |
| staffage/targets/react | tsconfig.json | 0 |
| staffage/packages | tsconfig.json | 0 |

All 24 errors of site `tsconfig.json` are in shared framework files (not modified in the last ~30 min, oldest 3 to 5 days), reached through the `../../🟦️.ts` and builder includes:

- TS2307 missing generated modules (`🤖️generated/...` for manifest, actor, ui-contract): generated output absent on this machine, needs the codegen step, not a source fix. Followed by dependent implicit-any TS7006 in kernel extensions, ui graph, action-argument-resolution and TS2366 in manifest app-definition.
- repo product, owner's: `♻️reset/🟦️.ts` (`phase` lacks `"rescan"`, TS2345) and `🪪️identity/🟦️.ts(340)` (TS2339 `group`).

No files changed. Before: 24 / After: 24 (site tsconfig.json), all others 0.
