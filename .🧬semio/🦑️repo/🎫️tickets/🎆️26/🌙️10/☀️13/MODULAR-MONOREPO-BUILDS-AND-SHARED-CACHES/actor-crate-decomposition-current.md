# Actor Crate Decomposition

## Original Frontier and Chosen Owners

The actual repository policy remains 262144 Source bytes, 32 Source files and 4000 active lines per unit. The original Actor body alone is 260458 bytes / 5312 physical lines; its production module closure adds the original retained-turn, lifecycle/page/cold-pair and activation bodies. No cap or compiler/storage admission is changed.

The actual dependency inspection found that retained-turn depends on root pack primitives, while its context references the same retained-turn types. Lifecycle/return/cold-pair are a mutually related wire family; return consumes crate-private lifecycle helpers. Job replay/progress share a private JobPublicationOwner and depend on nominal actor/Job wire values. Scheduler and Kernel directly share private ScheduledActor and actors state, so cutting between those would invent a facade or widen internals.

Chosen downward native owners: pack primitives; retained-turn; actor vocabulary/wire; retained Job publication replay/progress; existing Actor runtime assembling those actual owners. The wire owner receives the same nominal IDs/lanes/budgets, Job value protocol, envelopes and return/lifecycle family. The publication owner receives existing replay/progress bodies and their private-owner laws. Runtime keeps the actual scheduler/Kernel private SCC, metrics, scene/shard state and activation reservation owner. Body moves are genuine canonical ownership moves; no Source body is copied to the ticket and no compatibility wrapper or Source exclusion is introduced.

The language-neutral ownership schema/corpus was authored before implementation. Its law parses actual Cargo manifests with independent @iarna/toml and checks recursive dependency closure with independent SQLite; expected representative domain declarations must physically exist in their canonical owner. The permanent test-ownership command is in the actual Actor package script/project and private actor-ownership route. The original missing-owner RED is running; no new native compilation success is claimed yet.

## Pending Qualification

Move original bodies and their private test helpers together, reconcile actual imports/manifests and direct law names, then use actual issued Cargo prototype/release via the private frontier route. Each production and test unit must pass the unchanged real Source boundary. Existing serde round-trip, pack and exact receiving laws remain meaningful; any stale preexisting law construction must receive real Original fields rather than weakened assertions. Root owns grid2d lock/collector retry and editor launch registration; this lane owns Actor sources, manifests and permanent owner commands only. Compiler/tooling/graph runtime remains untouched.

## Implemented Canonical Cut and Evidence

The schema-first missing-owner law genuinely failed at the absent pack manifest (actor-ownership-red1.log). Original bodies now have six actual native compilation owners. Pack primitives are independently compiled from their canonical implementation. Retained-turn remains the same physical original subtree, with package glue mounting that one implementation and the real pack dependency. Its existing context was then separated into its actual downward owner after the unchanged Source census found 34 test files; no tests or embedded inputs were excluded to make that pass. Actual context clients now import Actor retained_context; the old retained_turn::context route is removed from actual Rust clients and Source law literals. The exact context native command points to its canonical package/law names.

Vocabulary/wire owns IDs, lanes, budgets, Job wire values, envelopes, the existing mailbox/failure wire state and lifecycle/return/cold-pair family, preserving the private lifecycle helpers inside the same owner. Publication owns exact replay/progress storage plus private JobPublicationOwner, original cfg(test) bridge producer and their original laws. Runtime keeps Scheduler+Kernel private state, shard/scene/metrics, activation reservation and schema metadata, assembling the actual leaf types. Root body is now 1792 physical lines rather than 5312. Two stale TurnGrant law literals now borrow the existing supplied_retained_loan helper instead of omitting the required original loan. Native heap observation remains real: retained and context test roots each use the same original trace HeapWitness allocator after their test bodies changed crate ownership. No assertion or runtime original-control body was weakened.

The independent Cargo TOML/SQLite ownership law and actual cfg-selected production/test Source closure law are terminal GREEN 2 tests / 62 assertions. Targeted Actor ownership test + actual package script ES2022/noUnchecked strict exits 0 (actor-ownership-types1.log). Editor prototype/release ownership/native gate pairs are registered in both launch catalogs by Root. Private actor-ownership and crate-frontier-actor/-tests commands execute the permanent actual script and issued buildRepositoryCargoArtifacts route.

