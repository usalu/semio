# Note guest staging

The prototype browser guest uses the owned `components iterate note` route. Its managed Cargo driver builds `semio-hub-note-component` for `wasm32-wasip2`, then transpiles and publishes the browser component. The admitted Cargo invocation was still compiling dependencies when recovered. The existing process was retained to preserve its lane and avoid a duplicate build.

The prototype browser products already contained the Note playground session, Preview 2 vendor assets, shard support, and the shared font artifact. The owned serve route reached activation and reported that the Note module has yet to be staged.

## First build

Driver report `wF0epx3` records 817 compiled units and one failed unit. Rust rejected the PDF lexer's `data.retire_empty(turn)` call in a pattern guard with E0596. The current source already contained a peer correction that moves the mutable call into the pattern body. The owned guest build was started again with the same compiler target and build directories. Driver run `wTo80FQ` captures the retry.

## First retry

The warm retry reused 449 Cargo units at its first compile checkpoint. It found E0599 in the framework surface refresh. The rank lookup yields an optional utility value. The refresh now uses `as_deref()` through `and_then()` to borrow the utility string and preserve the existing absent-value behavior. The Cargo failure preceded the correction.

## Second retry

Driver run `wECswFj` emitted fresh WASI library artifacts for the corrected plugin program and for instance dispatch. It reached E0499 in the PDF normal-object cursor. The length arm now copies both scalar fields out of the borrowed phase before calling the cursor method with mutable access to the whole object.

## Third retry

Driver run `wCltzUW` advanced beyond the previous borrow failure and recorded 818 Cargo units. Its remaining error was an xref-stream call to the removed PDF lexer `ready_frame()` method. A peer corrected the current source to call `ready_value()`. A new warm build was launched against the corrected source after the previous compiler pass settled.

## Fourth retry

Driver run `waWNSG1` advanced through the corrected xref stream and recorded 819 Cargo units. The PDF 1.4 graph projection reported two E0308 mismatches in a partially converted cross-reference index. The reader now wraps its classic object offsets in normal xref entries, passes the resulting index to the local resolver, and keeps the existing object-number check. The next warm pass started after driver settlement.

## Fifth retry

Driver run `wgVSZvz` built the corrected PDF graph projection and compiled or reused 857 Cargo units. The first Note-owned error came from deriving `RetainedClone` for a snapshot containing a standard `BTreeMap<String, NoteImageAsset>`. The Rust compiler reported 22 diagnostics derived from the same missing trait. No browser guest was published.

## Sixth retry

Driver run `w2kdfo8` started after the Note asset schema migrated to the existing retained ordered map. The prior compiler lane `pc800cc` disappeared from the compiler cache between runs. The admitted build allocated `pbf8ed4` and reprocessed the dependencies there.

The cold pass settled after 816 Cargo units. PDF inferences compiled against a version of the stream decoder that lacked three newly called constructors. The live decoder now defines those constructors, so no duplicate implementation was needed. The next run uses a one-hour parent command budget to accommodate cold cache and shared host scheduling.

## Seventh retry

Driver run `wjXqDHc` cleared the PDF inference errors and the Note asset-map trait errors. It reached 858 Cargo units and found compile errors in the Note diff and snapshot I/O crates. The existing Note presence retirement implementation is now the sole implementation, and the presence record derives DSL receiving. SQLite native decode now writes through the framework-owned output slot. The existing native SQLite fixtures now exercise that slot through the shared original-owner test helpers.

## Eighth retry

The snapshot codecs now compile under the crate that defines the snapshot. The descriptor assets remain in their own facet to keep source admission within its cap. Hub's lock reflected the local package change and driver run `w1QuKiB` entered locked compilation. The Note workspace lock was updated for the same two path-package edges while preserving its development dependencies and registry pins. The standalone lock refresh waited for exclusive registry custody, so its queued invocation was cancelled after the Hub lock changed.

The ninth run settled after 815 Cargo units. Its only error was a missing borrowed canonical-tree implementation for PDF indirect objects. That model now derives the same canonical tree contract as its reference and dictionary-entry siblings. The current Hub and Note locks both carry the descriptor dependency and omit the removed snapshot I/O package. All 20 distinct embedded schema files exist under the path-length limit.

## Tenth attempt

Driver run `wUiPJfP` exited in the workspace planning phase when Cargo encountered a duplicate BRep analysis dependency in a concurrently edited native-registry manifest. The current manifest contains exactly one entry. The eleventh attempt started after that process settled.

## Eleventh attempt

