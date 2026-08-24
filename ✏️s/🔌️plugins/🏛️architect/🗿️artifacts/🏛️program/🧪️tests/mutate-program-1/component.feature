@capability-program-1-mutate
@no-oracle-architect-program-mutation-semantics
@comparison-ordered-json-v1
@mutations-program-1-any
Feature: Apply every typed architect program mutation to its committed specification vectors
  `s.architect.program` is a semio-NATIVE artifact: it is persisted as `.dsl.semio` text and
  `.pack.semio` binary through this subset's own codecs, and no third party reads or writes either.
  There is therefore no reference implementation to register as an oracle — recorded as the
  `architect-program-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed specification vectors and the inverse law. Because that decision is recorded, the runner
  dispatches NO oracle role for this case: every assertion below lives inside the subject handler,
  and a handler that merely ran the mutation and returned would report a pass having checked nothing.

  What distinguishes this subset is SCALE and the fact that its vocabulary is DERIVED rather than
  designed. `ProgramSnapshot` is an architectural brief: 66 registers holding stakeholders, users,
  activities, functions, requirements of sixteen different families, records, constraints and
  governance. `📓️derivation-rules.md` assigns each register's shape a verb set — a header-shaped
  id-keyed register yields create/delete/rename/replace (rule 2), the two EDGE-shaped registers
  `adjacencies` and `traces` yield connect/disconnect instead (rule 4), and the three document-level
  scalar facets `meta`, `project` and `governance` yield rename/replace (rule 1). 266 kinds come out
  of that, and the interesting property of the set is that it is CLOSED: there is no whole-document
  replace, because a whole-document replace is not an in-history mutation and goes through
  `ArtifactStore::reset` instead.

  📌️ 260 of the 266 committed vectors move the document. The six that do not are
  `delete`/`rename`/`replace` over `knowledge-record` and `benchmark-record`, and the reason is
  structural rather than an authoring gap: those two registers alone are composed
  `s.stdio.semio.table` CHILD handles whose rows live in a working-scene cache a fresh process has
  never populated, so the only branch reachable from a committed snapshot is the
  `mutation.target-missing` rejection — which is exactly what those six vectors pin, and what the
  `mutate` scenario asserts for them (declared status, declared code, empty diff). They are named in
  the adapter's `GUARD_VECTORS` list and exempted from the observability law on that basis; the
  other 260 kinds carry it with no exemption.

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json
    When <id> is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                                 | vector                                                                              |
      | create-information-requirement     | 🌱ℹ️create-information-requirement/🧪️tests/creates-information-requirement-a         |
      | delete-information-requirement     | 🗑️ℹ️delete-information-requirement/🧪️tests/deletes-information-requirement-a        |
      | rename-information-requirement     | ✏️ℹ️rename-information-requirement/🧪️tests/renames-information-requirement-a        |
      | replace-information-requirement    | 🔁ℹ️replace-information-requirement/🧪️tests/replaces-information-requirement-a       |
      | create-sustainability-requirement  | 🌱♻️create-sustainability-requirement/🧪️tests/creates-sustainability-requirement-a   |
      | delete-sustainability-requirement  | 🗑️♻️delete-sustainability-requirement/🧪️tests/deletes-sustainability-requirement-a  |
      | rename-sustainability-requirement  | ✏️♻️rename-sustainability-requirement/🧪️tests/renames-sustainability-requirement-a  |
      | replace-sustainability-requirement | 🔁♻️replace-sustainability-requirement/🧪️tests/replaces-sustainability-requirement-a |
      | create-accessibility-requirement   | 🌱♿create-accessibility-requirement/🧪️tests/creates-accessibility-requirement-a      |
      | delete-accessibility-requirement   | 🗑️♿delete-accessibility-requirement/🧪️tests/deletes-accessibility-requirement-a     |
      | rename-accessibility-requirement   | ✏️♿rename-accessibility-requirement/🧪️tests/renames-accessibility-requirement-a     |
      | replace-accessibility-requirement  | 🔁♿replace-accessibility-requirement/🧪️tests/replaces-accessibility-requirement-a    |
      | create-conflict                    | 🌱⚔️create-conflict/🧪️tests/creates-conflict-a                                       |
      | delete-conflict                    | 🗑️⚔️delete-conflict/🧪️tests/deletes-conflict-a                                      |
      | rename-conflict                    | ✏️⚔️rename-conflict/🧪️tests/renames-conflict-a                                      |
      | replace-conflict                   | 🔁⚔️replace-conflict/🧪️tests/replaces-conflict-a                                     |
      | create-option-evaluation           | 🌱⚖️create-option-evaluation/🧪️tests/creates-option-evaluation-a                     |
      | delete-option-evaluation           | 🗑️⚖️delete-option-evaluation/🧪️tests/deletes-option-evaluation-a                    |
      | rename-option-evaluation           | ✏️⚖️rename-option-evaluation/🧪️tests/renames-option-evaluation-a                    |
      | replace-option-evaluation          | 🔁⚖️replace-option-evaluation/🧪️tests/replaces-option-evaluation-a                   |
      | create-function                    | 🌱⚙️create-function/🧪️tests/creates-function-a                                       |
      | delete-function                    | 🗑️⚙️delete-function/🧪️tests/deletes-function-a                                      |
      | rename-function                    | ✏️⚙️rename-function/🧪️tests/renames-function-a                                      |
      | replace-function                   | 🔁⚙️replace-function/🧪️tests/replaces-function-a                                     |
      | create-risk                        | 🌱⚠️create-risk/🧪️tests/creates-risk-a                                               |
      | delete-risk                        | 🗑️⚠️delete-risk/🧪️tests/deletes-risk-a                                              |
      | rename-risk                        | ✏️⚠️rename-risk/🧪️tests/renames-risk-a                                              |
      | replace-risk                       | 🔁⚠️replace-risk/🧪️tests/replaces-risk-a                                             |
      | create-decision                    | 🌱✅create-decision/🧪️tests/creates-decision-a                                        |
      | delete-decision                    | 🗑️✅delete-decision/🧪️tests/deletes-decision-a                                       |
      | rename-decision                    | ✏️✅rename-decision/🧪️tests/renames-decision-a                                       |
      | replace-decision                   | 🔁✅replace-decision/🧪️tests/replaces-decision-a                                      |
      | create-validation-record           | 🌱✔️create-validation-record/🧪️tests/creates-validation-record-a                     |
      | delete-validation-record           | 🗑️✔️delete-validation-record/🧪️tests/deletes-validation-record-a                    |
      | rename-validation-record           | ✏️✔️rename-validation-record/🧪️tests/renames-validation-record-a                    |
      | replace-validation-record          | 🔁✔️replace-validation-record/🧪️tests/replaces-validation-record-a                   |
      | create-priority-record             | 🌱⭐create-priority-record/🧪️tests/creates-priority-record-a                          |
      | delete-priority-record             | 🗑️⭐delete-priority-record/🧪️tests/deletes-priority-record-a                         |
      | rename-priority-record             | ✏️⭐rename-priority-record/🧪️tests/renames-priority-record-a                         |
      | replace-priority-record            | 🔁⭐replace-priority-record/🧪️tests/replaces-priority-record-a                        |
      | create-flow-requirement            | 🌱🌊create-flow-requirement/🧪️tests/creates-flow-requirement-a                        |
      | delete-flow-requirement            | 🗑️🌊delete-flow-requirement/🧪️tests/deletes-flow-requirement-a                       |
      | rename-flow-requirement            | ✏️🌊rename-flow-requirement/🧪️tests/renames-flow-requirement-a                       |
      | replace-flow-requirement           | 🔁🌊replace-flow-requirement/🧪️tests/replaces-flow-requirement-a                      |
      | create-environmental-requirement   | 🌱🌿create-environmental-requirement/🧪️tests/creates-environmental-requirement-a      |
      | delete-environmental-requirement   | 🗑️🌿delete-environmental-requirement/🧪️tests/deletes-environmental-requirement-a     |
      | rename-environmental-requirement   | ✏️🌿rename-environmental-requirement/🧪️tests/renames-environmental-requirement-a     |
      | replace-environmental-requirement  | 🔁🌿replace-environmental-requirement/🧪️tests/replaces-environmental-requirement-a    |
      | create-workshop                    | 🌱🎓create-workshop/🧪️tests/creates-workshop-a                                        |
      | delete-workshop                    | 🗑️🎓delete-workshop/🧪️tests/deletes-workshop-a                                       |
      | rename-workshop                    | ✏️🎓rename-workshop/🧪️tests/renames-workshop-a                                       |
      | replace-workshop                   | 🔁🎓replace-workshop/🧪️tests/replaces-workshop-a                                      |
      | create-scenario                    | 🌱🎬create-scenario/🧪️tests/creates-scenario-a                                        |
      | delete-scenario                    | 🗑️🎬delete-scenario/🧪️tests/deletes-scenario-a                                       |
      | rename-scenario                    | ✏️🎬rename-scenario/🧪️tests/renames-scenario-a                                       |
      | replace-scenario                   | 🔁🎬replace-scenario/🧪️tests/replaces-scenario-a                                      |
      | create-benchmark-record            | 🌱🏁create-benchmark-record/🧪️tests/creates-benchmark-record-a                        |
      | delete-benchmark-record            | 🗑️🏁delete-benchmark-record/🧪️tests/rejects-deleting-absent-benchmark-record-a       |
      | rename-benchmark-record            | ✏️🏁rename-benchmark-record/🧪️tests/rejects-renaming-absent-benchmark-record-a       |
      | replace-benchmark-record           | 🔁🏁replace-benchmark-record/🧪️tests/rejects-replacing-absent-benchmark-record-a      |
      | create-activity                    | 🌱🏃create-activity/🧪️tests/creates-activity-a                                        |
      | delete-activity                    | 🗑️🏃delete-activity/🧪️tests/deletes-activity-a                                       |
      | rename-activity                    | ✏️🏃rename-activity/🧪️tests/renames-activity-a                                       |
      | replace-activity                   | 🔁🏃replace-activity/🧪️tests/replaces-activity-a                                      |
      | create-infrastructure-requirement  | 🌱🏗️create-infrastructure-requirement/🧪️tests/creates-infrastructure-requirement-a   |
      | delete-infrastructure-requirement  | 🗑️🏗️delete-infrastructure-requirement/🧪️tests/deletes-infrastructure-requirement-a  |
      | rename-infrastructure-requirement  | ✏️🏗️rename-infrastructure-requirement/🧪️tests/renames-infrastructure-requirement-a  |
      | replace-infrastructure-requirement | 🔁🏗️replace-infrastructure-requirement/🧪️tests/replaces-infrastructure-requirement-a |
      | create-organizational-requirement  | 🌱🏢create-organizational-requirement/🧪️tests/creates-organizational-requirement-a    |
      | delete-organizational-requirement  | 🗑️🏢delete-organizational-requirement/🧪️tests/deletes-organizational-requirement-a   |
      | rename-organizational-requirement  | ✏️🏢rename-organizational-requirement/🧪️tests/renames-organizational-requirement-a   |
      | replace-organizational-requirement | 🔁🏢replace-organizational-requirement/🧪️tests/replaces-organizational-requirement-a  |
      | create-issue                       | 🌱🐛create-issue/🧪️tests/creates-issue-a                                              |
      | delete-issue                       | 🗑️🐛delete-issue/🧪️tests/deletes-issue-a                                             |
      | rename-issue                       | ✏️🐛rename-issue/🧪️tests/renames-issue-a                                             |
      | replace-issue                      | 🔁🐛replace-issue/🧪️tests/replaces-issue-a                                            |
      | create-approval-record             | 🌱👍create-approval-record/🧪️tests/creates-approval-record-a                          |
      | delete-approval-record             | 🗑️👍delete-approval-record/🧪️tests/deletes-approval-record-a                         |
      | rename-approval-record             | ✏️👍rename-approval-record/🧪️tests/renames-approval-record-a                         |
      | replace-approval-record            | 🔁👍replace-approval-record/🧪️tests/replaces-approval-record-a                        |
      | create-stakeholder                 | 🌱👥create-stakeholder/🧪️tests/creates-stakeholder-a                                  |
      | delete-stakeholder                 | 🗑️👥delete-stakeholder/🧪️tests/deletes-stakeholder-a                                 |
      | rename-stakeholder                 | ✏️👥rename-stakeholder/🧪️tests/renames-stakeholder-a                                 |
      | replace-stakeholder                | 🔁👥replace-stakeholder/🧪️tests/replaces-stakeholder-a                                |
      | create-quality-record              | 🌱💎create-quality-record/🧪️tests/creates-quality-record-a                            |
      | delete-quality-record              | 🗑️💎delete-quality-record/🧪️tests/deletes-quality-record-a                           |
      | rename-quality-record              | ✏️💎rename-quality-record/🧪️tests/renames-quality-record-a                           |
      | replace-quality-record             | 🔁💎replace-quality-record/🧪️tests/replaces-quality-record-a                          |
      | create-resilience-requirement      | 🌱💪create-resilience-requirement/🧪️tests/creates-resilience-requirement-a            |
      | delete-resilience-requirement      | 🗑️💪delete-resilience-requirement/🧪️tests/deletes-resilience-requirement-a           |
      | rename-resilience-requirement      | ✏️💪rename-resilience-requirement/🧪️tests/renames-resilience-requirement-a           |
      | replace-resilience-requirement     | 🔁💪replace-resilience-requirement/🧪️tests/replaces-resilience-requirement-a          |
      | create-assumption                  | 🌱💭create-assumption/🧪️tests/creates-assumption-a                                    |
      | delete-assumption                  | 🗑️💭delete-assumption/🧪️tests/deletes-assumption-a                                   |
      | rename-assumption                  | ✏️💭rename-assumption/🧪️tests/renames-assumption-a                                   |
      | replace-assumption                 | 🔁💭replace-assumption/🧪️tests/replaces-assumption-a                                  |
      | create-cost-requirement            | 🌱💰create-cost-requirement/🧪️tests/creates-cost-requirement-a                        |
      | delete-cost-requirement            | 🗑️💰delete-cost-requirement/🧪️tests/deletes-cost-requirement-a                       |
      | rename-cost-requirement            | ✏️💰rename-cost-requirement/🧪️tests/renames-cost-requirement-a                       |
      | replace-cost-requirement           | 🔁💰replace-cost-requirement/🧪️tests/replaces-cost-requirement-a                      |
      | create-document                    | 🌱📄create-document/🧪️tests/creates-document-a                                        |
      | delete-document                    | 🗑️📄delete-document/🧪️tests/deletes-document-a                                       |
      | rename-document                    | ✏️📄rename-document/🧪️tests/renames-document-a                                       |
      | replace-document                   | 🔁📄replace-document/🧪️tests/replaces-document-a                                      |
      | create-schedule-requirement        | 🌱📅create-schedule-requirement/🧪️tests/creates-schedule-requirement-a                |
      | delete-schedule-requirement        | 🗑️📅delete-schedule-requirement/🧪️tests/deletes-schedule-requirement-a               |
      | rename-schedule-requirement        | ✏️📅rename-schedule-requirement/🧪️tests/renames-schedule-requirement-a               |
      | replace-schedule-requirement       | 🔁📅replace-schedule-requirement/🧪️tests/replaces-schedule-requirement-a              |
      | create-growth-plan                 | 🌱📈create-growth-plan/🧪️tests/creates-growth-plan-a                                  |
      | delete-growth-plan                 | 🗑️📈delete-growth-plan/🧪️tests/deletes-growth-plan-a                                 |
      | rename-growth-plan                 | ✏️📈rename-growth-plan/🧪️tests/renames-growth-plan-a                                 |
      | replace-growth-plan                | 🔁📈replace-growth-plan/🧪️tests/replaces-growth-plan-a                                |
      | create-performance-criterion       | 🌱📊create-performance-criterion/🧪️tests/creates-performance-criterion-a              |
      | delete-performance-criterion       | 🗑️📊delete-performance-criterion/🧪️tests/deletes-performance-criterion-a             |
      | rename-performance-criterion       | ✏️📊rename-performance-criterion/🧪️tests/renames-performance-criterion-a             |
      | replace-performance-criterion      | 🔁📊replace-performance-criterion/🧪️tests/replaces-performance-criterion-a            |
      | create-operational-requirement     | 🌱📋create-operational-requirement/🧪️tests/creates-operational-requirement-a          |
      | delete-operational-requirement     | 🗑️📋delete-operational-requirement/🧪️tests/deletes-operational-requirement-a         |
      | rename-operational-requirement     | ✏️📋rename-operational-requirement/🧪️tests/renames-operational-requirement-a         |
      | replace-operational-requirement    | 🔁📋replace-operational-requirement/🧪️tests/replaces-operational-requirement-a        |
      | create-requirement                 | 🌱📌create-requirement/🧪️tests/creates-requirement-a                                  |
      | delete-requirement                 | 🗑️📌delete-requirement/🧪️tests/deletes-requirement-a                                 |
      | rename-requirement                 | ✏️📌rename-requirement/🧪️tests/renames-requirement-a                                 |
      | replace-requirement                | 🔁📌replace-requirement/🧪️tests/replaces-requirement-a                                |
      | create-site-context                | 🌱📍create-site-context/🧪️tests/creates-site-context-a                                |
      | delete-site-context                | 🗑️📍delete-site-context/🧪️tests/deletes-site-context-a                               |
      | rename-site-context                | ✏️📍rename-site-context/🧪️tests/renames-site-context-a                               |
      | replace-site-context               | 🔁📍replace-site-context/🧪️tests/replaces-site-context-a                              |
      | create-template-record             | 🌱📐create-template-record/🧪️tests/creates-template-record-a                          |
      | delete-template-record             | 🗑️📐delete-template-record/🧪️tests/deletes-template-record-a                         |
      | rename-template-record             | ✏️📐rename-template-record/🧪️tests/renames-template-record-a                         |
      | replace-template-record            | 🔁📐replace-template-record/🧪️tests/replaces-template-record-a                        |
      | create-report-record               | 🌱📑create-report-record/🧪️tests/creates-report-record-a                              |
      | delete-report-record               | 🗑️📑delete-report-record/🧪️tests/deletes-report-record-a                             |
      | rename-report-record               | ✏️📑rename-report-record/🧪️tests/renames-report-record-a                             |
      | replace-report-record              | 🔁📑replace-report-record/🧪️tests/replaces-report-record-a                            |
      | create-audit-event                 | 🌱📒create-audit-event/🧪️tests/creates-audit-event-a                                  |
      | delete-audit-event                 | 🗑️📒delete-audit-event/🧪️tests/deletes-audit-event-a                                 |
      | rename-audit-event                 | ✏️📒rename-audit-event/🧪️tests/renames-audit-event-a                                 |
      | replace-audit-event                | 🔁📒replace-audit-event/🧪️tests/replaces-audit-event-a                                |
      | create-knowledge-record            | 🌱📚create-knowledge-record/🧪️tests/creates-knowledge-record-a                        |
      | delete-knowledge-record            | 🗑️📚delete-knowledge-record/🧪️tests/rejects-deleting-absent-knowledge-record-a       |
      | rename-knowledge-record            | ✏️📚rename-knowledge-record/🧪️tests/rejects-renaming-absent-knowledge-record-a       |
      | replace-knowledge-record           | 🔁📚replace-knowledge-record/🧪️tests/rejects-replacing-absent-knowledge-record-a      |
      | create-regulatory-requirement      | 🌱📜create-regulatory-requirement/🧪️tests/creates-regulatory-requirement-a            |
      | delete-regulatory-requirement      | 🗑️📜delete-regulatory-requirement/🧪️tests/deletes-regulatory-requirement-a           |
      | rename-regulatory-requirement      | ✏️📜rename-regulatory-requirement/🧪️tests/renames-regulatory-requirement-a           |
      | replace-regulatory-requirement     | 🔁📜replace-regulatory-requirement/🧪️tests/replaces-regulatory-requirement-a          |
      | create-change-record               | 🌱📝create-change-record/🧪️tests/creates-change-record-a                              |
      | delete-change-record               | 🗑️📝delete-change-record/🧪️tests/deletes-change-record-a                             |
      | rename-change-record               | ✏️📝rename-change-record/🧪️tests/renames-change-record-a                             |
      | replace-change-record              | 🔁📝replace-change-record/🧪️tests/replaces-change-record-a                            |
      | create-communication-requirement   | 🌱📡create-communication-requirement/🧪️tests/creates-communication-requirement-a      |
      | delete-communication-requirement   | 🗑️📡delete-communication-requirement/🧪️tests/deletes-communication-requirement-a     |
      | rename-communication-requirement   | ✏️📡rename-communication-requirement/🧪️tests/renames-communication-requirement-a     |
      | replace-communication-requirement  | 🔁📡replace-communication-requirement/🧪️tests/replaces-communication-requirement-a    |
      | create-resource                    | 🌱📦create-resource/🧪️tests/creates-resource-a                                        |
      | delete-resource                    | 🗑️📦delete-resource/🧪️tests/deletes-resource-a                                       |
      | rename-resource                    | ✏️📦rename-resource/🧪️tests/renames-resource-a                                       |
      | replace-resource                   | 🔁📦replace-resource/🧪️tests/replaces-resource-a                                      |
      | create-status-record               | 🌱📶create-status-record/🧪️tests/creates-status-record-a                              |
      | delete-status-record               | 🗑️📶delete-status-record/🧪️tests/deletes-status-record-a                             |
      | rename-status-record               | ✏️📶rename-status-record/🧪️tests/renames-status-record-a                             |
      | replace-status-record              | 🔁📶replace-status-record/🧪️tests/replaces-status-record-a                            |
      | create-process                     | 🌱🔄create-process/🧪️tests/creates-process-a                                          |
      | delete-process                     | 🗑️🔄delete-process/🧪️tests/deletes-process-a                                         |
      | rename-process                     | ✏️🔄rename-process/🧪️tests/renames-process-a                                         |
      | replace-process                    | 🔁🔄replace-process/🧪️tests/replaces-process-a                                        |
      | create-search-filter               | 🌱🔍create-search-filter/🧪️tests/creates-search-filter-a                              |
      | delete-search-filter               | 🗑️🔍delete-search-filter/🧪️tests/deletes-search-filter-a                             |
      | rename-search-filter               | ✏️🔍rename-search-filter/🧪️tests/renames-search-filter-a                             |
      | replace-search-filter              | 🔁🔍replace-search-filter/🧪️tests/replaces-search-filter-a                            |
      | create-access-rule                 | 🌱🔑create-access-rule/🧪️tests/creates-access-rule-a                                  |
      | delete-access-rule                 | 🗑️🔑delete-access-rule/🧪️tests/deletes-access-rule-a                                 |
      | rename-access-rule                 | ✏️🔑rename-access-rule/🧪️tests/renames-access-rule-a                                 |
      | replace-access-rule                | 🔁🔑replace-access-rule/🧪️tests/replaces-access-rule-a                                |
      | create-privacy-requirement         | 🌱🔒create-privacy-requirement/🧪️tests/creates-privacy-requirement-a                  |
      | delete-privacy-requirement         | 🗑️🔒delete-privacy-requirement/🧪️tests/deletes-privacy-requirement-a                 |
      | rename-privacy-requirement         | ✏️🔒rename-privacy-requirement/🧪️tests/renames-privacy-requirement-a                 |
      | replace-privacy-requirement        | 🔁🔒replace-privacy-requirement/🧪️tests/replaces-privacy-requirement-a                |
      | create-relationship                | 🌱🔗create-relationship/🧪️tests/creates-relationship-a                                |
      | delete-relationship                | 🗑️🔗delete-relationship/🧪️tests/deletes-relationship-a                               |
      | rename-relationship                | ✏️🔗rename-relationship/🧪️tests/renames-relationship-a                               |
      | replace-relationship               | 🔁🔗replace-relationship/🧪️tests/replaces-relationship-a                              |
      | create-quantity-requirement        | 🌱🔢create-quantity-requirement/🧪️tests/creates-quantity-requirement-a                |
      | delete-quantity-requirement        | 🗑️🔢delete-quantity-requirement/🧪️tests/deletes-quantity-requirement-a               |
      | rename-quantity-requirement        | ✏️🔢rename-quantity-requirement/🧪️tests/renames-quantity-requirement-a               |
      | replace-quantity-requirement       | 🔁🔢replace-quantity-requirement/🧪️tests/replaces-quantity-requirement-a              |
      | create-analysis-record             | 🌱🔬create-analysis-record/🧪️tests/creates-analysis-record-a                          |
      | delete-analysis-record             | 🗑️🔬delete-analysis-record/🧪️tests/deletes-analysis-record-a                         |
      | rename-analysis-record             | ✏️🔬rename-analysis-record/🧪️tests/renames-analysis-record-a                         |
      | replace-analysis-record            | 🔁🔬replace-analysis-record/🧪️tests/replaces-analysis-record-a                        |
      | create-storage-requirement         | 🌱🗄️create-storage-requirement/🧪️tests/creates-storage-requirement-a                 |
      | delete-storage-requirement         | 🗑️🗄️delete-storage-requirement/🧪️tests/deletes-storage-requirement-a                |
      | rename-storage-requirement         | ✏️🗄️rename-storage-requirement/🧪️tests/renames-storage-requirement-a                |
      | replace-storage-requirement        | 🔁🗄️replace-storage-requirement/🧪️tests/replaces-storage-requirement-a               |
      | create-meeting-record              | 🌱🗓️create-meeting-record/🧪️tests/creates-meeting-record-a                           |
      | delete-meeting-record              | 🗑️🗓️delete-meeting-record/🧪️tests/deletes-meeting-record-a                          |
      | rename-meeting-record              | ✏️🗓️rename-meeting-record/🧪️tests/renames-meeting-record-a                          |
      | replace-meeting-record             | 🔁🗓️replace-meeting-record/🧪️tests/replaces-meeting-record-a                         |
      | create-survey                      | 🌱🗳️create-survey/🧪️tests/creates-survey-a                                           |
      | delete-survey                      | 🗑️🗳️delete-survey/🧪️tests/deletes-survey-a                                          |
      | rename-survey                      | ✏️🗳️rename-survey/🧪️tests/renames-survey-a                                          |
      | replace-survey                     | 🔁🗳️replace-survey/🧪️tests/replaces-survey-a                                         |
      | create-delivery-constraint         | 🌱🚚create-delivery-constraint/🧪️tests/creates-delivery-constraint-a                  |
      | delete-delivery-constraint         | 🗑️🚚delete-delivery-constraint/🧪️tests/deletes-delivery-constraint-a                 |
      | rename-delivery-constraint         | ✏️🚚rename-delivery-constraint/🧪️tests/renames-delivery-constraint-a                 |
      | replace-delivery-constraint        | 🔁🚚replace-delivery-constraint/🧪️tests/replaces-delivery-constraint-a                |
      | create-constraint-record           | 🌱🚧create-constraint-record/🧪️tests/creates-constraint-record-a                      |
      | delete-constraint-record           | 🗑️🚧delete-constraint-record/🧪️tests/deletes-constraint-record-a                     |
      | rename-constraint-record           | ✏️🚧rename-constraint-record/🧪️tests/renames-constraint-record-a                     |
      | replace-constraint-record          | 🔁🚧replace-constraint-record/🧪️tests/replaces-constraint-record-a                    |
      | create-compliance-record           | 🌱🛂create-compliance-record/🧪️tests/creates-compliance-record-a                      |
      | delete-compliance-record           | 🗑️🛂delete-compliance-record/🧪️tests/deletes-compliance-record-a                     |
      | rename-compliance-record           | ✏️🛂rename-compliance-record/🧪️tests/renames-compliance-record-a                     |
      | replace-compliance-record          | 🔁🛂replace-compliance-record/🧪️tests/replaces-compliance-record-a                    |
      | create-service-requirement         | 🌱🛎️create-service-requirement/🧪️tests/creates-service-requirement-a                 |
      | delete-service-requirement         | 🗑️🛎️delete-service-requirement/🧪️tests/deletes-service-requirement-a                |
      | rename-service-requirement         | ✏️🛎️rename-service-requirement/🧪️tests/renames-service-requirement-a                |
      | replace-service-requirement        | 🔁🛎️replace-service-requirement/🧪️tests/replaces-service-requirement-a               |
      | create-equipment                   | 🌱🛠️create-equipment/🧪️tests/creates-equipment-a                                     |
      | delete-equipment                   | 🗑️🛠️delete-equipment/🧪️tests/deletes-equipment-a                                    |
      | rename-equipment                   | ✏️🛠️rename-equipment/🧪️tests/renames-equipment-a                                    |
      | replace-equipment                  | 🔁🛠️replace-equipment/🧪️tests/replaces-equipment-a                                   |
      | create-security-requirement        | 🌱🛡️create-security-requirement/🧪️tests/creates-security-requirement-a               |
      | delete-security-requirement        | 🗑️🛡️delete-security-requirement/🧪️tests/deletes-security-requirement-a              |
      | rename-security-requirement        | ✏️🛡️rename-security-requirement/🧪️tests/renames-security-requirement-a              |
      | replace-security-requirement       | 🔁🛡️replace-security-requirement/🧪️tests/replaces-security-requirement-a             |
      | create-collaboration-record        | 🌱🤝create-collaboration-record/🧪️tests/creates-collaboration-record-a                |
      | delete-collaboration-record        | 🗑️🤝delete-collaboration-record/🧪️tests/deletes-collaboration-record-a               |
      | rename-collaboration-record        | ✏️🤝rename-collaboration-record/🧪️tests/renames-collaboration-record-a               |
      | replace-collaboration-record       | 🔁🤝replace-collaboration-record/🧪️tests/replaces-collaboration-record-a              |
      | create-safety-requirement          | 🌱🦺create-safety-requirement/🧪️tests/creates-safety-requirement-a                    |
      | delete-safety-requirement          | 🗑️🦺delete-safety-requirement/🧪️tests/deletes-safety-requirement-a                   |
      | rename-safety-requirement          | ✏️🦺rename-safety-requirement/🧪️tests/renames-safety-requirement-a                   |
      | replace-safety-requirement         | 🔁🦺replace-safety-requirement/🧪️tests/replaces-safety-requirement-a                  |
      | create-user-profile                | 🌱🧑create-user-profile/🧪️tests/creates-user-profile-a                                |
      | delete-user-profile                | 🗑️🧑delete-user-profile/🧪️tests/deletes-user-profile-a                               |
      | rename-user-profile                | ✏️🧑rename-user-profile/🧪️tests/renames-user-profile-a                               |
      | replace-user-profile               | 🔁🧑replace-user-profile/🧪️tests/replaces-user-profile-a                              |
      | create-human-factor-requirement    | 🌱🧠create-human-factor-requirement/🧪️tests/creates-human-factor-requirement-a        |
      | delete-human-factor-requirement    | 🗑️🧠delete-human-factor-requirement/🧪️tests/deletes-human-factor-requirement-a       |
      | rename-human-factor-requirement    | ✏️🧠rename-human-factor-requirement/🧪️tests/renames-human-factor-requirement-a       |
      | replace-human-factor-requirement   | 🔁🧠replace-human-factor-requirement/🧪️tests/replaces-human-factor-requirement-a      |
      | create-flexibility-requirement     | 🌱🧩create-flexibility-requirement/🧪️tests/creates-flexibility-requirement-a          |
      | delete-flexibility-requirement     | 🗑️🧩delete-flexibility-requirement/🧪️tests/deletes-flexibility-requirement-a         |
      | rename-flexibility-requirement     | ✏️🧩rename-flexibility-requirement/🧪️tests/renames-flexibility-requirement-a         |
      | replace-flexibility-requirement    | 🔁🧩replace-flexibility-requirement/🧪️tests/replaces-flexibility-requirement-a        |
      | create-wayfinding-requirement      | 🌱🧭create-wayfinding-requirement/🧪️tests/creates-wayfinding-requirement-a            |
      | delete-wayfinding-requirement      | 🗑️🧭delete-wayfinding-requirement/🧪️tests/deletes-wayfinding-requirement-a           |
      | rename-wayfinding-requirement      | ✏️🧭rename-wayfinding-requirement/🧪️tests/renames-wayfinding-requirement-a           |
      | replace-wayfinding-requirement     | 🔁🧭replace-wayfinding-requirement/🧪️tests/replaces-wayfinding-requirement-a          |
      | create-program-element             | 🌱🧱create-program-element/🧪️tests/creates-program-element-a                          |
      | delete-program-element             | 🗑️🧱delete-program-element/🧪️tests/deletes-program-element-a                         |
      | rename-program-element             | ✏️🧱rename-program-element/🧪️tests/renames-program-element-a                         |
      | replace-program-element            | 🔁🧱replace-program-element/🧪️tests/replaces-program-element-a                        |
      | connect-adjacency                  | 🔗🧲connect-adjacency/🧪️tests/connects-reception-to-waiting                           |
      | disconnect-adjacency               | ✂️🧲disconnect-adjacency/🧪️tests/disconnects-reception-from-waiting                  |
      | connect-trace                      | 🔗🧵connect-trace/🧪️tests/connects-requirement-a-to-decision-a                        |
      | disconnect-trace                   | ✂️🧵disconnect-trace/🧪️tests/disconnects-requirement-a-from-decision-a               |
      | rename-meta                        | ✏️🏷️rename-meta/🧪️tests/renames-the-document-title                                  |
      | replace-meta                       | 🔁🏷️replace-meta/🧪️tests/replaces-the-document-meta-block                            |
      | rename-project                     | ✏️📁rename-project/🧪️tests/renames-the-project-code                                  |
      | replace-project                    | 🔁📁replace-project/🧪️tests/replaces-the-project-definition                           |
      | rename-governance                  | ✏️🏛️rename-governance/🧪️tests/renames-the-governance-framework                      |
      | replace-governance                 | 🔁🏛️replace-governance/🧪️tests/replaces-the-governance-block                         |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                                 | vector                                                                              |
      | create-information-requirement     | 🌱ℹ️create-information-requirement/🧪️tests/creates-information-requirement-a         |
      | delete-information-requirement     | 🗑️ℹ️delete-information-requirement/🧪️tests/deletes-information-requirement-a        |
      | rename-information-requirement     | ✏️ℹ️rename-information-requirement/🧪️tests/renames-information-requirement-a        |
      | replace-information-requirement    | 🔁ℹ️replace-information-requirement/🧪️tests/replaces-information-requirement-a       |
      | create-sustainability-requirement  | 🌱♻️create-sustainability-requirement/🧪️tests/creates-sustainability-requirement-a   |
      | delete-sustainability-requirement  | 🗑️♻️delete-sustainability-requirement/🧪️tests/deletes-sustainability-requirement-a  |
      | rename-sustainability-requirement  | ✏️♻️rename-sustainability-requirement/🧪️tests/renames-sustainability-requirement-a  |
      | replace-sustainability-requirement | 🔁♻️replace-sustainability-requirement/🧪️tests/replaces-sustainability-requirement-a |
      | create-accessibility-requirement   | 🌱♿create-accessibility-requirement/🧪️tests/creates-accessibility-requirement-a      |
      | delete-accessibility-requirement   | 🗑️♿delete-accessibility-requirement/🧪️tests/deletes-accessibility-requirement-a     |
      | rename-accessibility-requirement   | ✏️♿rename-accessibility-requirement/🧪️tests/renames-accessibility-requirement-a     |
      | replace-accessibility-requirement  | 🔁♿replace-accessibility-requirement/🧪️tests/replaces-accessibility-requirement-a    |
      | create-conflict                    | 🌱⚔️create-conflict/🧪️tests/creates-conflict-a                                       |
      | delete-conflict                    | 🗑️⚔️delete-conflict/🧪️tests/deletes-conflict-a                                      |
      | rename-conflict                    | ✏️⚔️rename-conflict/🧪️tests/renames-conflict-a                                      |
      | replace-conflict                   | 🔁⚔️replace-conflict/🧪️tests/replaces-conflict-a                                     |
      | create-option-evaluation           | 🌱⚖️create-option-evaluation/🧪️tests/creates-option-evaluation-a                     |
      | delete-option-evaluation           | 🗑️⚖️delete-option-evaluation/🧪️tests/deletes-option-evaluation-a                    |
      | rename-option-evaluation           | ✏️⚖️rename-option-evaluation/🧪️tests/renames-option-evaluation-a                    |
      | replace-option-evaluation          | 🔁⚖️replace-option-evaluation/🧪️tests/replaces-option-evaluation-a                   |
      | create-function                    | 🌱⚙️create-function/🧪️tests/creates-function-a                                       |
      | delete-function                    | 🗑️⚙️delete-function/🧪️tests/deletes-function-a                                      |
      | rename-function                    | ✏️⚙️rename-function/🧪️tests/renames-function-a                                      |
      | replace-function                   | 🔁⚙️replace-function/🧪️tests/replaces-function-a                                     |
      | create-risk                        | 🌱⚠️create-risk/🧪️tests/creates-risk-a                                               |
      | delete-risk                        | 🗑️⚠️delete-risk/🧪️tests/deletes-risk-a                                              |
      | rename-risk                        | ✏️⚠️rename-risk/🧪️tests/renames-risk-a                                              |
      | replace-risk                       | 🔁⚠️replace-risk/🧪️tests/replaces-risk-a                                             |
      | create-decision                    | 🌱✅create-decision/🧪️tests/creates-decision-a                                        |
      | delete-decision                    | 🗑️✅delete-decision/🧪️tests/deletes-decision-a                                       |
      | rename-decision                    | ✏️✅rename-decision/🧪️tests/renames-decision-a                                       |
      | replace-decision                   | 🔁✅replace-decision/🧪️tests/replaces-decision-a                                      |
      | create-validation-record           | 🌱✔️create-validation-record/🧪️tests/creates-validation-record-a                     |
      | delete-validation-record           | 🗑️✔️delete-validation-record/🧪️tests/deletes-validation-record-a                    |
      | rename-validation-record           | ✏️✔️rename-validation-record/🧪️tests/renames-validation-record-a                    |
      | replace-validation-record          | 🔁✔️replace-validation-record/🧪️tests/replaces-validation-record-a                   |
      | create-priority-record             | 🌱⭐create-priority-record/🧪️tests/creates-priority-record-a                          |
      | delete-priority-record             | 🗑️⭐delete-priority-record/🧪️tests/deletes-priority-record-a                         |
      | rename-priority-record             | ✏️⭐rename-priority-record/🧪️tests/renames-priority-record-a                         |
      | replace-priority-record            | 🔁⭐replace-priority-record/🧪️tests/replaces-priority-record-a                        |
      | create-flow-requirement            | 🌱🌊create-flow-requirement/🧪️tests/creates-flow-requirement-a                        |
      | delete-flow-requirement            | 🗑️🌊delete-flow-requirement/🧪️tests/deletes-flow-requirement-a                       |
      | rename-flow-requirement            | ✏️🌊rename-flow-requirement/🧪️tests/renames-flow-requirement-a                       |
      | replace-flow-requirement           | 🔁🌊replace-flow-requirement/🧪️tests/replaces-flow-requirement-a                      |
      | create-environmental-requirement   | 🌱🌿create-environmental-requirement/🧪️tests/creates-environmental-requirement-a      |
      | delete-environmental-requirement   | 🗑️🌿delete-environmental-requirement/🧪️tests/deletes-environmental-requirement-a     |
      | rename-environmental-requirement   | ✏️🌿rename-environmental-requirement/🧪️tests/renames-environmental-requirement-a     |
      | replace-environmental-requirement  | 🔁🌿replace-environmental-requirement/🧪️tests/replaces-environmental-requirement-a    |
      | create-workshop                    | 🌱🎓create-workshop/🧪️tests/creates-workshop-a                                        |
      | delete-workshop                    | 🗑️🎓delete-workshop/🧪️tests/deletes-workshop-a                                       |
      | rename-workshop                    | ✏️🎓rename-workshop/🧪️tests/renames-workshop-a                                       |
      | replace-workshop                   | 🔁🎓replace-workshop/🧪️tests/replaces-workshop-a                                      |
      | create-scenario                    | 🌱🎬create-scenario/🧪️tests/creates-scenario-a                                        |
      | delete-scenario                    | 🗑️🎬delete-scenario/🧪️tests/deletes-scenario-a                                       |
      | rename-scenario                    | ✏️🎬rename-scenario/🧪️tests/renames-scenario-a                                       |
      | replace-scenario                   | 🔁🎬replace-scenario/🧪️tests/replaces-scenario-a                                      |
      | create-benchmark-record            | 🌱🏁create-benchmark-record/🧪️tests/creates-benchmark-record-a                        |
      | delete-benchmark-record            | 🗑️🏁delete-benchmark-record/🧪️tests/rejects-deleting-absent-benchmark-record-a       |
      | rename-benchmark-record            | ✏️🏁rename-benchmark-record/🧪️tests/rejects-renaming-absent-benchmark-record-a       |
      | replace-benchmark-record           | 🔁🏁replace-benchmark-record/🧪️tests/rejects-replacing-absent-benchmark-record-a      |
      | create-activity                    | 🌱🏃create-activity/🧪️tests/creates-activity-a                                        |
      | delete-activity                    | 🗑️🏃delete-activity/🧪️tests/deletes-activity-a                                       |
      | rename-activity                    | ✏️🏃rename-activity/🧪️tests/renames-activity-a                                       |
      | replace-activity                   | 🔁🏃replace-activity/🧪️tests/replaces-activity-a                                      |
      | create-infrastructure-requirement  | 🌱🏗️create-infrastructure-requirement/🧪️tests/creates-infrastructure-requirement-a   |
      | delete-infrastructure-requirement  | 🗑️🏗️delete-infrastructure-requirement/🧪️tests/deletes-infrastructure-requirement-a  |
      | rename-infrastructure-requirement  | ✏️🏗️rename-infrastructure-requirement/🧪️tests/renames-infrastructure-requirement-a  |
      | replace-infrastructure-requirement | 🔁🏗️replace-infrastructure-requirement/🧪️tests/replaces-infrastructure-requirement-a |
      | create-organizational-requirement  | 🌱🏢create-organizational-requirement/🧪️tests/creates-organizational-requirement-a    |
      | delete-organizational-requirement  | 🗑️🏢delete-organizational-requirement/🧪️tests/deletes-organizational-requirement-a   |
      | rename-organizational-requirement  | ✏️🏢rename-organizational-requirement/🧪️tests/renames-organizational-requirement-a   |
      | replace-organizational-requirement | 🔁🏢replace-organizational-requirement/🧪️tests/replaces-organizational-requirement-a  |
      | create-issue                       | 🌱🐛create-issue/🧪️tests/creates-issue-a                                              |
      | delete-issue                       | 🗑️🐛delete-issue/🧪️tests/deletes-issue-a                                             |
      | rename-issue                       | ✏️🐛rename-issue/🧪️tests/renames-issue-a                                             |
      | replace-issue                      | 🔁🐛replace-issue/🧪️tests/replaces-issue-a                                            |
      | create-approval-record             | 🌱👍create-approval-record/🧪️tests/creates-approval-record-a                          |
      | delete-approval-record             | 🗑️👍delete-approval-record/🧪️tests/deletes-approval-record-a                         |
      | rename-approval-record             | ✏️👍rename-approval-record/🧪️tests/renames-approval-record-a                         |
      | replace-approval-record            | 🔁👍replace-approval-record/🧪️tests/replaces-approval-record-a                        |
      | create-stakeholder                 | 🌱👥create-stakeholder/🧪️tests/creates-stakeholder-a                                  |
      | delete-stakeholder                 | 🗑️👥delete-stakeholder/🧪️tests/deletes-stakeholder-a                                 |
      | rename-stakeholder                 | ✏️👥rename-stakeholder/🧪️tests/renames-stakeholder-a                                 |
      | replace-stakeholder                | 🔁👥replace-stakeholder/🧪️tests/replaces-stakeholder-a                                |
      | create-quality-record              | 🌱💎create-quality-record/🧪️tests/creates-quality-record-a                            |
      | delete-quality-record              | 🗑️💎delete-quality-record/🧪️tests/deletes-quality-record-a                           |
      | rename-quality-record              | ✏️💎rename-quality-record/🧪️tests/renames-quality-record-a                           |
      | replace-quality-record             | 🔁💎replace-quality-record/🧪️tests/replaces-quality-record-a                          |
      | create-resilience-requirement      | 🌱💪create-resilience-requirement/🧪️tests/creates-resilience-requirement-a            |
      | delete-resilience-requirement      | 🗑️💪delete-resilience-requirement/🧪️tests/deletes-resilience-requirement-a           |
      | rename-resilience-requirement      | ✏️💪rename-resilience-requirement/🧪️tests/renames-resilience-requirement-a           |
      | replace-resilience-requirement     | 🔁💪replace-resilience-requirement/🧪️tests/replaces-resilience-requirement-a          |
      | create-assumption                  | 🌱💭create-assumption/🧪️tests/creates-assumption-a                                    |
      | delete-assumption                  | 🗑️💭delete-assumption/🧪️tests/deletes-assumption-a                                   |
      | rename-assumption                  | ✏️💭rename-assumption/🧪️tests/renames-assumption-a                                   |
      | replace-assumption                 | 🔁💭replace-assumption/🧪️tests/replaces-assumption-a                                  |
      | create-cost-requirement            | 🌱💰create-cost-requirement/🧪️tests/creates-cost-requirement-a                        |
      | delete-cost-requirement            | 🗑️💰delete-cost-requirement/🧪️tests/deletes-cost-requirement-a                       |
      | rename-cost-requirement            | ✏️💰rename-cost-requirement/🧪️tests/renames-cost-requirement-a                       |
      | replace-cost-requirement           | 🔁💰replace-cost-requirement/🧪️tests/replaces-cost-requirement-a                      |
      | create-document                    | 🌱📄create-document/🧪️tests/creates-document-a                                        |
      | delete-document                    | 🗑️📄delete-document/🧪️tests/deletes-document-a                                       |
      | rename-document                    | ✏️📄rename-document/🧪️tests/renames-document-a                                       |
      | replace-document                   | 🔁📄replace-document/🧪️tests/replaces-document-a                                      |
      | create-schedule-requirement        | 🌱📅create-schedule-requirement/🧪️tests/creates-schedule-requirement-a                |
      | delete-schedule-requirement        | 🗑️📅delete-schedule-requirement/🧪️tests/deletes-schedule-requirement-a               |
      | rename-schedule-requirement        | ✏️📅rename-schedule-requirement/🧪️tests/renames-schedule-requirement-a               |
      | replace-schedule-requirement       | 🔁📅replace-schedule-requirement/🧪️tests/replaces-schedule-requirement-a              |
      | create-growth-plan                 | 🌱📈create-growth-plan/🧪️tests/creates-growth-plan-a                                  |
      | delete-growth-plan                 | 🗑️📈delete-growth-plan/🧪️tests/deletes-growth-plan-a                                 |
      | rename-growth-plan                 | ✏️📈rename-growth-plan/🧪️tests/renames-growth-plan-a                                 |
      | replace-growth-plan                | 🔁📈replace-growth-plan/🧪️tests/replaces-growth-plan-a                                |
      | create-performance-criterion       | 🌱📊create-performance-criterion/🧪️tests/creates-performance-criterion-a              |
      | delete-performance-criterion       | 🗑️📊delete-performance-criterion/🧪️tests/deletes-performance-criterion-a             |
      | rename-performance-criterion       | ✏️📊rename-performance-criterion/🧪️tests/renames-performance-criterion-a             |
      | replace-performance-criterion      | 🔁📊replace-performance-criterion/🧪️tests/replaces-performance-criterion-a            |
      | create-operational-requirement     | 🌱📋create-operational-requirement/🧪️tests/creates-operational-requirement-a          |
      | delete-operational-requirement     | 🗑️📋delete-operational-requirement/🧪️tests/deletes-operational-requirement-a         |
      | rename-operational-requirement     | ✏️📋rename-operational-requirement/🧪️tests/renames-operational-requirement-a         |
      | replace-operational-requirement    | 🔁📋replace-operational-requirement/🧪️tests/replaces-operational-requirement-a        |
      | create-requirement                 | 🌱📌create-requirement/🧪️tests/creates-requirement-a                                  |
      | delete-requirement                 | 🗑️📌delete-requirement/🧪️tests/deletes-requirement-a                                 |
      | rename-requirement                 | ✏️📌rename-requirement/🧪️tests/renames-requirement-a                                 |
      | replace-requirement                | 🔁📌replace-requirement/🧪️tests/replaces-requirement-a                                |
      | create-site-context                | 🌱📍create-site-context/🧪️tests/creates-site-context-a                                |
      | delete-site-context                | 🗑️📍delete-site-context/🧪️tests/deletes-site-context-a                               |
      | rename-site-context                | ✏️📍rename-site-context/🧪️tests/renames-site-context-a                               |
      | replace-site-context               | 🔁📍replace-site-context/🧪️tests/replaces-site-context-a                              |
      | create-template-record             | 🌱📐create-template-record/🧪️tests/creates-template-record-a                          |
      | delete-template-record             | 🗑️📐delete-template-record/🧪️tests/deletes-template-record-a                         |
      | rename-template-record             | ✏️📐rename-template-record/🧪️tests/renames-template-record-a                         |
      | replace-template-record            | 🔁📐replace-template-record/🧪️tests/replaces-template-record-a                        |
      | create-report-record               | 🌱📑create-report-record/🧪️tests/creates-report-record-a                              |
      | delete-report-record               | 🗑️📑delete-report-record/🧪️tests/deletes-report-record-a                             |
      | rename-report-record               | ✏️📑rename-report-record/🧪️tests/renames-report-record-a                             |
      | replace-report-record              | 🔁📑replace-report-record/🧪️tests/replaces-report-record-a                            |
      | create-audit-event                 | 🌱📒create-audit-event/🧪️tests/creates-audit-event-a                                  |
      | delete-audit-event                 | 🗑️📒delete-audit-event/🧪️tests/deletes-audit-event-a                                 |
      | rename-audit-event                 | ✏️📒rename-audit-event/🧪️tests/renames-audit-event-a                                 |
      | replace-audit-event                | 🔁📒replace-audit-event/🧪️tests/replaces-audit-event-a                                |
      | create-knowledge-record            | 🌱📚create-knowledge-record/🧪️tests/creates-knowledge-record-a                        |
      | delete-knowledge-record            | 🗑️📚delete-knowledge-record/🧪️tests/rejects-deleting-absent-knowledge-record-a       |
      | rename-knowledge-record            | ✏️📚rename-knowledge-record/🧪️tests/rejects-renaming-absent-knowledge-record-a       |
      | replace-knowledge-record           | 🔁📚replace-knowledge-record/🧪️tests/rejects-replacing-absent-knowledge-record-a      |
      | create-regulatory-requirement      | 🌱📜create-regulatory-requirement/🧪️tests/creates-regulatory-requirement-a            |
      | delete-regulatory-requirement      | 🗑️📜delete-regulatory-requirement/🧪️tests/deletes-regulatory-requirement-a           |
      | rename-regulatory-requirement      | ✏️📜rename-regulatory-requirement/🧪️tests/renames-regulatory-requirement-a           |
      | replace-regulatory-requirement     | 🔁📜replace-regulatory-requirement/🧪️tests/replaces-regulatory-requirement-a          |
      | create-change-record               | 🌱📝create-change-record/🧪️tests/creates-change-record-a                              |
      | delete-change-record               | 🗑️📝delete-change-record/🧪️tests/deletes-change-record-a                             |
      | rename-change-record               | ✏️📝rename-change-record/🧪️tests/renames-change-record-a                             |
      | replace-change-record              | 🔁📝replace-change-record/🧪️tests/replaces-change-record-a                            |
      | create-communication-requirement   | 🌱📡create-communication-requirement/🧪️tests/creates-communication-requirement-a      |
      | delete-communication-requirement   | 🗑️📡delete-communication-requirement/🧪️tests/deletes-communication-requirement-a     |
      | rename-communication-requirement   | ✏️📡rename-communication-requirement/🧪️tests/renames-communication-requirement-a     |
      | replace-communication-requirement  | 🔁📡replace-communication-requirement/🧪️tests/replaces-communication-requirement-a    |
      | create-resource                    | 🌱📦create-resource/🧪️tests/creates-resource-a                                        |
      | delete-resource                    | 🗑️📦delete-resource/🧪️tests/deletes-resource-a                                       |
      | rename-resource                    | ✏️📦rename-resource/🧪️tests/renames-resource-a                                       |
      | replace-resource                   | 🔁📦replace-resource/🧪️tests/replaces-resource-a                                      |
      | create-status-record               | 🌱📶create-status-record/🧪️tests/creates-status-record-a                              |
      | delete-status-record               | 🗑️📶delete-status-record/🧪️tests/deletes-status-record-a                             |
      | rename-status-record               | ✏️📶rename-status-record/🧪️tests/renames-status-record-a                             |
      | replace-status-record              | 🔁📶replace-status-record/🧪️tests/replaces-status-record-a                            |
      | create-process                     | 🌱🔄create-process/🧪️tests/creates-process-a                                          |
      | delete-process                     | 🗑️🔄delete-process/🧪️tests/deletes-process-a                                         |
      | rename-process                     | ✏️🔄rename-process/🧪️tests/renames-process-a                                         |
      | replace-process                    | 🔁🔄replace-process/🧪️tests/replaces-process-a                                        |
      | create-search-filter               | 🌱🔍create-search-filter/🧪️tests/creates-search-filter-a                              |
      | delete-search-filter               | 🗑️🔍delete-search-filter/🧪️tests/deletes-search-filter-a                             |
      | rename-search-filter               | ✏️🔍rename-search-filter/🧪️tests/renames-search-filter-a                             |
      | replace-search-filter              | 🔁🔍replace-search-filter/🧪️tests/replaces-search-filter-a                            |
      | create-access-rule                 | 🌱🔑create-access-rule/🧪️tests/creates-access-rule-a                                  |
      | delete-access-rule                 | 🗑️🔑delete-access-rule/🧪️tests/deletes-access-rule-a                                 |
      | rename-access-rule                 | ✏️🔑rename-access-rule/🧪️tests/renames-access-rule-a                                 |
      | replace-access-rule                | 🔁🔑replace-access-rule/🧪️tests/replaces-access-rule-a                                |
      | create-privacy-requirement         | 🌱🔒create-privacy-requirement/🧪️tests/creates-privacy-requirement-a                  |
      | delete-privacy-requirement         | 🗑️🔒delete-privacy-requirement/🧪️tests/deletes-privacy-requirement-a                 |
      | rename-privacy-requirement         | ✏️🔒rename-privacy-requirement/🧪️tests/renames-privacy-requirement-a                 |
      | replace-privacy-requirement        | 🔁🔒replace-privacy-requirement/🧪️tests/replaces-privacy-requirement-a                |
      | create-relationship                | 🌱🔗create-relationship/🧪️tests/creates-relationship-a                                |
      | delete-relationship                | 🗑️🔗delete-relationship/🧪️tests/deletes-relationship-a                               |
      | rename-relationship                | ✏️🔗rename-relationship/🧪️tests/renames-relationship-a                               |
      | replace-relationship               | 🔁🔗replace-relationship/🧪️tests/replaces-relationship-a                              |
      | create-quantity-requirement        | 🌱🔢create-quantity-requirement/🧪️tests/creates-quantity-requirement-a                |
      | delete-quantity-requirement        | 🗑️🔢delete-quantity-requirement/🧪️tests/deletes-quantity-requirement-a               |
      | rename-quantity-requirement        | ✏️🔢rename-quantity-requirement/🧪️tests/renames-quantity-requirement-a               |
      | replace-quantity-requirement       | 🔁🔢replace-quantity-requirement/🧪️tests/replaces-quantity-requirement-a              |
      | create-analysis-record             | 🌱🔬create-analysis-record/🧪️tests/creates-analysis-record-a                          |
      | delete-analysis-record             | 🗑️🔬delete-analysis-record/🧪️tests/deletes-analysis-record-a                         |
      | rename-analysis-record             | ✏️🔬rename-analysis-record/🧪️tests/renames-analysis-record-a                         |
      | replace-analysis-record            | 🔁🔬replace-analysis-record/🧪️tests/replaces-analysis-record-a                        |
      | create-storage-requirement         | 🌱🗄️create-storage-requirement/🧪️tests/creates-storage-requirement-a                 |
      | delete-storage-requirement         | 🗑️🗄️delete-storage-requirement/🧪️tests/deletes-storage-requirement-a                |
      | rename-storage-requirement         | ✏️🗄️rename-storage-requirement/🧪️tests/renames-storage-requirement-a                |
      | replace-storage-requirement        | 🔁🗄️replace-storage-requirement/🧪️tests/replaces-storage-requirement-a               |
      | create-meeting-record              | 🌱🗓️create-meeting-record/🧪️tests/creates-meeting-record-a                           |
      | delete-meeting-record              | 🗑️🗓️delete-meeting-record/🧪️tests/deletes-meeting-record-a                          |
      | rename-meeting-record              | ✏️🗓️rename-meeting-record/🧪️tests/renames-meeting-record-a                          |
      | replace-meeting-record             | 🔁🗓️replace-meeting-record/🧪️tests/replaces-meeting-record-a                         |
      | create-survey                      | 🌱🗳️create-survey/🧪️tests/creates-survey-a                                           |
      | delete-survey                      | 🗑️🗳️delete-survey/🧪️tests/deletes-survey-a                                          |
      | rename-survey                      | ✏️🗳️rename-survey/🧪️tests/renames-survey-a                                          |
      | replace-survey                     | 🔁🗳️replace-survey/🧪️tests/replaces-survey-a                                         |
      | create-delivery-constraint         | 🌱🚚create-delivery-constraint/🧪️tests/creates-delivery-constraint-a                  |
      | delete-delivery-constraint         | 🗑️🚚delete-delivery-constraint/🧪️tests/deletes-delivery-constraint-a                 |
      | rename-delivery-constraint         | ✏️🚚rename-delivery-constraint/🧪️tests/renames-delivery-constraint-a                 |
      | replace-delivery-constraint        | 🔁🚚replace-delivery-constraint/🧪️tests/replaces-delivery-constraint-a                |
      | create-constraint-record           | 🌱🚧create-constraint-record/🧪️tests/creates-constraint-record-a                      |
      | delete-constraint-record           | 🗑️🚧delete-constraint-record/🧪️tests/deletes-constraint-record-a                     |
      | rename-constraint-record           | ✏️🚧rename-constraint-record/🧪️tests/renames-constraint-record-a                     |
      | replace-constraint-record          | 🔁🚧replace-constraint-record/🧪️tests/replaces-constraint-record-a                    |
      | create-compliance-record           | 🌱🛂create-compliance-record/🧪️tests/creates-compliance-record-a                      |
      | delete-compliance-record           | 🗑️🛂delete-compliance-record/🧪️tests/deletes-compliance-record-a                     |
      | rename-compliance-record           | ✏️🛂rename-compliance-record/🧪️tests/renames-compliance-record-a                     |
      | replace-compliance-record          | 🔁🛂replace-compliance-record/🧪️tests/replaces-compliance-record-a                    |
      | create-service-requirement         | 🌱🛎️create-service-requirement/🧪️tests/creates-service-requirement-a                 |
      | delete-service-requirement         | 🗑️🛎️delete-service-requirement/🧪️tests/deletes-service-requirement-a                |
      | rename-service-requirement         | ✏️🛎️rename-service-requirement/🧪️tests/renames-service-requirement-a                |
      | replace-service-requirement        | 🔁🛎️replace-service-requirement/🧪️tests/replaces-service-requirement-a               |
      | create-equipment                   | 🌱🛠️create-equipment/🧪️tests/creates-equipment-a                                     |
      | delete-equipment                   | 🗑️🛠️delete-equipment/🧪️tests/deletes-equipment-a                                    |
      | rename-equipment                   | ✏️🛠️rename-equipment/🧪️tests/renames-equipment-a                                    |
      | replace-equipment                  | 🔁🛠️replace-equipment/🧪️tests/replaces-equipment-a                                   |
      | create-security-requirement        | 🌱🛡️create-security-requirement/🧪️tests/creates-security-requirement-a               |
      | delete-security-requirement        | 🗑️🛡️delete-security-requirement/🧪️tests/deletes-security-requirement-a              |
      | rename-security-requirement        | ✏️🛡️rename-security-requirement/🧪️tests/renames-security-requirement-a              |
      | replace-security-requirement       | 🔁🛡️replace-security-requirement/🧪️tests/replaces-security-requirement-a             |
      | create-collaboration-record        | 🌱🤝create-collaboration-record/🧪️tests/creates-collaboration-record-a                |
      | delete-collaboration-record        | 🗑️🤝delete-collaboration-record/🧪️tests/deletes-collaboration-record-a               |
      | rename-collaboration-record        | ✏️🤝rename-collaboration-record/🧪️tests/renames-collaboration-record-a               |
      | replace-collaboration-record       | 🔁🤝replace-collaboration-record/🧪️tests/replaces-collaboration-record-a              |
      | create-safety-requirement          | 🌱🦺create-safety-requirement/🧪️tests/creates-safety-requirement-a                    |
      | delete-safety-requirement          | 🗑️🦺delete-safety-requirement/🧪️tests/deletes-safety-requirement-a                   |
      | rename-safety-requirement          | ✏️🦺rename-safety-requirement/🧪️tests/renames-safety-requirement-a                   |
      | replace-safety-requirement         | 🔁🦺replace-safety-requirement/🧪️tests/replaces-safety-requirement-a                  |
      | create-user-profile                | 🌱🧑create-user-profile/🧪️tests/creates-user-profile-a                                |
      | delete-user-profile                | 🗑️🧑delete-user-profile/🧪️tests/deletes-user-profile-a                               |
      | rename-user-profile                | ✏️🧑rename-user-profile/🧪️tests/renames-user-profile-a                               |
      | replace-user-profile               | 🔁🧑replace-user-profile/🧪️tests/replaces-user-profile-a                              |
      | create-human-factor-requirement    | 🌱🧠create-human-factor-requirement/🧪️tests/creates-human-factor-requirement-a        |
      | delete-human-factor-requirement    | 🗑️🧠delete-human-factor-requirement/🧪️tests/deletes-human-factor-requirement-a       |
      | rename-human-factor-requirement    | ✏️🧠rename-human-factor-requirement/🧪️tests/renames-human-factor-requirement-a       |
      | replace-human-factor-requirement   | 🔁🧠replace-human-factor-requirement/🧪️tests/replaces-human-factor-requirement-a      |
      | create-flexibility-requirement     | 🌱🧩create-flexibility-requirement/🧪️tests/creates-flexibility-requirement-a          |
      | delete-flexibility-requirement     | 🗑️🧩delete-flexibility-requirement/🧪️tests/deletes-flexibility-requirement-a         |
      | rename-flexibility-requirement     | ✏️🧩rename-flexibility-requirement/🧪️tests/renames-flexibility-requirement-a         |
      | replace-flexibility-requirement    | 🔁🧩replace-flexibility-requirement/🧪️tests/replaces-flexibility-requirement-a        |
      | create-wayfinding-requirement      | 🌱🧭create-wayfinding-requirement/🧪️tests/creates-wayfinding-requirement-a            |
      | delete-wayfinding-requirement      | 🗑️🧭delete-wayfinding-requirement/🧪️tests/deletes-wayfinding-requirement-a           |
      | rename-wayfinding-requirement      | ✏️🧭rename-wayfinding-requirement/🧪️tests/renames-wayfinding-requirement-a           |
      | replace-wayfinding-requirement     | 🔁🧭replace-wayfinding-requirement/🧪️tests/replaces-wayfinding-requirement-a          |
      | create-program-element             | 🌱🧱create-program-element/🧪️tests/creates-program-element-a                          |
      | delete-program-element             | 🗑️🧱delete-program-element/🧪️tests/deletes-program-element-a                         |
      | rename-program-element             | ✏️🧱rename-program-element/🧪️tests/renames-program-element-a                         |
      | replace-program-element            | 🔁🧱replace-program-element/🧪️tests/replaces-program-element-a                        |
      | connect-adjacency                  | 🔗🧲connect-adjacency/🧪️tests/connects-reception-to-waiting                           |
      | disconnect-adjacency               | ✂️🧲disconnect-adjacency/🧪️tests/disconnects-reception-from-waiting                  |
      | connect-trace                      | 🔗🧵connect-trace/🧪️tests/connects-requirement-a-to-decision-a                        |
      | disconnect-trace                   | ✂️🧵disconnect-trace/🧪️tests/disconnects-requirement-a-from-decision-a               |
      | rename-meta                        | ✏️🏷️rename-meta/🧪️tests/renames-the-document-title                                  |
      | replace-meta                       | 🔁🏷️replace-meta/🧪️tests/replaces-the-document-meta-block                            |
      | rename-project                     | ✏️📁rename-project/🧪️tests/renames-the-project-code                                  |
      | replace-project                    | 🔁📁replace-project/🧪️tests/replaces-the-project-definition                           |
      | rename-governance                  | ✏️🏛️rename-governance/🧪️tests/renames-the-governance-framework                      |
      | replace-governance                 | 🔁🏛️replace-governance/🧪️tests/replaces-the-governance-block                         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a ProgramSnapshot, printed back to `.architect` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
