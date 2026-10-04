# Unmounted Original Store Lifetime Probe Inputs

No native command was executed by this author. These are standalone authored ticket inputs, never mounted into production or a test cohort.

- Negative input: `🧫️assembly-lifetime-native-inputs/🧫️original-store-guard-escape.rs`. Current original API is expected to ACCEPT compilation and print the stated lifetime escape. This is RED against desired refusal; it is not expected to be a compiler error before the fix.
- Positive control: `🧫️assembly-lifetime-native-inputs/🧫️original-store-guard-order-control.rs`. Correct guard-before-barrier disposal remains valid.

The original barrier region and Store registry guard/error/acquisition region are copied byte-exact from the actual Store source. Only map value data types (`ArtifactCodec`, `DialectMigration`, `os_io::ArtifactDialect`), their adjacent unavailable error families and registry storage providers are explicitly stubbed. Actual mutex/write guards, poison mapping, returned guard lifetimes and acquisition function implementation are unchanged. No metadata/codec semantics, IO routing, whole Store compilation or product deletion is claimed.

Store whole-source SHA-256 `823f795f9f4eb30914d533202948c052821fedeb421539330bbe4269b21f7da7`.
Exact barrier slice SHA-256 `d06c2126b1a3050a52c9d8f7e30bf43ed0797984e99c52e9f6b8b441e5f6bcaa`.
Exact Store guard slice SHA-256 `49e9bb7e29255d6921e7a76b7ac058e8ca50d6999b6be9ec4b0480f0ffcee35f`.

Native queue owner may compile each with rustc --edition=2021 --crate-name assembly_lifetime_probe --emit=metadata or produce a binary exclusively under ticket generated/current-native-worker. Use current queue admission and preserve compiler diagnostics; do not overwrite the inputs. After the real API gains transaction-borrowed guards, use this same negative CLIENT block against the real current definitions and require compiler rejection at drop(transaction), while the positive client stays accepted. No corrected substitute implementation is supplied as production proof.
