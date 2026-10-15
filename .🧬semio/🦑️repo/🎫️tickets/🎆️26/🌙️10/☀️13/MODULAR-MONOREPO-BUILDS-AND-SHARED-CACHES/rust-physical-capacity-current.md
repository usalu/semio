# Rust Physical Capacity and Retirement

The Job public release operation 24935 genuinely failed with ENOSPC. Ingress confirmed every native and lock operation terminal before this investigation; no retry was launched. The fixed Rust image remains 16 GiB. No artifact, historic directory, database, or reader has been deleted by this investigation.

## Current Physical Observation

Read-only `df` observed device `/dev/disk4s2`, 16,440,280 KiB filesystem blocks, 16,440,216 KiB used and 64 KiB available. A complete `du` top-level observation measured 16,231,832 KiB allocated below the mount; filesystem metadata accounts for a further difference. Principal allocated roots were Cargo 8,733,832 KiB, Nx 5,550,652 KiB, compact compiler build roots 1,749,044 KiB, published artifacts 184,672 KiB, and compact target roots 2,216 KiB. These are allocated filesystem sizes, not the storage journal's logical reservations.

Cargo's old prototype/build subtree occupies 5,947,260 KiB, old release/build 534,600 KiB, the currently authored shared/build root 566,200 KiB, and registry home 1,620,284 KiB. Current compact build roots occupy 1,565,004 KiB for prototype and 184,040 KiB for release. Nx contains 2,166,476 KiB graph data, 1,001,700 KiB native material, 1,576,708 KiB temporary material and 805,752 KiB artifacts. Path recognition or apparent inactivity does not establish retirement authority for those namespaces.

## Admission Defect

`repositoryCompilerReservationV1` retains one `compiler-workspace` allocation of 4,294,967,296 bytes for each mode. The two exact charge receipts still exist. Recovery authenticates the journal allocation and physical backing but does not census the mode tree, test physical availability, reserve new compiler growth, or enforce an individual 4 GiB filesystem quota. Thus two 4 GiB logical charges are compatible with nearly 16 GiB physical usage; the image cap correctly stops growth at 16 GiB but does not preserve compiler runtime headroom. The documentation phrase “preserving headroom” is stronger than the implemented physical guarantee.

A sampled free-space check alone is insufficient: multiple concurrent compilers can each observe the same free blocks, and an accepted Cargo child can grow beyond a logical estimate. A corrected checkpoint must distinguish actual finite filesystem enforcement, concurrent admission custody, accepted write demand, and retained outputs. Sampling must not be reported as a hard per-cohort quota.

## Existing Retirement Seam

The permanent `InactiveCargoRetirementScript` derives only `.cargo/config.toml` shared build/target roots. It does not select the current compact layout owner. Its schema default retains 85,899,345,920 bytes, targets 25,769,803,776 available bytes, and permits 34,359,738,368 deletion bytes: host-era values which cannot fit the 16 GiB image. Existing retirement custody itself is materially useful: exact store-exclusive leases, complete process and descriptor capture, physical Cargo profile locks, a rescan, and path-by-path deletion. Those safeguards must remain. Unknown or historic old-layout paths are not authorized merely because they are large.

The existing registered preview Original 81235 refused before scanning/deleting because a previous fixed `processes.stdout` capture already existed. Retirement now creates a distinct operation evidence directory; it never overwrites that prior capture.

Root has coordinated a shared compiler/retirement Source edit window. Native/lock writers stay held. Material-report tests remain pending and unrelated to this capacity repair.

## Implemented Physical Admission

The new schema-owned compiler capacity leaf distinguishes initial free headroom from the hard enclosing image envelope. Actual compiler-workspace reservations must be privately issued and belong to a live workspace callback. Each accepted write job registers before asynchronous admission. The workspace closes births, drains accepted operations and propagates failures before releasing the actual image reader. Writer exclusion likewise drains accepted nested operations; late descendants refuse. No environment marker grants writer admission.

