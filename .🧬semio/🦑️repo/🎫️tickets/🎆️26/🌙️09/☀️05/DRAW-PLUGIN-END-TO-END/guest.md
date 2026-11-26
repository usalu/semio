# Draw component and guest acceptance

The canonical Draw component linked on November 25 at 21:08 and entered the managed prototype model at `.cache/b/p/m/browser/prototype/components/285067b377def16ed6e8c388`. The receipt names package `semio:draw`, target `wasm32-wasip2`, and 143779747 bytes. The managed model and producer artifact match the receipt SHA-256 `fcc8e11c2267be890c236bf25f06d9e378b87b4becc78bebb74d95a6bcfb10e2`.

The TypeScript guest oracle passed against this canonical model's emitted JSON descriptor. Its Ajv checks exercised editor and viewer identity, the native caller policy and its negative cases. Its SQLite query independently confirmed the authored mounted-owner release vectors. The earlier r47 model passed the same checks before replacement.

The native test targets now include the surface release-policy fixture and schema in their cache inputs. Both files affect the oracle and the release policy test. Bun's independent glob implementation verified that the named-input pattern matches both files.

The canonical native route is `repo run @semio-tech/draw-plugin:guest-instance-check --lang=en`. It waited in graph inventory and was cancelled with exit 130 to release capacity. The managed Cargo fallback reached dependency compilation against the r47 component bytes and was cancelled with exit 130 so the working direct compiler lane could proceed. Native acceptance requires Wasmtime to instantiate the Draw component, settle and acknowledge both Draw app opens, retire both instances, close their actor contexts, and retire the native callers.

## Stale operation recovery

The browser guest previously returned an actor-level error when the publication ladder found a stale immutable document root. The publication driver kept the operation in its registry and retried the same error on subsequent turns. Draw's camera action publishes window configuration and does not commit a document change. A document commit can still invalidate a camera operation that captured the previous root.

The publication ladder now freezes an inline diagnostic under its original retained grant and cancels that operation's lease. The existing cancellation path hands Store custody back and publishes the operation's fault page. The freshness gate continues to reject stale roots before more Store mutations run.

The neutral publication fixture stages a pending presence write at preflight with a stale root. The Rust law checks that it emits `interactive-job.stale`, leaves the lane uncommitted, retires the operation, and accepts a subsequent publication on the same app. Ajv accepted that fixture and rejected variants that committed the stale write, reported a guest trap, or failed to advance the next generation.

The bounded completion report now includes the required zero retained-progress record without allocating. Its Rust and TypeScript implementations fit the escaped message into the remaining frame and preserve the diagnostic code. The TypeScript fixture suite passed with 21 assertions, including an exact 128-byte code plus a long message against the 480-byte frame.

The focused tool-job coverage check and its host-only publication cases passed. The repository freshness law accepted the current source and refused variants that bypassed freshness, skipped local result reservation, or skipped cancellation of the affected lease. The broad latest-wins source-string harness still reports mismatches in existing command ingress, effect extraction and page reservation anchors. The focused check runs independently of that harness.

The dedicated Rust integration target `publication-boundaries` passed with 1 test and 0 failures using the pinned offline native toolchain. It checks the actual fault page, the unchanged stale lane root, exact paid retirement, and a successful next operation. Run it through `repo cargo --store=shared --configuration=prototype -- test --manifest-path <semio-plugin-tests-12-2 owner's Cargo.toml> --test publication-boundaries`.

The focused harness reconciles physical page releases against both the net caller receipt and the original birth ledger, retires the mounted cancellation lease through its issuer, and verifies app closure under its authored policy. The old companion lib-test target remains blocked by existing compile failures across reactor-driver and other fixtures. Its separate linearization case also refused an outdated receiving-scope quote when it was run alongside the focused law. The focused integration target excludes those older cases.

The first canonical browser run completed the editing, clipboard, PDF, history and persistence checkpoints. A window configuration publication during reload and reconnect then returned `original window authority is stale` to the whole actor. The typed publisher now checks the committed window partition's identity, address, generation and full revision before entering or continuing WindowConfig publication. A mismatch uses the same paid, operation-local `interactive-job.stale` result. Previously accepted receipts still complete through their original acknowledgement path. The lower Store admission independently enforces the same window authority predicate.

The window freshness fixture and live source law passed. The hostile variants remove the window authority comparison, bypass it before Store work, or discard the local result reservation. The pinned native `semio-plugin-tests-12-2 --test publication-boundaries -- --nocapture --test-threads=1` run passed both the document and WindowConfig laws in 0.63 seconds. The window law verified the stale page and its acknowledgement, unchanged committed root, and full retirement. It then captured the same partition, committed through a WindowConfig receipt, acknowledged a nonfault Terminal page, and checked the committed revision changed. Its setup uses the shared native Window genesis fixture. Its retirement helper checks the issued close grant, which covers the actual retained mutation and snapshot headers.

