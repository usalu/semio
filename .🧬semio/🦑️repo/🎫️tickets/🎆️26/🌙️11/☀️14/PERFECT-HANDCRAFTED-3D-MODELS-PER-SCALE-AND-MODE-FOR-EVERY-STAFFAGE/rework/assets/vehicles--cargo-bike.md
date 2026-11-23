# vehicles/cargo-bike

## 2026-11-22 23:00 — fittings rework pass (coarse scales per feature, measured rules proven)

Full report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-fittings.md`. Final proof: `r3-validate.ts vehicles/cargo-bike --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ cargo-bike (1/1)`.

- Findings before: 1:100 axles, nuts, carrier bars; 1:200 these plus chainstays, seatstays, bridge, the whole carrier, steering drum, bearing sleeves; 1:500 rims, front hub, every frame tube, riser, stem, clamp, carrier parts, drum, bearing sleeves and wheel mass markers. Measured (never proven before): 1:50 both crates hovering 2 and 3 mm, sprocket member 2 mm beside the hub, fork ends 2 mm from the blades in the structure; 1:100 front mudguard, steerer cap and head lamp loose, stand hinge 7 mm from the beam box; 1:200 far crank and both stand legs loose.
- Remodelled (`K/rework-cargo-bike-coarse-scales.py`, then `K/rework-cargo-bike-joins.py`):
  - 1:50: crates lowered onto the box floor and onto each other; fork ends 32 mm eyes, structure blade tubes 10 mm longer into them; sprocket no wheel member of the structure.
  - 1:100: axle and nuts one 15 mm stud, carrier bars 15 mm; steerer cap to the head tube, lamp bracket from the steering drum, mudguard bracket from the lamp, stand hinge sleeve 30 mm.
  - 1:200: hubs one drum with 30 mm studs; chainstays, bridge, carrier struts and bars 30 mm; seatstays, carrier rails and legs true rods (`slender`, 25:1 and more); steering drum 30 mm; no tail plate; 30 mm crank axle through the motor; bracket block under the beam for the stand legs.
  - 1:500: hubs plain drums dropout to dropout (front 75 mm), rims true hoops (`slender`), beam, head tubes, down tube, seat tube, chainstays, riser, stem, bar clamp, steering drum 75 mm masses (as the seat post already was); no bridge, struts, bars, tail plate; structure sleeves and wheel mass markers 75 mm.
- Sheets opened: 1:500 body before (with labels) and after, 2D 1:500 light-solid before. Rendered but not opened: 1:200 body, 1:500 structure, 1:200 thermal, 1:100 three-quarter, 2D 1:500 after.
- Imperfect: at 1:500 the 75 mm tubes are twice their real width (the rule's price for a drawn 1:500; the 2D strokes were not changed and the silhouette rule passes); 36 true spokes and nipples per wheel remain at every scale; the two 75 mm chainstays overlap into one slab; the mudguard bracket assumes the mudguard radius about the fork tip and was proven by the contact rule, not looked at in a close-up; the crate drop of 2 and 5 mm was not carried into the 2D.