The cold writer takes the exact shared Rust-root resource exclusively, observes current physical available blocks under that custody, and retains custody through the actual command, consumer and failure cleanup. Its required initial free bytes equal the actual issued workspace charge (4 GiB in production). The 16 GiB filesystem remains the hard envelope. This is deliberately **not** a physical 4 GiB per-mode quota or a guarantee that Cargo cannot consume more than its initial headroom.

Generic compiler-store callbacks remain readers. A broad writer around them genuinely creates parent–child self-wait: selected preparation and compiler fixture parents can await a child which opens its own compiler store. The implementation therefore places writers at cold execution boundaries: registry fetch/resolution, selected unit cold compilation/publication, repository Cargo test/build consumers, the admitted launcher body, exact-law build input consumer and the relay after its child preparation. Terminal synchronization currently has a broader writer around its run; narrowing it to actual Cargo execution while retaining ready reads is an explicit post-window follow-up.

Current production integration includes compiler workspace lifetime, Cargo dependency fetch/resolution, repository build/tests, exact laws, executable tests, admitted Cargo launcher, selected unit producer, terminal preparation and Cargo relay. Metadata and arbitrary external/unmanaged writer coverage must not be inferred from this list. Source inspection and subsequent ordinary runtime qualification remain necessary for each ingress.

## Native Artifact Publication Source

The current Nx fixed-cache owner retains output reader or restore-writer custody before inspecting copy demand. Publication demand consists of the measured explicit output trees, a schema-owned 128 MiB terminal ceiling and 1 MiB Native metadata allowance, bounded by the existing 1 GiB artifact limit. The patch refuses terminal output above the same ceiling before Native put. Deleted-output restore measures the actual supplied cached source tree; pure cache-reader scopes have no restore source and retain concurrency. Product lifetime and output leases remain with their existing separately funded product owner; current restore admission conservatively retains the Rust temporary-write allowance as well.

Native cache operations already holding output/cache locks request the common writer **without waiting**. This prevents artifact-reader → writer waiting from opposing compiler writer → artifact-publication exclusion. Existing Native retry can retry an honest refused publication after those locks release. Source assertions/types and the generic physical nonblocking law are qualified; **a newly captured SDK's actual Native put/deleted-restore/failure regression is still pending**. The prior 8d0 Native and ordinary successes do not qualify these new Source bytes.

## Actual TDD and Private Physical Proof

The new language-neutral observation corpus genuinely failed first with the missing capacity API, then passed independent Ajv and SQLite comparisons. The physical fixture creates a real 32 MiB fixed image inside the issued diagnostics backing; it does not fake a storage port or use the full Rust image as disposable test storage.

Final prototype Original 58004 passed two tests / 38 assertions in 3.7 seconds. Release Original 34421 passed the same two / 38 in 4.2 seconds. Expanded ES2022 strict/noUnchecked Original 33354 exited zero in 2.7 seconds, including the actual launcher, relay, unit entry, terminal preparation, compiler lifetime, pruning and current graph/Nx owner. Earlier expanded strict exposed and corrected own import/shebang/callback mistakes. A separate graph coedit's missing existing declaration and array narrowing were repaired by Root; the final expanded check includes those actual current bytes.

The actual laws establish:

- Independent system Python SQLite observes the real exclusive writer transaction while held.
- Two operations recovering one genuine 24 MiB compiler charge cannot both spend the same free blocks: the second queues, remeasures after the first's 12 MiB growth and refuses before its callback has any effect.
- An already-locking artifact operation using nonblocking writer admission refuses immediately without a publication effect.
- A real filesystem ENOSPC leaves no publication; exact owned failure output cleanup finishes before another writer acquires.
- A caller returning before an accepted writer finishes still retains its workspace/image lifetime until that writer settles; a late descendant refuses.
- Compact-root retirement waits for a genuine shared Cargo store reader, then obtains real physical profile locks, complete process/descriptor evidence and a rescan. Only its owned unit is removed; an independent retained artifact remains.

Private fixed images and exact allocations were retired after their users settled. Both editor catalogs and private target routes contain the new source and strict gates. No permanent new path exceeded 256 UTF-16 units in the read-only path check.

