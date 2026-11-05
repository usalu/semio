# Canonical Rust Allocator Audit

Read-only audit; no tests/builds/production changes. Root selected `.r` partition with unchanged `m/u/l/<id>/{b,t}`; this avoids the proposed flattened namespace collision. Existing laneIdsV1/listLanesV1 accepts every alphanumeric directory up to12 characters, which would classify unrelated Rust root directories as lanes after naive flattening. Retaining lane policy preserves GC, pool affinity, live markers, retirement and provenance ownership unchanged.

Root supplied declared maximum55-character crate codegen suffix208. With unchanged maximum lane ID, `.r` build base46 plus208 is254, while `.cache/b/r` base54 plus208 is262. Therefore maximum absolute build-root budget48 for that declared suffix (256-208); require both UTF16/codepoint measures, before issuing Cargo. Current physically compiled selected subset independently exhibits suffix187 under build root for causal rcgu.o and metadata suffix177; these are observed witnesses, not a replacement for declared maximum208. Native owner should preserve package/target/codegen shape rather than truncate outputs.

## Closed Ledger and Ignore Coupling

Budget policy alone is insufficient: ledger roots currently repository=.semio/repo/cache and native=.cache, with rust partition root=repository/directory=rust/fixed. Ledger forbids dot/dotdot segments. `.r` needs an explicit declared root representation. Closed coupling: ledger TypeScript CacheRootIdV1 line12, claimed map51, root loop102 and root resolver131; ledger JSON schema root enum/required/properties and policy rust area; downstream root enumerations and ledger neutral fixtures must follow schema. Preserve other repository loose areas. Explicit `.r/` exclusion is required in `.gitignore` and `.dockerignore` (existing old cache exclusion does not cover it); audit Nx/Bazel source ignore current policy as well.

## Exact Old-Root Literal Inventory Outside Tickets

Literal `.semio/repo/cache/rust/fixed` (actual emoji spelling) appears in21 files: `.cargo/config.toml`; `nx.json`; repo README; native-build/units/tests/plan-stability test; library/laws/test-store-isolation fixture; compiler/layout fixture; budget/opening/reset/tests/profiles test; budget policy; nx enforcement configuration schema; bounded artifacts fixture and test; cargo/units test; unit-cache/gc/units test; pruning/budget/history policy and schema; nx23.2 patch; process/cargo/policy fixture; build/input/membership/ignore fixture; Bazel Cargo fixture; build graph fixture; runtime verification/storage runtime fixture. Handcraft current path expectations; preserve intentional negative or historical rows where applicable. No source copies created.

## Provisioning and Concurrent Old Owners

Opening attempt lines71–74 selects exact authored declaration.directory and opens only that image; paths RootData helper derives declaration plus mounted m. There is no implicit lookup/move/retirement of the old image. New `.r` image therefore leaves existing old active image and its inode/custody untouched. Do not reset/remove old root during peer execution. Keep old owner declared/retained while active if changing ledger collection rules; provisioning itself performs no migration. New image has no old cargo/home/unit CAS; satisfy normal locked acquisition at the new owner, without private copies or live-image mutation.


## Final Canonical Decision: .r/i

Root selected loose root rust=.r and partition directory i, preserving explicit container/partition ownership and all u/l/id lane lifecycle paths. Declared buildbase48 + selected maximum55-name codegen suffix208 =256, exact boundary; maximum permitted absolute build base48. This supersedes earlier .r-only sketch.

Additional concrete hardcoded root consumers: build/cache-gc/runner TypeScript totals182, undeclared scan196, root report204, collection314. Add rust alongside ledger type12, claimed map51, validation loop102 and schema root enum/required/properties. Neutral ledger model22 sums only repository/native budgets and52–53 asserts two-root README text; main ledger test103 casts only two roots; native law61 checks pair separation. Update these current contracts, root tests/fixtures and documentation for three roots.

Old image handling: retain old rust/fixed path as finite managed tree owner=image, without collect or retirement. Runner collection295–299 excludes managed nonpartition/noncontrol trees from planning; it only defers owner, and no image-owner delegate branch was found in cache-gc source. Conversely retirement309–312 scans every child with quiet custody, which does not itself establish mounted-image/kernel ownership. Therefore do not put active old image in retirement roster. Normal new image opening remains independent and does not modify old image. No manual image work performed.

Boundary scope:208/max55 is the selected WGPU closure proof, not a universal repository name bound. Read-only manifest scan also found longer unselected test package names such as semio-s-artifact-trinity-rewriting-tests-narrows-the-lhs-pattern-to-a-shaft-neighbour. Keep precompile budgeting grounded in admitted selected graph and real compiler suffixes rather than treating55 as a global schema bound.


