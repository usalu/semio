# Procedural cargo "couldn't find file" and "Mutations source authority" audit (2026-11-22)

Read-only audit (lane 2 of `📓️handoff-2026-11-22.md`). No repository file was edited, no git command ran, and no cargo or `repo` build ran.

## Summary

- The local registry is healthy. The three crates needed by the procedural workspace are fully extracted and match their `.crate` archives file for file. There is no `.repair-` or `.stale-` leftover and no missing `.cargo-ok`. The registry race in `📓️cargo-registry-extraction-race.md` is not the current cause.
- The "couldn't find file" errors are the remote-offload bug already described in `📓️activation-build-blockers-2026-11-22.md` section 9. The `cfg_attr(..., path = "...")` module scan fix (`🗺️paths/🟦️.ts`) landed at 11:20 UTC. Every diagnostic-size offload failure for mio, errno, memmap2 and socket2 is at or before 10:24 UTC. Every later remote return for these crates is artifact-sized.
- "Mutations source authority ... Operation not permitted (os error 1)" is raised by `dsl::Mutations` in `🔖️mutation-source-authority/🦀️.rs`. The EPERM comes from `fs::symlink_metadata` (lstat) on a path component. No local log contains that string, so its origin is not confirmed. The most likely cause is a denied file-metadata read (macOS TCC or sandbox) in the process that expands the derive, which is plausibly a remote worker. Repo-side mutable flags are ruled out.
- The handoff's lane 2 is probably stale for the "couldn't find file" half. Re-verify with one non-offloaded cargo run of the procedural workspace before treating it as fixed.

## 1. Where repo cargo keeps the registry

- `.cargo/config.toml` (repo root) sets `rustc-wrapper = "repo"` and a `rustc` ingress script. It does not set `CARGO_HOME`. `CARGO_HOME` comes from the repo launcher and points at the per-image home (`.cache/b/r/m/cargo/home`, as the cargo-registry note states).
- Mounts (`mount`, `df -h`): `.cache/b/r/m` is `/dev/disk15s1`, 96 GiB, 16 GiB used, 32 GiB available, mounted `noowners`. The `.cache/b/r/m/cargo/home` directory is ordinary APFS data on that mount. The mount is not read-only.
- Registry index: `.cache/b/r/m/cargo/home/registry/{src,cache,index}/index.crates.io-1949cf8c6b5b557f`.

## 2. Locked versions

| Crate | Lock file | Version |
|---|---|---|
| mio | root `Cargo.lock` | 1.2.4 |
| mio | `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.lock` | 1.2.1 |
| mio | `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.lock` | 1.2.3 |
| mio | `…/🗿️artifacts/🧊️generation3d/Cargo.lock`, `…/🌀️generation2d/Cargo.lock` | 1.2.4 |
| errno | all of the above | 0.3.14 |
| memmap2 | all of the above | 0.9.11 |

Earlier-written lock files under `.cache/f/lost/…` are stale snapshots and were not used.

## 3. Extracted registry sources vs archives

Command: for each of errno-0.3.14, memmap2-0.9.11, mio-1.2.1, mio-1.2.3 and mio-1.2.4, list `tar tzf` file entries in `cache/…/<crate>.crate`, then compare with `find . -type f ! -name .cargo-ok` under `src/…/<crate>`.

Output:
```
== errno-0.3.14     archive 18, extracted 18, .cargo-ok {"v":1}, file list IDENTICAL
== memmap2-0.9.11   archive 16, extracted 16, .cargo-ok {"v":1}, file list IDENTICAL
== mio-1.2.1        archive 69, extracted 69, .cargo-ok {"v":1}, file list IDENTICAL
== mio-1.2.3        archive 71, extracted 71, .cargo-ok {"v":1}, file list IDENTICAL
== mio-1.2.4        archive 71, extracted 71, .cargo-ok {"v":1}, file list IDENTICAL
== leftovers: none (.repair-*, .stale-*, *.tmp*)
```

The archives exist for all five. The directory `src/…/mio-1.2.1/src/sys/unix/selector/kqueue.rs` is present (it is in the archive list), so the file the error names does exist now.