Driver run `wWYCh29` passed planning and compiled the new Note schema descriptor for WASI. The native cache had evicted the prior idle compiler lane, so it rebuilt in a new lane. Cargo settled after 700 units with two instance-document errors against a concurrently updated mounted-genesis API. The live owning source already defines both methods and its ingress caller passes the current operation/frontier pair. The twelfth attempt started after this run settled.

## Twelfth and thirteenth attempts

Driver run `whLhjxA` compiled the previously failing instance-document and PDF snapshot crates, then built the Note diff and reached the mutation family. Six Note block, asset, cell and nested text records now derive the DSL receiving contract their typed mutations require. Recursive block declarations use the existing boxed child cursor.

The DSL receiver also lacked both the funded handoff of individual terms from a statement list and unwrapping of the generated block-statement field. It now retains the authored block shape, unwraps it under the existing release budget, and allocates one owned statement at a time under the actual capacity effect. Borrowed receiving dispatches those variant records directly. Shared cases cover empty and nested groups, Unicode arguments, exact JSON results, cancellation and denied grants. Ajv and the Bun oracle passed all four tests and eight assertions. The native Rust fixture remains to be run.

The thirteenth driver stopped before Cargo with a workspace configuration selection error in shared compiler ingress. The parent agent is addressing the mismatch between capture and adoption of the normalized Cargo arguments.


## Fourteenth attempt

Driver run `winOvPi` entered real Cargo after the parent fixed the captured configuration selector. It compiled the updated DSL runtime and proc macro, then settled at 820 Cargo units after a missing direct lexer dependency in the original PDF graph projection. The live PDF manifest and Hub, PDF, and Note locks include the peer’s correction. Attempt 15 started after the previous driver settled.

Attempts 15 and 16 reached the genuine Cargo compiler. Attempt 15 (`wZS5xeD`) processed 859 units and reported the missing borrowed DSL receiving trait on the generic persisted child header. Attempt 16 (`w5Kwzvx`) reached a stale captured engine call to the removed `admit_apply_batch`. The live engine owner had already migrated that call to the paged admission method, so no store engine change was needed.

The child receiver now retains five typed string slots across a borrowed native operation. A malformed final field exercises retirement of the four copied fields. The valid cases compare all persisted identity fields and confirm an absent local owner. The existing Ajv and SQLite oracle passed one test and ten assertions. The Rust test source uses the same neutral corpus and is pending a managed native test run.

## Attempts Fifteen Through Thirty-One

Attempt 15 compiled the corrected PDF graph projection, then reached the Note schema and reported the missing borrowed DSL receiver for persisted text children. The store child now receives its existing published header through retained, metered text slots. The language-neutral child receiver fixture and Ajv/SQLite oracle cover malformed target shape. The native fixture adds rejection of a later text field after earlier copies and checks paid retirement.

Attempt 16 picked up an intermediate store cursor signature and stopped. Attempt 17 compiled the corrected store engine and the retained child receiver. It then stopped on intermediate PDF importer and plugin declaration APIs. The current declaration defines `NaturalFileDecodeStep::Yield` and the current PDF retirement helper requires `RetireOwned`. Attempt 18 compiled the corrected PDF mutation package, the Note schema and diff, and five Note mutation packages. The native compiler then found both Note composite scene call sites passing the declaration enum to the runtime scene API. The composite now uses the presentation contract for runtime scene calls, matching the existing viewer and keeping the declaration contract for window definitions. Attempt 19 stopped sooner when it captured two competing Viewport2d receiver implementations. The live viewport binding now contains only codecs and the mounted receiver module supplies the single implementation. Attempt 20 compiled the consolidated viewport. It stopped on an OS DSL receiver borrow conflict in an in-flight source revision. The current converted receiver copies the flag before creating the mutable closure, leaving both admitted effect routes intact. Attempt 21 compiled past that borrow error and exposed two OS DSL references to a lower-level crate outside the package’s manifest. Both references now use the existing facade reexports. Attempt 22 compiled the OS DSL change and reached 848 units, including reuse, then stopped with seven PDF page editor diagnostics from a single undeclared fonts dependency. The shared page-editor manifest now directly declares the owner and all six affected locks agree. Attempt 23 compiled the PDF page editor and the Note delete-blocks package that mounts the canvas renderer. It then found the Note composite-window camera mutation lacked the metered binary receiver required by the window config owner. The camera mutation now uses the existing ordinal pack operation and original receiving path. The existing neutral window fixture exercises parity with the original binary receiving law. Attempt 24 stopped in the controlled member-open store on an obsolete cast from the u64 work quote to u128. The admission now compares the two u64 work quantities directly. Attempt 25 compiled the controlled member-open store and the Note kinds catalog with the camera mutation receiver. Remaining compiler diagnostics found three camera and Ink commands still treating their owned optional view model as a dereferencing wrapper. They now borrow the model through Option::as_ref. Attempt 26 captured an unspaced range pattern in a concurrent instance-docs change. The current source already contains the spaced pattern. I cancelled the failed keep-going run after 682 compiler units and confirmed exit 130 before starting attempt 27 with the correction. Attempt 27 compiled the Note command suites and reached the versioned editor crate. Dispatch now borrows the view from either host entry point without cloning the shared record. The retained command closes its owned remnant through the current original effect and exact metering. Attempt 28 captured an intermediate GLB metadata API. The current JSON Object owner exposes both borrowed indexed lookup and in-place swap with the required signatures. I cancelled the failed keep-going pass and confirmed exit 130 before starting attempt 29 with those methods. Attempt 29 compiled the versioned Note editor, retained dispatch and command crates. The final Hub composition lacked the direct semio-framework-value crate used by its generated closed PluginApp dispatch. The composition now declares the workspace dependency, and the Hub Cargo.lock has the exact matching edge. Attempt 30 reused 876 units, compiled the final Hub composition and linked the 127,190,577-byte Note WASI component. Browser staging stopped because the native transpile tool was missing on this host. The normal component target already declares the tool as a dependency. The graph-independent wrapper ran the tool’s owned release build, reached exit zero and staged the native executable. Attempt 31 reenters the managed browser packaging path so model, transpilation, and component receipts retain their original provenance.

