# Owned Object Transfer

Current Object storage owns Vec<(String, Value)>; into_entries consumes and returns the actual backing vector. Flow WASM dependency ordering consumes that vector before sorting without key or descendant clones. The independent-serde/pointer ownership law is mounted but Native remains pending.

## Current focused Native verification

Coordinator-reported fresh uncached `@semio-tech/framework-pack-json-rs:test-native` selected `object_ownership_transfers_order_and_descendants_without_copies`: **1/1 passed**, 11 ms assertions, 47 ordinary laws outside the selector; Nextest `154c7755-aeb2-4be9-9324-951593ae5fef`, Nx 5.8 seconds. Evidence: `🗑️generated/root-authentic-pack-json-object1-current.log`. This proves the actual consuming member method’s insertion order, stable key/string pointers, and independent serde semantics. It does not prove the whole Pack JSON package or Flow’s full Native suite.
