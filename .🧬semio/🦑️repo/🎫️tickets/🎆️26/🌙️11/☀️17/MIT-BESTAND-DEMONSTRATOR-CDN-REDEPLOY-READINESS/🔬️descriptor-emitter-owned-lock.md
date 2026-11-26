# Descriptor Emitter Owned Lock

Affected repository paths across emitter reuse and owned-lock provenance:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🔒️resolver-lock/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`

The installed tool is Cargo `1.99.0-nightly (3efb1f477 2026-07-17)`. The supported override is the Cargo configuration key `resolver.lockfile-path`, whose value is an absolute path ending in `Cargo.lock`. Cargo accepts it from either the configuration override or `CARGO_RESOLVER_LOCKFILE_PATH`. [Cargo configuration reference](https://doc.rust-lang.org/nightly/cargo/reference/config.html#resolverlockfile-path)

```sh
repo tools exec cargo -- build --locked --offline --manifest-path "$manifest" \
  --config "resolver.lockfile-path=\"$owned_lock\""
```

The generated two-package fixture has a live lock that disagrees with the dependency's declared version and an alternate lock that agrees. Default locked metadata exited 101 against the live lock. Both override forms produced locked metadata for `lock-dep@1.0.0`. The configuration form also produced the expected two locked build units and compiled `liblock_app.rlib` from the original fixture workspace. Pointing the override at the stale lock again exited 101, and an override with the wrong filename was refused. Cargo's effective JSON configuration exposes the exact selected path for both forms. The experiment wrote only generated fixture sources and outputs in `🗑️generated/li`. Neither the legacy `--lockfile-path` flag, other configuration key spellings, nor a working-directory change selected the alternate lock.

## Compiler owner and handoff

The emitter compiler product lease selects an exclusive product lane before invoking the artifact compiler. Only the admitted descriptor-emitter producer asks that compiler to create a private lock snapshot. The compiler captures the current workspace lock after normal owned dependency preparation, verifies the copied bytes and retains the snapshot through planning, the terminal Cargo child and receipt consumption. The transient path is excluded from the reusable target recipe. Actual selected-closure movement still requires a fresh plan.

The compiler adds the private configuration selector after ordinary writer argument admission. The public validator still rejects explicit alternate lock selectors. The planner checks Cargo's effective JSON configuration against the issued path, then resolves locked metadata and units through that path. Private pinned planning skips the existing shared-root memo, whose questions name the workspace lock, and rechecks the actual copy instead of letting lock repair write it.

The provenance writer records the literal private selector in the Cargo arguments and the exact file claim for the alternate lock. It retains the bytes in the compilation inventory while continuing to check all other admitted inputs. The installed emitter checks the retained lock against the current selected closure. Disjoint live edits therefore cannot invalidate the raw invocation receipt, and a selected lock change triggers re-admission.

The authentic native Cargo fixture passed 12 assertions. Its running build script rewrites an unrelated live lock row. The real child's receipt retains the exact pre-edit private lock at the path specified in its arguments, and the captured executable runs. The fixture also verifies selected-closure refusal and private file retirement after both success and refusal. The native fixture uses the repository's short compiler scratch directory to meet the existing 256-character limit. Its transcript is `🗑️generated/owned-lock-native-verified.log`.

The freshness, prerequisite, artifact request and public Cargo ingress suites passed 26 tests and 191 assertions. They exercise freshness from the original native receipt or an alternate lock retired from disk, and refusal of both external configuration spellings. The transcript is `🗑️generated/owned-lock-freshness-ingress.log`. The unpinned planning and memo regression suites passed six tests and 87 assertions under the fixture Rust version probe. The already-running release retained the compiler configuration with which it started.

## Postpublication rechecks

Release `wZeuq82` in `l/job-describe-tool.log.2` emitted provenance, native source digests, stage, custody and consume events before the enclosing Cargo configuration requested a restart on the profile policy JSON. Attempt `.3` reached the same publisher boundary and then requested a restart on the framework workspace lock. The generic descriptor tool job waits for the child process exit. A successful publisher event alone cannot satisfy the job. Other attempts had genuine Rust compiler failures before publication. See [Descriptor Actor Input Ceiling](./🔬️descriptor-actor-input-ceiling.md).

The installed Cargo resolver reports the repository root as the descriptor emitter's selected workspace. The answer comes from actual `cargo metadata --locked --offline --no-deps` using its pinned toolchain and issuing the private resolver selection. Inferred ancestor locks served conventional invocations and incorrectly made the unrelated framework lock an invocation requirement with the private resolver selected. The compiler recipe and provenance session now infer ancestor locks only for conventional invocations. The owned resolver continues to record its actual file claim and any individually observed source dependencies. The real Cargo fixture uses an explicitly selected workspace above a nested package and an unrelated intermediate lock. The old code claimed that extra lock. The corrected build and receipt pass all 16 assertions, including running the binary, changing the unused ancestor after custody and refusing a changed selected closure. The transcripts are `🗑️generated/descriptor-root-exact-workspace.txt` and `🗑️generated/emitter-ancestor-lock-{red,green}.log`.

For the received descriptor-emitter Original, the capture now requires agreement between the already-loaded canonical semantic policy identity and the first physical observation. The immutable plan and frozen effective Cargo selectors retain that policy choice. The source records retain the raw digest and byte extent at admission. Later rechecks by this configuration view retain the same evidence for its lifetime, validate Original refinements and continue checking the selected workspace, native compiler and actual effective Cargo configuration. A mismatch at first capture still rejects that Original before invoking Cargo. A new unpinned request checks late edits and must reload to admit a new policy. Other producers preserve their existing checks.

The portable policy fixture uses real original resource controls, physical file observations, Ajv schema validation and SQLite canonical JSON as an independent identity oracle. The pre-fix run failed the late-edit retention and the mismatched initial capture (7 passed). The corrected suite passed all 9 tests and 28 assertions. It also verifies an unpinned late edit, an unpinned stale load, a freshly loaded replacement and another producer's retained live checks. See `🗑️generated/descriptor-profile-capture-{red,green}.log`.

Adjacent suites passed when run under their individual test births. The configuration-ingress suite passed 1 test and 34 assertions with the pinned Python oracle. Profile consumption passed 27 tests and 1,612 assertions. Received-Original pin routing passed 3 tests and 13 assertions. See `🗑️generated/descriptor-profile-{ingress,consumption,received}.log`.

The current public descriptor emitter pin is restricted to prototype. It independently verifies executable bytes, retained binary, index, receipt and the real compiler bin terminal. A release equivalent would also need a typed successful same-Original handoff from its producer and an immutable artifact generation. The generic job currently carries only process exit and logs. This change keeps public release freshness and performs no replay of a previously published executable.

Additional repository paths for policy capture:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/👁️profile-recheck/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/👁️profile-recheck/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/👁️profile-recheck/🧬️.json`
