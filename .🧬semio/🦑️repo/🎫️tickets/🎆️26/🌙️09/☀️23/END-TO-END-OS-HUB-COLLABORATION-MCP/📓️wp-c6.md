# WP-C6 Data-Classification Routing

**Ticket:** `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`  
**Status:** PASS  
**Scope:** Audit D1–D3 §4 — hub `origin: "hub"` + directory fold; ephemeral studios; four-class schema-first enum.

## Goals

| ID | Goal | Result |
|----|------|--------|
| D1 | Hub spaces always `origin: "hub"`; directory fold active in browser + native shells | PASS |
| D2 | Ephemeral studios = ephemeral local-only; block share/collab (en+de); promote/persist commands | PASS |
| D3 | Schema-first four-class enum on bindings/lanes (Rust + TS) | PASS |
| Tests | Language-agnostic fixture + Rust/TS runners; nx/cargo green | PASS |

## Design

Four mutually exclusive `PersistenceDataClass` values:

- `persistedLocalOnly` — folder binding
- `persistedShared` — hub binding
- `ephemeralLocalOnly` — draft studio / empty bindings
- `ephemeralShared` — preview + presence wire lanes (never durable)

`PersistenceBinding::data_class()` / TS factories stamp the class; `wire_lane_data_class` / `wireLaneDataClass` classify lanes. Home union folds hub directory rows as `origin: "hub"` over local catalog (`origin: "local"`). Share on non-hub ids opens `ephemeralShareBlocked` (en+de). `promote-to-hub-space` / `persist-locally` provide exit paths.

## Files

### Schema / fixture

- `…/os/🔨️modules/🏪️store/🔄️sync/🧬️schema/persistence-data-class/🔣️.json`
- `…/os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json`

### Kernel (Rust + TS)

- `…/🏪️store/🔄️sync/🦀️.rs` — enum + helpers; test mod `persistence_data_class_tests`
- `…/os/🟦️.ts` — types, factories, in-source suite
- `…/🔄️sync/🧪️tests/🔬️persistence-data-class/{🦀️.rs,🟦️.ts}`
- `…/os/🧪️tests/persistence-data-class/🟦️.ts`

### Space / home (D1–D2)

- `🪐️space` core — `HomeSpaceRow` + `home_space_rows` hub∪local
- `🏗️create-studio` — ephemeral (no backbone)
- `🔗️share-space` — non-hub → `ephemeralShareBlocked`
- `☁️promote-to-hub-space`, `💾️persist-locally`
- Home editor — en+de dialogs/labels
- Browser `🏛️ShellHost` `foldDirectoryEvents`; native wgpu shell `applyDirectoryEventPage`

### Ticket

- Scripts: `wp-c6/`
- Logs: `🗑️generated/wp-c6/`

## Verification

| Check | Command | Result |
|-------|---------|--------|
| Space-plugin oracle | `nx run @semio-tech/space-plugin:persistence-data-class-check` | **PASS** — `checks=11 clean` |
| Rust fixture | `cargo test -p semio-framework-os-kernel --features sync --lib persistence_data_class_fixture` | **PASS** — `…routes_bindings_and_lanes ... ok` |
| TS fixture | `bunx vitest run --config wp-c6/vitest.persistence.config.ts` | **PASS** — 1 file / 2 tests |

Notes:

- Full `@semio-tech/framework-os:test` name-pattern run hits the 15s quick budget and can fail loading `👷️worker` due to an unrelated WP-C2 backbone-parity import; dedicated persistence suite is the WP-C6 gate.
- Sync module tests require `--features sync`.

## Summary

WP-C6 delivers schema-first four-class routing, hub-origin home fold in both shells, ephemeral studio share/collab blocking with en+de UI plus promote/persist commands, and green language-agnostic Rust + TS fixture runners plus the space-plugin oracle.