## Envelope return accounting

The envelope field-decoder and completed-record return pumps previously reported zero on a partial child step even though the child had debited its original receipt on the same context. Both now keep the original owner in its slot and forward the child's progress. The independent SQLite fixture verifies the expected four-axis receipt, pending parent state, and retained owner. Both focused return-pump source laws passed in the repository test-file run. The file also reported three unrelated stale assertions in worker handback and typed publication tests.

## Native Draw guest harness

The full native Draw library first reached the composition and failed compiling its Wasmtime harness at existing lifecycle API changes. The harness now uses the framework's original native fixture caller, its funded Source retirement, and effect-driven retirement of turn and event payloads. It imports `TurnFault` from the guest contract that owns it.

The refreshed native build passed the instance and maintenance crates. The host warden terminated its parallel PDF model compile. A pinned single-job retry reached a shared-tree API race in `semio-plugin-reactor-turn` while the matching `LazySlots::resident_slots` and `take_next_occupied_slot` updates arrived. The next build passed reactor compilation and reached another shared-tree race during PDF model crate separation. A fresh locked dependency query resolved the four new PDF model providers. The pinned `semio-hub-draw --lib --no-run` build then succeeded. The editor/viewer dialect, viewer mutation refusal, and mounted release tests passed natively (3 passed, 0 failed).

The native lifecycle test selected by `--exact plugin::guest_instance_tests::quick::wasmtime_runtime_instance_open_settles_against_the_draw_component` reached Wasmtime's allocator validation with the hash-verified canonical `fcc8e11c` component. The old production pool rejected its core module with `module table does not fit in pooling allocator requirements`. Reading the component's core declarations independently confirmed a fixed table of 182,233 entries, 25,014 globals and 426,354 defined functions. The previous Wasmtime pool and per-Store policies limited tables to 20,000 and 100,000 respectively.

The production pool and Store now share a finite limit of 262,144 entries per table. Pooled core and component context limits are 16 MiB and 32 MiB respectively. The native capacity fixture uses Wasmtime to initialize a component with the shipped table size and a deliberately large core context, then checks the Store and pool both refuse a larger table. The managed capacity test held `libc` at an estimated 5.9 GiB plus its 2.4 GiB host margin. A pinned, single-job native fallback advanced from that lane but was retired to preserve link capacity for the full Draw build. The repository-mounted engine tests were then compiled directly with the actual e25 Draw native dependency artifacts and passed all four cases in 8.06 seconds. The capacity case alone took 6.47 seconds. The full Draw native host is compiling against the same e25 store.

The full native host rebuild reached a shared PDF writer/Decryptor source transition. The refreshed PDF sources supplied the missing `RetireOwned` implementation, and the hub lock gained precisely the authored `semio-framework-value` dependency of `semio-s-artifact-stdio-pdf-lifter`. The locked restart progressed. Independently, the native production isolated compiler began cold compilation of the pinned fcc8 component with the shared pool active and its output directed to the lifecycle runner's cache.

## Draw native SQLite snapshots

The model companion's erased SQLite snapshot tests now run the framework's original codec helpers with actual native decode and encode authorities and a separate SQLite retirement recipient. They retain both native encodings, exact row counts, Draw snapshot equality, the independent Bun SQLite oracle for all authored IEEE words, and the foreign-dialect and cancellation checks.

The directly mounted test file linked against the e25 production Draw dependencies and ran all four tests. The declaration and rejection/cancellation cases passed. Both positive cases reached erased import and exposed `UnsupportedOwner: assets.field owner has no controlled native projection implementation`. The chosen `PagedMap` binding had controlled shape creation and decoding but lacked the corresponding owned projection. It now uses the existing guarded map producer and copies each paged UTF-8 key under that original encode control. The native provider's failure occurred before native payload publication, after it had reconstructed the relational rows.

The focused negative case was rerun with a valid drawing payload and exact assertions for the foreign dialect's `UnsupportedOwner` cause and the original initial `DecodeNative` cancellation checkpoint. It passed (one test, zero failures). A fresh direct build of the actual DSL model package also passed a native map projection check under genuine original encode owners. It projected an empty key and a multi-page Unicode key with exact values and refused an interior progress callback. The full SQLite positive producer cases are part of the fresh Draw model companion build.

## History identity budget audit

The native clipboard law reached undo's reserved commit and hit the native encoder ownership limit. The old reserved permit used the route's 1,024-byte output ceiling for undo's independent, cumulative Store identity encoding. The parent team's route change adds the Store's 1,048,576-byte one-item allowance to that output bound. Grouped history can spend over several participant routes under the same identity receipt. Exact per-action charge can be read from the original resumed control immediately before its pause. The saved progress observer can lag the final charge because it reports at checkpoints.

