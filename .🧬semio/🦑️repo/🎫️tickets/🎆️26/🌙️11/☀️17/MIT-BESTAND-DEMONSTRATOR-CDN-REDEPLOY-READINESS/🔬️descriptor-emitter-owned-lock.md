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