|Native owner|Production files / bytes / active lines|Test files / bytes / active lines|
|---|---:|---:|
|pack|1 / 8144 / 195|1 / 8144 / 195|
|retained-turn|10 / 77073 / 556|29 / 136539 / 723|
|retained-context|1 / 26823 / 95|5 / 42853 / 339|
|vocabulary/wire|6 / 122449 / 2625|17 / 178734 / 3173|
|Job publication|1 / 82568 / 1647|5 / 148769 / 2721|
|Actor runtime|3 / 107060 / 1790|7 / 187177 / 3046|

Those measurements are actor-ownership-green5 before the tiny explicit failure-policy helper doc/visibility, test allocator declarations and two supplied-loan field reconciliations; exact final Source successors are running. All remain below the unchanged policy; a final snapshot will replace this intermediate table. These authored closure observations are not represented as successful compiler executions.

The first genuine bounded Cargo prototype frontier (actor-frontier-prototype1.log, terminal exit 1 after 1 m 2 s) compiled and qualified the actual pack owner, then refused semio-framework-job before compiler byte allocation. Thus no Actor runtime/native law success is claimed yet. Root has the exact transitive Job frontier and owns the coordinated decision to cut it; this lane continues Actor reconciliation without bypassing its Source guard, fabricated Job types or a cap increase.

## Exact Current Actor Checkpoint

Exact prototype-final and release-final ownership gates are both terminal GREEN 2 tests / 62 assertions. Targeted ownership/script ES2022/noUncheckedIndexedAccess strict remains terminal exit 0. The separate retained-context manifest now declares its actually derived serde interface directly, as flagged by the independent audit; its real trace witness remains dev-only. Unused direct async from wire vocabulary, the replay-only unconditional async edge from runtime, and unused runtime dev pack-json were removed. The browser-only async bridge dependency remains target-scoped. These are honest dependency narrowings, not alternate compiler inputs.

- semio-framework-actor-pack production: 1 physical inputs, 8144 bytes, 195 active lines.
- semio-framework-actor-pack test: 1 physical inputs, 8144 bytes, 195 active lines.
- semio-framework-actor-retained-turn production: 10 physical inputs, 77201 bytes, 558 active lines.
- semio-framework-actor-retained-turn test: 29 physical inputs, 136667 bytes, 726 active lines.
- semio-framework-actor-retained-context production: 1 physical inputs, 26951 bytes, 97 active lines.
- semio-framework-actor-retained-context test: 5 physical inputs, 42981 bytes, 342 active lines.
- semio-framework-actor-vocabulary production: 6 physical inputs, 122557 bytes, 2626 active lines.
- semio-framework-actor-vocabulary test: 17 physical inputs, 178842 bytes, 3174 active lines.
- semio-framework-actor-job-publication production: 1 physical inputs, 82568 bytes, 1647 active lines.
- semio-framework-actor-job-publication test: 5 physical inputs, 148769 bytes, 2721 active lines.
- semio-framework-actor production: 3 physical inputs, 107060 bytes, 1790 active lines.
- semio-framework-actor test: 7 physical inputs, 187239 bytes, 3046 active lines.

Native results remain limited to the genuine pack prototype compile before the transitive Job refusal. Ingress owns the canonical Job core/worker split; Actor uses no worker/session symbols and retains only actual core authority, StepContext, payload and InteractiveJob interfaces. No further native retry is launched until that coherent Job checkpoint. Shared compiler/tooling/graph Source and activation are untouched during Nx qualification.

## Existing Original Source Laws Reconciled

