# Retained Member Publication Boundary

The previous member store owned snapshot and mutation retirement but no retained publication factory. `ChildEmit` owns encoded operation vectors. The former active typed publisher called `dispatch_emit_group` on a cloned child, performing whole-group preview, decode, apply, and log construction. That fallback is now removed; unsupported Child publication fails closed before any parent mutation is popped.

This packet adds exact member-owned typed and wire preparation factories, a retained erased publication with member-identity validation, group metadata sealed into the edit before digesting, and heterogeneous enum delegation. Tests use language-neutral JSON cases and the independent `serde_json` number decoder as a differential oracle. The production test grant remains one item and 4096 bytes.

The member preparation seam now reserves the real fixed history slot and displaced-owner capacity without publishing. Exact member identity is checked by downcast and shared lease-registry identity; generation/content revision is revalidated every preparation advance. Another append cannot consume the reserved history slot. Abort releases the real history reservation and retirement credits in separate turns before closing the preparation owner. The heterogeneous member enum forwards these authorities, including the test member.

Admission remains fail-closed until the composition layer owns retained group preparation, all-member freshness/reservation, kernel/invocation/undo log preparation, and child-content-root publication. Per-member move publication alone does not provide atomic multi-member commit and must not be advertised as a complete Child lane. The new tests prove ordered sequential member publication and prepublication reservation/abort isolation, not atomic multi-member plugin visibility or grouped undo.

The large-wire case is an 8194-byte scalar JSON ingress with whitespace padding, not a large semantic edit. The independent canonical-sealer packet owns semantic 16/64-KiB edit laws.

Compiler execution is coordinated by the root agent. On2026-08-27 the coordinator reported all three focused retained-member native tests passed (0.13seconds); the seven exact canonical-edit sealer tests also passed in their separate gate. No mounted atomic group, grouped undo/log, or Wasm pass is implied.
