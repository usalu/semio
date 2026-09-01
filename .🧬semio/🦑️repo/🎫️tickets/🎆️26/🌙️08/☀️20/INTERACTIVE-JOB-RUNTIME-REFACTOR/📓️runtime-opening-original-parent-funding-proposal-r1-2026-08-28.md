# Runtime Opening Original Parent Funding — Post-R11 Proposal

## Decision And Evidence Boundary

Use one original `ResidentLedgerRoot`, the existing `PluginRuntime` registries and one original `RuntimeAppCell` across Opening/live/close. Fund that original parent before any app factory runs. Its closed, typed Store field selects the original Store and its existing displaced-owner FIFO; one `ResidentRecord<ArtifactStoreBackboneRetirement>` is the retirement shell backing. No second pool, retirement queue, external shell Box, numeric capacity permit, or structural Option witness is proposed.

This is a **ticket-only declaration**, not production/API implementation or a source/native test result. The adjacent `🧪️runtime-opening-funding/🧬️schema/🔣️.json` defines this proposal before its `🔣️.json` declaration. Neither file is a wire ABI or executable fixture. No oracle, Cargo, native test, Wasm check, dependency edit or include was run/mounted for this proposal. Existing Opening7 and resident25 sources remain unchanged.

Read the complete actual R11 report, original Store-parent proposal, R2 owned-source candidate, earlier Opening/private-consumer reports, and current source below. R11 actually ran25/25,0skip,.149s,Nx0 with73 stable tuples. Passing per-test stdout was unavailable. R11 proves the tested neutral Destroy/Free/Refund/Clear implementation, **not** parent funding, Store field selection, live child release, arbitrary large allocation retirement, or callback-tail quiescence. The R2 report's pre-R11 free/refund paragraph is historical and superseded only for that neutral implementation. Its Directory/codec changes and unjoined SyncSession detach remain separate.

## Current Source, Not Assumed Capability

Plugin `plugin_runtime::RuntimeAppCell` at27749 contains `Mutex<AppInstance<PA>>` and the maintenance pump. `PluginRuntime::new` at28087 allocates three1024-slot registries. `RuntimeInstanceRegistry::new` at27870 uses `try_reserve_exact` then `into_boxed_slice`; `allocation_admitted` means allocator success, not composition authority. The registry's empty Drop does not prove exact backing retirement or payload recovery after its enclosing root is lost.

`Plugin::create_app` at24832 invokes `(AppDefinition, fn(&AppDefinition)->PA)` from declarations27066. Editor/viewer factories26989/27004 synchronously resolve `VcsArtifactApp::with_registry_on_bus`. `plugin_create_app_with_id` at28507 receives and binds a complete PA before creating `Arc<RuntimeAppCell>`. The constructor18970 creates four Store locals before the catalog assertion. The actual earlier R4 constructor unwind remains a defect; fixing its valid roster did not repair failure ownership.

Existing `ArtifactStoreInitializationAuthority`/`ArtifactStoreInitializationJob` at13339 already own typed partial initialization and cancellation. Reuse that domain step/close authority where supplied; do not create a second general initializer. Its current boxed authority, cancellation Arc, by-value `take_candidate`, terminal Box drop and ignored close error are **not** funded/in-place transfer evidence. The actual Opening join must replace those specific escape points for the selected initializer before using it, not wrap the whole old constructor in a future.

Store1489 owns a1024-entry `VecDeque<Box<dyn ErasedSnapshotRetirement>>` with eight reservation slots. `replace_backbone_retained`14177 still allocates its Box after taking the original backbone. `detach_backbone`15687 clears the descriptor, calls `bump`, and returns `Result<Option<Backbones>,VcsError>`. SyncSession900 still sends first, awaits that synchronous result, then clears command/event owners. None is repaired here.

## Smallest Reviewable Cut

1. **Fund the actual runtime/cell before producer invocation.** Replace eager registry construction with retained preparation in the original runtime registration. Reserve the real three registry backings, exact Opening/cell/error/close metadata and selected app backing before installing a factory. The first native gate tests this actual constructor and cancellation, not a synthetic successful app. At this cut no Store detach or host Open success is claimed.
2. **Initialize the selected app into that original parent.** Replace the selected factory's whole-PA return with a typed construction descriptor for the existing PA branch. The same parent owns every partially initialized field and error before a step returns or unwinds. Exact field terminal ownership must be established before exposing the existing live app API. Existing Opening7 remain the seven desired integration laws, not silently rewritten passing tests.
3. **Bind one original Store/FIFO child.** Only after cuts1–2 can the original `VcsArtifactApp` issue a Store retirement receiver. Mount the one shell backing and targeted release into the same R11 root slot. This is Store's next typed child, not a separate funding implementation. SyncSession request/channel retirement is still another dependent join.