## Actual Canonical Retirement

Both operations used the existing permanent retirement entry through actual private Nx, held the authentic Rust image, acquired the same global writer before the matching Cargo store exclusions, obtained physical profile locks and complete descriptor evidence, and rescanned before deletion. Current compact roots come from the canonical compiler layout, not an old path alias. Build units and finalized incremental units are included; working/held units remain excluded. The policy's retained, target-available and maximum-deletion values are each 4 GiB and fit the fixed image.

| Mode | Preview | Retirement | Units Removed | Logical Bytes Removed | Available Before | Available After |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| Prototype | 14415, green 2.0 s | 66288, green 15.2 s | 260 | 1,536,616,789 | 0 | 1,585,156,096 |
| Release | 74595, green | 83384, green 2.7 s | 146 | 184,648,742 | 1,585,156,096 | 1,773,330,432 |

Prototype retained 14,650,012 logical bytes in its protected working unit. Release selected all 146 admitted finalized units. Combined reclamation removed 406 units and 1,721,265,531 logical bytes. Final independent `df` observed 1,731,728 KiB available (1,773,330,432 bytes). Physical and logical measurements are deliberately reported separately.

The 4 GiB initial headroom requirement remains unmet. **New compilation must still refuse.** Neither a cap increase nor an unknown-tree cleanup is justified by the shortfall.

## Historical Authority Frontier

No old `cargo/{prototype,release}/build`, Cargo home, Nx graph/native/temporary/artifact tree, published executable or live output was removed. Read-only independent review located a retained compiler program at `rc/6013cbc0b38262821df16abaedaefa0dc6514fd70fd09bac552080fd2cbad94c/entry.mjs` matching its recorded program SHA256 `1df6e2670be27a00709d3ba0f9f34df99891258a4034e622a1834e99e8b34c5b`. Its compiled body contains the exact old mode/build layout and compiler charge protocol. That is stronger than a directory-name guess, but this bundle does not contain the old Cargo store/physical-lock protocol. Reviewed SDK authorizer manifests contain path/digest rows rather than historical raw Source bytes and do not alone provide the missing protocol.

Further old-data retirement therefore needs the original charge plus authenticated retained code or SHA-matched historical Source proving the precise old roots and matching locking behavior, including exclusion of a future opener. The current complete descriptor census establishes current users only. Physical lock/profile inodes and unrelated children must remain intact. Missing evidence is not permission to adopt or delete historical material.

## Current Handoff

All own Originals are terminal. Production Source is held stable for Storage's independent non-Rust diagnostics/baseline proof window. Canonical namespace enrollment remains held. No new SDK was published and no ordinary seven-phase pair or Job native retry was launched during this repair. Pending work is the terminal synchronization ready-read narrowing, any final lifetime audit corrections, new exact SDK/Native publication/restore qualification, and additional authenticated reclamation sufficient for the compiler initial headroom requirement. Material-report correction tests remain unexecuted.


## Read-Only Enclosing Image Provenance Follow-Up

No managed operation, maintenance transaction, test, SDK publication or deletion was started during Storage's independent baseline proof window. The observations below were ordinary read-only file/hash/SQLite-identity inspections. They are a proof plan, not an executed retirement grant.

The retained `rc/6013cbc0b38262821df16abaedaefa0dc6514fd70fd09bac552080fd2cbad94c/entry.mjs` contains more than the old layout. Its actual 136,463 program bytes hash to `1df6e2670be27a00709d3ba0f9f34df99891258a4034e622a1834e99e8b34c5b`. Recomputing SHA256 of the receipt's exact Source digest plus compiler facts yields its recorded Source identity `76152b8686d1ac14fe53eaf3027e62f015a633087791b3f3ab45a1539c1280b1`; recomputing the Source/program identity yields the exact directory name. These are content identities; the read-only observation did not itself retain a code-reader lease.