The permanent ownership route now runs the same actual Source law list as original-receiving-source, including the new ownership law, and strict ES2022/noUnchecked checks before executing it. Expanding this genuine route retained a RED of 18 pass / 7 fail (actor-source-contracts-red1.log). The failed Source observations were reconciled to the actual canonical owner, not replaced by expected booleans: lane/budget checks read vocabulary; descriptor/allocation checks read Job publication; context boundary checks use independently parsed canonical Cargo manifest and require the old upward module mount absent. Existing scheduling/receiving checks now explicitly include the required held loan, actual independent work argument and remaining-fuel preservation. The loan fixture was importing the unrelated retained receipt schema; it now uses its already-authored closed semantic-loan schema. Four noUnchecked diagnostics were fixed by immutable literal tuples.

The expanded actual suite is terminal GREEN 25 tests / 324 assertions in prototype (3.5 s) and release (7.7 s); release includes strict ES2022/noUnchecked before all 25 laws. Independent Rust tree-sitter AST, Ajv, SQLite and RFC6902 observations all remain in those existing laws. No native Actor compiler result is inferred from this Source success. Job remains the only observed precompiler frontier from the actual native attempt, and its execution lane owns that cut.

## Direct Canonical Value Boundary

Root found every protocol:: reference in actual Actor Rust sources names protocol::value; no actor Source uses Replication IDs, wire, mutation or causal contracts. Four real Actor packages already directly depend on semio-framework-value. The closed ownership fixture now names those four consumers and checks their real Cargo dependencies plus their complete authored native Source closures. Genuine Original53260 is RED25pass/1fail/326 assertions for the unnecessary Replication edge. Root replaced130 path/derive-owner references with the exact existing canonical semio_framework_value owner and removed the four unused Cargo edges. No alias facade, grammar/type duplication, Source omission or cap increase is introduced. The first successor Source gates caught a transient malformed leftover manifest comment; it was removed, and current both-mode Source successors are running. Actual native retry follows that Source checkpoint and coherent owned lock refresh.

Current direct-Value successors4368 prototype and97305 release are both GREEN26 tests/368 assertions, Nx13.6/13.0s, including the actual current strict Source check. The new neutral law reads the real four manifests and complete native test closures and verifies canonical Value paths plus absence of Replication dependencies. Actor Source is stable and returned to Storage for actual native aggregate qualification. This avoids compiling the unrelated large protocol solely for an alias; it makes no claim that the actual Replication package meets its own Source policy.

## Actual Compiler-Exposed Vocabulary Correction

actor-frontier-prototype3 is genuinely terminal1 after2m26s: actual pack, retained-turn and retained-context compiled under the unchanged Source boundary and issued Cargo store. Vocabulary reached rustc and reported E0425/E0433 because Effect::SpawnJob names JobReplayRequest. The request had incorrectly been assigned to the upward publication owner; adding a reverse dependency would create a real cycle.

The neutral unique-owner fixture was extended first. actor-vocabulary-request-red1 recorded the genuine ownership failure (25pass/1fail). The exact JobReplayRequest body and its private request-digest functions were then moved into the vocabulary owner. The retained publication header consumes the exact shared vocabulary-owned replay_mix; no bodies were copied, hidden, stubbed or duplicated. The publication/runtime dependency direction remains downward. The two unused private vocabulary imports were removed; actual retained receipt usage remains. actor-vocabulary-request-green1 is terminal GREEN26 Source laws.

Ingress's actual Framework offline generation then locked fetch57844 is terminal GREEN after the current PackSource and UI/BREP membership. It reported no lock jobs remaining. Fresh actual six-owner prototype Original55529 is now executing, retained as actor-frontier-prototype4.log. It is not a compiler success until terminal output exists; release follows only a genuine green prototype. The shared compiler/tooling/graph Source remains frozen for Nx and is not edited to facilitate Actor.

## Actual Six-Owner Prototype Is Qualified

actor-frontier-prototype4 (Original55529) is terminal GREEN, Nx3m44s. Its actual [DEBUG] qualification receipt lists all six real selected manifests: Actor Pack, RetainedTurn, RetainedContext, Vocabulary, JobPublication and Actor runtime. Each went through the genuine buildRepositoryCargoArtifacts route, issued compiler store, private retained toolchain and unchanged first-party Source admission before rustc. The actual runtime crate compiled; no compiler/API shim or Source/cap omission was introduced.

