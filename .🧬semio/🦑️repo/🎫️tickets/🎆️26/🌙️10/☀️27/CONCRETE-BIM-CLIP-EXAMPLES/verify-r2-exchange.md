# Verify Round 2 Exchange (Beat5_Interfaces)

**Verdict: FAIL**

Defects: **2 blocker, 6 major, 9 minor.**

Scope: clip `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py` (class `Beat5_Interfaces`, 16 keys, en and de). Frames `T/🗑️generated/frames/Beat5_Interfaces/` (all 32 PNG read, both layout reports read). Model values computed with the project interpreter through `T/🗑️generated/verify-exchange/values_check.py`; zoom crops in the same folder. Research read: `T/research-villa-savoye.md`, `T/research-ifc.md`, `T/scene-exchange.md`, `T/agent-rules.md`. No `T/audit-r2-exchange.md` exists.

Severity: blocker = breaks a stated rule or teaches a wrong fact or a wrong object link; major = a printed number or label that does not hold, or a drawn value without its meaning; minor = wording, units, cosmetics.

## Defects

### Blockers

**B1. `domain` (en, de). Example block, highlight and leaders point at different objects.**
- Location: `_beat_domain` in the clip file. The highlight and the shell leader use `analysis["shells"][-1]` (SH-D5). The support leader uses `view.at("A5.support")`. The example text uses `analysis["shells"][0]` (SH-A0) and the B3 foot (`F-B3`).
- Frames: `Beat5_Interfaces-en-domain.png`, `Beat5_Interfaces-de-domain.png`. The highlighted cell is top right, the green leader ends at the bottom-right node A5, and the box reads `SHELL SH-A0 …` and `FIX F-B3 FIXED`. The B3 member leader is correct.
- Fix: use one object per role. Highlight and wire `sh = analysis["shells"][0]`. Wire the support to the B3 foot node `sp["node"]` (F-B3). Make the text, highlight and leaders all use `sh`, `mb` and `sp`.

**B2. `purpose` (en, de). A schema section is printed as an IFC entity.**
- Location: `_beat_purpose`, line 532, `analysis['target_domain']` shown as "here with IFC entities: IfcStructuralAnalysisDomain" (de: "hier mit IFC-Entitäten: …").
- Frames: `Beat5_Interfaces-en-purpose.png`, `Beat5_Interfaces-de-purpose.png`.
- Problem: `IfcStructuralAnalysisDomain` is the name of the structural analysis schema section, not an entity. The research lists the entities actually used (`IfcStructuralSurfaceMember`, `IfcStructuralCurveMember`, `IfcStructuralPointConnection`). The owner rule is that IFC names are real IFC 4.3 entity names. Research open item 5 (`T/research-ifc.md`, section 16) still has the exact relations unverified.
- Fix: keep the domain only as a domain name. Replace the line with the entity names from `analysis["mapping"]`, for example "IFC entities: IfcStructuralSurfaceMember, IfcStructuralCurveMember" (de the same with German wording), and verify against 4.3.2.0 before publication.

### Major

**M1. `interface` (en, de). Sender count does not match the sender drawing or file A.**
- Location: `_beat_interface` (sender label built from `f.objects['architecture']`, set in `_facts`). `partial_model('architecture')` has 28 tags and no pilotis. The sender axonometric uses `disciplines=("architecture", "structure")`, so the 25 pilotis are drawn. File A (`_beat_semantic`) exports B3 as an architecture `IfcColumn`, but B3 is not in the architecture partition.
- Frames: `Beat5_Interfaces-en-interface.png` ("Sender · architecture model / 28 objects"), `Beat5_Interfaces-de-interface.png` ("28 Objekte").
- Fix: decide who owns the pilotis. If the architecture model exports B3, add the 25 pilotis to the architecture partition and show 53 (or "28 + 25 shared"). Otherwise draw the sender without pilotis and take file A from the structure partition. Either way the label must count what is drawn and what is exported.

**M2. `cells` (en, de). Printed arithmetic fails.**
- Location: `_beat_cells`, rows at lines 862–865. The printed values are 22.6 m² (full cell) and 7.0 m² (cut), and the result is printed as 15.5 m². 22.6 − 7.0 = 15.6. Computed: 22.5625 − 7.0275 = 15.535.
- Frames: `Beat5_Interfaces-en-cells.png` ("full cell … = 22.6 m²", "ramp opening cuts away 7.0 m²", "B3 cell = 15.5 m²"), `Beat5_Interfaces-de-cells.png`.
- Fix: print two decimals: 22.56 m², 7.03 m², B3 cell 15.53 m² (this also matches the area shown in `feedback`).

