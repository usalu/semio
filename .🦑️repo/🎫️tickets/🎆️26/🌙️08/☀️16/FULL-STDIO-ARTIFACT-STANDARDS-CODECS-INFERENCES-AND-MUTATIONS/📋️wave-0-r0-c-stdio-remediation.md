# 🧾 Wave 0 R0-C Stdio Remediation

## Completed Structural Work

- Added exactly 36 schema-owned `📜️artifact-definition.json` leaves and reduced the registry catalog to their paths.
- Validated definition version, exact fields, canonical identities, local references, dependency DAG, duplicate identities/dialects/MIMEs/extensions, EN+DE descriptors, EPW's empty MIME claims, and support-ledger evidence rules.
- Replaced singular representation claims with `mimes: []` and `extensions: []`; descriptor assembly and TypeScript ledger now derive every claim without a primary MIME or extension.
- Replaced the retired filesystem-derived roster and codec claim path. The ledger reports only schema data and preserves unimplemented support states.
- Changed all 26 executable roots to accept the one `ArtifactDefinition`, terminate with fallible `.try_build()`, and return the typed result. The 10 roots with no declaration remain definition-only. The stdio root iterates the 36 root assembly leaves once.
- Consolidated PDF into one declaration with plural 1.4/1.7 facets and removed its duplicate assembly/stale two-declaration explanation.
- Replaced the stdio VS Code catalog gate's inline `bun -e` command with `bun nx run workspace:stdio-quick`.

## Verified Checks

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts stdio ledger` | Passed: 36 artifacts, 35 registered MIME claims, 40 standards/profiles/dialects, 36 representations, 36 conformance suites. |
| `bun nx run workspace:stdio-quick` | Passed. |
| `bun nx run @semio-tech/stdio-plugin:test-quick` | Blocked before stdio: `semio-framework-os-kernel-dsl-derive` fails E0753 because its included component begins with inner `//!` comments. No framework file was changed. |

## Explicit P0 Follow-Up

Wave 0 does **not** close runtime capability support:

- The 26 preserved runtime declarations register schemas, document codecs, inference descriptors/services, composers, validators, and languages, but their source definitions do not yet enumerate and validate an exact definition-to-runtime mapping for every registration.
- Most schema leaves still contain empty codec/mutation/inference collections. GLTF has legacy entries, but their identities omit required version suffixes and its registry validator deliberately rejects every nonempty executable leaf pending typed executable mapping.
- Consequently the ledger's six codecs, 28 mutations, and 15 inferences are not support claims and must not be treated as verified or closed.
- Runtime, fuzz, and cross-platform gates intentionally fail explicitly until normative provenance, executable mapping, validators, and fixture evidence exist for every implemented claim.
- The current framework representation constructor accepts one optional MIME. Registry assembly uses one representation capability with all schema MIME claims; a future framework API can expose plural MIME input directly without altering schema leaves.
- The remaining unrelated `bun -e` launch entries are outside the stdio catalog entry changed here and remain a workspace-wide launch-configuration follow-up.

## Deferred GLTF Debt

GLTF's semantic mutation/inference/codec leaves were not edited. Its definition and artifact-root declaration are wired only. The missing generated planning/mutation integration and the capability identity/mapping correction remain program work, not a Wave 0 support claim.
