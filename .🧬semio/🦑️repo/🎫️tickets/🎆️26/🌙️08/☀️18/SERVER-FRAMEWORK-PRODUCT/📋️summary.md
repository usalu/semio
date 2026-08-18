# Server Framework Product — session summary

Ticket stays **open**: Wave 1 and two thirds of Wave B are landed and green; the rest is blocked on
one design decision (B3) recorded in `📋️waveB-db-decoupling-plan.md`.

## Landed

**`🧰️framework/🔨️modules/📡️replication`** — crate `semio-framework-replication`, `[lib] name = "protocol"`.
The product-neutral wire contract, extracted out of the os kernel: frames, causal envelopes, the
mutation contract, conflict vocabulary, `.spr` format, codec floor, `⚠️diagnostic`, `🌱️value`.
TS twin `@semio-tech/framework-replication` with the 20 wire fixtures as the single canonical copy.

**`🧰️framework/🔨️modules/🎒️pack`** — crate `semio-framework-pack`, `[lib] name = "pack"`.
The `.spk` container: header/footer/segments/manifest/chunk table/recovery + native/async/http sources.

**`🧰️framework/🔨️modules/⚠️diagnostic`**, **`🧰️framework/🔨️modules/🌱️value`** — crate-less, mounted once
by the replication crate (`Severity`/`Fault`/`TextSpan`; `DslValue` + serde bridge).

**`🧰️framework/🛍️products/🖥️server`** — the fourth framework product, registered in the products
manifest as `framework.product.server`, with a real `🔨️modules/🧬️contract` crate
(`semio-framework-server-contract` types live in it today via `semio-framework-server`, `[lib] name = "server"`):
`ActorKey`, `CommandEnvelope`, `CommandOutcome{Accepted,Transformed,Rejected,Pending}`, `OfflinePolicy`,
`QueryEnvelope`, `QueryConsistency{Local,AtFrontier,Authority}`, `EventRecord` vs `EphemeralFrame`,
`PolicyPoint`/`PolicyTemplate`, `ModuleManifest`, `ServerInstanceDefinition`. TS package + nx + launch.json wired.

**Guard**: `every_path_mount_in_this_glue_resolves_to_an_existing_file` in the kernel glue — a moved
file whose `#[path]` mount was not updated is now one named failing test instead of a repo-wide red.

## The facade result worth keeping

The approved plan budgeted a sweep of ~45 crates that alias `semio_framework_os_kernel as protocol`.
None needed changing: the kernel re-exports the extracted modules as a facade, so every historical
`protocol::` / `os_pack::` / `os_dsl::` path still resolves. Only db and hub were repointed, because
they should depend on the contract directly.

## Verification (scoped `-p`; never workspace-wide, see `s0-baseline.txt`)

replication 185 (184 + deflate) · pack 42 · os-kernel 776 · db 424 · server 5 · hub compiles ·
kernel and pack wasm `--lib` clean · replication TS fixture parity 1 · os TS 184/185.

776 + 42 = 818 of the original 820 kernel tests; the other 2 moved into the replication crate's own suites.

## Known-not-mine (evidence in `s0-baseline.txt`)

- `cargo check --workspace` is red in `semio-framework-plugin-host` and `semio-framework-os-renderer-wgpu`:
  a concurrent session is mid-restructure there (`🔌️plugin/🧵️shard/` no longer exists).
- `bun install` fails repo-wide: `package.json:249` patches `@electron-forge/core-utils@7.11.2` but
  `patches/` does not exist. Worked around by hand-symlinking the two new packages into `node_modules/@semio-tech/`.
- os TS `plan_workflow … decoded via wasm` fails: `🖥️host/📦️packages/🦀️rust/pkg/` was never built here.
- Two pre-existing db test-build breaks found and fixed (facade re-exports; a Cargo.toml comment
  tripping a substring guard).

## Next

Resolve B3 (`📋️waveB-db-decoupling-plan.md`), then B4/B5, then the server core
(`🎭️authority`, `🗄️storage`, `🛡️policy`, `📡️gateway` + testkit instance), then hub onto `Server::builder`.

## Post-hoc wiring verification (gap found and closed)

The first `bun nx run @semio-tech/framework-replication:test` failed, which surfaced that the two new
**Rust** packages had a `Cargo.toml`/`📦️glue.rs` but no `📋️project.json`/`📜️script.ts` — they were
invisible to nx. Added for both; all five new scripts parse under bun.

nx now discovers every new project, with `test-quick`/`test-long`/`test-exhaustive` auto-derived from
`test` by the emoji plugin exactly as the convention expects:

| project | targets |
|---|---|
| `@semio-tech/framework-replication-rs` | build, test, test-quick, test-long, test-exhaustive |
| `@semio-tech/framework-pack-rs` | build, test, test-quick, test-long, test-exhaustive |
| `@semio-tech/framework-server-rs` | build, test, test-quick, test-long, test-exhaustive |
| `@semio-tech/framework-replication` (ts) | test (+ derived levels) |
| `@semio-tech/framework-server` (ts) | test (+ derived levels) |

No nx name collisions introduced (two pre-existing duplicates elsewhere in the repo: one unnamed
`📋️project.json` and a duplicated `@semio-tech/assets` — neither mine).

Manifest bijection verified both ways: `🛍️products/🔣️component.json` ≡ the four product dirs, and
`🖥️server/🔨️modules/🔣️component.json` ≡ `{🧬️contract}`.

Running an nx *target* end-to-end is impractical right now — the box has ~500 nx/cargo processes from
concurrent sessions and target startup exceeds ten minutes. The underlying suites were verified
directly instead (`bunx vitest run` for TS, `cargo test -p` for Rust).