## Revised Lane Build Equals Base

Root revised `.r/i` design to preserve u/l/id while removing redundant b child: lane.build=lane.base, target remains lane.base/t. This recovers2characters without exposing unrelated root directories to lane enumeration. Concrete mandatory coupling: retention withLaneRetentionV1 line132 currently records lane destination at dirname(build), which would become shared u/l/lane.ledger and overwrite between peers; retainRemovedLaneV1 line123 reads lane.base/lane.ledger. Carry explicit base in withLaneRetention input and record there. laneBuildDirectoryOwnedV1 lines33–34 must change exact id/b two-component matching to exact id one-component matching. compilerOriginOwnedV1 line43 currently computes sibling target from dirname(build), which becomes wrong, though its inside(build,target) fallback accepts true nested target; rewrite target identity from explicit lane base for clarity and closure.

No concrete target-nesting prohibition found: generic native compiler already uses recipe target inside build/t/hash; provenance ownership explicitly accepts target descendants of admitted lane buildroot. Metadata/live/stamps do not match retention receipt filename regex, so retainLaneOutputsV1 does not publish them. Existing lane claim/live marker validation still protects whole-base deletion; enumeration remains u/l only. Update schema Policy.layout (remove build), lanePathsV1 implementation, retention-model schema/data/TypeScript/Python path oracle, lane neutral fixture expected paths and any fixture assembling lane base/b directly; preserve shared repository compiler layout b/p, which is a different owner.

Managed control old-image decision is preferable to tree workaround: ledger line77 permits control with managed owner and no collect/budget. Cache GC runner295–299 neither plans removal nor defers managed control (unlike tree). Status runner151–160 may measure/read it but cannot remove it. It stays declared so undeclared pruning does not select old image. Keep old path out of retirement roster. New third-root provisioning is path-derived independent image opening and leaves old image untouched. No source edits/tests/builds by audit; root informed.


## Third-Root Scanner and Source Readiness

Additional current production boundaries absent from canonical-oldroot literal inventory require `.r` handling: caching policy generatedDirectories; build/inputs/read-oracle/policy ambientTreesV1 line20; discovery/source-access POLICY_SKIP_DIRS17; discovery/semantic-census SEMANTIC_SKIP_DIRS126; Bazel script BAZEL_SOURCE_EXCLUDED222; semio scheduler EXCLUDED19; graph owner-discovery152 and graph module240 (both additionally use central generatedDirectories); resident command rooted list11; build/source/closure58 (external implementation admission rejects node_modules/.cache only); dependencies/inventory skip lists13,800,901,946; Storybook MDX skip7; language-catalog law skip41. These are direct authored lists, not Git-ignore readers. Add canonical .r tree and neutral boundary cases rather than relying only on `.gitignore`/`.dockerignore`. Budget/workspace-root scratch list5 separately distinguishes allowed generated private workspaces; current scratch semantics include .cache/.tmp/.semio but omit .r.

Actual test-store-isolation owner policy storeLiterals line24 only matches .cache/b and old cache/rust. Add literal root-segment .r matcher plus neutral checkout join/template/store-call findings, private-root and .r-neighbour nonfindings. Its independent Python owned-path oracle99 already derives authored budget partitions, so follows .r/i automatically. Membership ignore fixture needs .r/i/m children ignored with empty Cargo/build directory results. Raw-process-spawn fixture skip list and independent Python language-catalog scanner must stay aligned with owner policy. Numerous Python law fixtures carry duplicated skip lists; they scan supplied file corpora, so update exercised boundary corpus where .r is supplied rather than claim Git ignore changes them.

Scope discipline: genuine .cache paths for lease stores, graph-only maintenance fixture, machine host free-space probes, account runtime sockets and test sandbox remain valid owners; do not globally replace every .cache occurrence. New .r needs explicit third-root ignore and source exclusion while these other stores remain. All findings sent early to root. No gates or source edits launched by audit.


## Authorized Scanner Corrections and Actual Checks

Root authorized low-complexity implementation after read-only findings. Added exact .r cache-directory exclusion to explicit Source scanners, compiler source closure refusal, graph and scheduler inventories, resident watcher and mirrored scanner policies; preserved other .cache owners and root-owned strictStorage changes. Test-store-isolation now recognizes compact Rust root segments with five additional portable join/template/private-root/neighbour/store-call cases; existing current Rust fixture uses.r/i. Membership fixture current roots use.r/i and adds retained observation output ignored plus visible Source neighbour. Raw spawn registry ignoredSegments and neutral registry mirror stay aligned. No AGENTS/git modifications.

Files changed by this lane:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪢️policy/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧩️semantic-census/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🏁️scheduler/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/⌨️command/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🐍️.py`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📂️membership/🧪️tests/🛡️ignore/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`

