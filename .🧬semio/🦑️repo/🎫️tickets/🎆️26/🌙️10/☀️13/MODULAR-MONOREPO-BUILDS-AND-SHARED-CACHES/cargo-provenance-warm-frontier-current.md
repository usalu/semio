# Cargo Provenance Warm Frontier — Read-Only Audit

2026-10-15. No builds, Source edits, lock work or timing experiments. Root supplied observed target durations of 38–90 seconds for tiny borrowed/projection graphs and 69–137 seconds for owned ArtifactRef targets, plus a progress row near 222 MB / 24,448 work. These establish end-to-end latency and a cumulative physical observation amount; they do not attribute latency to hashing, registry, compiler, locks or buffered logs. Protected Nx pair 86516 was not disturbed.

## Actual Call Path and Authority

Repository entry `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:35` composes `withRepositoryCompilerStoreV1` → selected workspace preparation (owned subprocess) → `withRepositoryCargoInputsV1` → neutral `buildCargoArtifacts`. The dependencies owner at `…/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:37–40` holds bounded registry writer custody during fetch and then reader custody through consumer settlement. This is real dependency preparation, distinct from post-build provenance hashing.

Neutral implementation: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`. `buildCargoArtifacts` captures the compiler unit graph at line 245 before the artifact execution. Target recipe collection at 221–224 uses its own `CurrentPhysicalOwnerV1`. In `captureCargoArtifacts`'s finally block (886–888), a **new** physical owner is created and passed to `writeCompletedCargoInvocationProvenanceV1`, which creates a one-invocation session (742–749), writes and closes it. Closing performs a full current-byte recheck before publishing receipts.

Physical owner: `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts`. Streamed reader: adjacent parent `…/🧾️observation/🟦️.ts`. Every body is streamed through SHA-256, bounded chunks (at most 1 MiB), cancellation/deadline/work checks, descriptor/path identity checks and final close; no body is trusted from a stat-only observation. Current owner claims compare dev/inode/size/mtimeNs/ctimeNs; final recheck rehashes bodies as well as checking identity. Missing paths and directory rosters have explicit structural rechecks.

Provenance control lives at `…/🏗️native-build/🧾️receipt/🟦️.ts:18–35`: 16 GiB byte budget, 4,194,304 work, 1 MiB chunks, 600-second maximum deadline, intersected with outer authority; scaled for multi-component passes. Progress filter is one row per second; it does not bypass physical control checks.

## What the Logged Bytes and Work Include

The `cargo-provenance` prefix is emitted only by the new post-build owner at 886. It is **not** an active Rust Source size report. Its cumulative budget counters include:

- Every Cargo compiler-artifact filename (rlib/rmeta/executable/proc-macro outputs), even fresh units announced by Cargo; initial parallel hashing at writer line 525.
- Candidate dep-info file text, actual source files named by dep-info, and producer receipt text for received evidence. This includes Rust `include!`/`include_bytes!` inputs when named by the genuine compiler, and generated/resource observation files. It is not a recursive scan of every file in each package.
- Staged deliverable copies, in addition to original compiler outputs, where `stagedPaths` names them (artifact rows around 626).
- Build resource logs `semio-runtime-resource-inputs.jsonl` and witnessed read/copy/directory resources; compiler `semio-compiler-resources/.../observation.json` and its witnessed resources. Preserve these rather than reducing the world to `.rs` files.
- Selected package manifests plus ancestor `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `.cargo/config`, `rust-toolchain.toml` and `rust-toolchain`; Cargo-home config files. Missing ancestors are explicit observations.
- Source-owner ancestry probes, candidate output discovery directories and their canonical rosters; retained path/claim/roster bookkeeping is also charged as bytes.
- The final current-byte rehash of all claimed files. This means body bytes normally count initial read **and** final recheck, while work also counts filesystem/control/roster sorting operations. `totalBytes` in this owner is emitted equal to current cumulative bytes, not an advance prediction.

No code in this receipt writer recursively hashes the full Cargo registry, entire toolchain, or all unselected package source. Registry dependency Sources appear if actual dep-info names them; binaries and manifests/configuration also appear. Cargo/rustc identity capture, compiler admission Source collection, preparation and generic graph input capture are separate operations and cannot be assigned to the 222 MB row without a phase trace. Large output artifacts are a plausible contributor even for small source graphs; the observed row alone does not give a per-category byte breakdown.

Generic Source capture is separate: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`. `captureObservedSourceInputsV1` at 139 collects actual observed facts and optionally module-load authority. Its `fileStamp` at 156–160 hashes actual bytes; project/async memoization validates retained observations and binds implementation authority digest. This is not the provenance file custody and its caches cannot be treated as a free current physical claim.

## Existing Reuse and Smallest Sound Warm Optimization

A genuine reusable mechanism already exists: `sharedProvenanceCustodyV1` at neutral file 334–336 supplies per-invocation owners sharing `CurrentPhysicalFilesV1`; `CargoProvenanceSessionV1` interns digest/length/text and pending reads. The real component-pass path `…/🏗️native-build/🧩️component-pass/🟦️.ts:386` uses it for the whole pass. Shared claims hash each physical file once, and session close rehashes once; each invocation independently settles missing-path and roster observations. The ordinary `buildRepositoryCargoArtifacts` route still creates a new one-invocation post-build session. Different subprocesses/agents do not share the JavaScript objects.

**Smallest candidate:** expose the existing session as an explicitly held owner capability to a bounded group of repeated native invocations within the same compiler-store reader lifetime, rather than constructing a fresh provenance session per invocation. Keep final receipt publication after one shared close. This directly reuses existing tested custody design and removes repeated initial/final reads of overlapping unchanged files within that group. It is an intra-lifetime optimization, not a persistent warm-stat cache and not a promise to accelerate an isolated one-invocation build. Cross-process/agent sharing requires a real owner service/held reader lease and settled publication protocol; a global Map, receipt digest or path stamp cannot grant that authority.

Additional local opportunity: overlap `session.prefetch` (already implemented at line 487) with actual Cargo artifact announcement, as the component pass does, while retaining final close. This may hide latency but does not reduce required byte reads; measure before selecting it.

Do not reuse file claims across the compiler writing their outputs without detecting the identity transition. A changed crate must invalidate its claim even if bytes later return (ABA): current inode/mtime/ctime checks plus final body rehash remain required. Preserve dep-info environment/checksum content, actual `include!`/generated inputs, resource witnesses, missing path observations and per-invocation directory rechecks. If sharing an owner across agent requests, cancellation/deadline authority and reader lifetime must remain valid until all admitted consumers settle; one agent ending must not prematurely close another consumer's owner.

## Evidence Needed Before a Performance Claim

Add bounded phase durations and `observedReads()` category totals under the existing ticket-owned qualification path: preparation/fetch, planning/admission, cargo execution, receipt initial bodies, final recheck, staging/publication, plus time waiting on owners. The existing physical custody exposes exact per-file read/byte totals. Compare equal actual native target selections under cold and held-session conditions; assert each shared body is first-read once and final-rechecked once, mutation/ABA/absence refuse, and cancellation leaves no published partial receipt. Existing receipt tests exercise shared custody at `…/🧾️receipt/🧪️tests/🟦️.ts:375,404`; they were read, not run here.

The actual timing supplied by Root supports investigating this phase. Source inspection supports the repeated-session hypothesis. Neither proves that provenance dominates the wall clock or that an optimization will achieve a particular duration.
