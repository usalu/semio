# Core Fixed Artifact Cache Extraction Plan

This is the preimplementation ownership/API plan. No Source file, physical image, allocation, compiler, lease namespace, SDK or existing evidence was modified. The last pressure epoch remains the qualified 22/288 BOTH plus expanded strict BOTH zero. The initial inspection attempt failed before process creation with system `ENFILE`; subsequent bounded reads succeeded. No cleanup/action was taken in response.

## Current Ownership and Concrete Seam

The Rust state volume currently contains domain-neutral fixed-image funding/binding, catalog/slot/retirement transitions, bounded tree inventory, rooted descriptor exclusion, session receiver drainage, pressure planning and physical child receiver management. It also imports repository cache opening, compiler layout/reservation and CompilerWorkspaceCustody. Moving that file unchanged into Core would introduce an invalid Core-to-repository dependency.

Actual Native interfaces are `NativeImageStorageV1` (`port`, `recheck`, `close`, real maintenance) and `NativeStorageReservationV1`. Native `port.withOperation(control,run(actualControl))` already authentically retains a provider's private borrower/creator frame, joins accepted descendants and prevents its guard from releasing early. A genuine reservation is tracked by the allocator's private WeakMap, but the public image interface alone is structural. The extraction must not authenticate a copied/fake parent image merely by its fields or a successful callback.

The minimal prerequisite is a read-only retained-parent operation in the existing canonical platform opener:

```ts
withNativePlatformImageOperationV1<T>(
  image: NativeImageStorageV1,
  control: ResourceLeaseControl,
  body: (actualControl: ResourceLeaseControl) => Promise<T>
): Promise<T>
```

The canonical `openNativePlatformStorageV1` privately captures only the actual backend return and its exact opening control, immutable root/backing/envelope and real port/method references. There is no public register function, new provider factory or serialized brand. The operation refuses unknown/copied/mutated/retired images, authenticates actual requested-control refinement and calls the captured real `port.withOperation` synchronously before the first effect. It delegates retention and returned-phase selection to that existing Native owner. It does not manufacture an Original/control or give callback access to a port/maintenance/close capability. Direct backend or temporary handles are not silently adopted; planned cache consumers use the canonical platform opener already used by repository cache opening.

This prerequisite needs Root review before private-authority Source changes. It avoids broadening the allocator's existing signal-only public reservation checker and avoids exposing a private raw port callback. Inside the cache, the supplied allocation is separately checked as genuinely issued at its exact `allocationControl`, rechecked through its own real captured owner, required to back the same authenticated parent root/envelope, and `allocationControl -> control` must be a genuine private refinement. Neither raw signal equality nor policy text supplies that relationship.

## Dependency Direction and Finite Modules

Let `P` be framework Process and `S` be the existing repository Rust state directory.

New single implementation under `P/📦️artifacts/💾️cache`:

- `🧬️schema/🔣️.json`: closed generic policy, binding/intent, clean/dirty/retiring catalog, observations, pressure and receiver events. All current entry/depth/path/descriptor/output/pending bounds remain ceilings.
- `⚙️policy/🟦️.ts`: bounded schema admission and private primitive copy/freeze; generic capacity/namespace values are captured once.
- `📋️model/🟦️.ts` and `📋️model/📨️receiver/🟦️.ts`: move the current finite selection, dirty/clean commit, recoverable exact-root tombstones, deterministic clean pressure plan and accepted-work ledger. Names become Native artifact cache names, not Rust names.
- `📏️inventory/🟦️.ts`: move the same bounded no-follow/EOF/chunk, complete hardlink-group, logical/allocated byte and same-scan fingerprint algorithm, taking the captured generic policy.
- `📂️descriptors/🟦️.ts`: move the one domain-neutral lsof parser and rooted exclusion predicate. Existing repository pressure execution/tests and ticket build-maintenance import the new actual owner; no alias remains at the old path.
- `📍️layout/🟦️.ts`: canonical relative policy layout beneath the authenticated parent. It validates literal names/segments before effects and preserves existing production paths (`cs/v`, mount `m`, `w`, `i`, `slots.json`, and the production `b/p` output subtree). No environment/raw absolute output path is accepted.
- `💾️volume/🟦️.ts`: single actual funding/binding/opener, immutable receipt/backing identity, nonblocking real Native EX, complete FD/fresh recensus, plan/mark/remove/recheck, child reopening and error-preserving receiver settlement.
- `🟦️.ts`: public opaque cache/workspace operation facade only. No public raw image, port, reopen, close or caller action callback.
- adjacent `🧪️tests/{🟦️.ts,🔣️.json}`: the moved generic corpus/oracles plus distinct small-policy DATA cases and private-lifetime negative laws.

