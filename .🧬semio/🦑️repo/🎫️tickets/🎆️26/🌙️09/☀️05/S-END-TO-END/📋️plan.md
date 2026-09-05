# S End To End — Plan of Record

Ticket: `26/09/05/S-END-TO-END` · Goal: `🎯r2603` · Coordinator: Claude Fable 5.1 (this session) · Implementers: Opus 5 (`📓️opus-*.md`) · Explorers: Sonnet 5 (`📓️explore-*.md`)

Sibling ticket run by a separate GPT fleet: `26/09/02/COMPLETE-SEMIO-END-TO-END` (hub, AI, admin, provider authority). This ticket owns only the `s` OS shell surface: every registered plugin/app/artifact opens and renders in the React shell from the registered launch targets, with WGPU wasm and native parity as the second and third tiers.

## Definition of done

1. `🛠️dev🖥️s⚛️react` boots to the readiness beacon with the complete live catalog (59 registry rows) installed, no plugin in `failed`/`crashed` status.
2. A registered catalog-wide smoke (`framework-os-dev verify catalog`) spawns every program inside one `s` session and proves a rendered window with zero page errors for each declared app.
3. Every plugin has a fresh owner descriptor pair and the registry `check` is fail-closed and green.
4. `framework-os-dev test-quick`, `.storybook/os-plugins.spec.ts`, `.storybook/s-end-to-end.spec.ts`, `verify e2e`, and `verify collab` run non-vacuously and pass.
5. `🛠️dev🖥️s🧊️wgpu🌐️wasm` boots and `parity verify s` passes; `🛠️dev🖥️s🧊️wgpu🖥️native` boots and opens Home/Studio.

## Baseline — 2026-09-05 03:50

- Served React shell (`dev s served`, port 6070): booting; evidence pending in `📓️baseline-runtime.md`.
- `framework-os-dev:test-quick`: RED, the vitest run exceeds the 30 s quick budget and is killed (`[budget] … exceeded 30000ms`).
- `plugin-registry:check`: RED, crashes with `ENOENT` scanning a concurrent agent's `target-block/` root before reaching the descriptor gate.
- Catalog cache: 57/59 rows have core wasm + JS; `draw`, `layout` unbuilt; `energy` descriptor without module; 19 rows (block, playbook, stdio, trinity + 15 extensions) have no owner descriptor pair; 8 pairs semantically divergent; 4 CAD extension pairs are placeholders.
- `.storybook/os-plugins.spec.ts` and `OsBootHost` import a non-existent `🤖️generated/🟦️plugins.ts`.
- `semio-s-plugin-stdio` reported non-compiling by peer session semio-f4 (`#[path]` mount drift after the emoji rename); f4 owns the mount repair. Native check census running into `target-s-e2e`.
- Action classification: 427 `Migrated` vs 414 `BatchOnlyPendingRewrite` repo-wide; norm (15 apps) at 0 migrated; puzzle 2d/3d/5d, note, flow, fem, cad, gis, dag, imperative, reasoning viewers carry large dead sets.
- WGPU wasm: no completed trunk build since 2026-09-03; native: `ProgramBridge::attach_backbone` is a hard `Err` stub.

## Waves

### Wave 1 (now, independent of stdio)
| Lane | Model | Scope | Report |
|---|---|---|---|
| A `catalog-smoke-harness` | Opus | fix storybook `🟦️plugins.ts` import; make `test-quick` honest within budget; add `verify catalog` Playwright smoke that spawns every program inside `s`; register in project.json + launch.json; run it against the served shell | `📓️opus-catalog-smoke-harness.md` |
| B `descriptor-producer` | Opus | raw+core receipt `describePluginComponent` contract; shared extension `describe` route + targets for 26 extensions; registry `check` fail-closed on missing/divergent pairs; classification-drift gate; re-emit pairs where raw components exist | `📓️opus-descriptor-producer.md` |
| C stdio census | coordinator | native `cargo check --keep-going` census in `target-s-e2e` | `📓️stdio-check-census.md` |
| D runtime baseline | coordinator | browser evidence for the served shell | `📓️baseline-runtime.md` |

### Wave 2 (after stdio compiles)
- stdio non-mount compile errors + `describe`; then dependency-first rebuild of the catalog into the cache (`draw`, `layout`, `energy`, `block`, `trinity`, `playbook`, animate/remodel/fem mutation modules).
- Shared bounded tool-job factory for the 15 norm apps; `PuzzleCommandWork::step` app-instance parameter; viewer action migrations per plugin (one Opus lane per plugin family).
- Descriptor re-emission for all 59 rows; registry check green.

### Wave 3
- WGPU wasm `s` boot + `parity verify s`; native `attach_backbone` replacement and native Home/Studio boot; `verify collab` green.

## Coordination rules for every lane
- No `git commit/stash/checkout`, no worktrees, no ticket close/reopen, never delete another ticket's `🗑️generated`.
- Write build/boot logs to the scratchpad or this ticket's `🗑️generated/`; reports as `📓️*.md` in this ticket.
- Foreground commands only; `cd /Users/ueli/Documents/semio` explicitly per call; private `CARGO_TARGET_DIR` for any cargo work.
- Re-read a region immediately before editing; attribute failures to current evidence, never revert peer work.