Release Original95845 now executes the same six selections sequentially, log actor-frontier-release1.log. Native test execution is separate and not claimed by the successful builds; test aggregate follows the genuine release result. A read-only sample of the still-displayed prototype PID refused because that exact child had already naturally exited; no signal, fallback process or invented custody was used.

### Exact Post-Correction Source Census

The actual26-law successor emitted these current physical cfg-selected closures before the successful prototype compilation. Counts include the actual owned module and embedded input roles, not package-root glob guesses. Production and selected tests both satisfy the unchanged262144-byte/32-file/4000-active-line boundary.

|Owner|Production files / bytes / active lines|Test files / bytes / active lines|
|---|---:|---:|
|Pack|1 /8144 /195|1 /8144 /195|
|RetainedTurn|10 /77255 /558|29 /136721 /726|
|RetainedContext|1 /26951 /97|5 /42981 /342|
|Vocabulary|6 /125392 /2684|17 /181677 /3232|
|JobPublication|1 /80237 /1590|5 /146444 /2664|
|Actor Runtime|3 /107264 /1790|7 /187527 /3046|

This replaces the intermediate pre-request-move census above. Test closure admission is not native test success; the actual test aggregate remains separate.

## Actual Both-Mode Six-Owner Builds

Release Original95845 is terminal GREEN, Nx5m2s. Its real compiler receipt again lists all six selected manifests. The production split is now actual compiler-qualified in prototype and release; these results preserve every unchanged Source/file/line cap and current Job/Value protocols. Prototype native test aggregate Original75445 now executes the same six actual manifests with command=test and --lib (actor-native-tests-prototype1.log). No native test count is claimed until the actual test output is terminal.

## Genuine Native Tests Reveal Law Ownership and Current Inputs

Prototype test aggregate75445 is terminalRED at the actual RetainedTurn rustc invocation after the real Pack test harness0/0. The full compiler log reports63 errors, including Kernel/runtime symbols in the lower retained-turn tests, missing current work and loan fields, omitted actual work arguments and an undeclared pack-json test oracle. Production builds remain BOTHmode all-six GREEN; native test execution is not inferred from them.

Root inspected real test references and added only RetainedTurn's required dev-only pack-json dependency; its owned Framework offline→locked fetch53957 is GREEN. No speculative Context/publication oracle edges were added. The neutral schema/fixture was first expanded with six exact Kernel law names. actor-kernel-laws-red1 genuinely found no actual runtime-owned definitions. The exact six law bodies plus their runtime helpers have now moved to the runtime test owner; the lower receipt-codec law stays with retained vocabulary. No reverse Cargo edge or test body copy was introduced. The first Source successor exposed a relative-versus-absolute test-path comparison, corrected at the observation boundary.

Remaining source-cursor laws are being reconciled to the actual OriginalCallerSourceParent and finite paid work receipt, including the real parent birth/closure and unchanged source backing. Omitted work cannot be replaced with fabricated0; it is observed from actual StepContext before drop and conserved into the same returned original input. The next native attempt waits Ingress's coherent Job Source checkpoint.

## Current Test Ownership and Native Successor

The exact six Kernel law bodies and runtime helpers now live only in the runtime test owner; the lower receipt codec remains with RetainedTurn. Current source ownership suites are BOTH27/384 GREEN including embedded strict. Current actual source closures remain within unchanged limits: Pack195active/8144bytes/1file; RetainedTurn prod558/77255/10 and test677/116428/26; Context97/26951/1 and test342/42981/5; Vocabulary2684/125392/6 and test3232/181677/17; Publication1590/80237/1 and test2664/146444/5; Runtime1792/107360/3 and test3105/208376/11. These are actual closure observations, not inferred compiler success.

Actor aggregate native prototype78567 is terminal RED45.7s after a real Pack0/0 harness and actual RetainedTurn compilation. The63-error frontier is reduced to5 genuine test-only errors: four stale StepContext.work_units_spent calls and one local returned shadowing its original receipt helper. The actual public accessor is original_work_units_spent; those calls and the local source-return name are now corrected without fabricated work, weakened assertions or production changes. Fresh native execution is pending Ingress's announced genuine Job fault-publication owner/member checkpoint. No Actor compiler remains live; Ingress was explicitly released to make that coherent production cut.