These are three dependent source boundaries, not three independent pools or alternate runtime modes. The existing unfunded constructor must not remain as a production fallback. During an incomplete cutover, affected construction stays unavailable; test-only old APIs are not proof of the new path.

## Root Construction And Bootstrap

The embedding composition retains one actual inline `ResidentLedgerRoot` outside every fallible callback through final close. Its exact fixed native Layout is the sole declared bootstrap exclusion. No registry, control array, Plugin bundle, RuntimeAppCell, factory, fault payload, Arc control block or descendant is included in that exclusion. This does not invent a process-wide total: the embedding host/guest must provide the already reviewed explicit original composition; multiple native runtimes cannot each claim the process-static UI domain.

Preparation first reserves a canonical consumer registration for the actual runtime control state, then allocates/initializes/publishes it through the root's existing pending ownership. Only a borrowed exact access facade escapes. The runtime's retained backing contains no reference to a movable facade. New metadata is included in the requested containing Layout before reservation. Lost method returns leave the root's pending/prepared slot intact. A second equal-capacity root cannot recover or advance it.

The original runtime registration owns its three registry preparation records. Their fixed logical capacities remain1024; their initialized frontier/occupancy and actual backing descriptor remain recoverable if preparation fails after any allocation. The app cell registration is recorded in the original instance slot **before** the first app/registry/bus/envelope producer. An occupied, closing or failed Opening remains that same slot and cannot be overwritten by numeric-ID reuse.

The implementation must not store `ResidentConsumer<'root,_>` inside a self-referential root or use a last-Arc owner for the ledger. Root-internal registration keys are private lookup associations, never public pointer authority. Recovering an access facade must verify the original root-owned descriptor and mint a phase-qualified temporary alias under the existing gate. That alias cannot outlive the original root. No thread-local/global default ledger is added.

## Exact Fields And Native Pricing

All sizes below are **expressions to measure from the actual selected native types**, not numbers inferred from TS prices or a claimed layout measurement. `N(T)=Layout::new::<T>()`; `A(T,n)=Layout::array::<T>(n)` with checked overflow/alignment. For each separately allocated canonical node charge `(requested_layout.size(),1,1)` plus only an explicitly owned external envelope; inline metadata is counted once in its containing Layout. Native allocator requests, not heap-bin estimates, are the physical evidence.

| Existing owner / exact proposed retained field | Backing and charge |
| --- | --- |
| Original ledger | `N(ResidentLedgerRoot)`, fixed external bootstrap only; measure final original-root work separately. |
| Original runtime registration | `N(ConsumerNode<PluginRuntime<PA>>)` including retained registry-preparation descriptors, phase, exact app-slot reservation, first fault handle and binding fields. Control. A moved facade is not this storage. |
| Three original registry backings | Actual element types for instances, close quarantine and actor authority, each1024; `A(MaybeUninit<(u32,T)>,1024)` describes current monolithic requested extent. Charge every actual replacement backing before allocation; no extra capacity or allocator-success permit. Control. |
| Original app cell | One canonical registered allocation for `RuntimeAppCell<PA>`; replace inline whole instance with private stable app-allocation ownership, add selected Opening handle, `retiring_instance:Option<RuntimeInstanceAllocation<PA>>`, phase, exact close binding and fault/tail handles. Include existing maintenance/close state; no duplicate pump. Control. |
| Selected app backing | One canonical `RecordNode<AppInstance<PA>>` representation, including its actual initialization state/field bitmap if needed. Data; no extra Box and no completed PA temporary. Its stable handle moves, not arbitrary PA. |
| Admission/receiver metadata | Existing `AdmissionNode` and actual private parent/child edge metadata, reserved from the same partition as their shell. A field descriptor embedded in an already funded parent/FIFO is not charged as another heap allocation. |
| Opening partial owners | Selected `VcsArtifactAppOpening<A,M>` construction state with actual app/registry/bus input, IDs, envelopes, four Store fields, actual disposer/initializer handles, catalog progress, presence/transient and existing registry preparation. Every referenced allocation separately admitted; no blanket envelope inferred from the state size. |
| Store displaced FIFO | Same FIFO with typed `Backbone` and existing erased entry variants; actual `A(MaybeUninit<ArtifactStoreDisplacedEntry>,1024)` and the unchanged eight reservation entries. Changed element Layout must be newly accounted, never inherited from1024 Box-pointer bytes. Control. |
| Backbone retirement record | `N(RecordNode<ArtifactStoreBackboneRetirement>)` after adding retained `ArtifactBackboneRef`/descriptor ownership to the existing four fields. Control. Original transferred backing/payload charges retain their original partition and owner. |
| Final receipt/binding | Extend the original root's one inline `ResidentRelease` association and original FIFO entry, not S or a new allocation. Root enlargement is explicit bootstrap metadata; FIFO enlargement is charged before initialization. |

