# Primary Beam and Column Material Ownership R30

Implemented an explicit ownership partition for every positive primary beam/column source joint, preserving all column profiles, original post heights, beam outlines and source member extents. No historic silhouette was shortened to remove an intersection. The449-element source register remains the same size.

## Source and Schema Contract

Read physical-architecture-audit-r29.md in full, garden-r28.md, material-ownership-r19.md and facade-structure-r23.md. The portable joint fixture reproduces all97positive source section/beam-part pairs:31primary beam products and49distinct column products. Its pairwise common-volume sum is0.529620619921m³; this is expressly a sum of pairs, not an asserted global union correction.

The geometry schema now permits explicit joint_owners column tags. All31affected beam records carry the actual owner tags and updated inferred ownership evidence. A complete independent sweep of all63nonauxiliary beam products against every column section reproduces exactly the97registered pairs. This includes both cellar beam products; neither adds a positive column joint. Column14's ground structural section remains covered regardless of cellar containment. Noncolumn owners and stale nonintersecting owners fail at runtime.

The column owns the common occupied joint material. Its existing full geometry and material remain authoritative. Garden posts16/22 therefore retain all three functional section silhouettes and the existing finished rendered-concrete composite envelope; the joint partition does not invent a measured bare concrete/render boundary. Shared beam labels acknowledge that member envelope while its physical body excludes column-owned matter.

## Public Geometry and Quantities

For a beam, parts contains exclusive occupied beam material, shared_parts contains slab-owned member strips plus exact column-common material with owner, material and column_joint role, and full_parts contains the complete member geometry net of registered reserved passages. Full member parts include the original source beam sections and member-tagged slab strips. The source parts remain the uncut authored member envelope.

quantities.volume describes the full net member. exclusive_volume and shared_volume partition that quantity. material_quantities, weight and mass describe exclusively owned beam material, so combining physical product inventories does not count column joints or slab strips twice. The proven disjoint vertical beam partitions use direct cell integration; arbitrary geometry retains the existing occupied-union integrator. No external runtime geometry dependency was introduced. Existing no-owner AST tests, nonprismatic solids and isolated returned geometry remain supported.

## Fixed Rainwater Passages

The four actual F-2-7/F-4-10/R-2-7/R-4-10 beam records now register fixed100mm circular geometry.voids profiles, copied geometrically from their corresponding registered native slab openings. These fixed host profiles continue the existing column/slab reserve through the full raw beam vertical interval, including upstands between adjacent column intervals. They are independent of current pipe positions; rerouting a pipe does not drill a new host hole. The100mm clearance/profile registration is inferred; the nominal80mm pipe source dimension remains separately attributed.

Column ownership removes the gross column envelope from beam physical matter, while shared material contains only the occupied column annulus. Reserved bore volume is excluded from full physical member volume. Already-cut slab-owned strips retain their existing slab geometry. No column silhouette is extended across an unmodeled vertical interval.

The independent facade executor's five passage contracts passed in27.231s: all DP1/DP2 and outlet occupied material is disjoint from all110beam products, including cellar members, across physical/full/shared parts. The four fixed cuts match native openings, retain finite bounds, survive pipe rerouting, and independently preserve approximately9.994645mm minimum100/80mm polygonal clearance. See beam-passages-r30.md for that separate proof.

## Test-First Verification and Precision

The schema and portable97pair fixture were authored before implementation. The first actual pair oracle passed and the ownership test failed for missing source references. Final35joint contracts plus the two existing shared-beam contracts pass:37tests in15.196s. Tests prove complete source registration, owner validity, actual column profile/void intersections, per-elevation union conservation, material disjointness, full/exclusive/shared volume agreement, owned inventory volume, source immutability and native scope declaration. NumPy/SciPy ConvexHull and GEOS independently reproduce the source pair register and volume.

Near-coincident CAG fragment boundaries caused unquantized GEOS overlay calls to stall. The independent partition oracle therefore uses an explicit1e-10m GEOS precision grid, STRtree pair intersections, cell containment and area conservation against direct source polygons. Its1e-8m² identity tolerance is unchanged. Every cell also checks original-versus-snapped area against perimeter×1e-10m+1e-16m². This is a bounded precision oracle, not an exact unquantized-overlay claim. Product geometry was not snapped by these test-only operations. Tiny duplicate floating-point elevation bands are grouped at12decimals and intervals under1e-10m are omitted consistently.

## Native IFC

Native STEP excerpts now support IfcBeam through the shared occupied-part interface. Body contains exclusively owned material. A derived Pset_MaterialOwnership independently states BodyScope=exclusive_owned_material, the net authored full-member scope, FullMemberVolumeM3, OwnedVolumeM3, SharedVolumeM3 and the slab/column owner tags. This preserves semantic full member quantities without adding duplicate physical solids.

Pinned independent web-ifc0.0.78 decoded all12final samples: F-2-7,F-4-10,R-2-7,R-4-10,R-6-22 andR-20-32, each in English and German. Native identity, localized name, material ownership property values/owner references, placed bounds and occupied mesh volumes passed. Maximum volume error is approximately2.1e-8m³. For one language:

|Beam|Native parts|Native volume m³|API owned volume m³|
|---|---:|---:|---:|
|F-2-7|102|0.336346321886|0.336346303|
|F-4-10|1|0.337935416168|0.337935395|
|R-2-7|107|0.297787789811|0.297787786|
|R-4-10|1|0.296225720122|0.296225707|
|R-6-22|4|0.350111944206|0.350111933|
|R-20-32|297|0.598751830256|0.598751811|

No native Manim frame was produced; the repository launcher gate remains parent-owned. Tests certify the authored source reconstruction and material partition, not newly recovered historic measurements.

## Files and Coordination

Product edits: model/🐍️.py; model/🔣️.json (31ownership records plus four fixed beam passages only); model/🧬️schema/🔣️.schema.json; model/🔌️exchange/🐍️.py; model/🧪️tests/🏗️joints.json and🏗️joints.py. Existingbeamtests pass without fixture changes. Main suite discovery is owned by clips_r11; source analytical-link refresh and final whole-cohort validation are parent-owned.

Durable independent reader inputs are joints-r30-native.py and joints-r30-native.mjs. Generated samples, reader dependencies, output and temporary checkpoints are under 🗑️generated for parent final cleanup. No shell, Git mutation, worktree, IDE, AGENTS edit, checked-in source copy or migration was used. API/source geometry is frozen for the parent's final integration; native property export completed afterward without changing geometry.
