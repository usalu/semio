# Current OpBinary Paged Adoption Topology

Bounded current source audit; no production edits or executions. Machine locators in `📥️inputs/current-opbinary-authored-implementation-locators.json` exclude ticket/cache/generated/test/fixture directories. Inline cfg(test) sections and macro patterns are explicitly not a compiled membership census.

## Authority and Real Producer Families

The sole actual shared trait authority is Replication `🎮️mutation/🦀️.rs:1403`: encode_op returns Vec<u8>, decode_op takes &[u8]. Current trait has no caller-supplied options, output, control or paged source entrypoint. Its doc mentions DslOps derive, but no current DslOps proc-macro entry was found in the actual first-party derive crates; do not treat that historical doc as a producer authority.

The real shared generated producer authority is Plugin root `app_commands!` at13558 and `view_commands!` at13790, with OpBinary emission around13625/13699/13774 and subsequent view branches. These generate delegates to OS DSL variants_binary. Transient window macro at507 likewise generates implementations. Their invocations emit real command types; changing generated cache text would not repair the actual source generator.

OS DSL `variants_binary` at186–257 is the central handwritten variants route. encode_with lowers a named record, builds the ordinary schema, encodes a full Record-body Vec with default options, then allocates a second complete operation Vec and copies it after format1/ordinal-or-protocol-tag header. decode_with takes a slice, decodes a terminal Record and canonically reencodes a complete operation Vec for byte equality. The paged join must preserve exact ordinal versus tagged selection, unknown-field refusal and canonical byte comparison while eliminating this reencoding buffer or admitting its ownership explicitly.

Concrete framework delegates include WorkflowMutation/root1685, RunMutation/mutations63, FlowMutation/mutations79, DagMutation/vcs554, SpaceMutation/root219, CollectionMutation/root353 and BackboneMessage/Store24630. Store OpsHeaderLine12647 and CommandHeaderLine14306 independently implement the same format/tag/Record framing and therefore bypass the central helper; they require their own coherent join. Print ChangeChartValue codec7 has exact literal header `[1,1]` followed by canonical Record, and needs its own same-wire producer/source pair.

SpaceHistoryMutation/Store26532 is a distinct exception: it encodes an intrinsic Pack value directly and decodes with whole-number renormalization. A mechanical format/tag/Record replacement changes its authored wire semantics. OS DSL op_rt at323–366 is another actual family: ToValue payload/tagging lowering, intrinsic encode_wire_value plus second whole operation Vec, then inverse tagging reconstruction. Plugin handwritten artifact aggregate implementations can select this route; inspect each locator's actual delegate rather than assuming every OpBinary is DslVariants.

## Concrete Caller Joins

ChildEmit::push at Plugin13265 still calls Vec-returning OpBinary::encode_op and pushes that Vec into the child prefix. Mounted child preparation43 installs an append closure that calls this exact method. Thus typed retirement ordering does not yet eliminate contiguous encoded payload storage. New paged production must append exact header/body under the caller's real options/control and keep accepted bytes plus the refusing current operation owned until retirement/handoff; a default encode-to-Vec adapter cannot substantiate paged adoption.

Plugin prepared_ops at9110 calls encode_op(...).expect and retains complete byte Vecs. The same registered worker/contributed publication family must propagate typed refusal rather than panic or drop the accepted prefix. Independently inspect cfg ownership of each inline root match before promoting it to production membership.

Replication causal encode_ops_vec1204 creates one contiguous envelope from &[Vec<u8>], while decode_ops_vec1214 reconstructs Vec<Vec<u8>> with copied payloads. Store apply/wire at11661–11664 feeds these copied payloads into Mutation::decode_op. Store13352 encodes an operation to a binary Vec; decode_op13610–13612 accepts OpPayload binary slices; replacement14242 again encodes a complete Vec. These are real wire seams, not repaired by merely adding a paged Record API.

The output and source joins therefore need three coherent stages: shared required OpBinary paged authority plus actual generator/delegate implementations; actual retained ChildEmit/publication owner storage and prefix restoration; outer causal/history framing and canonical source comparison. Preserve actual wire order and per-operation framing throughout. Existing ordinary slice APIs may remain separate during a held implementation interval, but a new API that internally calls them cannot receive paged physical credit.

## Execution Qualification

Reported Core streamed Record owner1/1 and Replication paged floor/reader laws establish their selected primitives, not these OpBinary producers or consumer publication. Full Core regressions and ChildEmit adoption are separate pending assertions. This topology identifies exact missing joins without asserting that their current ordinary contiguous behavior is itself malformed.

## Held Central DSL Producer Readback

Current input operation-byte-pages/dsl-streamed-variants.rs uses genuine to_named_record_controlled, variants_controlled and schema producer.encode with the supplied NativeEncodeControl, stack-only format/varint header and direct Core Record sink. The external caller owns every accepted header/body byte across refusal; no copied payload Vec adapter is introduced. Controlled projection still creates a Record owner as expected, so its scratch ownership/retirement is distinct from paged payload backing.

One concrete held API gap sent to Immutable: new into entrypoints accept no EncodeOptions and call EncodeOptions::default. This matches the old ordinary route's default convention, but cannot propagate a prospective caller's actual max_file_len/depth/unknown policy. The new caller-owned producer must carry the actual supplied options alongside control if it claims that policy. Tagged and ordinal paths remain distinct and their exact format/tag order must stay pinned.

Immutable reports full current Core owning131/131 separately; that receipt does not compile or execute this held central DSL producer. Its proposed immutable ByteSpan comparison sink can avoid canonical reencode Vec, but actual source comparator/control completion demands remain required.
