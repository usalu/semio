# Repo-Wide TSC Syntax Hygiene

## Root cause

Half-landed `ephemeralBox` refactor: `.current` was pasted into **property names** and **type literals** instead of staying on runtime reads/writes.

## Fixes applied (4 files, 19 diagnostics)

| File | Repair |
|------|--------|
| `✏️s/🔌️plugins/🔱️trinity/…/jack/🧠️lsp/🟦️component.ts` | `graphDomain` / `fixtureJson` in param types and reads (not `*.current` keys) |
| `✏️s/🔌️plugins/🗄️stdio/…/ifc/…/🟦️component.ts` | `PLACEHOLDER_TEXT_COLON: string` (missing colon) |
| `✏️s/🔌️plugins/🗄️stdio/…/step/…/🟦️component.ts` | same colon fix |
| `🧰️framework/…/vscode/…/🟦️extension.ts` | `constructor(public filterProvider?: …)` and `this.filterProvider` in `MonorepoTreeDataProvider` |

## Verification

- **Before:** `🧪️tsc-before.txt` — exit **2**, exactly **19** errors (all syntax).
- **After:** `🧪️tsc-after-incremental.txt` — exit **1**, **8529** errors. All 19 original diagnostics are gone.

## Parse-barrier effect

With any of the four files still syntactically broken, `tsc` reports only those parse errors (~19 total) and does not complete a full-program semantic pass. Once all four parse cleanly, the same command surfaces thousands of pre-existing semantic errors across `compose/`, `.storybook/`, `extension.ts` codegen region, etc.

Fixing only the 19 syntax breaks does **not** yield exit 0 on a fresh `tsconfig.tsbuildinfo` rebuild. Follow-up ticket(s) needed for the latent semantic backlog (or narrowing root `tsconfig.json` include).

## MCP

`repo` MCP server was unavailable in this session (`ticket_open` / `ticket_close` not invoked).
