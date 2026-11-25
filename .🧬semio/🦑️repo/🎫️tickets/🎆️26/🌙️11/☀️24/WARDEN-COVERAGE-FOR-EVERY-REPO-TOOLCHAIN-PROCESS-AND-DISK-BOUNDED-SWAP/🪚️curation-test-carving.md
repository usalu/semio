# 🪚️ Curation unit-test carving (2026-11-25) — status: measured, change reverted

## Closure analysis (cargo metadata, normal + dev edges of `semio-s-artifact-sourcing-curation`)

- 774 packages in total (714 workspace `semio_*` crates are linked into `curation_unit`).
- Every sub-crate of the curation domain (domain, operations, schema, inferences, tests, examples) declares the same ~30 heavy dependencies, so cargo-level carving of the domain crates does not shrink the test closure: the editor tests (92 of 108) need `semio_plugin` (540 crates with kernel and framework) at minimum.
- The one separable block: `semio_s_artifact_stdio_semio::SemioMembers` adds about 180 crates (editor/schema/mutation families of the semio standard plus 18 member crates) to the `semio_plugin` base of 540. The tests mounted `VcsArtifactApp<EditorApp<SourcingCurationApp>, SemioMembers>`, i.e. the monomorphised dispatch of all 18 member families. `VcsArtifactApp<A, M>` takes the roster as a separate parameter (only the production builder uses `E::Members`), and the tests only ever open the `kit` catalogue child.
- Tests of the pure layers (domain/operations/inferences/schema/diff) already live in their own crates; the pure part inside `curation_unit` (binary/text/sqlite snapshot) needs the kernel and plugin as well (store, protocol, plugin declaration), so a second target would not reduce the editor target.

## Change

- Harness defines `CurationTestMembers` (kit family only, `dsl::space_member_groups!`) and the harness app, the registry constructor and the SQLite test apps use it instead of `SemioMembers`.
- Dev-dependency `semio-s-artifact-stdio-semio-kit-member` added to the facade package.
- Production `SourcingCurationApp::Members` stays `SemioMembers`.

## Measurement

Blocked: a peer's temporary `mod dbg` with `#[path=…/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs]` is still in four framework sources (`🌱️value/🛬️decode/🧳️owned/{🏗️record-machine,🧳️original,🧬️protocol}`, `🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record`) while the file was deleted with that ticket's generated folder, so every cargo build fails with "module dbg has no single matching file" (same incident as in 🔗️test-link-memory.md §6). Not mine to remove. Re-run once clear:

`OUT=<ticket>/🗑️generated sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/native-cargo.sh run1 ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation test --test curation_unit` with `🔗️link-sampler.sh` running; expected 108 tests; then set `curation_unit|host` in the admission schema to the measured value or remove it if below 4096.

Before: 714 rlibs, 424 MB text, ld 6.05 GB, unit 7179 MB. After: pending.

Expectation (unverified): dropping about 180 of 714 crates and the 18-way member dispatch lowers the link, but 4096 is not certain; if it stays above, the next lever is a lighter `SourcingCurationApp` member declaration (production-visible, needs a decision).

## Outcome (2026-11-25, measured after the orphaned `mod dbg` ledger was stripped)

- The kit-only roster broke `curation_unit`: the SQLite declaration test calls `crate::artifact::<SqliteApps>()`, and `SourcingApplication` is bound to `SemioMembers` (production fleet), so `SqliteApps` must keep the production roster. Fixed by restoring `SemioMembers` there.
- With the kit-only roster on the editor harness: 108/108 pass; Apple `ld` peak 5.96 GB (was 6.05 GB), unit peak 7737 MB in the warden log. The roster is not what drives link memory, so it was reverted (harness back to `SemioMembers`, `semio-s-artifact-stdio-semio-kit-member` dev-dependency removed): tests stay faithful to the production fleet.
- Carving entry `curation_unit|host` set to the honest value: measured 7737 MB, budget 8200 MB. Admission reserves the budget before the unit starts, so on a 16 GB host it runs alone and cannot exhaust application memory; the warden reports above budget below danger and cancels at danger.
- Remaining real lever: a test target whose closure excludes most of the 540-crate `semio_plugin` floor, i.e. framework-level carving of the mounted-app harness (`semio_plugin::artifact_app_laws`) — not a sourcing change.
- Admission/warden suites: 10 of 12 test files pass; the 2 failures (`🧪️tests/🟦️.ts` "Actual finite memory capacity required" from a live host-capacity read, `🔭️probe` live broker verdict `admitted:false` under load) read the live host and do not touch carving entries.
