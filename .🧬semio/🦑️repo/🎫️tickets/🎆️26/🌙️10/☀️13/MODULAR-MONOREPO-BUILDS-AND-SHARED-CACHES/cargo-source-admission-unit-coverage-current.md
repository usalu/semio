# Cargo Source Admission Unit Coverage

2026-10-16. Source/neutral execution checkpoint. No Cargo, rustc, image, Native allocation, cache deletion, or SDK operation ran.

The actual ordinary Cargo Source admission gate deduplicated using packageId/entry/profileDigest/operation/platform/features. The key excluded compiler flags/arguments and kind/roles. More directly, its `continue` skipped assigning the second unit's `sourceInputs`, `compilerCfg`, and any captured compiler environment references, even for truly equivalent units. A real graph can retain distinct variant identities for equivalent raw Cargo units; the existing parser already models them explicitly.

Removed that incomplete skip. Every actual selected workspace unit now receives its own Source/cfg admission and bounded closure facts. The existing per-unit byte/file/active-line ceilings remain unchanged. This does not change Nx artifact keys, dependents, profile interpretation, storage or compiler dispatch. Future cfg reuse requires the exact retained compiler reader and observation identity; it must still populate every unit and must not share by incomplete package keys.

The new language-neutral corpus covers equivalent variants and cfg-distinct variants sharing the old key. It uses the real Cargo graph parser and genuine received Original/issued control. The Source admission interface supplies finite declared Source DATA and cfg observations, with no fake private storage/receiver/registrar. Independent Ajv validates the resulting full actual graph, and SQLite provides each expected unit's cfg and Source byte count. Actual filesystem/compiler effects are outside this proof.

The genuine RED expected two cfg admissions and observed one. BOTH successor runs passed1test/14assertions; owned strict BOTH0. All Root handles are terminal. Existing test/types-owned commands have an internal selector; no new human command or editor row was introduced.

## Evidence

| File | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| [cargo-admission-duplicate-red1-prototype.log](🗑️generated/cargo-admission-duplicate-red1-prototype.log) | genuine RED:0pass/1fail/2asserts; two units expected, only one cfg admission | 3477 | `12ae9c6f017606d3307ebded695eba1ce531853fb5a72841b0ace40c86a63db8` |
| [cargo-admission-duplicate-green1-prototype.log](🗑️generated/cargo-admission-duplicate-green1-prototype.log) | exit0;1test/14asserts | 1015 | `42714afb2d8f15ac89bbd9e963995094204936cac27a3b5b5decaa33e0686c05` |
| [cargo-admission-duplicate-green1-release.log](🗑️generated/cargo-admission-duplicate-green1-release.log) | exit0;1test/14asserts | 1015 | `60cd8136ae423017801f7f0a42be7334e4ac89ddb29e358453f51b388304629c` |
| [cargo-admission-duplicate-types1-prototype.log](🗑️generated/cargo-admission-duplicate-types1-prototype.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [cargo-admission-duplicate-types1-release.log](🗑️generated/cargo-admission-duplicate-types1-release.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

| [cargo-admission-duplicate-green2-prototype.log](🗑️generated/cargo-admission-duplicate-green2-prototype.log) | exit0;1test/14asserts | 1015 | `99946aaa313fe176fc19a50892a1751035e0a0f1b530fd16a4fb3a2d76e87515` |
| [cargo-admission-duplicate-types2-prototype.log](🗑️generated/cargo-admission-duplicate-types2-prototype.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [cargo-admission-duplicate-green2-release.log](🗑️generated/cargo-admission-duplicate-green2-release.log) | exit0;1test/14asserts | 1015 | `10b766c483cfb86f9701a4aac7fa938e2bb62124f0810f3bd89f2ab2fa2abfc2` |
| [cargo-admission-duplicate-types2-release.log](🗑️generated/cargo-admission-duplicate-types2-release.log) | strict exit0 | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The final test successor compares interface paths with the real parser's platform-normalized unit entry instead of a Unix fixture literal. BOTH1/14 and strictBOTH0 were recaptured. Only macOS execution is claimed.

## Source Roster

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` — `20a3793e6b5ba079646f0fdf94490bd3a5e056fa5ea0e98dc90ae3649eaa32a2`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🧪️tests/🟦️.ts` — `432a5c7a88df078f85019c6f81019dfe0d61b40827b4a00148775e8efd326385`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🧪️tests/🔣️.json` — `682b82fc4e2ae45fa165526dd1ee65773cb178c9251c3463bb9156df2f2e1617`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` — `4c939d1187e748279d51553473b579c744d7a851bb8b5582dc37de04623a23bb`
