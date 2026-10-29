# Store Metadata Read Review

2026-10-28. Read-only exact git diff and source review; no compiler, test or graph execution. Inspected both requested files plus the actual StoreCursor/StoreLog method implementations and branch Cargo dependency.

## Result

No actionable correctness defect was found in the three metadata delegates or the branch import correction. The split retains generation, content_revision and local_actor_id under precisely the same generic bounds required by their underlying owners: P:ToValue+Send+Sync+static and Mutation:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+static. No Clone, P:FromValue or ColdMutation bound is needed by these three operations. Delegation bodies are unchanged.

StoreCursor generation/content_revision still read through StoreWire.durable_group_read_root before falling back to the cursor fields. Therefore the split preserves coherent published root selection and does not bypass durable-group visibility. StoreLog.local_actor_id still returns a shared reference to the stored actor ID. Each public delegate takes only &self; none exposes mutation authority or moves write methods into the weaker block.

The remaining original impl now adds P:Clone+FromValue and Mutation:Clone+ColdMutation<P>. This strengthens its admission rather than relaxing write capability. Existing method-level where clauses remain; the diff removes only the three delegates from that impl, surrounding region markers, and adds the weaker dedicated impl. It does not change operation bodies, signatures, return types, error propagation or publication semantics. A source-level limitation is that the stronger bound now applies to every remaining delegate, including other pure reads such as clock_now/generation_now/envelope. That may require future separation if a real generic consumer needs those methods; no such failure is established by this review and broad speculative refactoring is unwarranted.

The branch source import changes from crate::lifted_store to semio_framework_os_store_engine::lifted_store. Its package is semio-framework-os-store-engine-branch and already depends directly on semio-framework-os-store-engine; that dependency publicly exports lifted_store. Thus the new import addresses the actual defining crate. Branch planning retains its original P/Mutation bounds and obtains only the same three shared metadata reads plus existing parts_ref. No branch planning behavior or fences change in this diff.

## Paths and Limits

The accessors source absolute path is138characters, branch source is97characters, and branch Cargo manifest is120characters, all below256. Underlying cursor source is150characters. This source review does not claim that the native37E0599 errors have cleared; that requires the owning compiler check.