Attempt 31 initially reported two missing imports from a PDF document editor change. The current manifest and all seven affected locks declare both owners. The component driver observed additional changed source and restarted compilation once under the still-live lane, so I retained the existing process and observed its new pass.

The managed retry compiled the corrected PDF editor and linked the Note guest again. It passed native transpilation and then stopped because the prototype descriptor emitter receipt was absent. The normal component graph declares the descriptor writer before the component task. For the direct workflow, the ticket wrapper now runs the emitter’s owned prototype build and preserves the executable’s actual compilation receipt.

The descriptor producer compiled 572 units and published the mode/platform executable with the original compilation receipt. Attempt 32 retried with all build tools. Cargo linked the Note guest, and all 11 managed stages completed with zero driver failures. The published Note module contains the compiled core, bindings, bridge and both descriptors and receipts. The outer wrapper returned 75 on its trailing shared-generation recheck after the successful publication. The owned ServeScript independently opened the actual product, wrote the activation receipt and served the editor entry with HTTP 200.

The first live browser loaded the Note wasm, mounted the shell runtime, and dispatched setActiveExample. The command was refused because the Note config owner did not declare its original window-store disposer birth. The owner now delegates the demand to the same bounded disposer it constructs. Attempt 33 compiled and linked the Note guest with the runtime correction, including native persisted camera pair receiving. Provenance correctly refused to publish after the same PDF image source was consumed at different versions during the build. The old browser receipt stayed unchanged. Attempt 34 linked again but observed a second concurrent change, this time in the framework Pack value cursor. It settled without replacing the module. Attempt 35 linked all 878 Cargo units. Staging stopped during fresh artifact capture because the shared lease directory could not grant its open admission inside the configured 5-second patience. The linked WASI file exists in the run directory. The browser module and activation remained on the prior build. Attempt 36 started after the previous command returned terminal exit 1.

## Staging work files

