# Proc-Macro Resource Helper Independent Review

Read-only source checkpoint on 2026-10-08. No compiler was launched by this audit.

The helper owner is `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs`; the consumer is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts`, function `runtimeCompilerResourceInputsV1`.

The helper retains successful original read bytes in content-addressed snapshots, original paths, producer callsite, complete directory entries including raw symlink targets, and an immutable completed observation. Original, snapshot, and observation paths are registered through the caller tracking function. `read_to_string` records successful bytes before UTF-8 conversion, preserving invalid UTF-8 semantics. Directory iteration and metadata failures mark the capture incomplete.

A concrete initial gap was reported directly to Runtime: immediate `read`/`read_dir` errors returned before marking the capture incomplete. A caller that handles such an error could otherwise complete without recording a path whose later appearance changes behavior. The fresh read now confirms both error branches track the attempted original path and set `capture.complete = false` before returning the original I/O error. This is a source repair observation, not an executed regression verdict.

The consumer refuses incomplete captures; binds exact retained producer/caller Cargo messages in the same observation; requires production profile and proc-macro producer versus ordinary caller; verifies physical manifest/source membership, tracked observation SHA, actual rustc-consumed input checksums, retained roster equality, original current digest, snapshot equality, and exact directory enumeration. Fixture-owned original paths produce an explicit runtime fixture edge. Missing or stale proof remains a finding.

The collector independently binds caller crate to the actual target name. The consumer currently repeats manifest/source and message binding but does not itself compare captured `caller.crate` with the selected target name. This does not invalidate the reviewed collector-produced observation, but adding the same exact comparison would make the public helper independently enforce its caller identity invariant.

This review covers the explicit wrapper and retained-resource consumer only. It does not prove uninstrumented filesystem operations, arbitrary macro behavior, real guest currentness, publication, or a complete runtime exclusion verdict. The owner-reported neutral native helper result (79 passing with five expected parent callsite RED failures) remains a separately qualified intermediate receipt.
