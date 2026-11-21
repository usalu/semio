# Norm specification coverage

The current goal is complete coverage of the provisions in every Norm family shipped by Semio. The previous browser smoke test established that the 15 apps start and render. This audit follows the normative requirement, its edition and jurisdiction, the implementation it reaches, and concrete expected outcomes. A repository target checklist can prove coverage of its own roster. Publisher documents determine whether that roster itself is complete.

## Current worktree

| Family | Repository target IDs | Check | Data | Design case | Explicit gaps |
| --- | ---: | ---: | ---: | ---: | --- |
| DIN EN 16798 | 21 | 16 | 0 | 0 | VE-04, VE-06, VE-08, VE-10, VE-12 |
| DIN V 18599 | 23 | 17 | 0 | 0 | DE-03, DE-09, DE-10, DE-12, DE-14, DE-15 |
| DIN 4108 | 28 | 9 | 0 | 13 | D-05, D-07, D-09, D-10, D-12, D-14 |
| EN 1990 | 30 | 29 | 0 | 1 | — |
| EN 1991 | 110 | 77 | 0 | 32 | E17.10 |
| EN 1992 | 160 | 63 | 1 | 96 | — |
| EN 1993 | 181 | 158 | 5 | 16 | V-506, V-609 |
| EN 1994 | 57 | 50 | 3 | 4 | — |
| EN 1995 | 68 | 34 | 0 | 34 | — |
| EN 1996 | 37 | 37 | 0 | 0 | — |
| EN 1997 | 61 | 61 | 0 | 0 | — |
| EN 1998 | 71 | 58 | 5 | 6 | AS-09, ST-03 |
| EN 1999 | 42 | 41 | 1 | 0 | — |
| ISO 16757 | 10 | 6 | 0 | 0 | ISO-03, ISO-05, ISO-07, ISO-09 |
| VDI 3805 | 38 | 9 | 21 | 0 | VD-02, VD-08, VD-11, VD-12, VD-20, VD-30, VD-67, VD-101 |
| **Total** | **937** | **665** | **36** | **202** | **34 explicit target gaps** |

`audit-norm-spec-traces.py` derives the inventory from the 15 artifact declarations and 15 actual trace rosters. A trace mapped to a check substring or design case still needs to demonstrate the applicable branches and outcomes for the selected edition. Independent worked examples are especially important where related target IDs map to the same check family. EN 1990 E90.08 now identifies the characteristic-action calculation itself. The catalogue distinguishes a qualified mean from two statistical or prescribed bounds, and 21 cases matched SciPy.

## Edition scope and source access

