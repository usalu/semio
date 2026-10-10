# Required Original JSON Grammar and Receiving Control

The existing required method is now public on `semio_framework_pack_json::JsonGrammarCursor<V>`:

```rust
pub fn step_source<S: JsonReadSource + ?Sized>(
    &mut self,
    input: &S,
    maximum_units: usize,
    control: &mut semio_framework_value::NativeDecodeControl<'_>,
    grant: RetainedCloneGrant,
    known_utf8: bool,
) -> Result<Option<V>, JsonError>;
```

No forwarding method or alternate scalar API was added. Existing JsonSourceCursor and str callers still use this same implementation. Original received octets must pass known_utf8=false. A true flag requires a genuinely already validated original str; it is not a shortcut for unchecked archive or command bytes.

The grammar stores no source reference. The original parent can establish its immutable pointer/extent and complete caller limits through JsonSourceCursor::new(original_source, members, limits), then consume into_grammar() and store only the original grammar. Each subsequent step supplies a fresh borrowed slice/view from the same unchanged original holder. normal_step_demands(input) quotes the next independent copy/capacity/release/depth frontier. normal_step_progress() reports actual native work for that step. The recipient must combine the actual child progress with its original parent grant/receipt even when step_source returns JsonError::Native with retained progress. Parent depth is quoted before mutation and deducted from the original supplied grant.

Original complete limits are JsonReadLimits { maximum_bytes, maximum_allocation_bytes, maximum_depth, maximum_items }. They come from the receiving operation's fixed original policy. They are neither derived from the next demand nor reset on each turn. The parent also retains the separately fixed full five-axis normal/cleanup authority; remaining/granted intersections preserve every independent axis.

A NativeDecodeControl passed to grammar is the same receiving capacity/cancellation owner. NativeDecodeControl::new_forwarded(original_maximum_bytes, callback, original_allocate_port) returns NativeForwardedDecodeControl. Before any retained decoding ownership exists, install_retirement_recipient(original_recipient) binds the actual empty NativeDecodeRetirementRecipient. Every native reservation invokes the original allocation port with NativeDecodeAllocation { bytes, owned_bytes, next_owned_bytes, maximum_bytes }. No accepted=true callback or scalar maximum manufactured from a step demand represents receiving authority.

The original forwarded control supports a lifetime-free handoff of its cumulative ledger:

- detach(self) -> Result<NativeForwardedDecodeReceipt, (ValueError, Self)> consumes the actual loan, retaining the original ceiling, owned-byte ledger and recipient identity.
- NativeForwardedDecodeControl::rebind(receipt, callback, original_allocate_port, same_original_recipient) -> Result<Self, (ValueError, NativeForwardedDecodeReceipt)> reborrows those exact original resources for the next hop.
- On every refusal, the returned original control/receipt remains owned. Recreating a new forwarded control would reset admission and is not equivalent.
- Recipient address must remain the same across detach/rebind. An owned holder that can move needs a genuinely funded stable native recipient allocation; an uncharged Box or self-reference is not an alternative.

NativeForwardedDecodeContinuation also supports explicit original-port/caller-recipient borrowing and decode(callback, operation) while that external authority lifetime remains valid. It stores actual references to external original allocation/retirement owners; do not place it into a parent whose owned recipient it would borrow self-referentially.

Grammar completion returns the actual original V. Receiving that V into the parent's real Option<V> must preserve its native storage and consume the actual grammar step progress; it must not materialize a second DslValue or clone. MutationOrigin construction from a generic owned JSON result still requires a granted projection: native key bodies, candidate pages, empty vector backing and unused fields cannot disappear through a cold from_value/from_json_str call.

Grammar closure must preserve its actual owner on denied retirement birth. semio_framework_value::admit_owned_retirement(original_grammar, original_nested_grant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Progress), (ValueError, original_grammar)> exposes that custody. Quote owned_retirement_birth_bytes::<JsonGrammarCursor<V>>() before taking the grammar, debit its actual birth progress, then advance/close the same retirement ticket under supplied full five-axis grants. The original result/error has a genuine retained recipient before grammar/output cleanup. A matching external NativeDecodeRetirementRecipient is progressed by NativeDecodeControl::close_retirement_recipient(grant); all actual child/error progress stays in the original parent StepContext.