- Local optional utility borrow fix. [🪟️surfaces/native.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/native.rs)
- Copy the PDF length phase fields before borrowing the cursor. [🧭️normal-object/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/🧭️normal-object/🦀️.rs)
- Complete the peer's existing PDF 1.4 xref conversion. [🚪️io/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🦀️.rs)
- Derive the existing canonical-tree contract for the indirect-object wrapper. [🔗️object-model/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs)
- Keep one retirement implementation and derive DSL receiving. [👥️presence/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs)
- Use the actual native output slot and original-owner tests. Overlaps the parent’s map and snapshot I/O work. [📸️snapshot/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs)
- Add six DSL receiving derives. Overlaps the parent’s retained map migration. [🧬️schema/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs)
- Reconcile the local Note descriptor and removed snapshot package edges. The peer PDF dependency refresh later also touched this file. [🗒️note/Cargo.lock](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.lock)
- Borrowed statement dispatch. [🫴️receiving/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🦀️.rs)
- Owned statement receiver declaration. [🧬️contract/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🧬️contract/🦀️.rs)
- Fund one statement handoff at a time. [📋️vector/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/📋️vector/🦀️.rs)
- Retain the statement keyword until paid retirement. Fix found in the WGPU agent’s audit. [🎁️wrapping/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎁️wrapping/🦀️.rs)
- Unwrap authored block statements and select owned enum recipients. [📥️owned/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📥️owned/🦀️.rs)
- Unwrap authored block statements and select borrowed enum recipients. [🪢️receiving-and-projection/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🪢️receiving-and-projection/🦀️.rs)
- Native nested statement receipt, denied-grant and cancellation exercise. [🧊️field-receiving/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🦀️.rs)
- Independent canonical fixture check. [🧊️field-receiving/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🟦️.ts)
- Shared empty and nested statement vectors. [🧊️field-receiving/🔣️.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🔣️.json)
- Recursive fixture schema. [🧬️schema/🔣️.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🧬️schema/🔣️.json)
- Extend the build forwarding budget. Overlaps the parent’s owned serve wrapper changes. [NOTE-INFINITE-CANVAS-APP/stage-note-guest.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/stage-note-guest.ts)
- Guest compiler audit. [NOTE-INFINITE-CANVAS-APP/guest-staging-2026-11-20.md](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/guest-staging-2026-11-20.md)
- Publish the borrowed persisted identity from its paid string slots. [🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs)
- Exercise borrowed success and retained refusal against the existing neutral corpus. [🛬️receive/🧪️tests/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🦀️.rs)
- Verify the persisted identifiers and malformed target with the independent Ajv and SQLite oracle. [🛬️receive/🧪️tests/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🟦️.ts)

## Attempt 36 and the next staging run

The run began at 23:35 local time on November 20. Cargo linked all 878 units and the build staged the linked artifact. Source provenance then refused `prototype-00` because the compiler’s consumed bytes for the plugin maintenance ladder matched neither an admitted witness for that path nor the current physical file. The admission memo recorded 81,918 bytes at 23:41:37, rustc recorded 82,031 bytes at 23:45:08, and the live file changed to 82,325 bytes at 23:53:04. Those are three distinct versions of the same admitted source. The live file and the next admission memo agree on the latest version. The outer wrapper exhausted its snapshot retries, exited 75 and cleaned the failed generation. The published browser receipt remained unchanged.

Attempt 37 began after the changed source settled. It uses the existing per-run lease policy setting to allow 30 seconds for the shared directory admission gate. The fresh artifact claim and the source-verification rules remain in force. The longer gate wait addresses the separate post-link timeout observed in attempt 35.

Attempt 37 linked 878 units after 280 recompilations and 598 reuses. The widened directory gate outlasted the artifact-capture admission. Provenance refused a different shared source, the plugin document store replacement initializer. The active compiler unit had read 29,812 bytes at 00:07:48. After 00:13:31 the live file contained 31,128 bytes, which the later native compiler also recorded. A read-only replay of the repository's source closure confirmed that the unconditional Rust module is included in the standard source roster. The failed generation's witness directory was cleaned, so its exact admitted body could not be inspected afterward. The browser receipt still points to the previous publication.

Attempt 38 runs with early source witness capture. The prototype driver reads dep-info adjacent to each newly compiled workspace artifact before advancing Cargo's output stream for its single component. The reader uses the retained input authority, confirms the source was already admitted, and retains only a payload whose length and BLAKE3 checksum match the compiler's record. A later provenance pass still independently selects the witness and seals the publication. The neutral fixture and independent Noble BLAKE3 oracle passed five cases, including a later edit, concurrent capture, and refusal of an unrelated source path.

Attempt 38 exercised early witness capture during compilation and linked the real guest. Publication then found a distinct receipt issue. The final Cargo uplift carried checksum-free transitive dependencies and was merged into the target unit alongside the byte-identical internal compiler artifact's checked dep-info. The two actual retirement-contract compiler records agree on the older input. The uplift's later physical read caused the contradiction. The compiler dependency ownership audit records both artifact hashes, the source evidence, and the fix under test in attempt 39.

Attempt 39 warmed the retained lane after a concurrent plugin ownership trait edit and Cargo completed its retry with 878 artifact announcements. The final verifier found a missing witness for the OS DSL converted receiver. The actual demarcation is exact. Rustc wrote the absolute checksum for the 12,969-byte source at 01:35:19.204301. The edit to 13,591 bytes arrived at 01:35:19.240753, before rustc wrote the metadata artifact at 01:35:19.524840. Cargo's artifact announcement came too late for the new capture path to retain the old body.

