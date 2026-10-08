# Architect Remaining Native Compiler Receipt

Actual current native compile errors, before direct consumer repairs. Verification must run again after all relevant repairs.

```text
error[E0063]: missing field `index` in initializer of `create_stakeholder::component::CreateStakeholder`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
532 |         "stakeholders" => create!(CreateStakeholder, create_stakeholder, stakeholder, default_stakeholder(label)),
    |                           --------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_user_profile::component::CreateUserProfile`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
533 |         "users" => create!(CreateUserProfile, create_user_profile, user_profile, default_user(label)),
    |                    ---------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_activity::component::CreateActivity`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
540 |             create!(CreateActivity, create_activity, activity, item)
    |             -------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_function::component::CreateFunction`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
542 |         "functions" => create!(CreateFunction, create_function, function, default_function(label)),
    |                        --------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_program_element::component::CreateProgramElement`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
543 |         "elements" => create!(CreateProgramElement, create_program_element, program_element, default_element(label)),
    |                       ---------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_risk::component::CreateRisk`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
544 |         "risks" => create!(CreateRisk, create_risk, risk, default_risk(label)),
    |                    ----------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_requirement::component::CreateRequirement`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
545 |         "requirements" => create!(CreateRequirement, create_requirement, requirement, default_requirement(label)),
    |                           --------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_assumption::component::CreateAssumption`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
546 |           "assumptions" => create!(
    |  __________________________-
547 | |             CreateAssumption,
548 | |             create_assumption,
549 | |             assumption,
...   |
554 | |             )?
555 | |         ),
    | |_________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_constraint_record::component::CreateConstraintRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
557 | /             create!(
558 | |                 CreateConstraintRecord,
559 | |                 create_constraint_record,
560 | |                 constraint_record,
...   |
570 | |                 )?
571 | |             )
    | |_____________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_compliance_record::component::CreateComplianceRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
574 | /             create!(
575 | |                 CreateComplianceRecord,
576 | |                 create_compliance_record,
577 | |                 compliance_record,
...   |
587 | |                 )?
588 | |             )
    | |_____________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_approval_record::component::CreateApprovalRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
590 |           "approvals" => create!(
    |  ________________________-
591 | |             CreateApprovalRecord,
592 | |             create_approval_record,
593 | |             approval_record,
...   |
602 | |             )?
603 | |         ),
    | |_________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_meeting_record::component::CreateMeetingRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
605 | /             create!(
606 | |                 CreateMeetingRecord,
607 | |                 create_meeting_record,
608 | |                 meeting_record,
...   |
613 | |                 )?
614 | |             )
    | |_____________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_analysis_record::component::CreateAnalysisRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
616 |           "analyses" => create!(
    |  _______________________-
617 | |             CreateAnalysisRecord,
618 | |             create_analysis_record,
619 | |             analysis_record,
...   |
624 | |             )?
625 | |         ),
    | |_________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_report_record::component::CreateReportRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
626 |           "reports" => create!(
    |  ______________________-
627 | |             CreateReportRecord,
628 | |             create_report_record,
629 | |             report_record,
...   |
639 | |             )?
640 | |         ),
    | |_________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_issue::component::CreateIssue`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |             (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                               ^^^^^^^^^ missing `index`
...
641 |         "issues" => create!(CreateIssue, create_issue, issue, default_issue(label)),
    |                     --------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0063]: missing field `index` in initializer of `create_template_record::component::CreateTemplateRecord`
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs:528:31
    |
528 |               (ProgramMutation::$variant(leaves::$module::$variant { $field: item }), id)
    |                                 ^^^^^^^^^ missing `index`
...
642 |           "templates" => create!(
    |  ________________________-
643 | |             CreateTemplateRecord,
644 | |             create_template_record,
645 | |             template_record,
...   |
655 | |             )?
656 | |         ),
    | |_________- in this macro invocation
    |
    = note: this error originates in the macro `create` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0308]: mismatched types
   --> 📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:127:20
    |
125 |         let mut next = base.clone();
    |                        ------------ expected due to this value
126 |         for operation in &emit.config_mutations {
127 |             next = operation.diff(&next).into_parts().0;
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ArchitectConfig`, found `ArchitectConfigDiff`

