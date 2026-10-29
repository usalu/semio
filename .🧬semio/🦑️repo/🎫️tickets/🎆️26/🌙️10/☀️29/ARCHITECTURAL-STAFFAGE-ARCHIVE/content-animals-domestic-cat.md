# domestic-cat notes

- Dimensions: heightM 0.357 (evaluated standing-alert incl. ears), widthM 0.8 (tail trailing), depthM 0.13, massKg 4.2 (bone masses sum to 4.2). Withers about 0.25 m, nose to rump about 0.5 m, tail 0.28 m.
- Rig: 32 bones; pelvis, lumbar, thorax-a, thorax-b spine; neck, head, jaw, near/far ears; 5-bone tail; near and far hind (thigh, shank, metatarsus, paw) and fore (scapula, humerus, forearm, metacarpus, paw) chains.
- Poses (named): standing-alert, sitting-tail-wrapped, stretch-stance, play-bow, stretch-deep. Key poses (key true): walk-phase-0..7, stalk-phase-0..7.
- Clips: walk-cycle (0.8 s, advanceM 0.352, duty 0.625, lateral sequence), stalk-cycle (2.4 s, advanceM 0.1467, duty 0.75), stretch (8 s, non-looping, all four paws planted throughout, yawn emitter and tongue track).
- Thermal: Meeh A = 0.1*M^(2/3) = 0.26 m2 (k = 10 cm2/g^(2/3); the briefed 0.21 m2 does not follow from k = 10), baseMet 0.8 from Kleiber 3.4 W/kg^0.75 x 1.2; breath-nose period 2.5 s; yawn-mouth emitter.
- Sources: Muybridge 1887, Hildebrand 1976, Halbertsma 1983, Goslow 1973, Hoy and Zernicke 1985, Leyhausen 1979, Kleiber 1947, Spector 1956, Scholander 1950, Cena and Monteith 1975, Gebremedhin and Wu 2001; segment masses, joint limits, insulation and activity met are authored estimates.
- Simplifications: stretch clip starts from a long stance (not the standing pose) so no paw slides; gait swing feet have up to 1.4 cm IK residual, stance feet up to 1 cm in three fore phases; sitting tail lies on the ground beside the haunch rather than truly encircling the paws; structure.supports is [] per coordinator rule.
- Scratch: scripts/animals/cat-emit.py, cat-kin.py, cat-partial.sh; fragments in scripts/animals/domestic-cat/.