Native successor31131 is terminal RED36.9s at genuine global lease filesystem SQLITE_FULL before RetainedTurn rustc; Pack0/0 already passed. It therefore does not qualify the corrected work/name laws. Current accessor observations are u128 and now use checked u64::try_from(...).unwrap() at the fixture receipt boundary, preserving the issued u64 fuel domain without truncation. Ingress's current core230058bytes/24files plus new fault-publication upper owner is coherent; its next worker-law move proceeds after this terminal signal. Lease physical capacity details are recorded in lease-metadata-capacity-current.md.

## Current Paid Inputs and Wire Oracle

After authenticated lease reserve recovery, prototype55055 compiled RetainedTurn without Rust diagnostics but failed its actual harness. The existing artifact-message collector did not preserve libtest stdout, so the ticket Actor test branch now uses the existing Original-authenticated public Cargo command route and retains the same store, caps, selected six manifests and `--lib --locked -- --nocapture`. Prototype5851 then exposed the exact result: Pack0/0 and RetainedTurn8/10 passed.

The source-crossing law attempted a second checkout using the pre-debit original fuel after the first loan consumed4 units. Production correctly refused renewed work. Its second checkout now borrows the actual current input from the same StepContext, preserving3 remaining semantic fuel and deadline12345. The independent Serde receipt oracle also found a real wire mismatch: four retained input/work/loan/receipt Value derives lacked the camelCase naming already required by their neutral corpus, Serde declarations and schema. Those four own derives now use that same wire spelling; no fixture spelling or production budget guard was relaxed.

The first post-fix invocation used the canonical repository task path for the ticket-private owner and refused `Unknown canonical build owner: repo-private-cache` before any native test (prototype6). The registered editor entry calls the ticket headroom script, which issues its genuine private Nx invocation; prototype7 uses that exact entry. Neither attempt changes compiler/tooling/graph Source.

Prototype7 (Original43322) is terminalRED40s with actual RetainedTurn10/10 GREEN, including independent Serde wire agreement and System allocation/work conservation. RetainedContext compiled and ran3 laws:2 passed; final slot-retirement law refused before its last enclosing metadata removal. Inspection confirms a production lifecycle bug: the child context clears its identity during paid inner closure, while the enclosing slot calls the child identity validator before reaching its own paid removal.

The slot now authenticates the original held binding and next epoch as before, then validates the terminal child’s exact remaining currency independently of its already retired child identity. The actual final metadata removal still requires one original work/item and depth. Regular live-context identity validation remains unchanged. The language-neutral boundary fixture adds attempted fuel and item renewal at this final closed-context frontier; the native law checks refusal with0heap,0physical receipt,0work and unchanged held slot before lawful closure. Prototype8 is the real successor; no success is inferred before its terminal output.

Current ownership prototype73798 and release72255 are BOTH27/386 GREEN, including embedded strict after the closed-context schema was completed. Actual prototype9 (Original19529) then qualified Pack0/0, RetainedTurn10/10, Context3/3 and Vocabulary19/19; its publication compilation stopped with15 precise errors. One remaining TurnGrant/Decision wire-producer law belongs to the runtime and has moved intact with its exact input helper and an authored current loan field. The runtime ownership neutral corpus now enumerates all seven moved law names. No reverse runtime dependency was added to publication.

The other errors expose real StepContext lifetime settlement: fixture replay operations reused one cumulative recipient while their previous context remained alive, and the production bridge read the operation while its context still held the preview-sequence borrow. Each context now drops after its actual operation and before the next borrow/publication observation. The same recipient is retained; no reset or fabricated receipt replaces its real Drop settlement. Current source successor and subsequent native capture remain pending Ingress’s short core-observation checkpoint.

Prototype10 (Original55740) is terminalRED3m13s after real first-four-owner successes and a fully compiled publication test binary. Publication now runs16 laws; several actual Job fixture callers refuse because this newly separated test binary has no global HeapWitness: their real root receipt is64 bytes but the absent allocator oracle reports0. Production budget/refusal behavior is not relaxed. The publication package now declares its actual test-only Trace dependency and installs the same global HeapWitness used by the retained owners. The replay test abort during panic cleanup is recorded as failure, not a completed replay qualification. Next actual execution waits the coherent Framework lock observation and Ingress’s announced test-target manifest checkpoint.