**M3. `feedback` (en, de). One reaction shown two ways, and the product does not hold.**
- Location: `_beat_feedback`, table rows at lines 884–886 print R with one decimal (151.5 kN) and A as 15.53 m². The analysis-tool label on the same frame prints "151 kN" (line 896 uses 0 decimals). Computed R = 151.46 kN; 15.53 × 9.75 = 151.42, not 151.5.
- Frames: `Beat5_Interfaces-en-feedback.png` ("Reaction R 151.5 kN" next to "151 kN"), `Beat5_Interfaces-de-feedback.png` ("151,5 kN").
- Fix: print R with 0 decimals (151 kN) like `reaction` and `buildup`, keep A at 15.53 m². Then 15.53 × 9.75 = 151.4, which rounds to 151 and holds.

**M4. `properties`, `purpose`, `buildup`, `slab`, `mapping`, `roundtrip` (en, de). Assumed geometry shown as fact.**
- Shown without a basis: level "+3.50" and dimension "3.25 m" (`properties`); thickness "t = 0.25 m" (`purpose`, `buildup`, `assumptions` formula); cantilever "1.25 m west and east" and the ramp opening outline 7.6–10.0 × 2.8–14.5 m (`slab`, `mapping`).
- The model marks storey elevations, slab thickness and wall build-up as assumed (`C/🧱️model/🔣️.json`, assumption id `piloti-diameter`, and the `storeys` entries with `"basis": "assumed"`). Research has no source for storey heights (fact 2.6), slab thickness (3.2), cantilever lengths (3.4) or ramp dimensions (3.5). Only the piloti diameter carries "(assumed)" on screen.
- Fix: add "(assumed)" at the first appearance of each value (use `information(...)["basis"]` and the storey basis). Alternatively one "teaching assumption" line per beat. The loads and support already carry "assumed", so this is the same convention.

**M5. `slab` (en, de). "Roof garden" placed on the first-floor slab.**
- Location: `_beat_slab`, names at lines 750–752. The label "SP-TERRACE · Roof garden (hanging garden), 110.2 m²" is the first-floor storey (`"storey": "first"`, 3.5 m, assumed). The model also has a `roof` storey at 6.7 m and a roof slab `SL-R`, neither used for the garden.
- Research: the garden area 110 m² is sourced (2.7). Its height of about 3.5 m is only a low-confidence intention (2.9), so the placement is not established.
- Frames: `Beat5_Interfaces-en-slab.png`, `Beat5_Interfaces-de-slab.png`.
- Fix: either move `SP-TERRACE` to the roof storey and use `SL-R`, or relabel it "hanging garden at first-floor level (height assumed)" and mark the basis, as in M4.

**M6. `mesh`, `roundtrip` (en, de). Volume comparison printed but not drawn.**
- Location: `_beat_mesh`, live labels at lines 489–491: "mesh volume 0.2239 m³", "true cylinder 0.2297 m³ (−2.6 %)". `_beat_roundtrip`: "volume −2.6 %". Only the triangulated piloti is drawn. The true cylinder that the percentage compares against is not drawn.
- Frames: `Beat5_Interfaces-en-mesh.png`, `Beat5_Interfaces-de-mesh.png`, `Beat5_Interfaces-en-roundtrip.png`, `Beat5_Interfaces-de-roundtrip.png`.
- Fix: overlay a dashed true circle of radius 0.15 m on the top-view 16-gon in `_beat_mesh` (the top view already uses scale 6.5), and in `roundtrip` an outline on the returned piloti. The −2.6 % then has a drawn meaning.

### Minor

**m1. `domain` (en, de).** The example block has unit-less numbers: "SHELL … 0.25", "BEAM … 0.3". The 0.3 diameter is assumed and not labelled. Fix: "t 0.25 m", "Ø 0.30 m (assumed)".

**m2. `check` (en, de).** The line "units: m, kN, kN/m²" is `True` hard-coded in `_beat_check` (line 970), while the GlobalId and revision checks are computed. Fix: compute it from the model's unit set or drop the check.

**m3. `loss` (en, de).** The chip "× 4 Render colour (white) is not kept" counts walls, not colours. Fix: reword the model label for `IfcWall.STANDARD:material.layer.render` to "× 4 walls: render layer (white) not kept".

**m4. German wording (`slab`, `cells`).** "Die Decke ist die der Villa: …" is stilted (en "The slab is the villa's own"); suggested: "Die Decke ist original: …". "Einflussflächen-Näherung" (`cells`, de) should be "Lasteinzugsflächen-Näherung", the term used elsewhere on the same screen.