The compiled generic compiler-store function `o4` at byte 89,344 opens the Rust partition, derives `cargo/<mode>/build`, captures, temporary and target directories, awaits the complete consumer callback, and closes that partition in `finally`. Its partition opener `t0` at byte 72,658 retains the enclosing leases image and opens the selected Rust image with the canonical `.cache/b/l/m/data` lease directory. Its partition policy and strict-storage policy agree on `.🧬semio/🦑️repo/⚡️cache/rust/fixed`, the 16 GiB envelope and the original mode charge.

The actual compiled Mac provider `j4` at byte 53,696 acquires shared custody on `native-fixed-image:` plus the resolved original image directory before it returns the mounted backing. Its close releases that reader. Its exclusive-maintenance method releases only its own reader, takes EX on the same resource, rechecks the original image/receipt/mount identities, retains EX through the complete callback, then releases it. New openers request the same SH before accessing the returned backing. The old compiled source therefore establishes a concrete enclosing-image exclusion route for this authenticated compiler-store generation, including future managed openers, rather than a descriptor snapshot alone.

The historical receipt's raw lease implementation hashes still match the current actual files: process `🔒️leases/🟦️.ts` is `9c30f9d5bb1ff1c0914dee648d0de3a259c68a173d709304dd424d89417202e5`, and its TypeScript package bridge is `0d5870ad0181479dd186c38b951cc2e1affd357d025e85f7c7ffdc8a312d4a06`. The compiled `X0` uses SHA256(resource) under the resolved lease directory, and the actual lease implementation retains rollback-journal SQLite `BEGIN` with an identity read for SH versus `BEGIN EXCLUSIVE` for EX. The exact original guard resolves to `.cache/b/l/m/data/0b26e7bbbccda603f096482062c8797d11795406b3bb43e8ef42f42a1e86f99e.sqlite`. A read-only database query returned version 1 and that exact image resource. The image receipt still reports device 16777232, inode 335960199 and size 17,178,820,608. A contemporaneous `lsof` of this exact guard returned no holder; that transient census is not permission to retire and does not justify stopping any process.

The stronger route does not establish that arbitrary historic commands, old Nx temporary/native trees, or every file below an old root used this generation. No old Cargo execution adapter or historical compatibility path should be introduced. The recognized Cargo-unit scanner, exact physical Cargo locks and complete descriptor recensus remain necessary. The old Cargo per-store guard Source is still absent from this direct-unit bundle; enclosing image exclusion would replace that missing *future managed opener* premise only after the code/image custody chain is authenticated inside the real maintenance operation.

### Proposed Required Operation Order

1. Hold the actual tools image and the original `rust-code:<absolute code directory>` SH before the first authoritative code/receipt read. Validate no-follow exact two-file inventory, single links, closed receipt bounds, full identity derivations, program size/hash and immutable source/compiler facts. Hold this reader through maintenance; do not execute historical compiler code.
2. Open the authentic current Rust image under a genuine finite maintenance Original and verify its original receipt/physical backing identity. Revalidate the original prototype/release committed compiler charge and the exact old root projection demonstrated by the authenticated program. Refuse unknown layout or changed content.
3. Enter that image's existing `withExclusiveMaintenance` without retaining a second reader of the same image in the parent scope. The operation must acquire the original exact image EX; it must neither wait under a compiler-store child nor detach/reopen the image. Active managed builds/readers are protected by that EX admission. If held, refuse or wait only under the existing finite control; do not kill a reader or daemon.
4. While image EX remains held, invoke the existing recognized-unit retirement machinery on only the proven mode build root and its exact related target root. Retain the actual original Cargo physical lock/profile inodes, acquire those physical locks, capture complete process/descriptors, rescan every selected unit, and remove only admitted unchanged unit paths. Preserve unrelated children, journal/charge/lock/profile inodes and every unrecognized tree.
5. Recheck the actual image, charge, code and post-retirement physical free blocks before releasing image EX and code SH. Availability below the required initial 4 GiB continues to refuse compilation. Test actual future SH exclusion against the same original guard before canonical old-data deletion is authorized.

