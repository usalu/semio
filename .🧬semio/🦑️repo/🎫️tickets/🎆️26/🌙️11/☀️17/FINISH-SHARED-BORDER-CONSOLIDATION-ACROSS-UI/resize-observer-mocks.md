# ResizeObserver Mocks Get `unobserve`

Every ResizeObserver mock in the nine listed test files now has an `unobserve` no-op next to `observe`, because the shared-border compositor calls `unobserve`.

## Per File

| File | Edited | Result |
|---|---|---|
| `✏️s/🧪️tests/🏘️plugin-composition/🛍️layout-canvas-catalogue/🟦️.ts` | y (1 mock) | Pass, 21 tests (extra config, see below) |
| `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🔭️viewpoints/🟦️.tsx` | y (1 mock, manual edit: `observe` has a body) | Not run: fails at import with ENOENT on the missing fixture `🎓️teaching/🧫️fixtures/🚶️reference-walker/🔣️.json`, unrelated to the mock |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/🧪️tests/🟦️.test.tsx` | y (1 mock) | Pass, 4/4 |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/🧩️component/placement.tsx` | y (1 mock) | Not a suite: the file name does not match the ui include pattern. Its importer `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` fails in jsdom setup with `Invariant violation: "new TextEncoder().encode("") instanceof Uint8Array" is incorrectly false`, before any test runs (environment issue) |
| `🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/🧪️tests/🪶️demand-frames/🟦️.tsx` | y (1 mock) | Not a suite: exports `registerTests2`, which no test file calls ("No test suite found") |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪆️embedded-mount/🟦️.ts` | y (1 mock) | 32 pass, 1 fail: `Failed to resolve entry for package "@semio-tech/framework-surface-rs"` (module resolution, unrelated to the mock) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🖱️selection-focus/🟦️.tsx` | y (21 mocks) | Pass, 75/75 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx` | y (1 mock) | Pass, 26/26 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/⏳️attachment/🟦️.tsx` | y (1 mock) | Pass, 7/7 |

None of the nine files imports `bun:test`.

## Verification

- Each file has one `unobserve` per `observe` mock (Paint2dHost: 21 of 21).
- Os suites ran with `vitest-os.config.ts`, ui suites with `vitest-ui.config.ts`, the teaching suite with `vitest-teaching.config.ts`.
- The two files outside those roots (plugin-composition, demand-frames) ran with `vitest-extra.config.ts`, a ticket-local config rooted at the repo.

## Artifacts in the Ticket

- `add-unobserve.mjs`: the regex script that inserted `unobserve` into the single-line mocks.
- `vitest-extra.config.ts`: ticket-local config for the two out-of-root files.
- `🗑️generated/`: vitest logs for this run.
