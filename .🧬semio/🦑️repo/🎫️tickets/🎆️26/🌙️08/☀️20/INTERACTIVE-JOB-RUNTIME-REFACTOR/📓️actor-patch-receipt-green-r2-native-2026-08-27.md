# Actor Patch Receipt — Native GREEN R2

Canonical Actor selector `actor_ui_patch_receipt_`: actual 3 passed, 0 failed, 102 skipped; nextest 0.043s, exit 0. R1 was compile RED before the exact type and outer field existed.

```text
actor_ui_patch_receipt_matches_shared_wire_and_json_oracles ... ok
actor_ui_patch_receipt_outer_field_preserves_order_and_rejects_partial_publication ... ok
actor_ui_patch_receipt_rejects_malformed_and_unpaired_authority_before_writing ... ok
Summary [0.043s] 3 tests run: 3 passed, 102 skipped
NX Successfully ran target test for project @semio-tech/framework-actor-rs
```

Raw output: `🧪️member-actor-patch-receipt-green-r2-native-2026-08-27.txt`. This validates the Actor codec/outer envelope only; Kernel/WIT/plugin guest integration remains separately owned and unverified at this snapshot.