No daemon stop was needed or attempted: the exact image guard had no observed holder at the read-only sample. If a future operation finds a daemon, only its authenticated original service/socket shutdown protocol may be considered; a PID, age or fd census is not a stop grant. Canonical namespace enrollment remains held.


## Executed Historical Enclosing-Image Checkpoint

Storage explicitly handed back its final isolated baseline/funding window after both runtime modes and strict checks settled. The new historical reader stays in the existing compiler-code owner: actual tools image plus exact `rust-code:<directory>` SH, canonical two-file no-follow/single-link inventory, exact byte ceilings, full program/receipt hashes, complete Source manifest hash and compiler/source identity derivations. It does not execute historical code or substitute it for the current compiler. A closed Source policy binds the one audited program/receipt identity, exact original mode charges, original image/lease paths and unchanged primitive lease Source witnesses. The maintenance request accepts only mode and preview; it grants no caller-selected path, capacity or compiler version.

The receipt test's first attempted run failed on my incorrect relative import; that was not the intended behavior RED. Corrected Original 49463 then genuinely failed because `readRustUnitCodeReceiptV1` was absent, with the original two physical laws still passing. The implemented reader and neutral fixture subsequently passed independent Ajv/crypto checks. The closed historical request corpus independently rejects arbitrary roots, missing fields and unknown modes.

The actual private fixed-image law passed in prototype and release: an existing real image reader causes nonblocking image maintenance to refuse without entering the callback; after it closes, actual image EX retains a future `openNativeImageStorageV1` caller outside the body until settlement. An independent Python SQLite reader attempting `BEGIN` plus the actual identity SELECT is locked during that EX. No fabricated storage port or timeout-based success assertion is used. Both modes' four-law intermediate checkpoint passed 54 assertions; both five-law checkpoint passed 68 assertions. Final allocation-retirement assertions and final types are recorded below when terminal.

The canonical action uses the private headroom route, whose outer Original retains only tools and exact SDK readers. The ordinary repository Nx frontend must not launch this maintenance as its awaited task: it retains a Rust image reader around the child and would self-wait. The canonical action itself successfully obtains image EX; no parent Rust reader was bypassed or forcibly released.

Historical prototype preview Original 56995 completed in 7.5 s, after a first preview Original 35807 honestly refused my overbroad storage-policy object before any scan/deletion. The projection was corrected to the actual closed native-storage policy. The successful preview authenticated the exact RC/charge and original Rust image EX, scanned 1,717 recognized units, measured 6,186,882,459 logical bytes and selected 556 units totaling 2,527,062,122 logical bytes. The original physical availability was 1,773,289,472 bytes.

| Actual Historical Operation | Units Removed | Logical Bytes Removed | Physical Available Before | Physical Available After |
| --- | ---: | ---: | ---: | ---: |
| Prototype 87592, green 19.6 s | 556 | 2,527,062,122 | 1,773,289,472 | 4,243,349,504 |
| Prototype 23544, green | 22 | 51,918,665 | 4,243,316,736 | 4,297,129,984 |

The first retirement remained 51,617,792 physical bytes short of the 4 GiB requirement despite its logical plan, so no compilation was launched on that result. The second bounded operation independently reauthenticated the same code/charge/image, replanned against current statfs and retired only 22 more recognized units. The final actual free space was 4,297,129,984 bytes, 2,162,688 above the required initial 4 GiB. This small margin is explicit: admitted compiler growth may require another genuine maintenance cycle later. It is not a physical per-mode quota or a promise that a large compiler operation will fit.

Both operations retained original image EX through the existing matching physical Cargo profile locks, complete process/descriptor census, exact unit rescan and path-by-path deletion. Original profile/lock inodes, unrelated children, old release outputs, Cargo home, all Nx roots, published executables and unknown trees remain. No cap changed, PID-based decision, force detach or raw removal occurred.

