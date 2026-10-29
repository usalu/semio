# Animals content index

File: `S/🗂️catalog/🐕️animals/🔣️.json` (CategoryDocument, six assets in manifest order). The check CLI prints a check mark for all six.
Sources: handcrafted fragments in `T/🧰️scripts/animals/<id>/` (00-head, 10-rig, 20-anchors, 30-geometry, 40-poses, 50-clips, 60-thermal), concatenated by `category.sh`. Scratch tooling in the same folder: `kin.py` (forward kinematics and limb inverse kinematics), `*-emit.py` (computes joint angles of gait key poses from footfall tables, texts authored by hand), `preview.ts`, `probe.ts`.

| asset | nominal H x W (m), mass | named poses | clips | notes |
|---|---|---|---|---|
| labrador-dog | 0.73 x 1.48, 30 kg | standing-alert, sitting-waiting, sniffing-head-down (+16 key poses) | walk-cycle (4-beat, duty 0.65), trot-cycle (diagonal pairs, duty 0.55, panting), sniff-ground, sit-and-pant | below |
| domestic-cat | 0.36 x 0.80, 4.2 kg | see content-animals-domestic-cat.md | walk-cycle, stalk-cycle, stretch | |
| horse | 2.37 x 2.90, 550 kg | see content-animals-horse.md | walk-cycle, graze-and-swish | Buchner et al. 1997 as guide |
| pigeon | 0.24 x 0.32, 0.35 kg | see content-animals-pigeon.md | walk-cycle, peck-cycle, take-off (flight phase ballistic) | |
| roe-deer | 1.08 x 1.46, 25 kg | see content-animals-roe-deer.md | walk-cycle, bound-cycle, alarm-freeze | |
| dairy-cow | 1.47 x 2.57, 650 kg | see content-animals-dairy-cow.md | walk-cycle, cud-chewing, lie-down, graze-and-chew | |

Per-animal notes: `content-animals-<id>.md` in this folder (dog notes below).

## labrador-dog
Rig: 29 bones (pelvis root, lumbar, thorax, neck, head, jaw, two ears, three tail bones, near and far hind chains thigh-shank-metatarsus-paw, near and far fore chains scapula-humerus-forearm-metacarpus-paw). Far side tone 0.55 and lower z. Masses sum to 30 kg (authored estimates guided by Amit et al. 2009). Joint limits authored inside the goniometric ranges of Jaegger et al. 2002. Gait footfalls follow Muybridge 1887 and Hildebrand 1976; stance excursions and angles from limb inverse kinematics (scratch scripts). Thermal: coat regions by Meeh area 1.08 m2 with authored core-resistance insulation (Scholander 1950, Cena and Monteith 1975), nose and pads as skin, moist tongue exposed only with open jaw, nasal breath 3 s, sniff and pant emitters (0.2 s, 0.4 s). Met values authored (Kleiber, Taylor et al. 1970).
Simplifications: planar elevation model; trunk shapes are three overlapping body shapes so seams show at 1:50; walk stance slightly lowered against the standing hip height to fit hind-limb excursion; sitting pose rests on ischium and four paws with all contacts level within 1 mm.

## Engine rules applied
`structure.supports` is `[]` everywhere; contacts come only from pose contacts and clip spans. Gait phase poses carry `key: true`. The pigeon take-off airborne phase carries `ballistic: true`.
