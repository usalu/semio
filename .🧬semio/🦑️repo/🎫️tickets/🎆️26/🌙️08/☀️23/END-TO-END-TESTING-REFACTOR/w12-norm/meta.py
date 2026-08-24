# 📓️ Per-artifact facts for the 📕️norm mutation cases. Every sentence here is read off the
# artifact's own committed source (mutation-module doc header, snapshot struct, committed example
# and committed per-kind fixtures) — nothing is invented.
META = {
 "📓️iso16757": dict(
   slug="iso16757", ty="Iso16757", mod="iso16757",
   title="ISO 16757",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="ISO 16757 — product data for building services, the catalogue/dictionary exchange standard",
   shape=(
     "This is the RICHEST document shape in the plugin and the only one besides `📔️vdi3805` whose "
     "vocabulary is a lifecycle rather than a parameter form. `Iso16757Snapshot` is a multi-collection "
     "document — catalogue, dictionary, geometry, selection, part-number rule, part-number inputs, "
     "script limits and exchange process — and the twenty-one kinds split into three genuinely "
     "different families: document-root scalars (`change-exchange-process`, `update-script-limits`, "
     "`replace-part-number-rule`, `change`/`remove-part-number-input`, `change-selection-class`, "
     "`change-selection-series`), ordered constraint edits on the selection facet "
     "(`add`/`remove-selection-constraint`), and full create/delete(+rename) lifecycles over four "
     "id-keyed collections — the catalogue's `product_groups` and `products`, its "
     "`property_definitions`, and the dictionary's `subjects`."),
   distinguishing=(
     "What the fixtures are chosen to expose is REFERENTIAL: `delete-product-group` is committed as "
     "`removes-the-radiators-group-and-strands-its-class`, i.e. the vector deliberately leaves a "
     "dangling class reference behind, so an implementation that silently cascades — or silently "
     "repairs — the reference fails against the committed after-snapshot. `create-product` appends "
     "into an EXISTING series rather than a fresh one, and `create-subject` appends under an existing "
     "parent, so an implementation that ignores the parent/series address and appends at the document "
     "root cannot pass either. `rename-catalogue` and `rename-manufacturer` are the only two identity "
     "mutations in this vocabulary; every other collection member is addressed by id."),
   deferred=(
     "The vocabulary is deliberately partial and says so in its own module header: `product_classes`, "
     "`product_series`, `product_indexes`, `descriptive_objects`, the `accessories`/`compositions` "
     "edges, the dictionary's `relationships`/`properties`/`controlled_lists`/`meta_subjects` and the "
     "whole `geometry` pool carry no mutation yet. This case measures the twenty-one kinds that DO "
     "exist; it makes no claim about the deferred surface, which is tracked in that header rather "
     "than hidden behind a `deferredKinds` entry here."),
 ),
 "📔️vdi3805": dict(
   slug="vdi3805", ty="Vdi3805", mod="vdi3805",
   title="VDI 3805",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="VDI 3805 — manufacturer product data for building services",
   shape=(
     "`Vdi3805Snapshot` carries a manufacturer-file header, an id-keyed `catalog.products` pool, "
     "edition-profile overrides per VDI sheet, a correction cut-off date, a strict-mode flag, "
     "parametric geometry definitions with named connections, characteristic curves, and security "
     "limits on untrusted input. The nineteen kinds cover the header and policy scalars "
     "(`update-manufacturer-file`, `change-correction-as-of`, `change-strict-mode`, `update-limits`, "
     "`change`/`remove-edition-profile`), the product lifecycle (`create`/`delete`/`rename-product`, "
     "`replace-product-configuration`), the geometry lifecycle (`create`/`delete`/`resize-geometry`, "
     "`add`/`remove-geometry-connection`, `replace-geometry-parameters`) and the curve lifecycle "
     "(`create`/`delete-curve`, `replace-curve-points`)."),
   distinguishing=(
     "The one thing that genuinely separates this vocabulary from every other in the plugin is "
     "DERIVED PERSISTED STATE. `catalog.index` mirrors `catalog.products` one-to-one — it is written "
     "to the document, not recomputed on read — so every product mutation has to keep it in lockstep "
     "or the document is internally inconsistent the moment it is saved. The committed fixtures are "
     "named for exactly that obligation and are useless without it: "
     "`appends-vlv-80-002-and-its-index-entry`, `removes-vlv-50-001-and-its-index-entry`, "
     "`retitles-vlv-50-001-and-resyncs-its-index-tags`, "
     "`reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn`. An implementation that edits the "
     "product and forgets the index passes nothing here, and neither does one that rebuilds the whole "
     "index from scratch and reorders it."),
   deferred=None,
 ),
 "📕️din4108": dict(
   slug="din4108", ty="Din4108", mod="din4108",
   title="DIN 4108",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="DIN 4108 — thermal protection and energy economy in buildings",
   shape=(
     "`Din4108Snapshot` is seventeen document-root scalars — assembly category, climate zone, "
     "airtightness (n50 and class), thermal-bridge sum, indoor relative humidity and design "
     "temperature, catalogue and material ids, solar absorptance, design irradiance, interior and "
     "exterior vapour-diffusion mu values, envelope area, the Beiblatt-2 conformity flag, application "
     "type and declared application class — plus `layers`, an id-less ORDERED construction build-up. "
     "That gives seventeen `change-<field>` kinds and five collection kinds: `insert-layer` and "
     "`remove-layer` address by index (inserted = final-state index, removed = base-state index), "
     "`reorder-layers` moves one layer inside the build-up, and `change-layer-thickness` / "
     "`change-layer-lambda` edit one field of one layer by base-state index."),
   distinguishing=(
     "Layer ORDER is physics here, not presentation: a construction is a sequence from inside to "
     "outside, and the interstitial-condensation check reads it in that sequence. The committed "
     "fixtures are chosen against that — `inserts-an-interior-plaster-layer-at-index-1` inserts in the "
     "MIDDLE rather than appending, `removes-the-load-bearing-masonry-layer` removes a non-terminal "
     "member, and `moves-the-insulation-in-front-of-the-masonry` is a reorder whose whole meaning is "
     "the position swap. An implementation that treats `layers` as an unordered set, or that appends "
     "on insert, matches none of the three committed after-snapshots. Together with `📘️en1990` this is "
     "one of only two norm vocabularies with an ordered collection at all."),
   deferred=None,
 ),
 "📗️din16798": dict(
   slug="din16798", ty="Din16798", mod="din16798",
   title="DIN EN 16798-1",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="DIN EN 16798-1 — indoor environmental input parameters for energy performance",
   shape=(
     "Sixty-two document-root scalars and not one collection: this is the largest flat mutation "
     "vocabulary in the repository, and every kind is a `change-<field>`. The fields group into the "
     "standard's own clause families — thermal comfort (operative temperature, humidity, draught air "
     "speed, running-mean outdoor temperature), air quality (CO2, IDA class, supply airflow), "
     "daylight and acoustics, three separate occupancy models (non-residential persons, dwelling "
     "bedrooms, residential occupants) each with their own airflow field, specific fan power, heat "
     "recovery (achieved and required efficiency, mass flow, specific heat, temperature lift, "
     "operating hours, savings reference), infiltration and blower-door, cellar ventilation, "
     "transmission and ventilation heat transfer, cooling (set point, period, gains, utilization "
     "factor, reference, chiller type, EER, annual demand), storage and DHW, and duct leakage."),
   distinguishing=(
     "Sixty-two independent scalars is precisely the shape in which a vocabulary rots silently: "
     "adjacent fields differ by one word (`change-heat-recovery-eta` versus "
     "`change-heat-recovery-eta-min`, `change-humidification-required-kg-h` versus "
     "`change-humidification-provided-kg-h`, `change-hr-th` versus `change-storage-th`), so a diff "
     "builder that writes the neighbouring field is invisible to any check weaker than a full "
     "snapshot comparison. That is what this case runs: every one of the sixty-two committed vectors "
     "is applied and compared as a WHOLE document, not field-by-field, so a mutation that also moves a "
     "field it was not asked to move fails. The committed fixtures also pin the DIRECTION of each "
     "edit — `tightens-the-comfort-category-to-i`, `relaxes-the-indoor-air-class-to-ida-3`, "
     "`halves-the-measured-duct-leakage-to-0-point-0625` — so a sign error is a red scenario rather "
     "than a different-but-plausible number."),
   deferred=None,
 ),
 "📘️en1990": dict(
   slug="en1990", ty="En1990", mod="en1990",
   title="EN 1990",
   example="📕️high-consequence-office", dsl="🗣️high-consequence-office.dsl.semio", pack="🎒️high-consequence-office.pack.semio",
   standard_line="EN 1990 — basis of structural design",
   shape=(
     "The SMALLEST vocabulary in the plugin, and the only one whose collection is a composed CHILD "
     "artifact. `En1990Snapshot` is five document-root scalars (`g_k`, `resistance_kn`, "
     "`consequence_class`, `annex`, `seismic_a_ed_kn`) plus `q_k`, which is not an inline `Vec` at all "
     "but a fixed `s.stdio.semio.table` child slot holding a handle — a `child_id` and an "
     "`ArtifactRef` — to a separate table artifact. Five `change-<field>` kinds cover the scalars; "
     "`insert-variable-action`, `remove-variable-action`, `reorder-variable-actions`, "
     "`change-variable-action-category` and `change-variable-action-value` reach through the handle "
     "into the composed table."),
   distinguishing=(
     "Composition is what this case exists to protect. The committed before/after snapshots carry the "
     "literal child handle (`\"childId\": \"en1990-qk-7904dd65836c8ff4\"` plus its dialect-qualified "
     "`ArtifactRef`), and `switches-the-national-annex-from-de-to-en` asserts that a scalar edit "
     "leaves that handle byte-identical rather than re-minting it — an implementation that rebuilds "
     "the child on every write would produce a plausible-looking document that no longer resolves to "
     "the same table.\n"
     "  ⚠️ Four of the ten committed vectors — `remove-variable-action`, `reorder-variable-actions`, "
     "`change-variable-action-category` and `change-variable-action-value` — carry "
     "`{\"status\": \"rejected\"}` in their committed `🎯️outcome`, because the fixture's child slot is "
     "unseeded and index 0 does not exist. Those four are not weaker rows, they are a STRICTER "
     "contract: the scenario requires the mutation to be refused AND the document to come back "
     "bit-identical, so an implementation that silently clamps an out-of-range index, or that "
     "half-applies before noticing, fails where a plain \"the projection moved\" check would have "
     "passed it. The remaining six carry `{\"status\": \"applied\"}` and are held to the observability "
     "law instead."),
   deferred=None,
 ),
 "📘️en1991": dict(
   slug="en1991", ty="En1991", mod="en1991",
   title="EN 1991",
   example="📕️retail-hydrocarbon-fire", dsl="🗣️retail-hydrocarbon-fire.dsl.semio", pack="🎒️retail-hydrocarbon-fire.pack.semio",
   standard_line="EN 1991 — actions on structures",
   shape=(
     "Thirty-two document-root scalars, one `change-<field>` each, spanning the whole of Eurocode 1: "
     "loaded area and imposed-load category, national annex, self-weight (material and layer "
     "thickness, plus an assumed characteristic value), fire (curve, required resistance, member "
     "capacity), snow (zone, altitude, characteristic load), wind (zone, basic speed), thermal delta "
     "T, construction activity, accidental impact (vehicle mass and speed), bridge traffic (notional "
     "lanes, span, lane width, moment resistance), crane and hoist classes with hoisting speed, silo "
     "bulk material (density, height, hydraulic radius, wall friction mu, lateral pressure ratio K) "
     "and the size and dynamic factors c_s and c_d."),
   distinguishing=(
     "This vocabulary is the one whose SPELLING is load-bearing, and its own module header says why: "
     "the derive's `to_kebab` merges adjacent all-caps runs when no lowercase letter anchors a word "
     "boundary, so `ChangeEnVBMS` becomes `change-en-vbms` and not `change-en-v-b-m-s`, "
     "`ChangeEnSKKnM2` becomes `change-en-sk-kn-m2`, `ChangeCS` becomes `change-cs` and `ChangeCD` "
     "becomes `change-cd` — while the payload's own Rust field still addresses `en_v_b_m_s`. The "
     "catalog beside this feature is generated from each leaf's own `SemanticDescriptor.kind`, not "
     "from a hand-transliteration of the variant name, and `kinds_match_the_enum_and_the_catalog` "
     "fails the moment those two spellings part company.\n"
     "  The committed fixture chosen for the identity scenario is a REAL retail hydrocarbon-fire "
     "case, not a default document: `switches-fire-curve-to-hydrocarbon` and "
     "`extends-fire-resistance-to-120-min` are the same design decision the example asset records."),
   deferred=None,
 ),
 "📘️en1992": dict(
   slug="en1992", ty="En1992", mod="en1992",
   title="EN 1992",
   example="📕️liquid-retaining-fem-anchor", dsl="🗣️liquid-retaining-fem-anchor.dsl.semio", pack="🎒️liquid-retaining-fem-anchor.pack.semio",
   standard_line="EN 1992 — design of concrete structures",
   shape=(
     "Thirty-five document-root scalars, one `change-<field>` each, feeding five distinct EN 1992 "
     "checks: bending and shear (M_Ed, V_Ed, f_ck, b, d, A_s, f_yk, rho_l, N_Ed, P, A_c, the FEM "
     "toggle, span and UDL), fire (rating and provided axis distance), bridge fatigue (concrete "
     "stress and steel stress range), the liquid-retaining crack-width check (tightness class, "
     "h_D/h ratio, sigma_s, rho_p,eff, f_ct,eff, E_s and s_r,max) and the anchor check (h_ef, the "
     "cracked-concrete flag, f_uk, f_yk, A_s, d, c_1, N_Ed and V_Ed)."),
   distinguishing=(
     "Three of the five families carry their OWN copy of a symbol that already exists at the document "
     "root — `change-liquid-sigma-s-mpa` beside `change-bridge-delta-sigma-s-mpa`, "
     "`change-anchor-as-mm2` beside `change-as-mm2`, `change-anchor-f-yk-mpa` beside `change-f-yk`, "
     "`change-anchor-n-ed-kn`/`change-anchor-v-ed-kn` beside `change-n-ed-kn`/`change-v-ed-kn`. They "
     "are different physical quantities in different clauses that happen to share a symbol, and the "
     "single most likely defect in this vocabulary is a diff builder wired to the wrong one of a "
     "pair. Every scenario below compares the whole document, so writing the sibling field is a "
     "failure rather than a plausible number.\n"
     "  The committed example is a liquid-retaining structure WITH a FEM run and an anchor check, so "
     "the real asset actually exercises all three of the families that overlap."),
   deferred=None,
 ),
 "📘️en1993": dict(
   slug="en1993", ty="En1993", mod="en1993",
   title="EN 1993",
   example="📕️high-strength-connection", dsl="🗣️high-strength-connection.dsl.semio", pack="🎒️high-strength-connection.pack.semio",
   standard_line="EN 1993 — design of steel structures",
   shape=(
     "The ONE norm vocabulary that is not a parameter form. `En1993Snapshot` carries 74 scalar "
     "fields, yet declares only seventeen mutations: `change-annex` for the lone document-identity "
     "scalar, and sixteen `update-<family>-inputs` kinds — member properties, fire, cold-formed, "
     "stainless, plated, silo shell, bolt, weld, fatigue, through-thickness, tension component, HSS, "
     "bridge, tower, pile and crane. The grouping is not editorial: `⚙️engine`'s "
     "`check_full_steel_member` has one region per EN 1993 part, each calling exactly one check "
     "function with exactly that part's fields, and the mutation families are those argument sets."),
   distinguishing=(
     "This is the only place in the plugin where the derivation rules' `update-<facet>` exception is "
     "applied at scale, and the fixtures are written to prove the grouping is real rather than "
     "convenient. `moves-the-connection-to-four-m24-grade-10-9-bolts` changes bolt count, diameter "
     "and grade in ONE mutation, because `bolt_e1_mm` alone means nothing without `bolt_e2_mm` and "
     "`bolt_d0_mm`; `thickens-the-cold-formed-flange-and-reverses-its-stress-gradient` and "
     "`upsizes-the-stainless-section-to-a-duplex-grade` do the same for their parts. "
     "`update-silo-shell-inputs` is the deliberate exception to the one-part-one-mutation rule: "
     "`silo_t_mm` and `silo_r_mm` are read by both the part 1-6 shell-buckling check and the part 4 "
     "silo-wall check because they describe ONE physical silo, so they live in one group rather than "
     "being duplicated into two. Seventeen whole-document comparisons is therefore also a check that "
     "no group has quietly grown a field belonging to another part."),
   deferred=None,
 ),
 "📘️en1994": dict(
   slug="en1994", ty="En1994", mod="en1994",
   title="EN 1994",
   example="📕️composite-bridge-girder", dsl="🗣️composite-bridge-girder.dsl.semio", pack="🎒️composite-bridge-girder.pack.semio",
   standard_line="EN 1994 — design of composite steel and concrete structures",
   shape=(
     "Twenty-two document-root scalars, one `change-<field>` each: national annex, the design actions "
     "M_Ed and V_Ed, the plastic-resistance pair M_pl,a / M_pl,Rd with the degree-of-connection eta "
     "and the longitudinal shear resistance V_L,Rd, the fire inputs (insulation thickness, rating, "
     "deck type), the fatigue inputs (stress range and detail category), and the stud-connector set — "
     "shank diameter, stud height, f_ck, f_u, E_cm, the per-stud design shear, span, f_y, cycle count "
     "and the stud stress range."),
   distinguishing=(
     "Composite design is where a steel quantity and a concrete quantity sit next to each other under "
     "similar names, and this vocabulary keeps both: `change-f-ck-mpa` (concrete) beside "
     "`change-f-y-mpa` and `change-f-u-mpa` (steel), `change-e-cm-mpa` (concrete secant modulus) "
     "beside them, and `change-d-mm` (stud shank) beside `change-h-sc-mm` (stud height) — the two "
     "geometric inputs to the same push-out resistance formula, where swapping them still yields a "
     "number. Unlike its `📘️en1993` sibling this artifact takes NO `update-<facet>` grouping: its own "
     "module header records that none of the twenty-two fields forms a set that is never meaningfully "
     "set one field at a time, so the exception was not invented for it. The committed example is a "
     "composite bridge girder, which is why the fatigue and stud-connector fields carry real values "
     "rather than defaults."),
   deferred=None,
 ),
 "📘️en1995": dict(
   slug="en1995", ty="En1995", mod="en1995",
   title="EN 1995",
   example="📕️glulam-footbridge", dsl="🗣️glulam-footbridge.dsl.semio", pack="🎒️glulam-footbridge.pack.semio",
   standard_line="EN 1995 — design of timber structures",
   shape=(
     "Twenty document-root scalars, one `change-<field>` each: national annex, the design actions "
     "M_Ed, N_Ed and V_Ed, the section properties W, A, b and h, the characteristic strengths f_m,k, "
     "f_c,0,k and f_v,k, the two classification enums that drive k_mod and k_def — service class and "
     "load-duration class — the lateral-torsional M_crit, the connection inputs F_Ed and A_ef, the "
     "fire inputs (duration and section depth) and the footbridge vibration inputs (vertical "
     "acceleration and bridge cycle count)."),
   distinguishing=(
     "Timber is the Eurocode whose answer depends on two ENUMS more than on any number: k_mod and "
     "k_def are looked up from the service class and the load-duration class together, so "
     "`change-service-class` and `change-load-duration` move every derived resistance in the document "
     "at once while touching a single field. Both are exercised here as whole-document comparisons, "
     "which is the only way a lookup wired to the wrong axis of that table shows up. Its own module "
     "header is explicit that this artifact follows the `📘️en1992`/`📘️en1994` flat-scalar precedent and "
     "NOT `📘️en1993`'s per-part grouping, because `⚙️engine`'s EN 1995 checks read the snapshot as one "
     "flat bag of fields rather than as named per-part sub-structs — so the twenty kinds here are a "
     "derivation result, not a stylistic choice. The committed example is a glulam footbridge, which "
     "is why the vibration pair is real data."),
   deferred=None,
 ),
 "📘️en1996": dict(
   slug="en1996", ty="En1996", mod="en1996",
   title="EN 1996",
   example="📕️loadbearing-wall", dsl="🗣️loadbearing-wall.dsl.semio", pack="🎒️loadbearing-wall.pack.semio",
   standard_line="EN 1996 — design of masonry structures",
   shape=(
     "Twenty-two document-root scalars and enums, one `change-<field>` each: the design actions "
     "M_Ed, N_Ed, V_Ed and H_Ed, the section properties Z, gross area and shear area, the "
     "characteristic strengths f_k and f_vk, the national annex, masonry class, design situation, the "
     "friction coefficient mu, wall thickness, required fire resistance, the unit and mortar "
     "classifications, bed-joint thickness, storey count and the effective height and thickness "
     "h_ef / t_ef."),
   distinguishing=(
     "Masonry is characterised by CLASSIFICATIONS rather than by continuous properties, and this "
     "vocabulary keeps four of them side by side — `change-unit`, `change-mortar`, "
     "`change-masonry-class` and `change-exposure` — where the first two together determine f_k "
     "through a table lookup and the third and fourth select the partial factor. Four enum kinds in "
     "one document is the shape in which a lookup keyed on the wrong enum still returns a plausible "
     "strength, so each is committed with a whole-document after-snapshot rather than a spot check on "
     "the derived value. The effective-geometry pair `change-h-ef-mm` / `change-t-ef-mm` sits beside "
     "the physical `change-wall-thickness-mm`, and confusing effective with actual thickness is the "
     "classic EN 1996 slenderness defect — the fixtures keep all three separately addressable."),
   deferred=None,
 ),
 "📘️en1997": dict(
   slug="en1997", ty="En1997", mod="en1997",
   title="EN 1997",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="EN 1997 — geotechnical design",
   shape=(
     "Twenty-two document-root scalars and enums, one `change-<field>` each, covering two independent "
     "check families that share one ground model. The shallow-footing family carries the actions V_Ed "
     "and H_Ed, the footing area, the ground parameters phi, c and gamma, the footing width B, the "
     "embedment depth D_f, the stiffness E_s and Poisson's ratio nu, the settlement limit, and the "
     "design approach. The pile family carries N_Ed, the shaft factor alpha_s, the pile diameter and "
     "length, the shaft and base resistances q_s and q_b, the base area, the profile count and the "
     "investigated depth. `change-annex` and `change-design-approach` sit above both."),
   distinguishing=(
     "EN 1997's Design Approaches 1, 2 and 3 apply partial factors at DIFFERENT points — to actions, "
     "to resistances, or to ground properties — so `change-design-approach` is the one mutation in "
     "this vocabulary that changes the meaning of every other field without touching any of them. It "
     "is exercised as a whole-document comparison here for exactly that reason. The second thing this "
     "vocabulary is exposed to is family confusion: `change-b-m` (footing width) beside "
     "`change-pile-d-m` (pile diameter), `change-footing-area-m2` beside "
     "`change-pile-base-area-m2`, and `change-v-ed-kn` beside `change-n-pile-ed-kn` are pairs where a "
     "diff builder wired to the sibling still produces a number the check will happily consume."),
   deferred=None,
 ),
 "📘️en1998": dict(
   slug="en1998", ty="En1998", mod="en1998",
   title="EN 1998",
   example="📕️seismic-rc-frame", dsl="🗣️seismic-rc-frame.dsl.semio", pack="🎒️seismic-rc-frame.pack.semio",
   standard_line="EN 1998 — design of structures for earthquake resistance",
   shape=(
     "Forty-nine document-root scalars and booleans, one `change-<field>` each — the second-largest "
     "vocabulary in the plugin — spanning seven of EN 1998's own structure classes in ONE document: "
     "buildings (seismic zone, ground type, importance class, structural system, T_1, mass, V_Rd, "
     "drift, height, the multiple-resisting-systems flag), the EN-annex spectrum (a_gR, ground type, "
     "spectrum type, period ratio), bridges (V_Rd, bearing displacement demand and capacity), "
     "retrofit assessment (knowledge level, limit state, E_d, R_k, gamma_el), silos and tanks "
     "(height, radius, N_Rd, V_Ed, V_Rd, behaviour factor q, plus the tank mass and V_Rd), towers and "
     "chimneys (M_Ed, M_Rd, the chimney flag, q, mass), foundations (area, p_Rd, H_Ed, H_Rd, the two "
     "stiffness factors k) and retaining walls (height, phi, soil gamma, the ductility factor r, "
     "H_Rd)."),
   distinguishing=(
     "Seven structure classes in one flat namespace means the SAME symbol appears up to five times "
     "under different prefixes: V_Rd exists as `change-v-rd-kn` (building), `change-bridge-v-rd-kn`, "
     "`change-silo-v-rd-kn` and `change-tank-v-rd-kn`; the behaviour factor q exists as "
     "`change-silo-q-nominal` and `change-tower-q-nominal`; ground type exists twice, as "
     "`change-ground-type` and `change-en-ground-type`, because the national and EN spectra classify "
     "soil differently. That last pair is the sharpest: the two fields must be able to disagree, so "
     "an implementation that keeps them in sync as a convenience fails the committed after-snapshot "
     "for either kind. Forty-nine whole-document comparisons is what makes a prefix mix-up visible."),
   deferred=None,
 ),
 "📘️en1999": dict(
   slug="en1999", ty="En1999", mod="en1999",
   title="EN 1999",
   example="📕️aluminium-roof-purlin", dsl="🗣️aluminium-roof-purlin.dsl.semio", pack="🎒️aluminium-roof-purlin.pack.semio",
   standard_line="EN 1999 — design of aluminium structures",
   shape=(
     "Twenty-six document-root scalars and enums, one `change-<field>` each: the design actions N_Ed "
     "and M_Ed, the section properties A and W_el, the alloy selection, the buckling reduction chi, "
     "the torsion constant I_T and the critical length L_cr, the elevated-temperature theta, the "
     "fatigue set (applied and detail stress ranges, the slope m, and the cycle count), the fillet "
     "weld set (V_Ed, throat, length and the correlation factor beta_w), the thin-sheet local-buckling "
     "set (b, t, k_sigma, W_el and M_Ed) and the shell set (t, r and the applied meridional stress)."),
   distinguishing=(
     "Aluminium is the Eurocode where the ALLOY is not a material constant but a branch: "
     "`change-alloy` re-selects f_o and f_u, the heat-affected-zone softening factors and the "
     "buckling class in one step, which is why it is committed as a whole-document vector rather than "
     "as a strength edit. The vocabulary also carries THREE separate copies of the same section "
     "symbols because EN 1999 treats them as different members: `change-w-el-mm3` and "
     "`change-m-ed-knm` at the document root, `change-sheet-w-el-mm3` and `change-sheet-m-ed-knm` for "
     "the thin-sheet local-buckling check, and `change-shell-t-mm`/`change-shell-r-mm` for the shell. "
     "A diff builder wired to the root copy instead of the sheet copy produces a document that still "
     "checks out numerically and is wrong — which is exactly what a full after-snapshot comparison "
     "catches and a spot check does not."),
   deferred=None,
 ),
 "📙️din18599": dict(
   slug="din18599", ty="Din18599", mod="din18599",
   title="DIN V 18599",
   example="🎬️demo", dsl="🗣️example.dsl.semio", pack=None,
   standard_line="DIN V 18599 — energy efficiency of buildings, primary-energy balance",
   shape=(
     "Twelve document-root scalars — use class, heated area, occupants, the transmission and "
     "ventilation heat-transfer coefficients H_T and H_V, internal and solar gains, system losses, "
     "renewable yield, the annual primary-energy limit, the energy carrier and the reference Q_p — "
     "each with its own `change-<field>` kind, plus ONE `update-climate`."),
   distinguishing=(
     "`update-climate` is the only `update-<facet>` mutation in this artifact and the reason this "
     "vocabulary is thirteen kinds rather than fourteen: `climate: MonthlyClimate` is two parallel "
     "twelve-month arrays, `theta_e_c` and `g_h_w_m2`, which are always entered as one dataset — "
     "typically loaded whole from `MonthlyClimate::german_reference` for a climate zone — and never "
     "one month or one array at a time from this app's own input surface. Splitting it into two "
     "`change-*` mutations would let a document exist with outdoor temperatures from one zone and "
     "irradiation from another, which is not a state the standard admits.\n"
     "  ⚠️ Its committed vector carries `{\"status\": \"rejected\"}`: the fixture offers a climate "
     "dataset the artifact refuses, so the scenario requires the mutation to be REFUSED and the "
     "document to come back bit-identical. That is a stricter contract than the observability law the "
     "other twelve kinds are held to — a partial application of a twenty-four-value facet is exactly "
     "the defect it catches — and it is the one kind in this vocabulary whose forward effect is a "
     "rejection rather than a change."),
   deferred=None,
 ),
}
