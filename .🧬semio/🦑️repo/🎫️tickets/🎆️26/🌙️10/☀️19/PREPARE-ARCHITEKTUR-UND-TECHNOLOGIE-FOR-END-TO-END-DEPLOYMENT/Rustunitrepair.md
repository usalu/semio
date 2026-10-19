# Rust Unit and Schema Coherence Repair

The seven assigned domain unit files referenced a crate that is absent from the current Cargo manifests. The actual support package is `semio-framework-course-tests-support`, whose Rust crate identifier is `semio_framework_course_tests_support`. The actual schema library declares `[lib] name = "course_schema"`. Lifecycle and views already called `course()` but still imported `catalog`.

The unit imports now match those current crates and the companion owner's actual `course() -> Course` factory. No legacy aliases or child quiz schema renames were introduced. The shared learner-lifecycle neutral fixture still has `course.schema = "semio.course/v1"` and both child quiz documents use `semio.quiz/v1`.

The subsequent production audit found real unresolved paths and imports. The schema aggregator now explicitly exports its authored `QuestionPool`, `Topic`, `COURSE_ROOT_IDS` and eleven presence presentation types. The course façade explicitly reexports its declared schema dependency as its current `schema` namespace, which existing domain modules import. The presence wire module imports its twelve referenced sibling contract types. The read-view module imports `Topic`. Scoring retains exactly one import of `answer_rejection`.

## Authored Source Changes

Under `🧰️framework/🛍️products/📚️course/`:

- `🔨️modules/⛰️challenge/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/👁️views/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/🏅️badges/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🦀️.rs`
- `🧬️schema/👥️presence-wire/🦀️.rs`
- `🧬️schema/👁️read-views/🦀️.rs`
- `🦀️.rs`
- `🔨️modules/📏️scoring/🦀️.rs`

The companion owner edited the five companion files; this repair only reads those files for cross-source validation. Retained diagnostic inputs are `rust-unit/🔣️.json`, `rust-unit/🧫️.json` and `rust-unit/📜️script.ts` inside this ticket.

## Actual Red and Green Evidence

The schema-first language-neutral fixture is validated with installed Ajv. Existing framework `rustTokens` checks agree with the independently installed tree-sitter Rust grammar about the forbidden stale identifiers. Installed `@iarna/toml` reads actual Cargo package and library names. The AST checks compare schema imports against current public reexports, companion imports against actual support exports, and struct/enum literal field names against the current Rust declarations. Four neutral field-set cases additionally verify complete, missing optional, missing required and unknown fields against both plain and enum-qualified Rust literal syntax.

Observed sequence:

1. Initial scoped import diagnostic: exit 1, thirteen concrete violations. After the seven unit import repairs: exit 0.
2. Public schema export and namespace diagnostic: exit 1, fourteen omitted authored public schema items plus the absent `crate::schema` namespace. After the explicit exports and façade wiring: exit 0.
3. Sibling import and duplicate scoring import diagnostic: exit 1, twelve missing presence sibling imports plus duplicate `answer_rejection`. These were repaired.
4. Broader schema sibling check: exit 1, the missing read-view `Topic` import. This was repaired.
5. Final expanded diagnostic: **exit 0; 26 Rust files parse; 285 closed struct/enum literals checked; zero findings**. This includes the seven owned unit files, five companion files, five schema source modules and nine production domain modules. Existing analogous unit literals already provide `Course.topics` and `Command::StartRun.origin`; no extra field edit in those seven files was needed. Companion builders now provide their current `pool`, `topics` and `origin` fields.

Final generated evidence: `🗑️generated/rust-unit-final-green.json` and `🗑️generated/rust-unit-final.log`. Earlier red artifacts retain their specific finding lists. The `fields-red` artifact name was an invocation label; that expansion observed zero field findings, and is not claimed as a red result.

## Validation Limit

This is actual parser, metadata and bounded declaration/import validation. **No Rust compilation, linking, Rust test execution or native release result is claimed.** No Cargo/rustc command, compiler volume allocation, fake funding or authority bypass was used. Native compilation remains dependent on the genuine external runner and capacity admission described by the root readiness report.
