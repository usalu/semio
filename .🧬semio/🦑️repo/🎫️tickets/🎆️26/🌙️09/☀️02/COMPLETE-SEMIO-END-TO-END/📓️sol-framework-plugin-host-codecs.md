# Framework and plugin-host first-party codec repair

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Packet: bounded downstream first-party codec migration needed by the real `semio-os-mcp` build

## Outcome

The original downstream codec failure is repaired:

- `semio-framework-plugin-host` production code moved its remaining `IoPayload`, `ForeignStep`, `MergePolicy`, and `AppRef` runtime boundaries off serde-only derives and onto `ToValue`/`FromValue` plus the canonical `os_pack` JSON or existing pack-runtime wire codec.
- OS opening-preference and merge-policy schema/mutation containers now use only first-party value codecs. Their committed language-neutral JSON fixtures remain the wire oracle; `serde_json::Value` is retained only inside tests as the independent structural oracle.
- `semio-framework`'s 26 stale test compile errors were migrated to the same first-party codec surface without restoring serde to `AgentContributions`, `ActionInvocation`, `CommandInvocation`, `AppRef`, or `DslValue` leaves.
- The two subsequently exposed MCP-owned codec mismatches were repaired: action-argument `DslValue` schemas cross the already-defined catalog bridge, and guest fault wire bytes are decoded/read as `DslValue` directly.
- A new language-neutral MCP codec fixture pins both the compiled input-schema shape and guest-fault extraction. It is a declared Nx input.

The original error counts are now zero. The real binary is not emitted because the next build reaches a separate concurrently changing store/replication bootstrap exhaustiveness gap (two errors, detailed below). Consequently the TypeScript 30-process matrix and exact binary smoke cannot truthfully run in this snapshot.

## Root cause and contract

The first-party leaf migration was already substantially complete, but several runtime containers and tests still invoked serde on those leaves:

- `HostArtifactMutationPlanResult` derived serde over `ForeignStep`.
- OS opening configuration derived serde over `AppRef`.
- OS merge-policy configuration derived serde over `protocol::MergePolicy`.
- the host's local `IoRunInputWire` containers derived serde over `IoPayload`, although the actual boundary already called `os_pack::json::to_json_string`.
- framework tests tried to round-trip first-party-only values through serde or passed `serde_json::Value` to the generic first-party `dsl::to_dsl_value` trait boundary.
- MCP catalog construction tried to insert `ActionArgDef::json_schema()`'s `DslValue` directly into a `serde_json::Map`; MCP guest-fault decoding incorrectly asked the first-party `FromValue` generic to manufacture a `serde_json::Value`.

The repaired contract keeps runtime domain values on `ToValue`/`FromValue`. JSON text uses `dsl::os_pack::json::{to_json_string,from_json_str}`. Binary plugin wire values continue to use the existing `store::pack_rt::{encode_wire_value,decode_wire_value}`. Serde remains only where the MCP JSON protocol itself owns a serde-shaped value or where tests use `serde_json` as an independent oracle.

## Focused verification

All commands ran from `/Users/ueli/Documents/semio` unless a package directory is explicitly shown. The ticket-only cargo target was `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target` and was removed after verification.

### Production plugin host

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target" bun nx run @semio-tech/framework-plugin-host:check --skip-nx-cache -- --message-format=short
```

Result: PASS, exit 0. The initial 21 production serde diagnostics became 0.

The exact initial 21-error attribution was:

- host `🦀️.rs:4117` and `:4133`: `IoPayload: Serialize`;
- host `🦀️.rs:6454/:6464`: `ForeignStep` serde requirements;
- OS config `🎚️config/🧬️schema/🦀️.rs:16/:22`: `AppRef` serde requirements;
- `set-default-app/🦀️.rs:13/:20`: `AppRef` serde requirements;
- `change-merge-policy/🦀️.rs:10/:14/:33/:38`: `MergePolicy` serde requirements.

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- --no-run --message-format=short
```