Attempt 40 uses the existing bounded same-generation retry for an additional grounded case. A published compiler checksum with no retained matching body can produce retry evidence only when a stable physical read proves a different BLAKE3 and byte length fingerprint inside the admitted workspace. The driver checks again for the exact original body before asking Cargo to retry. Re-admission and the normal final provenance seal still run before publication.

Attempt 40 published the new Note browser module after all eleven native stages passed. The outer worktree generation check returned 75 afterward. The browser receipt and a fresh activation independently identified the published source and artifact digests.

Attempt 41 includes the exact host publication owners for tool and utility switches and the two paid retirement progress corrections. Rustc's dep-info checksums match the current body of all four relevant files. The same evidence shows that the Pack retained record decoder was compiled before the archive receiving fix. A later stage is required to publish that decoder change.

Attempt 41 stopped at the Note component while the shared macro call and definition were converging on the fourth `prepare` argument. The compiler reported an unnumbered macro error during the overlap. Attempt 42 started after both source files exposed the same call shape. It also includes the later Pack decoder correction.

The bounded warm retry now checks the exact failing Cargo target source first, including when rustc omits an error code. Retry still requires proof that an admitted file's bytes changed. Dependency drift analysis continues to require a diagnostic with a recognized error code. The language-neutral fixture reproduced the missing target witness before the correction. The failure-churn suite then passed 26 tests with 75 assertions.

Attempt 42's warm generation `wpGiJqh` compiled the retained Pack record decoder. Its checksummed rustc dependency record matches the live corrected decoder source.

The same generation’s checked rustc records also match the current reserved tool and utility owner, full-operation witness, Store retirement frontier, and mounted publication pump. All five source bodies were compiled from the intended fixes. Final linking, provenance, and browser acceptance are still pending at this checkpoint.

Attempt 42 linked the Note component, sealed 4,925 compiler provenance files, and completed all eleven staging steps with zero failures. The command returned successfully. The published source tree digest is `22e4c66a0c3c0c6176f3295f92a50054e30db9894f9a351c32097203d1accb59` and the browser Wasm SHA-256 is `eeb3696464232e5db7e057e1b9635f6bf7253d4135d46e8378294c1a0df0426c`.

The attempt 43 compiler records match the corrected command capture and expanded Store stall witness, alongside the five earlier corrected files. Attempt 43 already had retained worker outputs when the later archive receipt correction was written. Their checksums identify the previous worker code. A further artifact publication is required to test the worker correction.

Attempt 43 completed all eleven staging steps and published a fresh module with 4,925 sealed provenance files. Its component source digest is `24136bd3a1a2ce3e2c78d25f329851abaf640546c0171c81f77958cc2b3fb02f`, and its Wasm digest is `a0981aea6c702d2b5a0dc509dbf8b0c6f5c5c22bc17978f31dfb21dfb9db3a92`. The HMR-off Vite launcher accepted the publication and served the Note route. Focused Text and Pencil browser tests use this guest while attempt 44 stages the subsequent worker close receipt correction.

The attempt 44 dep-info matches the current worker receipt reconciliation in the worker authority, retirement slot runner and worker session runner. This establishes that the archive correction compiled for the Note guest before final linking and publication.

The first attempt 44 Cargo pass observed a live command-route edit and reported three missing `original_press_selected` fields. The native driver found an exact admitted-source mismatch in the updated action router and began its bounded same-generation retry. The retry immediately reused 372 compiler units under the held lane. This is independent live evidence for the recovery path while the exact fixture integration still needs its own compiler context.

The retried Cargo pass compiled the command route without the original missing-field diagnostic. The checked dep-info for the retry identifies the current reserved metadata owner, issuer, normal completion and instance teardown sources. It also identifies the current paid Closing Store route and worker close receipt reconciliation.

The attempt 44 retry linked all 879 Cargo units with exit 0 and sealed compiler provenance. The descriptor stage then exited 2 because the installed descriptor emitter rejected `--prepare-actor`. The current descriptor emitter source accepts that argument. The repository-owned descriptor emitter build is refreshing the prototype host tool before another publication attempt. The browser still serves no artifact from attempt 44.

The first descriptor emitter rebuild exited 75 after proof observed a concurrent edit to the reactor receipt source. It had compiled 539 units before the snapshot refusal. A new owned emitter run started after the change. The installed executable is still the old artifact and does not contain the actor preparation option.

The second descriptor emitter build completed. It reused 539 Cargo units, published a fresh approved host artifact and the executable contains `--prepare-actor`. The executable’s SHA-256 is `dfeaa3bd93c140ae7aab2bc5c4716641eea617af3b1840cf4b46870f571df67d`. Attempt 45 started only after the tool published.