Terminal synchronization is now guarded only around its actual `execute` callback; authenticated unchanged dependency readiness and version reads no longer require the cold compiler headroom grant. Workspace lifetime and every accepted workspace write recheck the actual reservation before use, including same-process writer reuse. This closes the inspected retired-allocation route without trusting the private brand alone. The existing fixed-cache hook's new graph/native storage coedit remains intact; its empty-string conditional was mechanically narrowed to `undefined` to restore TypeScript narrowing. Expanded strict Original 8183 passed after that correction; new graph/native behavior still requires fresh SDK/Native qualification.


## Fresh SDK and Graph-Only ABI / Graph4 Frontier

Final capacity/receipt/request/physical suite Originals 99953 prototype and 24076 release passed five tests / 71 assertions in 7.2 / 8.4 seconds. Final expanded strict Original 15457 exited zero. The real retired-allocation law specifically retires the issued fixture allocation inside an otherwise live workspace and verifies a write refuses before its callback has any effect.

Normal SDK setup Original 1033 exited zero and activated `9974f635170ea6f5cc346469c7024ee2a07116ac06ee964c0daffe0f976a920f`, with actual staging/final 33-hook Node preflight and bundle SHA256 `b97a5dd8a652a255cb53aca28a6646d1fa82b5ddb7fbd5de6a84b0fba636ad5b`. The new captured graph-image ABI correctly rejected the private CLI before dispatch because the private outer launcher had not supplied an authentic Native graph owner. Native put/restore was not reached.

The graph-only correction retains the actual graph partition, funded Nx graph pool and exact Native namespace image, with no Rust image reader. The received frozen owner likewise opens only that graph scope. The private launcher projects the actual watched private root and `nxBuildDependencyImplementationV1()` into the same owner before child launch. The CLI patch now guards every actual Native path; its pure `--version` branch remains outside Native setup so the normal publisher can verify its installed version. Initial private launch after this repair caught my incorrect library mjs import before any Original; correcting that private-only path required no SDK change.

Normal successor setup Original 87385 exited zero and activated `60f95aa15d05e3957d142b61b7d3a663484c180683142df4820ca2ec71de1c2a`, actual staging/final 33-hook preflight, authorizer SHA256 `b30e497b5923543b637b394e43cb0a3b502bac3b58f347eb61f1d545ab16fb2f`. Actual Native prototype Original 57816 then honestly refused at graph pre-admission, before Node/test dispatch: the 1.5 GiB fully allocated Nx graph pool plus 1 GiB mandatory headroom requires 2,684,354,560 available bytes, but optional retirement found zero recognized optional bytes and only 2,102,169,600 bytes physically available. No pool was created, capacity increased, unknown cache swept, or Native behavior claimed.

### Bounded Read-Only Graph4 Census for Root

A subsequent no-follow read-only census of `.cache/b/g/m` (device 16777243) found actual filesystem total 3,949,944,832 bytes, allocated 1,847,832,576 and available 2,102,112,256. All 18,158 encountered regular files were on that actual device; no links or unknown file kinds were reported. HFS filesystem usable blocks differ from the declared enclosing 4 GiB image envelope. The single committed graph charge remains allocation `3a51bec0ae6c1d5576c488aa0d3dc956`, owner `repository:graph-store`, 4,293,918,720 logical bytes.

| Existing Graph Path | Files | Logical Bytes | Allocated Bytes |
| --- | ---: | ---: | ---: |
| `canonical.json` | 1 | 224,637,980 | 224,641,024 |
| `authority.json` | 1 | 62,777,229 | 62,779,392 |
| `snapshots` | 135 | 479,590,721 | 479,899,648 |
| `inference` | 59 | 560,272,954 | 560,402,432 |
| `build-input-closures` | 16,155 | 334,813,823 | 368,459,776 |
| `build-lock-closures` | 1,656 | 297,699 | 6,782,976 |
| `selected` | 30 | 103,815,132 | 103,878,656 |
| `.fseventsd` | 117 | 5,717,559 | 5,967,872 |

