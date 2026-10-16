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

## Implemented Frozen Source Checkpoint

This section supersedes the proposed implementation details above where they differ. The extraction is now implemented. Core owns the sole generic policy/model/receiver/inventory/descriptor/layout/volume implementation; the old generic files were removed without compatibility exports. Rust retains its plan/context/content/environment/cwd checks, CompilerWorkspaceCustody and opaque Rust loan. Production remains 4 GiB child charge, eight slots, 1 GiB pressure recovery target and 8 GiB initial parent availability. Release write containment remains a separate frontier.

The actual wrapper handoff is owner-scoped. Budget opening maps only its factory-created frozen wrapper to the real canonical image and its lease dependency. Compiler construction maps both the base store and the final toolchain-bearing store to that same genuine retained owner. The two closed operations capture policy/key/content before deferred work and pass the real image to Core internally; neither public callback exposes it. Copied, invented, foreign and closed owner references refuse. The Native platform map is populated solely by its actual backend opener, captures opening options before the backend await, and returns the same actual frozen image and port rather than accepting an alias.

Core separately authenticates the real parent allocation at its exact allocation control. A new read-only policy-fact API captures immutable policy only in the actual allocator factory; it does not widen the existing reservation checker or grant authority. Core compares every captured parent policy field, requires the policy floor to equal the actual allocation maximum workspace, enters the authentic Native operation and awaits allocation.recheck before effects. Policy DATA is independently copied from own plain data descriptors, bounded and frozen before any await; accessor/Proxy/unsupported shape inputs refuse.

The moved cache workspace uses a bounded Core Workspace binding and actual Native allocator. It does not import the repository mode-specific compiler reservation owner. The ordinary repository compiler reservation remains its separate domain lifecycle; there is no second copy of the extracted cache volume/model/inventory logic. Durable binding/intent ambiguity and unbound owner-name allocation candidates retain/refuse. Existing clean-only pressure, dirty retention, exact retiring tombstones, descriptor exclusion and fresh recensus predicates remain intact.

The private receiver now enrolls the exact returned promise before invoking work, rejects same-frame close and synchronously refuses overlapping replacement. Closure stops births, drains accepted descendants and attempts every owned receiver close. Partition admission and close preserve ordered primary Error or thrown undefined while still attempting image and lease dependency close. A callback failure is never replaced by finally-close failure. The borrowed parent is retained by its real Native operation, not put in the owned-child receiver-close set.

## Executed Evidence and Strict Boundary

All commands used the existing preauthored build-mode state test/types route. Core and Rust received laws ran in separate owned Bun children. No Native/image/compiler/Rust/Cargo/download/SDK/retirement effect ran.

| Evidence | Actual result |
| --- | --- |
| `core-cache-handoff-red.log` | Genuine missing owner handoff Source RED, 22 pass / 1 fail / 289 assertions. |
| `core-cache-ledger-reentrance-red.log` | Genuine pre-enrollment own-frame close RED, 0 pass / 1 fail / 1 assertion. |
| `core-cache-parent-freeze-red.log` | Genuine actual platform freeze Source RED, 22 pass / 1 fail / 313 assertions in the Rust integration child. |
| `core-cache-source-prototype1.log` | Intermediate moved-name AST expectations failed, 20 pass / 3 fail / 297 assertions; corrected to actual Core owners and retained Rust domain names. |
| `core-cache-final-test-prototype.log` | Core 7/55 plus Rust integration 23/316, all pass: total 30 tests / 371 assertions. |
| `core-cache-final-test-release.log` | Same 30 tests / 371 assertions, all pass. |
| `core-cache-types-prototype1.log` | Actual moved descriptor import missing; corrected at its real consumer. |
| `core-cache-types-prototype2.log` | Expanded actual Source roots strict exit 0 at that preceding epoch. |
| `core-cache-final-types-prototype.log`, `core-cache-final-types-release.log` | Both exit 1 exclusively at moving peer component-pass line 122 (`options`/`plan` unknown and consequent parameter inference), no owned extraction diagnostic. CPU acknowledged and corrected the misplaced preflight afterward. No unexecuted successor is called green. |

The tests include schema/Ajv and independent SQLite policy/ordered-prefix/transition outputs, actual ordinary-file Python SHA256/SQLite census, bounded policy mutation/accessor refusal, genuine received control negatives, receiver success/Error/undefined and admission-order drainage. Source AST laws check actual private factory associations, same image identity, real allocation policy/recheck/floor ordering and absence of Core-to-repository reverse imports. They do not qualify Native physical admission or platform freezing at runtime. Every original strict root and flag is retained. Root will collect integration strict after the peer lanes settle; no unchanged rerun was launched.

