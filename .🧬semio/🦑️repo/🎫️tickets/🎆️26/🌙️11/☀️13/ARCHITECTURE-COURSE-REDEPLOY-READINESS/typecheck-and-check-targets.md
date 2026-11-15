# Typecheck and check targets of the architecture course

## Result
All of these exit 0 through `repo run … --skip-remote-cache`: `@teaching/architecture-course:typecheck`, `@teaching/architecture-course:check`, `@teaching/proctor:test`, `@teaching/architecture-course:test` (848 tests).

## Root causes
1. `@semio-tech/framework-rs:generate` runs `cargo test exports_typescript_bindings`, which compiles every test target of `semio-framework`. Two kernel tests still used a harness API that no longer exists (`harness::capture/receive/post_step/close_owned`, `ControlledRetirement::step(grant)`, `CancelTokenRetirement::close_step(grant)`). Ported to the effect-based drivers:
   - `🧰️framework/🔨️modules/🎠️kernel/📥️events/🧪️tests/🦀️.rs` (via `receiver-driver.rs` `driver::lifecycle`; new dev-dependency `semio-framework-value-native-owned` in the framework `Cargo.toml` and `Cargo.lock`).
   - `🧰️framework/🔨️modules/🎠️kernel/♻️retirement/🧪️tests/🦀️.rs` (`close_original_effect_step`, `EffectBank`).
2. `@teaching/proctor:build` captured 11 sources the `proctorRust` input did not declare. The `semio-framework-job-context` crate builds sibling owners' files through `#[path]` (`♻️retirement/🎟️context`, `♻️retirement/📄️payload`, `📄️payload-pages-and-ledger`, `📬️outcome/⏸️continuation`, `📬️outcome/🎟️descriptor`, `📬️outcome/📖️source`) and one JSON schema (`🛂️allocation/🧬️schema/🔣️.json`). Fixes:
   - `proctorReachedRoots` in `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts` extends the Cargo closure by `#[path]` reach from crate roots (skipping `#[cfg(test)]`), used by `proctorSourceDirectories`; unit test added in `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts`.
   - Data file added to `🛂️proctor/🏗️bootstrap/🧾️product/📥️inputs/🔣️.json`.
   - `proctorRust` (proctor `📋️project.json`) and mirror `proctorCargoManifests` (course `📋️project.json`) regenerated from the closure.
3. No quiz content issue: `proctor check` passes on the course and its ten quizzes unchanged.
4. The TypeScript typecheck pulls in the repo build library through the proctor bootstrap; stale declarations and type errors there were fixed (graph-hooks `.d.mts` policy/notice/seams, `directoryStampsFile`, new `.d.mts` for `directory-stamps` and `doubt`, tuple optional syntax, `loadGenerationGraphV1` cast, remote-exchange and job-limits contextual typing, `graph-plan-wait-cancelled` text in en+de, `observed.target` in dependency retirement). No renderer/course source had type errors.

## Unverified
The two ported Rust tests were compiled (generate passes) and the framework crate's filtered test run executed, but their own bodies were not run in isolation.
