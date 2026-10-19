# Rust Production Surface Audit

Read-only static inspection of current production crate roots, dependencies and imports, 2026-10-19. No compiler, native command, installation or source mutation was performed. Concurrent companion rename fixes are excluded.

## P1: Production Course Imports Refer to an Undeclared Root Schema

`🧰️framework/🛍️products/📚️course/📦️packages/🦀️rust/🦀️.rs` declares the production modules and component façade, but declares no `schema` module or alias. The façade `../../🦀️.rs` selectively reexports `course_schema` names, also without declaring `schema`. Nevertheless validation line 27, sheet line 21, lifecycle line 27, views line 27 and presence line 11 import through `crate::schema`. The Cargo dependency itself exports the external crate name `course_schema`, not a `crate::schema` root member. These current production imports cannot resolve. Repair the owned imports or expose the intended schema namespace consistently. This is separate from the known Topic/COURSE_ROOT_IDS omission.

## P1: Schema Omits Four Presence Types Used by Production

`🧰️framework/🛍️products/📚️course/🔨️modules/👥️presence/🦀️.rs:11` imports `PresenceBadgeAward`, `PresenceProfile`, `PresenceRun`, and `PresenceRunSummary` from the schema namespace. The actual declarations exist in `🧬️schema/👥️presence-wire/🦀️.rs` at lines 140, 145, 68 and 124 respectively. That module is private (`mod presence_wire`), and the explicit pub-use list in `🧬️schema/🦀️.rs` exports none of these four. Fixing the root namespace alone therefore still leaves four unresolved production imports. Add the authored schema reexports alongside the already-owned Topic and COURSE_ROOT_IDS repair.

The production proctor package, base and domain module declarations and manifest dependencies were also read. No further independently confirmed defect is claimed. This bounded source audit does not establish compilation, linker success or deployment readiness.

## P1: Presence Schema Module Uses Unimported Sibling Types

`🧬️schema/👥️presence-wire/🦀️.rs:5` imports only ClassificationAnswer, Identity, Slug and SortingAnswer from its parent. Its new production declarations starting at line 68 also refer to RunStatus, Sheet, Answer, Timestamp, RunResult, Hint, Challenge, Score, Best and LeaderboardPeriod without importing or qualifying those names. The file contains no other use declaration. Parent reexports do not implicitly put sibling types into a child module scope. Extend the actual presence-wire imports to the types its declarations use; the existing local present deserializer helper does exist.

## P1: Scoring Imports the Same Value Twice

`🔨️modules/📏️scoring/🦀️.rs:17` imports answer_rejection together with answer_complete; line 22 independently imports answer_rejection again. Both occupy the same module value namespace, which Rust rejects as a duplicate import. Remove the duplicate declaration while preserving the intended validation function.

## P1: Read Views Also Need the Topic Import

`🧬️schema/👁️read-views/🦀️.rs:53` declares `CourseView.topics: Option<Vec<Topic>>`, while its sole parent import at line 5 omits Topic. The known schema-root Topic reexport repair does not implicitly bring Topic into this child module. Add Topic to the actual read-views import as part of the production schema repair.

## Final Current Declaration Review After Owner Repair

After runtimeHigh explicitly notified completion, this auditor reread the current production sources. The Course façade now reexports `course_schema as schema`, which reaches the crate root through its component reexport. The schema aggregator explicitly exports Topic, COURSE_ROOT_IDS, QuestionPool and the new presence types; the presence-wire child imports the sibling types its declarations reference; read-views imports Topic; scoring contains one answer_rejection import. The earlier findings above are therefore resolved in the inspected current source.

An additional bounded read-only scan collected 79 distinct Course imports/path identifiers referenced by production proctor modules and compared them against public declarations in Course production source. It found no unmatched declaration (the separately scoped LoadedCourse belongs to proctor base). The actual Cargo library names are course, course_schema and proctor; the proctor base/domain manifests declare the expected Course dependency.

The shared support package is currently named semio-framework-course-tests-support and its exposed sheet/lifecycle modules match the current unit import spelling semio_framework_course_tests_support. The companion and companion-runs manifests declare that exact support dependency. No old quiz_schema/quiz crate path or LoadedCatalog identifier was observed in the bounded current Course/proctor Rust scan. Historical prose/metadata naming is outside this production name-resolution review.

No additional concrete P1/P2 declaration defect was identified in this final scope. These are fresh source observations only. Rust was not compiled, linked or executed here; the owner's parser/literal diagnostics and root's Node/TypeScript tests cannot establish native Rust build or application release readiness.
