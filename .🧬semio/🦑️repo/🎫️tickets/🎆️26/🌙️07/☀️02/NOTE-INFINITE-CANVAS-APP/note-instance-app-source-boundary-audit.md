# Note guest instance-app source boundary audit

Attempt 47 (driver `w2OTETc`) reported 18 files and 263,166 source bytes for `semio-plugin-instance-app`, which exceeded the 262,144-byte admission limit. The source observation also recorded a concurrent edit growing the composer API from 13,579 to 14,601 bytes. The captured production unit has no package features enabled. The composer API is 14,300 bytes in the current source snapshot. [Composer API](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🧬️composer-api/🦀️.rs)

The package and caller each set limits of 262,144 bytes, 32 files and 4,000 active lines. The admission gate takes the minimum of the two policies, so increasing the package value alone could not resolve the refusal. [Package boundary](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust/Cargo.toml#L13) [Repository policy](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json#L655) [Admission gate](../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts#L39)

| Current configuration | Files | Bytes | Active lines |
| --- | ---: | ---: | ---: |
| Note WASI guest | 18 | 259,465 | 3,079 |
| Native macOS test with `artifact-app-testing` | 18 | 259,465 | 3,136 |
| Admission limit | 32 | 262,144 | 4,000 |

The repository’s Rust source collector produced both measurements using a physical file reader. Python independently summed file sizes across the 18 mounted Rust sources and reproduced 259,465 bytes. The measured shared source is 2,679 bytes below the cap. No source or policy edits were needed. A full native build was outside this verification. [Repository source collector](../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts)
