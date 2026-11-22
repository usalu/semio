# Norm wire twins: canonical wire bytes (2026-11-22)

## Cause
- DIN EN 16798: the snapshot schema gained `zones[].unoccupiedVentilation` (optional, nullable, twin default `null`, schema/twin regenerated 2026-11-21). The TypeScript twin materialises the default, so the canonical re-encoding of every admitted snapshot now spells `"unoccupiedVentilation": null`; the committed mutation fixtures predated it (stale fixtures, schema and encoder are right; same convention as din4108/din18599 fixtures that spell their defaulted nulls).
- EN 1998 `change-foundation-pile-axial-load-n` before/after: hand-edited, non-canonical (integers at number positions, non-schema member order). Unrelated to `cases` content; the fixture already carried it.
- Once fixtures were canonical, the drift gate (`test-wire-twins` runs `drift/🟦️.ts` too) showed the EN 1998 TypeScript twins (`schema/🟦️.ts`, `📸️snapshot`, `🔺️diff`, `🧬️mutations`, `insert-case`) were stale against the schema after the `cases` table / `insert-case` addition (drift 2, style 3).

## Fix
- Rewrote the stale fixtures through the test's own `encode` and twin (script: `norm-wire-twins-rewrite.test.ts`, run with `APPLY=1`; kept in the ticket). Semantic check: all 103 rewritten files are JSON-identical to the originals modulo the added `unoccupiedVentilation: null`.
- Regenerated the EN 1998 TypeScript twins with the framework generator (`generate --under .../en1998`, via `twins-check.ts`).
- No schema, DSL document or Rust change. Rust test serde skips `None`, and its mutate tests compare deserialised values, not bytes.

## Results
`test-wire-twins`: 11886 pass, 0 fail (was 11763 pass, 123 fail).

## Changed files
- 101 DIN EN 16798 fixtures under `din16798/.../🧪️tests/🧫️fixtures/🧬️mutations/*/✅apply/🔣️.{before,after,diff,mutation}.json`
- 2 EN 1998 fixtures `✏️change-foundation-pile-axial-load-n/✅apply/🔣️.{before,after}.json`
- 5 regenerated EN 1998 twins under `en1998/.../🧬️schema/` (`🟦️.ts`, `📸️snapshot/🟦️.ts`, `🔺️diff/🟦️.ts`, `🧬️mutations/🟦️.ts`, `🧬️mutations/➕️insert-case/🧬️schema/🟦️.ts`)
- Ticket: `norm-wire-twins-rewrite.test.ts`, `twins-check.ts`, this report.

## Browser
The regenerated EN 1998 twins are TypeScript sources; a browser bundle that embeds the norm EN 1998 twin should be rebuilt. Fixtures need no rebuild.