Ingress Framework84335 is terminalGREEN with the exact publication Trace dev edge and its coherent new core test target. Source production remains stable. Actual prototype11 now uses that current locked checkpoint; all earlier first-four-owner successes remain scoped to their actual original captures, and publication/runtime success is still pending.

Prototype11 (Original59806) is terminalRED2m53s with actual first-five-owner native success: Pack0/0, RetainedTurn10/10, Context3/3, Vocabulary19/19, Publication16/16 including genuine replay, actual allocator and paid closure laws. The runtime compiler is now reached and reports16 precise errors from the moved current laws: missing actual Async dev edge, missing direct Value progress import and stale activation/TurnGrant/receipt signatures. Those test declarations now name the actual first-party Async owner and current finite loan fields. The unissued activation transition fixture returns an empty receipt with0 declared work because it has no admitted semantic invocation; this is not a substituted measurement from an executed body. All measured source-cursor work remains observed from actual StepContext.

Ingress owns the requested next Framework offline→locked refresh for the newly explicit Runtime Async dev edge, after its already live original handles settle. No Actor native successor or release success is claimed before that coherent lock checkpoint. Actual first five successes are scoped to the reported prototype capture; runtime and full release remain pending.

### Latest Current Source Checkpoint

Ownership prototype69930 and release83991 are BOTH GREEN27/388 with embedded ES2022/noUnchecked strict exit0. The exact current source closures are:

|Owner|Production files / bytes / active lines|Tests files / bytes / active lines|
|---|---:|---:|
|Pack|1 /8144 /195|1 /8144 /195|
|RetainedTurn|10 /77347 /558|26 /116738 /677|
|RetainedContext|1 /27104 /98|5 /44399 /360|
|Vocabulary|6 /125392 /2684|17 /181997 /3232|
|Publication|1 /80291 /1590|4 /144685 /2657|
|Runtime|3 /107360 /1792|12 /211053 /3131|

All actual units remain under262144bytes/32files/4000active lines. This source result does not substitute for the pending runtime native harness or a full current release aggregate.

Prototype12 (Original94007) is terminalRED1m48s with first-five owners still fully nativeGREEN and the actual runtime99-law harness now executing:97pass/2fail. All original receiving, forgery, epoch, currency and lane/loan laws passed. The remaining failures were the checked TypeScript projection still omitting current work/loan fields, and an activation-reservation fixture expecting generic InvalidTransition where the current Original receiving gate correctly refuses the unissued turn first with TurnNotIssued. The projection has been hand-reconciled to the existing owned schema metadata (no raw unbounded typegen runner was invoked), and the precise unissued cause assertion now matches its actual protocol while retaining status/shard invariants. Prototype13 is the actual successor, still pending.

Prototype13 (Original4606) is terminalRED1m35s: runtime98/99, all five lower ownersGREEN. The only remaining equality failure was narrowed by decoding the actual native assertion and comparing linewise: the checked projection still used the old command header and omitted the metadata-declared Budget.max_frames. Those two asset lines are now reconciled; direct comparison with the captured authentic Rust renderer is byte-identical. Prototype14 remains the required actual successor; release will follow only after its genuine terminal success.

Prototype14 Original32633 is terminal GREEN1m45s: Pack0, RetainedTurn10, RetainedContext3, Vocabulary19, Publication16 and Runtime99 actual native laws,147 total. This includes the authentic checked TypeScript projection equality. No release aggregate on this exact defining closure is claimed. The next native capture is paused for the explicitly coordinated Job lower-worker authority ownership/manifests window; it will use the coherent new checkpoint after its owned lock and compiler qualification. No Actor handles remain live.

## Current Completed Native Pair

