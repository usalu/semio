# Primary Dimensional Integration

The previous goal turn made concrete progress through finer podium/signage and capsule fixture work, and the objective audit identified recoverable core and dimensional discrepancies. This pass replaces the known2.75m kit pitch with the original section’s2.90m chain, the3.5m office band with3.72m, the assumed3.5m basement with the5.00m section depth, and the derived A roof datum with its42.14m primary height plus11.36m head.

The4.?0 ground-height glyph and B roof10.?3 glyph are degraded in the original1280px raster. They must remain explicitly inferred until stronger primary evidence is found; independent low-agent searches are active. Native CAD is a kit, not a measured all-room survey.

The native square capital GLB has corner height sets1,6,6,11.5m, confirming two roof facets rather than proving one planar cut. Current profile should not be flattened merely to make it coplanar. The datum correction will preserve this observable profile and primary A maximum, with each derived/inferred part recorded honestly.

Primary source: https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html . Source crops are generated research output, not copied product assets. Portable dimension tests will run red before data/derivation updates.

## Applied Primary Grid

Six portable primary-dimensional tests ran red (five geometry/value failures and one missing-datum error) before this update. The verticalgrid nowuses2.90m pitch; sparsefloor3 restores its native phase1/2 membership rather than an invented same-height pair. Working ground4.40 is explicitly derived from the ambiguous glyph and local raster ratio, upper3.72 gives roof8.12, basementreference5.00 is derived from the section endpoint interpretation. A uses explicit42.14/11.36 datums. B36.72/10.90 are explicitly provisional derived values with raster/portal constraints, not recovered exact digits. Every tower stores its own derived roofclearance; shared arbitrary2.8moverrun was removed. XYports/inventory/programs unchanged.

The explicitdatum implementation passed allsix primarydimension tests in1.697s. Actualcorewalltop, capitalenvelopeoverall53.50, officefloorband3.72, basementfloor−5.00 and all140threephaseplacements were checked with NumPy. The first-floorphase collapse is removed; logicalphase0 remains8.32m while physicalsparsefloor3entries usephases1/2. This is a consistency gate for the selected source interpretations, not recovery of the two still-ambiguous glyphs. GH/Excelbindingsandcoreintegrationarebeingresynchronised; a nonfinalvisualpreviewisrunningtoinspectnewdetail.

## Independent Head Parameters And Native Profile

Two additional regressions ran red: roof ratios differed fromnativeGLB and changingAheadrescaledB. Removedsharedcapital.heightfield/API ratherthan retaining a compatibilityalias; eachtowerheadnowusesitsownquantity. Nativecorner1/6/6/11.5proportionsscale uniformly toprimary A11.36maximum; steelrearframing followsactualprofilelow/highvalues. items.staggernowderives2.90/3. IndependentNumPyprofilesandperTowerparameterisolationarerunningagain.

## Measured-Archive Pose Corrections

The new four-case signed-surface/placement gate first failed three implementation checks while the independent NumPy relative-phase evidence passed. Registered pre-demolition mesh supports A floor3 W/Slash plus N/J, both phase1, and B Jphase1/Slashphase2. Bregularfloors4/6/11 confirm phase1/2 reversed acrossNorth/South/East relative to the kit, so allBnonzero phases are swapped. XYofBremains unchanged; AwrongW/SisreplacedwithobservedN/J. Actualbranchesincrease74→75 and singletons8→10; total140 andfloorcountsunchanged. Proposedprogramcountschange {"C":35,"D":37,"B":33,"A":35}→{"B":34,"C":35,"D":36,"A":35} and consumerfixturesmustfollow. This relativeposeproofdoesnotvalidate absolute scan metric scale.

## Scan-Calibrated Pitch And Exceptional A Floors

The original repeated glyph is re-read as2.80, replacing the earlier2.90 assertion. Independent calibration uses A53.50m, fitted local ground and supported roof apex; it gives scan pitch≈2.793m. The portable contract ran red with2 failures before correction. A4/5/13 E1/L is restored from positive facade evidence. S0 is removed under fixed seven-unit counts; top13 is directly supported by allseven other exposed roofs, lower4/5 remain inferred because the merged capture has occluded faces. All140 phasegrid elevations and perTower clearances are recomputed without changing counts. Proposed program counts now{"B":35,"C":33,"D":36,"A":36}.

## Final Registered Datum Interpretation

Three independent NumPy/raw-scan regressions ran red before correction. Flat podium roof is7.72m (scan≈7.71), distinct from upperedge≈8.1; officefloor4.00 is derived by subtracting sourced3.72 band, not a confident ground-glyph reading. Separate.40m coping preserves observed edge height. Sparse A phase1 nominal roofs11.153 agree with scan11.094 within.15m; the prior8.12support value missed by.459m. Nominal2.50m shell versus scanned≈2.715m roof/underside extent remains an explicitly unresolved convention, not invented low hardware. B36.62+10.83=47.45 is a derived working split agreeing within.12m with independently A-calibrated47.333; the exact degraded B glyph and tick mapping remain less secure than A datums. All140Zvalues, bridges and clearances nowderivefromthefinal2.80gridand7.72roof.

## Selected-Tower API And Core Metadata

Two regressions ran red before correction: partial heads were clamped to the global full-building capsule crown, and core envelope metadata still invented equally spaced floors from ground zero. capital_base now uses the explicit selected towerheight directly; defaultsource heights are unchanged and independently clearance-checked. Core envelope levels now use ground/office, actual phase0 capsule floors and the concrete roof datum, clipped for selectedheight. NumPy independently derives expectedlevels fromraw authored placements. No legacy aliases or silent height fallback remain.
