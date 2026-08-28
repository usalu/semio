# Whole Patch Owner R46 Native GREEN

Canonical UI contract selector `retained_pending_patch_` completed **2 passed, 140 skipped**, 0.163 seconds. New public `UiPendingPatch` owns the exact optional patch and a private bound typed cursor; no external cursor rebinding or retirement-page allocation. Its native inline size is **1,744 bytes** and must be admitted by callers separately from retained descendants.

API: `default()`, `source_mut() -> Result<&mut Option<UiPatch>, &'static str>` before closing, `get()`, `close_step(items, bytes)`, `terminal_is_empty()`. `retained_operation_bytes()` measures only actual operation-page backing, not complete resident ownership.

The first native law closes all eighteen concrete components in real patches at 1/64/4096 byte grants, verifies exact surface and payload semantic bytes, zero-grant stability, no growing page backing, and denied read/mutation after close begins. The second holds the owner structurally outside an injected panic at six retirement frontiers, then closes exact empty reserved page backing and verifies move handoff. Expected caught panic traces are present in the raw log.

This is not guest lifecycle integration or whole native renderer timing evidence. Dag received the exact API/native checkpoint for Kernel/reactor adoption; those sources remain in flight.

Actual output:

```text
15:[DEBUG] fixed-list-page-oracle checks=47
23:[DEBUG] pending-whole-patch components=18 close-grants=1,64,4096 exact-surface-bytes=4 no-close-allocation=true
128:[DEBUG] pending-whole-patch unwind-frontiers=6 empty-reserved-backing-retired=true exact-handoff=true owner-bytes=1744
135:     Summary [   0.163s] 2 tests run: 2 passed, 140 skipped
136:[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-0qHzKe
140: NX   Successfully ran target test for project @semio-tech/ui-contract-rs
```