The largest snapshots are `fa53c2c277685b941612a86ba130c62f75b7f3a6ccc5cc87b33d0bd51c0d252f.json` (224,637,980 bytes) and `8af9074d9e322568cf841e7fd05110c3a940d87a522bac60a65670c8b875d0fc.json` (224,634,972). The largest inference publications are `inference/inference/{p-39769/,}emoji-project-json.publication.json` (61,270,744 each), followed by p-11306, p-92620 and p-81654 at about 61 MiB each. These old-looking process-named paths were only measured; no age/PID/filename ownership inference was used to remove them.

The failed destination was `.cache/b/g/m/nx/cache/fixed.dmg`, before provisioning the 1.5 GiB Nx pool. The existing `prepareNxGraphCacheStorageV1` computes missing physical pool bytes plus its mandatory minimum, invokes `prepareBuildGraphOptionalHeadroomV1` before retaining graph readers, then provisions only via `withRepositoryCacheStoreMaintenanceV1` under actual parent-image EX. That is the correct exclusion seam, but it currently has no authorized bytes sufficient for this request.

Existing snapshot publication `withRepositoryBuildGraphArtifactV1` owns `snapshots/<sha>.json`; its private retention helper takes exact `build-graph:<path>` EX, authenticates full content digest and removes only unheld snapshots when aggregate retention exceeds 2 GiB. The measured 480 MiB is below that threshold, and there is no current public demand-driven snapshot retirement entry. Current optional maintenance inventories only schema-qualified leaves under `inference`; it cannot adopt the measured publications/closures/snapshots simply to satisfy the pool. Current graph-state loader owns canonical/authority and selected publications, with only transient stage cleanup, not old publication retention. No closure retirement API was found at the inspected input owner. Root owns budget and authenticated-retention reconciliation; all further Native/ordinary launches are held pending its coherent checkpoint.


## Repeatable Compact Recovery: Final Private Qualification

The current compact recovery is now qualified in both modes, independently from historical reclamation. Prototype Original 34035 passed five tests / 88 assertions in 19.43 seconds; release Original 83554 passed the same five / 88 in 34.37 seconds. Expanded strict Original 75082 exited zero and includes the generic compiler store, automatic recovery leaf, lower retirement execution, preparation entry and current Nx cache hook. These are bootstrap-prerequisite Original-owned tests, not ordinary Nx qualification.

The actual private 32 MiB image law repeatedly fills the compact recognized unit path with a 12 MiB unit against a 24 MiB initial grant. A genuine held CargoStore reader and genuine held physical profile lock each refuse immediate retirement with zero effects. After release, two successive cold rounds each remove exactly the admitted unit and remeasure physical availability before entering the cold callback. Unrelated children remain. The recovery retains the real global compiler writer, exact current CargoStore EX and physical locks, complete rooted descriptor census and aggregate unit recensus. Two unrelated ambient renderer rustc processes were running during the passing laws; their presence is diagnostic, not a global retirement veto for authenticated current compact roots. No process was stopped.

This does not assert arbitrary raw-filesystem ABA identity protection: the existing recognized-unit recensus compares path, bytes, recency and lock state, rather than a complete per-child inode/ctime roster. Authentic global writer, CargoStore EX and profile locks exclude enrolled writers, while foreign descriptors below candidate/build/target refuse reclamation. The enclosing historical image EX remains the stronger authority for authenticated old roots.

The final read-only canonical Rust availability was 4,297,097,216 bytes, only about 2 MiB above the 4 GiB initial floor. No canonical compiler retry was launched on that sample. Each actual accepted cold write rechecks its reservation and fresh physical free blocks under the retained writer; pressure now attempts bounded current compact reclamation before refusing. This is repeatable admission/recovery, not a physical 4 GiB per-mode quota or guaranteed runtime growth allowance.

## Current Integration Hold and Test-Side Adaptation

Root's canonical Graph4 preparation Original 53571 subsequently succeeded against the external current 896 MiB pool policy: 938,475,520 fully allocated backing bytes, 938,434,560-byte mounted filesystem, device 16777270. The measured existing free blocks met the revised actual admission floor; no canonical snapshot or current graph/authority retirement occurred. The private pressure recovery tests remain separate evidence. Current graph owner/provider bytes differ from SDK 60f95, so a new coherent normal SDK publication is still required before Native/ordinary claims.