| Family | Publisher evidence and implementation implication |
| --- | --- |
| DIN 4108 | Use the applicable part edition and the current GEG reference. The catalogue exposes four §26 pressure-test calculations with persistent editor cases. The independent persistence and mutation checks passed. The 2024 Annex A moisture balance uses the fixed standard boundary pressures and matches the published drying example. Editable design cases supply the site elevation and roof exposure and cover both insulated slab cases, internally insulated exterior walls, Table 3 roofs and the Figure 17 attic ceiling limits. [Annex A verification](din4108-annex-a-period-2026-11-21.md) Fresh native and Play execution remain pending. [GEG §26](https://www.gesetze-im-internet.de/geg/__26.html) |
| DIN EN 16798 | Part 1 now distinguishes the EN and German zone design methods and retains the complete-mixing minimum. [Design airflow evidence](din16798-design-airflow-2026-11-21.md) · [Thermal design](din16798-thermal-criteria-2026-11-21.md) · [Local comfort](din16798-local-comfort-2026-11-21.md) Part 17 puts inspection frequency outside the standard and defers to national rules. Complete ventilation, controls and inspection procedures still require matching edition-specific evidence. [DIN EN 16798-1 German annex](https://www.normsplash.com/Samples/DIN/366818232/DIN-EN-16798-1-2022-de.pdf) |
| DIN V 18599 | DIN Media identifies DIN/TS 18599-1:2025-10 as the successor to DIN V 18599-1:2018-09. DIN/TS Supplement 1:2026-06 also has substantive changes. The live balance limits estimated renewable electricity credits month by month. The statutory equipment inputs remain in scope. Potsdam Annex E is represented by its horizontal, temperature and directional data. The residential hot-water balance now uses net floor area and dwelling count. The Annex 4 catalogue has all 16 rows and the report flags network-specific factors. The Annex 3 envelope report groups the four classes by heating setpoint and uses the statutory adjacency and perimeter corrections. The non-residential reference energy model uses the Annex 2 envelope rows. [Annex 2 reference envelope](din18599-annex2-reference-2026-11-21.md) [Annex 3 envelope](din18599-annex3-envelope-2026-11-21.md) [Primary-energy factors](din18599-primary-energy-factors-2026-11-21.md) [Residential useful hot water](din18599-residential-dhw-2026-11-21.md) [Monthly renewable electricity](din18599-monthly-renewable-electricity-2026-11-21.md) · [Potsdam reference climate](din18599-potsdam-climate-2026-11-21.md) [DIN V 18599-1](https://www.dinmedia.de/de/vornorm/din-v-18599-1/293515783) · [DIN/TS 18599 Supplement 1](https://www.dinmedia.de/de/vornorm/din-ts-18599-beiblatt-1/400836333) |
| EN 1990–1999 | The first generation has 58 parts. The transition is part-specific and national annexes attach to exact editions. DIN already publishes DIN EN 1999-1-1:2024-11 and its German annex:2025-10, while the coordinated national implementation and withdrawal deadlines are later. Match each trace to the actual adopted part and annex instead of assigning one generation date across all families. [CEN transition letter](https://eurocodes.jrc.ec.europa.eu/sites/default/files/2023-09/CEN-TC%20250_N3659_CEN-TC%20250%20Letter%20-%20Timeline%20for%20the%20evolution%20of%20the%20EN%20Eurocodes%20%282%29_0.pdf) · [JRC national annexes](https://eurocodes.jrc.ec.europa.eu/en-eurocodes-implementation/national-standards) · [DIN EN 1999-1-1](https://www.dinmedia.de/de/norm/din-en-1999-1-1/356881996) · [German annex](https://www.dinmedia.de/de/norm/din-en-1999-1-1-na/394180706) |
| ISO 16757 | ISO lists the published first editions of Parts 1, 2, 4 and 5, with a new Part 1 under development. Part 5 uses IFC and EN 17549-2. Verify the exchanged records against the prescribed IFC representation and its model view. [ISO catalogue](https://www.iso.org/cms/live/live/en/sites/isoorg/contents/data/committee/04/91/49180/x/catalogue/) · [ISO Part 5](https://www.iso.org/standard/85071.html?browse=tc) |
| VDI 3805 | Resolve sheet number, status, subject and edition from the publisher catalogue. The current array contains incorrect subject bindings. VDI identifies Part 10 as air filters and Part 19 as thermal solar collectors. Part 66 covers electrical installation boxes and Part 100 covers systems. Published and draft editions have distinct records. [VDI catalogue](https://www.vdi.de/mitgliedschaft/vdi-richtlinien/details/vdi-handbuch-produktdatenaustausch) · [Part 66](https://www.vdi.de/mitgliedschaft/vdi-richtlinien/details/vdi-3805-blatt-66-product-data-exchange-in-the-building-services-electrical-installation-boxes) · [Part 100](https://www.vdi.de/en/home/vdi-standards/details/vdi-3805-blatt-100-product-data-exchange-in-the-building-services-systems) |

Public publisher pages establish the scope, editions, and selected requirements. The checked worktree search found no complete primary-text corpus for these families. A definitive roster needs the edition-specific normative text, tables, amendments, national annexes and referenced exchange specifications wherever the public material stops at a synopsis or table of contents.

## VDI 3805 source and regression evidence

The VDI Part 100:2018-06 public scope requires a system package or combination to reference its component product records. The evaluator now emits a §1 result for the declared component links and refuses an empty list, empty identifiers and self-links. The existing catalogue referential-integrity check still validates references that are meant to resolve within its snapshot. Section 5 record syntax needs separate verification. [VDI Part 100 scope and normative references](https://www.vdi.de/fileadmin/pages/vdi_de/redakteure/richtlinien/inhaltsverzeichnisse/2810323.pdf)

In the production registry, Part 100 is now Systems (2018-06) with its cited Part 1 edition (2011-10). Part 66 is Electrical installation boxes (2026-04). The implementation removed its inherited hydraulic checks for Parts 66 and 100. Reports for these products surface an unverified record schema instead of claiming conformance based on those checks. [VDI Part 66 scope](https://www.vdi.de/fileadmin/pages/vdi_de/redakteure/richtlinien/inhaltsverzeichnisse/3652715.pdf)

The six language-neutral Part 100 vectors passed the exact Rust predicate against an independent SQLite projection. The checked-in Rust tests also target the normal evaluation and per-sheet results. A separate native crate run is queued and recorded below.

## National annex evidence

Ten EN 1992 registry entries recorded zero available German value as jointly verified. The official projection now carries their existing EN oracle evidence and explicit unverified status for the absent German value. The Rust model also refuses to mark an absent annex value verified through its constructors. The exact Rust registry model and full EN 1992 table compiled independently and matched the SQLite projection for 241 rows. The two missing-value constructor cases passed, and the full NDP table generator checked 3,994 entries against the committed output.

The baseline counted 253 rows marked verified for both annexes. The live post-edit census records 243 rows flagged for both annexes, 82 for EN alone and 3,669 for neither. The remaining flags and unverified values require edition-specific provenance review. A verified library oracle establishes the behavior or values it actually encodes. Establishing German annex compliance requires evidence that also resolves the nationally applicable value.

## Native tests and next requirements

Both targeted VDI 3805 native tests passed. The first common assessment invocation selected the library suite and ran zero matching tests. The retry targets the `unit` integration suite that contains the new German-annex assertion. Its output is `🗑️generated/ndp-compliance-native-integration.log`. The source-level passes above remain independent of the integration test.

Exact verification of VDI Part 66 §4 and Part 100 §5 requires the full published record descriptions and matching examples. EN 1991 now has catalogue cases for the Annex D railway fatigue calculations, HSLM load layouts and model applicability, crane loading class recommendations, cooling pressure in circular silos and tank wind and pressure actions. The reports record the implemented methods and requirements that remain. [EN 1991 railway fatigue](en1991-rail-fatigue-2026-11-21.md) · [EN 1991 crane classification](en1991-crane-classification-2026-11-21.md) · [EN 1991 silo and tank actions](en1991-silo-tank-actions-2026-11-21.md)

EN 1994 C-46 now includes Table 4.6 for partially encased columns through the design-case editor. Other tables and automatic validation of the fire load ratio remain in scope. [EN 1994 fire tables](en1994-fire-tables-2026-11-21.md)

Fresh native tests are pending. The attempted DIN EN 16798 scene check exited at the storage gate with 716.51 MiB available against a 5.5 GiB requirement. The workspace has no configured remote executor and the expected Play staging path has no component build to verify. The completion audit remains open across the 15 trace rosters. Remaining scope includes the 34 explicit targets, edition and annex provenance, and the clauses absent from the repository rosters.
