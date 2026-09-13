# Wave Z — browser checklist verification (2026-09-10)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Serve: `http://127.0.0.1:6014` (wasm #31, do not kill).
Harness: `🔍️browser-probe.ts` (additive flags). Fill is owned by another wave — not probed here.

| Item                                       | Verdict | Artifacts                                                   | Notes                                       |
| ------------------------------------------ | ------- | ----------------------------------------------------------- | ------------------------------------------- |
| 1. Example-switch undo is ONE history step | PENDING |                                                             | `--interact --undo --settle=30 --port=6014` |
| 2. Clipboard copy/cut/paste                | PENDING |                                                             |                                             |
| 3. Marquee rectangle + click pick          | PENDING |                                                             |                                             |
| 4. Import/export round trip                | PENDING |                                                             |                                             |
| 5. Locked-volume gumball refusal           | PENDING |                                                             |                                             |
| 6. Brush tool strokes                      | PENDING |                                                             |                                             |
| 7. Gumball drag → one history entry        | PENDING |                                                             |                                             |
| 8. Suggestions window opens/populates      | PENDING |                                                             |                                             |
| 9. Perspective tab red + warning triangle  | PENDING | `🗑️generated/probe-2026-09-10T05-19-26-fill-wait-ready.png` |                                             |

## Probe runs
