# Wave W4b Norm Done

Refined all 15 norm artifacts from `Document` field lists + example DSL key=value lines (family-sheet, `layout = "lines"`).

## Artifacts
- `📕️din4108` (din4108): 20 fields [category, layers, climate, airtightness-n50, psi-times-l-sum, rh-int, catalog-id, material-id, airtightness-class, t-int-c…]; ops=[]
- `📙️din18599` (din18599): 16 fields [started, actor, use-class, heated-area-m2, occupants, h-t, h-v, internal-gains-w-m2, solar-gains-kwh, system-losses-kwh…]; ops=[]
- `📗️din16798` (din16798): 64 fields [annex, occupancy, comfort-category, t-op-c, rh-percent, air-speed-m-s, theta-rm-c, co2-ppm, df-percent, l-aeq-db…]; ops=[]
- `📘️en1995` (en1995): 22 fields [annex, m-ed-knm, n-ed-kn, v-ed-kn, w-mm3, a-mm2, b-mm, h-mm, f-m-k, f-c-0-k…]; ops=[]
- `📘️en1992` (en1992): 37 fields [annex, m-ed-knm, v-ed-kn, f-ck, b-mm, d-mm, a-s-mm2, f-yk, rho-l, n-ed-kn…]; ops=[]
- `📘️en1993` (en1993): 76 fields [annex, n-ed-kn, m-ed-knm, v-ed-kn, a-mm2, a-v-mm2, w-pl-mm3, f-y-mpa, f-u-mpa, chi…]; ops=[]
- `📘️en1994` (en1994): 24 fields [annex, m-ed-knm, v-ed-kn, m-pla, m-pl-rd, eta, v-l-rd, insulation-thickness-mm, fire-rating, deck-type…]; ops=[]
- `📔️vdi3805` (vdi3805): 57 fields [manufacturer-file, catalog, edition-profile, correction-as-of, strict-mode, index, geometry, curves, limits, started…]; ops=[]
- `📓️iso16757` (iso16757): 71 fields [catalogue, dictionary, geometry, selection, part-number-rule, part-number-inputs, script-limits, exchange-process, started, actor…]; ops=[]
- `📘️en1991` (en1991): 34 fields [area-m2, category, annex, self-weight-material, self-weight-thickness-m, assumed-g-k-kn-m2, fire-curve, fire-resistance-min, fire-member-capacity-c, snow-zone…]; ops=[]
- `📘️en1996` (en1996): 24 fields [m-ed-knm, n-ed-kn, v-ed-kn, h-ed-kn, z-mm3, area-mm2, shear-area-mm2, f-k-mpa, f-vk-mpa, annex…]; ops=[]
- `📘️en1998` (en1998): 51 fields [seismic-zone, ground-type, importance-class, structural-system, t1-s, mass-t, v-rd-kn, drift-mm, height-m, multiple-resisting-systems…]; ops=[]
- `📘️en1999` (en1999): 28 fields [n-ed-kn, m-ed-knm, a-mm2, w-el-mm3, alloy, chi, i-t-mm4, l-cr-mm, theta-c, delta-sigma-ed…]; ops=[]
- `📘️en1997` (en1997): 24 fields [v-ed-kn, h-ed-kn, footing-area-m2, phi-deg, c-kpa, gamma-kn-m3, b-m, d-f-m, e-s-mpa, nu…]; ops=[]
- `📘️en1990` (en1990): 8 fields [g-k, q-k, resistance-kn, consequence-class, annex, seismic-a-ed-kn, started, actor]; ops=[]

## Files
75 facet specs under `✏️s/🔌️plugins/📕️norm/🗿️artifacts`.

## Sample shape
```
document = header assign*
header = "semio" IDENT "v" INT
assign = ("annex" | "m-ed-knm" | …) "=" value
```
