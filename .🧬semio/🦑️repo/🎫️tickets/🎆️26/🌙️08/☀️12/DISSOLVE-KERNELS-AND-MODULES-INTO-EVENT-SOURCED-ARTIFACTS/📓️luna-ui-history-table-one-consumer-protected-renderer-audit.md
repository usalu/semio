# Luna UI History Table One-Consumer Protected-Renderer Audit

## Scope and Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Read-only Luna audit; no source or Git state changed.
- Active scope excluded `compose`, hub, mit-bestand, legacy/exempt areas, tickets/history, generated output, dependencies, and build caches.

## Definition

The framework UI `HistoryTable` owner exports `HistoryColumnAuthor`, `HistoryColumn`, `HistoryTableProps`, and `HistoryTable`.

| Path | SHA-256 | State |
|---|---|---|
| HistoryTable component | `3b8e2828fe9ce02dfc7f19c51696bf9ffc6bd4006414660e57074e3eeb405c49` | clean |
| HistoryTable story | `9309596f4f84495e77ce476233cc46e3839c8459f55a26093e3d3f13b10af3e0` | clean |
| UI React barrel | `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc` | accepted serialized registrar changes only |
| OS renderer GraphTimelineHost | `fb45a78f633bef91b983ba116729baebb5aa17c55c7b6db3c9c9db5475ab4d7d` | clean |
| OS renderer package index | `e7fb1d126be1fc771e0258fd34cd2131e82ef9302b08b74531b39e6a5c6368a7` | inspected only |

## Consumer Closure

- The canonical UI barrel is mechanical glue.
- The component's story and three UI package SSR tests are example/test evidence.
- `GraphTimelineHost` is the sole direct production consumer. It renders `HistoryTable`, dispatches `checkoutCheckpoint`, and is mounted through the Interpreter `graph-timeline` host registry.
- An OS renderer package-index import is stale/unused glue and not an independent terminal.
- The native Rust graph-timeline scene is a separate renderer implementation, not a consumer of the React component.

## Decision

The framework element has exactly one production terminal consumer and therefore cannot remain a shared module under the two-consumer rule. Its lowest owner is the OS renderer's `GraphTimelineHost`, so it is an inline/collapse candidate, not a zero-consumer deletion. No implementation lease is issued because the only correct atomic closure crosses the protected renderer boundary. Preserve it until that owner is explicitly released and rehashed; then move the implementation, contract, tests, story disposition, package glue, and host validation in one protected-owner lease.
