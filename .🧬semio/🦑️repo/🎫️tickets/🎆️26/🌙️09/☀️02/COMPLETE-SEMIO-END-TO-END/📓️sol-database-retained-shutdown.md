# Retained Database Shutdown

Status: source-complete on 2026-09-05. The registered schema/oracle target is green after an observed strict-AJV red. Native laws are registered but not yet run.

## Ownership contract

`Database::shutdown` now borrows `&mut self` and a `DatabaseShutdownControl`; the consuming `shutdown(self, Duration)` API no longer exists. `shutdown_step` exposes bounded progress, shared-owner blockage, interruption, and terminal acknowledgement. Dropping either future keeps the Database and every mounted owner in the caller.

The Database has one `closing_authority` slot. It moves only an exclusively owned authority out of the open registry, advances at most one retained actor step, and empties the slot only after the actor's shared terminal witness is visible. A live `ArtifactHandle` yields `Blocked(Authorities(n))`; it is never drained or closed underneath its caller.

`ArtifactAuthority::shutdown_step` closes the mailbox, requests cancellation, advances one retained runner turn, and reads a shared terminal flag. It does not take the one-shot receiver into a cancel-unsafe future.

`VersionGraph::shutdown_step` advances at most one retained document Store. The VCS implementation reinserts the exact Store on Pending, Blocked, and Err and removes its registry cell only after terminal Store retirement. A test-only fault is injected after the Store is taken and proves the identical cell and Store are restored before the error escapes.

Shutdown emission is initiated once after authority and graph retirement. Cancellation before any bounded step returns without mutation; cancellation after mounting an authority leaves that exact pointer in the Database for retry. A deadline or cancellation error from the terminal driver also preserves the borrowed Database.

## Call sites

All actual repository callers were migrated to the borrowed control: DB facade laws, engine laws, testkit, every CLI command, and the Hub recovery server. Callers with live handles explicitly retire them before terminal shutdown. No duration-shaped compatibility overload remains.

## Laws and registration

The language-neutral fixture is `db/engine/fixtures/shutdown`: strict JSON Schema plus four traces for cancellation during actor retirement, Store-close error/retry, shared-handle blockage, and idempotent terminal acknowledgement. The Bun oracle independently executes the state machine and checks production ownership markers.

The registered native group selects:

- `db_engine::vcs_integration::retained_tests::vcs_shutdown_error_reinstalls_exact_store_and_retry_reaches_terminal`
- `db_engine::tests::database_shutdown_cancellation_and_vcs_error_preserve_exact_retry_owners`
- `db_engine::tests::database_shutdown_shared_authority_blocks_without_closing_live_handle`

Registered Nx targets are `database-shutdown-check` and `database-shutdown-native-check`. Launch generation is owned by the shared plugin registry.

## Receipts

The first registered source run was RED in strict AJV because the four-element tuple omitted exact `minItems`/`maxItems`. The schema was corrected rather than weakening AJV.

```text
NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:database-shutdown-check --skip-nx-cache --output-style=stream
exit 0
database-shutdown-independent-oracle: AJV=1 cases=4 retained-authority=1 retained-graph=1 terminal-ack=1
```

This receipt is source/schema evidence only. No Rust compile or native runtime claim is made yet.

## Files

- `db/engine/rs`: retained shutdown state, VCS step, tests, call sites.
- `db/artifact/rs`: shared actor terminal witness and borrowed shutdown step.
- `db/version-graph/rs`: bounded shutdown vocabulary.
- `db/rs`, `db/cli/rs`, `db/testkit/rs`, and Hub `bin.rs`: borrowed caller migration.
- OS Rust package `script.ts` and `project.json`: source/native targets.
- `db/engine/fixtures/shutdown`: neutral schema and fixture.