An owned standalone probe ran the refreshed descriptor emitter against the exact Note component linked by attempt 44. Both the native describe and probe wrapper completed. The probe emitted the semantic JSON and Pack descriptors and a 22,045,230-byte prepared actor image. The fixture remained local to the ticket generated directory. The attempt 45 product awaits normal provenance and staged publication.

Attempt 45 linked 879 Cargo units, sealed 4,925 provenance files, and completed all eleven stages with no failures. Its current browser module source digest is `0eb29d1b5283cd2b7b2cd80da3f5c1a5e0604fc10908183262e83f03e2401ff0`, and its Wasm digest is `8b3e73b2c45a0cf56c7f37487cce631c25a0a1b29ae02ed1cc20848065b62442`. The owned HMR-off activation served the Note route over HTTP 200.

The attempt 46 compiler lane is `wQgb47c`. While the build ran, its checksum-bearing rustc dependency records matched the current archive replacement initializer, the maintenance ladder, the WindowConfig paid refresh correction and the reactor effect-view borrow. At that check the Note inline command had no compiler record yet.

Attempt 46 continued with 724 completed compiler units after 41.6 minutes. Of the unresolved Cargo unit-graph records, 129 came from the stdio artifact subtree and 16 came from Note. The optional native suite commands were cancelled through their own handles to prioritize the live guest. The ticket's forward staging wrapper now allows three hours for any successor run because it previously had a one-hour fixed wall timeout alongside the native driver's progress watchdog. The already running attempt retains its original deadline.

The initial attempt 46 pass captured the PDF inference graph before a shared manifest and source update. It reported E0432 against the old graph. The active driver verified the new input and began a held-lane Cargo retry. The replacement graph includes the direct pixels dependency, and the first native progress report during that retry recorded 144 reused units.

Attempt 46 was terminated by the original 3,600,000 ms forward wrapper timeout. The driver had recorded 1215 real compiler invocations over its initial and held retry passes. The held retry had no new compiler diagnostic at its last sampled point. It did not stage a component, so the browser receipt remained on attempt 45. Attempt 47 started after the owner reported released leases, using the longer wrapper timeout and current PDF input.

Attempt 47 cleared its compiler-image admission after 649 seconds and reserved the expected 6.0 GiB with 11.9 GiB measured free. The supported repository admission and reclamation flow handled the wait. The warm generation is `w2OTETc`. Its initial pass had 322 Cargo artifact records and triggered a held-lane retry when the compiler witness disagreed with the current replicated plugin app implementation. No initial-pass Rust diagnostic was recorded at the time of the retry.

Attempt 47 ended during the retry source check when the instance-app package reached 263,166 bytes against its 262,144-byte limit. The refused read was the composer API at 14,601 bytes with 13,579 bytes available in that checked closure. A fresh census against the current replicated sources found 18 files totaling 259,426 bytes. Both the command policy and package policy enforce the 256 KiB bound. Attempt 48 started against the current in-bounds sources.

Attempt 48 cleared image admission after 70 seconds with 19.6 GiB free. Its workspace source admission completed and the owned warm compiler generation is `wJjV81j`. The instance-app source auditor independently measured the latest guest and native fixture closures at 259,465 bytes across 18 files. No package or central limit changed.

The attempt 48 driver refreshed the held graph twice after concurrent edits to the plugin root and the instance maintenance ladder. At 17 minutes of native work it reported 1,248 reused Cargo units across the retries. The retry-2 checksum claims match the current archive initializer, paid WindowConfig refresh, Pencil effect-view borrow and separate maintenance-ladder source. The Note command had not reached its own compiler unit at that check.

The attempt 48 retry-2 Cargo lane compiled the Note diff, mutation, artifact and v1 units without a compiler diagnostic. The retained rustc checksum claim for the inline `ink-apply-events` source matches the current edit implementation. The current archive, camera and Pencil source claims also match within the same retry pass.

Attempt 48 retry-2 completed the Note guest Wasm link successfully. The driver captured and staged one component and began browser publication stages. The installed browser module receipt is still on the previous guest while the publication tool's compiler-writer admission waits on volume headroom.

Attempt 48 completed the describe step and shared its three-file result to the fleet cache. The first remaining failure was the model stage's immediate exclusive `cache-prune` lease timeout during compiler-image recovery with 2.8 GiB available against a 4.0 GiB requirement. The transpile refusal followed the closed process admission. The retained linked guest is 127,556,752 bytes, and browser publication still needs a live model receipt. A complete repository-owned replay must use the warm driver because the published browser bundle requires that original receipt and provenance.

