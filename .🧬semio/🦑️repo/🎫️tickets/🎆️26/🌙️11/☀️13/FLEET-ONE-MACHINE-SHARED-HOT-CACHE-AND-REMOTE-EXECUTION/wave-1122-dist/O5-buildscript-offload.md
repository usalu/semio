# O5: Build-script RUN offload

Paths: UC = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache`, BS = `UC/🏗️buildscript`.

## Changed
- BS/☁️offload/🟦️.ts (new) + 🧬️schema/🔣️.json (closed policy: layout, env, source/binary markers, build-dependency deny list, stand-in tool): `offloadBuildScriptRunV1`.
  1. Hermetic predicate before any byte leaves: mode never -> local; cwd must be the manifest dir; build.rs (Cargo.toml `build` key honoured) scanned for spawned processes, network, host probing (pkg_config, cc, cmake, bindgen, ...), literal paths outside the package (absolute, ~, ..), undeclared or bulk env reads; Cargo.toml build-dependencies deny list; the compiled binary scanned for PKG_CONFIG/CMAKE/RUSTC/CARGO_ENCODED_RUSTFLAGS/TcpStream...; env `DEP_*`/`CARGO_MANIFEST_LINKS` refuse; unsafe values dropped unless the script names them. Refusal = `declineOffloadV1(not-portable, "build-script: <reason>")` and local run.
  2. Shipping: manifest dir by a bounded sequential walk (target/.git/node_modules skipped; symlink, odd names, > limits refuse) at `in/manifest/..`, real binary at `in/run`, tiny shell stand-in as the shipped `build-script` tool (records worker root, mkdir OUT_DIR + marker, cd manifest, exec); outputs: directory `out/out-dir` -> real OUT_DIR, file `out/root`.
  3. Translation: worker `<root>/out/out-dir` and `<root>/in/manifest` (both spellings of the root) rewritten to the real OUT_DIR / manifest in stdout and stderr.
  4. Path-dependence guard: K2's `leakedRootKindsV1` over the landed OUT_DIR files plus any remaining worker root in the streams -> landed entries removed, local with `path-dependent-output`.
- BS/🟦️.ts: after the fleet decision the miss path calls `runOffloadedV1`: offload (holds only `acquireLightSlotV1` when SEMIO_UNIT_CACHE_LIGHT_SLOTS>0, SIGINT/SIGTERM abort -> status 130) and, on any local answer, the real run under a real `acquireSlotV1` (SEMIO_UNIT_CACHE_SLOTS) as the wrapper does for local rustc units (coordinator requirement, O7). Same cancellation/timeouts as every command offload; mode `never` skips; published and shared exactly like a local run.

## Tests
- BS/☁️offload/🧪️tests/🟦️.ts (+ language-agnostic 🔣️.json). Verified (scratch runner with a bun:test shim, since `repo test-file` is broken host-wide): schema 1/1, fixtures 1/1, bounded walk 1/1, outside-path stays local with recorded reason 1/1, offload off / DEP_ env stays local 1/1.
- Written but NOT verified green: tiny script remote byte-identical to local (OUT_DIR tree, stdout, stderr), leak withdrawn, wrapper end to end, -j (3 concurrent runs, SEMIO_UNIT_CACHE_SLOTS=1) serialises, offloaded run holds a light slot. Scratch cluster reached placement (after the worker advertised command-format properties) but the content store timed out under host load 45-70 with taskpolicy -b; the wrapper test then hung.
- `repo test-file` fails on every file with "Explicit finite owned Bun file selection required" (test-file of testing/🟨️bun/📁️files own test also fails); not caused by this lane.

## Open
- Re-run the 7 live tests once the host is quiet (`repo test-file` once fixed).
- Worker policy has no cwd option: the stand-in does `cd in/manifest`. Another lane overwrote my schema file at 22:24 with a different policy (regenerated mine at 23:03; check it stays).