## 4. Where "couldn't find file" comes from

Evidence from `.cache/f/ready/workspace.log` (line 53752 and 53760). The log's mtime is 07:43 UTC, so it was last written before the fix:
```
error: couldn't find file `/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home/registry/src/index.crates.io-1949cf8c6b5b557f/mio-1.2.1/src/sys/unix/selector/kqueue.rs`
  --> index.crates.io-1949cf8c6b5b557f/mio-1.2.1/src/sys/unix/mod.rs:56:5
```
The `-->` line is a remapped path (`--remap-path-prefix`), while the header is an absolute requester path. That pattern is what a remotely compiled unit prints after its paths are translated back.

The mod declaration in `mio-1.2.1/src/sys/unix/mod.rs` is selected by `#[cfg_attr(any(...), path = "selector/kqueue.rs")] mod selector;`. The offload scanner `reachedFilesV1` (`🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts`) ships only files named by `#[path]`. The `INCLUDES` regex on line 23 now also matches `cfg_attr(..., path = "...")`. Its file mtime is 11:20 UTC.

Offload records, `.cache/b/r/m/u/x/offload.jsonl`, parsed with `python3 -I`. Diagnostic-size means `bytesReturned < 2000`. Artifact-size means at least 2000 bytes:

```
small (diagnostic-size) returns: 32  first 11-22 05:20:22  last 11-22 10:24:57  after fix: 0
large (artifact-size) returns:   10  first 11-22 11:29:52  last 11-22 13:43:12  after fix: 10
```

Examples: mio 1084 and 1352 bytes; memmap2 1024 bytes; errno 1269 bytes (the diagnostic). Post-fix: mio 431127 bytes and memmap2 96913 bytes (real rlibs).

The unit-cache log (`pca4555.jsonl` and the other `u/x/*.jsonl`) has 30 `failed` records after 11:20 UTC. They are for local workspace crates (`semio_*`, `wgpu`), and none mention mio, errno, memmap2 or socket2 or the EPERM text. The records carry no reason field, so these are ordinary compile failures of work in progress, not this bug.

Conclusion: the code-less "couldn't find file" was a worker-side missing-input problem. It was fixed at 11:20 UTC. There is no post-fix evidence of it in the logs. The handoff is out of date on this half.

Ruled out:
- Registry extraction race: the tree is complete and the markers are valid.
- Per-lane CARGO_HOME: there is one home (`.cache/b/r/m/cargo/home`), and it is present and complete.
- Rust image remount: the home is intact after the remount.