Actual normal-admission validation: initial68943 pretest invocation rejected missing ./ operand (corrected; no behavioral result). Corrected scanner RED60941 terminal1:0pass1fail8assertions, reproduced current Rust checkout-path miss. After regex fix focused GREEN terminal0:1pass24assertions, including independent Python fixture oracle. Membership first2pass1fail69assertions exposed invalid hidden .r-neighbour Source expectation; TypeScript glob intentionally omits hidden directories. Kept exact .r-neighbour precision in detector law and used visible r-neighbour for shared Nx/TS fixture. Membership successor terminal0:3pass82assertions, real Nx/independent ignore+actual TypeScript excludes+cleanup classifier. Existing raw-spawn law focused successor terminal0:1pass5assertions. A drafted raw judgment row incorrectly assumed explicit-file judgment performs directory traversal; actual failure showed correct explicit-file behavior, and the invalid draft row was withdrawn without changing that API. No unsupported suppression added. Generated logs remain ticket-generated for controller cleanup.


## Current Root Patch Review

Read-only review after scanner implementation found no actionable runtime ownership issue in root third-root patch. Ledger schema roots is closed and requires repository/native/rust; policy rust=.r, partition area rust/i and nested Rust areas now align. CACHE_ROOT_IDS derived from authored policy drives claimed map, budget validation and GC totals/status/undeclared/collection loops. Old rust/fixed is a managed control owner=image, no budget/collect, and collection neither delegates nor plans it. No full old canonical prefix remains outside tickets in read-only tracked source search; relative held-control rust/fixed is intentional. Git/Docker exact /.r/ ignores and root scratch .r are present.

StrictStorage schema requires partition const rust, derives directory from sole budget rust declaration, and preserves capacity/filesystem/workspace/artifact limits. One observed catalog refresh remains: repo.library.caching schema expected hash345614c5456d018a13684ee4ebf01affd3e4446cffa543160df2fc103a70b64a versus current57b42823840bbd2733f901b39c5e4f144bf7d2df3f7f1f9f37e6476a211f3cfa. Root owns refresh. Ledger schema has no existing catalog scope row found, so no renamed/missing existing ledger export was inferred. Medium in-progress lane/envelope deliberately not audited at partial checkpoint. No new gates launched.


## Stable Lane and Selected Path Envelope Review

Read-only stable checkpoint review found no actionable implementation issue. Lane build equals base but enumeration remains u/l/id; retention records explicit base, killed-holder recovery reads same marker; origin ownership accepts exact lane target t or descendants rather than arbitrary lane metadata. Incremental traversal skips root target/live/stamps namespaces, retaining compile profile traversal; whole lane removal retains existing claim/live-marker gates.

New path-envelope helper projects exactly six selected unit DATA fields, validates owned Unit schema, models metadata/static/dynamic/native library/codegen and recipe publication paths. Null host platform avoids cross-target suffix; custom-target JSON basename is modeled; dev/test maps debug. Known codegen widths16/25/7 and recipe32 remain schema-owned. Both UTF16 and codepoint measures reject over256; added astral257/within256scalar case plus Python UTF16-le/pathlib oracle validates the original measure distinction. Actual native-build planning.units check at300 precedes Source admission and recipe production; recheck311 reapplies envelope+Source checks; produce315–316 rechecks before capture. Existing Source bounds and issued control remain unchanged. No new gates launched; native owner retains focused23087 terminal ownership.

Catalog/index consistency now confirmed: repo.library.caching hash57b428... matches current schema; new framework.process.artifacts.native-build.path-envelope scope path/format/hash00b32e... matches actual JSON, empty exports consistent with definitions-only schema; Markdown row matches same scope/path and no exports. DSL schema index maps semio:process/cargo/path-envelope/v1 to that file beneath framework/process owner. No full generator rewrite required.


## Whole-Repository Remaining Lane Coupling

Read-only tracked whole-repo search found canonical default .cargo/config.toml line11 still lane d/b; notified root, and fresh read confirmed corrected d with target d/t. Additional actual missed fixture consumer: unit-cache/lane/pool/tests/sessions Python oracle line8 computes relative path against base/b, while TypeScript test creates sessions using paths.build=base. Therefore oracle includes current profile/incremental bytes that real readPool excludes, breaking byte/trim comparison. Native Medium and root notified; no edits or gates.

Other narrow matches are not stale canonical consumers: retention-model /ab12/b and /ab12/bx are intentional ownership refusal vectors; consumer-lanes test explicitly rejects base/b; pruning workspace projection layout.build is a generic corpus field, not removed UnitCache Policy.layout.build; process storage fake base/b and jobserver explicit isolated build paths remain separate test layouts. No further production dirname(build) retention guess or removed layout.build policy consumer found outside completed owners. Broad and focused read-only grep commands completed terminal0; no active process retained by this lane.