The maintenance owner now treats the exact nonblocking `cache-prune` timeout as a deferred attempt when compact compiler headroom recovery opts into that outcome. The caller remeasures and follows its existing progress and cancellation loop. Actual pruning still acquires the complete original guard, and other callers retain their typed failure behavior. Warm attempt 49 started with the correction while its targeted lease regression was running.

Attempt 49 then stopped before compilation with ENOSPC opening the headroom claim at `u/k/headroom.lock`. Its admission log had reached a rounded 0.0 GiB free while two other commands still occupied the shared compiler image. This is independent of the previously corrected cache-prune timeout. No new guest or browser receipt was produced.

Attempt 50 did not reach compiler admission. The replicated component driver had added a named descriptor capability import whose emitter implementation and CLI command were missing. The TypeScript check, schema validation and Rust `capabilities` command are now in place. The emitter request suite passed 53 assertions and the actual driver imports resolved. The owned prototype descriptor emitter rebuild is running before the next Note staging attempt.

The first descriptor capability tool rebuild exited 75 after detecting a concurrent edit in the reactor context call input source. It did not replace the installed binary. A new owned prototype emitter run started after the changed source was observed.

The second host descriptor attempt also exited 75 after a new edit to the reactor context input. The third attempt is live and waiting for the specific shared descriptor-emitter compiler lane. Its exact holder is another component's existing browser build. The admission line mentions the total live lane count, but the source confirms it is a failed claim of the named product lane rather than an exhausted lane pool.

The eighth owned descriptor capability build completed successfully. The installed prototype emitter has SHA-256 `01c88f1793e8867fe4c1941d998838288acd34826ddaf12315523b611019c64f` and contains the authored version-1 actor-preparation payload. The preceding snapshot retries were restricted to confirmed source-change exits. Attempt 51 started using the newly installed emitter and the current warm staging policies.

Attempt 51 passed the installed descriptor preflight and cleared compiler-volume admission after 134 seconds. It entered warm generation `w2Fqq8r` with 17.5 GiB available at admission.

Attempt 51 completed the guest link and sealed compiler provenance. It entered the real browser publication stages with the rebuilt descriptor emitter admitted. At this observation the installed browser receipt was still the previous guest.

Attempt 51 finished with 897 compiler units, one staged component, all 11 publication stages, and zero failures. The published prototype browser receipt changed to source digest `dc848381f98c79502dbed55829b0fc6f9d7c2db975913bf03ed1f6347b96a068` and Wasm SHA-256 `c64817ab5e55632787d8831ff52e05d9d00ea38524b7550b5c0229f673b8503e`. The repository-owned Vite activation served the Note editor on port 6080 with HTTP 200. Separate full, Pencil and eraser, and focused archive browser runs started against that published guest.

Three independent Chromium sessions on attempt 51 reached the Note shell and stayed on its “Loading plugins…” screen through the acceptance timeout. A startup probe confirmed the page received the actual Note plugin snapshot as sequence zero on the configured development WebSocket. The Canvas and Navigator were absent, and page-level resource timing showed no Note guest download. The readiness probe now recognizes the shell's element-level ready and fault beacons in addition to the document attributes.

The next probe traced the browser's instance-open turn on the actual published Wasm. A context-level request trace corrected the page-level absence of guest resources. The shard fetched the Note core, activated successfully, and entered the native open. Chromium's V8 CPU profiler found a tight loop in the cyclic runtime instance registry during the 39th guest poll. The native raw-media picker scan revisited earlier occupied slots when an instance lacked a picker effect. The source guard now exits when a returned index lies behind the cursor. Attempt 52 released the previous staging gate after the optional WGPU test yielded its reservation, then admitted the owned Note build with 9.7 GiB free and a 6.0 GiB claim.

In the attempt 52 compiler run wE56uxw, checksum-bearing dep-info from the actual runtime ops and runtime ops surface units matches the current picker wrap guard and both background/refresh view-borrow fixes. The Note editor/viewer archive replay producer wiring landed during the same run. Note artifact/host checksum validation still depends on their compiler units running.

The same attempt 52 dep-info now matches all three current archive replay source files. The compiler verified the shared issuer helper in the Note mutation crate and the editor and viewer registrations in the Note v1 crate. The component still needs linking, staged publication and final fresh Chromium tests.

