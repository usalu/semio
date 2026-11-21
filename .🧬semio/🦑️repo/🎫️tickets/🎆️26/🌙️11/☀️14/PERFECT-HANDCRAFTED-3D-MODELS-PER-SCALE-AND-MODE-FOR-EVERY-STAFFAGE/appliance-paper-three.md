# Diffuser, fan-heater and MVHR paper ranges

All three office-laser drawing bands were checked at every integer denominator from 1:20 through 1:350. At 1:75 and 1:150 both adjoining authored representations were tested. The four digital volume scales in each asset exactly match the pre-edit geometry fingerprint and ticket authoring document.

| Asset | Resolved paper drawing | Drawing checks | Full product findings | Digital volume SHA-256 |
|---|---|---:|---:|---|
| ceiling-diffuser | The 7.5 mm backplate edge in both side views uses a 0.20 mm pen. | 333 | 0 | `f163f3c1443a31aa4d7efbe0db1c070f9ac88eea5f4afcef0764228ae5f75cee` |
| electric-fan-heater | The fine side view draws the lower return lip at the measured cable entry. The 6 mm mounting bosses use a 0.20 mm pen, resolving the isolated 1:56 failure. The intermediate side and plan views draw the 2.5 mm edge of the outlet grille with a 0.20 mm pen. | 333 | 0 | `232b320b52178fdab6336b0a92ad09e581b768ae60f007123792544cdfeeac8e` |
| mvhr-unit | The 1:100 side view marks each EPDM sealing bead by the 100 mm duct connection it seats on. Four separate sleeve lips retain their original centerlines and 150 mm center spacing. The two projected front-view spigots use a 0.13 mm pen to keep the 42 mm gap readable. | 333 | 0 | `492ccc8afdb61372c64f2b010d40d2dc5dd87ac75709f1b56694b76796e8184b` |

The product validator also checks asset schema, all available 3D representations, thermal coverage and structural geometry. Run `rotor-paper.ts` and `r3-validate.ts --file <asset> --volumes --json` against the live catalog documents to reproduce the checks.
