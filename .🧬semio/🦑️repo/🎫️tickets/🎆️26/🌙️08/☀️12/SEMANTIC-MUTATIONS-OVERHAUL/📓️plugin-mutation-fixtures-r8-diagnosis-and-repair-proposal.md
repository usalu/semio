# Plugin Mutation Fixtures R8 Diagnosis and Repair Proposal

## Scope and Evidence

This is a read-only diagnosis of the retained R8 executable inventory at [`📓️plugin-mutation-fixtures-r8-binary-inventory-2026-08-27.md`](../../../../20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️plugin-mutation-fixtures-r8-binary-inventory-2026-08-27.md) and all 24 retained per-case logs in that ticket's `🧪️member-plugin-mutation-case-r1-*-2026-08-27.txt` set. The executable was stale relative to current source; it is evidence of failure modes, not current compiler or runtime evidence. No Cargo, test executable, source, fixture, or controller was changed for this review.

The current read inputs were:

| Input | SHA-256 |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | `3e69e94d509fa56eda3f84c24cb55fd908390cf62a3c5c9227e0d9c5eb8894a4` |
| `🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs` | `0ddd5d7b88e026cdcd55b4def9beb4fb30e6d63fd1ecf450b6130c720af53b35` |
| `🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs` | `2eb46ec3079249d8522a63b67cba5c60861ae3f2e160eeef637e9eb0097ec157` |
| `🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs` | `937a2de33bd227d5ba3553c55fc1e0da8ae2f405cba7c05d2a3e4f24da26880f` |
| R8 inventory report | `4a522c44322572142b7f784de8dcf9f65badf0e0cc0206b2bfb1e0108abfdaa6` |

The retained outcome is exactly three substantive process passes (dummy meta, surface dialect, keyed no-state), one vacuous viewer process pass, nineteen `SIGABRT`s, and one ordinary failed test. The vacuous case is `surface::viewer_never_mutates_the_document_or_draft_store`: R8 did not await the helper. Current source does await it; that root repair is preserved and is not credited as a native replay.

## Primary Failures Versus Abort-on-Drop

`SIGABRT` is not the primary result for most entries. Each of the following primary failures unwinds into `ArtifactStore::Drop` at Store `component.rs:16389`, which then panics for lack of its exact terminal-empty shallow-shell witness. That destructor panic during unwinding causes the observed abort.

| R8 cases | First diagnostic | Classification |
| --- | --- | --- |
| 01, 03, 05 | `interactive-job.missing-factory` from dummy typed dispatch | Primary missing app controller/owner/factory/tool/schema proof; store abort secondary. |
| 08 | `interactive-job.missing-factory` from editor typed dispatch | Primary missing fixture authority; store abort secondary. |
| 14, 15, 18–21 | `interactive-job.missing-factory` or the resulting asserted mismatch (`transaction.instance-busy` expected) | Primary missing transaction fixture authority; store abort secondary. |
| 13 | `interactive-job.unknown-key` for `undo`, where `viewer.read-only` is asserted | Primary guard ordering/registered-vocabulary defect; store abort secondary. |
| 07, 09–11, 17, 22–23 | Store terminal-empty `Drop` assertion is the first panic | Primary unretired artifact/config/draft owner; no earlier law was reached in the retained log. |
| 16 | Presence-store detached terminal-empty assertion | Primary missing presence retirement/disposer; ordinary test failure, not a secondary abort. |
| 02 | `has overflowed its stack` / fatal runtime stack overflow | Only confirmed primary stack overflow. The log has no preceding Rust panic or destructor assertion, so a recursion source is not established. |

The R8 viewer guard failure is reproducible from current source shape: `VcsArtifactApp::dispatch_action` looks up `self.registry.get(action)` at `🦀️component.rs:21661–21663` before the viewer guard at `21666`. A registry-less viewer therefore yields `interactive-job.unknown-key` before it can yield `viewer.read-only`. This contradicts the adjacent claim that the guard is before every other branch. This is a framework guard ordering repair, not a reason to accept an unregistered action or weaken the unknown-key boundary.

## Exact Existing Seams

No replacement factory family is needed. These existing APIs provide the only supported path:

| Need | Existing exact seam | Current evidence |
| --- | --- | --- |
| Registered command construction | `VcsArtifactApp::with_registry_on_bus` at Plugin `🦀️component.rs:18891`; it calls `A::register_tool_job_factories` at `18922–18924` and joins manifest registrations to `<A::Command as OpBinary>::TOOL_JOB_IDS` | `testkit::new_app_with_registry` at `6645–6648` only supplies an `AppActionRegistry`; it cannot manufacture fixture factories or a manifest. |
| App-local job authority | `ArtifactApp::register_tool_job_factories` and async `build_tool_job` at `10850–10861` | Both default to no authority; Dummy and Txn currently inherit those defaults. |
| Document/config/draft close | `bounded_document_store_owners`, `bounded_config_store_owners`, `bounded_document_store_disposer`, `bounded_config_store_disposer` at `13132–13169`, backed by `ArtifactDocumentStoreDisposer::close_step` and `SpaceMember::close_owned_step` at `13183–13206` | `TxnApp` supplies only document/config owners, has no three closers, and compensates with the unbounded `close_transaction_store_roots` loop. |
| Presence/transient close and local roots | App-local `mutation_fixture::no_state::{presence_store_disposer, transient_store_disposer, presence_peer_retirement_factory, presence_local_root_retirement_factory, transient_local_root_retirement_factory}` | The accepted keyed fixture wires all five hooks at `32987–32991`; the helper is deliberately app-private because it uses private `BoundedConfigRetirementFactory`. |
| Full app close contract | `ArtifactOwnedDisposer::close_step`/`terminal_is_empty` at `11401–11410`, retained by `VcsArtifactApp` at `19093–19098` | A disposer must work on `&mut` actual store ownership; `current_root()` clones are expressly not a disposal substitute. |
| Viewer mutation boundary | `VIEWER_REJECTED_ACTION_IDS` and `viewer_read_only_fault` at `18575–18585`, plus the import guard at `22448–22451` | The string-action guard must execute before registry lookup for its documented contract. |

`SurfaceViewerCommand` at `surface/🦀️.rs:103–116` is also not a truthful command grammar: its binary decoder accepts every byte sequence as `Noop`, and it has no named text form. This is separate from the viewer guard. A canonical no-op vocabulary must round-trip exactly and reject malformed/trailing binary input; it must not decode arbitrary bytes as an accepted view command.

## Bounded Repair Packet

### 1. Fixture-Local Runtime Authority

Add exact, fixture-owned `ArtifactApp` hook implementations for Dummy, Txn, and the editor adapter; do not add a permissive `ArtifactApp` default or a generic testkit factory. Each fixture that dispatches a typed command must declare its own manifest action/tool identity, register its own `ArtifactToolFactoryRegistry` key, and return the corresponding owned job from `build_tool_job`. The fixture's selected command id and manifest tool id must equal the same `OpBinary::TOOL_JOB_IDS` entry.

`testkit::new_app` must remain registry-less and fail closed. Its current docstring promises exactly that. The successful-operation tests need an explicit registered fixture constructor (or a caller-provided fixture manifest/factory constructor) instead of changing `new_app` to silently create authority. This requires coordination with the app/testkit owner because `new_app_with_registry` takes only `fn() -> App`, while exact factories remain on the concrete `ArtifactApp` implementation.

Required direct owner writes after coordination:

- Dummy: `🎲️dummy/🦀️.rs` plus its own app/action/factory test fixture files if separated.
- Transaction: `🔀️transaction/🦀️.rs` plus its own app/action/factory fixture files.
- Surface: `🪟️surface/🦀️.rs` plus its own editor/viewer app/action/factory fixture files.
- Shared only if the app/testkit owner agrees: a parameterized registered-fixture constructor whose required inputs are the concrete manifest and `ArtifactApp`; it must not infer factory, tool, schema, or owner from type bounds.

### 2. Exact Retirement Join

For Dummy, Txn, and the editor adapter, install all store owners and close adapters actually retained by `VcsArtifactApp`: document, config, and draft have `MemberStoreOwners` plus their matching bounded disposer; `NoPresence` and `NoTransient` use the accepted app-local no-state disposer and local/peer retirement factories. This is a join to the existing app-private no-state child, not a public visibility expansion of `BoundedConfigRetirementFactory`.

The viewer cannot receive that repair solely in `surface/🦀️.rs`: `ArtifactViewer` presently offers config preparation and presence/transient local-root factories at Plugin `🦀️component.rs:26044–26077`, but no document/config/draft owner or disposer hooks, no presence disposer, and no peer-retirement factory. `ViewerApp<V>` correspondingly forwards only that incomplete subset at `26511–26535`; it defaults the other retained `ArtifactApp` close hooks to `None`. The R8 viewer/drop cases therefore require a separately released, narrow adapter-contract expansion that forwards the same explicit hooks from `ArtifactViewer` to `ViewerApp`; a fixture-local clone or generic fallback would conceal this real ownership gap.

Delete the transaction-only `close_transaction_store_roots` workaround after its app hooks make ordinary `VcsArtifactApp` close authoritative. It currently uses four independent `100_000` iteration loops and a `1_048_576` byte grant, installs draft owners late, never uses the retained disposer state machine, and does not retire presence or transient ownership. Replacing it with exact fixture hooks is the bounded repair; increasing its budget is not.

