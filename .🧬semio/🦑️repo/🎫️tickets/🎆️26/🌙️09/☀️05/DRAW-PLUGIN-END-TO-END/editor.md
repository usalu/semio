# Draw Editor Verification

## Fixes

- Removed eight byte-identical duplicate package entries from the Draw artifact `Cargo.lock`. Cargo previously refused to parse it. Repo Cargo metadata completed successfully with offline and locked arguments, and the dependency preparation selected zero version updates.
- The browser keyboard path now sends the paste action and target window through the clipboard queue. The dispatch funnel reads the fragment when the queued action gets its turn. This prevents a held fragment from overriding the selection of a newer copy or cut. The fixture starts with a prior selection and checks both subsequent writes and pastes in submission order.
- The native Demo-archive law now observes both effects returned by the dispatch and those emitted by subsequent work. It uses the shared fixture settlement and close procedures, so result-page loans are retired before acknowledgement and terminal completion is drained. It asserts that exactly one load is published.
- A second native archive law authors a rectangle, verifies that only the SPR changed, closes the writer, and loads the exact Pack and SPR into a fresh Draw app. It compares the restored snapshot with the full edited snapshot.
- Draw's `setActiveExample` now clears both its layer and point selections via the local interaction lane as well as publishing the requested file load. The reported zero layers with one selection came from the current guest interaction view, while the archive loader replaced the document store. Both renderer routes submit the registered Draw command, so the clear applies across the normal example switches. The command law covers registered examples and the empty drawing.
- Draw's native document and document member catalogs now declare the same whole-document conflict target as its existing app operation route. The catalog also installs the original edit message producer and the artifact's persisted operation receiver so restored and newly created stores can use the same native history wire.
- Updated the route completeness test with the existing export command's publication contracts and replaced the duplicate retained limits in the browser-grant layer test with the mounted component policy. The inspector event binding fixture now retires materialized built nodes separately from UI values.
- The vector port test drives mounted scene preparation and reads it with the current fixture render identity. A command-only view cannot address the mounted scene.
- The model companion drives vector and fill work through the existing owned retirement fixture. It retains the original heap and receipt assertions. The separate I/O test gives borrowed export a real I/O control so its refusal checks the route instead of a missing caller.
- The embedded child admission cursor uses the existing inline retained-owner gate. The allocator trace showed that the previous macOS pthread mutex dropped a lazily allocated 64-byte backing beside the correctly quoted 1,344-byte cursor. The framework admission law now also drops the closed wrapper during a measured turn.
- The host companion inspects presence through feature-gated mutation candidate queries. Its exact rollback cases now bind the original slot replacement and advance it independently of the operation's execution identity. Candidate steps fund a bounded native field page so controlled comparisons and copies can advance.
- Draw image asset import keeps its completed native text preparation while the sample clone advances and returns both empty clone authorities after publication. Duplicate builds its suffixed name through a growable funded native text owner, preserving the source name and retiring the exact displaced clone name.
- The native host catalog fixture describes separate directory and payload releases for each lane, with matching SQLite and Rust oracles. The retirement laws distinguish scaffold moves from payload copies and continue comparing native allocation and release receipts.
- The deep snapshot clone closes under the same payer and original source it used to advance. Its fixed native frame is admitted separately from the 4,096-byte payload body.

## Checks

