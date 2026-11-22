# School-age limb geometry

The runner and kick-scooter rider have distinct anatomical reference meshes. The running child's femur and tibia span 256.5 and 262.0 mm. The kick-scooter rider's corresponding lengths are 302.0 and 293.4 mm. The validation uses each child's own 1:50 wrist, forearm, shoulder, hip and knee geometry.

The independent Three.js fixture cuts the posed mesh on planes perpendicular to the bone at 34%, 52% and 70% of its length. It measures both diameters and the centre of each cut on the left and right femur, humerus, radius, ulna, tibia and fibula. The checks include two poses per child, all four scales and the four joint markers on each side. Joint sizes and separate radius–ulna and tibia–fibula passages are preserved in the coarser representations.

| Subject | Maximum source shaft diameter change across scales | Maximum source centre displacement | Fitted shaft diameter change | Fitted centre displacement |
|---|---:|---:|---:|---:|
| Running child | 18.811 mm | 15.946 mm | 0.131 mm | 0.065 mm |
| Child on kick scooter | 57.698 mm | 26.638 mm | 0.293 mm | 0.068 mm |

The original running-child catalog SHA-256 was `d162ce7cb712547ec96275caaf8fc84c0e784f332f92ed88b8aa085303416b00`. The scooter rider was `583406bc3da13ccb1131b1647f80e6e838796a647d0d3857c19995b7e268fc3d`. The geometry, language-independent case and independent oracle are `digital-schoolchild-bones.ts`, `digital-schoolchild-case.json` and `digital-schoolchild-check.ts`.

## Installed four-scale studies

The running child is catalog SHA-256 `a3d53630b01949b796272797e9b920ec83e71643b78a905c04db41b6d0924269`. The 12 GLB scenes total 3,713,908 bytes. The kick-scooter rider is `0f25a5afc7da01a94c60027e6c235529e4b1d016bde4fe615626a48fa0f2a095`. Its 12 scenes total 4,066,960 bytes. Both passed the native 52-rule suite, office-print checks and independent section cuts on the installed files.

The paired shaded presentations show the runner at toe-off and airborne, and the scooter rider gliding and pushing from the opposite side. The two leg shafts remain distinct at 1:500 with the backpack and scooter present. The magenta structural moment glyphs near the hips and knees cover parts of the bones in projection. The independent cross-sections use posed shaft surfaces directly.
