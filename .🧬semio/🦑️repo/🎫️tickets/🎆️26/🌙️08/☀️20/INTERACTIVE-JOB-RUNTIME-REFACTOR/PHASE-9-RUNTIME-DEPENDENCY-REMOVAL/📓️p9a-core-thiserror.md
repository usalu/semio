# P9a — Core Runtime `thiserror` Removal

Status: **COMPLETE for the bounded P9a-core dependency-removal packet.** The scoped native gates
pass. The actor wasm host remains blocked by pre-existing Phase 1 universal-async glue drift,
independent of the error implementation replacement.

## Outcome

The stable trace, async, job, and actor runtime crates now have zero direct normal `thiserror`
dependencies and zero `thiserror` derives. Trace, async, and job were already clean at packet
entry. Actor owned the only scoped direct edge.

Actor's three public error enums now use explicit owned implementations:

- `pack::PackError`: four variants.
- `JobPublicationError`: five variants.
- `KernelError`: three variants.

Each enum retains its existing derives other than `thiserror::Error`. Explicit
`std::fmt::Display` implementations preserve all twelve old messages byte-for-byte, and explicit
`std::error::Error` implementations preserve `source() == None`. The old derives declared no
`#[from]`, `#[source]`, or `#[transparent]` fields, so no `From` implementation existed to replace.
No third-party error type is exposed by any of these APIs.

The pre-change message/source contract was captured in `🧪️p9a-thiserror-golden.txt`. The actor
test `owned_errors_preserve_thiserror_display_and_source_contracts` constructs every variant and
checks the same display and source contract. It passes in both debug and release profiles.

## Dependency-edge reduction

`cargo tree -e normal --depth 1` at packet entry showed:

- trace: 0 direct normal edges.
- async: 2 direct normal edges (`semio-framework-trace`, `serde`).
- job: 2 direct normal edges (`semio-framework-async`, `semio-framework-trace`).
- actor: 3 direct normal edges (`semio-framework-job`, `serde`, `thiserror`).

After the change, actor has only `semio-framework-job` and `serde`. Thus the scoped direct normal
edge count is **7 → 6**, the scoped direct third-party edge count is **3 → 2**, and the scoped
direct `thiserror` edge count is **1 → 0**. `Cargo.lock` drops `thiserror 2.0.18` from actor's
package dependency list. The package remains in the workspace lockfile because out-of-scope crates
still depend on it.

The repository dependency ratchet remains at 238 unique third-party packages because removing one
direct edge does not remove a package that is still reachable elsewhere:

```text
[verify dependencies] baseline: 238 third-party dependenc(y/ies) (commit 95b8688ee2f62f4056b6403c282bf0c76172c37c); current: 238.
[verify dependencies] clean — no new third-party dependencies.
```

## Type generation

The existing async and actor Nx type-generation surfaces both pass. The actor TypeScript mirror's
SHA-256 remained `6fd596fbeab83acaf19567b76ce965414d8f4b9c532dcd08ca5f858e51cd9ef0`,
confirming that replacing non-exported error derives did not change the schema-owned boundary.
The async generator also completed and produced its ignored generated mirror; it has no scoped
source change.

## Verification evidence

Commands were run from the repository root on 2026-08-21.

| Command | Result |
| --- | --- |
| `bun nx run-many -t test-quick -p @semio-tech/framework-trace-rs @semio-tech/framework-async-rs @semio-tech/framework-job-rs @semio-tech/framework-actor-rs --parallel=1` | PASS: trace 13/13, async 43/43, job 16/16, actor 89/89; job was then rerun uncached |
| `bun nx run @semio-tech/framework-job-rs:test-quick --skip-nx-cache` | PASS: job 16/16 debug |
| `bun nx run-many -t test-quick -p @semio-tech/framework-trace-rs @semio-tech/framework-async-rs @semio-tech/framework-job-rs @semio-tech/framework-actor-rs --parallel=1 --skip-nx-cache -- --release` | PASS: trace 12/12, async 43/43, job 16/16, actor 89/89 release; trace's debug-only panic assertion is intentionally absent |
| `cargo clippy -p semio-framework-trace -p semio-framework-async -p semio-framework-job -p semio-framework-actor --all-targets -- -D warnings` | PASS |
| `cargo check -p semio-framework-trace -p semio-framework-async -p semio-framework-job --target wasm32-unknown-unknown` | PASS |
| `cargo check -p semio-framework-trace -p semio-framework-async -p semio-framework-job --target wasm32-wasip2` | PASS |
| `cargo check -p semio-framework-trace -p semio-framework-async -p semio-framework-job -p semio-framework-actor --target wasm32-unknown-unknown` | BLOCKED in actor's pre-existing async wasm glue; lower three crates compile |
| `cargo check -p semio-framework-trace -p semio-framework-async -p semio-framework-job -p semio-framework-actor --target wasm32-wasip2` | BLOCKED by the same actor glue drift |
| `bun nx run-many -t typegen -p @semio-tech/framework-async-rs @semio-tech/framework-actor-rs --parallel=1 --skip-nx-cache` | PASS: both export tests 1/1 |
| `bun ./📜️script.ts verify dependencies` | PASS: baseline 238, current 238, no new dependency |
| scoped `git diff --check` for actor component and manifest | PASS |
| scoped `rg` for `thiserror`, `#[error]`, `#[from]`, `#[source]`, and `#[transparent]` in the four core source/manifests | PASS: zero production matches; trace's module documentation mentions its zero-dependency policy only |

Workspace `cargo fmt` could not start because unrelated stdio MP4 files contain active Phase 1.5
syntax corruption. The owned actor source was formatted directly with repository rustfmt settings,
and scoped diff checking is clean.

## Actor wasm blocker outside P9a

Both wasm targets reach the actor host at
`🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📦️glue.rs` and fail with 15 errors from the
ongoing universal-async conversion:

- `pack::read_opt` no longer exists.
- `Kernel::new`, `Kernel::tick`, `Kernel::complete`, `Kernel::metrics`, pack decoders, encoders,
  `submit`, and `activate` are now futures but the wasm glue has stale synchronous calls.
- Exported async wasm methods require the currently undeclared `wasm_bindgen_futures` support
  crate, and the async constructor emits the corresponding wasm-bindgen deprecation warning.

These failures reproduce identically before any `thiserror`-specific code is exercised and require
an actor wasm-host contract repair, not an error API compatibility layer. P9a did not add a runtime
dependency or broaden into that Phase 1 repair.

## Files changed by P9a

- `Cargo.lock` — actor package dependency-list removal only; concurrent lockfile changes preserved.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml`.
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`.
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️p9a-thiserror-golden.txt`.
- This report.

Trace, async, job, generated mirrors, and all explicitly excluded product/plugin/schema areas have
no P9a source edits.
