# Showcase documents EN 1993, 1994, 1998, 1999 (wave 2)

Nothing was compiled or run on the Rust side (no cargo). Python only: the four generators ran and validated their output against the artifacts' JSON schemas. The `repo` MCP server did not connect in this session (`CONNECTION_CLOSED`), so no `ticket_*` call was made. Rebuild request appended to `📌️important/requests.md` ("- ready for rebuild: showcase docs").

## 1. What exists now

| Code | Fixture (artifact `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`) | Content |
|---|---|---|
| EN 1993 | 26 collections, all populated | compliant HEB 240 frame, the populated row of every special part (SPEC `show` values of `🧰️scripts/en1993-special-spec.py`), end-plate and base-plate joints, stainless member, pipeline, annex rows (ring-stiffened cylinder, corrugated web, orthotropic deck, restrained purlin, stiffened panel), frame portal, variants of the trace tests (tension, torsion, parabolic diagram, category B bolts, fillet-welded joint) |
| EN 1994 | beams 7, columns 3, slabs 3, placements 1 | bridge-kind document (`fatigueDetail = reinforcement`): plate girder with FLM3 action and all girder-detail leaves, propped and unpropped, end, interior, two-span and cantilever floor beams, point load for patch loading, filled, encased and partially encased columns, simple, continuous and punching-loaded slabs |
| EN 1998 | 14 collections, all populated | compliant multipart subject plus: second building (joint gap, steel members, given period), masonry house (simple-masonry rule, wall slenderness), timber hall (ductile connection), pending building (no systems, Rayleigh without stiffness), nonstructural and capacity-design data, soil profile with liquefaction data (30 m), diaphragm, three synthetic accelerograms, bridge deck EI, two piers, four bearings (abutments with empty `pierId`, as `compliant_de_structures`), placement |
| EN 1999 | 14 collections, all populated | neutral 14-collection subject plus: welded purlin (transverse weld), free-outline section with polygon, slender lipped section (class and edge-stiffener checks), pinned and bolted connections, second fire scenario, second fatigue detail (Miner group), sheet patch load, transverse weld run |

Generators (python3 -I, idempotent): `🧰️scripts/showcase/{common,en1993,en1994,en1998,en1999}.py`. `common.finish` is the language-agnostic law: the document must validate against the artifact's own `🧬️schema/🔣️.json` (types, required members, no unknown members, enums), every array collection must be non-empty and ids unique per collection; numbers are written as floats. Values are physically plausible demo inputs; nothing is an invented normative value (NDP-governed limits stay in the registry).

Twins: the typed JSON is the committed twin. The DSL twin is derived, not committed: the committed example DSL assets are stale (the EN 1994 bridge-girder asset lacks column fields), so the palette entry is `encode_enXXXX_dsl(Snapshot::full_showcase())` like the EN 1990 to 1997 "Full catalogue example". A palette test decodes that DSL and the JSON twin back to `full_showcase()`.

## 2. Rust changes (unverified by a compiler)

- Model, `🧬️schema/📸️snapshot/🦀️.rs` of each code: `Snapshot::full_showcase()` (one `include_str!` of the fixture, `semio_framework_pack_json::from_json_str`, no Cargo change).
- Editors `✏️editor/🦀️.rs`: `full_showcase_example()` (id `full-showcase`, "Full showcase example" / "Vollständiges Gesamtbeispiel") chained into `examples()`; palette tests in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- Shared `⚖️compliance/🧭️spec-trace/🦀️.rs`: `unreached_fragments` and `assert_reaches_all_but(table, emitted, allowed)` (allowed = fragment plus reason of at least 20 characters; an unexplained or stale entry fails), unit test in the crate's tests.
- Tests: EN 1993 `catalogue-trace`, EN 1994 `💡️inferences/🧪️tests/🔬️unit`, EN 1998 `🧪️tests/⚖️compliance`, EN 1999 `weld-runs-and-outlines`: showcase added to the `assert_traced` document list, and `the_full_showcase_alone_reaches_every_check_fragment_of_the_trace` (asserts every collection non-empty, then `assert_reaches_all_but` on the showcase alone).
- Scene: EN 1998 `fixtures()` and the golden digest list gain `full_showcase` (digest text is written when missing); EN 1994 valid-scene list and golden list gain the showcase (empty placeholder `🎬️scene/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, like the two existing empty goldens, regenerate with `SEMIO_UPDATE_GOLDEN=1`); EN 1993 and EN 1999 (single-golden tests, no list) get a valid-scene test of the showcase.

## 3. Fragments the showcase cannot reach

| Code | Fragment | Reason |
|---|---|---|
| EN 1998 | `.zone0` | the very-low-seismicity exemption needs `a_gR = 0`, which makes the evaluator skip every other building check of the same document; listed in `SHOWCASE_UNREACHED` |

Nothing else is expected to be unreachable; this is read from the evaluator source (gates of every opt-in), not from a run. The first run decides: a missed fragment is listed by name in the test failure; fix by extending the generator (or, if structurally impossible, add it with its reason to `SHOWCASE_UNREACHED`). Gates I verified by reading and covered: EN 1994 `2.lm1` and all `2.6.x` need `structureKind = bridge`, `rebar` needs `fatigueDetail = reinforcement`, `6.4.patch` needs a point force on a beam, `9.7.6.punching` a point force on a slab, `5.4.2.analysis` and `6.2.1.2.hogging` a continuous or cantilever support; EN 1999 `8.5.pin` needs a pinned connection, `6.7.7.edge` an outstand element with an edge stiffener, `6.1.6.haz` a weld run, `1-3.fat.miner` two details on one member; EN 1998 `.joint.gap` needs a following building, `.system.na` a building without systems, `.t1Rayleigh` Rayleigh with zero stiffness, `.masonryWall`, `.timberConnection`, `.steel.class`, `.capacityDesign.nodes`, `.nonstructural.force`, `.groundClass`, `.profile.siteResponse`, `.timeHistory.*` their opt-in rows.

## 4. Findings

- EN 1994: no document of the existing `assert_traced` list sets `fatigueDetail = "reinforcement"`, the only gate of the check family `en1994.2.6.8.2.rebar`; that fragment was unreached before (the showcase fixes it).
- EN 1998: committed mutation fixtures (`✏️change-silo-height-m` and siblings) still carry `silo.shellStrengthMpa` and `shellThicknessM`, which `En1998Silo` does not have; EN 1993 fixtures use `annex: "en"` (lower case) while the schema enum is `En|De`.
- Some rows fail by design (slender lipped aluminium section, the pending EN 1998 building with Rayleigh and no stiffness); the other entities are sized to pass. Pass and fail utilisations are unverified until the first run.
- Not done: independent third-party numbers for the showcase rows (numpy and the oracle libraries are not installed here); the existing per-row oracle features are unchanged.

## 5. Files

Ticket: `🧰️scripts/showcase/*.py`, `📌️important/requests.md`, this report. Artifacts: the four `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, four model snapshot files, four editors and editor unit tests, the four trace test files, EN 1998 and EN 1994 scene tests plus EN 1993 and EN 1999 scene tests, EN 1994 empty scene golden, shared `⚖️compliance/🧭️spec-trace/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`.
