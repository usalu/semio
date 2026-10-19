# Course Wire Source Audit

Read-only source audit on 2026-10-19. Scope: current proctor Rust bootstrap/package wiring, course loading and query/command prefixes; Course TypeScript/Rust schema identities; Rust companion fixture integration. No native compiler or runtime tests were run by this audit. Root AGENTS.md applies; no nested teaching AGENTS.md was found by the targeted file listing. No source files were changed.

## P2: Rust Companion Test Graph Retains Removed Crate and Contract Names

All paths below are relative to `🧰️framework/🛍️products/📚️course/`.

The schema manifest `🧬️schema/📦️packages/🦀️rust/Cargo.toml:25` names its library `course_schema`. Companion manifests depend on `semio-framework-course-schema`; there is no declared `quiz_schema` dependency alias. The following source files still import `quiz_schema`:

- `🧪️tests/🧷️companion-support/🧾️lifecycle/🦀️.rs:7` (also qualified references at 62, 67, 124).
- `🧪️tests/🧷️companion-support/🃏️sheet/🦀️.rs:3`.
- `🧪️tests/🎞️companion-runs/🧾️lifecycle/🦀️.rs:5`.
- `🧪️tests/🎞️companion-runs/👁️views/🦀️.rs:6`.
- `🧪️tests/🥣️companion/🃏️sheet/🦀️.rs:4`.
- `🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs:9`.
- `🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs:8`.

The support package is now `semio-framework-course-tests-support`, yielding crate name `semio_framework_course_tests_support`, but module unit tests still import `semio_framework_quiz_tests_support`: challenge line 6; sheet line 6; lifecycle lines 6 and 8; views lines 7, 10 and 397; badges line 7; validation line 6; scoring lines 7 and 9 (each at `🔨️modules/<module>/🧪️tests/🔬️unit/🦀️.rs`). These mounted companion tests cannot resolve those identifiers under their current manifests. Node oracle execution cannot validate Rust crate-name resolution.

Finish the rename directly; do not add compatibility aliases. The shared support lifecycle source additionally retains removed `Catalog` at lines 7, 16, 18, 102, 119 and 125, and obsolete `LearnerContext.catalog` at lines 103 and 140. The actual domain context declares `course: &Course` at `🔨️modules/🧾️lifecycle/🦀️.rs:155`. The views companion wrapper imports removed `CatalogView` at line 6; actual schema exports `CourseView`.

The same fixture builder writes `semio.quiz.catalog/v1` at support lifecycle line 20 and reads `vectors["catalog"]` at line 125. The neutral lifecycle vectors now have `"course"` at `🧫️fixtures/🧾️learner-lifecycle/🔣️.json:3`. Updating only crate identifiers would therefore leave fixture and context contract mismatches. Its helper is named `catalog`, whereas mounted lifecycle unit tests already invoke `course()` (first invocation at line 13).

## Reviewed Current Integration

The production proctor loader uses Course and Quiz separately and invokes course_issues plus nested quiz_issues. It resolves catalog entries relative to the course document without banning parent-relative topic paths. No source defect was identified in that reviewed wiring. The base and domain modules mount the current course module. Proctor queries map Course to `course.course`; command/event kinds and TypeScript client envelopes use the `course.` prefix. Both schema implementations use wire version 8. Course document identity is `semio.course/v1`, while child Quiz documents correctly remain `semio.quiz/v1`.

This establishes source consistency only in the reviewed production areas. It does not establish native Rust compilation, gateway runtime behavior, full deployment correctness, or absence of defects outside the sampled wiring/schema/fixture boundaries. Native Rust verification remains necessary after the runner prerequisite is restored.