- Renderer window action panel, 22 passed using the owner Vitest configuration.
- Draw publication authority passed after the selection-lane change, with 69 Ajv cases, 38 routes, five hostile mutations, and the Scheduler oracle.
- The earlier English port 6064 journey recorded 3,024 boot pixels and no logged errors, then timed out waiting for the pasted layer. Its copy step had only submitted the shortcut. The coordinator's trace confirmed that the copy completed with no clipboard effect, and paste entered without a fragment.
- The canonical English browser report verified zero selection after both the Blank and Demo switches. Selecting the restored layer and running copy, paste, and cut produced the expected layer counts of one, two, and one. The journey also observed the rectangle, undo, redo, persisted revision, and a real changed PDF. Reconnect after page reload failed on stale window configuration authority.
- The complete Draw JavaScript owner script passed. Its 15 Bun test invocations counted 715 passes, and the strict TypeScript, image admission and publication checks completed.
- The current WASI build compiled the Draw command crate containing the example-selection fix. The linked guest is still being verified separately.
- The repo-managed native test stopped making compiler progress after 312 units, with all 18 Cargo wrappers asleep and no active fleet action for that build. The local-only retry waited at memory admission. Pinned native Cargo built the three suites with one job and a private target in the ticket, with the repaired lockfile enforced offline.
- The exact Demo archive law passed with the registered fixture context. The initial native run reported one inspector failure and one original export-read cleanup failure. The full editor rerun includes the fixture, history and shared document loader changes. The host suite's first compile failed on one ambiguous fixture conversion, which the publication owner corrected before the rerun.
- The native retest combines the three editor suites and eight sibling owners in one locked, offline Cargo build. It uses the pinned toolchain, a serial compiler frontend, and the standard 128 MiB test stack. The host capacity warden terminated the intervening nonserial exact export build.
- The first serial build passed the previously terminated kernel unit and then reached the plugin reactor. The reactor compile observed an older `LazySlots` without two methods already present in the shared checkout. The same serial command restarted against the updated dependency sources.
- That retry passed the reactor and encountered another shared-tree update in the PDF model, whose source had switched to four split crates after the build's dependency graph was loaded. The latest locked retry resolved those four packages from the updated artifact lockfile.
- The refreshed build compiled the Draw editor and native store catalogs. The model test companion still imported the Pack package by package name even though it declares the Rust library name `pack`. The test now uses the declared name through the extern prelude, and the native aggregate restarted.
- Near the end of compilation, the drawing PDF exporter switched to separate PDF lowering and conformance crates. The shared checkout added the direct dependencies and both workspace lockfiles before the next locked native retry. The editor did not need an exporter or descriptor change for that compile error.
- Focused native tests passed both I/O laws and all five fill laws. A second focused build recompiled the framework I/O ports and Draw model against the inline gate. All four vector laws passed, including the physical release and original-source custody assertions.
- The framework I/O ports owner passed all seven admission and provider laws, including retirement of the closed cursor during the measured turn.
- The fresh cold Draw overlay passed the nonempty archive reopen law. The law verifies that a new app restores the authored rectangle from a changed SPR and the same genesis Pack.
- Focused native Style, Structure and Transform owners passed 47, 34 and 56 laws respectively. The second editor companion passed 31 laws and the editor interaction companion passed all 42, including the inspector event binding case.
- The Scene companion first passed 43 of 48 laws. All 18 preparation laws then passed in a focused native build, covering the five reported failures. That executable opened macOS stderr before observing allocations. The current source removes the historical trace from the paid turn.
- The same focused stderr procedure passed all 30 editor command laws, including image publication and atomic selection. The fresh native Draw host image asset import and removal law also passed, including exact reference refusal.
- The Draw host native-owner Bun suite passed 22 laws with 400 assertions after the catalog and independent-axis quota fixture changes.
- All 37 Draw host native companion laws passed after recompiling the current host, arena and digest against their matching native dependency set. This includes the image asset path, every mutation, depth and duplicate capacity refusal, stale replacement rollback, and full original-source snapshot clone closure.
- All five native Examples laws passed against the matching dependency set, including Demo native text and binary decoding.

## Reproduction

- Run `repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p semio-s-artifact-draw-drawing-editor-tests -p semio-s-artifact-draw-drawing-editor-tests-2 -p semio-s-artifact-draw-drawing-editor-interaction-tests` from the Draw artifact workspace.
- The normal Draw JS target waited in the repository graph scanner. The ticket's `draw-native-dev.ts` launcher invokes the same declared package `test` script with its issued repo process envelope and uses the same route for the renderer's `test long window-action-panel`.

## Native compilation boundary

- The combined Draw editor store preparation library occupied about 315 MB in both native caches. The member wire receiver now owns its production crate at `🏭️member/📨️wire`. The store preparation crate still owns document and member registration, the shared typed issuer, and the persisted operation authority.
- Both Cargo workspaces resolve the new edge in locked offline mode. Metadata-only serial checks completed for the wire and store preparation crates at about 220 MB and 197 MB peak resident memory. The existing member catalog and wire law target also type-checked at about 201 MB. LLVM code generation and linking were outside the reduced-memory verification.
- The full native aggregate was stopped during the machine memory incident, with its completed providers preserved. The independently verified native owners above have recorded results. The integrated SQLite companion did not run against the refreshed provider closure.

## Focused regression driver

- `repo tools exec --light python3 -- "$T/run-draw-native-regressions.py" all` checks and prints the captured dependency plan. From the repository root, set `T` to this ticket's path. The planning command verified the current source paths, cached external crates, and manifests without starting a Rust compiler.
- Add `--run` when the local component pipeline is idle. The DSL branch recompiles the current record model, facade, and derive closure and runs only the optional statement law. The Draw branch recompiles the current metadata owner and runs the entire current mutation test root against the captured native dependencies. The two branches run in sequence and record separate compile and test results under `🗑️generated/focused-regressions`.
- The DSL fixture keeps an original allocated statement vector even for the absent case. It checks empty and present statements against the neutral JSON expectations and tests paid cleanup at cancellation prefixes. The cached Draw provider closure can still exhibit the historical factory-ticket stderr allocation and does not establish a fresh framework verdict.