The control/data split remains the canonical disjoint `control` and `total-control` on bytes/slots/owners. Closure never borrows another domain's capacity. The existing Opening64MiB/control8MiB fixture is isolated test input, not a native policy, UI32MiB increase or fit result. Any missing capacity source is a refusal before producers, not a guessed default.

### Non-Negotiable4096 Layout Gate

R11 `prepare_consumer`/`reserve_record` and Destroy/Free currently require grants including the whole node Layout. Thus a monolithic registry or sufficiently large AppInstance cannot be honestly initialized/freed by this implementation under4096. A small stable transfer handle fixes transfer size **only**, not allocation, initialization or final deallocation. This packet does not change that accounting or assert fit.

Before positive Opening tests, a source-selected native inventory must measure all containing Layouts and each write/free frontier. Oversized layouts are recorded as a desired refusal after cleanup, never resolved with larger work grants. Registry backings may be page-partitioned **inside the same existing fixed-capacity registry**, with every page plus resident metadata fitting the actual per-phase bound; no second pool or additional logical slots. This representation change requires its own exact layout declaration. The single AppInstance backing remains the reviewed choice; if its measured Destroy/Free frontier exceeds4096, that is an explicit unresolved representation prerequisite requiring review, not permission to lie about freed bytes or invent chunked calls to a single deallocation. No positive all-app/Store live mount may precede that decision.

## Source-Owned Construction And Receiver Surface

The following signatures are proposed source contracts, not compiled APIs. All opaque argument types have private constructors. They are captured from the actual original registered owner, not built from an ID/offset/Layout. Each operation returns bounded progress and leaves variable error/panic ownership in its original funded field.

```rust
PluginRuntime::<PA>::prepare_resident(root: &ResidentLedgerRoot, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
PluginRuntime::<PA>::capture_prepared(root: &ResidentLedgerRoot)
    -> Result<Option<RuntimeParentAccess<'_, PA>>, ResidentFault>;
RuntimeParentAccess::prepare_app_open(&mut self, selected: &AppFactory<PA>, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
RuntimeParentAccess::advance_app_open(&mut self, opening: &RuntimeOpeningKey<PA>, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
RuntimeAppParentAccess::prepare_instance_retirement_field(&mut self, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
RuntimeAppParentAccess::handoff_instance_for_close(&mut self, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
VcsArtifactApp::<A,M>::prepare_store_backbone_retirement(
    parent: &mut RuntimeAppParentAccess<'_, PA>, store: StoreField, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
ArtifactStore::<P,Mutation>::advance_backbone_detach(
    &mut self, field: &mut StoreBackboneRetirementField<'_, P,Mutation>, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
ArtifactStore::<P,Mutation>::close_backbone_detach(
    &mut self, field: &mut StoreBackboneRetirementCloseField<'_, P,Mutation>, grant: ResidentGrant)
    -> Result<ResidentStep, ResidentFault>;
```

`StoreField` is the existing wrapper's closed set document/config/draft/interaction, not a public callback or arbitrary offset. The Plugin-owned selected PA branch alone can obtain the actual wrapper and issue its exact typed field descriptor. Store owns the displaced-field/reservation half. The neutral crate owns the canonical private parent/record association and alias/release checks; it does not import Plugin or Store types. A cross-crate facade must carry that already-issued association, not expose a public unsafe projection method. The exact field integration must be reviewed with both modules before implementation; address containment is only a memory-safety check, never authorization.

