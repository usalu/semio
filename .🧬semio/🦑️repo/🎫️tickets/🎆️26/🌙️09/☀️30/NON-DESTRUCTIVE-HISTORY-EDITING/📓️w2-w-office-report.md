# 📓️ W2-W-office — wire-witness conversion (xlsx, pptx, zip, binary, deflate) + presentation set-snapshot order

Status: IN PROGRESS (interim checkpoint, rewritten at the end).

## Progress log

- Adapter surface: every in-scope artifact crate root re-exports `protocol::json::{from_json_str, to_json_string}` and
  `protocol::{DslValue, Mutation}` — the generated test host links the artifact crate alone, so this is how an adapter
  reaches `Mutation::from_payload_value` and `Mutation::inverse` (the law under test) without a Cargo edit.
- binary: feature rows → wire (`remove_len`, snapshot `schema`), no-mutation scenarios removed, adapter decodes through
  `from_payload_value` and inverts through `Mutation::inverse`, oracle reads wire + `oracle_round_trip`. Lint 0,
  `parity exhaustive` 18/18 (subject only, no-oracle feature).
- deflate: same conversion; oracle `oracle_inverse_spec(base, forward)` + `oracle_round_trip`. Lint 0, parity 18/18,
  comparisons 9/9.
- Next: zip base + iso21320, presentation set-snapshot fix, xlsx/pptx (conformance + base).