## Files changed on November 25 and 26

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml`
- `🌎️hub/Cargo.lock`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🪜️candidate-step/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/📐️footprint/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🔏️digest/🪞️duplicate-rewrite/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🧰️kit/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🧩️candidate-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🪞️duplicate-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/✏️edit-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🔎️lookup-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🧬️snapshot-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🏗️initialization/📚️catalog/🧪️tests/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🥣️companion/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧵️vector/🧪️preparation.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🎨️sampling/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧾️bounded-commands/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/🏭️member/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/🏭️member/📨️wire/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/🏭️member/📨️wire/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️archive-load/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/⌨️command-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🖼️canvas-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🪟️window-laws/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/📋️clipboard-order.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪪️window-action-panel/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧪️tests.rs`
- `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/📖️provider/🧪️tests.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/editor.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/run-draw-native-regressions.py`

## Component pass parent memory

The component driver now reads the broker owner's verified live parent reservation once and shares it across profile lanes. The driver RSS baseline is charged once, each local pass reserves its predicted final peak plus the authored local process footprint, and each fleet pass reserves the local process footprint. Whole-MiB rounding is conservative. A pass that cannot fit its local peak chooses a viable strict fleet placement or immediately returns a bilingual error carrying the required and granted MiB. A failed fleet placement returns its reservation and is readmitted through the local gate. A fallback that can fit waits for active reservations to settle. A permanently oversized fallback terminates. A build without an explicit parent reservation retains the existing host guard and opportunistic distribution.

The strict fleet branch registers the pass with the actual admission broker before spawning Cargo. If a wrapper asks for local admission, the broker denies it. The driver retires the Cargo attempt and converts the broker's placement result into a requeue only after the issued input callback settles normally. It buffers linked component publication until the placement result is known. The first implementation threw the retry marker inside that accepted input callback, which closed the Original before requeue. The isolated integration law exposed the failure and now covers the corrected path.

The pre-fix scheduler failed 11 of 14 new tests. The final job-slot run passed 45 tests with 17,979 assertions, including language-neutral route vectors cross-checked with both Python and Ajv. The first driver integration law observed zero Cargo compile launches with an oversized pass and a verified 2,048 MiB grant. The fleet law observed exactly one Cargo attempt, a real broker `fleet-placement` refusal and a final parent-grant error before a local retry could launch. Both passed through `repo test-file` in isolated issued processes. The test Cargo process uses actual metadata and lockfile operations and an explicit local admission protocol exchange. No Rust build or live worker run was performed for this change.

### Component memory files

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧪️tests/🎟️parent/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧪️tests/🎟️parent/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧪️tests/🎟️parent/🐍️oracle/🐍️.py`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🎭️stub/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🎟️parent/🟦️.ts`

The existing driver streaming suite passed all six tests through the same exact repo test door, including unreserved local serialization, fleet overlap, and bounded local memory waits.

## November 26 mutation failure attribution

The captured mutation run finished with 32 passed and three failed. The failing binary cases were empty-group receiving and nested-page receiving. Its other failure was the retained visibility inverse's exact allocator receipt. The later cached executable lists 36 tests, including a new addressed rectangle case. Replaying its four native original-receiver cases without compilation produced one pass (rename) and three failures (rectangle, empty group and nested group). All three failing producers expose the same empty statement block under Drawing's default fill. The addressed rectangle carries a nonempty parent and index and reaches the same failure, which rules out those optional scalars as the shared trigger. The nested case additionally traverses 180 path segments.

The Draw model declares `DrawingAttributes.fill: Option<FillStyle>` with `#[dsl(statements, block)]`. The generated owner in `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📥️owned/🦀️.rs` releases the wire's Box before giving `FieldValue::Statements([])` to the typed optional cursor. The live fix in `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎁️wrapping/🦀️.rs` now stages that original vector for paid close and produces None. The owned statement cursor still requires exactly one row for a present value. The native control comparison and third-party JSON expectations are in `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🦀️.rs`. That focused law now tests both the direct optional field and the generated record/block route Draw actually uses, for each absent/present and cancellation vector.

