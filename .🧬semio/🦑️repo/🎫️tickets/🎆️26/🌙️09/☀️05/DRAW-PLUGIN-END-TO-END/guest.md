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

## Envelope return accounting

The envelope field-decoder and completed-record return pumps previously reported zero on a partial child step even though the child had debited its original receipt on the same context. Both now keep the original owner in its slot and forward the child's progress. The independent SQLite fixture verifies the expected four-axis receipt, pending parent state, and retained owner. Both focused return-pump source laws passed in the repository test-file run. The file also reported three unrelated stale assertions in worker handback and typed publication tests.

## Native Draw guest harness

The full native Draw library first reached the composition and failed compiling its Wasmtime harness at existing lifecycle API changes. The harness now uses the framework's original native fixture caller, its funded Source retirement, and effect-driven retirement of turn and event payloads. It imports `TurnFault` from the guest contract that owns it.

The pinned direct build of the full Draw native library suite, including the Wasmtime lifecycle tests, has started.