## Native Freeze Source Census

`core-cache-native-freeze-census.json` is a bounded read-only census of 427 TypeScript files under Native storage and repository caching (maximum 4096 files, 2 MiB per file). Four public-write pattern matches occur only in SDK derivative-index tests that instrument ordinary filesystem FileHandle.close; none is a Native image or port write. Both actual physical constructors were inspected: macOS persistent construction in cleanup/command and the shared fixed Linux/Windows receiver. They return ordinary data/method object literals, without field accessor syntax. Lifecycle mutation occurs in private receiver/closure state (`stage`, held guard, maintenance/closing state), rather than reassigning public image/port members. The three platform forwarders and canonical platform owner likewise have no such accessor definitions. Their six hashes are recorded in the census. This establishes Source compatibility of freezing the same actual object; no physical or cross-platform runtime qualification is inferred.

## Next Separately Funded Consumer and Whole-Cache Retirement Plan

The authored small policy is genuine different consumer DATA: 64 MiB charge, 16 MiB parent metadata allowance, 48 MiB backing, 47 MiB inner filesystem, two slots, 4 MiB recovery target and 16 MiB initial parent floor. It exercises the same engine/model, not a production override. It has not created a volume. An actual parent allocation must privately capture a 16 MiB maximum workspace to satisfy the floor equality, and its real journal must fund the distinct child cache charge. The canonical Diagnostics whole-image allocation cannot simply be relabeled that smaller allocation or double-charged.

The actual macOS child creator additionally requires backing plus 64 MiB available before creation. A proposed 128 MiB parent with a 16 MiB actual consumer allocation and distinct 64 MiB child charge could satisfy the logical envelope; creation still needs at least 112 MiB actual parent free for the 48 MiB child and may honestly refuse after formatting overhead. Parent plus child known backing is 176 MiB. Therefore it does not fit the earlier 128 MiB total private-image fixture authorization. A separately reviewed finite 192 MiB outer Diagnostics scratch quote is a possible next consumer, not existing approval or a global cap increase. Alternatively an actual API factoring that uses an already funded fixed child could avoid the nested envelope; that requires its own custody design. No fixture will run by silently weakening either capacity predicate.

Before either physical fixture, implement and review a closed whole-cache retirement operation under the same privately authenticated parent owner. It must resolve the exact durable Binding/Intent, allocation ID and immutable backing/receipt identity without creating a missing cache. It must stop births, drain real producer and receiver descendants, obtain nonblocking actual child EX, complete rooted FD exclusion and fresh unchanged census, and preserve unknown/unbound/corrupt/ambiguous candidates. Dirty producer state is not inferred abandoned from PID/name. Whole-cache owning teardown is distinct from clean-slot pressure and needs actual whole-volume ownership/exclusion.

A durable whole-cache retirement transition is still missing. The proposed closed state order is bound -> retiring-content -> retiring-image -> retired, with the existing real child charge retained through all interrupted/uncertain stages. Exact inner workspace retirement and known owned content removal precede canonical Native backing retirement. Only confirmed physical backing retirement may acknowledge the parent charge retirement. The EX/gap/reopen and inner journal ordering must be qualified rather than represented by a boolean receipt. No public raw path/callback/argv retirement surface is proposed. Failures including undefined retain primary order, exact charged unresolved state and the same genuinely prefunded deadline; no new Original or cleanup control is minted.

This teardown Source transaction, independent custody review and an explicit finite actual Diagnostics funding quote are prerequisites to the small physical lifecycle/pressure cohort. Production 4 GiB/8 GiB remains unqualified and currently unfunded at the last observational outer availability of about 4.325 GiB. There is no physical operation authorized by this handback. All own handles are terminal.

## Frozen File Roster and SHA256

