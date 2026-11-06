# Retained Job Publication Close Work

The publication owner is `🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🦀️.rs:57`. Its retirement demand selects terminal empty, begin-close, nested builder, and final metadata reset. The maximum next work is zero when terminal empty and one otherwise. This is an admission bound; callers must still reconcile the actual returned `RetainedWorkOutcome.work`, which is zero on a denied grant or pre-effect error.

The nested builder `📬️outcome/📦️builder/🦀️.rs:120` selects incoming page, reservation, staged owner, payload, and final metadata. Each acting branch returns work one. Staged close is `📦️builder/⛽️staged/🦀️.rs:57`; original payload page and ledger closure are `♻️retirement/📄️payload/🧱️payload-owner/🦀️.rs:6`. Those operations also return one on an acting turn. No current close branch executes multiple work units per turn. Expose a borrowed publication work quote next to `retirement_demands`, then use `close_original_quoted_retained_step`; do not discard the actual work carrier.

The existing physical, cancellation-frontier, and denied-axis native law is `📬️outcome/📤️publication/🧪️tests/🦀️.rs:10`; its close loop at line 27 uses the unquoted helper. Add a genuinely funded zero-work denial while nonempty and verify unchanged publication, no heap activity and zero work, then quote/admit a funded close. The existing test uses `HeapWitness` and `OriginalJobFixtureCaller`.

Read-only census of `📬️outcome/📤️publication/📦️packages/🦀️rust/Cargo.toml`: 2 files, 9,242 bytes, 81 active lines across default, artifact-app-testing and test configurations. The census reported no declared bound; it is not compiler admission. No native build was run.
