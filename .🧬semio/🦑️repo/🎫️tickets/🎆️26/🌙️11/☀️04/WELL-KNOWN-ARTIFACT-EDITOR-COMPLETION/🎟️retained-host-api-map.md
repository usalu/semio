# Retained Command Fixture API Map

The removed admit_command_wire helper should not be restored. Current DispatchCtx::drive_runtime_typed_command (instance/dispatch/staging/runtime-command16) retains OriginalTypedCommandSource, produces runtime command once, admits the exact wire from committed declared catalog, then delegates to dispatch_typed_command_inner. The latter is DispatchCtx typed-command42, not a VcsArtifactApp method; parameters are mutable source, optional tool tag, optional ActionMeta, identity and actual StepContext. It returns optional InvocationResult while admission/receipt need further funded turns.

Latest-wins advancement is TypedCtx::advance_latest_wins_command_one(retained,cx), publication/inline-interaction46. Use app.typed_ctx(), preserve original context. tools.test_tool_clock is cfg(test) only in the jobs component and unavailable to a companion crate; current clock authority belongs to StepContext. Do not widen feature policy simply to restore a removed test hook.

OriginalFixtureScope in testing/caller source52 provides a single authentic caller context via cx54 across async fixture lifetime; it retires decoder/source using remaining original finance. Its neutral fixture clock is Some1000. For authored variable clock, OriginalWorkFixture::with_source in caller/work11 accepts clock, stock, deadline, fuel, operation and generation; async steps must resolve while the context loan remains in the turn. Existing bounded EntityIdentityAuthority construction in part3/typed-operation-slots8–11 is valid.

Registry cloning is no longer available. A per-case async registry factory creates fresh registration authority; retained-keyed-dispatch has only two source callers, part3/typed-operation-slots12 and108, both currently passing keyed_test_registry().await. Accept a factory closure/future and pass keyed_test_registry itself, invoking per case. Do not duplicate or shallow-clone authority. Current authored plugin Rust search has no test_retained_host_paging symbol.

No edits or build executed.

Gate contention uses RetainedOwnerGate::try_lock, returning a Deref/DerefMut guard whose Drop releases occupancy; fairness cancellation_state.lock is stale and should become try_lock while retaining the guard through the contention assertion. NativeDecodeControl must be lent from the same actual context via with_native_receiving; existing FixtureJob.receive568 provides that wrapper. PresenceStore.advance_publish_one now takes publication, grant, receiving. A newly constructed independent decoder would lose the caller-recipient provenance.
