# Historical Compiler Provenance Independent Review

Read-only inspection, 2026-10-15. No SQLite opens, bundle execution, Originals, gates, Native operations, mutation or deletion. Read retained files directly without issuing image/SDK reader custody; therefore these are observed candidate evidence and stored-hash consistency, **not** a live authenticated retirement grant. Exact findings sent to Nx.

## Actual old layout bytes exist in retained compiler code

Observed `.cache/b/t/m/rc/6013cbc0b38262821df16abaedaefa0dc6514fd70fd09bac552080fd2cbad94c/entry.mjs` is 136463 bytes and SHA256 `1df6e2670be27a00709d3ba0f9f34df99891258a4034e622a1834e99e8b34c5b`, matching its receipt's programDigest. The directory identity recomputes from receipt sourceDigest/programDigest with the current sealed-code identity framing.

At byte offset approximately 89634, the actual compiled function first recovers the compiler mode reservation, then computes `join(image.root,"cargo",mode)`, followed by `join(base,"build")`, `captures`, `tmp` and shared `cargo/home`. The return sets CARGO_BUILD_BUILD_DIR to that build path and CARGO_TARGET_DIR to `join(base,"target")`. Thus actual retained compiled bytes, not just a Source path row, establish the old `cargo/prototype/build` and `cargo/release/build` layout behavior. Several earlier retained rc generations contain the same layout; newer d8dac7… contains compact b/p,b/r policy instead.

This rc namespace is the independently sealed Rust compiler code owner, not the installed SDK authorizer bundle. Its current publication/authentication owner is library `⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts` 24–35. A later retirement proof must retain the real tools image and exact rust-code reader and validate receipt/program/source identity under that owner; direct disk reads here do not substitute.

## Actual charge records agree, but do not name layout or guards

Observed Rust root mode receipts contain:

- prototype allocation `9198ef11dead054182778c3cad78a2c8`, owner `repo:compiler:prototype`,4294967296 bytes.
- release allocation `3544182cd46f67a136fa96b73127f0c4`, owner `repo:compiler:release`,4294967296 bytes.

The textual `storage-events.jsonl` contains matching reserved compiler-workspace rows and subsequent committed events for both. No replayed SQLite or runtime reservation recovery was attempted. Compiler receipt schema binds version/mode/allocation/owner/bytes only; it has no exact build-root, compiler code digest, Cargo guard protocol or physical inode fields. These records support charged mode ownership but cannot alone prove every old subtree uses the current locking protocol.

## Precise missing provenance: old Cargo guard protocol

Inspected 22 retained managed SDK authorizer bundles for old layout/guard text and their Source manifests for compiler owner/lease rows. The manifests contained no compiler-store or native-build Cargo lease owner rows in this inspected set. Two actual bundle hashes were independently compared with their stored output receipts:

- SDK `0c3a89…` authorizer SHA `cef1d032f6b285e80ef7d25859825779033c065f733fde9cd98921189311b65a`.
- SDK `8d0ff6…` authorizer SHA `f76255051b34f91d61bb52986d9622636e2320e457391646920b9f88a148d566`.

Their source-file bytes also match stored sourceSha256, but those bundles do not contain the old compiler layout or matching Cargo store/physical lock implementation. The rc bundles contain the layout/reservation owner but no Cargo store/profile protocol or `.cargo-build-lock` handling; direct Rust unit compiler closure does not automatically include Cargo launcher locking.

Current actual Cargo lease Source is `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts` 26–35 canonical directory/profile identity,44–64 store/profile acquisition and73–75 store resource. Current maintenance also requires actual Cargo physical locks and complete descriptors. Merely observing this current Source does not prove the historical old writers used it. A short read-only git history query found revisions for that path, but no retained historical Source version was SHA-matched to an authenticated historical producer closure during this audit; no such provenance is claimed.

## Decision boundary

There is useful exact old layout evidence in retained rc compiled bytes and consistent existing compiler charges. The remaining blocker is an authenticated old Cargo producer/launcher bundle (or genuinely SHA-matching retained historical Source tied to its real owner) showing the same exact store/profile lock identity and physical Cargo exclusion protocol for those roots. Source manifest path+SHA alone is not the missing bytes, and a layout literal does not establish reader exclusion.

If that evidence is obtained under actual held custody, the current generic exact-unit retirement API may use it solely to authenticate old-data roots, while retaining current global writer, matching original store/physical locks, complete descriptor checks and recensus. No old execution adapter is needed. Until then preserve old roots as unqualified, retain the 4 GiB initial guard and report unmet physical capacity rather than converting root size into deletion permission. Nx old graphs remain outside this evidence entirely.
