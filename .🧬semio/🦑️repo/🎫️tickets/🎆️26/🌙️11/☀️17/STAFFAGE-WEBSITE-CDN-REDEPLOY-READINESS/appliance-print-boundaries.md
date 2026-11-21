# Appliance print boundaries

The native print suitability checker evaluated the authored 1:200 drawings at every integer denominator from 150 through 350, including every named pose and planar view at each denominator. Gas meter and laptop passed all 201 denominators on the shared geometry received for this check.

The MVHR spigots passed at 350 but failed at 334, 337, 338, 346 and 349. In the front view, the two rows stood 140 mm apart on centre and their 58.5 mm widths left a nominal white clearance of only 0.133 mm at 1:350 after the 0.1 mm pen. Narrowing their transverse footprints to 49 mm in the front and plan views left 0.16 mm of white clearance at the coarsest denominator. The longitudinal profile, original centres, four drawn spigots and 3D geometry remain in place.

The updated MVHR drawing passed all 201 denominators. Asset validation reported zero issues for all three appliances. The ticket's `check-appliance-print.ts` reproduces the measurement.
