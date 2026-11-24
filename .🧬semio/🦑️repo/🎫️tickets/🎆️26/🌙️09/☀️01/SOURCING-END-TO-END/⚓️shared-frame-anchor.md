# Shared frame anchor

Symptom: wasm trap `a shared receiving frame is anchored before use` in `setContributions` and `setActiveExample`.

Root cause: the 01:10 wasm build predates the 01:31 derive edit. In that build the template seeding path (`write_seed_field`, used by `receiver_template`) wrote record slots without anchoring them. Only `receiver()` called `anchor_frames`. List rows and nested slots are seeded from templates, so their first deref of `SharedFrame` hit `anchored == false`. The current derive anchors every seeded slot and default slot to `state.frame`, `share_reset` re-anchors, and `receiver()` calls `anchor_frames`. A native run of the current tree did not reproduce the trap.

Fix and follow-ups in this ticket:
- Leaf-only records own no slot, so their `frame` is now `()`. This brings `native-record-ordinal-leaf` back under its size bound (14064 bytes before).
- The `struct-seed.json` fixture counts the new `frame` seed part (fixedParts 18, Row parts 20).
- New law `a_template_seeded_record_receiver_anchors_its_shared_frames_and_survives_a_move` in the native-receiving-original tests: part-by-part template seeding, a move of the owner, then a full receive, publish and close.

Results: native-receiving-original 10/0, native-owned all suites ok, curation lib 99/0 before the frame-type edit. The rerun after it was blocked by an unrelated peer edit (`semio-plugin-decl-natural-file-media` dependency). A wasm rebuild is needed.
