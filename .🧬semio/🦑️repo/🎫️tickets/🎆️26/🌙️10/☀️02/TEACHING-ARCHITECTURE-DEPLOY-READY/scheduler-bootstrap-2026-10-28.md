# Scheduler Bootstrap — 2026-10-28

## Result

Implemented scheduler dependency acquisition through the existing repository Cargo input contract. The real focused Cargo ingress now passes offline metadata/unit graph capture and source admission, then invokes rustc. Native executable completion remains blocked by this host's missing `cc` linker; no scheduler executable or course backend success is claimed.

## Ownership and Lifetime

`prepareRepositoryCargoSchedulerV1` acquires the scheduler's exact locked resolve with `withRepositoryCargoInputsV1` before compiler capacity. It uses the Original network grant unchanged, so offline callers remain offline. Existing lock refresh, configuration custody, finite capture, storage rechecks, cancellation/deadline and registry writer/read handling remain owned by that API.

Both repository native-helper setup and the Cargo launcher use this preparation. Launcher preparation precedes its product input scope, avoiding a nested registry-reader-to-writer upgrade and preserving the shared helper's actual store/environment independently of isolated product lanes. The generic component driver uses its existing domain-neutral compiler-input port in a separate one-unit CPU phase before main build leases/capacity. Planning remains offline.

The admitted arguments and environment flow into both scheduler graph capture and compiler execution. Repository scheduler builds use the actual issued prototype/release mode; the repository profile contract correctly rejects a hardcoded release selection inside a prototype Original. The standalone scheduler manifest now opts into Cargo's `trim-paths` feature required by those issued profiles.

Receipt reuse happens after input/configuration admission and before writer/CPU acquisition. Configuration digests participate in scheduler identity, so changing the admitted profile/configuration invalidates a receipt; executable paths/stamps remain captured from actual Cargo artifacts. Receipt lookup with the raw unadmitted store environment was deliberately avoided because configuration admission adds effective compiler flags.

## Verification

- Full dependency-consumption suite: **6 passed, 0 failed, 161 assertions** through `repo tools exec --light bun -- test ./<consumption-file>`.
- Focused admission selection and scheduler identity suites: **6 passed, 0 failed, 20 assertions** through the same registry ingress with explicit operands and a test-name filter.
- Fixtures cover online/offline cold preparation, receipt reuse, fetch rejection and compiler rejection. Tests execute the actual extracted preparation function against modeled owned ports, assert lifetime/control/argument/environment propagation, and independently verify event ordering with SQLite. AJV validates the language-neutral vectors; smol-toml validates the scheduler feature declaration.
- Actual profile rendering/admission accepts both scheduler modes and rejects the opposite mode.
- Supporting extracted-caller test initially failed because its mocked ports lacked the current compiler lane/planning/source APIs. Updated those mocks to the existing API; the entire suite now passes.
- Changed launcher and neutral driver import successfully through the registered Bun runtime.
- `repo tools exec --light git -- diff --check -- <changed-scopes>`: passed.

## Real Native Integration

Ran the focused command (without starting another product/course build):

```sh
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype --parallelism=1 -- check --manifest-path '🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.toml' --bins
```

Initial diagnostic exposed the missing `trim-paths` feature declaration and was corrected. The final run captured the one-unit scheduler graph offline, admitted its 28,834-byte Rust source, and reached compilation. Cargo then reported `error: linker cc not found` with operating-system error 2. Read-only inspection confirmed `/usr/bin/cc`, `/usr/bin/gcc` and `/usr/bin/clang` are absent while PATH includes `/usr/bin`. Managed host C-toolchain provisioning is outside this bounded prerequisite and handed to the parent.

Earlier suspicion that source line numbers indicated stale served code was withdrawn: the actual diagnostic showed the new prototype arguments and admitted selectors. There is no evidence of a stale source closure.

The source tests do not simulate a fresh remote crate download, and the real run used the registry provisioned earlier by the backend owner. This change removes the missing preparation boundary; a successful final link awaits host toolchain provisioning.

## Authored Files

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🔖️scheduler/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🔖️scheduler/🧪️tests/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/scheduler.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/scheduler.schema.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/scheduler-bootstrap-2026-10-28.md`

No git state was modified. Temporary command log removed; other agents' generated files were retained.