Core depends only on existing Process Native storage, issued invocation/control, leases, owned execution, framework schema validator and system modules. It imports no Repo, Rust unit, build-mode, UI or compiler policy owner.

Rust `S/🟦️.ts` remains the actual domain extension: exact repository/native caller, prototype mode, real plan object/context/content, positive environment admission, CompilerWorkspaceCustody/global writer, actual deterministic cwd and private Rust loan. It calls the Core engine directly. `S/📍️layout` remains a real Rust projection from canonical Rust partition DATA to the Core layout. `S/🔣️policy.json` remains the exact production policy, checked against its own literal contract and Core finite-policy schema. Rust environment schema/tests remain in S. Generic schema/model/inventory/volume/receiver bodies leave S rather than being copied or reexported as compatibility wrappers. Existing callers/tests move atomically.

## Proposed Public Cache Surface

```ts
withNativeArtifactCacheV1<T>({
  parent, allocation, allocationControl, control, invocation,
  parentPolicy, policy, key, content
}, async (cache, actualControl) => T)

withNativeArtifactCacheWorkspaceV1<T>(
  cache, actualControl,
  async (facts, actualControl) => T
)
```

`parent` is the authenticated actual canonical Native image. `allocation` is the actual retained parent consumer allocation, not a descriptor or child-charge copy. Base allocation control and requested effect control remain separate exact roles. The engine owns distinct child-cache charge acquisition/recovery through the now-authenticated parent's actual port, using the existing Native allocation APIs; the supplied parent allocation does not replace or double-count that child charge. Parent journal state, durable binding/intent, allocation ID and owner/bytes must agree exactly. Unbound owner-name candidates still retain/refuse. An interruption between reserve and durable intent is not adopted by name.

Policy contains required finite byte/slot/entry/depth/path/pending/output bounds, fixed relative namespace/record names and source-authored owner identities. Core admits no unbounded values and snapshots the policy before any queued work. Rust's literal policy stays 4 GiB charge / 8 slots / 1 GiB recovery target / 8 GiB first-creation availability. A diagnostic consumer can later provide its own closed smaller policy and genuinely issued parent allocation; its actual parent capacity and its distinct real child charge must fund the complete backing plus metadata. There is no test flag, production override, alternate fake backend or claim that logical observed size enforces quota.

Opaque `cache` is private WeakMap state, single workspace claim and bounded accepted operation ledger. `facts` contains only canonical work/incremental/output workspace paths, actual inner workspace reservation and its captured policy; it contains no Native image, port or receiver release. The inner workspace allocation logic becomes one Core actual helper, with repository compiler mode/owner checks remaining a domain caller if shared with the ordinary compiler-store reservation path. No second copied allocation implementation is permitted. The Rust extension validates required cwd against the actual facts before accepting its own opaque plan-bound loan.

## Exact Lifetime and Mutation Ordering

1. Authenticate Original/control, actual parent image and real parent allocation/base/phase relationships before caller policy/key/path data is used.
2. Enter the parent's existing genuine Native operation before awaiting or creating anything; retain this operation through the entire cache result and all child-image closure. Parent-close convention is not the retention mechanism.
3. Capture closed policy/key/content, recheck real allocation, recover only exact durable child funding, apply unchanged production creation predicate, open the actual child and immediately track it in the private receiver set.
4. Under actual kernel EX + complete rooted FD + fresh same-scan census, verify complete catalog/slot namespace and consume the finite clean pressure plan. Preserve dirty/unknown/malformed/replaced-root refusal, preeffect sequence refusal, and recoverable retiring dev/ino/digest tombstones.
5. Persist producer dirty state before any transient removal/writes. Hold all actual image receivers and real inner workspace reservation; enter a genuine requested child for the accepted callback and Rust loan. Every task uses the returned actual phase, never a sampled/rebased timeout or rebound reservation.
6. Close births and drain private plus canonical descendants while the body remains live. Preserve body failure including thrown `undefined` before accepted/close failures. Only a successful full settled producer reaches the existing exact final EX/FD/recensus and clean commit.
7. Drain/close every accepted receiver, including failed reopen/admission receivers, before leaving the parent Native operation. Replacements stay synchronously exclusive; copied/foreign/late/overlap acceptance refuses before effects.

