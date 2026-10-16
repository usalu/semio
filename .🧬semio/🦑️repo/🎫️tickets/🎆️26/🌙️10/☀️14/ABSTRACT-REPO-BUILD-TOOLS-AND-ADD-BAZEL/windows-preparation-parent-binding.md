# Actual Parent Binding for Native Preparation

Ticket-only design and Source census; no permanent changes or runtime. This refines [the provisioning plan](windows-native-provisioning-plan.md). The x64 Source scope and separately recorded ARM64 host gap remain unchanged.

## Exact Existing Authority

platform/🟦️.ts keeps a private WeakMap of actual canonical image objects, captured control/leaseDirectory/port/method identities. requireNativePlatformImageV1 rejects a spread copy. withNativePlatformImageOperationV1 authenticates that map and delegates the exact real port operation. State already retains withNativeImageMetadataV1(parent,actual,…); NativeArtifactCache retains withNativePlatformImageOperationV1(parent,…). Both nevertheless call the one-argument child opener and therefore discard explicit parent identity at preparation.

FixedPreparation.parents is only physical ancestry. Its bodyControl, accepted cleanup, kernel writer proof, receipt descriptor/inode and exact directory stamps authenticate the child creator, but none is an actual parent Native loan. The parent must be carried from a genuine factory operation, not inferred from directory names, matching dev, a structural NativeImageStorage object or an ad-hoc callback.

Repo partition wrappers are deliberately spread objects whose close method additionally owns leases. They are not platform-WeakMap images. Existing withRepositoryCachePartitionArtifactCacheV1 and withRepositoryDevelopmentStatePartitionV1 correctly unwrap their private partitionOwners entry and hand scope.image to Core; preserve that pattern. Do not weaken platform auth to accept a wrapper. RepositoryCacheStore similarly has private issuedStores/StoreOwner records and actual maintenance scopes; a structural store.image field is not enough for unwrapping.

## Proposed Closed APIs

The platform-level child entry is `withNativePlatformChildStorageV1<T>(parent, options, body(child, actualControl)): Promise<T>`. Parent and options.control are mandatory. Authenticate the exact registered parent, enter its actual operation, require the actual returned phase to refine both captured parent and requested child phases, capture actual backing, check containment, then delegate the concrete backend child scope. The callback is ordinary work; it never supplies or verifies an authority proof. The function issues the opaque binding privately before invoking work.

The concrete fixed entry is `withNativeFixedChildStorageV1<T>(parent, options, body(child, actualControl)): Promise<T>`. It independently finds the exact parent image in fixedReceivers, checks stage/owner/facts/control, and retains withNativeImageBorrowerOperationV1. It creates an unexported FixedParentBinding token in a private WeakMap in the same concrete owner as FixedPreparation. That scope captures receiver, exact immutable backing/mapping, captured parent directory/root identities, actual returned phase, accepting/live/failure state and pending child work. Only its private birth path attaches the token to FixedPreparation. No token constructor, arbitrary binding registration, function-valued verifier or public parent-proof argument is exported.

The fixed opener implementation becomes a private primitive receiving the already-issued binding; the existing public host entry cannot accept that private parameter or a bypass flag. macOS has the equivalent concrete scope using its persistent receiver/borrower registry. Platform dispatch authenticates its own registry before calling the concrete scopes; lower implementations authenticate their own actual receiver. This avoids a fixed→platform runtime import or NativeStorage→platform cycle. Shared declarations may live in the existing platform contract as codebase-owned types only; no runtime dependency or Repo type leaks.

The child scope drains accepted body work, closes the actual child, then retires the binding and releases the parent operation. Body primary wins over any close failure, including undefined/frozen values; unresolved close retains custody. New births stop after first failure. The parent cannot close until the child close/drain settles. Do not return an unscoped child handle and end the parent loan immediately. Existing multi-child scopes can hold individual child scopes concurrently within their bounded owner; no new unbounded queue.

## Parent EX Maintenance Is a Separate Admission

Nx pool creation can occur inside genuine parent withExclusiveMaintenance, where normal parent borrower admission is intentionally stopped. Reentering the normal shared operation would reject or deadlock. A second closed entry is therefore necessary: `withNativePlatformMaintenanceChildStorageV1<T>(parent, actualMaintenanceCustody, options, body)`. The concrete parent maintenance owner must have privately registered the **exact returned custody object and actual port** during its callback; require its live frame, exact kernel EX proof/resource/control and original receiver facts. A caller-built `{kind:'kernel-ex',proof,…}` is insufficient. A fresh-private object likewise requires private live owner registration, or refuses before effects. Do not add a public held-gate boolean or reacquire parent SH under EX.

This maintenance child scope does not outlive its parent maintenance callback. It drains child close before callback exit and uses the callback's actual control for all continuation checks. The lower fixed/persistent owner already owns both its maintenance state and receiver registry, so issuance stays local. The normal and maintenance tokens can share a private discriminated implementation; the public APIs remain explicit and mandatory.

