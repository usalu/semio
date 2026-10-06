# Shared Pack Intrinsic Visitor Source Audit — 2026-10-05

Read-only current existing preflight owner/test/neutral JSON review. Runtime H::Value emits TAG_VALUE then intrinsic tags; number variants retain Int/UInt/F64 discrimination and exactfloat little-endian bytes. Borrowed projection views traverse arrays/objects in source occurrence order, object keys use projection_key and inline text tags, while symbol discovery visits values only. No runtime materialized owner or alternate allocator observed.

64slot path check and declared maxdepth guards precede descent; control.step/checkpoint flows remain, symbol spill allocation goes through NativeEncodeControl and existing checked capacity growth. Test-owned canonical materialization is an independent actual byte-length/decode oracle, not runtime preflight path. Neutral serde roundtrip checks nested null/bool/number/text/array/object types, canonical decoded wrapper tag, exact limits, paid spill and cancellation parity. Actual shared RED85281 UnsupportedOwner precedes candidate; shared41352 and print19 runtime gates remain required before claims.

Only explicit authorized preflight owner/test/fixture paths are attributed; complete Rust813context is source binding rather than ownership. All before authored inputs retained outside generated. No edits to product/tests/jobs performed here.