## Files touched on November 25

- `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🦀️.rs`
- `🌎️hub/Cargo.lock`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/⚠️fault/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/⚠️fault/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/📤️publication-contracts/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪝️tests-12-2/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍌️tests-05/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🏁️publication-boundaries/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🚫️cancellation-publication-boundaries/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🔐️two-turn-presence-and-root-retirement/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/♻️envelope/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📜️contract/🔏️pack-authority-and-publication/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🧪️tests/🏊️capacity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🧪️tests/🏊️capacity/🔣️.json`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/guest.md`

## Cached native lifecycle with production imports

The separated `semio-hub-draw-guest-tests` package owns the native editor and viewer lifecycle test. Its `WasmtimeRuntime::new` installs the generated actor bindings, the framework operation imports and the real WASI p2 linker. The test admits an original actor context, drives and acknowledges instance capture, drives instance retirement, closes the actor context, and pays host-side retirement for both roles. It now has an opt-in `SEMIO_DRAW_GUEST_CACHE_ONLY` path that uses the production compiled-cache reader and refuses a miss. The ordinary package test still calls the normal Wasmtime compile path.

`guest-native-cached.py` verifies the fcc8 WASM, raw descriptor, bound receipt and 590,573,904-byte native artifact. Its `--preflight` passed through `repo tools exec --light python`. The verification can take a current executable via `--executable`, or `--cargo` can compile only the separated guest-test package with managed Cargo. Execution takes a managed 1,536 MiB grant, selects the exact unit test and requires evidence of both role closures and one passing test. The driver rechecks the AOT hash after execution. No current native guest lifecycle executable or complete guest-runtime rlib was found in the earlier native build stores, and no compiler was started while the component build held the host.

The fleet remote executor answered its capabilities and execution probes and advertised 13 confined macOS arm64 workers. This target cannot run strictly remotely through the current repo interfaces. The Bazel native Cargo action and its general host-state script action are explicitly tagged `no-remote-exec`. Cargo unit distribution uses opportunistic remote execution, while `--offload=pin:<origin>` permits local fallback on pin failure and unsupported unit kinds stay local. A strict offload command for the entire native test was unavailable.

The additional files are `🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🦀️.rs` and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/guest-native-cached.py`.

## Compiler broker and inherited parent memory

The original broker returned headroom for a first compiler whose predicted peak exceeded its parent grant. It also admitted a compiler while its driver's other resident memory exhausted the remaining allowance. The new owner tests reproduced both behaviors before the fix.

The generic Cargo launcher and component driver now pass a parent allowance read from the same live host-capacity ledger. The compiler broker uses the active reservation, independently of the carried tool envelope. Quiet, lent, missing and mismatched reservations supply zero compiler memory when an enclosing grant was declared. The broker charges each running compiler and the driver's observed other resident memory against the parent. A compiler whose forecast exceeds the original allowance receives a terminal, bilingual refusal with its unit, predicted peak and active allowance. Host-availability fallbacks cannot enlarge the parent allowance.

An explicitly fleet-only pass can use cached artifacts. A local compiler request for that pass receives a distinct fleet-placement refusal, which the component driver can use to requeue without launching the compiler locally. Local requests without an enclosing reservation still use the existing soft host share and warden behavior. The parent cap governs broker admissions using forecast and observed bytes. A running compiler can use more than its forecast before the next telemetry sample, so the existing warden still enforces its runtime policy.

Five managed `repo test-file` suites passed 27 tests. The new ledger, oversized first requester, driver memory, fleet-only and retry cases used the actual sockets, with SQLite as the independent allowance and arithmetic oracle and Ajv as the wire oracle. The two production-callsite and process-capacity laws also passed under a filtered repo Bun run. The standard-library Rust scheduler owner compiled directly under a 256 MiB repo capacity grant and passed all 13 tests under a 128 MiB grant. Peak RSS was 176,766,976 bytes during link and 2,752,512 bytes during the test. The broader native-artifacts runner was retired before its separate Cargo fixture completed. No Draw or workspace Rust build was started for this change.

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🪆️parent/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🪆️parent/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🔭️probe/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`

## Native verification of the next Draw publication

`guest-native-current.ts` runs the existing dedicated native executable produced by `guest-native-cached.py --cargo`. Take its absolute path from the `Lifecycle executable SHA-256 … at …` line. Use the exact WASM SHA-256 from the successful new component's receipt. The runner refuses the old fcc8 digest and a mismatch among the selected WASM, current semantic descriptor pack, published descriptor pair and receipt. It verifies the shared guest scenario contract and pins copies of the verified publication for the test.

