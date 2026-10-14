# Native Tooling and Bazel Storage Qualification

This continuation records actual commands and authored Source on 2026-10-14. The ticket and parent goal remain active. Existing live SDKs, ordinary cache baselines, Git state, and worktrees were preserved.

## Source Changes

The normal Nx bootstrap retains the authentic 8 GiB tools store throughout SDK acquisition, activation, and loaded Node child settlement. Bun, SDK snapshots, downloads, staging, and temporary paths descend that fixed backing. Toolkit publication and activation now execute the actual production `semio-authorizers.mjs` in Node and verify seventeen required runtime functions. File identity and exact SHA-256 remain unchanged throughout that probe. A hash-valid but non-loadable bundle is refused before activation.

The frozen journal worker uses the workspace's canonical lease namespace rather than deriving it from the former toolkit directory layout. The graph seal's repository-relative `createRequire` call was replaced with its equivalent static import, so the frozen bundle embeds the observed Source rather than asking Node to locate an absent repository-relative module after relocation. The Nx owner subsequently supplied the precise adjacent declaration.

Bazel executable and embedded installation writes use tools; its fixed module setup, downloads and repository blobs use dependencies; its generated workspace, output roots, action cache and temporary writes use products. `withBazelStorageV1` retains all three authentic fixed stores until owned children settle. All stores must match the declared partition roots. Bazel uses batch invocations; continuous services receive separate output bases, avoiding an idle daemon writer after the original store callback or an output-base lock against their consumer build. Original build mode now contributes to Bazel task identity.

Binaryen acquisition now requires an authentic tools store and writes archive, staging, executable and libraries beneath it. Extraction and version probes inherit its bounded temporary environment. Its selected reader path uses the same canonical mounted tools namespace.

The static Cargo target and build paths now descend `rust/fixed/m/cargo/shared`. Managed prototype/release facades retain their explicit issued paths. Native grant projection reads the actual authored Cargo directory configuration and preserves independent explicit target/build overrides; the separate ordinary `canonical-architecture` fallback was removed. Raw Cargo launches without the original enforcing wrapper are not qualified by this change.

Permanent source commands are registered for tools partition, Bazel storage corpus, Bazel native runtime, Binaryen partition and toolkit preflight. Private Nx ticket routes isolate these qualifications from unrelated whole-workspace rediscovery. Editor launch entries are coordinated with the root owner because another owner regenerates the shared launch file.

## Actual Evidence

| Command or Gate | Observed Result | Generated Log |
| --- | --- | --- |
| Normal `bun repo --version` | GREEN; actual Node Nx v23.2.0 after real SDK provisioning | `tooling-store-bootstrap-red1.log` |
| Direct bootstrap `nx --version` | GREEN; SDK clone 4,363 files / 91,606,802 bytes; actual Node Nx v23.2.0 | `tooling-store-bootstrap-red2.log` |
| Tools physical and reusable charge, private Nx prototype | GREEN / 1.6 s; image file and allocated bytes 8,588,886,016; one authentic allocation reused three times | `tools-partition-nx1.log` |
| Pinned Bazel normal setup | GREEN; verified native executable acquired into tools backing | `bazel-bounded-setup1.log` |
| Bazel path corpus, private Nx prototype | GREEN / 998 ms; one law / ten assertions; independent SQLite and Ajv | `bazel-storage-private-nx1.log` |
| Cargo authority law before repair | Genuine RED; one passed / one failed, current configured target differed from ordinary fallback | `bazel-cargo-authority-red2.log` |
| Cargo authority and Bazel path corpus, private Nx prototype/release | GREEN; two laws / fourteen assertions; independent TOML, SQLite and Ajv | `bazel-cargo-authority-green1.log`, `bazel-cargo-authority-release1.log` |
| Initial backed Bazel runtime, prototype | GREEN / 46.6 s; two fixed production partitions newly provisioned; 156 verified module blobs; actual task then unchanged cache restoration | `bazel-storage-runtime-nx3.log` |
| Initial backed Bazel runtime, release | RED; a prior action cache hit left the deliberately undeclared execution counter absent, exposing the omitted build-mode identity | `bazel-storage-runtime-release1.log` |
| Corrected backed Bazel runtime, prototype | GREEN / 12.8 s; original mode-specific 22-byte output and one execution, then actual cache restoration | `bazel-storage-runtime-nx4.log` |
| Actual Node import of activated SDK `b11ab199…` | RED; absent repository-relative observation `.mjs`, before any Binaryen body | `toolkit-frozen-import-red1.log`, `binaryen-partition-nx1.log` |
| Static import repair and normal SDK setup | GREEN; actual authorizer release / Node CLI launch | `toolkit-frozen-import-green1.log` |
| Runtime preflight neutral cases, private Nx prototype | GREEN / 2.4 s; one law / five assertions, four independent SQLite outcomes; esbuild produces the tested module bytes | `toolkit-preflight-nx1.log` |
| Production preflight and activation | GREEN; actual production Node bundle imports, seventeen required functions, unchanged hash; actual Nx v23.2.0 | `toolkit-preflight-production1.log` |

