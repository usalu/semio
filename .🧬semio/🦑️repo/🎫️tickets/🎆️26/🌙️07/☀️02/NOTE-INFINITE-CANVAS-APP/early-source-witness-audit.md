# Early consumed source witness capture

The source witness now reads the neighboring dep-info candidates of an announced compiler artifact without taking an authoritative output observation. It normalizes absolute paths before checking the build roots, so Windows separators work. It considers only sources already registered in the workspace roster. A body is retained only when both its length and BLAKE3 agree with the rustc checksum line. Missing or concurrently changed files leave the authoritative receipt to resolve the evidence. Cancellation and unexpected filesystem failures propagate.

Concurrent capture tasks share existing content-addressed byte storage. Association admission checks its bound again after the asynchronous blob write, so overlap cannot spend the same last association slot twice. Early capture leaves the selected and consumed version state for final provenance.

The focused Bun test passed all five neutral cases with ten assertions using the independent Noble BLAKE3 implementation. It exercises duplicate concurrent captures and confirms the retained intermediate body remains available after the source is replaced. Equal-length wrong bytes, wrong length, sources outside the roster and a missing dep-info candidate do not add evidence.

- [Source witness](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📎️source-witness/🟦️.ts)
- [Focused test](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📎️source-witness/🧪️tests/🟦️.ts)
- [Neutral cases](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📎️source-witness/🧪️tests/🔣️.json)

The live Note run wZfUlVc had retained two additional source bodies after Cargo opened its output log, before the completed invocation's provenance pass. This shows the compiler-announcement route retaining evidence during a real build. The final browser receipt is tracked in the staging report.


The early capture covers absolute dep-info checksum paths. The final receipt resolves relative checksum paths from the compiler unit's established base directory.

## Stable mismatches at artifact announcement

Attempt 39's missing source witness used an absolute dep-info path. The source changed 36 ms after rustc wrote dep-info and 320 ms before it emitted rmeta. That gap was enough for early capture at artifact announcement to see newer bytes. The witness now returns the admitted source path and both BLAKE3-and-length fingerprints when a stable read proves an exact mismatch and the expected body has no retained witness. An exact match retains the body. An absent or over-budget body supplies no mismatch evidence. The driver owns retry admission.

The focused Bun run passed both tests with 20 assertions. The neutral cases cover same-length changes, length changes, absent and unadmitted sources, and a replacement after the expected body was already retained. Noble BLAKE3 supplies the independent fingerprints.

## Retry composition and fixture ingress

The complete stub driver fixture reaches the real Source admission before dispatching its stub Cargo. Source admission uses the actual Cargo planning port and resolves the checkout compiler configuration through its current directory. The full fixture therefore hit the repository ingress guard at Cargo’s compiler version probe. The supported full route is the compiler store provider’s scoped configuration input port. Acquiring the shared store during an active Note build was deferred.

The bounded composition law ran real dep-info parsing, retained witness storage, warm graph admission and the retry loop. An accepted repeat retained the exact source body and reached the stage spy. A repeated identical mismatch stopped after one retry with no stage event. The focused source witness suite passed three tests with 31 assertions without opening the repository compiler store.
