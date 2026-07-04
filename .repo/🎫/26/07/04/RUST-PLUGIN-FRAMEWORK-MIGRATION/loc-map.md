# LOC Map — True Full-LOC S Parity

Reference commit: `f8376e848` (readonly `git show`)

## Framework (target ~18k old → new)

| Old | Old LOC | New | Current LOC | Target |
|-----|---------|-----|-------------|--------|
| platform/renderer/react/index.tsx | 5880 | framework/renderer/react/os-shell.tsx | 870 | 5880 |
| playground/renderer/react/index.tsx | 2207 | (merged into os-shell) | — | 2207 |
| platform/core/js/index.ts | 3804 | framework/core/rs/ui.rs + layout.rs | 1374 | 3804 |
| playground/core/js/index.ts | 1566 | framework/core/rs/layout.rs | 419 | 1566 |
| os/core/js/index.ts | 3095 | framework/product/os/core/rs | 2844 | 3095 |
| framework/core/js/index.ts | 2020 | framework/core/rs | 1175 | 2020 |

## S (target ~2100)

| Old | Old LOC | New | Current LOC | Target |
|-----|---------|-----|-------------|--------|
| s/core/js/index.ts | 1579 | s/plugin/rs/lib.rs | 1944 | 2100+ |
| s/react/index.tsx | 521 | s/plugin/rs/lib.rs | (shared) | — |

## Tech plugins (target ~90k cumulative)

See `git show f8376e848:<tech>/core/js` + react for per-tech targets.

## Session baseline (start)
- New implementation total: ~10,656 LOC (framework + s plugin + renderer)
- Old deleted total: ~110,000 LOC
- Gap: ~99,344 LOC

## Session end (2026-07-04)
- Plugin lib.rs total: ~19,562 LOC
- Framework + renderer + s + draw domain: ~40,000+ LOC cumulative
- Old deleted total: ~110,000 LOC
- Remaining gap: ~70,000 LOC (mostly ui-react consumed as-is + incremental tech surface depth)