## Complete Isolation Law Actual Result

Normal registered repo Bun explicit ./ file session16678 completed terminal1:3pass1fail57assertions,4tests/1file/8.53s. Rules/schema, independent Python fixture corpus and actual preload fence/private workspace tests passed. Whole actual Source law failed at test source40 with24 checkout-store findings. No exemption, detector weakening or Source change made. Exact log: 🗑️generated/fleet-rust-isolation-full-2026-11-04.log. Findings:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:20 checkout-store-call withRepositoryCacheStoreV1(root,"graph",original,control,(_graph,graphPhase)=>withNxGraphCacheStoreV1(root,original,graphPhase,body))`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:21 checkout-store-call acquireResourceLease({directory:join(root,".cache/b/l/m/data"),resource:nxGraphNamespaceResourceV1(nxGraphNamespaceDirectoryV1(store.root,value)),mode:"exclusiv`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:21 checkout-store-path join(root,".cache/b/l/m/data")`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:23 checkout-store-call withRepositoryCacheStoreV1(root,"graph",original,phase,()=>bind(value.toolingIdentity,null,value.implementationIdentity,root,fixture.foreignWatcher))`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:24 checkout-store-call withRepositoryCacheStoreV1(root,"graph",original,phase,()=>acquireNxGraphNamespaceV1(store,value))`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts:25 checkout-store-call withRepositoryCacheStoreV1(root,"graph",original,phase,async(_graph,inner)=>{await acquireNxGraphNamespaceV1(store,value);controller.abort(Error("nx deadline la`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/🧪️tests/🟦️.ts:112 checkout-store-path join(process.cwd(),".cache/b/t/m/repo-watch",pin.sources[0]!.sha256.slice(0,16),"semio-repo-watch")`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/✅️conformance/📜️script.ts:33 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,session,async()=>{const options=(name:string)=>({directory,leaseDirectory,filesystemBytes,maximumPhysicalB`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🎚️capacity/📜️script.ts:10 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{ const directory=join(base,"p"),leaseDirectory=join(base,"leases");fs.mkdirSync(leaseDir`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/📜️script.ts:18 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,()=>withNativeStorageSelectionV1({backend:"directory-partition"},async()=>{ const base=join(area,m`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts:16 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{ if(phase==="cleanup"){ const image=await openNativePlatformStorageV1(options),port=nati`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👶️lifecycle/📜️script.ts:11 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,()=>withNativeStorageSelectionV1({backend:"directory-partition"},async()=>{ const options={directo`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📁️directory/📜️script.ts:11 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{ if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Original test artifact directory r`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/📜️script.ts:14 checkout-store-call withRepositoryRuntimeEntriesV1(workspace,original,control,async()=>{ const image=await openRepositoryCachePartitionV1(workspace,"tools",control);let failed=fals`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/📜️script.ts:15 checkout-store-call openRepositoryCachePartitionV1(workspace,"tools",control)`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/📜️script.ts:17 checkout-store-call repositoryLeaseDirectoryV1(workspace)`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/📜️script.ts:16 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{ let facts:{identity:string;source:string;configuration:string;manifest:string;buildDire`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📜️script.ts:23 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{if(mode==="positive"){await withAcceptedPhysicalCleanupV1(control,{version:1,kind:"nativ`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/📜️script.ts:17 checkout-store-call withOriginalNxGraphNativeStorageV1(workspace,Object.freeze({...environment,SEMIO_NX_GRAPH_DIAGNOSTIC_DIRECTORY:diagnostic}),async()=>{entered=true;})`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/📜️script.ts:17 checkout-store-path join(workspace,".cache/b/l/m/data")`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🧹️whole/📜️script.ts:9 checkout-store-call withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{let image:Awaited<ReturnType<typeof openNativePlatformStorageV1>>|undefined,token:Native`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts:349 checkout-store-call repositoryCachePartitionRootV1(root,"rust")`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/📥️invocation/🟦️.ts:16 checkout-store-path join(testCheckoutRootV1(),".cache/b/t/m/nx",identity)`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/📥️invocation/🟦️.ts:40 checkout-store-call withRepositoryCompilerRuntimeEntriesV1(testCheckoutRootV1(),original,createIssuedResourceControlV1(original,"private-repository:runtime",0),()=>body(original))`

Observed findings refer to existing store-call classification or .cache paths, not added .r literal matches. Current detector addition does not change call/identifier inference. Root notified; causal review stays with actual offending source ownership, not suppressions.
