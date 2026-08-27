# DSL Facade Macro Export Repair

## Subsequent Shared Source Change

After this repair's native and facade-check checkpoint, the active mutation-leaf owner added an actual derive macro and its hash dependency. The eight-registered/seven-facade-export fixture below is historical evidence, not the current intended macro roster. Its exact roster must be refreshed against that owner's coherent registered/exported API at the next stable boundary, without weakening or removing the export test. The later registered-plugin r14 failure was in the new macro's `Option<PathBuf>` source handling; the owner corrected that and r15 proceeded into kernel compilation. This executor did not revert the new macro or its descriptor helpers.

The owner has now refreshed the shared fixture to nine registered derives and eight facade exports, including the actual MutationLeaf macro; `traitOnly` is correctly empty. Source inspection confirmed both derive source files and the facade match that roster. The previous hostile vector used an empty `traitOnly`, which is no longer invalid, so it now tests an invalid macro identifier instead. The existing strict schema, duplicate rejection, and exact source/Syn roster equality remain intact. Canonical source target passed with `facade=8 registered=9 hostileRejections=3` in `🧪️dsl-facade-current-exports-source-2026-08-27.txt`. This executor has not rerun the native Syn selector after the owner addition.

## Outcome

Removed the premature `MutationLeaf` proc-macro re-export from `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`. The derive crate does not register that macro. Its existing `MutationLeaf` trait, descriptor parser/emitter helpers, and descriptor tests are preserved. No substitute macro or compatibility adapter was introduced.

The native kernel metadata check now passes. This fixes the dependency error that stopped registered-plugin r11; it does not claim that the later plugin tests pass.

## Source Identity And Consumer Evidence

The derive primary `✨️derive/🦀️component.rs` and compiled `✨️derive/📦️packages/🦀️rust/📦️glue.rs` are separate regular files, not symlinks or hardlinks. Their observed inode numbers were 116313568 and 116412239, respectively. The package manifest names `📦️glue.rs` as its proc-macro library entry. The two bodies already differ; this packet did not copy one over the other or overwrite peer changes.

Both sources register the same eight macros: CompositeMutation, DslArtifact, DslDiff, DslEnum, DslOps, DslRecord, DslScalar, and Mutations. Neither registers MutationLeaf. The framework/app Rust consumer census found MutationLeaf as a trait and descriptor test bound, not an actual derive use. The facade legitimately re-exports seven of the registered macros; CompositeMutation is not newly exposed by this repair.

The root policy code describes the two source files as mirrored build-shape entry points. That existing drift remains separate work. Only an additive test-module mount, with its correct local path, was added to each source.

## Tests And Tooling

A new language-neutral roster plus strict JSON Schema lives beside the existing derive fixtures at `✨️derive/🧪️tests/📤️macro-exports/`. The native test uses the existing third-party Syn parser to inspect the real facade and both derive sources, independently of the source check's textual extraction.

- Native RED: `facade_exports_match_registered_macros` failed with the extra MutationLeaf facade export; 0 passed, 1 failed, 5 filtered. Log: `🧪️dsl-facade-export-red-2026-08-27.txt`.
- Native GREEN: 1 passed, 0 failed, 5 filtered; 0.89 s compilation and 0.02 s test execution. Log: `🧪️dsl-facade-export-green-2026-08-27.txt`.
- Strict source check: seven facade exports, eight registered derives, three hostile schema rejections. Log: `🧪️dsl-facade-export-source-2026-08-27.txt`.
- Canonical `@semio-tech/framework-os-kernel:check --args=--lib` passed with exit 0 in 49.26 s; 33 existing warnings. Log: `🧪️dsl-facade-kernel-check-2026-08-27.txt`.
- Canonical registry generation passed and both new source/native launch entries are present in the authoritative seed and generated launch file. Log: `🧪️dsl-facade-export-launch-2026-08-27.txt`.
- `git diff --check` passed.

The derive script now awaits its existing asynchronous test runner, and exposes exact source/native export-test routes through Bun/Nx. Native compilation used the coordinator's warm ticket target, one build job, and unchanged profiles/RUSTFLAGS. The short compiler lease has been returned to the registered-plugin executor. No peer process was interrupted and no git mutation occurred.
