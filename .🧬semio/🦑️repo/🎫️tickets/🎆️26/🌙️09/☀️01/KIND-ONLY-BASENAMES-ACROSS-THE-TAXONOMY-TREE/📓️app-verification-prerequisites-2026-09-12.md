# App Verification Prerequisites

Date: 2026-09-12

## Executed Current Controls

The coordinator read the actual Mathematical, GIS and VCS command bodies and invoked their existing read-only source/fixture verification paths. No product source, fixture, schema, native build, catalog activation or final artifact changed.

| Current path | Invocation | Observed result |
| --- | --- | --- |
| Mathematical TypeScript package script | test | Failed strict Ajv before production oracle or the two example tests: routes minItems requires 7, fixture has 6. |
| VCS Rust package script | native-codec-check --oracle-only | Passed: 1 receipt, 9 hostile cases, Ajv plus Node/WebCrypto protocol hashes and 3 dependency checks. |
| VCS Rust package script | native-openable-identity-check --oracle-only | Passed: 1 positive, 11 denied hostile identities, WebCrypto/Node protocol identity. |
| GIS Rust package source | proveGisNativeCodecReceipts | Passed: 2 receipts, 8 hostile cases and independent protocol hashes; 124.1 ms. |
| GIS Rust package source | proveGisControlledProposal | Passed: literal/bounds, 3 interruption and 7 rejection controls; 14.3 ms. |
| GIS Rust package source | proveGisMapCreateRegionGroup | Failed strict Ajv: unknown keyword x-semio-child-kind; 15.4 ms. |
| GIS Rust package source | proveGisDurableThreeStoreAssembly | Passed: 3 roles, 4 cancellation and 3 rejection controls, Ajv/Node/WebCrypto; 15.1 ms. |
| GIS Rust package source | proveGisComponentColdMapPatch | Passed: 5 hostile cases, 9 source markers and fixture hash oracles; 12.3 ms. |

The GIS functions were imported and invoked individually so one failed schema compile did not hide the remaining controls. These are source/fixture oracles. No Rust laws, fresh native component production, browser behavior, WAL recovery, Hub publication or EnergyPlus simulation were executed here. Registered Nx routes were not rerun in this preflight.

## Concrete Follow-Up for the App Extraction Lane

The Mathematical fixture contains exactly six current action IDs. The current production EQUATION_TOOL_IDS, six publication contracts and six action_interactive_job registrations agree with those IDs. The schema alone still requires a minimum of seven rows. There is a second actual discrepancy beyond that first error: production nodeGraphViewport now uses ArtifactToolPublicationLane::WindowConfig, whereas the current script's extraction regex admits only Artifact or Config and its hostile removal string names Config. Thus merely lowering minItems would not make the real production oracle sound.

During extraction, inspect the current schema/production contract and keep the verifier and its hostile controls aligned with the real declared lane identity. Do not invent a seventh action, add compatibility vocabulary or loosen strict checking to hide this mismatch. Validate any mutation changes the intended current production construct before claiming its hostile rejection proves the property. Current data may still be concurrently edited; source hashes below are provenance only.

The GIS map schema has three x-semio-child-kind annotations. Its local compileGisScopeExport registers x-semio-formats, x-semio-state and double but not that annotation. The extracted shared verification schema concern needs the actual annotation contract and strict Ajv test registration rather than disabling strict mode. The four other GIS controls completed and remain independently useful prerequisites.

## Energy Native Python Prerequisite

The existing uv-managed Python environment and pinned OpenStudio 3.11.0 darwin-arm64 tree are locally present, including OpenStudio, EnergyPlus and its IDD. Root read the selftest path first: it binds that local toolchain, builds ten Honeybee native cases and translates one semio model; it does not invoke an EnergyPlus simulation.

The unchanged mandatory oracle script's test command then passed in 376 ms with UV_OFFLINE=1, Python bytecode disabled, and uv/temp/test outputs redirected to the current ticket's generated coordinator child. Native output reports 11 checks and zero failures. All ten cases have 129.60 cubic metres and 12 square metres of glazing, with the intended free-float versus HVAC distinction; the translated case has one room, six faces and the same geometry checks. No archive download/extraction, environment synchronization, physics simulation or live result publication ran. Existence of the prepared toolchain is not a fresh archive checksum proof.

Raw availability and run evidence is in coordinator/energy-prerequisite-availability.json and energy-selftest-prerequisite.log. The executable/module selector remains the queued implementation ownership work; the Python source is currently under the package's 🔮️oracles/🐍️.py path.

## Source Provenance

| Source | SHA-256 |
| --- | --- |
| Mathematical package script | de8a98cc6a049aa9f797015cc82aa6974795106edff9ceec47f18aa7f523e9c1 |
| Mathematical publication fixture | 2e4c8c75b62af3a65fb489be39479146ac95fbc29da07e64bb4ccdd8f494255a |
| Mathematical scope schema | fa832ede4f36c47e30b547b33ad840f17f4a361bfb6be311f2c937952d6c9c9d |
| Mathematical equation editor Rust owner | 898adb812845359b17091e9fc65be67fb976fe51e7cefd2e741019ba603cb79b |
| VCS package script | 269d17151f130bad45a848254682c77b2162b37e27e9e5db551ecbfc477acc0d |
| GIS package script | 0b211477c744ee3bf9d8e470cecff5f1de8bffb2da6ee7ab6df1da2235c38213 |
| GIS map subset schema | 8454bd7bea64e08244e39fa4ac7d91e3339e7995dd9ecf08161ddd1dace9d4e8 |

Full exact paths and byte sizes are captured in 🗑️generated/coordinator/app-verification-prerequisite-sources.json; retained exact source paths are already in 📓️app-verification-script-extraction-packet-2026-09-12.md. Four raw command logs are retained in the coordinator generated folder until ticket cleanup. This report does not attribute either current failure to a particular author or change.
