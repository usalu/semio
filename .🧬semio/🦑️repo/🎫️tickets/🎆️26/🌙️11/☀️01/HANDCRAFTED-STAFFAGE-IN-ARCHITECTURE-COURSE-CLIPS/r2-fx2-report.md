# R2 test fixes (agent fx2)

## 1. React vitest catalog-loading failure (fixed)

Root cause: a hand-edit slip in the manifest, not a generator. The manifest `🧍️staffage/🗂️catalog/🔣️.json` is handcrafted (no generator exists). The consistency rule is in `🔨️modules/🗂️catalog/🟦️.ts:510-511`: `thermal` in `analytics` iff the asset has thermal regions, `transport` iff the asset has emitters. `plate-heat-exchanger` has regions and emitters but its entry listed only `["thermal", "structural"]`.

Fix: added `"transport"` (between thermal and structural, like the other 101 entries) to the `plate-heat-exchanger` entry.

Sweep (script `🧰️fx2/analytics.py`, compares every manifest entry with its asset): 147 entries, 147 asset folders. Before: 1 mismatch (plate-heat-exchanger). After: 0 mismatches; 102 entries are thermal+transport+structural, 45 thermal+structural. gas-meter, electricity-meter and ceiling-diffuser exist as assets and manifest entries and agree (re-checked at the end of this run; re-check once g4 finishes).

React vitest (ticket-local alias config): `Tests 107 passed | 1 skipped (108)`, rc 0.

## 2. Python module 31, heading (fixed)

Root cause: the heading test (`🔤️fonts/🧪️tests/📑️heading/🐍️.py`) AST-scans all canonical clip scripts for any attribute named `_title` or a removed API (`scene_title`, `play_scene_title`, `beat_subtitle`). Both clips already use the current API `fonts.section_heading`; only a private helper was still called `_title`. Renamed (a rename, no behaviour change):
- `📊️excel/🎬️clip/⚡️energy/🐍️.py`: `_title` to `_heading` (def line 410, call line 440). 2 sites.
- `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py`: `_title(self, view)` to `_view_title` (def line 252, 14 calls at 643, 648, 659, 683, 700, 707, 723, 732, 741, 763, 776, 784, 804, 820). 15 sites. `_title_text` untouched.

Tests: heading module 3 of 3 OK; excel clip tests 85 OK; grasshopper clip tests 66 OK.

## Render check
Both single-scene renders (Beat4_Energy, Beat4_Casting, quality l) did not finish: the host was heavily loaded by concurrent agents (over 50 min, still on partial movie files), so I stopped them and deleted their output. Verified instead by the heading test (imports and executes both scripts and the actual header scene), the excel clip tests (85, OK) and the grasshopper clip tests (66, OK). Open: a visual render of one scene each when the host is idle.
