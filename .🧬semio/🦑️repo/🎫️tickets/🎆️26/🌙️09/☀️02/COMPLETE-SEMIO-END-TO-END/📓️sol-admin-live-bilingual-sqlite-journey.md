# Sol Admin Live Bilingual SQLite Journey

## Scope

This packet adds one registered, uncached Nx journey that drives the shipped admin SPA through the protected loopback relay into a real hub process backed by a ticket-local SQLite directory. It proves English and German UI selection, bounded reads, one typed durable mutation, operation polling/cancellation, and shutdown. It makes no production OIDC claim.

## Neutral contract

- `semio.hub.admin-live-journey/v1` is represented by a JSON Schema and literal fixture covering the administrator profile, 60-second journey budget, 50-millisecond bounded polling, 64-KiB API response ceiling, EN/DE labels, `create-space`, and `rebuild-directory-projections` intents.
- `semio.hub.local-bootstrap-idle-admission/v1` is represented by a JSON Schema and literal frame fixture. Its independent Bun/AJV/Buffer oracle requires an idle interval longer than the 15-second exchange deadline and proves the admitted frame prefix exactly matches the payload.
- The registered journey validates both schemas and the independent frame/bilingual/bounded-intent oracle before Rust compilation, then retains that exact fixture instance for the real browser run.

## Production repairs discovered by the journey

- `DbIoU64List` moved its 4096-entry backing off worker stacks. List admission now reserves fixed credit for the task plus two 32-KiB transient backings before allocation, and releases the exact ledger witness on every close path. A plain production hub build completed without the prior worker stack abort.
- List backing admission is ordered after aggregate-credit attachment. If backend or page admission later rejects, the unstarted heap backing is dropped before task credit is detached; the process and operation ceilings therefore cover both the driver source and transfer destination without an uncredited allocation interval.
- `LocalBootstrapTransport::accept` now owns cancellation rather than a precomputed exchange deadline. The inherited transport polls cancellation while idle, starts a fresh 15-second deadline only after the first frame byte is admitted, and bounds the remaining prefix, body, validation, issuance, and delivery.
- `/admin/` is an explicit static SPA route instead of falling between `/admin` and `/admin/{*path}`.
- The local relay keeps API responses at 64 KiB and uses a separate bounded 4-MiB static ceiling, sufficient for the current 3.12-MiB shipped entry asset. Static and API requests share the 64-request admission cap and stop/downstream abort ownership.
- SQLite projection rebuild snapshots and atomically restores the co-located user/SSO/password identity rows before replay, then restores live auth sessions, invite credentials, and sync-session bindings. This preserves the `SpaceCreated.owner_id` authority needed by event replay and prevents foreign-key cascades from destroying non-projection authority state.

## Laws and evidence

- Session `437d38`: actual Tokio idle/admitted-frame law, 1 passed, 0 failed. The listener survived an injected clock jump beyond 15 seconds before the first byte; a post-admission partial frame failed closed after its deadline.
- Session `04214b`: current-source `cargo build --bin os-hub`, exit 0, without a larger worker stack.
- Session `dc0019`: runtime journey reached the actual SPA, selected English, read SQLite overview, submitted and durably read `create-space`, started/polled the rebuild operation, and exposed the pre-repair `directory-rebuild-rejected` terminal.
- Session `42502`: focused final-source law retry terminated before either owned assertion because concurrent plugin-host source had two stale `spawn_recoverable` recovery-closure arities (`E0593`). Those call sites were repaired by their owning lane afterward; this session is compile-blocker evidence, not a law result.
- Session `92885`: the first exact DB heap-credit attempt stopped before assertion on three stale DB lib-test self-includes under preview/artifact. The self-includes now resolve to their physical current modules.
- Sessions `63182`/`21548`: the exact idle/admission law stopped before assertion on a concurrent trusted-catalog test-only `E0509` at `trusted-catalog/🦀️.rs:1190`. The error moves `fixture.schema` out of a `Drop` owner and is outside this packet.
- Session `64458`: registered `os-hub:admin-relay-check`, exit 0. The protected relay oracle passed and the real admin SPA test file passed 15/15.
- Session `43400`: registered plugin-registry `check-generated`, exit 0. Generated catalog and `.vscode/launch.json` are byte-fresh from their owned sources; project, seed, and generated launch all contain the exact uncached journey target.
- Final exact DB invocation: `cargo test --lib 'db_storage::db_io_retained_fixtures::db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger' -- --exact --test-threads=1`, exit 0; 1 passed, 0 failed. The law proves allocation-free construction, exact task/operation/process heap credit including two transient 32-KiB backings, max+1 rejection, and complete ledger release.
- Registered sessions `69619`, `69165`, and `43740` were invalidated during concurrent source changes before browser acceptance. Session `69165` did prove the registered idle/admission Rust law 1/1 and the SPA build before a later hub binary rebuild failed on a transient shared-tree state. They are not recorded as full gate PASS.
- Session `47498`: the diagnostic real journey exited 0 after the identity-preserving rebuild repair: SQLite overview/create/read, English/German UI, operation polling, and the already-terminal cancellation race all passed. Its temporary command and diagnostics were then removed.
- Session `30455`: final registered `bun nx run os-hub:admin-live-journey-check --skip-nx-cache`, exit 0. The neutral AJV/Buffer oracle, both exact Rust laws, SPA build, production hub binary, protected loopback bootstrap/relay, real SQLite browser reads, typed durable mutation, operation polling/cancellation, EN/DE selection, and bounded shutdown all passed from final source. The banner explicitly makes no production OIDC claim.

## Stable boundary and qualifications

- The registered journey is a local loopback/SQLite development acceptance path. It does not prove a production OIDC deployment.
- Temporary hub startup diagnostics and the diagnostic-only runtime command are absent. The ticket-local journey compiler target is removed after the final diff check; durable schema, fixture, source, launch ownership, and this report remain.

The earlier admin backend boundary remains qualified: six portable Rust laws, relay, SPA, and all-feature source check were green, while the mandatory PostgreSQL runtime law remains environment-red because `/Users/ueli/.docker/run/docker.sock` is absent. It was not skipped or weakened.
