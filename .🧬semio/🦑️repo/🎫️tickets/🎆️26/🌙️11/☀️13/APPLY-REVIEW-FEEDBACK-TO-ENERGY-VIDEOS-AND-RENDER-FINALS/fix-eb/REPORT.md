# Energy balance final: layout fixes

All changes are in `📊️demand/🎬️clip/**/🐍️.py`; nothing in `🏗️authoring`.

## Defects and fixes
1. **Title against reference chip (10:40, 36:20, note at 31:20)** — real. `_din_ref(text, title)` flows the
   sources into as few lines as fit right of the beat title. Per-chapter copy in all seven chapter files
   (e.g. `⚖️regulations/🐍️.py`); every call passes the title.
2. **Red law box wider than the frame** — real. Heading "Gesetz: Gebäudemodernisierungsgesetz", value
   "GModG (Stand 2026-07-29) — vormals GEG"; inside the frame and clear of the Q_P bar (10:30–11:10).
3. **Overprinted `e_E` line (26:20)** — real, a held state. The old `e_E = e_ce · e_d · e_s · e_g` row stayed
   because its parts sit in the scene individually and the transition removed only the group. It also
   garbled `Q_E = Q_h · e_E` at 26:45–26:55. `_regroup(scene, group)` in `♨️system-losses/🐍️.py` fixes both.
4. **Duplicate certificate sheet (36:00–36:15)** — real, same root cause, same `_regroup` fix in
   `🏷️certificate/🐍️.py`. § 87 listing rows spaced further apart ("wesentlicher Energieträger" overlapped
   the `Q_E` fraction).
5. **153 vs 154** — transient (counter still counting). Settled value 154 (32:20). Narration numbers in
   `🏭️primary-energy/🐍️.py` now derive from `Q_PRIM_GAS` and `Q_PRIM_WP`.

## Sweep
Old and new German presenter finals viewed every 5 s: no other held overlaps or out-of-frame elements;
the rest are mid-morph frames (e.g. label cross-fade at 26:40).

## Follow-ups (shared code, not done)
- Move the width-aware reference chip into `🏗️authoring`; heating, cooling and ventilation chips are
  single-line and can collide with long titles.
- `stage.transition` should remove a source group's parts even when they were added individually (root
  cause of 3 and 4); `_regroup` could then go.
- The `e_P` note (1,54 and 0,56) and the practice chapter's narration still use hard-coded numbers.