Native laws to add for each fixture must assert:

1. zero item grant reports no release and is nonterminal;
2. a short-byte grant does not consume an item;
3. each pending close reports no more than its provided item/byte grant;
4. `Complete` is idempotent and only follows exact terminal emptiness;
5. a held reader makes close `Blocked` without losing the original owner, then can complete after release;
6. missing factory remains a clean `interactive-job.missing-factory` Result in the explicitly registry-less constructor, with a separately bounded close—not an unwind/drop abort.

These laws must call disposers on the original mutable stores. They must neither dispose `current_root()` clones nor use `Option::take`/default replacement to claim retirement.

### 3. Surface Viewer Semantics and Grammar

Repair the action pipeline so the existing viewer-rejected action list is tested before registry lookup. The concrete native law must enumerate the seven existing string verbs and import, require `FaultOrigin::Framework` and `viewer.read-only`, and also separately prove an unrelated unregistered viewer action remains `interactive-job.unknown-key`.

Give `SurfaceViewerCommand::Noop` one canonical named grammar and strict binary framing (using the existing DSL variant codec rather than its current byte-ignoring `OpBinary` implementation). Required laws are: canonical text and binary round-trip, malformed/trailing binary rejection, wrong textual keyword rejection, and an awaited `assert_viewer_never_mutates` check against original document and draft generations. Root already added the missing await; this packet must preserve it.

### 4. Dummy Convergence Stack Overflow

Do not change stack size, budget, recursion guards, defaults, or test count. The retained case 02 has no backtrace beyond the runtime stack-overflow notice, so it does not identify an owning loop. First add a fixed, bounded phase witness around the current two-instance path: construct/attach A, construct/attach B, dispatch A, dispatch B, checkpoint A, checkpoint B, then probe. The law must record the last completed phase in its assertion context while preserving the existing command and backbone. Once the phase is known, repair only the owning recursive conversion/dispatch path and add a law that executes the failed phase exactly once without recursion; no broad fallback is justified today.

The dummy wrapper is a distinct imported `assert_two_instances_converge`; it has no direct self-call. The shared helper at Plugin `🦀️component.rs:6724–6734` retains two `VcsArtifactApp`s across its awaited dispatch/checkpoint sequence. Before constructing or polling that helper future, stage the following ticket-only native diagnostic law for the fixture owner to mount after the exact write set is released:

```rust
fn future_type_bytes<Factory, Future>(_factory: Factory) -> usize
where
    Factory: FnOnce() -> Future,
    Future: std::future::Future,
{
    std::mem::size_of::<Future>()
}

#[semio_framework_async_macros::async_test]
async fn dummy_convergence_future_size_is_observed_before_execution() {
    let app_bytes = std::mem::size_of::<VcsArtifactApp<DummyApp>>();
    let future_bytes = future_type_bytes(|| {
        assert_two_instances_converge::<DummyApp, i32>(
            "mem://testkit-converge",
            DummyCommand::Increment,
            DummyCommand::Increment,
            |app| app.snapshot().unwrap().count,
        )
    });
    eprintln!("[DEBUG] dummy convergence pre-execution sizes: app={app_bytes} future={future_bytes}");
    assert!(app_bytes > 0 && future_bytes > 0, "the type-size probe must retain both measured values");
}
```

The `FnOnce() -> Future` witness is never invoked, so the helper body is neither constructed nor polled. It distinguishes type-derived retained-size evidence from a recursion hypothesis without choosing an arbitrary size cap. A subsequent bounded phase witness may use the recorded measurements, but no stack/budget increase or `Box::pin` workaround is permitted.

## Required Coordination and Non-Overlap

1. **App/testkit owner:** registered constructor shape, real fixture manifests, action registry/tool registration, and the guard-order correction. `new_app` must remain fail-closed.
2. **Fixture owner:** Dummy, Txn, and Surface concrete factories, declared command vocabulary, and exact `ArtifactApp` hooks.
3. **Viewer adapter owner:** a released, explicit `ArtifactViewer`/`ViewerApp` forwarding set for document/config/draft/presence/transient ownership and disposal; no fixture-local clone can satisfy the current missing surface.
4. **No-state fixture owner:** retain the app-local private helper boundary; no `pub(crate)` expansion of the private factory and no copied disposer implementation.
5. **Store/runtime owner:** native replay after source capture only. This report does not claim compilation or native success.

Out of scope: PluginApp import and surface await already added by root, Store/DSL generic behavior, test stack/budget changes, cloned-root disposal, factory inference, and any source modification before an owner releases the precise write set.
