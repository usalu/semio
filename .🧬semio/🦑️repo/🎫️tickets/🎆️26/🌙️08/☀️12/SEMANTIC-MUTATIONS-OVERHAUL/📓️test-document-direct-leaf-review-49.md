# Test Document Direct Leaf Review 49

## Boundary

Read-only root review during runtime's Plugin R5 source hold. No Plugin, Store, native test, codec, schema or controller source was changed. The earlier 40-check source receipt is retained, not treated as native acceptance.

Reviewed the document root, both direct leaves, aggregate, their descriptor, ticket controller/vector/descriptor copy, main TestSnapshot/TestDiff definitions, and actual DslRecord/DslOps/Mutations derive implementations.

## Confirmed Ownership And Codec Facts

SetCount and SetLabel own their payload, diff contribution, inverse and semantic metadata in direct folders. They assign rather than add, so their i32 count operation does not have an arithmetic overflow path. The aggregate wraps both leaf types and derives mutation delegation.

The current leaves intentionally have keywordless DslRecord specs. DslOps supplies the aggregate keywords from the variant names, and its newtype path delegates to each unchanged leaf spec. Consequently stripping and restoring the keyword in this particular aggregate is consistent with its current representation. The earlier builder fix for a keyword-bearing record must not be copied blindly here. A future leaf-authoritative keyword cutover must update leaf and generic framing together and preserve the actual wire contract.

## Findings To Resolve After The Source Hold

1. The aggregate's native test uses unqualified TestSnapshot but imports only its own parent module and protocol traits. TestSnapshot is reexported by the document parent, not this aggregate. The actual derive expansion does not import that name. This is a source-level likely compile error; the current R5 compiler outcome remains authoritative and has not yet been received.
2. The aggregate contains a mutation-specific test with concrete SetCount/SetLabel construction. The governing target permits only structural aggregate tests. Move behavior/codec/inverse laws to a directly mounted test facet with leaf-owned entry points; do not hide implementations behind generated source.
3. The neutral controller compiles a handcrafted ticket copy of the descriptor schema instead of the authoritative repository schema. It does not read aggregate, snapshot or diff JSON schemas; the packet currently supplies only payload schemas. Schema parity is therefore incomplete.
4. The native test uses two literals rather than consuming the neutral vector. Its assertions do not establish the declared exact text strings, i32 endpoints, malformed/truncated binary handling, no-op outcome, sparse diff identity/absorption or invalid aggregate serde behavior. The controller's custom text-prefix/Number/JSON.parse checks are not an actual DSL runtime oracle.
5. Controller inputs are parsed before the initial hash capture, paths depend on cwd, and reads lack nofollow checks. Main Plugin source is consumed for mount checks but omitted from the captured input list. It can therefore change without failing the final equality check.
6. Main TestSnapshot and TestDiff have existing permissive unknown-field serde behavior; both snapshot fields are required for nonempty decoding, while missing/null diff options are identity. Empty snapshot text/binary explicitly produces Default. Preserve and characterize these actual contracts rather than assuming strict serde attributes or changing unrelated clone counters/preparation behavior.

## Next Packet

Use one domain-owned, schema-validated fixture for native and reference tests. Validate actual aggregate/payload/snapshot/diff contracts and authoritative descriptors. Capture controller and every consumed main/leaf/derive/codec/schema/native input before evaluation, then reread all inputs with canonical nofollow. Mount behavior laws outside the transparent aggregate and invoke each concrete leaf's own law entry point. Preserve main snapshot clone/accounting, ordinary-command preparation, Interaction and lifecycle regions. Execute a source-first failing gate before cutover and use the serialized Plugin native slot for real codec/inverse acceptance.

No native result, codec failure or whole-owner completion is claimed by this review.