New Core files: 11. Updated files: 15. Removed generic implementation files: 5. Maximum absolute roster path is 150 UTF-16 code units (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json`), within 256. No editor catalog or new human command was needed; existing state commands remain registered in both catalogs.

| Source path | SHA256 |
| --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/⚙️policy/🟦️.ts` | `a9d019290af89fae77ab9197ccfa47792e53320606ae719867cd7dc8a12aed0f` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` | `858aad39aff33ff87559b7fc3c8bf75e34cf6a08fccf72b2a615817247be904d` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts` | `3fb031bd6043e5d61e07ff03ecadf13f9bb41519694d8397d51f43c3d4757660` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📋️model/📨️receiver/🟦️.ts` | `381445e40e0cdfc8593aa6861fc1e74fda87a40759920dfb77cb0d4bea2b3bf0` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📋️model/🟦️.ts` | `b207d1ce4637de204007e79145dfa70800811fc8f1d7ee35291fe268e3a4f71c` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📍️layout/🟦️.ts` | `8ab38c5b6278d12e73f32e218d80e5a1d44e310875361b697fe779325aaf292b` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📏️inventory/🟦️.ts` | `0b3508cf2da2777d6a3cd547c1d42de5007735669edb80497caac1ced97cc3f5` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🟦️.ts` | `d62d4812b9ef5a1590fcf883252afcb17e10f35d026eeaf459632370e1f88e83` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🔣️.json` | `f1ccf2f4eafd808827f4cb5728962498312ad0d7a8dbb5959a3182dc8e9d8641` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | `f4af5cda030497229d63ed52a7b7fa207276b829a1cadbcefdb8493e59c1363e` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧬️schema/🔣️.json` | `35a61ad6e719034792ccaa34f408701df85025b06d527e73042ccdde799525b2` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` | `2826304c1b8ba5c92b3323d4cae955bb893f5b1609b723123b33cfd2dab9da45` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` | `6b02ba012f0e6ff9367ae52c8a945faae77bcaf724ecbcc4d28be64e04cbbd68` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` | `3f371803307c563169f7e9227140e19dea5df86764d6fd5540e0bd33ee2ba2d1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` | `9286dd571434492905bd29e313c896f00ca57080c7e28f8786c9eb4558e6dec2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` | `bd762245b3fe0ceef54bc30bd220bfbf52ce67a7733de0294f7e91c578c11840` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/⚙️policy/🟦️.ts` | `94332ca35a21aca497306bd5b5f44d5d92e946cb172f3378a0d5c51eaf73697c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📍️layout/🟦️.ts` | `fa5a86b0ee8853977c0f95096818cb94d4929ab28a1c192c9a6a24218493f3ac` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🔣️policy.json` | `e4225e951f0017352d472a190cc3a10c165d743b2c14fc226587a01e595f027c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json` | `075f6c9fd47d6671abc64a3e3ba36247c56725826220d8983dd371e63cd2ba8d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | `673c37fec32f18aa9a93e12bea95d68e07c26c48a52adc2f97a7786ab9bca17b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🔣️.json` | `a86cd0c9b36e85b5d669a951326227e1d96f433323adcfb6fb0bbb36c9eba599` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` | `86a056c785135f92bcb45a0b8dddb8c37d109af079958bc27cfd904c1a129e98` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/🟦️.ts` | `7f4df8ed88074ca373c74677ddbd868eca5363284a18d34643614da3bd3cf436` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-maintenance/📜️script.ts` | `954a90ebd6d3d78648e3da6d0468e520a8ab6c67cb7644b4a3c919d13ed40d65` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | `538ca1c93fd18dfdee586d778b3167990dff147fd09729950ed45f2901ddb50e` |

Removed, without alias/reexport:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/📨️receiver/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📏️inventory/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/📂️descriptors/🟦️.ts`

## Retained Generated Evidence SHA256

| Generated file | SHA256 |
| --- | --- |
| `core-cache-final-test-prototype.log` | `306bcdc379f584b363c6e031802dcf8ca7e02bcc909ec17e6caee94cbe7d3774` |
| `core-cache-final-test-release.log` | `ecfdf02c74f5e8bc897f9b9a805f78514d6b51055fede46f12a69fffab0ca4e3` |
| `core-cache-final-types-prototype.log` | `8a2b2c97bc7a6d146197c10d635bbfe8682fbe1061315439686b65b395182118` |
| `core-cache-final-types-release.log` | `8a2b2c97bc7a6d146197c10d635bbfe8682fbe1061315439686b65b395182118` |
| `core-cache-handoff-red.log` | `6e31802f0a4fc8c21c89e529de097a325fe1237c7df70ae240604ecdc7abb9a7` |
| `core-cache-ledger-reentrance-red.log` | `d79fb49dc68ba35226b393c8127d2ce595383175767384d9ebd7fc4df63285c1` |
| `core-cache-parent-freeze-red.log` | `b6332fbe0c3123d33ab6cedf6e34faaed8fc491fb181d679b518fe7ff4ecd5dc` |
| `core-cache-source-prototype1.log` | `c55461d7df8c95340910abd49805494c0489337a19caba08cd556a5f399a267f` |
| `core-cache-source-prototype2.log` | `781f1c4eefda76ea21d570be89081a51bb31b14b93c0b378f68343a99591de44` |
| `core-cache-types-prototype1.log` | `78afefae873098a04c86b71b0b0995b9f8c3703e73e55bc799f26dd3270489b0` |
| `core-cache-types-prototype2.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `core-cache-native-freeze-census.json` | `c3b5b5565e0c13345d892d4344edbd888f2fcd5111638fd17c49878966cba2a9` |
