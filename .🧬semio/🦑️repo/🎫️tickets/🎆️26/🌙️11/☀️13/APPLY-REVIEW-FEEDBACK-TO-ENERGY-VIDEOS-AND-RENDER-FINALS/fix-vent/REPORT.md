# fix-vent: visual polish pass of the German ventilation final

Base: 31:09 speaker final audited by check-a. New final: `.🧬semio/🎓️teaching/⚡️cache/ve/final/de/speaker/rendered/Full_Ventilation_speaker_de_1080p30.mp4` (31:09.80). Frames: `🗑️generated/fix-vent/old` (old final every 4 s), `new` (previews), `fin` (new final every 6 s, 4x4 sheets). All paths below are under `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/`.

## Leads
| Lead | Verdict | Change |
|---|---|---|
| h = 4,4 m under Winter box (Beat4_Auftrieb) | real | `🪟️natural/🐍️.py` h tokens rotated and placed left of the dimension arrow (x -0.20). Seen clean in new final (sheet f_12). |
| V̇ 0,069 vs 0,072, 219 vs 220 | real | v is shown with 2 decimals (1,15 m/s, `🪟️natural/🐍️.py` readout and panel plus narration en/de), so 0,6 · 0,1 · 1,15 = 0,069 m³/s = 248,7 -> 250 m³/h; tall 1,53 m/s -> 330, winter 2,52 -> 540. sqrt panel now `sqrt(2·0,8 Pa / 1,2 kg/m³)` in one root. `🧪️architecture/🐍️numbers.py` checks quoted 1,15. 219/220: not reproduced, the 219 was a counting-up readout. |
| 340 Wh hidden / 827 vs 830 | real (total) / transient (hidden label) | Reveal energy rounded once (`e_lb = round(..., -1)` = 190), so 340 + 300 + 190 = 830 exactly; bar, label and total agree. Hidden 340 label was the 0,8 s ratio morph. |
| humidity 14,4/10,1 vs 15,0/10,5 | transient | counting-up ramp (2,4 s) of the readouts; held values are 15,0 and 10,5 (running example summer). No change. |
| night ventilation chips garbled | real (morph) | `🪟️natural/🔀️transitions/🔣️.json`: mass-cooled, schedule-winter, season-winter, mass-warm, legend-winter use recipe `fade` (cross-fade) instead of `replace` (glyph move). Clean in new final (f_13). `🧪️architecture/🐍️transitions.py` accepts `fade`. |
| stale 386 W in hand-over | transient | count-up of Q̇_V,sens (0 -> 1500 W) mid-bar. No change. New final shows 1500 W. |
| empty stage ~5:20 | not reproduced as a held frame | old final shows content at 5:19-5:22; the nearby real bug is next. |
| (new) air-change scale pile-up | real | `📦️volume/🐍️.py` Beat2_Luftwechselrate show_use: stage label and readout were morphed as an unattached VGroup, so every earlier use left its label and n readout stacked on screen. They are now grouped in the scene before the morph. Clean in new final (f_04). Test driver got a `remove` stub. |
| mechanical supply low / extract high | made explicit | `⚙️mechanical/🐍️.py` zone tags "Zuluft unten" / "Abluft oben" appear with each system (en "supply low" / "extract high"). Visible in f_15. |
| latent chip vs bars (handover/mech Beat2) | partly fixed | chip moved from y -1.28 to -1.46; it now touches the top of the formula box (see Remaining). |
| "Heizlast: nur fühlbar" note touching 770 W ring (handoff) | fixed | `🤝️handoff/🐍️.py` narrower note, shifted right. |

## Tests (with SEMIO_TEACHING_PROFILE=preview and SEMIO_TEACHING_OUTPUT_ROOT set)
architecture 8 OK, numbers 4 OK, transitions 7 OK, volume 2 OK.

## Remaining
- Mechanical Beat2 (`⚙️mechanical/🐍️.py`, `lat_note`): the Q̇_V,lat winter/summer chip sits between the bar value row and the formula and touches the formula box; needs a layout slot of its own (for example moved left of the bars).
- Beat4_Auftrieb: the sqrt panel slightly overlaps the room drawing at its top edge.
- Old finals begin and end with screen recordings of the app (intro/outro), not in scope here.
- Only about every sixth sheet of the new final was viewed (f_04, 10, 12, 13, 15, 19); the other 14 sheets are not reviewed. Shared follow-up: none needed.
