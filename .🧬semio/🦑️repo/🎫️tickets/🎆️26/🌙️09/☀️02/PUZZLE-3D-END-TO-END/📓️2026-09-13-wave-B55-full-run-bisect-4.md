# Wave B55 — full-run bisect #4 (battery #61 reds)

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Baseline: `🗑️generated/battery-2026-09-13-61-6013.txt`
(FAULTS=0, PASS=71, FAIL=25). Serve: `:6013` (wasm #61, host vite-live).

Assignment: 8 red verdicts + make the `mutate` group run on a document the verdicts can reason about.

## 0. Baseline position map (read off battery #61, no new run needed)

`plan` (battery #61, line 5) puts the `mutate` group in this order:

```
context-menu-rows, pick-object, context-menu, tool-category, fill-tab, fill-abort-engagement,
fill-wait-ready, fill-apply-max, fill-history, selection-surfaces, clipboard-copy-paste,
marquee-drag, marquee-click, locked-refusal, gumball-drag, frame-perspective, brush-stroke,
suggestions-open, volume-brush, relocate, engagement-bar, outliner-rows, catalogue-panel,
selection-keybindings
```

Census timeline from the same file (`count=` samples, in step order):

| after step | instances |
| --- | --- |
| reboot for `mutate` | 1 (`treeItems=24`, `instanceCount=1`) |
| `fill-abort-engagement` | 0 |
| **`fill-apply-max`** | **161** |
| `clipboard-copy-paste` … `catalogue-panel` | 161 |

**The polluter is `fill-apply-max`, not `brush-stroke`.** `brush-stroke` sits at position 17 —
*after* `clipboard-copy-paste` (11) and `locked-refusal` (14), both of which already saw 161
objects. The 160 `puzzle3d.brush.*` ids are what the **Fill** tool registers when the count slider
is driven to its maximum with `End` (`fill-apply-max`, probe line 720). So ordering `brush-stroke`
last would change nothing; the bound has to go on the fill slider. → `--paint-limit` (§9).

`context-menu-rows` is position 1 and runs on `instanceCount=1` immediately after the group reboot,
i.e. it is *already* a fresh lane inside the battery. Its two reds are therefore genuine defects,
not pollution.