The field binds original root + runtime registration + cell registration + source consumer/record + closed Store selector + actual Store registration + FIFO `(slot,generation,count=1)` + original backbone generation + phase. The original Store association must be captured during construction, not inferred from current `&mut self`, numeric Store generation or `P/Mutation` equality. Source and target grant/write extents are revalidated together immediately before any take. Receiver preparation/installation and actual source movement are distinct granted transitions. A rejected receiver leaves the exact input source, both reservations and original channels unchanged.

Do not keep a `ResidentRecord<S>` facade alias alive through Free. Before release, the original private binding revokes forward/field access and relinquishes payload aliases; the original root retains a pointerless association to this exact binding through Refund/Clear. Only the root can report this release's Clear back to that binding. A copied generation, empty Option or external allocator observation cannot mint the terminal receipt. Lost return after Clear remains recoverable from the same original FIFO entry; no freed record pointer is read.

## Constructor Before Return, Error And Cancellation

The selected descriptor places its real construction state into the funded cell before invoking app callbacks. Retain `A`, `AppActionRegistry` and `ActionBus` inputs in that state; setup-owned fixture input is not proof that production bundle/input allocation was preadmitted. Existing initializer authorities must write directly into those original fields or transfer through a privately registered destination. Current whole-store `take_candidate` into a local is not accepted.

App/config/draft/interaction identifier and envelope progress, each partially initialized Store, its disposer, and partial factory/catalog registrations remain in the original Opening state. A failure after the fourth Store therefore retains those same four roots, even before a successful `VcsArtifactApp` or PA exists. Retaining only a returned error is insufficient. The original first fault and panic payload must have a preadmitted typed handoff; arbitrary panic final disposal remains unknown until its actual owner supplies proof. No fault-string substitution, leaked test owner or strict-Drop suppression is proposed.

Cancellation stages are: reserved/no allocation → allocated/uninitialized → typed Opening fields installed → partial Store/catalog construction → ready/unpublished → live child preparation → committed child retirement. Each has an exact original close path. Closing revokes new writes and field minting first. Precommit cancellation retains the original backbone/descriptor and retires only the prepared empty child. Postcommit cancellation advances the same committed FIFO owner; it never restores the old backbone, resends a request or refunds its backing early. Unknown live poison remains retained under R11; it is not recovered by this parent proposal.

No fallible allocation, callback, clock, registry lookup, unchecked generation advance, cursor clone or ordinary `bump` occurs between the admitted fixed Store source transfer and its structural destination update. The descriptor plus original Backbones move into the already initialized shell; semantic content revision is unchanged. After this commit, errors refer to the original committed owner and are retained, not treated as a failed preflight.

## Targeted Child Release In The Same Root

R11 public root close sets whole-root closing and selects its list head. Calling it to retire a live Store child would revoke unrelated app work and can block on another live record. Therefore this cut needs **private exact descriptor detach** under the original parent/Store binding, not another public root close or release queue.

The child’s terminal/alias proof authorizes moving only its original `ErasedRecord` into the existing single `ResidentRelease` when that slot is empty. A busy release retains both FIFO and original record unchanged. The parent stays live; no generic C-empty assertion is weakened. Record Destroy/Free/Refund/Clear remains R11's ordering and original partition. Then retire its exact admission registration under that same binding; do not retain one dead admission per completed detach until capacity is exhausted. The existing linked registration needs a checked exact unlink mechanism and a preadmitted unlink/receipt cursor or link metadata. Its concrete extra native Layout must be included before allocation; unbounded list search is not authorized. No extra root-owned heap queue is necessary.

The Store FIFO cannot pop its typed entry on `S::terminal_is_empty` alone. It stays through the actual empty-S destruction, node destruction, Free, Refund, Clear, admission unlink/release and final binding retirement. Typed S destruction is separately granted and source-owned; neutral record empty checking does not know Store terminal semantics. The charge receipt lives outside S. All released backing, slots and owners are observed on the original partition; other live children are unchanged.

### Chosen Minimal Live-Child Metadata And Call Decision

Do not implement a second release driver. Extend the existing root-owned descriptor path with these exact fields, all still proposals:

