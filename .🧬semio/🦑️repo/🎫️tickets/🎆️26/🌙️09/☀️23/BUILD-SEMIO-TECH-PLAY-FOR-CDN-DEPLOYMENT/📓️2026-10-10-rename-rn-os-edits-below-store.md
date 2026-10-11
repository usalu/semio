# rn-os: edits made below the store before ownership split (09:58-10:15)

Made while unblocking the os foundation, before `value-core` took ownership of `🧵️job`, `📡️replication`, `🚪️io`. `value-core` should keep or supersede them.

- `🧰️framework/🔨️modules/🧵️job/🦀️.rs`: mounted the two orphan submodules (`#[path="🏃️work/🎟️grant/🦀️.rs"] mod retained_work;`, `#[path="♻️session-return/🦀️.rs"] mod session_return;`); `WorkerJobSessionArc` now wraps `Option<session_return::SessionHandle<J>>`; session `Arc::new`/`strong_count`/`try_unwrap` sites use `SessionHandle::{new,is_unique,try_return}`; the 3 `StepContext` initializers set `retained_work: RetainedWorkBudget::new(NO_RETAINED_WORK)` (no `StepBudget::with_retained_work` / `retained_work_*` accessors exist yet; the job tests that use them still fail).
- `🧰️framework/🔨️modules/🧵️job/♻️session-return/🦀️.rs`, `…/🔐️handle/🦀️.rs`: `return_terminal` uses `self.inner.0.take()` instead of `ManuallyDrop`/`inner_returned`; handle gained `is_unique()`.
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`: ported to the sealed value API (`project_owned(.., grant)` returns `(projection, progress)`, `take_authority(grant)` returns `Pending/Ready`; `returned`/`Unwrap` phase removed).
- Result at 10:15: `cargo check -p semio-framework-job --lib` and `-p semio-framework-replication --lib` pass; `semio-framework-io-sqlite-snapshot` (12 errors: `SqliteSnapshotControl.ledger`, `SchemaValidationStorage`, `with_retirement_owner`) is the remaining blocker below `semio-framework-os-kernel`.