## Source Caller Census and Threading

| Actual owner | Existing open and required route |
| --- | --- |
| process/💾️state/🟦️.ts openBound | Child directory is beneath actual parent.root. Replace return-an-open-child structure with the closed child scope, inside existing actual parent metadata scope; retain through State tasks/data drain and child close. Existing actual parent metadata token can continue owning parent records, but does not become a forged preparation binding. |
| artifacts/💾️cache/💾️volume/🟦️.ts openBoundCompilerStateVolumeV1 | Child under actual parent.root, real parent operation already present. Scope the physical child through compiler state tasks/receiver drain/close. Carry returned actual phase throughout. |
| Repo enforcement/🔌️nx/📦️graph/🟦️.ts openNxGraphCacheStoreV1 | Pool under RepositoryCacheStore parent.root. Add a **closed Repo bridge** on the private issued store/partition owner that unwraps the exact image and actual body/maintenance custody, then calls the matching Core scope. Provision mode needs actual EX variant. No generic native-image getter or structural field acceptance. |
| Same Nx owner namespace acquisition | Each namespace image is beneath the actual registered pool image store.image. Use normal child scope there; retain through namespace worker/publication and close. |
| Repo opening/🏪️store consumer fixture | Diagnostic image lies inside actual parent store root. Use the same closed Repo bridge. Reopening for retirement must stay inside a legitimate parent body or already accepted retained cleanup owner; do not issue a fresh body phase after cancellation. |
| Repo budget opening partition opens | leases/products/graph/etc backing directories are authored host siblings. Holding leases is a dependency, not their containing Native parent. Preserve distinct initial host preparation. Do not pass the lease image as parent merely because it is open. |
| Repo graph/headroom open | Options identify the authored host-backed graph partition, not the nested Nx pool. Preserve host entry and real lease dependency. |
| platform/📥️issuance → baseline | Generic whole-image loan used by resource baseline; callers supply its original declared backing. Census actual request owners before designating a nested request host. Baseline seals and raw logical reads retain their existing independent rules. |
| Linux/Windows backend wrappers | Sole permanent direct fixed opener callers. Route their host and child variants explicitly; do not export the private primitive. |

All positive test callsites must use actual issued Original/phase and explicit physical model ports where appropriate. Structural controls stay only in intentional refusal tests. Existing AST path/admission/export-boundary laws will need truthful schema/API updates; no declaration-list-only proof.

## Host and Containing-Volume Rules

Initial host publication is authenticated by the actual private creator (writer proof plus exact owned receipt/file/directory/handle). It does not fabricate a Native parent token. A nested caller must use the child entry; normal host entry rejects paths inside any currently retained canonical Native root unless entered through the proper child binding. That registry exclusion is a useful immediate guard but does not prove that every externally mounted VHD is known to this process. A complete host policy must not relabel an unknown external Native mount as a retained parent; retain an explicit unsupported/ambiguous observation before effects where provenance cannot be established.

For nested preparation, capture actual parent backing/volume before mutation and compare the owned receipt's containing volume with it pre/post barrier. Parent physical root→child creator ancestry stays bounded, no-alias and same-device; private expected Native mapping is distinct from child mounted-root mapping. Flush applies only to captured parent volume. For initial host, scope the host-volume permission only to this creator's own receipt publication/cleanup; it cannot become an API for arbitrary host paths. Exact mapping plus descriptor identity must survive rename/delete, and accepted cleanup retains the already-issued owner instead of reminting a body grant.

## Focused Fault Laws Before Release

Actual Source-model ports should verify: copied parent/partition/custody rejected before effects; correct factory parent accepted; sibling lease dependency rejected as containing parent; parent replacement/mapping change; outer phase versus returned narrower phase; child waiting under parent EX without recursive SH; attempted parent close while child work pending; child close precedes parent release; cancelled body still drains accepted cleanup; caught first undefined barrier failure closes further births; frozen body primary wins. Independent Ajv/SQLite observes the closed input/trace ordering, while OS physical qualification remains separate. Extend the prepared ticket input corpus rather than using a constant-only mirror of implementation.


## Close and Adjacent Writer Findings

An external parent.close request must wait for already accepted child work/close. A child body awaiting its own parent.close is a different case and must synchronously refuse; waiting creates a selfcycle. Existing borrower/🟦️.ts has requireOutsideBorrower walking its actual AsyncLocalStorage frames and refusing close from an accepted parent operation. Preserve that frame around the complete new child callback. Maintenance child scopes need the equivalent actual-maintenance-frame refusal. Add both distinct trace laws; an external-drain law alone is insufficient.

The Source census also found NativeArtifactCache writeCompilerStateRecordV1 still renames then opens/fsyncs its containing directory directly. That owner writes parent intent/binding before/around child preparation and child catalogs later. It must eventually use the matching authentic parent/child metadata scope for Windows task completeness. This is an adjacent existing writer finding, not authorization to alter it during this design hold or proof that preparation receipt alone fixes every Windows build task.
