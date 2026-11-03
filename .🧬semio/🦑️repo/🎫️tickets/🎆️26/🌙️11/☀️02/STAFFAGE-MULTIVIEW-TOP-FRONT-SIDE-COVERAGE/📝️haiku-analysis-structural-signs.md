# 📝️ haiku-analysis-audit: Structural Sign Conventions in the Left-Handed Frame (relayed by the coordinator)

This is a static review. No tests were run.

## Verified by reading the code

- The `CAMERAS` table matches D6.
- Dot and cross symbols for components toward and away from the viewer are correct.
- Curl senses are correct per view:
  - elevation: positive mz is counter-clockwise, seen from −z;
  - front: positive mx is clockwise (↻);
  - plan: positive my is clockwise (↻).
- M = r × F is used consistently for joint, cut, reaction and equilibrium moments.
- The sagging sign is correct, and the residual covers all six components.

## Defects

| ID | Severity | Where | Issue |
|---|---|---|---|
| D1 | Med | `🏗️structure:8`, `📽️projection:43`, `🔗️contracts:57` | The sign is stated only as a picture rule. The thumb rule is inverted in this left-handed frame, but nothing warns about it. |
| D2 | Med | `🏗️structural-view:531-538` | The in-plane moment vector is drawn with two heads pointing the same way, but the docs call it "double-headed". |
| D3 | Low-Med | `🏗️structural-view:223,182` | `${id}:cross` is used twice, so two paths share one id. |
| D4 | Med | `🌉️simply-supported/🐍️.py:46-49`, `🟦️.ts:30-35` | The fixture mixes sign conventions: values are stored with the anastruct sign, and the test negates them ad hoc. |
| D5 | Med | `⚖️equilibrium`, `🧰️harness:91` | No 3D oracle covers fz, mx, my or a non-zero z. |
| D6 | Low | `🔗️contracts:57,201,204` | The signs of mxNm and myNm are not stated, nor the reference point of each moment (anchor, joint or cut). |
| D7 | Med | `📽️projection:8`, `🏗️structure:14` | Both cite `T/🏗️structural-projection.md`, which does not exist. |
| D8 | Low | `🏗️structural-view:594` | The legend does not say that positive mx and my are clockwise in front and plan. |
| D9 | Low | `🏗️structural-view:371-372` | On a moment diagram the dot or cross follows the ordinate direction, not the moment vector, and this is undocumented. |

## Coordinator rule D23

- A moment vector keeps the engineering convention: an arrow with two heads pointing the same way.
- Every legend and doc must state the following, in en and de:
  - the components are those of M = r × F in the (x, y, z) frame;
  - the frame is left-handed, so the arrow's sense follows the LEFT-hand rule;
  - the curl sense in each view: positive mz is counter-clockwise in the elevation; positive mx (front) and my (plan) are clockwise.