[native:owner-command] running elapsedMs=420090
[cargo:build] running elapsedMs=320076
[artifact-rust:semio-s-artifact-architect-program:test] running elapsedMs=424015

error[E0277]: the trait bound `&mut <D as CollectionDelta>::Row: Identified<EntityId>` is not satisfied
   --> 📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️algebra/🦀️.rs:130:63
    |
130 |         if let Some(row) = added.iter_mut().find(|row| row_id(row) == id) {
    |                                                        ------ ^^^ the trait `dsl::Identified<standards::v1::subsets::any::schema::kernel::EntityId>` is not implemented for `&mut <D as CollectionDelta>::Row`
    |                                                        |
    |                                                        required by a bound introduced by this call
    |
    = help: the following other types implement trait `dsl::Identified<TId>`:
              standards::v1::subsets::any::schema::kernel::TraceLink
              standards::v1::subsets::any::schema::registers::AccessRule
              standards::v1::subsets::any::schema::registers::AccessibilityRequirement
              standards::v1::subsets::any::schema::registers::Activity
              standards::v1::subsets::any::schema::registers::Adjacency
              standards::v1::subsets::any::schema::registers::AnalysisRecord
              standards::v1::subsets::any::schema::registers::ApprovalRecord
              standards::v1::subsets::any::schema::registers::ArtifactRecord
            and 60 others
note: required by a bound in `row_id`
   --> 📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️algebra/🦀️.rs:36:14
    |
 36 | fn row_id<R: Identified<EntityId>>(row: &R) -> &str {
    |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `row_id`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-architect-program/b68f3217c547b28f/out/semio_s_artifact_architect_program-b68f3217c547b28f.long-type-13583892229606276553.txt'
    = note: consider using `--verbose` to print the full type name to the console


error[E0277]: the trait bound `&<D as CollectionDelta>::Row: Identified<EntityId>` is not satisfied
   --> 📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️algebra/🦀️.rs:206:70
    |
206 |     added.extend(other.iter().filter(|row| !base_ids.contains(row_id(row))).cloned());
    |                                                               ------ ^^^ the trait `dsl::Identified<standards::v1::subsets::any::schema::kernel::EntityId>` is not implemented for `&<D as CollectionDelta>::Row`
    |                                                               |
    |                                                               required by a bound introduced by this call
    |
    = help: the following other types implement trait `dsl::Identified<TId>`:
              standards::v1::subsets::any::schema::kernel::TraceLink
              standards::v1::subsets::any::schema::registers::AccessRule
              standards::v1::subsets::any::schema::registers::AccessibilityRequirement
              standards::v1::subsets::any::schema::registers::Activity
              standards::v1::subsets::any::schema::registers::Adjacency
              standards::v1::subsets::any::schema::registers::AnalysisRecord
              standards::v1::subsets::any::schema::registers::ApprovalRecord
              standards::v1::subsets::any::schema::registers::ArtifactRecord
            and 60 others
note: required by a bound in `row_id`
   --> 📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️algebra/🦀️.rs:36:14
    |
 36 | fn row_id<R: Identified<EntityId>>(row: &R) -> &str {
    |              ^^^^^^^^^^^^^^^^^^^^ required by this bound in `row_id`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-s-artifact-architect-program/b68f3217c547b28f/out/semio_s_artifact_architect_program-b68f3217c547b28f.long-type-9556557490398312865.txt'
    = note: consider using `--verbose` to print the full type name to the console

[native:owner-command] running elapsedMs=430093
[cargo:build] running elapsedMs=330077
[artifact-rust:semio-s-artifact-architect-program:test] running elapsedMs=434016
[native:owner-command] running elapsedMs=440094
[cargo:build] running elapsedMs=340078
[artifact-rust:semio-s-artifact-architect-program:test] running elapsedMs=444017
```
