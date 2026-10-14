@capability-program-1-mutate
@oracle-architect-program-zip-reader
@oracle-input-subject-raw
@comparison-ordered-json-v1
@mutations-program-1-any
Feature: Export every typed architect program mutation through ZIP and decode it independently
  This case applies the 14 typed mutation kinds through the Rust subject, over 266 committed vectors (one per register for each generic register-row kind, plus the edge and facet kinds),, checks each result against its
  committed before, after and outcome vectors, then exports the result through the production
  `program -> stdio.zip` serializer. The oracle receives those exact produced bytes through
  `@oracle-input-subject-raw` and opens them with the approved third-party `zip` 6 reader. It does not
  link the architect plugin, apply a mutation, or manufacture an expected archive.

  The semantic comparison is the sorted set of the archive's seventy JSON members: `program`,
  `meta`, `project`, `governance`, and all 66 registers. The subject projects the logical `ZipSnapshot`
  before encoding; the oracle projects the independently decompressed members after encoding. Thus
  an empty archive, a missing register, a dropped record, corrupt JSON, or a ZIP writer defect is a
  real divergence. XLSX is independently covered by the sibling exporter contract with `calamine`;
  ZIP is the mutation carrier because its JSON members reconstruct nested program values without a
  spreadsheet type heuristic.

  `🐍️.py` remains a supplemental cross-language implementation of the mutation algebra,
  registered as `architect-program-python-independent`, but it is no longer asked to stand in for a
  third-party carrier reader. The qualifying oracle in this feature adjudicates the externally
  observable bytes, while the committed vectors continue to adjudicate mutation semantics.

  📌️ 260 of the 266 committed vectors move the document. The six that do not are
  `delete-register-row`/`rename-register-row`/`replace-register-row` over `knowledgeRecord` and `benchmarkRecord`, and the reason is
  structural rather than an authoring gap: those two registers alone are composed
  `s.stdio.semio.table` CHILD handles whose rows live in a working-scene cache a fresh process has
  never populated, so the only branch reachable from a committed snapshot is the
  `mutation.target-missing` rejection — which is exactly what those six vectors pin. They are named
  in the subject adapter's `GUARD_VECTORS` list and exempted from the observability law on that basis; the
  other 260 vectors carry it with no exemption.

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them. The subject additionally asserts that every non-guard
  mutation changes the ZIP carrier projection, so an exporter that returns a valid but invariant
  archive cannot satisfy the feature.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                                                | vector |
      | create-register-row-information-requirement      | 📋️register-row/🌱️create/ℹ️information-creates-a |
      | delete-register-row-information-requirement      | 📋️register-row/🗑️delete/ℹ️information-deletes-a |
      | rename-register-row-information-requirement      | 📋️register-row/🏷️rename/ℹ️information-renames-a |
      | replace-register-row-information-requirement     | 📋️register-row/♻️replace/ℹ️information-replaces-a |
      | create-register-row-sustainability-requirement   | 📋️register-row/🌱️create/♻️sustainability-creates-a |
      | delete-register-row-sustainability-requirement   | 📋️register-row/🗑️delete/♻️sustainability-deletes-a |
      | rename-register-row-sustainability-requirement   | 📋️register-row/🏷️rename/♻️sustainability-renames-a |
      | replace-register-row-sustainability-requirement  | 📋️register-row/♻️replace/♻️sustainability-replaces-a |
      | create-register-row-accessibility-requirement    | 📋️register-row/🌱️create/♿️accessibility-creates-a |
      | delete-register-row-accessibility-requirement    | 📋️register-row/🗑️delete/♿️accessibility-deletes-a |
      | rename-register-row-accessibility-requirement    | 📋️register-row/🏷️rename/♿️accessibility-renames-a |
      | replace-register-row-accessibility-requirement   | 📋️register-row/♻️replace/♿️accessibility-replaces-a |
      | create-register-row-conflict                     | 📋️register-row/🌱️create/⚔️conflict-creates |
      | delete-register-row-conflict                     | 📋️register-row/🗑️delete/⚔️conflict-deletes |
      | rename-register-row-conflict                     | 📋️register-row/🏷️rename/⚔️conflict-renames |
      | replace-register-row-conflict                    | 📋️register-row/♻️replace/⚔️conflict-replaces |
      | create-register-row-option-evaluation            | 📋️register-row/🌱️create/⚖️option-creates |
      | delete-register-row-option-evaluation            | 📋️register-row/🗑️delete/⚖️option-deletes |
      | rename-register-row-option-evaluation            | 📋️register-row/🏷️rename/⚖️option-renames |
      | replace-register-row-option-evaluation           | 📋️register-row/♻️replace/⚖️option-replaces |
      | create-register-row-function                     | 📋️register-row/🌱️create/⚙️function-creates |
      | delete-register-row-function                     | 📋️register-row/🗑️delete/⚙️function-deletes |
      | rename-register-row-function                     | 📋️register-row/🏷️rename/⚙️function-renames |
      | replace-register-row-function                    | 📋️register-row/♻️replace/⚙️function-replaces |
      | create-register-row-risk                         | 📋️register-row/🌱️create/⚠️risk-creates-a |
      | delete-register-row-risk                         | 📋️register-row/🗑️delete/⚠️risk-deletes-a |
      | rename-register-row-risk                         | 📋️register-row/🏷️rename/⚠️risk-renames-a |
      | replace-register-row-risk                        | 📋️register-row/♻️replace/⚠️risk-replaces-a |
      | create-register-row-decision                     | 📋️register-row/🌱️create/✅️decision-creates |
      | delete-register-row-decision                     | 📋️register-row/🗑️delete/✅️decision-deletes |
      | rename-register-row-decision                     | 📋️register-row/🏷️rename/✅️decision-renames |
      | replace-register-row-decision                    | 📋️register-row/♻️replace/✅️decision-replaces |
      | create-register-row-validation-record            | 📋️register-row/🌱️create/✔️validation-record-creates-a |
      | delete-register-row-validation-record            | 📋️register-row/🗑️delete/✔️validation-record-deletes-a |
      | rename-register-row-validation-record            | 📋️register-row/🏷️rename/✔️validation-record-renames-a |
      | replace-register-row-validation-record           | 📋️register-row/♻️replace/✔️validation-record-replaces-a |
      | create-register-row-priority-record              | 📋️register-row/🌱️create/⭐️priority-record-creates-a |
      | delete-register-row-priority-record              | 📋️register-row/🗑️delete/⭐️priority-record-deletes-a |
      | rename-register-row-priority-record              | 📋️register-row/🏷️rename/⭐️priority-record-renames-a |
      | replace-register-row-priority-record             | 📋️register-row/♻️replace/⭐️priority-record-replaces-a |
      | create-register-row-flow-requirement             | 📋️register-row/🌱️create/🌊️flow-creates-a |
      | delete-register-row-flow-requirement             | 📋️register-row/🗑️delete/🌊️flow-deletes-a |
      | rename-register-row-flow-requirement             | 📋️register-row/🏷️rename/🌊️flow-renames-a |
      | replace-register-row-flow-requirement            | 📋️register-row/♻️replace/🌊️flow-replaces-a |
      | create-register-row-environmental-requirement    | 📋️register-row/🌱️create/🌿️environmental-creates-a |
      | delete-register-row-environmental-requirement    | 📋️register-row/🗑️delete/🌿️environmental-deletes-a |
      | rename-register-row-environmental-requirement    | 📋️register-row/🏷️rename/🌿️environmental-renames-a |
      | replace-register-row-environmental-requirement   | 📋️register-row/♻️replace/🌿️environmental-replaces-a |
      | create-register-row-workshop                     | 📋️register-row/🌱️create/🎓️workshop-creates-a |
      | delete-register-row-workshop                     | 📋️register-row/🗑️delete/🎓️workshop-deletes-a |
      | rename-register-row-workshop                     | 📋️register-row/🏷️rename/🎓️workshop-renames-a |
      | replace-register-row-workshop                    | 📋️register-row/♻️replace/🎓️workshop-replaces-a |
      | create-register-row-scenario                     | 📋️register-row/🌱️create/🎬️scenario-creates-a |
      | delete-register-row-scenario                     | 📋️register-row/🗑️delete/🎬️scenario-deletes-a |
      | rename-register-row-scenario                     | 📋️register-row/🏷️rename/🎬️scenario-renames-a |
      | replace-register-row-scenario                    | 📋️register-row/♻️replace/🎬️scenario-replaces-a |
      | create-register-row-benchmark-record             | 📋️register-row/🌱️create/🏁️benchmark-record-creates-a |
      | delete-register-row-benchmark-record             | 📋️register-row/🗑️delete/🏁️benchmark-record-absent-a |
      | rename-register-row-benchmark-record             | 📋️register-row/🏷️rename/🏁️benchmark-record-absent-a |
      | replace-register-row-benchmark-record            | 📋️register-row/♻️replace/🏁️benchmark-record-absent-a |
      | create-register-row-activity                     | 📋️register-row/🌱️create/🏃️activity-creates-a |
      | delete-register-row-activity                     | 📋️register-row/🗑️delete/🏃️activity-deletes-a |
      | rename-register-row-activity                     | 📋️register-row/🏷️rename/🏃️activity-renames-a |
      | replace-register-row-activity                    | 📋️register-row/♻️replace/🏃️activity-replaces-a |
      | create-register-row-infrastructure-requirement   | 📋️register-row/🌱️create/🏗️infrastructure-creates-a |
      | delete-register-row-infrastructure-requirement   | 📋️register-row/🗑️delete/🏗️infrastructure-deletes-a |
      | rename-register-row-infrastructure-requirement   | 📋️register-row/🏷️rename/🏗️infrastructure-renames-a |
      | replace-register-row-infrastructure-requirement  | 📋️register-row/♻️replace/🏗️infrastructure-replaces-a |
      | create-register-row-organizational-requirement   | 📋️register-row/🌱️create/🏢️organizational-creates-a |
      | delete-register-row-organizational-requirement   | 📋️register-row/🗑️delete/🏢️organizational-deletes-a |
      | rename-register-row-organizational-requirement   | 📋️register-row/🏷️rename/🏢️organizational-renames-a |
      | replace-register-row-organizational-requirement  | 📋️register-row/♻️replace/🏢️organizational-replaces-a |
      | create-register-row-issue                        | 📋️register-row/🌱️create/🐛️issue-creates |
      | delete-register-row-issue                        | 📋️register-row/🗑️delete/🐛️issue-deletes |
      | rename-register-row-issue                        | 📋️register-row/🏷️rename/🐛️issue-renames |
      | replace-register-row-issue                       | 📋️register-row/♻️replace/🐛️issue-replaces |
      | create-register-row-approval-record              | 📋️register-row/🌱️create/👍️approval-record-creates-a |
      | delete-register-row-approval-record              | 📋️register-row/🗑️delete/👍️approval-record-deletes-a |
      | rename-register-row-approval-record              | 📋️register-row/🏷️rename/👍️approval-record-renames-a |
      | replace-register-row-approval-record             | 📋️register-row/♻️replace/👍️approval-record-replaces-a |
      | create-register-row-stakeholder                  | 📋️register-row/🌱️create/👥️stakeholder-creates-a |
      | delete-register-row-stakeholder                  | 📋️register-row/🗑️delete/👥️stakeholder-deletes-a |
      | rename-register-row-stakeholder                  | 📋️register-row/🏷️rename/👥️stakeholder-renames-a |
      | replace-register-row-stakeholder                 | 📋️register-row/♻️replace/👥️stakeholder-replaces-a |
      | create-register-row-quality-record               | 📋️register-row/🌱️create/💎️quality-record-creates-a |
      | delete-register-row-quality-record               | 📋️register-row/🗑️delete/💎️quality-record-deletes-a |
      | rename-register-row-quality-record               | 📋️register-row/🏷️rename/💎️quality-record-renames-a |
      | replace-register-row-quality-record              | 📋️register-row/♻️replace/💎️quality-record-replaces-a |
      | create-register-row-resilience-requirement       | 📋️register-row/🌱️create/💪️resilience-creates-a |
      | delete-register-row-resilience-requirement       | 📋️register-row/🗑️delete/💪️resilience-deletes-a |
      | rename-register-row-resilience-requirement       | 📋️register-row/🏷️rename/💪️resilience-renames-a |
      | replace-register-row-resilience-requirement      | 📋️register-row/♻️replace/💪️resilience-replaces-a |
      | create-register-row-assumption                   | 📋️register-row/🌱️create/💭️assumption-creates-a |
      | delete-register-row-assumption                   | 📋️register-row/🗑️delete/💭️assumption-deletes-a |
      | rename-register-row-assumption                   | 📋️register-row/🏷️rename/💭️assumption-renames-a |
      | replace-register-row-assumption                  | 📋️register-row/♻️replace/💭️assumption-replaces-a |
      | create-register-row-cost-requirement             | 📋️register-row/🌱️create/💰️cost-creates-a |
      | delete-register-row-cost-requirement             | 📋️register-row/🗑️delete/💰️cost-deletes-a |
      | rename-register-row-cost-requirement             | 📋️register-row/🏷️rename/💰️cost-renames-a |
      | replace-register-row-cost-requirement            | 📋️register-row/♻️replace/💰️cost-replaces-a |
      | create-register-row-document                     | 📋️register-row/🌱️create/📃️document-creates-a |
      | delete-register-row-document                     | 📋️register-row/🗑️delete/📃️document-deletes-a |
      | rename-register-row-document                     | 📋️register-row/🏷️rename/📃️document-renames-a |
      | replace-register-row-document                    | 📋️register-row/♻️replace/📃️document-replaces-a |
      | create-register-row-schedule-requirement         | 📋️register-row/🌱️create/📅️schedule-creates-a |
      | delete-register-row-schedule-requirement         | 📋️register-row/🗑️delete/📅️schedule-deletes-a |
      | rename-register-row-schedule-requirement         | 📋️register-row/🏷️rename/📅️schedule-renames-a |
      | replace-register-row-schedule-requirement        | 📋️register-row/♻️replace/📅️schedule-replaces |
      | create-register-row-growth-plan                  | 📋️register-row/🌱️create/📈️growth-plan-creates-a |
      | delete-register-row-growth-plan                  | 📋️register-row/🗑️delete/📈️growth-plan-deletes-a |
      | rename-register-row-growth-plan                  | 📋️register-row/🏷️rename/📈️growth-plan-renames-a |
      | replace-register-row-growth-plan                 | 📋️register-row/♻️replace/📈️growth-plan-replaces-a |
      | create-register-row-performance-criterion        | 📋️register-row/🌱️create/📊️performance-creates-a |
      | delete-register-row-performance-criterion        | 📋️register-row/🗑️delete/📊️performance-deletes-a |
      | rename-register-row-performance-criterion        | 📋️register-row/🏷️rename/📊️performance-renames-a |
      | replace-register-row-performance-criterion       | 📋️register-row/♻️replace/📊️performance-replaces-a |
      | create-register-row-operational-requirement      | 📋️register-row/🌱️create/📋️operational-creates-a |
      | delete-register-row-operational-requirement      | 📋️register-row/🗑️delete/📋️operational-deletes-a |
      | rename-register-row-operational-requirement      | 📋️register-row/🏷️rename/📋️operational-renames-a |
      | replace-register-row-operational-requirement     | 📋️register-row/♻️replace/📋️operational-replaces-a |
      | create-register-row-requirement                  | 📋️register-row/🌱️create/📌️requirement-creates-a |
      | delete-register-row-requirement                  | 📋️register-row/🗑️delete/📌️requirement-deletes-a |
      | rename-register-row-requirement                  | 📋️register-row/🏷️rename/📌️requirement-renames-a |
      | replace-register-row-requirement                 | 📋️register-row/♻️replace/📌️requirement-replaces-a |
      | create-register-row-site-context                 | 📋️register-row/🌱️create/📍️site-creates-a |
      | delete-register-row-site-context                 | 📋️register-row/🗑️delete/📍️site-deletes-a |
      | rename-register-row-site-context                 | 📋️register-row/🏷️rename/📍️site-renames-a |
      | replace-register-row-site-context                | 📋️register-row/♻️replace/📍️site-replaces-a |
      | create-register-row-template-record              | 📋️register-row/🌱️create/📐️template-record-creates-a |
      | delete-register-row-template-record              | 📋️register-row/🗑️delete/📐️template-record-deletes-a |
      | rename-register-row-template-record              | 📋️register-row/🏷️rename/📐️template-record-renames-a |
      | replace-register-row-template-record             | 📋️register-row/♻️replace/📐️template-record-replaces-a |
      | create-register-row-report-record                | 📋️register-row/🌱️create/📑️report-record-creates-a |
      | delete-register-row-report-record                | 📋️register-row/🗑️delete/📑️report-record-deletes-a |
      | rename-register-row-report-record                | 📋️register-row/🏷️rename/📑️report-record-renames-a |
      | replace-register-row-report-record               | 📋️register-row/♻️replace/📑️report-record-replaces-a |
      | create-register-row-audit-event                  | 📋️register-row/🌱️create/📒️audit-creates |
      | delete-register-row-audit-event                  | 📋️register-row/🗑️delete/📒️audit-deletes |
      | rename-register-row-audit-event                  | 📋️register-row/🏷️rename/📒️audit-renames |
      | replace-register-row-audit-event                 | 📋️register-row/♻️replace/📒️audit-replaces |
      | create-register-row-knowledge-record             | 📋️register-row/🌱️create/📚️knowledge-record-creates-a |
      | delete-register-row-knowledge-record             | 📋️register-row/🗑️delete/📚️knowledge-record-absent-a |
      | rename-register-row-knowledge-record             | 📋️register-row/🏷️rename/📚️knowledge-record-absent-a |
      | replace-register-row-knowledge-record            | 📋️register-row/♻️replace/📚️knowledge-record-absent-a |
      | create-register-row-regulatory-requirement       | 📋️register-row/🌱️create/📜️regulatory-creates-a |
      | delete-register-row-regulatory-requirement       | 📋️register-row/🗑️delete/📜️regulatory-deletes-a |
      | rename-register-row-regulatory-requirement       | 📋️register-row/🏷️rename/📜️regulatory-renames-a |
      | replace-register-row-regulatory-requirement      | 📋️register-row/♻️replace/📜️regulatory-replaces-a |
      | create-register-row-communication-requirement    | 📋️register-row/🌱️create/📡️communication-creates-a |
      | delete-register-row-communication-requirement    | 📋️register-row/🗑️delete/📡️communication-deletes-a |
      | rename-register-row-communication-requirement    | 📋️register-row/🏷️rename/📡️communication-renames-a |
      | replace-register-row-communication-requirement   | 📋️register-row/♻️replace/📡️communication-replaces-a |
      | create-register-row-resource                     | 📋️register-row/🌱️create/📦️resource-creates-a |
      | delete-register-row-resource                     | 📋️register-row/🗑️delete/📦️resource-deletes-a |
      | rename-register-row-resource                     | 📋️register-row/🏷️rename/📦️resource-renames-a |
      | replace-register-row-resource                    | 📋️register-row/♻️replace/📦️resource-replaces-a |
      | create-register-row-status-record                | 📋️register-row/🌱️create/📶️status-record-creates-a |
      | delete-register-row-status-record                | 📋️register-row/🗑️delete/📶️status-record-deletes-a |
      | rename-register-row-status-record                | 📋️register-row/🏷️rename/📶️status-record-renames-a |
      | replace-register-row-status-record               | 📋️register-row/♻️replace/📶️status-record-replaces-a |
      | create-register-row-change-record                | 📋️register-row/🌱️create/🔀️change-record-creates-a |
      | delete-register-row-change-record                | 📋️register-row/🗑️delete/🔀️change-record-deletes-a |
      | rename-register-row-change-record                | 📋️register-row/🏷️rename/🔀️change-record-renames-a |
      | replace-register-row-change-record               | 📋️register-row/♻️replace/🔀️change-record-replaces-a |
      | create-register-row-process                      | 📋️register-row/🌱️create/🔄️process-creates |
      | delete-register-row-process                      | 📋️register-row/🗑️delete/🔄️process-deletes |
      | rename-register-row-process                      | 📋️register-row/🏷️rename/🔄️process-renames |
      | replace-register-row-process                     | 📋️register-row/♻️replace/🔄️process-replaces |
      | create-register-row-search-filter                | 📋️register-row/🌱️create/🔍️search-filter-creates-a |
      | delete-register-row-search-filter                | 📋️register-row/🗑️delete/🔍️search-filter-deletes-a |
      | rename-register-row-search-filter                | 📋️register-row/🏷️rename/🔍️search-filter-renames-a |
      | replace-register-row-search-filter               | 📋️register-row/♻️replace/🔍️search-filter-replaces-a |
      | create-register-row-access-rule                  | 📋️register-row/🌱️create/🔑️access-rule-creates-a |
      | delete-register-row-access-rule                  | 📋️register-row/🗑️delete/🔑️access-rule-deletes-a |
      | rename-register-row-access-rule                  | 📋️register-row/🏷️rename/🔑️access-rule-renames-a |
      | replace-register-row-access-rule                 | 📋️register-row/♻️replace/🔑️access-rule-replaces-a |
      | create-register-row-privacy-requirement          | 📋️register-row/🌱️create/🔒️privacy-creates |
      | delete-register-row-privacy-requirement          | 📋️register-row/🗑️delete/🔒️privacy-deletes |
      | rename-register-row-privacy-requirement          | 📋️register-row/🏷️rename/🔒️privacy-renames |
      | replace-register-row-privacy-requirement         | 📋️register-row/♻️replace/🔒️privacy-replaces |
      | create-register-row-quantity-requirement         | 📋️register-row/🌱️create/🔢️quantity-creates-a |
      | delete-register-row-quantity-requirement         | 📋️register-row/🗑️delete/🔢️quantity-deletes-a |
      | rename-register-row-quantity-requirement         | 📋️register-row/🏷️rename/🔢️quantity-renames-a |
      | replace-register-row-quantity-requirement        | 📋️register-row/♻️replace/🔢️quantity-replaces |
      | create-register-row-analysis-record              | 📋️register-row/🌱️create/🔬️analysis-record-creates-a |
      | delete-register-row-analysis-record              | 📋️register-row/🗑️delete/🔬️analysis-record-deletes-a |
      | rename-register-row-analysis-record              | 📋️register-row/🏷️rename/🔬️analysis-record-renames-a |
      | replace-register-row-analysis-record             | 📋️register-row/♻️replace/🔬️analysis-record-replaces-a |
      | create-register-row-relationship                 | 📋️register-row/🌱️create/🕸️relationship-creates-a |
      | delete-register-row-relationship                 | 📋️register-row/🗑️delete/🕸️relationship-deletes-a |
      | rename-register-row-relationship                 | 📋️register-row/🏷️rename/🕸️relationship-renames-a |
      | replace-register-row-relationship                | 📋️register-row/♻️replace/🕸️relationship-replaces-a |
      | create-register-row-storage-requirement          | 📋️register-row/🌱️create/🗄️storage-creates |
      | delete-register-row-storage-requirement          | 📋️register-row/🗑️delete/🗄️storage-deletes |
      | rename-register-row-storage-requirement          | 📋️register-row/🏷️rename/🗄️storage-renames |
      | replace-register-row-storage-requirement         | 📋️register-row/♻️replace/🗄️storage-replaces |
      | create-register-row-meeting-record               | 📋️register-row/🌱️create/🗓️meeting-record-creates-a |
      | delete-register-row-meeting-record               | 📋️register-row/🗑️delete/🗓️meeting-record-deletes-a |
      | rename-register-row-meeting-record               | 📋️register-row/🏷️rename/🗓️meeting-record-renames-a |
      | replace-register-row-meeting-record              | 📋️register-row/♻️replace/🗓️meeting-record-replaces-a |
      | create-register-row-survey                       | 📋️register-row/🌱️create/🗳️survey-creates |
      | delete-register-row-survey                       | 📋️register-row/🗑️delete/🗳️survey-deletes |
      | rename-register-row-survey                       | 📋️register-row/🏷️rename/🗳️survey-renames |
      | replace-register-row-survey                      | 📋️register-row/♻️replace/🗳️survey-replaces |
      | create-register-row-delivery-constraint          | 📋️register-row/🌱️create/🚚️delivery-constraint-creates |
      | delete-register-row-delivery-constraint          | 📋️register-row/🗑️delete/🚚️delivery-constraint-deletes |
      | rename-register-row-delivery-constraint          | 📋️register-row/🏷️rename/🚚️delivery-constraint-renames |
      | replace-register-row-delivery-constraint         | 📋️register-row/♻️replace/🚚️delivery-constraint-replaces |
      | create-register-row-constraint-record            | 📋️register-row/🌱️create/🚧️constraint-record-creates-a |
      | delete-register-row-constraint-record            | 📋️register-row/🗑️delete/🚧️constraint-record-deletes-a |
      | rename-register-row-constraint-record            | 📋️register-row/🏷️rename/🚧️constraint-record-renames-a |
      | replace-register-row-constraint-record           | 📋️register-row/♻️replace/🚧️constraint-record-replaces-a |
      | create-register-row-compliance-record            | 📋️register-row/🌱️create/🛂️compliance-record-creates-a |
      | delete-register-row-compliance-record            | 📋️register-row/🗑️delete/🛂️compliance-record-deletes-a |
      | rename-register-row-compliance-record            | 📋️register-row/🏷️rename/🛂️compliance-record-renames-a |
      | replace-register-row-compliance-record           | 📋️register-row/♻️replace/🛂️compliance-record-replaces-a |
      | create-register-row-service-requirement          | 📋️register-row/🌱️create/🛎️service-creates |
      | delete-register-row-service-requirement          | 📋️register-row/🗑️delete/🛎️service-deletes |
      | rename-register-row-service-requirement          | 📋️register-row/🏷️rename/🛎️service-renames |
      | replace-register-row-service-requirement         | 📋️register-row/♻️replace/🛎️service-replaces |
      | create-register-row-equipment                    | 📋️register-row/🌱️create/🛠️equipment-creates-a |
      | delete-register-row-equipment                    | 📋️register-row/🗑️delete/🛠️equipment-deletes-a |
      | rename-register-row-equipment                    | 📋️register-row/🏷️rename/🛠️equipment-renames-a |
      | replace-register-row-equipment                   | 📋️register-row/♻️replace/🛠️equipment-replaces-a |
      | create-register-row-security-requirement         | 📋️register-row/🌱️create/🛡️security-creates-a |
      | delete-register-row-security-requirement         | 📋️register-row/🗑️delete/🛡️security-deletes-a |
      | rename-register-row-security-requirement         | 📋️register-row/🏷️rename/🛡️security-renames-a |
      | replace-register-row-security-requirement        | 📋️register-row/♻️replace/🛡️security-replaces |
      | create-register-row-collaboration-record         | 📋️register-row/🌱️create/🤝️collaboration-creates-a |
      | delete-register-row-collaboration-record         | 📋️register-row/🗑️delete/🤝️collaboration-deletes-a |
      | rename-register-row-collaboration-record         | 📋️register-row/🏷️rename/🤝️collaboration-renames-a |
      | replace-register-row-collaboration-record        | 📋️register-row/♻️replace/🤝️collaboration-replaces |
      | create-register-row-safety-requirement           | 📋️register-row/🌱️create/🦺️safety-creates |
      | delete-register-row-safety-requirement           | 📋️register-row/🗑️delete/🦺️safety-deletes |
      | rename-register-row-safety-requirement           | 📋️register-row/🏷️rename/🦺️safety-renames |
      | replace-register-row-safety-requirement          | 📋️register-row/♻️replace/🦺️safety-replaces |
      | create-register-row-user-profile                 | 📋️register-row/🌱️create/🧑️user-creates-a |
      | delete-register-row-user-profile                 | 📋️register-row/🗑️delete/🧑️user-deletes-a |
      | rename-register-row-user-profile                 | 📋️register-row/🏷️rename/🧑️user-renames-a |
      | replace-register-row-user-profile                | 📋️register-row/♻️replace/🧑️user-replaces-a |
      | create-register-row-human-factor-requirement     | 📋️register-row/🌱️create/🧠️human-creates |
      | delete-register-row-human-factor-requirement     | 📋️register-row/🗑️delete/🧠️human-deletes |
      | rename-register-row-human-factor-requirement     | 📋️register-row/🏷️rename/🧠️human-renames |
      | replace-register-row-human-factor-requirement    | 📋️register-row/♻️replace/🧠️human-replaces |
      | create-register-row-flexibility-requirement      | 📋️register-row/🌱️create/🧩️flexibility-creates-a |
      | delete-register-row-flexibility-requirement      | 📋️register-row/🗑️delete/🧩️flexibility-deletes-a |
      | rename-register-row-flexibility-requirement      | 📋️register-row/🏷️rename/🧩️flexibility-renames-a |
      | replace-register-row-flexibility-requirement     | 📋️register-row/♻️replace/🧩️flexibility-replaces-a |
      | create-register-row-wayfinding-requirement       | 📋️register-row/🌱️create/🧭️wayfinding-creates-a |
      | delete-register-row-wayfinding-requirement       | 📋️register-row/🗑️delete/🧭️wayfinding-deletes-a |
      | rename-register-row-wayfinding-requirement       | 📋️register-row/🏷️rename/🧭️wayfinding-renames-a |
      | replace-register-row-wayfinding-requirement      | 📋️register-row/♻️replace/🧭️wayfinding-replaces-a |
      | create-register-row-program-element              | 📋️register-row/🌱️create/🧱️program-element-creates-a |
      | delete-register-row-program-element              | 📋️register-row/🗑️delete/🧱️program-element-deletes-a |
      | rename-register-row-program-element              | 📋️register-row/🏷️rename/🧱️program-element-renames-a |
      | replace-register-row-program-element             | 📋️register-row/♻️replace/🧱️program-element-replaces-a |
      | connect-adjacency                                | 🧲️adjacency/🧲️connect/🧲️reception |
      | disconnect-adjacency                             | 🧲️adjacency/🫷️disconnect/🫷️reception |
      | connect-trace                                    | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                                 | 🧵️trace/✂️disconnect/✂️requirement |
      | rename-meta                                      | 🏷️meta/🏷️rename/🏷️title |
      | replace-meta                                     | 🏷️meta/♻️replace/♻️block |
      | rename-project                                   | 🏙️project/🏷️rename/🏷️code |
      | replace-project                                  | 🏙️project/♻️replace/♻️definition |
      | rename-governance                                | 🏛️governance/🏷️rename/🏷️framework |
      | replace-governance                               | 🏛️governance/♻️replace/♻️block |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                                                | vector |
      | create-register-row-information-requirement      | 📋️register-row/🌱️create/ℹ️information-creates-a |
      | delete-register-row-information-requirement      | 📋️register-row/🗑️delete/ℹ️information-deletes-a |
      | rename-register-row-information-requirement      | 📋️register-row/🏷️rename/ℹ️information-renames-a |
      | replace-register-row-information-requirement     | 📋️register-row/♻️replace/ℹ️information-replaces-a |
      | create-register-row-sustainability-requirement   | 📋️register-row/🌱️create/♻️sustainability-creates-a |
      | delete-register-row-sustainability-requirement   | 📋️register-row/🗑️delete/♻️sustainability-deletes-a |
      | rename-register-row-sustainability-requirement   | 📋️register-row/🏷️rename/♻️sustainability-renames-a |
      | replace-register-row-sustainability-requirement  | 📋️register-row/♻️replace/♻️sustainability-replaces-a |
      | create-register-row-accessibility-requirement    | 📋️register-row/🌱️create/♿️accessibility-creates-a |
      | delete-register-row-accessibility-requirement    | 📋️register-row/🗑️delete/♿️accessibility-deletes-a |
      | rename-register-row-accessibility-requirement    | 📋️register-row/🏷️rename/♿️accessibility-renames-a |
      | replace-register-row-accessibility-requirement   | 📋️register-row/♻️replace/♿️accessibility-replaces-a |
      | create-register-row-conflict                     | 📋️register-row/🌱️create/⚔️conflict-creates |
      | delete-register-row-conflict                     | 📋️register-row/🗑️delete/⚔️conflict-deletes |
      | rename-register-row-conflict                     | 📋️register-row/🏷️rename/⚔️conflict-renames |
      | replace-register-row-conflict                    | 📋️register-row/♻️replace/⚔️conflict-replaces |
      | create-register-row-option-evaluation            | 📋️register-row/🌱️create/⚖️option-creates |
      | delete-register-row-option-evaluation            | 📋️register-row/🗑️delete/⚖️option-deletes |
      | rename-register-row-option-evaluation            | 📋️register-row/🏷️rename/⚖️option-renames |
      | replace-register-row-option-evaluation           | 📋️register-row/♻️replace/⚖️option-replaces |
      | create-register-row-function                     | 📋️register-row/🌱️create/⚙️function-creates |
      | delete-register-row-function                     | 📋️register-row/🗑️delete/⚙️function-deletes |
      | rename-register-row-function                     | 📋️register-row/🏷️rename/⚙️function-renames |
      | replace-register-row-function                    | 📋️register-row/♻️replace/⚙️function-replaces |
      | create-register-row-risk                         | 📋️register-row/🌱️create/⚠️risk-creates-a |
      | delete-register-row-risk                         | 📋️register-row/🗑️delete/⚠️risk-deletes-a |
      | rename-register-row-risk                         | 📋️register-row/🏷️rename/⚠️risk-renames-a |
      | replace-register-row-risk                        | 📋️register-row/♻️replace/⚠️risk-replaces-a |
      | create-register-row-decision                     | 📋️register-row/🌱️create/✅️decision-creates |
      | delete-register-row-decision                     | 📋️register-row/🗑️delete/✅️decision-deletes |
      | rename-register-row-decision                     | 📋️register-row/🏷️rename/✅️decision-renames |
      | replace-register-row-decision                    | 📋️register-row/♻️replace/✅️decision-replaces |
      | create-register-row-validation-record            | 📋️register-row/🌱️create/✔️validation-record-creates-a |
      | delete-register-row-validation-record            | 📋️register-row/🗑️delete/✔️validation-record-deletes-a |
      | rename-register-row-validation-record            | 📋️register-row/🏷️rename/✔️validation-record-renames-a |
      | replace-register-row-validation-record           | 📋️register-row/♻️replace/✔️validation-record-replaces-a |
      | create-register-row-priority-record              | 📋️register-row/🌱️create/⭐️priority-record-creates-a |
      | delete-register-row-priority-record              | 📋️register-row/🗑️delete/⭐️priority-record-deletes-a |
      | rename-register-row-priority-record              | 📋️register-row/🏷️rename/⭐️priority-record-renames-a |
      | replace-register-row-priority-record             | 📋️register-row/♻️replace/⭐️priority-record-replaces-a |
      | create-register-row-flow-requirement             | 📋️register-row/🌱️create/🌊️flow-creates-a |
      | delete-register-row-flow-requirement             | 📋️register-row/🗑️delete/🌊️flow-deletes-a |
      | rename-register-row-flow-requirement             | 📋️register-row/🏷️rename/🌊️flow-renames-a |
      | replace-register-row-flow-requirement            | 📋️register-row/♻️replace/🌊️flow-replaces-a |
      | create-register-row-environmental-requirement    | 📋️register-row/🌱️create/🌿️environmental-creates-a |
      | delete-register-row-environmental-requirement    | 📋️register-row/🗑️delete/🌿️environmental-deletes-a |
      | rename-register-row-environmental-requirement    | 📋️register-row/🏷️rename/🌿️environmental-renames-a |
      | replace-register-row-environmental-requirement   | 📋️register-row/♻️replace/🌿️environmental-replaces-a |
      | create-register-row-workshop                     | 📋️register-row/🌱️create/🎓️workshop-creates-a |
      | delete-register-row-workshop                     | 📋️register-row/🗑️delete/🎓️workshop-deletes-a |
      | rename-register-row-workshop                     | 📋️register-row/🏷️rename/🎓️workshop-renames-a |
      | replace-register-row-workshop                    | 📋️register-row/♻️replace/🎓️workshop-replaces-a |
      | create-register-row-scenario                     | 📋️register-row/🌱️create/🎬️scenario-creates-a |
      | delete-register-row-scenario                     | 📋️register-row/🗑️delete/🎬️scenario-deletes-a |
      | rename-register-row-scenario                     | 📋️register-row/🏷️rename/🎬️scenario-renames-a |
      | replace-register-row-scenario                    | 📋️register-row/♻️replace/🎬️scenario-replaces-a |
      | create-register-row-benchmark-record             | 📋️register-row/🌱️create/🏁️benchmark-record-creates-a |
      | delete-register-row-benchmark-record             | 📋️register-row/🗑️delete/🏁️benchmark-record-absent-a |
      | rename-register-row-benchmark-record             | 📋️register-row/🏷️rename/🏁️benchmark-record-absent-a |
      | replace-register-row-benchmark-record            | 📋️register-row/♻️replace/🏁️benchmark-record-absent-a |
      | create-register-row-activity                     | 📋️register-row/🌱️create/🏃️activity-creates-a |
      | delete-register-row-activity                     | 📋️register-row/🗑️delete/🏃️activity-deletes-a |
      | rename-register-row-activity                     | 📋️register-row/🏷️rename/🏃️activity-renames-a |
      | replace-register-row-activity                    | 📋️register-row/♻️replace/🏃️activity-replaces-a |
      | create-register-row-infrastructure-requirement   | 📋️register-row/🌱️create/🏗️infrastructure-creates-a |
      | delete-register-row-infrastructure-requirement   | 📋️register-row/🗑️delete/🏗️infrastructure-deletes-a |
      | rename-register-row-infrastructure-requirement   | 📋️register-row/🏷️rename/🏗️infrastructure-renames-a |
      | replace-register-row-infrastructure-requirement  | 📋️register-row/♻️replace/🏗️infrastructure-replaces-a |
      | create-register-row-organizational-requirement   | 📋️register-row/🌱️create/🏢️organizational-creates-a |
      | delete-register-row-organizational-requirement   | 📋️register-row/🗑️delete/🏢️organizational-deletes-a |
      | rename-register-row-organizational-requirement   | 📋️register-row/🏷️rename/🏢️organizational-renames-a |
      | replace-register-row-organizational-requirement  | 📋️register-row/♻️replace/🏢️organizational-replaces-a |
      | create-register-row-issue                        | 📋️register-row/🌱️create/🐛️issue-creates |
      | delete-register-row-issue                        | 📋️register-row/🗑️delete/🐛️issue-deletes |
      | rename-register-row-issue                        | 📋️register-row/🏷️rename/🐛️issue-renames |
      | replace-register-row-issue                       | 📋️register-row/♻️replace/🐛️issue-replaces |
      | create-register-row-approval-record              | 📋️register-row/🌱️create/👍️approval-record-creates-a |
      | delete-register-row-approval-record              | 📋️register-row/🗑️delete/👍️approval-record-deletes-a |
      | rename-register-row-approval-record              | 📋️register-row/🏷️rename/👍️approval-record-renames-a |
      | replace-register-row-approval-record             | 📋️register-row/♻️replace/👍️approval-record-replaces-a |
      | create-register-row-stakeholder                  | 📋️register-row/🌱️create/👥️stakeholder-creates-a |
      | delete-register-row-stakeholder                  | 📋️register-row/🗑️delete/👥️stakeholder-deletes-a |
      | rename-register-row-stakeholder                  | 📋️register-row/🏷️rename/👥️stakeholder-renames-a |
      | replace-register-row-stakeholder                 | 📋️register-row/♻️replace/👥️stakeholder-replaces-a |
      | create-register-row-quality-record               | 📋️register-row/🌱️create/💎️quality-record-creates-a |
      | delete-register-row-quality-record               | 📋️register-row/🗑️delete/💎️quality-record-deletes-a |
      | rename-register-row-quality-record               | 📋️register-row/🏷️rename/💎️quality-record-renames-a |
      | replace-register-row-quality-record              | 📋️register-row/♻️replace/💎️quality-record-replaces-a |
      | create-register-row-resilience-requirement       | 📋️register-row/🌱️create/💪️resilience-creates-a |
      | delete-register-row-resilience-requirement       | 📋️register-row/🗑️delete/💪️resilience-deletes-a |
      | rename-register-row-resilience-requirement       | 📋️register-row/🏷️rename/💪️resilience-renames-a |
      | replace-register-row-resilience-requirement      | 📋️register-row/♻️replace/💪️resilience-replaces-a |
      | create-register-row-assumption                   | 📋️register-row/🌱️create/💭️assumption-creates-a |
      | delete-register-row-assumption                   | 📋️register-row/🗑️delete/💭️assumption-deletes-a |
      | rename-register-row-assumption                   | 📋️register-row/🏷️rename/💭️assumption-renames-a |
      | replace-register-row-assumption                  | 📋️register-row/♻️replace/💭️assumption-replaces-a |
      | create-register-row-cost-requirement             | 📋️register-row/🌱️create/💰️cost-creates-a |
      | delete-register-row-cost-requirement             | 📋️register-row/🗑️delete/💰️cost-deletes-a |
      | rename-register-row-cost-requirement             | 📋️register-row/🏷️rename/💰️cost-renames-a |
      | replace-register-row-cost-requirement            | 📋️register-row/♻️replace/💰️cost-replaces-a |
      | create-register-row-document                     | 📋️register-row/🌱️create/📃️document-creates-a |
      | delete-register-row-document                     | 📋️register-row/🗑️delete/📃️document-deletes-a |
      | rename-register-row-document                     | 📋️register-row/🏷️rename/📃️document-renames-a |
      | replace-register-row-document                    | 📋️register-row/♻️replace/📃️document-replaces-a |
      | create-register-row-schedule-requirement         | 📋️register-row/🌱️create/📅️schedule-creates-a |
      | delete-register-row-schedule-requirement         | 📋️register-row/🗑️delete/📅️schedule-deletes-a |
      | rename-register-row-schedule-requirement         | 📋️register-row/🏷️rename/📅️schedule-renames-a |
      | replace-register-row-schedule-requirement        | 📋️register-row/♻️replace/📅️schedule-replaces |
      | create-register-row-growth-plan                  | 📋️register-row/🌱️create/📈️growth-plan-creates-a |
      | delete-register-row-growth-plan                  | 📋️register-row/🗑️delete/📈️growth-plan-deletes-a |
      | rename-register-row-growth-plan                  | 📋️register-row/🏷️rename/📈️growth-plan-renames-a |
      | replace-register-row-growth-plan                 | 📋️register-row/♻️replace/📈️growth-plan-replaces-a |
      | create-register-row-performance-criterion        | 📋️register-row/🌱️create/📊️performance-creates-a |
      | delete-register-row-performance-criterion        | 📋️register-row/🗑️delete/📊️performance-deletes-a |
      | rename-register-row-performance-criterion        | 📋️register-row/🏷️rename/📊️performance-renames-a |
      | replace-register-row-performance-criterion       | 📋️register-row/♻️replace/📊️performance-replaces-a |
      | create-register-row-operational-requirement      | 📋️register-row/🌱️create/📋️operational-creates-a |
      | delete-register-row-operational-requirement      | 📋️register-row/🗑️delete/📋️operational-deletes-a |
      | rename-register-row-operational-requirement      | 📋️register-row/🏷️rename/📋️operational-renames-a |
      | replace-register-row-operational-requirement     | 📋️register-row/♻️replace/📋️operational-replaces-a |
      | create-register-row-requirement                  | 📋️register-row/🌱️create/📌️requirement-creates-a |
      | delete-register-row-requirement                  | 📋️register-row/🗑️delete/📌️requirement-deletes-a |
      | rename-register-row-requirement                  | 📋️register-row/🏷️rename/📌️requirement-renames-a |
      | replace-register-row-requirement                 | 📋️register-row/♻️replace/📌️requirement-replaces-a |
      | create-register-row-site-context                 | 📋️register-row/🌱️create/📍️site-creates-a |
      | delete-register-row-site-context                 | 📋️register-row/🗑️delete/📍️site-deletes-a |
      | rename-register-row-site-context                 | 📋️register-row/🏷️rename/📍️site-renames-a |
      | replace-register-row-site-context                | 📋️register-row/♻️replace/📍️site-replaces-a |
      | create-register-row-template-record              | 📋️register-row/🌱️create/📐️template-record-creates-a |
      | delete-register-row-template-record              | 📋️register-row/🗑️delete/📐️template-record-deletes-a |
      | rename-register-row-template-record              | 📋️register-row/🏷️rename/📐️template-record-renames-a |
      | replace-register-row-template-record             | 📋️register-row/♻️replace/📐️template-record-replaces-a |
      | create-register-row-report-record                | 📋️register-row/🌱️create/📑️report-record-creates-a |
      | delete-register-row-report-record                | 📋️register-row/🗑️delete/📑️report-record-deletes-a |
      | rename-register-row-report-record                | 📋️register-row/🏷️rename/📑️report-record-renames-a |
      | replace-register-row-report-record               | 📋️register-row/♻️replace/📑️report-record-replaces-a |
      | create-register-row-audit-event                  | 📋️register-row/🌱️create/📒️audit-creates |
      | delete-register-row-audit-event                  | 📋️register-row/🗑️delete/📒️audit-deletes |
      | rename-register-row-audit-event                  | 📋️register-row/🏷️rename/📒️audit-renames |
      | replace-register-row-audit-event                 | 📋️register-row/♻️replace/📒️audit-replaces |
      | create-register-row-knowledge-record             | 📋️register-row/🌱️create/📚️knowledge-record-creates-a |
      | delete-register-row-knowledge-record             | 📋️register-row/🗑️delete/📚️knowledge-record-absent-a |
      | rename-register-row-knowledge-record             | 📋️register-row/🏷️rename/📚️knowledge-record-absent-a |
      | replace-register-row-knowledge-record            | 📋️register-row/♻️replace/📚️knowledge-record-absent-a |
      | create-register-row-regulatory-requirement       | 📋️register-row/🌱️create/📜️regulatory-creates-a |
      | delete-register-row-regulatory-requirement       | 📋️register-row/🗑️delete/📜️regulatory-deletes-a |
      | rename-register-row-regulatory-requirement       | 📋️register-row/🏷️rename/📜️regulatory-renames-a |
      | replace-register-row-regulatory-requirement      | 📋️register-row/♻️replace/📜️regulatory-replaces-a |
      | create-register-row-communication-requirement    | 📋️register-row/🌱️create/📡️communication-creates-a |
      | delete-register-row-communication-requirement    | 📋️register-row/🗑️delete/📡️communication-deletes-a |
      | rename-register-row-communication-requirement    | 📋️register-row/🏷️rename/📡️communication-renames-a |
      | replace-register-row-communication-requirement   | 📋️register-row/♻️replace/📡️communication-replaces-a |
      | create-register-row-resource                     | 📋️register-row/🌱️create/📦️resource-creates-a |
      | delete-register-row-resource                     | 📋️register-row/🗑️delete/📦️resource-deletes-a |
      | rename-register-row-resource                     | 📋️register-row/🏷️rename/📦️resource-renames-a |
      | replace-register-row-resource                    | 📋️register-row/♻️replace/📦️resource-replaces-a |
      | create-register-row-status-record                | 📋️register-row/🌱️create/📶️status-record-creates-a |
      | delete-register-row-status-record                | 📋️register-row/🗑️delete/📶️status-record-deletes-a |
      | rename-register-row-status-record                | 📋️register-row/🏷️rename/📶️status-record-renames-a |
      | replace-register-row-status-record               | 📋️register-row/♻️replace/📶️status-record-replaces-a |
      | create-register-row-change-record                | 📋️register-row/🌱️create/🔀️change-record-creates-a |
      | delete-register-row-change-record                | 📋️register-row/🗑️delete/🔀️change-record-deletes-a |
      | rename-register-row-change-record                | 📋️register-row/🏷️rename/🔀️change-record-renames-a |
      | replace-register-row-change-record               | 📋️register-row/♻️replace/🔀️change-record-replaces-a |
      | create-register-row-process                      | 📋️register-row/🌱️create/🔄️process-creates |
      | delete-register-row-process                      | 📋️register-row/🗑️delete/🔄️process-deletes |
      | rename-register-row-process                      | 📋️register-row/🏷️rename/🔄️process-renames |
      | replace-register-row-process                     | 📋️register-row/♻️replace/🔄️process-replaces |
      | create-register-row-search-filter                | 📋️register-row/🌱️create/🔍️search-filter-creates-a |
      | delete-register-row-search-filter                | 📋️register-row/🗑️delete/🔍️search-filter-deletes-a |
      | rename-register-row-search-filter                | 📋️register-row/🏷️rename/🔍️search-filter-renames-a |
      | replace-register-row-search-filter               | 📋️register-row/♻️replace/🔍️search-filter-replaces-a |
      | create-register-row-access-rule                  | 📋️register-row/🌱️create/🔑️access-rule-creates-a |
      | delete-register-row-access-rule                  | 📋️register-row/🗑️delete/🔑️access-rule-deletes-a |
      | rename-register-row-access-rule                  | 📋️register-row/🏷️rename/🔑️access-rule-renames-a |
      | replace-register-row-access-rule                 | 📋️register-row/♻️replace/🔑️access-rule-replaces-a |
      | create-register-row-privacy-requirement          | 📋️register-row/🌱️create/🔒️privacy-creates |
      | delete-register-row-privacy-requirement          | 📋️register-row/🗑️delete/🔒️privacy-deletes |
      | rename-register-row-privacy-requirement          | 📋️register-row/🏷️rename/🔒️privacy-renames |
      | replace-register-row-privacy-requirement         | 📋️register-row/♻️replace/🔒️privacy-replaces |
      | create-register-row-quantity-requirement         | 📋️register-row/🌱️create/🔢️quantity-creates-a |
      | delete-register-row-quantity-requirement         | 📋️register-row/🗑️delete/🔢️quantity-deletes-a |
      | rename-register-row-quantity-requirement         | 📋️register-row/🏷️rename/🔢️quantity-renames-a |
      | replace-register-row-quantity-requirement        | 📋️register-row/♻️replace/🔢️quantity-replaces |
      | create-register-row-analysis-record              | 📋️register-row/🌱️create/🔬️analysis-record-creates-a |
      | delete-register-row-analysis-record              | 📋️register-row/🗑️delete/🔬️analysis-record-deletes-a |
      | rename-register-row-analysis-record              | 📋️register-row/🏷️rename/🔬️analysis-record-renames-a |
      | replace-register-row-analysis-record             | 📋️register-row/♻️replace/🔬️analysis-record-replaces-a |
      | create-register-row-relationship                 | 📋️register-row/🌱️create/🕸️relationship-creates-a |
      | delete-register-row-relationship                 | 📋️register-row/🗑️delete/🕸️relationship-deletes-a |
      | rename-register-row-relationship                 | 📋️register-row/🏷️rename/🕸️relationship-renames-a |
      | replace-register-row-relationship                | 📋️register-row/♻️replace/🕸️relationship-replaces-a |
      | create-register-row-storage-requirement          | 📋️register-row/🌱️create/🗄️storage-creates |
      | delete-register-row-storage-requirement          | 📋️register-row/🗑️delete/🗄️storage-deletes |
      | rename-register-row-storage-requirement          | 📋️register-row/🏷️rename/🗄️storage-renames |
      | replace-register-row-storage-requirement         | 📋️register-row/♻️replace/🗄️storage-replaces |
      | create-register-row-meeting-record               | 📋️register-row/🌱️create/🗓️meeting-record-creates-a |
      | delete-register-row-meeting-record               | 📋️register-row/🗑️delete/🗓️meeting-record-deletes-a |
      | rename-register-row-meeting-record               | 📋️register-row/🏷️rename/🗓️meeting-record-renames-a |
      | replace-register-row-meeting-record              | 📋️register-row/♻️replace/🗓️meeting-record-replaces-a |
      | create-register-row-survey                       | 📋️register-row/🌱️create/🗳️survey-creates |
      | delete-register-row-survey                       | 📋️register-row/🗑️delete/🗳️survey-deletes |
      | rename-register-row-survey                       | 📋️register-row/🏷️rename/🗳️survey-renames |
      | replace-register-row-survey                      | 📋️register-row/♻️replace/🗳️survey-replaces |
      | create-register-row-delivery-constraint          | 📋️register-row/🌱️create/🚚️delivery-constraint-creates |
      | delete-register-row-delivery-constraint          | 📋️register-row/🗑️delete/🚚️delivery-constraint-deletes |
      | rename-register-row-delivery-constraint          | 📋️register-row/🏷️rename/🚚️delivery-constraint-renames |
      | replace-register-row-delivery-constraint         | 📋️register-row/♻️replace/🚚️delivery-constraint-replaces |
      | create-register-row-constraint-record            | 📋️register-row/🌱️create/🚧️constraint-record-creates-a |
      | delete-register-row-constraint-record            | 📋️register-row/🗑️delete/🚧️constraint-record-deletes-a |
      | rename-register-row-constraint-record            | 📋️register-row/🏷️rename/🚧️constraint-record-renames-a |
      | replace-register-row-constraint-record           | 📋️register-row/♻️replace/🚧️constraint-record-replaces-a |
      | create-register-row-compliance-record            | 📋️register-row/🌱️create/🛂️compliance-record-creates-a |
      | delete-register-row-compliance-record            | 📋️register-row/🗑️delete/🛂️compliance-record-deletes-a |
      | rename-register-row-compliance-record            | 📋️register-row/🏷️rename/🛂️compliance-record-renames-a |
      | replace-register-row-compliance-record           | 📋️register-row/♻️replace/🛂️compliance-record-replaces-a |
      | create-register-row-service-requirement          | 📋️register-row/🌱️create/🛎️service-creates |
      | delete-register-row-service-requirement          | 📋️register-row/🗑️delete/🛎️service-deletes |
      | rename-register-row-service-requirement          | 📋️register-row/🏷️rename/🛎️service-renames |
      | replace-register-row-service-requirement         | 📋️register-row/♻️replace/🛎️service-replaces |
      | create-register-row-equipment                    | 📋️register-row/🌱️create/🛠️equipment-creates-a |
      | delete-register-row-equipment                    | 📋️register-row/🗑️delete/🛠️equipment-deletes-a |
      | rename-register-row-equipment                    | 📋️register-row/🏷️rename/🛠️equipment-renames-a |
      | replace-register-row-equipment                   | 📋️register-row/♻️replace/🛠️equipment-replaces-a |
      | create-register-row-security-requirement         | 📋️register-row/🌱️create/🛡️security-creates-a |
      | delete-register-row-security-requirement         | 📋️register-row/🗑️delete/🛡️security-deletes-a |
      | rename-register-row-security-requirement         | 📋️register-row/🏷️rename/🛡️security-renames-a |
      | replace-register-row-security-requirement        | 📋️register-row/♻️replace/🛡️security-replaces |
      | create-register-row-collaboration-record         | 📋️register-row/🌱️create/🤝️collaboration-creates-a |
      | delete-register-row-collaboration-record         | 📋️register-row/🗑️delete/🤝️collaboration-deletes-a |
      | rename-register-row-collaboration-record         | 📋️register-row/🏷️rename/🤝️collaboration-renames-a |
      | replace-register-row-collaboration-record        | 📋️register-row/♻️replace/🤝️collaboration-replaces |
      | create-register-row-safety-requirement           | 📋️register-row/🌱️create/🦺️safety-creates |
      | delete-register-row-safety-requirement           | 📋️register-row/🗑️delete/🦺️safety-deletes |
      | rename-register-row-safety-requirement           | 📋️register-row/🏷️rename/🦺️safety-renames |
      | replace-register-row-safety-requirement          | 📋️register-row/♻️replace/🦺️safety-replaces |
      | create-register-row-user-profile                 | 📋️register-row/🌱️create/🧑️user-creates-a |
      | delete-register-row-user-profile                 | 📋️register-row/🗑️delete/🧑️user-deletes-a |
      | rename-register-row-user-profile                 | 📋️register-row/🏷️rename/🧑️user-renames-a |
      | replace-register-row-user-profile                | 📋️register-row/♻️replace/🧑️user-replaces-a |
      | create-register-row-human-factor-requirement     | 📋️register-row/🌱️create/🧠️human-creates |
      | delete-register-row-human-factor-requirement     | 📋️register-row/🗑️delete/🧠️human-deletes |
      | rename-register-row-human-factor-requirement     | 📋️register-row/🏷️rename/🧠️human-renames |
      | replace-register-row-human-factor-requirement    | 📋️register-row/♻️replace/🧠️human-replaces |
      | create-register-row-flexibility-requirement      | 📋️register-row/🌱️create/🧩️flexibility-creates-a |
      | delete-register-row-flexibility-requirement      | 📋️register-row/🗑️delete/🧩️flexibility-deletes-a |
      | rename-register-row-flexibility-requirement      | 📋️register-row/🏷️rename/🧩️flexibility-renames-a |
      | replace-register-row-flexibility-requirement     | 📋️register-row/♻️replace/🧩️flexibility-replaces-a |
      | create-register-row-wayfinding-requirement       | 📋️register-row/🌱️create/🧭️wayfinding-creates-a |
      | delete-register-row-wayfinding-requirement       | 📋️register-row/🗑️delete/🧭️wayfinding-deletes-a |
      | rename-register-row-wayfinding-requirement       | 📋️register-row/🏷️rename/🧭️wayfinding-renames-a |
      | replace-register-row-wayfinding-requirement      | 📋️register-row/♻️replace/🧭️wayfinding-replaces-a |
      | create-register-row-program-element              | 📋️register-row/🌱️create/🧱️program-element-creates-a |
      | delete-register-row-program-element              | 📋️register-row/🗑️delete/🧱️program-element-deletes-a |
      | rename-register-row-program-element              | 📋️register-row/🏷️rename/🧱️program-element-renames-a |
      | replace-register-row-program-element             | 📋️register-row/♻️replace/🧱️program-element-replaces-a |
      | connect-adjacency                                | 🧲️adjacency/🧲️connect/🧲️reception |
      | disconnect-adjacency                             | 🧲️adjacency/🫷️disconnect/🫷️reception |
      | connect-trace                                    | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                                 | 🧵️trace/✂️disconnect/✂️requirement |
      | rename-meta                                      | 🏷️meta/🏷️rename/🏷️title |
      | replace-meta                                     | 🏷️meta/♻️replace/♻️block |
      | rename-project                                   | 🏙️project/🏷️rename/🏷️code |
      | replace-project                                  | 🏙️project/♻️replace/♻️definition |
      | rename-governance                                | 🏛️governance/🏷️rename/🏷️framework |
      | replace-governance                               | 🏛️governance/♻️replace/♻️block |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🎬️demo/🗣️.dsl.semio
    When the artifact is parsed to a ProgramSnapshot, printed back to `.architect` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
