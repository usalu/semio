# Official Tool Jobs R9 Working Inventory

Source: `📊️coordinator-official-tool-jobs-live-r9-working-2026-08-27.json`.

This is a working snapshot while Flow, Note, and Sequence are in flight; it is not a final gate.

## Counts

- Unique command rows: 772
- Admitted bounded rows: 147 (r8 working snapshot: 118)
- Batch-only rows: 128
- Remaining fail-closed registrations: 661
- Scan-then-monolith routes: 53
- Process-global payload candidates: 16 (r8 working snapshot: 25)

## Gate findings

- Flow/Note `Migrated declaration has 0 exact bounded reducer proofs` failures are gone after exact proof discovery repair.
- Jack `.spr`/`.ops` and Trinity Rewrite retained-envelope requirements remain red.
- Puzzle clipboard/import exact factories remain red.
- 36 app-owned import-media routes and four framework-reserved routes remain fail-closed.
- The remaining global candidates belong to FEM (3), Forms (7), Remodel (3), Energy (1), Draw (1), and Puzzle (1). Remodel's three are staged-blob/reconstruction operation registries, not child identity caches.
