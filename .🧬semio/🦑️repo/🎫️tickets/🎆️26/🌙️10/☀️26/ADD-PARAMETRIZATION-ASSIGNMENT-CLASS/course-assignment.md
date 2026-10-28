# Course Assignment Integration

## Result

Added schema-first ungraded Assignment, AssignmentDocument and AssignmentDocuments definitions, TypeScript types and explicit Rust projections. Course and CourseView optionally carry assignments; existing quizzes, badges and scoring remain independent. Assignments reference one course topic and two language-specific published PDF artifacts. The architecture manifest registers Fingerübung Parametrisierung with English and German metadata.

The browser shows a localized accessible download section on the course overview and the corresponding topic. Both links use the current language, same-origin PDF publication and the download filename. They do not require the proctor to fetch a worksheet and use the existing local course material projection.

The print publishing agent owns the Vite/build/deployment publication. This integration expects /assignments/parametrization-en.pdf and /assignments/parametrization-de.pdf. Manifest artifact references identify namespace print, catalog IDs parametrization-en/de and those filenames. No filesystem path or TeX source leaks into the public assignment contract.

## Validation And Tests

Language-neutral JSON vectors and Gherkin specify accepted localized assignments, missing translations, non-PDF/traversal filenames, undeclared properties, duplicate IDs and unknown topics. The TypeScript suite compares the owned validator to independent AJV validation, then verifies solution-free CourseView projection. The existing Rust validation suite consumes the same JSON fixture and checks serde structure, validation and projection.

Red verification: AJV initially failed because the Assignment definition did not exist.

Green verification through registered repository tools:

- Full course core Vitest configuration: **19 files, 591 tests passed**.
- Focused post-final-schema check: **2 files, 10 tests passed**, assignments and schema projection.
- Ticket assignment-conformance.mjs: all seven asserted conformance scenarios passed, including AJV/owned-validator agreement and projection.
- Ticket assignment-view.mjs: existing React server renderer and TypeScript compiler confirmed localized accessible markup, language-specific PDF filenames and topic filtering.
- Python's existing wire-contract oracle regenerated the fingerprint after raising the contract version from 9 to 10. Final fingerprint: **678b91fb9e03e81a**.

The standard repo run and repo cargo ingress both failed during repository source admission before executing their requested checks, with ENOENT for C:\git\semio\.cache\b\l\fixed.dir. Vitest was then run successfully using repo tools exec --light node and the installed Vitest CLI. Rust compile execution is therefore unverified; the same-fixture Rust test is present and the two existing explicit Course test constructors were updated.

## Design Audit

Assignments are ungraded authored material rather than artificial quizzes. Their IDs are unique, referenced topic IDs must exist, both languages are required, PDF filenames have a bounded safe alphabet, and all additional properties are refused. TypeScript has no new runtime library dependency; existing AJV, React and TypeScript libraries are used only for independent verification. No AGENTS file, git state or worktree was changed.

The public schema version increase is intentional: CourseView changed and the existing contract requires every structural change to raise the wire version. The existing Python oracle generated the new pinned fingerprint.

## Files

- 🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/📚️course/🧬️schema/🟦️.ts
- 🧰️framework/🛍️products/📚️course/🧬️schema/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🧬️schema/📚️quiz-documents/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🧬️schema/👁️read-views/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🔨️modules/✅️validation/🟦️.ts
- 🧰️framework/🛍️products/📚️course/🔨️modules/✅️validation/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🔨️modules/👁️views/🟦️.ts
- 🧰️framework/🛍️products/📚️course/🔨️modules/👁️views/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🧪️tests/🧷️companion-support/🧾️lifecycle/🦀️.rs
- 🧰️framework/🛍️products/📚️course/🧪️tests/🎚️config/🟦️.ts
- 🧰️framework/🛍️products/📚️course/🧪️tests/📝️assignments/🟦️.ts
- 🧰️framework/🛍️products/📚️course/🧪️tests/📝️assignments/🥒️.feature
- 🧰️framework/🛍️products/📚️course/🧫️fixtures/📝️assignments/🔣️.json
- 🧰️framework/🛍️products/📚️course/🧫️fixtures/🤝️wire-version/🔣️.json
- 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/📝️assignments/🟦️.tsx
- 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx
- 🎓️teaching/🏛️architecture/📚️course/🔣️.json
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-conformance.mjs
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-view.mjs
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/course-assignment.md