Attempt 52 published successfully with 880 compiler units, all 11 publication stages, 4,977 provenance files and zero failures. The installed Note receipt has source tree digest f7487c49e1e97b6bc0ba9a89137a6a24d4011baccdfe9e7a7135ea85cd7c352c and Wasm SHA-256 03d0acb41446622c8ee4fb484280adcb68989250b77dce3b3b5dbbb2a97b4f76. Its sealed linked component is 99,358,291 bytes. The local Vite route returned HTTP 200 on port 6080 after the previous server's owned runtime expired. Three fresh Chromium scenarios began without the isolated picker replacement.

Attempt 53 is running as wsiZgfi. Checksum-bearing compiler dep-info matches the current Source-versus-issued retirement admission, the newer shared native history projection guard, and both Pencil SurfaceVisible ownership files in the first or retry pass. The camera tracing also found a shared WindowConfig constructor admission change. The retry compiler claims match its contract, store owners and owner registry while the publication-unit claim still requires its unit to finish.

The attempt 53 retry compiler dep-info now matches the current typed-operation publication unit alongside the three WindowConfig owner and contract files. The preceding attempt 52 camera trace independently enters canonical publication phase 5 well after Store construction, so the construction change alone cannot establish successful camera acceptance.

The retry also contains the current ArtifactStoreCanonicalSource work quote for phase 5. The Wasm debugger recorded an issued Work limit of 3 and a child quote of 4 in the old linked guest, with 53 copy bytes selected. The published attempt 52 receipt predates this source change. Both Cargo run logs refer to a mutable unit-cache path, so the current dep-info at that path proves the retry's input and cannot establish a historical claim for attempt 52.

Attempt 53 linked the Note component with 880 units. Browser staging then refused the missing prototype native transpiler. The run retained a 99,354,568-byte linked artifact under wsiZgfi with SHA-256 a31b3cba10bfdd5e3f70f7aa94519072cc90b750917c8edefbed0520e2c48865. Browser publication requires the live model admission, receipt and writer from the managed driver, so that linked artifact is useful for isolated diagnostics while a warm managed attempt publishes a new receipt.

Attempt 54 started as wjymwNY using the supported SEMIO_NATIVE_TRANSPILE_BIN override pointing at the existing repository-owned release executable for this host. The prototype descriptor emitter already exists. The expired Vite process was replaced by a new owned ServeScript and returned HTTP 200 on port 6080. Browser acceptance remains gated on the fresh sealed receipt and the matching served core.

The isolated diagnostic transpiled the exact linked artifact to the ticket generated directory with the repository's production transpile helper. Its core measures 99,269,501 bytes. The current binding has different Wasm trampoline numbering from attempt 52. The isolated browser route therefore serves the generated binding together with its core and applies the same preview2 import rewrite as browser publication. The live Vite publication is untouched by this diagnostic.

Attempt 54 caught a mixed-source error when the first pass compiled instance rendering against earlier ViewModel APIs. The driver reran in the held lane and compiler dep-info for all three Pencil projection files matched the live source. The same retry observed earlier versions of the five archive maintenance wake files, which landed later. The managed source-witness check has two additional retry suffixes available.

Attempt 54 installed a fresh real browser receipt with source digest c1da6ae189d9851334d8ecbbca29dfa2b6baf80862597ecbf8adf82c08a10b7b, component SHA-256 ef4ed9a41879dab2812e696cef67af18a8968053428ae40f75888e2004c0dc0d, and 99,348,606 component bytes. Vite served the new 99,263,539-byte core. The command returned status 75 after ten stages because pass reclamation had removed the claimed host unicode-ident rmeta before final provenance recheck. The direct browser camera test passed against the new served bytes. The linked snapshot predates the archive maintenance wake, initializer receiving-scope fix and later shared close-stall accounting.

Attempt 55 started in warm run wpl13R0 with the latest archive wake, receiving scope, cleanup and compiler claim retention sources. The latest driver keeps observed original compiler outputs through provenance seal unless an exact retained output witness has already replaced the original. The custody suite passed eight tests and 52 checks. The long-lived owned Note Vite route is available on port 6080.

## Managed Attempt 55

The managed build published Note after 880 Rust units and all 11 staging steps, with zero reported failures. It sealed 4,977 provenance files and finished in 2,180 seconds. Driver report: `.cache/b/r/m/components/runs/wpl13R0/r/report.md`.

The browser receipt identified component SHA-256 `a6b57e42c9836f6201473bb3d34a5f715b8435f6ac2df2350bb416e2eba34485` and source tree digest `43af9d03688f26ca1ad2cfbf81c4eaea91ae29d8de821d51204b2adabc80b341`. The real development server returned HTTP 200 for the staged core and reported its expected 99,276,774-byte length. The earlier provenance failure from losing the observed host metadata artifact did not recur.