The Native image fixture is being adapted only on its test side: one received process Original now encloses its four physical law groups, and each frozen Node DbCache operation enters the actual graph-only Native owner before constructing the database. Independent Node SQLite checks additionally observe the real common compiler writer during Native put and deleted-output copy. These test changes have not yet been executed or type-qualified. Storage is completing actual production prebirth Domain declarations and the closed Repo-to-Nx child data serializer. The ordinary seven-phase fixture's old structural invocation copy must be removed in favor of that genuine prebirth boundary before its next run. No new SDK, Native runtime or ordinary pair has been launched during that integration hold.


### Native Fixture and Genuine Prebirth Data Staging

The physical Native fixture now uses one actual received Original across all four law groups. Its language-neutral artifact corpus additionally specifies busy-physical-writer publication refusal, zero entered publication body, no hit and zero task-details rows. A genuine queued compiler-writer EX is held by the funded parent while the Node child attempts the actual frozen publication gate; independent Node SQLite reads inspect the artifact task-details table. Normal Native put and deleted restore separately assert the common writer remains EX-held at the real N-API operation. These are staged assertions, not passing runtime claims. Original test limit remains 65 seconds.

The ordinary seven-phase body now retains `invocation = original`. Before the private headroom root is born, its finite launches include the actual outer private request and the canonical fixture/dependency launches projected by `createNxBootstrapCapabilitiesV1`; the existing root's policy/native ceilings stay unchanged. No structural invocation copy, second received root, or authority remint is introduced. The canonical Nx role has no `tool` field.

Focused strict Original 53564 terminated with two classes of type failure: the newly included fixture files' direct `.mjs` imports lack declarations, and Root's current graph transaction stat type was still nullable. The fixture imports now use an own typed `createRequire` interface to the same actual implementation function, without a default identity or altered defining bytes. Root owns the graph narrowing. No strict successor or Native/ordinary execution is claimed yet.


Final expanded fixture/production strict Original 62705 exited zero. Intermediate 86912 caught the peer bootstrap import placed above its shebang; restoring the existing shebang to line one preserved the new handoff import. Intermediate 59843 then caught the handoff ports type omitting the environment field required by the current receipt verifier. The actual SDK caller already passes that field; the annotation now includes it without a runtime fallback or guard change. These current captured Source corrections must be included in the next normal SDK. All owned Originals are terminal; Native and ordinary runtime qualification remain pending Storage's coherent actual prebirth/identity handback.


### First Canonical Automatic Recovery and Protected Busy Refusal

The independent Rust validation continuation Original 44292 exercised the production compact route while refreshing downstream locks. Its first pressure admission genuinely removed 95 recognized units / 183,688,884 logical bytes, increasing physical free space from 4,115,779,584 to 4,297,097,216 bytes; Raster lock refresh then completed. A later fetch admission observed peer growth leaving 4,254,797,824 bytes free, planned 71 units / 42,085,957 logical bytes, and honestly refused because the actual physical compiler profile lock remained held. The second deletion body did not run. All validation Rust handles are terminal.

The common writer is not evidence that every historical/foreign compiler participates in it. Actual physical profile exclusion remains required. Current compact recovery deliberately uses nonblocking CargoStore/profile admission while holding its writer, so it never waits behind a potentially conflicting ancestor/old writer. Seven ambient compiler names were diagnostic only; the observed physical lock was the refusal cause. No queued physical wait, process stop, cap increase or blind retry was introduced. A later improvement may select a separately unheld mode or release writer custody before any bounded retry, but no such behavior is claimed in this frozen checkpoint.

Storage's actual Source-to-installed-SDK private identity callback gate is now known: the issuer-bootstrap `sdk` route. Authenticated SDK 60f95 genuinely fails same-signal/private-registry equality because it predates the current canonical owner. The next normal SDK must capture the coherent current closure and pass this gate in both modes before Native/ordinary qualification. No structural registration or global registry fallback is permitted.