A small real diagnostic consumer will also need a closed exact whole-cache teardown for its own child/charge after all accepted work/receivers settle, using existing canonical Native retirement rather than `rm` of an image path. That teardown must reauthenticate durable binding and actual child backing/receipt plus full maintenance guards. It is a distinct explicit owning-consumer retirement operation, not a dirty-slot pressure bypass. Its exact Source/qualification will be reviewed before any physical fixture; this plan does not authorize it or claim it implemented.

## Implementation and Verification Order After Review

1. Schema/corpus/Source ownership RED first: generic policy/corpus in Core, no Core imports Repo, no duplicate generic bodies or raw receiver escape. Preserve all prior relation cases and exact production constants.
2. Add the minimal canonical platform retained-parent helper with copied/foreign/control/after-close negative Source/actual-control laws. No physical positive claim yet; no fake image or structural provider may satisfy it.
3. Move pure model/receiver/inventory/descriptor algorithm once, update actual users and imports, then move actual volume lifecycle and replace the Rust volume call with the genuine Core engine plus domain checks.
4. Test both real production-policy DATA and separately authored tiny-policy DATA against Ajv + independent SQLite ordered-prefix/tombstone/funding arithmetic. Keep actual ordinary-file Python/SHA256/SQLite full census and genuine Original child/error/undefined/control laws. No physical image needed for these claims.
5. Reuse existing preauthored build-mode state test/types routes, preserving strict roots. Run Core and Rust received test files in separate owned Bun children when both require their one external receive; do not weaken the once-only receiver. No new permanent executable filename or human command is needed. Existing launch catalogs remain intact.
6. Freeze exact Source/log hashes and obtain independent review. Only then request a separately funded small physical diagnostics window. That future operation must report actual small image/allocation facts and never relabel them production 4 GiB/8 GiB qualification.

No physical operation is authorized now. Complete production state funding remains unmet at the last observed outer availability; quotas/floors/CPU/global writer/exclusivity and release's separate containment frontier are unchanged.

## Authoritative Factory Handoff Correction

Root identified two concrete gaps in the initial API description. Actual budget opening returns a frozen spread wrapper whose `close` retains a separate lease-image dependency. That wrapper is not the actual object returned by the canonical Native opener and must never be enrolled into its private image map as an alias. Also, `RepositoryCompilerStoreV1` does not contain a Native image; its `storage.backing.root` is DATA and cannot reconstruct one.

The traced actual chain is:

1. `budget/opening.openRepositoryCachePartitionV1`: real lease partition -> actual `openNativePlatformStorageV1` result `admitted` -> frozen public partition wrapper with dependency-aware close.
2. `compiler.withRepositoryCompilerStoreOwnedV1`: opens that Rust partition wrapper, issues the actual compiler reservation, constructs its frozen base store, then enters `withCompilerWorkspaceLifetimeV1` and its canonical requested body.
3. Public `withRepositoryCompilerStoreV1`: inside the retained toolchain callback, constructs another frozen full store with the genuine toolchain field. Both actual store creations must be privately associated with the SAME original compiler owner; a spread supplied by a caller must not be accepted.
4. Current Rust state opens a second partition by root; that reopening is removed. The real held partition is handed to Core through the owners below, without adding a raw image field to Store or Rust facts.

### Closed Partition Operation

