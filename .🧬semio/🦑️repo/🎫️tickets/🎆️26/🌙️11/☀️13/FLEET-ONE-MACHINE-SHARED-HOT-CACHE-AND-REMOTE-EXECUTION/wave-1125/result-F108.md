# F108: atomic schema writers, goal-gate dialect fix

Lane F108, 2026-11-26.

## 1. Atomic writes in the schema commands
- No shared general helper existed (only private ones: cargo manifest, sidecars, unit-cache). New helper `writeTextAtomicV1(path, text, write?)` in `📚️library/🎛️commands/🧬️schema/🔏️atomic-write/🟦️.ts`: unique temp sibling in the same directory, rename, temp removed on failure, old file untouched.
- Every `writeFileSync` in the schema commands is gone: `schema generate` (catalog), `schema docs` (document), `schema check --out` report, `schema audit` (json + md), and `schema twins` (annotate x2, generate target) in `🧬️twin-projection/🟦️.ts`.
- Tests (appended to `🧬️schema/🧪️tests/🟦️.ts`, 23/0): a failing write leaves the old file intact and no temp file behind; a following write replaces it; a source law forbids `writeFileSync(` in both command sources.

## 2. goal-gate
Root cause: the acceptance schema is draft-07 (`$schema` http://json-schema.org/draft-07/schema#), the test forced Ajv2020. The repo supports both dialects (11046 draft-07 vs 1221 2020-12 documents), so the test now selects the Ajv class by the document's `$schema` (Ajv draft-07 default class, Ajv2020 for 2020-12). Strict mode unchanged (`strict: false` was already there). No schema was changed. goal-gate 28/0.

## Verification
- `repo test-file`: goal-gate 28/0, schema command tests 23/0, twin-projection tests (schema-layout, record-homes) pass, catalog-freshness 1/0 (about 109 s).
- catalog-freshness was red at first: a peer changed scope `s.dag.dag` since F107. I regenerated with F107's script (`T/🧰️scripts/F107/regenerate-catalog.ts` into scratch) and swapped only `🔣️schema-catalog.json` atomically under `landing.lock.schema`; the document was current. Only `s.dag.dag` differed.
- `repo typecheck` (schema commands + goal-gate folders) passed, `dangling-imports.py` exit 0.
- A peer's half-landed `📋️build-owners.json` (duplicate `@semio-tech/os-store-schema-exports-rs`) made every test-file fail with "Canonical build owner identity collision" for about 5 min around 17:36-17:40; it cleared by itself.

## Notes
- My source edits were atomic (temp + mv) but landed without a landing lock (area `commands`/`test`); the lock was held only for the catalog swap.
