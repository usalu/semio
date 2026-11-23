# vehicles/city-bicycle

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts vehicles/city-bicycle --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ city-bicycle (1/1)`.

- Findings before: axle 10 mm and nuts 8 mm at 1:100 and 1:200, rim 25 mm and structure bearing sleeve 22 mm at 1:200. The full proof then showed that the asset had never passed `volumes-connected`: 1:50 kickstand group 2.5 mm under the chainstays, saddle 4.3 mm beside its clamp, front wheel (hub shell without bearing) 2.2 mm from the fork; 1:100 both mudguards and both pedals loose; 1:200 front mudguard and both stand legs loose; 1:500 far crank with pedal and one stand leg loose; the stand structure without load path at every scale.
- Remodelled (`K/rework-city-bicycle-coarse-wheels.py`, then `K/rework-city-bicycle-joins.py`):
  - 1:50: kickstand mounting plate under both chainstays, cross bolt of the saddle clamp through both rails, two bearing cones with locknuts carrying the front hub between the fork ends; bracket as frame member in the structure.
  - 1:100: axle and nuts one 15 mm stud nut face to nut face; 15 mm seatstay bridge with tab for the rear mudguard, boss under the fork crown for the front one, pedal spindles as body parts (were structure only), bracket as frame member.
  - 1:200: hub, brake, axle and nuts one turned body with a 30 mm stud to the dropouts; rim a closed 30 mm band; nipples left out; bearing sleeve 30 mm; 30 mm mudguard boss; 30 mm kickstand bracket block.
  - 1:500 (3D only): hub one 90 mm drum from dropout to dropout, rim band as wide as the tyre, no nipples; crank axle as body part (was thermal and structure only); 75 mm kickstand bracket block.
- Sheets opened: 1:200 body, 1:500 body, 1:100 body three-quarter. Rendered but not opened: 1:50 body, 1:100 body, 1:200 structure, 1:200 and 1:500 thermal. 2D not rendered (not changed).
- Imperfect: 1:200 and 1:500 keep the 36 true spokes per wheel (slender rods; the 2D of 1:200 draws six); frame tubes of 1:500 keep their real 28 to 37 mm; the rear hub of 1:50 still has no cones (it touches the stays through its shell); the new small parts are not in the 2D drawings (all under 0.2 mm on paper at their scale except the 1:50 stand plate, hidden behind the chain case in the elevation).