The existing budget opener keeps a PRIVATE WeakMap keyed only by the exact wrapper it creates, recording actual `admitted`, the actual lease-image dependency, workspace/partition identity, exact opening control, live/accepting state and bounded pending/frame ledger. Entries are populated only at this existing factory. There is no public `register`, lookup-by-root, wrapper adoption or constructor accepting a caller image.

Its new operation is CLOSED to the Core artifact-cache effect:

```ts
withRepositoryCachePartitionArtifactCacheV1<T>(
  partition,
  { allocation, allocationControl, control, invocation, parentPolicy,
    policy, key, content },
  body: (cache, actualControl) => Promise<T>
): Promise<T>
```

It resolves only the private wrapper identity, authenticates original/control/refinement and actual allocation/root relationships, captures request DATA, and synchronously enrolls before the first await. Internally it invokes the Core engine with the private real `admitted`, never the wrapper. The public callback receives only the Core opaque cache and actual phase. It cannot obtain a raw Native port, close or maintenance capability. This is an owner-specific effect entry, not a generic trusted image callback.

Wrapper `close` stops top births, joins already accepted operations/canonical children while opening authority remains live, then closes the actual Native image and finally the retained lease image. Accepted nested frames may finish; returned/after-close references refuse. Close from its own accepted frame refuses rather than deadlocking. A bounded pending ledger preserves body/accepted/close errors, including thrown `undefined`. Actual Native operation retention remains nested inside this partition ledger; there is no reliance on a promise to keep a bare image alive.

### Closed Compiler Operation

The existing compiler factory keeps a PRIVATE store identity map. The base store is recorded where it is created, and the final toolchain-bearing store is recorded where that exact factory creates it. Both keys refer to one private owner containing actual partition wrapper, storage, base control, workspace/mode and actual received Original. The owner lives only through the existing `withCompilerWorkspaceLifetimeV1` plus inner canonical body drainage. No extra raw image/port field is added to either public store.

```ts
withRepositoryCompilerArtifactCacheV1<T>(
  store, invocation, phaseControl, { policy, key, content },
  body: (cache, actualControl) => Promise<T>
): Promise<T>
```

This authenticates the exact privately created store/Original/allocation/base/control association and existing `requireCompilerWorkspaceCustodyV1`. It enrolls the complete call through the existing `withCompilerWorkspaceWriteV1` owner, then calls the partition's CLOSED cache operation using the privately held partition. It does not reconstruct a Native image from allocation fields. Copied stores, foreign owner/phase, after-close stores and inconsistent fields refuse before effects. The returned actual phase is threaded into Core and the private Rust loan, never replaced with a root control. Existing CPU/global writer/physical-floor custody remains authoritative.

The Rust extension calls this compiler operation directly; its old lower volume owner is removed during the extraction rather than kept as an alias. The Core exact-image map accepts only the unwrapped actual Native result created by its existing canonical opener. Neither repository owner can enroll another image or populate that map.

### Handoff Proof Scope and Order

Before moving bodies, add Source AST/closed relation corpus REDs for actual wrapper-to-underlying handoff, private-map writes exclusively in the two canonical factories, full-store association, copied/foreign/mutated/after-close refusal predicates, synchronous pending acceptance, drain-before-real-image-before-lease close, and actual carried-phase propagation. SQLite independently computes the language-neutral membership/lifetime relation. Genuine received-Original negative laws exercise invented/copy-shaped handles only as REFUSED inputs before any Native effect; they are never accepted fake providers. A real positive copied/foreign Native handle and physical closing interleave require the later separately authorized small-image cohort and will not be claimed by Source tests.

Implement the two owner seams and the minimal Core exact-image operation as one coherent handoff checkpoint, then move the generic bodies. No public registrar, structural image alias, free effect constructor, reservation-checker widening or raw Native field in Rust facts is introduced. Existing generic consumer APIs remain required; there is no compatibility overload.

Additional exact Source roster for this handoff: canonical Native `storage/platform/🟦.ts`; repository `budget/opening/🟦.ts`; repository `compiler/🟦.ts`; adjacent owner Source/normal-file-control tests; existing state test route. The Native borrower/cleanup/lease primitive and public reservation checker semantics are not edited. Parent/child receiver close order remains inside the genuine carried phase, before that phase settles.