- `LedgerState.next_field_generation:u64`, checked before registration; no wrap. It is a private binding discriminator, not an externally usable capability.
- `AdmissionFields.previous:Option<NonNull<AdmissionNode>>` and `binding:Option<ResidentFieldStamp>`. Preserve the existing next link. An exact detach checks reciprocal previous/next links and original root membership before changing either; no whole-list scan or second index allocation. Add both fields to the actual admission Layout and charge before allocating that node.
- `ResidentFieldStamp` contains the original privately registered parent generation, its source-owned field generation, and this child admission generation. The exact access capability still contains the original root/consumer association; copying equal stamps cannot construct one. Field stamps and the parent generation are reserved in the actual registered parent, not synthesized from instance ID or Store content generation.
- `ResidentRelease.binding:Option<ResidentFieldStamp>` retains the original bound release association. For a bound release, Clear produces an inline `ClearedAwaitingBinding` residue in **this same release slot**, not a returned receipt owner or a new queue. Its only data are the private stamp, released kind and original partition/resources; there is no freed-node address or destructor. Unbound root-close behavior remains R11. This added stage makes a lost completion return recoverable and deliberately blocks reuse until the original binding acknowledges it.
- Existing Store FIFO's new `Backbone` entry holds `StoreBackboneRetirementBinding { parent_stamp, store_stamp, reservation, record_key, phase }`; the keyed record facade is reacquired for a bounded access and dropped before release. The entry and original Store remain allocated through record Clear, then admission Clear, then separately granted FIFO pop. No final receipt is stored inside the shell it releases.

The source-owned Store half is captured during the real Store initialization into the selected Opening field. That private initialized association is the only input accepted by `prepare_backbone_detach`; `&mut ArtifactStore`, type equality or a newly borrowed wrapper is not sufficient. A source-owned constructor method registers the exact Store/FIFO field with the original parent capability; the Store crate exports the opaque field facade but not its constructor, pointer projection or field-offset API. Plugin's closed factory/Store selector passes the already registered constructor association. This is the cross-crate seam to implement, not a public generic closure that projects arbitrary memory.

The exact call sequence is: original parent prepares the Store field → Store reserves its existing count-one FIFO slot → canonical admission/record preparation stores the shell descriptor → Store initializes the real empty retirement state → Store commits original backbone plus descriptor in place → Store advances its own typed retirement → Store supplies its private terminal handoff → root begins that bound record's release → existing root release advances Destroy/Free/Refund/Clear → original binding acknowledges the matching residue → root unlinks and releases that exact now-empty admission → original binding acknowledges that residue → Store pops the original FIFO entry under its own grant. The consumer reference decrement for this exact completed child does not require the still-live C to be empty; it requires the private child binding's terminal/revoked proof and no aliases. Global whole-root admission close keeps its existing C-empty rule.

Proposed neutral operations are `begin_bound_record_release(field, grant)`, `advance_bound_release(field, grant)` and `acknowledge_bound_release(field, grant)`. They require the privately issued original field capability and check the current inline release stamp/kind. There is no public refund or arbitrary target Option. The entry owns progress; methods return only progress/fault. Unlinking the completed admission is separately granted and uses its still-live stored links before any free. A foreign or stale field cannot start, advance or acknowledge another release. A busy release does not detach a second source. Pointerless poisoned close may advance only the R11-safe numeric/root-local phases; it cannot inspect a live Store or fabricate its binding acknowledgment, so an unknown poisoned parent remains held.

The new root/bootstrap cost is exactly the Layout delta from `next_field_generation` and the enlarged inline release enum. New admission cost is its actual previous-link/stamp Layout delta. New FIFO backing cost is its actual typed-entry Layout delta times the unchanged1024 logical slots. No external receipt Box, list-search cursor allocation, hidden descriptor table or additional allocation receipt is introduced. Native tests must measure these deltas; this report gives no invented byte values.

The original parent itself cannot retire while callbacks can acquire a future access alias. Existing `RUNTIME_CLOSE_COMPLETE`, `PluginInstanceCloseLease::is_retired` and Arc count sampling are insufficient. Keep the original quarantine slot until the scheduler's exact completed-closure/closed-upgrade witness, existing pump/session/outcome close, actual app terminal proof and final original shell release. That existing WorkerPool submission extension is separately coordinated; this packet does not create a scheduler, fake completion bit or waive callback-tail work. Final original root close remains measured before the last clock sample; a late fault retains the released-shell result instead of resurrecting state.

