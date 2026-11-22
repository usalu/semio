# WGPU native scope test and Cargo crate search

## Failed shared invocation

The shared invocation of `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-renderer-wgpu-shell-plugins --lib -- native_scope_delivers_only_admitted_workpiece_contributions` exited 101. The failing compiler was `addr2line 0.26.1`, which reported E0460 while loading `libgimli-fff57f7217e30a1d.rmeta`. Its diagnostic listed only the two sysroot files for `hashbrown-fc967dc9b21ca6b7`. The captured invocation was non-verbose. Its stdout, stderr and supervisor receipt lack the actual rustc arguments, including the `-L` flags. The shared lane directory had already been removed when checked.

Read-only observations from shared lane `p6135b7`

| Unit | Event | Evidence |
| --- | --- | --- |
| `gimli` `f786922196…` | Local compiler miss and publication | The published basename exactly matches the failing diagnostic. The rustc key class was `--emit=dep-info,metadata,link`, and neither its event nor the other `gimli` miss recorded offload. |
| `hashbrown` `1717060388…` | Local hit | The stored `libhashbrown-115f633071e2d1af.rmeta.semio-deps` records SVH `de0a7a8908fde68700ed4d9139cf8e81`, which matches the failed consumer's `gimli` metadata entry. |
| `hashbrown` `5a7263035c…` | Local compiler miss and publication | The output was `libhashbrown-9726ab59f44e199e.rmeta`. The `gimli` metadata also expects this crate, with SVH `ea393196ce9087f08e665625438ef728`. The corresponding hashbrown sidecar was absent from the inspected CAS roots at read time. |
| `addr2line` `044d5660cf…` | Local compiler failure | Event immediately followed a fleet breaker skip. No successful cache lookup or output publication was recorded. |

The published `gimli` sidecar also lists sysroot `hashbrown-fc967dc9b21ca6b7`, so an error that reports only this candidate does not establish a sysroot mismatch. Both workspace instances were recorded as compiler dependencies. The evidence leaves the missing materialization and absent Cargo search argument cases open. Earlier worker reports independently record the same `addr2line` error and the analogous `gimli` failure while loading `indexmap`.

## Search and materialization paths

- `wrapRustcV1` preserves Cargo's original arguments and appends `linkRootArgumentsV1` and compiler temporary options. The link options are empty on macOS for `--crate-type lib`. The link helper adds only Mach-O object-root and proc-macro install-name options where applicable.
- Cargo's enabled per-unit build layout supplies `-L dependency=<build>/<profile>/build/<package>/<id>/out` paths for transitive libraries. A previous repo capture contains this spelling, including several different `hashbrown` versions. It belongs to a different invocation and cannot substitute for the failed compiler arguments. [Cargo build-directory layout tracking issue](https://github.com/rust-lang/cargo/issues/15010)
- The wrapper reads `-L dependency=` arguments and the parent directories of explicit `--extern` paths to locate current metadata. A cache hit stages and atomically renames its own outputs inside the current unit's output directory, then validates promised/announced paths and replays the compiler messages. It does not create transitive dependency paths or symlinks. Metadata-only publication records compiler-announced files and emits the dependency hash sidecar. The metadata-only identity tier cannot have served the failing `gimli` output.

## Bounded validator probe and contract

A `repo tools exec bun -- -e` probe called the real functions with a synthetic `addr2line` command, `--crate-type lib`, `--emit=dep-info,metadata,link`, one dependency directory, and a direct `--extern gimli` in another directory. The fake `gimli` artifact had a schema v1 `.semio-deps` sidecar listing `hashbrown-115f633071e2d1af` with the recorded `de0a7a…` SVH. The digest stub returned 64 ones for present files and null for missing ones. No compiler program was used and the scratch directory stayed under the ticket's generated folder.

| Input condition | Code restore closure | Metadata rebase |
| --- | --- | --- |
| Recorded workspace `hashbrown` artifact missing from both input directories | Accepted | Accepted |
| Artifact present with stale SVH `a99338b1ed8e53e6bb0a766ea35dedb7` | Rejected | Unmeasured |
| Artifact present with expected SVH | Accepted | Unmeasured |

The two validation paths currently skip any recorded crate ID absent from their directory scan. This deliberately keeps sysroot and otherwise undeclared metadata outside validation. A narrow additional check could reject a missing recorded crate only when Cargo explicitly supplies its exact per-unit output directory, including its 16-character crate suffix. An existing compatible library form must still be honored, and a same-named sysroot crate with a different suffix must never satisfy that promise. An ordinary directory containing no exact managed unit evidence is insufficient to infer absence. This conservative contract adds no filesystem or cache mutation and keeps local/fleet manifests unchanged. It would not diagnose a missing directory argument in the original failure.

The independent isolated WGPU invocation was still running at the time of this read. The observations above preceded the cache validator change.

## Cache validator change and verification

Cargo's compiler front end adds per-unit dependency directories for the transitive units the compiler needs. The output directory's hash is the same `unit_id` used for the emitted library filename suffix when both are present. This justifies matching the complete crate suffix to an explicitly declared Cargo output path. [Cargo compiler search path construction](https://doc.rust-lang.org/stable/nightly-rustc/src/cargo/compiler/mod.rs.html#1755) [Cargo unit metadata](https://doc.rust-lang.org/stable/nightly-rustc/src/cargo/compiler/build_runner/compilation_files.rs.html#100)

The cache validator now refuses both code restore and metadata rebase when a recorded crate has a matching declared Cargo unit directory and no available metadata file, dynamic library, or archive that could embed metadata. The rules only derive managed membership from the exact `build/<package>/<16 hex>/out` shape under the supplied directories. Existing metadata hash comparison and rebase rules still apply when metadata is present. The same check applies after cache lookup regardless of where the entry was obtained.

The language-agnostic fixture tests empty declared output, a same-named sysroot artifact with a different suffix, an ordinary directory outside Cargo's layout, a different Cargo unit ID, refreshed matching metadata, a stale generation, and an archive that may embed metadata. The Python oracle and TypeScript both interpret the fixture. An archive by itself is treated conservatively as a possible metadata source, so this change does not assert that any given archive actually contains metadata.

Focused pre-change test failed because both restore paths accepted the missing declared library. After the change the interface suite passed all 30 tests and 403 assertions, including the existing real-compiler closure laws. `git diff --check` passed. Scratch was inside `🗑️generated`.

```text
TMPDIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️15/DEV-PROCESS-REACT-E2E/🗑️generated" repo tools exec bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔭️interface/🧪️tests/🟦️.ts'
```

This check hardens an independently demonstrated cache restore case. The original WGPU failure remains unresolved because its recorded diagnostic and surviving receipts do not give the failed compiler's dependency arguments or files. The isolated test was still live at the latest check when the patch was being verified. No shared cache mutation or dedicated WGPU rebuild was started by this investigation.
