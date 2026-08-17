# Luna UI Ribbon Multi-Consumer Retention Audit

## Baseline

The bounded audit excluded `compose`, hub, mit-bestand, taxonomy legacy/exempt areas, generated/dependency/build-cache trees, and tests/examples from production-consumer counts.

| Path | SHA-256 | State |
|---|---|---|
| Ribbon component | `8c758ce447bb623f047fa80672d2ef012c410442f89b7ce4bc139e3a68c958a5` | clean |
| Ribbon story | `03498db493d530fd65a348f2054dbc09eac34d2cf3cc3518805dd1c6b88b7ef2` | clean |
| UI React index | `fa8dbb145f3c31af948dc7f18bc51a931cc7cb981fcdac3bd26086e273b99f0b` | accepted serialized UI removals |
| PanelTabBar | `8137457e8460a1023e42e8fa3426e2220bb7b782f17d85df527fe7bb4ce8ecab` | clean |
| Protected renderer UtilityTree | `63b3e4ec99a00a72fd0882587ecc2fca0b473d156852d820a07e6f7b2a008c8d` | inspected only |
| Protected renderer ShellHelpers | `276c5ead02bcf5ea22df5fc056aed003add9077c8ad8791c2457b723ce6b00c8` | inspected only |

## Production Closure

- `Ribbon` has two independent production terminal consumers: framework UI `PanelTabBar` and protected renderer `UtilityTree`.
- `RibbonDivider` has two production uses across `UtilityTree` and `ShellHelpers`.
- `RibbonRow` is consumed by PanelTabBar and UtilityTree.
- Renderer-root imports that are never subsequently used are glue residue, not terminal consumers.
- Ribbon, Window, and Navbar stories plus inline UI package tests do not count.

`RibbonZone`, `RibbonGroup`, `RibbonItem`, `RibbonDirection`, and `RibbonProps` have lower direct fanout but are cohesive facets/contracts of the qualifying Ribbon responsibility; splitting them would not create an independent semantic capability.

## Decision

Retain Ribbon at the framework UI owner. It satisfies the two-independent-production-consumer rule and its current owner is the lowest common owner of PanelTabBar and the OS renderer consumer. No source edit or protected-owner lease follows from this audit.
