# Wave 0 R1-C Stdio Remediation

## Result

The 36 schema-owned stdio artifact definitions now separate descriptive leaves from executable runtime registrations. There are 26 runtime declarations and 10 definition-only artifacts. Runtime rows are explicit schema data; the registry no longer synthesizes capability claims from definitions.

The typed declaration census is retained verbatim in [the temporary inventory log](📋️wave-0-r1-c-stdio-runtime-inventory.log). It recorded 284 rows emitted by the public typed builder inventory and 282 deduplicated requirements. The temporary declaration hooks used to obtain that inventory have been removed.

## Populated Runtime Capability Rows

| Category | Rows |
| --- | ---: |
| schema | 27 |
| inference | 27 |
| codec | 27 |
| composer | 54 |
| grammar | 120 |
| subset-validator | 27 |
| representation | 26 |
| **Total** | **308** |

The 282 non-format rows equal the deduplicated typed census. Each of the 26 active declarations owns exactly one explicit runtime representation row, while the 10 definition-only artifacts own none.

Every row has a canonical category-specific vN identity, a nonempty immutable schema-owned descriptor, and a complete claim set. The registry validates categories and exact claims, builds declarations by artifact key rather than positional pairing, and resolves runtime formats from explicit representation rows only.

## Representation And Standard Coverage

IFC4, DWG AC1024, PDF 1.7, and GIF89a each have their own declared standard and representation leaf. Repeated MIME/extension values are descriptive metadata, not multiple runtime registrations: only one explicit runtime representation capability may own a same-artifact claim set. The remaining formatter resolution is deterministic by representation identity and does not use the neutral flag as a filter.

## GLTF Capability Hygiene

GLTF now declares six canonical standard-owned codec leaves and 18 canonical, semantic mutation leaves. The forbidden generic NoMutation, SetSnapshot, and generic Set-star capability claims are absent. Its 15 inference leaves are also canonical vN leaves.

The capability ledger remains honest:

| Leaf Category | Declared | Registered | Implemented | Verified |
| --- | ---: | ---: | ---: | ---: |
| codec | 6 | 0 | 0 | 0 |
| mutation | 18 | 0 | 0 | 0 |
| inference | 15 | 0 | 0 | 0 |

GLTF cold inference was migrated to the request-aware inference SDK directly, without a payload-only fallback.

## Permanent Parity Coverage

The registry test schema_runtime_capabilities_exactly_match_registered_declarations compares the complete keyed category-and-claim set from each schema to the declaration's public typed requirements. It rejects both missing and extra schema rows. It also rejects a runtime-capability collection for every definition-only artifact.

## Verification

- PASS: bun ./📜️script.ts stdio quick — 36 artifacts, 40 dialects, 6 codecs.
- PASS: every stdio artifact-definition JSON document parses with jq empty.
- PASS: no temporary [DEBUG] runtime-capability census hook remains.
- BLOCKED externally: bun nx run @semio-tech/stdio-plugin:test-quick reached framework-plugin compilation but could not compile unrelated framework source:
  - E0063: AppDefinition initializer lacks dialect and role at framework plugin component line 4665.
  - E0004: AppCommand match lacks OpenArtifact, SetDefaultApp, and ClearDefaultApp at framework plugin component line 12957.

No framework files were edited by R1-C, and Cargo was not restarted after the blocker was identified.

## Remaining Stdio Gaps

The runtime parity and GLTF request-aware inference tests are unverified until the external framework-plugin compile errors are repaired. Capability support remains intentionally unimplemented and unverified where the schema ledger says so; this work does not claim format or standard conformance.