```sh
repo tools exec --light bun -- "$T/guest-native-current.ts" plan \
  --sha256 "$DRAW_NEW_SHA256" --home "$T/🗑️generated/wasmtime-fcc8-production"

repo tools exec --light bun -- "$T/guest-native-current.ts" run \
  --sha256 "$DRAW_NEW_SHA256" --home "$T/🗑️generated/wasmtime-fcc8-production" \
  --executable "$DRAW_GUEST_TEST_EXECUTABLE"
```

For a cache miss, add `--cold` to the second command after the component compiler run ends. The cache path uses the selected WASM's BLAKE3 hash under the actual Wasmtime metered pool or on-demand namespace. Cached execution receives a 1,536 MiB reservation and forces the production cache reader. The cold path runs the same executable and requests the larger of the runtime estimate (256 MiB plus 48 times the WASM bytes) or the measured prior cold peak (5.1 GiB plus a 25 percent margin), plus 512 MiB for the host test and coordinator. A byte size equal to fcc8 yields 6,838 MiB for the native estimate and a 7,350 MiB managed reservation. The repo capacity manager independently applies the desktop reserve. The executable's real native compile gate still reads current native memory. The requested reservation is a forecast, and the driver records the new maximum resident set and elapsed time.

The admitted child rechecks its own live reservation, executable digest, and presence of the exact test. Evidence requires both roles to report open, close and retirement, exactly one passing Rust test, and the selected executable's content-addressed artifact. A synthetic valid-format component verified plan identity and found the same pooled namespace as the prior actual fcc8 compile. The driver refused the old digest, a cold miss without opt-in, and an actual 128 MiB managed reservation against a 7,041 MiB synthetic forecast without running the test executable. The replacement component and dedicated test executable were unavailable during verification preparation. No host or Wasmtime compilation ran.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/stale-publication-smoke.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/draw-native-rustc.sh`

## Cold native Wasmtime admission

The production isolated compiler finished the registered fcc8 Draw component and atomically wrote its 590,573,904-byte native cache entry. A macOS sample recorded a 5.1 GiB physical footprint in the final assembly phase, where the main thread links the generated function bodies. Compiler-worker parallelism governs one part of this cost. Package size and retained native code also determine memory demand. The full native Draw lifecycle build was stopped after the macOS memory incident. The cached artifact was preserved in the ticket's private external store.

The shared Wasmtime engine now compiles serially. The MCP gateway runs one cancellable cold worker per process and joins requests for an identical cache destination. An advisory lock at the shared cache root admits one cold compiler across processes using that root, including native compilation outside the gateway. The worker takes its lock and samples memory before reading the component. The admitted length bounds its stream, and the worker checks available memory again before entering Wasmtime. Its estimated requirement is 256 MiB plus 48 times the component bytes, with the repo's 12.5% desktop reserve bounded between 1.5 and 4 GiB. The Darwin sample uses host VM counters and normal-pressure state. Linux reads physical available memory and a visible cgroup's limits. Windows compares available physical and committed memory. A missing or pressured sample and insufficient room defer cold compilation. The gateway reports a retryable budget error for a worker's admission refusal. Filesystem and compiler errors retain the permanent error path. Existing compiled cache loads bypass cold admission.

The memory guard measures current headroom prior to compilation and can race future allocations from other tools. Its estimate is calibrated to the shipped fcc8 component. It applies across processes whose cache root supports advisory locking. The host probe returned normal memory headroom for a minimal component in the managed macOS test. Runtime sampling and accounting on native Windows and Linux remain unverified on those hosts.

The focused harness compiled the actual repository admission module and passed all four mounted tests and the live Darwin counter probe. The compile/link and test used repo capacity grants of 384 MiB and 192 MiB. Maximum RSS was 132,497,408 bytes for the build and 2,965,504 bytes for the run. The independent Python flock oracle, bounded stream and repo policy floor reproduced all twelve fixture cases. The full Draw native lifecycle and refreshed positive Draw SQLite cases remain open.

The cache-only Wasmtime 47.0.3 harness loaded the same fcc8 AOT file with the serial compiler setting and the production pool and Store limits. It instantiated the actual Draw component and its async reactor reported channel version 25. The harness supplied typed trapping implementations for all imports. The tested reactor entry invoked none of them. Repo grants were 1,024 MiB for the link and 1,152 MiB for the run. Link RSS peaked at 444,448,768 bytes, execution RSS peaked at 526,467,072 bytes, and execution took 7.21 seconds. The harness used `Component::deserialize_file` only. The actual native host-bound clipboards and other Draw lifecycle turns still need the full Draw test executable.

## Files touched on November 26

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🔋️memory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🔋️memory/🍎️/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🔋️memory/🐧️/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🔋️memory/🪟️/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🔒️compile-admission/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🧪️tests/🏊️capacity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧊️compiled-components/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🏁️child-heads-hub-commit-and-probe-pack/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/guest.md`
