# Official Certification Independent Pilot

All ten unchanged official certification ADD2 inputs parsed successfully in the existing IfcOpenShell0.9.0.post1 interface and completed full EXPRESS validation with zero logger statements. No file was excluded, repaired, retagged or treated as an expected-failure whitelist. These observations do not establish ReferenceView conformance or acceptance by our own implementation.

## Provenance and Exact Inputs

Repository buildingSMART/Certification-datasets (Sample-Test-Files resolves to the same repository/commit), commit80d976a9b193a26a8e928c3e79bff67af1de68a8, original directory IFC4.3.2.0 (IFC4.3 ADD2)/Simple-Scene. Actual LICENSE at that commit attributes buildingSMART International Ltd. under Creative Commons Attribution4.0 International. Exact original paths/raw URLs/commit/SHA256 are retained in `🗑️generated/clipping-official-input-observations.json` and repeated in the certification outputs. Original source bytes were reused unchanged from the earlier primary-source research; no existing46-file corpus entry was replaced.

`certification-independent-inventory.py` records complete original FILE_SCHEMA, FILE_NAME and FILE_DESCRIPTION clauses in `🗑️generated/certification-independent-headers.json`. All10 exact schema identifiers are IFC4X3_ADD2; the external schema_identifier also records IFC4X3_ADD2. The single distinct description is FILE_DESCRIPTION(('ViewDefinition [ReferenceView]'),'2;1'); this is an original declaration, not an executable certificate or normative applicability decision.

| Original file | Bytes | Parsed entities | EXPRESS statements |
| --- | ---: | ---: | ---: |
| Building-Architecture.ifc | 141965 | 369 | 0 |
| Building-Hvac.ifc | 111304 | 152 | 0 |
| Building-Landscaping.ifc | 1073991 | 141 | 0 |
| Building-Structural.ifc | 185139 | 339 | 0 |
| Infra-Bridge.ifc | 1739554 | 902 | 0 |
| Infra-Electrical.ifc | 120043 | 214 | 0 |
| Infra-Landscaping.ifc | 2348837 | 1186 | 0 |
| Infra-Plumbing.ifc | 350835 | 443 | 0 |
| Infra-Rail.ifc | 161670 | 726 | 0 |
| Infra-Road.ifc | 263708 | 894 | 0 |

Total6497046bytes,5366parsedentities. Largest original is2348837bytes. The parsed per-type inventory includes220IfcTriangulatedFaceSet and220IfcCartesianPointList3D,359shape representations/product shapes,490local placements and484axis placements. Large bytes with relatively few entity records indicate substantial aggregate payloads: entity counts alone are an inadequate source-size or cooperative-work budget.

Lexical feature observations show no complex entity record syntax or Part21 double-quoted binary literals in this set. The output feature called typedValues is a broad lexical IFC-name-followed-by-parenthesis count which includes entity constructors; it is explicitly not a semantic typed-value count or parse guarantee. External parsed type rosters are the authoritative entity census here. No BooleanClippingResult/Clipping label was found in the prior exact token census.

## Executed Interface, Diagnostics and Performance

`certification-independent-oracle.py` defines a repository-owned CertificationOracle interface. Its existing external implementation opens original files, reads exact schema/header fields, inventories real entity types and invokes ifcopenshell.validate.validate(...,express_rules=True). Full observations, raw diagnostic arrays, byte hashes and timings are in `🗑️generated/certification-independent-observations.json`. Every diagnostics array is empty; therefore there are no omitted entity IDs, attribute/rule categories or raw error messages to normalize. No library instance/type is exported into production APIs and no dependency was installed.

Per-file parse observations range approximately0.004–0.08seconds; full validation observations approximately2.0–4.7seconds in this environment. These are actual external-oracle wall times, not our scheduler/receipt performance or deterministic thresholds. Each completed file prints a DEBUG progress line and saves the cumulative output immediately, allowing interruption between files without losing previous observations. This bounded pilot completed all10 and exited; no source mappings or processes remain live. It does not claim internal progress/cancellation control inside the third-party synchronous validator.

The original first header inventory regex was corrected to include complete clause terminators rather than stopping at the semicolon inside quoted2;1. Final exact header output comes from the corrected script. No source bytes or oracle outcome changed.

Native execution/type/physical receipt and full-view flags remain false. Actual repository endpoint parsing/validation must be compared separately against this independent baseline; discrepancies require investigation rather than fixture edits or silent exclusions.

