# Morph and Consumer Inventory (haiku-inventory, transcribed by coordinator)

Read-only result. Line numbers come from ripgrep at audit time (2026-11-11). Migration workers re-grep before editing.

## Toolkit transitions (`🔀️morph/🐍️.py`): 170 calls in 9 files

| Series / scene | File | Calls | Transition lines |
| --- | --- | --- | --- |
| GH `Beat1_Anatomy` | `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py` | 14 | 180, 191, 196, 213, 424, 480, 506, 533, 560, 612, 647, 702, 746, 762 (wrappers `_hop` 180, `_cap_to` 191, `_to_cap` 196, `_fly_discs` 213, `_repair` 424) |
| GH `Beat2_Capsule` | `🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py` | 29 | 814, 848, 849, 868, 881, 914, 940, 982, 986–988, 1026–1027, 1056, 1058, 1073, 1092, 1126, 1159–1161, 1181, 1184, 1202, 1209–1210, 1220–1221, 1237 |
| GH `Beat3_Tower` | `🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py` | 24 | 238, 282, 288–289, 307, 318, 330, 340, 346, 355, 360, 366, 376, 388, 406, 416, 433, 452, 459, 467, 479, 488, 498, 505 |
| GH `Beat4_Casting` | `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` | 23 | 669, 689, 690, 707, 708, 713, 724, 733, 739, 748, 757, 764, 765, 780, 795, 809, 841, 842, 848, 860, 870, 876 |
| GH `Beat5_DataTrees` | `🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py` | 0 toolkit | ad hoc `_Trip`/`_flight`/`_hatch` (FadeOut/FadeIn crossfade)/`_gather`/`_regroup`, `Transform` 309 |
| Excel `Beat1_Anatomy` | `📊️excel/🎬️clip/🦴️anatomy/🐍️.py` | 11 | 362, 368, 417, 466, 473, 495, 505–506, 520, 528, 553 (wrappers `_tok` 281, `_fly` 257 ReplacementTransform, `_legend` 573) |
| Excel `Beat2_References` | `📊️excel/🎬️clip/📍️references/🐍️.py` | 33 | 191 (`_bar_retext`), 353, 372–373, 377, 402, 419, 433, 447, 472, 495, 518, 537, 584, 591, 595, 632, 643, 645, 665, 694, 705, 708–709, 718, 721, 726, 729, 785, 796, 800, 828, 857 |
| Excel `Beat3_Program` | `📊️excel/🎬️clip/📋️program/🐍️.py` | 15 | 210 (`swarm`, uses `_morph.Swarm`), 218 (`collect`, `_morph.Morph`), 223 (`fly`), 476, 495, 506, 543–544, 556, 595, 629, 681, 719–720, 772 |
| Excel `Beat4_Energy` | `📊️excel/🎬️clip/⚡️energy/🐍️.py` | 18 | 474–475, 483, 496, 516, 526, 536, 551, 556, 567, 577, 589, 607, 617, 637, 655, 679, 876 |
| test only | `🦗️grasshopper/🎬️clip/🧪️tests/🗼️live-snapshot` | 3 | gather/split/retext |

Helper calls (`token`, `silhouette`, `proxy`, `capsule_pairs`, `tether`): about 108.

## Direct Manim transforms in clip code

| Series | Hits | Notes |
| --- | --- | --- |
| energy (31 files) | 261 | no toolkit; shared wrappers in `⚡️energy/🏗️authoring/🎨️visuals/🐍️.py` (`open_window` 892–900, `house_section` 925, arc helper 751); physics `_fill_panel` 111–132 (FadeTransform 126), `_ripples` 389–408; heating `_dim` 328–339; ventilation `window_opening` (`🪟️natural` 129, `❓️need` 132/197) |
| optimization | 19 | pareto 772 clip-local `self._morph` |
| grasshopper | 7 Manim (+4 Rhino) | |
| excel | 7 | |
| BIM | 2 | `information` 639 |

## Consumers

| Consumer | Path | Quality knowledge |
| --- | --- | --- |
| Render CLI | `⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py` | `-q l|m|h` default `l`, `--resolution`, `--fps` |
| Runtime | `🎬️clip/⚙️runtime/🐍️.py` | `QUALITY_FOLDERS` :30, rate map :817, `render_segment` :501, `compose_videos` :705; preview output can overwrite final slot without separate root |
| Presentation | `🎬️clip/🪟️presentation/🐍️.py` | duplicate quality map :72, reads `.render.json` :277, `-crf 0` |
| Interaction scene | `🪟️presentation/🖱️interaction/🎬️scene/🐍️.py` | cursor `.become` :128 |
| Camera test | `🪟️presentation/🌐️browser/🧪️tests/🎥️camera/🐍️.py:29` | hard-codes `-ql`, 15 fps |
| Course site | `📚️course` | no video/poster references found |
| Registry | `⚡️energy/🎬️clip/🔣️.json` | 26 videos; morph-bearing: GH anatomy/capsule/tower/casting/trees, Excel anatomy/references/program/energy |

## Findings

1. No preview/final morph concept; only `proxy`/`HEAVY=200` as lower detail.
2. Energy clips use 261 direct transforms; the declarative layer must cover replacement/copy/fade transforms too.
3. Preview and final share segment slots unless output roots differ.
4. Clause trimming cuts a full source render by trace marks (trim evaluates the original timeline).
5. Two drifting quality tables (runtime :30, presentation :72).
6. "Preview" today means `-q l`; "preview eye" in GH clips is UI vocabulary, unrelated.
7. Source fingerprints follow literal `import_module` calls, so morph edits invalidate segments.
8. `_live` updaters and `tether`'s `always_redraw` force per-frame redraws.