The retained visibility test uses the real source owners and compares measured requested and released bytes with the original receipt. Its captured failure was (591, 255) measured against (0, 0). Replaying only that case under `--exact --nocapture` reduced the mismatch to (64, 0), immediately after the old `[DEBUG] cft` line. The independent exact-size Darwin allocation trace already in the ticket records the same 64-byte mutex birth through Rust stderr and `close_factory_ticket`. The live ticket owner at `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎟️tickets/🦀️.rs` has no print. No distinct retained-inverse receipt defect is evidenced by this run. The real Draw law is `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/↩️inverse/🧪️tests/🦀️.rs`. The binary cases are in `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`.

The focused driver now requires enumeration of all four binary receiver cases and the retained inverse case, checks the entire executed count against the enumeration, and labels its provider freshness. Its DSL branch recompiles the current optional receiver and generated owner. Its Draw branch is a regression differential using current Draw mutation sources with the captured transitive providers. Only a Draw build linked through current DSL and retirement providers can close the three receiver regressions and the retained inverse regression end to end. The audit launched no additional compiler.

## Draw member wire WASI carve

The managed WASI prototype run recorded the member wire crate at 1,691,175,168 requested resident bytes peak (1,613 MiB rounded up), against the 1,536 MiB library budget. Its 122,769,610-byte archive has 49,288 WebAssembly functions with 14,850,675 bytes of function bodies. Parsing the archive's actual WebAssembly linking symbol table assigned 8,025 functions and 3,846,226 code bytes to the native record machine, 5,541 and 3,597,642 to the native receiver protocol and 9,455 and 2,463,205 to native-owned machinery. The repeated NativeReceiverBackingClose family appears in 1,449 symbols totaling 1,802,525 code bytes. The single largest body at 10,782 bytes is `OriginalJsonReceiving<DrawingMutation>::advance_turn`; the enclosing wire close entry is 9,015 bytes. These measurements distinguish the deep native schema monomorphization from the small typed store adapter.

The staged eight-file patch moves only the concrete original JSON advancement to `semio-draw-member-decode` at `📨️wire/📥️decode`. The existing wire owner still retains the same inline reader, original source witness, exact grant, scoped Work and ledger, cancellation/retirement state, typed mutation handoff and original factory. Passing the same owner by reference across an out-of-line non-generic function introduces no allocation or another authority. Both workspace lock entries and the normal library's build owner row are in the staged patch. Its dry-apply checked cleanly and preserved the active snapshot's source inputs. The virtual preimage produced one missing Cargo owner. The proposed rows matched canonical owner reconciliation, passed the scoped consistency check and matched Bun's independent TOML parse. The new package's longest modeled compiler artifact path is 197 UTF-16 units.

The language-neutral budget fixture uses the production admission policy and independent SQLite integer arithmetic. Running it against the actual current telemetry produced the expected red row with exactly 1,691,175,168 bytes and a 1,536 MiB boundary. The split mode requires fresh rows for both owning crates to pass. The existing wire fixture also enumerates canonical original mutation, ledger refusal, all grant-axis denials, paid parser cancellation and exact typed issuer retention. Those runtime tests are necessary after the structural change.

The observed unit graph already uses opt-level s, 16 codegen units, LTO off, symbol stripping, no incremental compilation and no debug information. The archive contains 18 Wasm objects. The largest includes 8,690 functions and 2,329,224 bytes of function bodies in 13,113,559 archive bytes. The earlier native comparison used opt-level 2 and changed codegen partitioning and embedded bitcode together. It cannot establish whether a package-only CGU change meets the WASI budget. The ticket's isolated measurement recipe resolved the actual cached WASI fingerprints and validated all eight dependencies without compiling. Its 16/32/64 sweep intentionally changes only the requested partition size relative to the reconstructed settings. The normal Cargo profile owner presently authorizes only package-scoped opt-level overrides, so adopting codegen policy would require its schema/render/verification support.

### Carve inputs and verification

- `draw-member-wire-carve-26.patch` stages the two new decode owner files, wire source and manifest, Draw workspace registration, both workspace lock changes, and the normal build owner declaration.
- `draw-member-wire-budget26.json` and `draw-member-wire-budget26.ts` contain the language-neutral memory cases and independent oracle.
- `draw-member-wire-measure26.py` plans one isolated Rust unit against the existing WASI provider closure; its default dry plan starts no compiler.

### Current optional statement DSL execution

The first issued native rerun exposed a stale cached `semio-framework-dsl-record-ownership` provider missing the current `retirement::retire_field` API. The focused driver now rebuilds that owner in the same fingerprint-directed closure before the current model. Under `repo capacity run --memory=1536 --cpu=1 --jobs=1`, all seven compile steps settled and the one exact optional statement law passed. This is an executed current-source law, including both the direct optional cursor and Draw's generated statements/block route and the full cancellation corpus. The running product component retained its original input snapshot throughout the diagnostic.