Result: BLOCKED after the codec-owned test diagnostics reached zero. Exactly 23 remaining errors are unrelated concurrent API drift: 8 missing `ui_patch_receipt` fields in `TurnResult` initializers, 1 obsolete `InstanceOpen { instance }` construction, and 14 stale `InstanceClose` constructor/serde uses. This packet did not edit those shard/executor APIs.

### Framework

From `🧰️framework/📦️packages/🦀️rust`:

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target' cargo test --manifest-path Cargo.toml --lib --no-run --message-format=short
```

Result: PASS, exit 0. One lib-test executable containing 266 tests was built; all 26 original compile diagnostics are gone.

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target" bun nx run @semio-tech/framework-rs:test --skip-nx-cache -- --lib canonical_json_round_trip_uses_camel_case_and_skips_empty_promoted
```

Rust phase: PASS, 1/1 test passed, 265 filtered/skipped by the focused selector. Overall Nx target: BLOCKED because the existing combined Rust+Vitest router forwarded Rust-only `--lib` to Vitest, which rejected it as an unknown option. No Rust test failed.

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target" bun nx run @semio-tech/framework-rs:check --skip-nx-cache
```

Compiler phase: PASS. The target then ran its single type-generation parity test, which failed 0/1 because the concurrently changed generated TypeScript mirror differs from current owned schema metadata. No compiler codec diagnostic remains; generated TypeScript output was not edited in this packet.

### MCP binary progression

```text
RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/framework-plugin-host-codecs-target" bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache
```

First retry after the framework/plugin-host repair: the original 21 plugin-host errors were gone and the build exposed exactly 2 MCP-owned mismatches at `🗂️catalog/🦀️.rs:316` and `🏠️workspace/🦀️.rs:866`. Both were fixed.

Second retry after those two fixes: those diagnostics were gone. The build stopped with exactly 2 upstream non-exhaustive match errors in `🏪️store/🔄️sync/🦀️.rs`:

1. line 1732 does not cover `ServerFrame::ArtifactBootstrapChunk` and `ServerFrame::ArtifactBootstrapDone`;
2. line 1739 does not cover `Bootstrap::ArtifactBootstrap`.

These originate in the concurrently changing replication/store bootstrap graph, which this packet was explicitly forbidden to edit. No `semio-os-mcp` binary was emitted. Therefore:

- TypeScript MCP quick real-process matrix: 0/30 started; blocked before process launch by the missing binary.
- exact binary smoke: not run; no binary exists at the deterministic target path.
- WGPU quick: not run because the required MCP build did not complete and WGPU source/config was concurrently owned and out of scope.

### Language-neutral and independent oracle

```text
bun -e 'const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🔣️first-party-codecs.json"; const fixture = JSON.parse(await Bun.file(path).text()); if (fixture.catalog.capabilityId !== "cad.editor.translateSelection" || fixture.guestFault.expected.code !== "mutation.fixture-rejected") throw new Error("codec fixture contract mismatch"); console.log("codec fixture oracle: 1/1 passed")'
```

Result: PASS, 1/1. Rust catalog/workspace tests consume the same fixture and compare against `serde_json` as the independent oracle. Those two MCP Rust tests cannot compile/run until the two upstream store matches are completed. The three existing OS config fixture suites were likewise migrated to decode/encode domain values with `os_pack` while comparing their committed JSON through `serde_json`; their containing plugin-host lib-test target is blocked only by the 23 unrelated API-drift errors above.

### Diff hygiene

```text
git diff --check -- <the 13 packet-owned product/config/test paths>
```

Result: PASS, no output.

The task-owned generated target was removed with an exact-path depth-first delete. No other ticket-generated directory was removed.

## Files changed

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/repins-the-cad-editor-to-the-drafting-app/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/tightens-the-authority-to-vigilant/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🔣️first-party-codecs.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json`
- this report.

No runtime dependency, compatibility layer, replication wire change, hub change, WGPU source/config change, descriptor backend change, GIS change, or `AGENTS.md` change was introduced. No new executable developer command was added, so no launch-configuration entry was appropriate.
