# W3 Verify Follow-up

## Subagent deliveries acknowledged
- CAD any ([Migrate CAD any reference](a60bd9dd-4c59-44aa-92d2-89792ab0d395)): structure + harness wiring — see `📓️w3-cad.md`
- EN1990 any ([Migrate EN1990 reference](bdcaca77-1168-4279-a5d2-23161380e7c9)): structure + harness — see `📓️w3-en1990.md`
- CSV/TIFF/XML ([Migrate CSV TIFF XML refs](0b7421f7-218f-4e3d-82c8-c252e4629411)): structure + xml valid TS — see `📓️w3-csv.md` / `w3-tiff.md` / `w3-xml.md`

## Coordinator unblock work
1. Plugin `E0499` already fixed earlier (`child_ptrs` in `dispatch_emit_group`).
2. Restored missing semio `📎️note` DSL/pack fixtures (lost during example moves).
3. Retargeted csv/tiff/xml artifact `io_registry` imports to `subsets::any::engine::io_registry`.
4. Restored accidentally relocated `🖍️draw/🔄️fsm` packages (engine batch move swallowed them).
5. Freed disk (ticket/peer `🎯️target*` dirs) after ENOSPC during verify.
6. Reexported `NoChildren` + `ArtifactChildren` from `app` at plugin crate root so `$crate::NoChildren` in `derive_artifact_facets!` resolves (UCAS nesting regression).
7. Kernel harness recheck passed: `assert_subset_harness` ok.

## Still open
- Runtime green for the seven refs (stdio check in progress after NoChildren fix).
- DOCX + semio mesh reports not yet present.
- EN1990 package tests previously hit unrelated `SetSnapshot` gaps on sibling EN199x apps.

## Verify2 results

- exits: `{'CAD_EXIT': 0, 'CSV_EXIT': 101, 'TIFF_EXIT': 0, 'XML_EXIT': 0}`
- test results: `['ok. 1 passed; 0 failed; 0 ignored; 0 measured; 139 filtered out; finished in 0.02s', 'FAILED. 38 passed; 1 failed; 1 ignored; 0 measured; 2034 filtered out; finished in 0.04s', 'ok. 50 passed; 0 failed; 1 ignored; 0 measured; 2023 filtered out; finished in 0.04s', 'ok. 45 passed; 0 failed; 0 ignored; 0 measured; 2029 filtered out; finished in 0.03s']`
- log: `scratch-w3-refs-verify2.txt`

## Verify green matrix (coordinator)

| Ref | Roundtrip test | Result |
|-----|----------------|--------|
| CAD any | `demo_subset_integrated_roundtrip` | PASS |
| EN1990 any | `high_consequence_office_subset_roundtrip` | PASS |
| CSV any | `demo_subset_integrated_roundtrip` + `inference_default_law` (exact) | PASS |
| TIFF any | `demo_subset_integrated_roundtrip` | PASS |
| XML | `xml::` filter suite | PASS |
| DOCX any | `demo_subset_integrated_roundtrip` | PASS |
| Semio mesh | `cube_subset_integrated_roundtrip` | PASS |

Also fixed: `CsvOutline::Default.has_header = true` to match `CsvSnapshot::default()`.
