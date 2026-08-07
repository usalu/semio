# W5 polish status (2026-08-07)

Follow-up after W5 fan-out (`files_written=240`, `examples_padded=160`).

## 1. Generic grammar / protocol scan

| Check | Result |
| --- | --- |
| `prop = IDENT` in `*.grammar.semio` under plugins | **0 hits** |
| `mesh-json` in plugin grammars | **0 hits** (only lowpoly negative tests mention it) |
| Catch-all `list \| map \| value` prop rules | **none** in plugin grammars |

Norm `family-sheet` fan-out grammars still use typed `assign = <keyword> "=" value` (domain keywords, not `prop = IDENT`). No rewrites required this pass.

## 2. Pack framing magics

- Scanned all `framing magic` lines in `*.protocol.semio`: **all unique** (fnv1a-derived `0x8953…` corpus + pilot ASCII magics `LWPL`, `N1992`, `DAG`, `CAD`).
- No shared `SPK` / `0x8953504B` literals remain.

## 3. Hot plugins distinctness

| Plugin | artifact-mark / keywords | Action |
| --- | --- | --- |
| **flow** | `flow.flow`, graph `add-node` / `rewire` | already distinct — no edit |
| **procedural** | `procedural.procedural2d-op` / `procedural3d-op` | added `loft-profile` (2d) and `revolve-profile` (3d) op + diff rules |
| **block** | `block.2d-op` | added `extrude-loop` op + diff |
| **puzzle** | `puzzle.2d-op` | added `snap-part` op + diff |
| **vcs** | `vcs.vcs` / `vcs.vcs-op` eng verbs | already distinct — no edit |

Touched files (8):

- `🧱️block/.../◻2d/🔧️op/📖️component.grammar.semio`
- `🧱️block/.../◻2d/🔺️diff/📖️component.grammar.semio`
- `🧩️puzzle/.../◻2d/🔧️op/📖️component.grammar.semio`
- `🧩️puzzle/.../◻2d/🔺️diff/📖️component.grammar.semio`
- `🌀️procedural/.../🌀️procedural2d/{🔧️op,🔺️diff}/📖️component.grammar.semio`
- `🌀️procedural/.../🧊️procedural3d/{🔧️op,🔺️diff}/📖️component.grammar.semio`

## 4. Empty / tiny pack & spr examples

Ticket script `🔧️pad-p4-pilot-binary-examples.mjs`:

- **written:** 17 binaries
- **pilots:** 11 (en1992×4, dag×2, cad×4, lowpoly spr refresh)
- **corpus:** demonstrator playground, reasoning wires, energy model (≤64 B → SEM + 128 B pad)

Post-run: **0** plugin pack/spr examples ≤ 64 B.

Pilot binaries now include domain magic + stub payload after the SEM envelope (same pattern as lowpoly LWPL stub).

## Out of scope (per ticket)

- No edits to `📜️script.ts` or engine `grammar.rs`.
- No git commit.