**m5. German terminology (`assumptions`, `buildup`).** "Ausbaulasten" (table and narration) and "Ausbau" (buildup label and legend, line 786). EN uses "finishes" throughout. Fix: "Ausbaulasten" everywhere.

**m6. `mapping` (de).** The bottom label "24 Schalen · 35 Knoten · 25 Stäbe · Rampenöffnung nicht vernetzt" ends about 15 px from the right edge (x ≈ 1265 of 1280). Not clipped. Fix: shorten or wrap.

**m7. `interface` (en, de).** Both axonometrics are faint in the final frame; only the pilotis and the B3 halo stand out. Legible, but low contrast. If the fade is intentional, ignore; otherwise raise the opacity.

**m8. `slab` (en, de).** The orange `V-RAMP` leader and the green `SP-TERRACE` leader cross. The cantilever leader ends only at the north-west corner although the cantilever is on both sides. Fix: swap the label order and point the cantilever leader at both strips.

**m9. `interface` (en, de), teaching-model badge.** The badge states "25 pilotis" and is the only teaching-model statement. The model's assumption notes record two open source conflicts that are not shown: the piloti count (25 vs 15) and the ribbon-window extent (all sides vs south only). The loss caption also states that infill walls carry no load, which research leaves to verify (section 13). Fix: add "sources differ" to the badge.

## Criteria

1. **Value and meaning at the same moment: FAIL.** Failing: `domain` (B1), `mesh` and `roundtrip` (M6). Holding: the dimension lines, hatched areas, load arrows, B3 cell and reaction arrows, and the STEP token rings.
2. **Numbers, arithmetic, research: FAIL.** Failing: B2, M1, M2, M3, M4, M5. Holding (recomputed):
   - 25 pilotis; 28 architecture and 33 structure partition counts (see M1 for the sender label)
   - 24 shells, 35 slab nodes at z = 3.375 m, 25 members, 25 supports
   - gross 21.5 × 19.0 = 408.5 m²; minus 28.08 = 380.4 m²
   - self-weight 25 × 0.25 = 6.25; 6.25 + 1.50 + 2.00 = 9.75 kN/m²
   - 9.75 × 15.5 = 151.1 kN; edge 11.28 m² → 110 kN; corner 8.61 m² → 84 kN
   - Σ R = 3,709.1 kN = 9.75 × 380.4
   - mesh 34 vertices, 64 triangles, volume 0.2239 vs cylinder 0.2297 m³ (−2.6 %)
   - +3.50 − 0.25 = 3.25 m; +3.375 = mid-surface
   - 49/49 GlobalIds; 25/25 IDS pass
3. **Overlaps, clipping, readability: PASS.** No overlapping text or clipped labels in 32 frames. Captions stay in the band. The 22 strict findings are leader lines running along the STEP baseline and rings around tokens; zooming to 2× (`zoom-en-properties-steps.png`, `zoom-en-semantic-fileA.png`) shows every token and value legible. Only m6, m7 and m8 remain.
4. **Narration and visual sync: PASS on content, timing not verifiable.** In each frame the caption's claim is visible. Captures are end states (`capture.py` runs each step at `run_time=.01` with skip animations), and timing comes from the framework's `hold_for`. Packet motion, the reaction build-up and the ValueTracker transitions were not seen.
5. **German: PASS.** All 16 keys in German, same structure as English. Decimal comma correct in running text, and STEP numbers keep the dot as ISO 10303-21 requires. The umlaut `Piloti-St\X2\00FC\X0\tze` is correct STEP escaping. Wording defects are m4, m5, m6.
6. **Depth comparable to energy clips: PASS.** This clip has 16 narration clauses, each with a concrete Villa Savoye element. The energy clip opened (`⚖️heating-comparison`) has 4 clauses. Energy frames were not rendered for comparison.

## Notes

- Not verified: IFC 4.3 names against buildingSMART (research marks the pages as unreachable, 403). Names were checked only against `T/research-ifc.md`. The STEP excerpt was checked for attribute counts (IFCCOLUMN 9, IFCCIRCLEPROFILEDEF 4, IFCEXTRUDEDAREASOLID 4, IFCBUILDINGSTOREY 10, IFCCOLUMNTYPE 10) and its ids, and matches the model.
- Scratch files are in `T/🗑️generated/verify-exchange/`: `values_check.py`, `crop_zoom.py` and the four zoom PNGs.
