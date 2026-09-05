@capability-program-1-mutate
@oracle-architect-program-zip-reader
@oracle-input-subject-raw
@comparison-ordered-json-v1
@mutations-program-1-any
Feature: Export every typed architect program mutation through ZIP and decode it independently
  This case applies all 266 typed mutations through the Rust subject, checks each result against its
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
  `delete`/`rename`/`replace` over `knowledge-record` and `benchmark-record`, and the reason is
  structural rather than an authoring gap: those two registers alone are composed
  `s.stdio.semio.table` CHILD handles whose rows live in a working-scene cache a fresh process has
  never populated, so the only branch reachable from a committed snapshot is the
  `mutation.target-missing` rejection — which is exactly what those six vectors pin. They are named
  in the subject adapter's `GUARD_VECTORS` list and exempted from the observability law on that basis; the
  other 260 kinds carry it with no exemption.

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them. The subject additionally asserts that every non-guard
  mutation changes the ZIP carrier projection, so an exporter that returns a valid but invariant
  archive cannot satisfy the feature.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                                 | vector                                                                              |
      | create-information-requirement     | ℹ️information-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-information-requirement     | ℹ️information-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-information-requirement     | ℹ️information-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-information-requirement    | ℹ️information-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-sustainability-requirement  | ♻️sustainability-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-sustainability-requirement  | ♻️sustainability-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-sustainability-requirement  | ♻️sustainability-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-sustainability-requirement | ♻️sustainability-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-accessibility-requirement   | ♿️accessibility-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-accessibility-requirement   | ♿️accessibility-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-accessibility-requirement   | ♿️accessibility-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-accessibility-requirement  | ♿️accessibility-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-conflict                    | ⚔️conflict/🌱️create/🧪️tests/🌱️creates-a |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-conflict                   | ⚔️conflict/♻️replace/🧪️tests/♻️replaces-a |
      | create-option-evaluation           | ⚖️option-evaluation/🌱️create/🧪️tests/🌱️creates-a |
      | delete-option-evaluation           | ⚖️option-evaluation/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-option-evaluation           | ⚖️option-evaluation/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-option-evaluation          | ⚖️option-evaluation/♻️replace/🧪️tests/♻️replaces-a |
      | create-function                    | ⚙️function/🌱️create/🧪️tests/🌱️creates-a |
      | delete-function                    | ⚙️function/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-function                    | ⚙️function/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-function                   | ⚙️function/♻️replace/🧪️tests/♻️replaces-a |
      | create-risk                        | ⚠️risk/🌱️create/🧪️tests/🌱️creates-a |
      | delete-risk                        | ⚠️risk/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-risk                        | ⚠️risk/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-risk                       | ⚠️risk/♻️replace/🧪️tests/♻️replaces-a |
      | create-decision                    | ✅️decision/🌱️create/🧪️tests/🌱️creates-a |
      | delete-decision                    | ✅️decision/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-decision                    | ✅️decision/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-decision                   | ✅️decision/♻️replace/🧪️tests/♻️replaces-a |
      | create-validation-record           | ✔️validation-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-validation-record          | ✔️validation-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-priority-record             | ⭐️priority-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-priority-record            | ⭐️priority-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-flow-requirement            | 🌊️flow-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-flow-requirement            | 🌊️flow-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-flow-requirement            | 🌊️flow-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-flow-requirement           | 🌊️flow-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-environmental-requirement   | 🌿️environmental-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-environmental-requirement   | 🌿️environmental-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-environmental-requirement   | 🌿️environmental-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-environmental-requirement  | 🌿️environmental-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-workshop                    | 🎓️workshop/🌱️create/🧪️tests/🌱️creates-a |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-workshop                   | 🎓️workshop/♻️replace/🧪️tests/♻️replaces-a |
      | create-scenario                    | 🎬️scenario/🌱️create/🧪️tests/🌱️creates-a |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-scenario                   | 🎬️scenario/♻️replace/🧪️tests/♻️replaces-a |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🧪️tests/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🧪️tests/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🧪️tests/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🧪️tests/🌱️creates-a                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🧪️tests/🗑️deletes-a                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🧪️tests/🏷️renames-a                                            |
      | replace-activity                   | 🏃️activity/♻️replace/🧪️tests/♻️replaces-a                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-infrastructure-requirement  | 🏗️infrastructure-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-infrastructure-requirement  | 🏗️infrastructure-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-infrastructure-requirement | 🏗️infrastructure-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-organizational-requirement  | 🏢️organizational-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-organizational-requirement  | 🏢️organizational-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-organizational-requirement  | 🏢️organizational-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-organizational-requirement | 🏢️organizational-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-issue                       | 🐛️issue/🌱️create/🧪️tests/🌱️creates-a |
      | delete-issue                       | 🐛️issue/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-issue                       | 🐛️issue/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-issue                      | 🐛️issue/♻️replace/🧪️tests/♻️replaces-a |
      | create-approval-record             | 👍️approval-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-approval-record            | 👍️approval-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🧪️tests/🌱️creates-a |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/🧪️tests/♻️replaces-a |
      | create-quality-record              | 💎️quality-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-quality-record             | 💎️quality-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-resilience-requirement      | 💪️resilience-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-resilience-requirement      | 💪️resilience-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-resilience-requirement      | 💪️resilience-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-resilience-requirement     | 💪️resilience-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-assumption                  | 💭️assumption/🌱️create/🧪️tests/🌱️creates-a |
      | delete-assumption                  | 💭️assumption/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-assumption                  | 💭️assumption/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-assumption                 | 💭️assumption/♻️replace/🧪️tests/♻️replaces-a |
      | create-cost-requirement            | 💰️cost-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-cost-requirement            | 💰️cost-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-cost-requirement            | 💰️cost-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-cost-requirement           | 💰️cost-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-document                    | 📃️document/🌱️create/🧪️tests/🌱️creates-a |
      | delete-document                    | 📃️document/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-document                    | 📃️document/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-document                   | 📃️document/♻️replace/🧪️tests/♻️replaces-a |
      | create-schedule-requirement        | 📅️schedule-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-schedule-requirement        | 📅️schedule-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-schedule-requirement        | 📅️schedule-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-schedule-requirement       | 📅️schedule-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🧪️tests/🌱️creates-a |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/🧪️tests/♻️replaces-a |
      | create-performance-criterion       | 📊️performance-criterion/🌱️create/🧪️tests/🌱️creates-a |
      | delete-performance-criterion       | 📊️performance-criterion/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-performance-criterion       | 📊️performance-criterion/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-performance-criterion      | 📊️performance-criterion/♻️replace/🧪️tests/♻️replaces-a |
      | create-operational-requirement     | 📋️operational-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-operational-requirement     | 📋️operational-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-operational-requirement     | 📋️operational-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-operational-requirement    | 📋️operational-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-requirement                 | 📌️requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-requirement                 | 📌️requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-requirement                 | 📌️requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-requirement                | 📌️requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-site-context                | 📍️site-context/🌱️create/🧪️tests/🌱️creates-a |
      | delete-site-context                | 📍️site-context/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-site-context                | 📍️site-context/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-site-context               | 📍️site-context/♻️replace/🧪️tests/♻️replaces-a |
      | create-template-record             | 📐️template-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-template-record             | 📐️template-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-template-record             | 📐️template-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-template-record            | 📐️template-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-report-record               | 📑️report-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-report-record               | 📑️report-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-report-record               | 📑️report-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-report-record              | 📑️report-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-audit-event                 | 📒️audit-event/🌱️create/🧪️tests/🌱️creates-a |
      | delete-audit-event                 | 📒️audit-event/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-audit-event                 | 📒️audit-event/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-audit-event                | 📒️audit-event/♻️replace/🧪️tests/♻️replaces-a |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🧪️tests/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🧪️tests/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🧪️tests/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-regulatory-requirement      | 📜️regulatory-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-regulatory-requirement      | 📜️regulatory-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-regulatory-requirement     | 📜️regulatory-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-change-record               | 🔀️change-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-change-record               | 🔀️change-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-change-record               | 🔀️change-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-change-record              | 🔀️change-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-communication-requirement   | 📡️communication-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-communication-requirement   | 📡️communication-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-communication-requirement   | 📡️communication-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-communication-requirement  | 📡️communication-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-resource                    | 📦️resource/🌱️create/🧪️tests/🌱️creates-a |
      | delete-resource                    | 📦️resource/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-resource                    | 📦️resource/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-resource                   | 📦️resource/♻️replace/🧪️tests/♻️replaces-a |
      | create-status-record               | 📶️status-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-status-record               | 📶️status-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-status-record               | 📶️status-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-status-record              | 📶️status-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-process                     | 🔄️process/🌱️create/🧪️tests/🌱️creates-a |
      | delete-process                     | 🔄️process/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-process                     | 🔄️process/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-process                    | 🔄️process/♻️replace/🧪️tests/♻️replaces-a |
      | create-search-filter               | 🔍️search-filter/🌱️create/🧪️tests/🌱️creates-a |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-search-filter              | 🔍️search-filter/♻️replace/🧪️tests/♻️replaces-a |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🧪️tests/🌱️creates-a |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-access-rule                | 🔑️access-rule/♻️replace/🧪️tests/♻️replaces-a |
      | create-privacy-requirement         | 🔒️privacy-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-privacy-requirement         | 🔒️privacy-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-privacy-requirement         | 🔒️privacy-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-privacy-requirement        | 🔒️privacy-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-relationship                | 🕸️relationship/🌱️create/🧪️tests/🌱️creates-a |
      | delete-relationship                | 🕸️relationship/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-relationship                | 🕸️relationship/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-relationship               | 🕸️relationship/♻️replace/🧪️tests/♻️replaces-a |
      | create-quantity-requirement        | 🔢️quantity-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-quantity-requirement        | 🔢️quantity-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-quantity-requirement        | 🔢️quantity-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-quantity-requirement       | 🔢️quantity-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-storage-requirement         | 🗄️storage-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-storage-requirement         | 🗄️storage-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-storage-requirement         | 🗄️storage-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-storage-requirement        | 🗄️storage-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-survey                      | 🗳️survey/🌱️create/🧪️tests/🌱️creates-a |
      | delete-survey                      | 🗳️survey/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-survey                      | 🗳️survey/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-survey                     | 🗳️survey/♻️replace/🧪️tests/♻️replaces-a |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🧪️tests/🌱️creates-a |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/🧪️tests/♻️replaces-a |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-service-requirement         | 🛎️service-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-service-requirement         | 🛎️service-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-service-requirement         | 🛎️service-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-service-requirement        | 🛎️service-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-equipment                   | 🛠️equipment/🌱️create/🧪️tests/🌱️creates-a |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-equipment                  | 🛠️equipment/♻️replace/🧪️tests/♻️replaces-a |
      | create-security-requirement        | 🛡️security-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-security-requirement        | 🛡️security-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-security-requirement        | 🛡️security-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-security-requirement       | 🛡️security-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-collaboration-record        | 🤝️collaboration-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-collaboration-record        | 🤝️collaboration-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-collaboration-record        | 🤝️collaboration-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-collaboration-record       | 🤝️collaboration-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-safety-requirement          | 🦺️safety-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-safety-requirement          | 🦺️safety-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-safety-requirement          | 🦺️safety-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-safety-requirement         | 🦺️safety-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-user-profile                | 🧑️user-profile/🌱️create/🧪️tests/🌱️creates-a |
      | delete-user-profile                | 🧑️user-profile/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-user-profile                | 🧑️user-profile/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-user-profile               | 🧑️user-profile/♻️replace/🧪️tests/♻️replaces-a |
      | create-human-factor-requirement    | 🧠️human-factor-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-human-factor-requirement    | 🧠️human-factor-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-human-factor-requirement    | 🧠️human-factor-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-human-factor-requirement   | 🧠️human-factor-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-flexibility-requirement     | 🧩️flexibility-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-flexibility-requirement     | 🧩️flexibility-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-flexibility-requirement     | 🧩️flexibility-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-flexibility-requirement    | 🧩️flexibility-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-wayfinding-requirement      | 🧭️wayfinding-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-wayfinding-requirement      | 🧭️wayfinding-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-wayfinding-requirement      | 🧭️wayfinding-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-wayfinding-requirement     | 🧭️wayfinding-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-program-element             | 🧱️program-element/🌱️create/🧪️tests/🌱️creates-a |
      | delete-program-element             | 🧱️program-element/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-program-element             | 🧱️program-element/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-program-element            | 🧱️program-element/♻️replace/🧪️tests/♻️replaces-a |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧪️tests/🧲️reception-waiting |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🧪️tests/🫷️reception-waiting |
      | connect-trace                      | 🧵️trace/🧵️connect/🧪️tests/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/🧪️tests/✂️requirement-decision |
      | rename-meta                       | 🏷️meta/🏷️rename/🧪️tests/🏷️title |
      | replace-meta                      | 🏷️meta/♻️replace/🧪️tests/♻️block |
      | rename-project                    | 🏙️project/🏷️rename/🧪️tests/🏷️code |
      | replace-project                   | 🏙️project/♻️replace/🧪️tests/♻️definition |
      | rename-governance                 | 🏛️governance/🏷️rename/🧪️tests/🏷️framework |
      | replace-governance                | 🏛️governance/♻️replace/🧪️tests/♻️block |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_program_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                                 | vector                                                                              |
      | create-information-requirement     | ℹ️information-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-information-requirement     | ℹ️information-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-information-requirement     | ℹ️information-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-information-requirement    | ℹ️information-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-sustainability-requirement  | ♻️sustainability-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-sustainability-requirement  | ♻️sustainability-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-sustainability-requirement  | ♻️sustainability-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-sustainability-requirement | ♻️sustainability-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-accessibility-requirement   | ♿️accessibility-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-accessibility-requirement   | ♿️accessibility-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-accessibility-requirement   | ♿️accessibility-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-accessibility-requirement  | ♿️accessibility-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-conflict                    | ⚔️conflict/🌱️create/🧪️tests/🌱️creates-a |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-conflict                   | ⚔️conflict/♻️replace/🧪️tests/♻️replaces-a |
      | create-option-evaluation           | ⚖️option-evaluation/🌱️create/🧪️tests/🌱️creates-a |
      | delete-option-evaluation           | ⚖️option-evaluation/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-option-evaluation           | ⚖️option-evaluation/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-option-evaluation          | ⚖️option-evaluation/♻️replace/🧪️tests/♻️replaces-a |
      | create-function                    | ⚙️function/🌱️create/🧪️tests/🌱️creates-a |
      | delete-function                    | ⚙️function/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-function                    | ⚙️function/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-function                   | ⚙️function/♻️replace/🧪️tests/♻️replaces-a |
      | create-risk                        | ⚠️risk/🌱️create/🧪️tests/🌱️creates-a |
      | delete-risk                        | ⚠️risk/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-risk                        | ⚠️risk/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-risk                       | ⚠️risk/♻️replace/🧪️tests/♻️replaces-a |
      | create-decision                    | ✅️decision/🌱️create/🧪️tests/🌱️creates-a |
      | delete-decision                    | ✅️decision/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-decision                    | ✅️decision/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-decision                   | ✅️decision/♻️replace/🧪️tests/♻️replaces-a |
      | create-validation-record           | ✔️validation-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-validation-record          | ✔️validation-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-priority-record             | ⭐️priority-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-priority-record            | ⭐️priority-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-flow-requirement            | 🌊️flow-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-flow-requirement            | 🌊️flow-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-flow-requirement            | 🌊️flow-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-flow-requirement           | 🌊️flow-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-environmental-requirement   | 🌿️environmental-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-environmental-requirement   | 🌿️environmental-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-environmental-requirement   | 🌿️environmental-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-environmental-requirement  | 🌿️environmental-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-workshop                    | 🎓️workshop/🌱️create/🧪️tests/🌱️creates-a |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-workshop                   | 🎓️workshop/♻️replace/🧪️tests/♻️replaces-a |
      | create-scenario                    | 🎬️scenario/🌱️create/🧪️tests/🌱️creates-a |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-scenario                   | 🎬️scenario/♻️replace/🧪️tests/♻️replaces-a |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🧪️tests/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🧪️tests/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🧪️tests/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🧪️tests/🌱️creates-a                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🧪️tests/🗑️deletes-a                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🧪️tests/🏷️renames-a                                            |
      | replace-activity                   | 🏃️activity/♻️replace/🧪️tests/♻️replaces-a                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-infrastructure-requirement  | 🏗️infrastructure-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-infrastructure-requirement  | 🏗️infrastructure-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-infrastructure-requirement | 🏗️infrastructure-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-organizational-requirement  | 🏢️organizational-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-organizational-requirement  | 🏢️organizational-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-organizational-requirement  | 🏢️organizational-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-organizational-requirement | 🏢️organizational-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-issue                       | 🐛️issue/🌱️create/🧪️tests/🌱️creates-a |
      | delete-issue                       | 🐛️issue/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-issue                       | 🐛️issue/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-issue                      | 🐛️issue/♻️replace/🧪️tests/♻️replaces-a |
      | create-approval-record             | 👍️approval-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-approval-record            | 👍️approval-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🧪️tests/🌱️creates-a |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/🧪️tests/♻️replaces-a |
      | create-quality-record              | 💎️quality-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-quality-record             | 💎️quality-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-resilience-requirement      | 💪️resilience-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-resilience-requirement      | 💪️resilience-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-resilience-requirement      | 💪️resilience-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-resilience-requirement     | 💪️resilience-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-assumption                  | 💭️assumption/🌱️create/🧪️tests/🌱️creates-a |
      | delete-assumption                  | 💭️assumption/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-assumption                  | 💭️assumption/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-assumption                 | 💭️assumption/♻️replace/🧪️tests/♻️replaces-a |
      | create-cost-requirement            | 💰️cost-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-cost-requirement            | 💰️cost-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-cost-requirement            | 💰️cost-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-cost-requirement           | 💰️cost-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-document                    | 📃️document/🌱️create/🧪️tests/🌱️creates-a |
      | delete-document                    | 📃️document/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-document                    | 📃️document/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-document                   | 📃️document/♻️replace/🧪️tests/♻️replaces-a |
      | create-schedule-requirement        | 📅️schedule-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-schedule-requirement        | 📅️schedule-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-schedule-requirement        | 📅️schedule-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-schedule-requirement       | 📅️schedule-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🧪️tests/🌱️creates-a |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/🧪️tests/♻️replaces-a |
      | create-performance-criterion       | 📊️performance-criterion/🌱️create/🧪️tests/🌱️creates-a |
      | delete-performance-criterion       | 📊️performance-criterion/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-performance-criterion       | 📊️performance-criterion/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-performance-criterion      | 📊️performance-criterion/♻️replace/🧪️tests/♻️replaces-a |
      | create-operational-requirement     | 📋️operational-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-operational-requirement     | 📋️operational-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-operational-requirement     | 📋️operational-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-operational-requirement    | 📋️operational-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-requirement                 | 📌️requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-requirement                 | 📌️requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-requirement                 | 📌️requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-requirement                | 📌️requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-site-context                | 📍️site-context/🌱️create/🧪️tests/🌱️creates-a |
      | delete-site-context                | 📍️site-context/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-site-context                | 📍️site-context/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-site-context               | 📍️site-context/♻️replace/🧪️tests/♻️replaces-a |
      | create-template-record             | 📐️template-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-template-record             | 📐️template-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-template-record             | 📐️template-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-template-record            | 📐️template-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-report-record               | 📑️report-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-report-record               | 📑️report-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-report-record               | 📑️report-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-report-record              | 📑️report-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-audit-event                 | 📒️audit-event/🌱️create/🧪️tests/🌱️creates-a |
      | delete-audit-event                 | 📒️audit-event/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-audit-event                 | 📒️audit-event/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-audit-event                | 📒️audit-event/♻️replace/🧪️tests/♻️replaces-a |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🧪️tests/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🧪️tests/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🧪️tests/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-regulatory-requirement      | 📜️regulatory-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-regulatory-requirement      | 📜️regulatory-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-regulatory-requirement     | 📜️regulatory-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-change-record               | 🔀️change-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-change-record               | 🔀️change-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-change-record               | 🔀️change-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-change-record              | 🔀️change-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-communication-requirement   | 📡️communication-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-communication-requirement   | 📡️communication-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-communication-requirement   | 📡️communication-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-communication-requirement  | 📡️communication-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-resource                    | 📦️resource/🌱️create/🧪️tests/🌱️creates-a |
      | delete-resource                    | 📦️resource/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-resource                    | 📦️resource/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-resource                   | 📦️resource/♻️replace/🧪️tests/♻️replaces-a |
      | create-status-record               | 📶️status-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-status-record               | 📶️status-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-status-record               | 📶️status-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-status-record              | 📶️status-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-process                     | 🔄️process/🌱️create/🧪️tests/🌱️creates-a |
      | delete-process                     | 🔄️process/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-process                     | 🔄️process/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-process                    | 🔄️process/♻️replace/🧪️tests/♻️replaces-a |
      | create-search-filter               | 🔍️search-filter/🌱️create/🧪️tests/🌱️creates-a |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-search-filter              | 🔍️search-filter/♻️replace/🧪️tests/♻️replaces-a |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🧪️tests/🌱️creates-a |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-access-rule                | 🔑️access-rule/♻️replace/🧪️tests/♻️replaces-a |
      | create-privacy-requirement         | 🔒️privacy-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-privacy-requirement         | 🔒️privacy-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-privacy-requirement         | 🔒️privacy-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-privacy-requirement        | 🔒️privacy-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-relationship                | 🕸️relationship/🌱️create/🧪️tests/🌱️creates-a |
      | delete-relationship                | 🕸️relationship/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-relationship                | 🕸️relationship/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-relationship               | 🕸️relationship/♻️replace/🧪️tests/♻️replaces-a |
      | create-quantity-requirement        | 🔢️quantity-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-quantity-requirement        | 🔢️quantity-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-quantity-requirement        | 🔢️quantity-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-quantity-requirement       | 🔢️quantity-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-storage-requirement         | 🗄️storage-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-storage-requirement         | 🗄️storage-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-storage-requirement         | 🗄️storage-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-storage-requirement        | 🗄️storage-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-survey                      | 🗳️survey/🌱️create/🧪️tests/🌱️creates-a |
      | delete-survey                      | 🗳️survey/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-survey                      | 🗳️survey/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-survey                     | 🗳️survey/♻️replace/🧪️tests/♻️replaces-a |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🧪️tests/🌱️creates-a |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/🧪️tests/♻️replaces-a |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-service-requirement         | 🛎️service-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-service-requirement         | 🛎️service-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-service-requirement         | 🛎️service-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-service-requirement        | 🛎️service-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-equipment                   | 🛠️equipment/🌱️create/🧪️tests/🌱️creates-a |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-equipment                  | 🛠️equipment/♻️replace/🧪️tests/♻️replaces-a |
      | create-security-requirement        | 🛡️security-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-security-requirement        | 🛡️security-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-security-requirement        | 🛡️security-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-security-requirement       | 🛡️security-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-collaboration-record        | 🤝️collaboration-record/🌱️create/🧪️tests/🌱️creates-a |
      | delete-collaboration-record        | 🤝️collaboration-record/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-collaboration-record        | 🤝️collaboration-record/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-collaboration-record       | 🤝️collaboration-record/♻️replace/🧪️tests/♻️replaces-a |
      | create-safety-requirement          | 🦺️safety-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-safety-requirement          | 🦺️safety-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-safety-requirement          | 🦺️safety-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-safety-requirement         | 🦺️safety-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-user-profile                | 🧑️user-profile/🌱️create/🧪️tests/🌱️creates-a |
      | delete-user-profile                | 🧑️user-profile/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-user-profile                | 🧑️user-profile/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-user-profile               | 🧑️user-profile/♻️replace/🧪️tests/♻️replaces-a |
      | create-human-factor-requirement    | 🧠️human-factor-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-human-factor-requirement    | 🧠️human-factor-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-human-factor-requirement    | 🧠️human-factor-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-human-factor-requirement   | 🧠️human-factor-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-flexibility-requirement     | 🧩️flexibility-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-flexibility-requirement     | 🧩️flexibility-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-flexibility-requirement     | 🧩️flexibility-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-flexibility-requirement    | 🧩️flexibility-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-wayfinding-requirement      | 🧭️wayfinding-requirement/🌱️create/🧪️tests/🌱️creates-a |
      | delete-wayfinding-requirement      | 🧭️wayfinding-requirement/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-wayfinding-requirement      | 🧭️wayfinding-requirement/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-wayfinding-requirement     | 🧭️wayfinding-requirement/♻️replace/🧪️tests/♻️replaces-a |
      | create-program-element             | 🧱️program-element/🌱️create/🧪️tests/🌱️creates-a |
      | delete-program-element             | 🧱️program-element/🗑️delete/🧪️tests/🗑️deletes-a |
      | rename-program-element             | 🧱️program-element/🏷️rename/🧪️tests/🏷️renames-a |
      | replace-program-element            | 🧱️program-element/♻️replace/🧪️tests/♻️replaces-a |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧪️tests/🧲️reception-waiting |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🧪️tests/🫷️reception-waiting |
      | connect-trace                      | 🧵️trace/🧵️connect/🧪️tests/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/🧪️tests/✂️requirement-decision |
      | rename-meta                       | 🏷️meta/🏷️rename/🧪️tests/🏷️title |
      | replace-meta                      | 🏷️meta/♻️replace/🧪️tests/♻️block |
      | rename-project                    | 🏙️project/🏷️rename/🧪️tests/🏷️code |
      | replace-project                   | 🏙️project/♻️replace/🧪️tests/♻️definition |
      | rename-governance                 | 🏛️governance/🏷️rename/🧪️tests/🏷️framework |
      | replace-governance                | 🏛️governance/♻️replace/🧪️tests/♻️block |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed to a ProgramSnapshot, printed back to `.architect` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
