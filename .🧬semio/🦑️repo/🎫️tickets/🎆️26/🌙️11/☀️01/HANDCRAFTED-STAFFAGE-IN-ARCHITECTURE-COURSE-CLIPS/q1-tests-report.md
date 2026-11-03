# Q1 staffage test report (final, 2026-11-01)

## Final counts
| Check | Result |
|---|---|
| Library suite (vitest) | 49 of 50 files pass, 920 tests pass, 10 skipped, 0 failing tests. `print-size` fails at import: `Cannot find package 'd3-format'`. |
| Catalog check, all categories, 5 samples | Green. Exit 0, manifest plus 125 assets pass, including server-rack, split-ac-indoor-unit, operating-table, chalkboard, storage-crate-stack and kitchen-extractor-hood. `stepladder` passes (`--asset stepladder`), so no edit was needed. |
| React suite | 11 of 17 files pass, 77 tests pass, 1 skipped, 0 failing tests. 6 suites fail at import of `@semio-tech/framework-transient`. |
| `tsc --noEmit`, library | One error: `print-size` cannot find `d3-format`. The `plan-projection` errors are gone. |
| `tsc --noEmit`, React | Errors only from the missing `@semio-tech/framework-transient`. Not re-run after the last change; same cause as before. |
| `tsc --noEmit`, site | Clean at the last run. |

## Fixes made
- `🧫️fixtures/✅️asset-validation/🔣️.json`: added the `appliances` category.
- `🧪️tests/🎚️config/🟦️.ts`: `testTimeout` and `hookTimeout` of 180 s (uv oracle first run took 48 s, loaded host). Assertions unchanged.
- `🧪️tests/🧰️support/🧰️.ts`: new `hasTool(name)`. ffmpeg is probed through `repo tools exec`, others from PATH. Cases skip with the tool named in the title:
  - `pdf-reopen` qpdf case.
  - `frame-timing` encoded-clip block (ffmpeg and ffprobe).
  - `zip-reopen` CLI case (`rsvg-convert`).
  - The capabilities probe case moved out of the skipped block.
- Tool registry: ffmpeg is registered but missing on this host. qpdf, ffprobe and rsvg-convert are not registered. ezdxf and reportlab run through the registered `uv`.
- React `catalog-loading`: the hard-coded 107 rows now come from the bundled manifest.
- `person-cooking`: removed the duplicate `standing-by` pose (kept the first; the copies differed only in the description wording) and the identical duplicate `stir-and-taste` clip.
- Manifest `analytics` of `laptop`, `desktop-monitor` and `pendant-lamp`: now `["thermal", "transport", "structural"]`, because each has an emitter (the check requires `transport` iff emitters exist).

## Open (infrastructure, not code)
`repo dependencies ensure` fails here with `Current immutable input capture work refused`, after long waits behind other builds. `d3-format` (listed in root `package.json` and `bun.lock`, present in the store as `node_modules/.bun/d3-format@3.1.0`) and `@semio-tech/framework-transient` are therefore not linked into the root `node_modules`. This is the single cause of the `print-size` failure, the 6 React suite failures and the remaining tsc errors. Once the install links them, re-run the library suite, the React suite and both tsc runs. No code change is expected.