## 5. "Mutations source authority ... Operation not permitted"

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` line 74:
`Err(error) => return super::source_authority::mutation_authority_refusal(&input, format!("Mutations source authority failed: {error}"))`

The only grep match for the string in `🧰️framework/🛍️products/🦑️repo` was none. The message is in the `💻️os` derive crate, not the repo product. `expand_derive_mutations` calls `mutation_aggregate_source_authority(&source, &compiler_cwd)` (`🔖️mutation-source-authority/🦀️.rs` line 111). That calls `mutation_authority_common`, then `mutation_authority_no_follow` (lines 267-282) and `mutation_authority_raw_no_follow` (lines 215-232). Each runs `fs::symlink_metadata(...).map_err(|error| error.to_string())` on every path component from the workspace root to the source file and to the mutation directory. A `format!("{error}")` of an io error gives `Operation not permitted (os error 1)`.

So EPERM comes from an lstat that the kernel refused. Conditions that produce it on macOS:
- The process has no Files-and-Folders (TCC) grant for `~/Documents`. The repo lives under `~/Documents/semio`. A launchd agent, a remote worker, or any sandboxed process without that grant gets EPERM on stat of those paths. This is the most likely case.
- A Seatbelt or agent sandbox denying `file-read-metadata` on the path.
- Not the file flags: `ls -lO` on the derive source and the procedural workspace shows no `uchg`, `uappnd` or `restricted` flag. Flags such as `uchg` do not make lstat fail anyway.

Why it appears only sometimes: the handoff notes say it occurred once. The activation note says it is a replayed warning from another machine (a `/var/folders/...` path that does not exist here). Both are consistent with a derive expanded on a remote worker. On the worker the source path is a virtual path, and the offload code does not ship the taxonomy markers (`package.json`, `📋️project.json`, the `🔣️mutation-authority.json` and the mutation collection directory) that `mutation_aggregate_source_authority` walks. A missing component would normally give ENOENT, not EPERM, so this part is unconfirmed.

I could not reproduce the EPERM from this shell. Stat calls on the same paths succeed here.

## Most likely root cause

1. "couldn't find file" (mio, errno, memmap2, socket2): the offload scanner did not ship `cfg_attr(path = ...)` module files. Fixed at 11:20 UTC. Current status: fixed and unverified in this session.
2. "Mutations source authority ... Operation not permitted": the `dsl::Mutations` expansion ran where the source tree is not readable as the requester sees it. Most likely a TCC or sandbox denial of lstat on a path under `~/Documents`, inside a remote worker's process. The derive also has no offload guard: proc-macros that read source authority are shipped like ordinary crates. Not yet fixed.

## Proposed fix (for an execution agent)

A. Verify the offload fix (no code change):
   - Run `repo test` for `🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts`. Confirm the `cfg_attr` test passes, and confirm it fails on the pre-11:20 code if you want a mutation check.
   - Run one non-offloaded procedural cargo build (mio, errno, memmap2, socket2 units). Use the local-only switch in `🚀️offload/📜️action/🟦️.ts` if there is one, or disable the offload lane for this run. Confirm no `couldn't find file`.
   - Record the result in the ticket, and update lane 2 of the handoff.

B. Make the EPERM diagnosable and stop it from being a bare io string:
   - File: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔖️mutation-source-authority/🦀️.rs`.
   - Change `mutation_authority_no_follow` (lines 267-282) and `mutation_authority_raw_no_follow` (lines 215-232) so each `symlink_metadata` error names the failing path component and its `ErrorKind`, for example `format!("{} ({:?}): {error}", current.display(), error.kind())`.
   - Add a test in `🧪️tests/🔬️mutation-aggregate-source-authority/🦀️.rs` that makes a component unreadable (chmod 000 on a parent directory, or a non-existent component) and checks the error names that component.

C. Ship the authority inputs with a remote derive, or decline the offload:
   - File: `🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts`.
   - Either ship the taxonomy markers and mutation collection directories used by `mutation_aggregate_source_authority` (package.json, 📋️project.json, 🔣️mutation-authority.json, the collection directory), or decline offload for any unit whose dependency is `semio-os-kernel-dsl-derive` with `unmapped-path`, which keeps the derive local.
   - Test: add a case to `🚀️offload/🧪️tests/🟦️.ts` for a crate that uses `#[derive(Mutations)]`. It must either ship the markers or be declined.

D. Housekeeping:
   - Update `📓️handoff-2026-11-22.md` lane 2: the "couldn't find file" half is fixed at 11:20 UTC and pending verification (A). The EPERM half remains open until B and C land.
   - Remove the `[DEBUG]` probes if any are added. No migration scripts.

Do not change `.cache/b/r/m/cargo/home`. It is healthy. Do not add a registry repair or lock; the current extraction code (`🧾️fetched/🧱️install/🟦️.ts`) already handles the race described in `📓️cargo-registry-extraction-race.md`.

## Files read

- `.cargo/config.toml`
- `📓️handoff-2026-11-22.md`, `📓️cargo-registry-extraction-race.md`, `rust-volume-zero-avail-recovery.md`, `📓️activation-build-blockers-2026-11-22.md`, `📓️rust-law-verification-2026-11-22.md`
- `🔖️mutation-source-authority/🦀️.rs`, `✨️derive/🦀️.rs`
- `🚀️offload/🗺️paths/🟦️.ts` (grep only)
- `.cache/f/ready/workspace.log` (grep only), `.cache/b/r/m/u/x/offload.jsonl` (parsed)
