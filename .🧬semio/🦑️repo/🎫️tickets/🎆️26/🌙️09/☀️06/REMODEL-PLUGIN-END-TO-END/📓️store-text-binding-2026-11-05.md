# Store Text IO Binding Audit — 2026-11-05

The current Store text IO owner `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📦️packages/🦀️rust/Cargo.toml` declares `semio-os-store-text-io`. Both @iarna/toml and smol-toml agree on it and the member-open manifest. The member-open dev-dependency path resolves to this owner, but had key `semio-framework-os-store-text-io` and no explicit package alias: the focused helper failed against the actual owner. Its key was corrected to `semio-os-store-text-io`; the same helper passed. The member-open genesis tests already use the canonical Rust crate name `semio_os_store_text_io`.

A bounded tracked-source search of OS and Remodel found eight old Rust crate references in the actively mounted Store facade `🏪️store/🦀️.rs` and test prelude `🏪️store/🧪️testing/🧭️prelude/🦀️.rs`. Their respective manifests already declare the canonical dependency. Added source assertions failed, then passed after replacing only these eight crate paths with `semio_os_store_text_io`. The same bounded search returns no old package or crate identifier. No alias or compatibility layer was introduced.

Retained input helper: `🧪️store-text-binding.mjs`. This is a focused manifest and source-binding check, not a Cargo compiler or runtime test. No native gate was launched; shared Windows spawn failure and capacity coordination remain with the parent.