The initial raw Bun test invocation omitted its required relative-path prefix, accidentally treated the long path as a search filter, and did not execute the intended law. Its crash/search output is not a meaningful semantic RED. Corrected explicit paths and the registered private Nx routes supplied the actual law results above.

## Remaining Qualification and Scope

Corrected Bazel release runtime, Binaryen acquisition/runtime and broader strict TypeScript checks are running at this checkpoint. Full normal `repo:test-bazel-storage` remains in whole-workspace source discovery under concurrent edits; no passing result is attributed to it.

Bazel's managed cache and the probe's produced bytes are physically bounded. Arbitrary authored task commands can still write elsewhere, and general declared frontend output publication/restoration requires the parent's first-party products routing. The canonical lease namespace still needs its own finite lease provider. Old cache trees remain outside this new envelope. Actual Linux/Windows fixed filesystem enforcement remains separate platform qualification; an ordinary directory fallback is not accepted as proof.


## Bounded Lease Metadata and Original Queue Custody

Repository callers now name `.cache/b/l/m/data`, physically inside the separate 64 MiB fixed image. Every non-lease partition opener retains the original lease-image reader through its full callback. The normal repository build entry opens this backing before discovery or dependency preparation; Cargo compiler, selected Rust unit and Nx fixed-cache openers now use that same partition owner. Existing SDKs and old lease trees remain a retained baseline. The lease image itself uses one nonqueued bootstrap SQLite resource in `.cache/b/c`, with 4096-byte pages and `max_page_count=4` set before initialization on every connection. Invalid, unaligned, oversized or preexisting oversized bootstrap databases fail before writes. This terminates the opener dependency without recursive leases or ordinary directory fallback.

Actual private Nx `leases-partition` prototype and release gates were GREEN (979 ms and 926 ms critical path). Their real fixed image was 66,060,288 bytes both file and allocation, below the 67,108,864-byte envelope; the mounted filesystem measured 66,019,328 bytes. A real writer received ENOSPC after 63,963,136 bytes and removed its own probe. Two original shared image readers excluded exclusive retirement. Independent Node system SQLite refused an oversized BLOB with SQLITE_FULL, retaining an 8192-byte database and no rollback journal after cleanup. Bootstrap control allocation measured 8192 bytes, below 1 MiB. The first Node oracle attempt was RED because SQLite automatically rolled back the FULL transaction and explicit cleanup tried a second rollback; the corrected oracle admits that exact engine outcome.

The closed neutral bootstrap corpus was GREEN through Nx in both modes (1 law, 12 assertions). Strict TypeScript plus noUncheckedIndexedAccess for the bounded lease Source and runtime proof exited zero. Actual full queue gates were GREEN prototype (10.3 s) and release (11.2 s): original FIFO, cancellation, exact 256-character admission, 128-bit nonce, Node/Bun/Python independent SQLite races, and new retained-claim and killed-Node-waiter laws executed. Temporary test SQLite databases use a compact owned namespace inside the lease volume; authored worker input remains ticket generated material. Initial long ticket-path attempts correctly refused before admission.

Production queue cleanup no longer calls process.kill or uses PID/age as retirement authority. Each original arrival first holds a shared SQLite claim, then publishes its ticket. Another waiter must obtain the exact exclusive claim before removing an abandoned ticket. A held claim labeled with a nonexistent PID remained intact; SIGKILL released a real Node waiter's kernel claim and allowed the surviving waiter to proceed. Per-ticket claim databases remain within the finite lease image; this proof does not claim unbounded-duration reclamation or custody of unmanaged legacy processes.

## Completed Tooling Follow-up

Corrected Bazel runtime gates were GREEN prototype and release, with one actual execution and restored output on the second hit; the task key now binds SEMIO_BUILD_MODE. Binaryen130 preparation and actual wasm-opt version were GREEN inside tools8GiB. Frozen authorizer preflight neutral tests were GREEN both modes (1 law, 5 assertions), and normal tools setup imported the real Node bundle and checked all 17 mandatory hooks before activation. Strict tooling/Binaryen/preflight and Bazel storage TypeScript checks exited zero. Linux/Windows native providers remain unqualified; only actual macOS fixed-image enforcement is claimed here.