The same rules apply to Artifact command OpBinary receiving: decoded bytes and their complete original policy are already held by the receiver. A cold OpBinary::decode_op remains incomplete. Borrow those exact bytes for every grammar/native codec turn, use the original control/recipient and exact receipts, and retain any partial typed owner on parse refusal.

This interface publication is integration work, not a new native acceptance claim. Full MutationOrigin, History and command receiving still require actual original runtime tests.

## Original Typed Origin JSON Receiver

History now exports RetainedHistoryOriginJsonDecode and RetainedHistoryOriginJsonStep. Its required admit(source, range, complete_JsonReadLimits, original_grant) stores only integer original pointer/extent and grammar state; complete limits come unchanged from the original receiving operation. demands(source, original_control, maximum_copy) and step(source, original_control, original_grant) receive a fresh borrow of that same holder on every turn. The supplied control maximum must equal the original complete allocation limit. Grammar normal progress and retained projection receipts remain in the original parent grant, with actual parent depth deducted; partial parse JsonError ownership is retained, not formatted into an unfunded error String.

When grammar produces its original owned DslValue, grammar_is_receivable becomes true. take_grammar_into(actual Option<JsonGrammarCursor<DslValue>>, original_grant) transfers that same original grammar under metadata copy0. The recipient then quotes/admit_owned_retirement/advances/closes the original grammar under its original full grant and receiving allocation control. Until this actual receiving slot accepts the grammar, typed projection is not advanced. No native grammar backing disappears in a cold Drop.

Subsequent step calls admit and advance RetainedHistoryOriginProjection from the original parsed slot. Its typed String recipients preserve their native storage; only the32 actual payload-hash bytes incur derived body writes. The normal wrapper forwards actual capacity demands to the same original cumulative NativeDecodeControl allocation port before each native projection/retirement birth. Ready typed receiving uses take_ready_into(actual Option<MutationOrigin>, original_grant). Its remaining empty projection then closes and is removed in separate paid turns.

cancel preserves the original grammar, parsed value, projected partial and parse error until their real recipients accept them: take_grammar_into, take_partial_value_into, take_projection_into and take_failure_into are required paid transfers into actual empty Option owners. Transferred projection cancellation returns RetainedHistoryOriginPartial into its real paid recipient; that partial exposes close_demands/close_step for actual key/nested-value/vector/body/child-frame release. Wrapper close_step cannot become terminal while any original receiving owner remains. Every receipt is consumed once before another operation; terminal Drop has no retained native owners.

Origin wrapper source RED/Green and syntax are actual evidence; its newly authored combined native original-control/grammar/typed-receiving law remains pending in Kernel l4143. These APIs are not yet a full History decoder or Archive/command integration claim.

## Direct Original Paged Wire Source

The required first-party protocol::codec::RetainedWireByteSource has original_source_identity(&self)->OriginalWireSourceIdentity and byte_at(&self,index:usize)->Option<u8>. OriginalWireSourceIdentity carries public address:usize, extent:usize and authority:Option<(u64,u64)>; actual Job payload views bind the original sealed payload header address, complete original length and original operation/generation. The trait lives in Protocol and has no Job dependency. All RetainedWireFieldDecode constructors, demands and advance now accept source:&S where S:RetainedWireByteSource+?Sized directly. Existing contiguous slices bind their actual pointer/length and no Job authority. new_range(source,(start,end),text) binds a field inside the same complete original owner; new_json_string reads quoted syntax across the same original pages. original_source_identity() returns the complete bound identity; outstanding receipt, real take_into/take_text_into receiving slots, admit_received/admit_received_text and close demands/step remain mandatory full-five-axis operations.

Source RED a5820 and GREEN b6080 executed through original managed Nx. Native c6370 passed the original paged fixed2 raw/JSON law, plain wire and UUID, with actual native heap receipts and cancellation. Timestamp/JSON standalone selector spellings in that invocation matched no additional tests, so its evidence is3/3,335skipped245ms. Dictionary currently remains contiguous and full binary record receiving is not complete. No gathered Vec or cold Tick/OpBinary parse is supplied by this API.

The origin JSON wrapper now keeps its complete original caller ceiling by scoping that same control around each direct grammar.step_source invocation. The grammar's lower per-turn allocation request maximum is the minimum of complete caller allowance and already admitted owned bytes plus the exact original capacity grant. That bound is not an alternate capacity supplier, and complete policy is restored after the original child turn. Source scope RED/GREEN executed; native wrapper is pending.
