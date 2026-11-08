# fleet-1108 ts-schema

Observed 2026-11-08 (light runs via `repo --service=off tools exec --light bun -- test ./path`):
- Wire corpus `🧪️tests/🧪️wire-twins/🟦️.ts`: 11334 pass, 0 fail (exit 0). Exact-byte formatting and semantic admission both pass (no formatting-hidden failures).
- All other plugin-level bun tests (21 files): 99 pass; 1 timeout-only fail (insert-bindings, 7 s under host load 30+); rerun alone with `--timeout 120000`: 2 pass, 0 fail.
- `🧬️schema-sync/🐍️check.py`: units 2672, rust types 3092, mismatches 0.
- `🏷️field-meta-choices/🐍️check.py`: choices 284, mismatches 0.
- The norm-js package `test` script delegates to `🔬️typescript-package-commands` (covered above).
- Gated `repo run @semio-tech/norm-js:test-wire-twins` was queued before the build gate and is left waiting in capacity; not a needed result.
No source changes were required; no requests filed for other lanes.

## Follow-up
- `🧪️tests/📥️insert-bindings/🟦️.ts`: cause was 227 fresh Ajv instances plus compiles and repeated payload reads. Now one Ajv instance, a compiled-validator cache keyed by (properties, required), a payload parse cache and a map lookup for bindings. Runtime 7 s to 1.5 s with default timeout; 2 pass.
- Per-artifact TS package suites (the `suites:` lists of all 15 `@semio-tech/norm-<x>` scripts) run light, all 0 fail: din16798 22, din18599 36, din4108 24, en1990 20, en1991 43, en1992 11, en1993 39, en1994 21, en1995 29, en1996 58, en1997 10, en1998 29, en1999 9, iso16757 28, vdi3805 118. No requests filed.