Current prototype2449 and release8044 are BOTH actual GREEN4m7s/3m53s using the coherent lower Job authority/preparation production checkpoint and owned Framework lock. All six selected actual native units qualified under unchanged262144byte/32file/4000active-line Source boundaries and actual fixed Rust compiler storage: Pack0, RetainedTurn10, RetainedContext3, Vocabulary19, Publication16, Runtime99,147 native laws total. Runtime includes exact checked renderer equality and all original receiving/forgery/epoch/currency/lane/loan laws. The current prototype ownership gate65400 is GREEN27laws/388assertions with embedded strict; release counterpart pending. All Actor native compiler handles are now settled and Ingress was given the next coherent Job window.

Ownership release74843 is terminal GREEN27laws/388assertions with embedded strict, matching prototype65400. Qualification scope clarification: native2449/8044 BOTH147 and these ownership gates are qualified defining snapshots. Subsequent Job worker terminal/cancellation/return-frame repairs occurred after those captures. They are NOT a latest-composite claim after those repairs. Root requires one final Actor composite recapture after worker native/unit/public BOTH and the affected20-workspace locked resolution settle; no new Actor run will start during the current leases64 capacity hold.


## Final Job closure recapture: current entry checkpoint

Rust Validation supplied the final frozen Job Native/Source checkpoint: all twenty affected lock/fetch scopes settled; core quartet BOTH thirteen laws, worker-public release eight laws, lower Error BOTH six and Crypto BOTH one. Prior Actor 2449/8044 BOTH147 remains evidence for its defining snapshots, not this later composite.

The first current private Nx attempt (`actor-final-job-prototype.log`, Original 98007) terminated before Cargo at `Original funded graph publication namespace required`. No Actor compiler/test result is inferred and the obsolete receipt/witness was not rewritten. A blind identical release was not launched.

A closed `actor-current/📜️script.ts test prototype|release` Source prerequisite now follows the established closed graph/Rust prerequisite pattern and is registered beside the Actor gates in both editor catalogs. At its actual Bun entry it issues one authored 600000 ms Original with the same existing Actor native route and ceilings. It reads the already authored Actor ownership corpus/schema for the exact six owner manifests and calls the same genuine `runRepositoryCargoCommandV1` with unchanged 240000 ms per-command bound. It does not cross the obsolete frozen SDK/graph receiver, mint a structural control, or claim ordinary Nx execution.

Current prototype Original 98939 is live in `actor-current-source-prototype.log` and reached real Cargo compilation. The existing automatic compact compiler recovery legitimately retired selected inactive units under real writer/store/physical/descriptor custody to satisfy its unchanged 4 GiB initial headroom guard. This driver contains no retirement operation, raw compiler invocation, capacity override or namespace mutation. Release follows actual terminal collection; no current pair result is claimed at this checkpoint.


### Collected prototype 98939

The current Source prototype is terminal exit 1. Actual Cargo tests passed Actor Pack (0), Retained Turn (10) and Retained Context (3). Vocabulary did not reach its compiler: initial Rust capacity admission invoked the existing compact retirement protocol, which refused a changed recensus. Exact observed unit `prototype/build/semio-framework-io-base64/2f5745745ec5f9bc` retained 609608 bytes and `lockHeld:false`, but its planned recency was 1792064587000 and its current recensus was 1792064516000. Available physical bytes were 3868667904 against the unchanged 4294967296 initial floor; the proposed 94-unit retirement was 436322533 logical bytes. The original exact recensus refusal and stack remain in `actor-current-source-prototype.log`.

This is partial current defining-closure runtime evidence (13 laws across the first three owners), not a complete current Actor composite. No recency/capacity/physical guard was weakened, no manual deletion was performed, and no stored witness was rewritten. Release was held rather than blindly repeating the newly observed capacity frontier. Root and Nx received the exact observation; Nx owns the actual retirement implementation. The new Source prerequisite and both editor entries remain ready for the coherent successor once capacity is lawfully admitted.

The expanded ES2022 strict/noUnchecked successor including the new closed Actor prerequisite is terminal exit 0 (`actor-current-source-types.log`, Original 31177). Core/process/domain/Actor implementation and manifests remain frozen. All own Original handles are now terminal; final release/composite awaits lawful compiler capacity and the retirement owner's coherent checkpoint.