## Desired Source And Native Laws — Not Executed

Preserve Opening7 and resident25. A later reviewed source controller should extend the existing Opening domain's `📜️script.ts` with the declaration, not create another script. Use strict Ajv against exact field/phase lists, BigInt checked layout/resource arithmetic and Immer as an independent state-transition reference. The third-party model validates declared arithmetic/identity/refusal only; actual native allocation and private authority still require native tests.

1. Actual runtime preparation refuses zero/short grant, insufficient bytes/slots/owners and foreign equal-capacity root before its first allocator/factory event. Observe requested native Layout and charge at allocation entry; clean exact roots before assertions.
2. Measure stable-handle/source/destination write Layouts independently of `required_move_bytes`; preserve4096 and actual oversized refusal. Layout overflow/allocation-null/partial initialization preserve original parent and charged reservation. No allocator unwind injection.
3. Inject a callback panic after real original-field placement at each of the four Store frontiers and catalog registration. Original roots stay reachable in the externally retained RuntimeAppCell, not in caught panic payload. Close exact disposers before intended assertions; unknown final payload remains explicit incomplete ownership.
4. Same type, same numeric instance/Store generation, same capacity, wrong closed selector, replaced Store and stale FIFO reservation cannot acquire the receiver. Actual other-thread close/cancel revokes further mint/forward access and preserves attempted input custody.
5. Actual Store detach fails before source movement when the existing FIFO is full or generation is exhausted. Exact admitted commit moves original Backbones plus descriptor into the one canonical record, preserves revision and retains the same FIFO ordering; no returned owner is ignored.
6. With two live app parents and an unrelated head registration, retire one Store child through the same Release slot without closing either root or losing the other child's access/charge. Busy release and concurrent alias preserve exact source.
7. Drop/move access facades and interrupt after reservation/allocation/commit/Free/Refund/Clear; recover from the original parent slot. All-axis conservation is checked separately for Data and Control, including nonzero other-owner charges. Original after-System-return hook observes Free before Refund; no callback observer mints authority.
8. Repeat child prepare/commit/retire enough to reuse actual FIFO/reservation positions; old keys cannot see or release new storage and dead admission nodes do not accumulate. Work bounds derive from captured page/link/Layout plans, never an arbitrary retry count.
9. Pause the actual scheduler-owned callback tail after app terminal work; parent backing and quarantine remain until the private completed-closure witness. Final exact shell destruction/free/refund/clear precede the final clock sample, including late-fault retention. This remains blocked until the real shared scheduler seam exists.

## SyncSession Is Not Implicitly Covered

Preserve Retained's separate census: no actual RuntimeAppCell-owned SyncSession was demonstrated; the two constructor calls were cfg tests. Do not issue a document Store receiver for an unrelated session merely because `P/Mutation` match. Its actual original parent must be identified before funding the session fields.

SyncSession must reserve the original mailbox request capacity and the exact Store receiver before a joint commit; keep the request, command sender and event receiver structurally owned through refusal/cancellation. Current `send` plus clearing Options is not that protocol. Tokio broadcast receiver Drop can walk unread slots and drop/clone event payloads; its retained channel retirement requires its own reviewed ownership proof. Store's `Result<Option<Backbones>>` cannot be ignored or turned into async void to hide the compile error. No caller/await/Directory/codec change is included here.

## Captured Source Boundary

Read-only SHA256 observations at this review; peers may independently change their owned Store/Sync regions afterward. No immutable whole-repository claim.

| File | SHA256 |
| --- | --- |
| resident/🦀️.rs | e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3 |
| resident/🧪️tests/🦀️.rs | e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175 |
| Plugin main | 2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca |
| Plugin lifetime | 5cc2eba5bc406eb3d6d232fc7e948f9f21be316e7099dc98c23eca959ff37046 |
| Store main | 7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4 |
| Store Sync | 62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6 |
| Opening7 Rust | 01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1 |
| R2 owned-source report | 3734b0036688955ea22bcf6b9b9be93c66860bddd105a3770a59dbfbb007f752 |

Only this report and its two ticket declaration/schema files were created. No production changes, new native/Wasm execution or proof of complete Opening/Store funding is claimed. The next implementation authorization should choose cut1 plus the exact layout/refusal tests first; cuts2–3 remain source-reviewed dependencies, not compiler-ready promises.
