# 🗺️ Puzzle Plugin End-to-End Coordination Plan

## Objective & Deliverables
Get the entire `🧩️puzzle` plugin (`✏️s/🔌️plugins/🧩️puzzle`, `🌎️hub/🧩️compositions/🧩️puzzle`, `✏️s/🧑‍💻dev/🧩️puzzle`, `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle`) working end to end, using native tools (`cargo`, `rustc`, `wasm-bindgen`, `bun`, `vitest`, `playwright`, `semio-plugin-describe`, and envelope-driven `📜️script.ts` runners) whenever the `repo` graph service stalls or build tooling is mid-refactor.

### Required End-State Evidence
1. **TypeScript, Graph Generation, Wiring & Source Laws**:
   - `graph-generate` and `graph-wire-check` green across `◻️2d`, `🧊️3d`, `🖐️5d`.
   - Dead `test-store-close` target/wiring in `◻️2d` cleanly resolved.
   - `@semio-tech/puzzle-js` vitest + `publication-authority-audit` green.
   - `@semio-tech/puzzle-5d-react` vitest (incl. `compose5d-preparetopologymodel`) green.
   - `@semio-tech/s-puzzle-composition-tests` (`puzzle-fill-policy-self-tests`, `browser-contribution`) green.
   - Source-level `verify` laws across `◻️2d`, `🧊️3d`, `🖐️5d`, and `🌎️hub/🧩️compositions/🧩️puzzle` green.
2. **Native Rust Suites (100% Compile & Test Green)**:
   - `semio-os-infinite-board-normal-host` (`board_normal_host_board_host`, 54/54 incl. closed lasso).
   - `✏️s/🔌️plugins/🧩️puzzle` root workspace (`semio-s-plugin-puzzle-jobs`, `semio-s-plugin-puzzle-retained`, `semio-s-puzzle-board-scene`).
   - `🗿️artifacts/◻️2d` all 30 workspace crates + `snapshot-sqlite` + presence law.
   - `🗿️artifacts/🧊️3d` all workspace crates (`semio-s-artifact-puzzle-3d`, `semio-s-artifact-puzzle-3d-editor`, session, window, viewer, model, etc.) + `snapshot-sqlite` + `original-fill-fixture` + `runtime-lifecycle` + `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`.
   - `🗿️artifacts/🖐️5d` all 29 workspace crates + `snapshot-sqlite` + `host-admission`.
   - `🌎️hub/🧩️compositions/🧩️puzzle` (`semio-hub-puzzle` `--lib`, `surface`, `guest-io`).
3. **Browser Engine WASM (`@semio-tech/puzzle-wasm`) & Storybook 2D / 3D / 5D**:
   - Fresh `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle*` built from current Rust sources.
   - Playwright Storybook suites green: `◻️storybook-2d` (12/12 incl. lasso), `🧊️storybook-spatial` (`puzzle2d` 2/2, `puzzle3d` 2/2, `puzzle5d` 4/4), and `📚️storybook-index` (6/6).
4. **Plugin Component (`wasm32-wasip2`), Descriptor/Catalog Publication & Live Editor Lifecycles (`en` + `de`)**:
   - Fresh `semio_hub_puzzle_component.wasm` (`wasm32-wasip2`), `.descriptor.semio`, and transpiled browser component published under `.cache/b/p/m/browser/prototype/modules/🧩️puzzle/`.
   - `puzzle2d` (6012), `puzzle3d` (6013), and `puzzle5d` (6014) activated & served in React prototype mode (`data-semio-os-ready` verified).
   - End-to-end Playwright time-travel / lifecycle suites passing in both `en` and `de`:
     - `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts` (2D)
     - `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️3d-lifecycle/🟦️.ts` (3D)
     - `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts` (5D)

## Fleet Architecture
- **Wave 1 (Parallel `Grok 4.7 Medium` Execution Fleet)**:
  - `lane1-ts-and-laws`: TypeScript suites, `graph-generate`/`graph-wire-check`, dead `store-close` cleanup, source laws, Storybook 2D `"Lasso Select"` story audit/fix.
  - `lane2-rust-2d`: `board-normal-host`, puzzle root crates, and all 30 Puzzle 2D crates + sqlite/presence laws.
  - `lane3-rust-3d`: All Puzzle 3D crates & editor suites (lifecycle/retirement, quiescence, mutation-latency B54, sqlite, fill).
  - `lane4-rust-5d-hub`: All 29 Puzzle 5D crates + sqlite/host-admission and `semio-hub-puzzle` unit/surface/guest-io tests.
- **Wave 2 (Parallel WASM + Component Publication & Browser E2E Fleet)**:
  - Rebuild `@semio-tech/puzzle-wasm` (`wasm32-unknown-unknown` + `wasm-bindgen`) -> run Storybook 2D/3D/5D Playwright battery.
  - Build & publish `semio-hub-puzzle-component` (`wasm32-wasip2` + descriptor + browser module) -> activate & serve 6012/6013/6014 -> run `en` + `de` lifecycle batteries.
- **Wave 3 (Completion Audit & Ticket/Goal Closure)**:
  - Remove all temporary `[DEBUG] ` logs, delete `🗑️generated`, verify zero regressions across every deliverable, close ticket and mark goal complete.
