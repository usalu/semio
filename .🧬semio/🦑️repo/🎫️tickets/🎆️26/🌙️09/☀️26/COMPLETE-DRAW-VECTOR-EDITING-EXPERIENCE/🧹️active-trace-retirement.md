# Active Trace Geometry Retirement

The preceding goal turn made verified progress: complete-plan retirement was authored and checked with 518 native and 561 full TypeScript passes. This continuation re-read active trace ownership and authored the next kernel boundary.

## Authoritative Changes

The shared bitmap trace schema now defines retirement progress. Eleven neutral interruption cases cover a fresh job, all eight real phases using the staircase simplification fixture, an actual edge-cap failure, and an already cancelled job with no retained mask. Both implementations add `into_retirement` / `intoRetirement`: these transfer the genuine source mask back to the caller without copying (or return None/null when prior cancellation released it) and move the active private geometry into a retirement owner. The trace job becomes cancelled immediately. Existing synchronous cancellation remains an eager path; mounted integration must use the new ownership transfer rather than claiming that old cancellation has changed.

Cleanup has thirteen fixed slots. Flat POD buffers are released individually. Nested raw/base/candidate contour collections consume one flat contour per unit and then release the empty outer buffer. Outgoing, position, kept and coverage-change trees consume one entry per unit. TypeScript preserves a single iterator while retiring each Map/Set, avoiding repeated scans of deleted prefixes. Its closure is released only when every slot completes. Native termination removes the whole job only after every owned collection is empty and its mask has already transferred. These are structural work units, not allocator byte credits. The returned source storage remains the caller's ownership responsibility.

Tests compare actual work against a separately serialized JSON inventory, validate TypeScript progress with AJV, preserve exact source mask identity and bytes, reject empty grants, and prove idempotent terminal progress. Native requires the final internal owner to be None; TypeScript checks all actual transferred collections are empty. No geometry result is published after transfer.

## Verification

- Red TypeScript gate `trace-retirement-red.log`, handle 44919: exit 1, 140 passed and one failed specifically because `intoRetirement` was absent.
- Full 2D TypeScript gate `trace-retirement-ts.log`, handle 45569: exit 0, 141 passed with strict checks.
- Final 2D TypeScript gate `trace-retirement-ts-terminal.log`, handle 26747: exit 0, **141 passed / zero failed**, seven files and strict checks. Eleven authored cases now require the same exact work totals in both languages. Invalid numeric grants, repeated transfers, post-transfer cancel, and retained published paths are covered.
- Full 2D native gate `trace-retirement-native.log`, handle 95745: exit 0, **50 passed / zero failed or skipped**. Updated exact-count full gate `trace-retirement-native-final.log`, handle 87822: exit 0, **50 passed / zero failed or skipped**.
- Native runtime gate `trace-retirement-native-runtime.log`, handle 24439: exit 0, one passed / 49 intentionally filtered; `[DEBUG]` printed exact inventories for fresh, all eight real phases and failure.
- Final native edge-case gate `trace-retirement-native-terminal.log`, handle 14105: exit 0, one passed / 49 intentionally filtered, including all eleven exact-count cases and actual `[DEBUG]` output. Shared Cargo wait explains the 7m8 total. All trace retirement gates are terminal.
- Full Draw TypeScript `trace-retirement-draw-ts.log`, handle 25969: exit 0, 560 passed / one PDF oracle skipped / zero failed. Re-run with the inspected ticket PDF inputs enabled, `trace-retirement-draw-ts-oracle.log`, handle 6419: exit 0, **561 passed / zero failed or skipped**, plus strict production and independent/publication/scheduler checks. The PDFs are existing ticket inputs, not newly generated outputs in this turn.
- Full Draw native `trace-retirement-draw-native.log`, handle 61583: exit 0, **518 passed / zero failed or skipped**.
- Scoped `git diff --check`: exit 0.

Outputs remain under the ticket generated directory. The existing 2D Nx commands and launch entries already cover the modified tests; no script or executable command was added.

## Files

Changes are in `🧰️framework/🔨️modules/◻️2d/🔍️trace`: production `🦀️.rs` and `🟦️.ts`, `🧬️schema/🔣️.json`, new `🧫️fixtures/🧹️retirement/🔣️.json`, and existing `🧪️tests/🦀️.rs` and `🧪️tests/🟦️.ts`.

## Remaining Scope

Trace wrapper and decode ownership, flatten/Boolean kernel retirement, plugin byte-credit admission, real mounted Draw scheduling and canvas consumption remain unfinished. Typography outlines, complete IO, browser journeys and multiuser verification are also still required. The complete editor goal and ticket remain active; repo MCP lifecycle tools are unavailable, and no ticket lifecycle operation is claimed.

## Next Wrapper Evidence

ImageDecodeJob owns optional BinarySourceJob and PngDecodeJob children and a flat RasterImage pixel buffer. BinarySourceJob has a shared encoded source, bounded MIME/error strings and two flat byte buffers. PngDecodeJob has a shared byte source, palette/alpha/range/scanline buffers and a PngInflater whose Drop invokes at most four first-party close_retained_step operations. This fixed structural destructor evidence does not establish byte-credit admission; capacity accounting remains required.

PathFlattenJob retains flat source segments, a flat subdivision stack and an outer FlatContour vector whose elements each own a point buffer. Success and cancellation currently release that outer structure synchronously. Its completed contour result can be shared by TypeScript callers; retirement must preserve published geometry rather than blindly pop a caller-visible output array. Document trace publication likewise needs an explicit owned-result handoff before retiring a complete producer, because a scene plan contains deeply owned group/reference/style containers.
