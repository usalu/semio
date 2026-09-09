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
      | id                                 | vector                                                                              |
      | create-information-requirement     | ℹ️information-requirement/🌱️create/🌱️creates-a |
      | delete-information-requirement     | ℹ️information-requirement/🗑️delete/🗑️deletes-a |
      | rename-information-requirement     | ℹ️information-requirement/🏷️rename/🏷️renames-a |
      | replace-information-requirement    | ℹ️information-requirement/♻️replace/♻️replaces-a |
      | create-sustainability-requirement  | ♻️sustainability-requirement/🌱️create/🌱️creates-a |
      | delete-sustainability-requirement  | ♻️sustainability-requirement/🗑️delete/🗑️deletes-a |
      | rename-sustainability-requirement  | ♻️sustainability-requirement/🏷️rename/🏷️renames-a |
      | replace-sustainability-requirement | ♻️sustainability-requirement/♻️replace/♻️replaces-a |
      | create-accessibility-requirement   | ♿️accessibility-requirement/🌱️create/🌱️creates-a |
      | delete-accessibility-requirement   | ♿️accessibility-requirement/🗑️delete/🗑️deletes-a |
      | rename-accessibility-requirement   | ♿️accessibility-requirement/🏷️rename/🏷️renames-a |
      | replace-accessibility-requirement  | ♿️accessibility-requirement/♻️replace/♻️replaces-a |
      | create-conflict                    | ⚔️conflict/🌱️create/🌱️creates-a |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🗑️deletes-a |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🏷️renames-a |
      | replace-conflict                   | ⚔️conflict/♻️replace/♻️replaces-a |
      | create-option-evaluation           | ⚖️option-evaluation/🌱️create/🌱️creates-a |
      | delete-option-evaluation           | ⚖️option-evaluation/🗑️delete/🗑️deletes-a |
      | rename-option-evaluation           | ⚖️option-evaluation/🏷️rename/🏷️renames-a |
      | replace-option-evaluation          | ⚖️option-evaluation/♻️replace/♻️replaces-a |
      | create-function                    | ⚙️function/🌱️create/🌱️creates-a |
      | delete-function                    | ⚙️function/🗑️delete/🗑️deletes-a |
      | rename-function                    | ⚙️function/🏷️rename/🏷️renames-a |
      | replace-function                   | ⚙️function/♻️replace/♻️replaces-a |
      | create-risk                        | ⚠️risk/🌱️create/🌱️creates-a |
      | delete-risk                        | ⚠️risk/🗑️delete/🗑️deletes-a |
      | rename-risk                        | ⚠️risk/🏷️rename/🏷️renames-a |
      | replace-risk                       | ⚠️risk/♻️replace/♻️replaces-a |
      | create-decision                    | ✅️decision/🌱️create/🌱️creates-a |
      | delete-decision                    | ✅️decision/🗑️delete/🗑️deletes-a |
      | rename-decision                    | ✅️decision/🏷️rename/🏷️renames-a |
      | replace-decision                   | ✅️decision/♻️replace/♻️replaces-a |
      | create-validation-record           | ✔️validation-record/🌱️create/🌱️creates-a |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🗑️deletes-a |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🏷️renames-a |
      | replace-validation-record          | ✔️validation-record/♻️replace/♻️replaces-a |
      | create-priority-record             | ⭐️priority-record/🌱️create/🌱️creates-a |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🗑️deletes-a |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🏷️renames-a |
      | replace-priority-record            | ⭐️priority-record/♻️replace/♻️replaces-a |
      | create-flow-requirement            | 🌊️flow-requirement/🌱️create/🌱️creates-a |
      | delete-flow-requirement            | 🌊️flow-requirement/🗑️delete/🗑️deletes-a |
      | rename-flow-requirement            | 🌊️flow-requirement/🏷️rename/🏷️renames-a |
      | replace-flow-requirement           | 🌊️flow-requirement/♻️replace/♻️replaces-a |
      | create-environmental-requirement   | 🌿️environmental-requirement/🌱️create/🌱️creates-a |
      | delete-environmental-requirement   | 🌿️environmental-requirement/🗑️delete/🗑️deletes-a |
      | rename-environmental-requirement   | 🌿️environmental-requirement/🏷️rename/🏷️renames-a |
      | replace-environmental-requirement  | 🌿️environmental-requirement/♻️replace/♻️replaces-a |
      | create-workshop                    | 🎓️workshop/🌱️create/🌱️creates-a |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🗑️deletes-a |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🏷️renames-a |
      | replace-workshop                   | 🎓️workshop/♻️replace/♻️replaces-a |
      | create-scenario                    | 🎬️scenario/🌱️create/🌱️creates-a |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🗑️deletes-a |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🏷️renames-a |
      | replace-scenario                   | 🎬️scenario/♻️replace/♻️replaces-a |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🌱️creates-a |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🌱️creates-a                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🗑️deletes-a                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🏷️renames-a                                            |
      | replace-activity                   | 🏃️activity/♻️replace/♻️replaces-a                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure-requirement/🌱️create/🌱️creates-a |
      | delete-infrastructure-requirement  | 🏗️infrastructure-requirement/🗑️delete/🗑️deletes-a |
      | rename-infrastructure-requirement  | 🏗️infrastructure-requirement/🏷️rename/🏷️renames-a |
      | replace-infrastructure-requirement | 🏗️infrastructure-requirement/♻️replace/♻️replaces-a |
      | create-organizational-requirement  | 🏢️organizational-requirement/🌱️create/🌱️creates-a |
      | delete-organizational-requirement  | 🏢️organizational-requirement/🗑️delete/🗑️deletes-a |
      | rename-organizational-requirement  | 🏢️organizational-requirement/🏷️rename/🏷️renames-a |
      | replace-organizational-requirement | 🏢️organizational-requirement/♻️replace/♻️replaces-a |
      | create-issue                       | 🐛️issue/🌱️create/🌱️creates-a |
      | delete-issue                       | 🐛️issue/🗑️delete/🗑️deletes-a |
      | rename-issue                       | 🐛️issue/🏷️rename/🏷️renames-a |
      | replace-issue                      | 🐛️issue/♻️replace/♻️replaces-a |
      | create-approval-record             | 👍️approval-record/🌱️create/🌱️creates-a |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🗑️deletes-a |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🏷️renames-a |
      | replace-approval-record            | 👍️approval-record/♻️replace/♻️replaces-a |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🌱️creates-a |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🗑️deletes-a |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🏷️renames-a |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/♻️replaces-a |
      | create-quality-record              | 💎️quality-record/🌱️create/🌱️creates-a |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🗑️deletes-a |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🏷️renames-a |
      | replace-quality-record             | 💎️quality-record/♻️replace/♻️replaces-a |
      | create-resilience-requirement      | 💪️resilience-requirement/🌱️create/🌱️creates-a |
      | delete-resilience-requirement      | 💪️resilience-requirement/🗑️delete/🗑️deletes-a |
      | rename-resilience-requirement      | 💪️resilience-requirement/🏷️rename/🏷️renames-a |
      | replace-resilience-requirement     | 💪️resilience-requirement/♻️replace/♻️replaces-a |
      | create-assumption                  | 💭️assumption/🌱️create/🌱️creates-a |
      | delete-assumption                  | 💭️assumption/🗑️delete/🗑️deletes-a |
      | rename-assumption                  | 💭️assumption/🏷️rename/🏷️renames-a |
      | replace-assumption                 | 💭️assumption/♻️replace/♻️replaces-a |
      | create-cost-requirement            | 💰️cost-requirement/🌱️create/🌱️creates-a |
      | delete-cost-requirement            | 💰️cost-requirement/🗑️delete/🗑️deletes-a |
      | rename-cost-requirement            | 💰️cost-requirement/🏷️rename/🏷️renames-a |
      | replace-cost-requirement           | 💰️cost-requirement/♻️replace/♻️replaces-a |
      | create-document                    | 📃️document/🌱️create/🌱️creates-a |
      | delete-document                    | 📃️document/🗑️delete/🗑️deletes-a |
      | rename-document                    | 📃️document/🏷️rename/🏷️renames-a |
      | replace-document                   | 📃️document/♻️replace/♻️replaces-a |
      | create-schedule-requirement        | 📅️schedule-requirement/🌱️create/🌱️creates-a |
      | delete-schedule-requirement        | 📅️schedule-requirement/🗑️delete/🗑️deletes-a |
      | rename-schedule-requirement        | 📅️schedule-requirement/🏷️rename/🏷️renames-a |
      | replace-schedule-requirement       | 📅️schedule-requirement/♻️replace/♻️replaces-a |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🌱️creates-a |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🗑️deletes-a |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🏷️renames-a |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/♻️replaces-a |
      | create-performance-criterion       | 📊️performance-criterion/🌱️create/🌱️creates-a |
      | delete-performance-criterion       | 📊️performance-criterion/🗑️delete/🗑️deletes-a |
      | rename-performance-criterion       | 📊️performance-criterion/🏷️rename/🏷️renames-a |
      | replace-performance-criterion      | 📊️performance-criterion/♻️replace/♻️replaces-a |
      | create-operational-requirement     | 📋️operational-requirement/🌱️create/🌱️creates-a |
      | delete-operational-requirement     | 📋️operational-requirement/🗑️delete/🗑️deletes-a |
      | rename-operational-requirement     | 📋️operational-requirement/🏷️rename/🏷️renames-a |
      | replace-operational-requirement    | 📋️operational-requirement/♻️replace/♻️replaces-a |
      | create-requirement                 | 📌️requirement/🌱️create/🌱️creates-a |
      | delete-requirement                 | 📌️requirement/🗑️delete/🗑️deletes-a |
      | rename-requirement                 | 📌️requirement/🏷️rename/🏷️renames-a |
      | replace-requirement                | 📌️requirement/♻️replace/♻️replaces-a |
      | create-site-context                | 📍️site-context/🌱️create/🌱️creates-a |
      | delete-site-context                | 📍️site-context/🗑️delete/🗑️deletes-a |
      | rename-site-context                | 📍️site-context/🏷️rename/🏷️renames-a |
      | replace-site-context               | 📍️site-context/♻️replace/♻️replaces-a |
      | create-template-record             | 📐️template-record/🌱️create/🌱️creates-a |
      | delete-template-record             | 📐️template-record/🗑️delete/🗑️deletes-a |
      | rename-template-record             | 📐️template-record/🏷️rename/🏷️renames-a |
      | replace-template-record            | 📐️template-record/♻️replace/♻️replaces-a |
      | create-report-record               | 📑️report-record/🌱️create/🌱️creates-a |
      | delete-report-record               | 📑️report-record/🗑️delete/🗑️deletes-a |
      | rename-report-record               | 📑️report-record/🏷️rename/🏷️renames-a |
      | replace-report-record              | 📑️report-record/♻️replace/♻️replaces-a |
      | create-audit-event                 | 📒️audit-event/🌱️create/🌱️creates-a |
      | delete-audit-event                 | 📒️audit-event/🗑️delete/🗑️deletes-a |
      | rename-audit-event                 | 📒️audit-event/🏷️rename/🏷️renames-a |
      | replace-audit-event                | 📒️audit-event/♻️replace/♻️replaces-a |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🌱️creates-a |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory-requirement/🌱️create/🌱️creates-a |
      | delete-regulatory-requirement      | 📜️regulatory-requirement/🗑️delete/🗑️deletes-a |
      | rename-regulatory-requirement      | 📜️regulatory-requirement/🏷️rename/🏷️renames-a |
      | replace-regulatory-requirement     | 📜️regulatory-requirement/♻️replace/♻️replaces-a |
      | create-change-record               | 🔀️change-record/🌱️create/🌱️creates-a |
      | delete-change-record               | 🔀️change-record/🗑️delete/🗑️deletes-a |
      | rename-change-record               | 🔀️change-record/🏷️rename/🏷️renames-a |
      | replace-change-record              | 🔀️change-record/♻️replace/♻️replaces-a |
      | create-communication-requirement   | 📡️communication-requirement/🌱️create/🌱️creates-a |
      | delete-communication-requirement   | 📡️communication-requirement/🗑️delete/🗑️deletes-a |
      | rename-communication-requirement   | 📡️communication-requirement/🏷️rename/🏷️renames-a |
      | replace-communication-requirement  | 📡️communication-requirement/♻️replace/♻️replaces-a |
      | create-resource                    | 📦️resource/🌱️create/🌱️creates-a |
      | delete-resource                    | 📦️resource/🗑️delete/🗑️deletes-a |
      | rename-resource                    | 📦️resource/🏷️rename/🏷️renames-a |
      | replace-resource                   | 📦️resource/♻️replace/♻️replaces-a |
      | create-status-record               | 📶️status-record/🌱️create/🌱️creates-a |
      | delete-status-record               | 📶️status-record/🗑️delete/🗑️deletes-a |
      | rename-status-record               | 📶️status-record/🏷️rename/🏷️renames-a |
      | replace-status-record              | 📶️status-record/♻️replace/♻️replaces-a |
      | create-process                     | 🔄️process/🌱️create/🌱️creates-a |
      | delete-process                     | 🔄️process/🗑️delete/🗑️deletes-a |
      | rename-process                     | 🔄️process/🏷️rename/🏷️renames-a |
      | replace-process                    | 🔄️process/♻️replace/♻️replaces-a |
      | create-search-filter               | 🔍️search-filter/🌱️create/🌱️creates-a |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🗑️deletes-a |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🏷️renames-a |
      | replace-search-filter              | 🔍️search-filter/♻️replace/♻️replaces-a |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🌱️creates-a |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🗑️deletes-a |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🏷️renames-a |
      | replace-access-rule                | 🔑️access-rule/♻️replace/♻️replaces-a |
      | create-privacy-requirement         | 🔒️privacy-requirement/🌱️create/🌱️creates-a |
      | delete-privacy-requirement         | 🔒️privacy-requirement/🗑️delete/🗑️deletes-a |
      | rename-privacy-requirement         | 🔒️privacy-requirement/🏷️rename/🏷️renames-a |
      | replace-privacy-requirement        | 🔒️privacy-requirement/♻️replace/♻️replaces-a |
      | create-relationship                | 🕸️relationship/🌱️create/🌱️creates-a |
      | delete-relationship                | 🕸️relationship/🗑️delete/🗑️deletes-a |
      | rename-relationship                | 🕸️relationship/🏷️rename/🏷️renames-a |
      | replace-relationship               | 🕸️relationship/♻️replace/♻️replaces-a |
      | create-quantity-requirement        | 🔢️quantity-requirement/🌱️create/🌱️creates-a |
      | delete-quantity-requirement        | 🔢️quantity-requirement/🗑️delete/🗑️deletes-a |
      | rename-quantity-requirement        | 🔢️quantity-requirement/🏷️rename/🏷️renames-a |
      | replace-quantity-requirement       | 🔢️quantity-requirement/♻️replace/♻️replaces-a |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🌱️creates-a |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🗑️deletes-a |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🏷️renames-a |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/♻️replaces-a |
      | create-storage-requirement         | 🗄️storage-requirement/🌱️create/🌱️creates-a |
      | delete-storage-requirement         | 🗄️storage-requirement/🗑️delete/🗑️deletes-a |
      | rename-storage-requirement         | 🗄️storage-requirement/🏷️rename/🏷️renames-a |
      | replace-storage-requirement        | 🗄️storage-requirement/♻️replace/♻️replaces-a |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🌱️creates-a |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🗑️deletes-a |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🏷️renames-a |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/♻️replaces-a |
      | create-survey                      | 🗳️survey/🌱️create/🌱️creates-a |
      | delete-survey                      | 🗳️survey/🗑️delete/🗑️deletes-a |
      | rename-survey                      | 🗳️survey/🏷️rename/🏷️renames-a |
      | replace-survey                     | 🗳️survey/♻️replace/♻️replaces-a |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🌱️creates-a |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🗑️deletes-a |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🏷️renames-a |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/♻️replaces-a |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🌱️creates-a |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🗑️deletes-a |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🏷️renames-a |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/♻️replaces-a |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🌱️creates-a |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🗑️deletes-a |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🏷️renames-a |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/♻️replaces-a |
      | create-service-requirement         | 🛎️service-requirement/🌱️create/🌱️creates-a |
      | delete-service-requirement         | 🛎️service-requirement/🗑️delete/🗑️deletes-a |
      | rename-service-requirement         | 🛎️service-requirement/🏷️rename/🏷️renames-a |
      | replace-service-requirement        | 🛎️service-requirement/♻️replace/♻️replaces-a |
      | create-equipment                   | 🛠️equipment/🌱️create/🌱️creates-a |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🗑️deletes-a |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🏷️renames-a |
      | replace-equipment                  | 🛠️equipment/♻️replace/♻️replaces-a |
      | create-security-requirement        | 🛡️security-requirement/🌱️create/🌱️creates-a |
      | delete-security-requirement        | 🛡️security-requirement/🗑️delete/🗑️deletes-a |
      | rename-security-requirement        | 🛡️security-requirement/🏷️rename/🏷️renames-a |
      | replace-security-requirement       | 🛡️security-requirement/♻️replace/♻️replaces-a |
      | create-collaboration-record        | 🤝️collaboration-record/🌱️create/🌱️creates-a |
      | delete-collaboration-record        | 🤝️collaboration-record/🗑️delete/🗑️deletes-a |
      | rename-collaboration-record        | 🤝️collaboration-record/🏷️rename/🏷️renames-a |
      | replace-collaboration-record       | 🤝️collaboration-record/♻️replace/♻️replaces-a |
      | create-safety-requirement          | 🦺️safety-requirement/🌱️create/🌱️creates-a |
      | delete-safety-requirement          | 🦺️safety-requirement/🗑️delete/🗑️deletes-a |
      | rename-safety-requirement          | 🦺️safety-requirement/🏷️rename/🏷️renames-a |
      | replace-safety-requirement         | 🦺️safety-requirement/♻️replace/♻️replaces-a |
      | create-user-profile                | 🧑️user-profile/🌱️create/🌱️creates-a |
      | delete-user-profile                | 🧑️user-profile/🗑️delete/🗑️deletes-a |
      | rename-user-profile                | 🧑️user-profile/🏷️rename/🏷️renames-a |
      | replace-user-profile               | 🧑️user-profile/♻️replace/♻️replaces-a |
      | create-human-factor-requirement    | 🧠️human-factor-requirement/🌱️create/🌱️creates-a |
      | delete-human-factor-requirement    | 🧠️human-factor-requirement/🗑️delete/🗑️deletes-a |
      | rename-human-factor-requirement    | 🧠️human-factor-requirement/🏷️rename/🏷️renames-a |
      | replace-human-factor-requirement   | 🧠️human-factor-requirement/♻️replace/♻️replaces-a |
      | create-flexibility-requirement     | 🧩️flexibility-requirement/🌱️create/🌱️creates-a |
      | delete-flexibility-requirement     | 🧩️flexibility-requirement/🗑️delete/🗑️deletes-a |
      | rename-flexibility-requirement     | 🧩️flexibility-requirement/🏷️rename/🏷️renames-a |
      | replace-flexibility-requirement    | 🧩️flexibility-requirement/♻️replace/♻️replaces-a |
      | create-wayfinding-requirement      | 🧭️wayfinding-requirement/🌱️create/🌱️creates-a |
      | delete-wayfinding-requirement      | 🧭️wayfinding-requirement/🗑️delete/🗑️deletes-a |
      | rename-wayfinding-requirement      | 🧭️wayfinding-requirement/🏷️rename/🏷️renames-a |
      | replace-wayfinding-requirement     | 🧭️wayfinding-requirement/♻️replace/♻️replaces-a |
      | create-program-element             | 🧱️program-element/🌱️create/🌱️creates-a |
      | delete-program-element             | 🧱️program-element/🗑️delete/🗑️deletes-a |
      | rename-program-element             | 🧱️program-element/🏷️rename/🏷️renames-a |
      | replace-program-element            | 🧱️program-element/♻️replace/♻️replaces-a |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧲️reception-waiting |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🫷️reception-waiting |
      | connect-trace                      | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/✂️requirement-decision |
      | rename-meta                       | 🏷️meta/🏷️rename/🏷️title |
      | replace-meta                      | 🏷️meta/♻️replace/♻️block |
      | rename-project                    | 🏙️project/🏷️rename/🏷️code |
      | replace-project                   | 🏙️project/♻️replace/♻️definition |
      | rename-governance                 | 🏛️governance/🏷️rename/🏷️framework |
      | replace-governance                | 🏛️governance/♻️replace/♻️block |

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
      | id                                 | vector                                                                              |
      | create-information-requirement     | ℹ️information-requirement/🌱️create/🌱️creates-a |
      | delete-information-requirement     | ℹ️information-requirement/🗑️delete/🗑️deletes-a |
      | rename-information-requirement     | ℹ️information-requirement/🏷️rename/🏷️renames-a |
      | replace-information-requirement    | ℹ️information-requirement/♻️replace/♻️replaces-a |
      | create-sustainability-requirement  | ♻️sustainability-requirement/🌱️create/🌱️creates-a |
      | delete-sustainability-requirement  | ♻️sustainability-requirement/🗑️delete/🗑️deletes-a |
      | rename-sustainability-requirement  | ♻️sustainability-requirement/🏷️rename/🏷️renames-a |
      | replace-sustainability-requirement | ♻️sustainability-requirement/♻️replace/♻️replaces-a |
      | create-accessibility-requirement   | ♿️accessibility-requirement/🌱️create/🌱️creates-a |
      | delete-accessibility-requirement   | ♿️accessibility-requirement/🗑️delete/🗑️deletes-a |
      | rename-accessibility-requirement   | ♿️accessibility-requirement/🏷️rename/🏷️renames-a |
      | replace-accessibility-requirement  | ♿️accessibility-requirement/♻️replace/♻️replaces-a |
      | create-conflict                    | ⚔️conflict/🌱️create/🌱️creates-a |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🗑️deletes-a |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🏷️renames-a |
      | replace-conflict                   | ⚔️conflict/♻️replace/♻️replaces-a |
      | create-option-evaluation           | ⚖️option-evaluation/🌱️create/🌱️creates-a |
      | delete-option-evaluation           | ⚖️option-evaluation/🗑️delete/🗑️deletes-a |
      | rename-option-evaluation           | ⚖️option-evaluation/🏷️rename/🏷️renames-a |
      | replace-option-evaluation          | ⚖️option-evaluation/♻️replace/♻️replaces-a |
      | create-function                    | ⚙️function/🌱️create/🌱️creates-a |
      | delete-function                    | ⚙️function/🗑️delete/🗑️deletes-a |
      | rename-function                    | ⚙️function/🏷️rename/🏷️renames-a |
      | replace-function                   | ⚙️function/♻️replace/♻️replaces-a |
      | create-risk                        | ⚠️risk/🌱️create/🌱️creates-a |
      | delete-risk                        | ⚠️risk/🗑️delete/🗑️deletes-a |
      | rename-risk                        | ⚠️risk/🏷️rename/🏷️renames-a |
      | replace-risk                       | ⚠️risk/♻️replace/♻️replaces-a |
      | create-decision                    | ✅️decision/🌱️create/🌱️creates-a |
      | delete-decision                    | ✅️decision/🗑️delete/🗑️deletes-a |
      | rename-decision                    | ✅️decision/🏷️rename/🏷️renames-a |
      | replace-decision                   | ✅️decision/♻️replace/♻️replaces-a |
      | create-validation-record           | ✔️validation-record/🌱️create/🌱️creates-a |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🗑️deletes-a |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🏷️renames-a |
      | replace-validation-record          | ✔️validation-record/♻️replace/♻️replaces-a |
      | create-priority-record             | ⭐️priority-record/🌱️create/🌱️creates-a |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🗑️deletes-a |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🏷️renames-a |
      | replace-priority-record            | ⭐️priority-record/♻️replace/♻️replaces-a |
      | create-flow-requirement            | 🌊️flow-requirement/🌱️create/🌱️creates-a |
      | delete-flow-requirement            | 🌊️flow-requirement/🗑️delete/🗑️deletes-a |
      | rename-flow-requirement            | 🌊️flow-requirement/🏷️rename/🏷️renames-a |
      | replace-flow-requirement           | 🌊️flow-requirement/♻️replace/♻️replaces-a |
      | create-environmental-requirement   | 🌿️environmental-requirement/🌱️create/🌱️creates-a |
      | delete-environmental-requirement   | 🌿️environmental-requirement/🗑️delete/🗑️deletes-a |
      | rename-environmental-requirement   | 🌿️environmental-requirement/🏷️rename/🏷️renames-a |
      | replace-environmental-requirement  | 🌿️environmental-requirement/♻️replace/♻️replaces-a |
      | create-workshop                    | 🎓️workshop/🌱️create/🌱️creates-a |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🗑️deletes-a |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🏷️renames-a |
      | replace-workshop                   | 🎓️workshop/♻️replace/♻️replaces-a |
      | create-scenario                    | 🎬️scenario/🌱️create/🌱️creates-a |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🗑️deletes-a |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🏷️renames-a |
      | replace-scenario                   | 🎬️scenario/♻️replace/♻️replaces-a |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🌱️creates-a |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🌱️creates-a                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🗑️deletes-a                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🏷️renames-a                                            |
      | replace-activity                   | 🏃️activity/♻️replace/♻️replaces-a                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure-requirement/🌱️create/🌱️creates-a |
      | delete-infrastructure-requirement  | 🏗️infrastructure-requirement/🗑️delete/🗑️deletes-a |
      | rename-infrastructure-requirement  | 🏗️infrastructure-requirement/🏷️rename/🏷️renames-a |
      | replace-infrastructure-requirement | 🏗️infrastructure-requirement/♻️replace/♻️replaces-a |
      | create-organizational-requirement  | 🏢️organizational-requirement/🌱️create/🌱️creates-a |
      | delete-organizational-requirement  | 🏢️organizational-requirement/🗑️delete/🗑️deletes-a |
      | rename-organizational-requirement  | 🏢️organizational-requirement/🏷️rename/🏷️renames-a |
      | replace-organizational-requirement | 🏢️organizational-requirement/♻️replace/♻️replaces-a |
      | create-issue                       | 🐛️issue/🌱️create/🌱️creates-a |
      | delete-issue                       | 🐛️issue/🗑️delete/🗑️deletes-a |
      | rename-issue                       | 🐛️issue/🏷️rename/🏷️renames-a |
      | replace-issue                      | 🐛️issue/♻️replace/♻️replaces-a |
      | create-approval-record             | 👍️approval-record/🌱️create/🌱️creates-a |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🗑️deletes-a |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🏷️renames-a |
      | replace-approval-record            | 👍️approval-record/♻️replace/♻️replaces-a |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🌱️creates-a |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🗑️deletes-a |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🏷️renames-a |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/♻️replaces-a |
      | create-quality-record              | 💎️quality-record/🌱️create/🌱️creates-a |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🗑️deletes-a |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🏷️renames-a |
      | replace-quality-record             | 💎️quality-record/♻️replace/♻️replaces-a |
      | create-resilience-requirement      | 💪️resilience-requirement/🌱️create/🌱️creates-a |
      | delete-resilience-requirement      | 💪️resilience-requirement/🗑️delete/🗑️deletes-a |
      | rename-resilience-requirement      | 💪️resilience-requirement/🏷️rename/🏷️renames-a |
      | replace-resilience-requirement     | 💪️resilience-requirement/♻️replace/♻️replaces-a |
      | create-assumption                  | 💭️assumption/🌱️create/🌱️creates-a |
      | delete-assumption                  | 💭️assumption/🗑️delete/🗑️deletes-a |
      | rename-assumption                  | 💭️assumption/🏷️rename/🏷️renames-a |
      | replace-assumption                 | 💭️assumption/♻️replace/♻️replaces-a |
      | create-cost-requirement            | 💰️cost-requirement/🌱️create/🌱️creates-a |
      | delete-cost-requirement            | 💰️cost-requirement/🗑️delete/🗑️deletes-a |
      | rename-cost-requirement            | 💰️cost-requirement/🏷️rename/🏷️renames-a |
      | replace-cost-requirement           | 💰️cost-requirement/♻️replace/♻️replaces-a |
      | create-document                    | 📃️document/🌱️create/🌱️creates-a |
      | delete-document                    | 📃️document/🗑️delete/🗑️deletes-a |
      | rename-document                    | 📃️document/🏷️rename/🏷️renames-a |
      | replace-document                   | 📃️document/♻️replace/♻️replaces-a |
      | create-schedule-requirement        | 📅️schedule-requirement/🌱️create/🌱️creates-a |
      | delete-schedule-requirement        | 📅️schedule-requirement/🗑️delete/🗑️deletes-a |
      | rename-schedule-requirement        | 📅️schedule-requirement/🏷️rename/🏷️renames-a |
      | replace-schedule-requirement       | 📅️schedule-requirement/♻️replace/♻️replaces-a |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🌱️creates-a |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🗑️deletes-a |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🏷️renames-a |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/♻️replaces-a |
      | create-performance-criterion       | 📊️performance-criterion/🌱️create/🌱️creates-a |
      | delete-performance-criterion       | 📊️performance-criterion/🗑️delete/🗑️deletes-a |
      | rename-performance-criterion       | 📊️performance-criterion/🏷️rename/🏷️renames-a |
      | replace-performance-criterion      | 📊️performance-criterion/♻️replace/♻️replaces-a |
      | create-operational-requirement     | 📋️operational-requirement/🌱️create/🌱️creates-a |
      | delete-operational-requirement     | 📋️operational-requirement/🗑️delete/🗑️deletes-a |
      | rename-operational-requirement     | 📋️operational-requirement/🏷️rename/🏷️renames-a |
      | replace-operational-requirement    | 📋️operational-requirement/♻️replace/♻️replaces-a |
      | create-requirement                 | 📌️requirement/🌱️create/🌱️creates-a |
      | delete-requirement                 | 📌️requirement/🗑️delete/🗑️deletes-a |
      | rename-requirement                 | 📌️requirement/🏷️rename/🏷️renames-a |
      | replace-requirement                | 📌️requirement/♻️replace/♻️replaces-a |
      | create-site-context                | 📍️site-context/🌱️create/🌱️creates-a |
      | delete-site-context                | 📍️site-context/🗑️delete/🗑️deletes-a |
      | rename-site-context                | 📍️site-context/🏷️rename/🏷️renames-a |
      | replace-site-context               | 📍️site-context/♻️replace/♻️replaces-a |
      | create-template-record             | 📐️template-record/🌱️create/🌱️creates-a |
      | delete-template-record             | 📐️template-record/🗑️delete/🗑️deletes-a |
      | rename-template-record             | 📐️template-record/🏷️rename/🏷️renames-a |
      | replace-template-record            | 📐️template-record/♻️replace/♻️replaces-a |
      | create-report-record               | 📑️report-record/🌱️create/🌱️creates-a |
      | delete-report-record               | 📑️report-record/🗑️delete/🗑️deletes-a |
      | rename-report-record               | 📑️report-record/🏷️rename/🏷️renames-a |
      | replace-report-record              | 📑️report-record/♻️replace/♻️replaces-a |
      | create-audit-event                 | 📒️audit-event/🌱️create/🌱️creates-a |
      | delete-audit-event                 | 📒️audit-event/🗑️delete/🗑️deletes-a |
      | rename-audit-event                 | 📒️audit-event/🏷️rename/🏷️renames-a |
      | replace-audit-event                | 📒️audit-event/♻️replace/♻️replaces-a |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🌱️creates-a |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory-requirement/🌱️create/🌱️creates-a |
      | delete-regulatory-requirement      | 📜️regulatory-requirement/🗑️delete/🗑️deletes-a |
      | rename-regulatory-requirement      | 📜️regulatory-requirement/🏷️rename/🏷️renames-a |
      | replace-regulatory-requirement     | 📜️regulatory-requirement/♻️replace/♻️replaces-a |
      | create-change-record               | 🔀️change-record/🌱️create/🌱️creates-a |
      | delete-change-record               | 🔀️change-record/🗑️delete/🗑️deletes-a |
      | rename-change-record               | 🔀️change-record/🏷️rename/🏷️renames-a |
      | replace-change-record              | 🔀️change-record/♻️replace/♻️replaces-a |
      | create-communication-requirement   | 📡️communication-requirement/🌱️create/🌱️creates-a |
      | delete-communication-requirement   | 📡️communication-requirement/🗑️delete/🗑️deletes-a |
      | rename-communication-requirement   | 📡️communication-requirement/🏷️rename/🏷️renames-a |
      | replace-communication-requirement  | 📡️communication-requirement/♻️replace/♻️replaces-a |
      | create-resource                    | 📦️resource/🌱️create/🌱️creates-a |
      | delete-resource                    | 📦️resource/🗑️delete/🗑️deletes-a |
      | rename-resource                    | 📦️resource/🏷️rename/🏷️renames-a |
      | replace-resource                   | 📦️resource/♻️replace/♻️replaces-a |
      | create-status-record               | 📶️status-record/🌱️create/🌱️creates-a |
      | delete-status-record               | 📶️status-record/🗑️delete/🗑️deletes-a |
      | rename-status-record               | 📶️status-record/🏷️rename/🏷️renames-a |
      | replace-status-record              | 📶️status-record/♻️replace/♻️replaces-a |
      | create-process                     | 🔄️process/🌱️create/🌱️creates-a |
      | delete-process                     | 🔄️process/🗑️delete/🗑️deletes-a |
      | rename-process                     | 🔄️process/🏷️rename/🏷️renames-a |
      | replace-process                    | 🔄️process/♻️replace/♻️replaces-a |
      | create-search-filter               | 🔍️search-filter/🌱️create/🌱️creates-a |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🗑️deletes-a |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🏷️renames-a |
      | replace-search-filter              | 🔍️search-filter/♻️replace/♻️replaces-a |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🌱️creates-a |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🗑️deletes-a |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🏷️renames-a |
      | replace-access-rule                | 🔑️access-rule/♻️replace/♻️replaces-a |
      | create-privacy-requirement         | 🔒️privacy-requirement/🌱️create/🌱️creates-a |
      | delete-privacy-requirement         | 🔒️privacy-requirement/🗑️delete/🗑️deletes-a |
      | rename-privacy-requirement         | 🔒️privacy-requirement/🏷️rename/🏷️renames-a |
      | replace-privacy-requirement        | 🔒️privacy-requirement/♻️replace/♻️replaces-a |
      | create-relationship                | 🕸️relationship/🌱️create/🌱️creates-a |
      | delete-relationship                | 🕸️relationship/🗑️delete/🗑️deletes-a |
      | rename-relationship                | 🕸️relationship/🏷️rename/🏷️renames-a |
      | replace-relationship               | 🕸️relationship/♻️replace/♻️replaces-a |
      | create-quantity-requirement        | 🔢️quantity-requirement/🌱️create/🌱️creates-a |
      | delete-quantity-requirement        | 🔢️quantity-requirement/🗑️delete/🗑️deletes-a |
      | rename-quantity-requirement        | 🔢️quantity-requirement/🏷️rename/🏷️renames-a |
      | replace-quantity-requirement       | 🔢️quantity-requirement/♻️replace/♻️replaces-a |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🌱️creates-a |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🗑️deletes-a |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🏷️renames-a |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/♻️replaces-a |
      | create-storage-requirement         | 🗄️storage-requirement/🌱️create/🌱️creates-a |
      | delete-storage-requirement         | 🗄️storage-requirement/🗑️delete/🗑️deletes-a |
      | rename-storage-requirement         | 🗄️storage-requirement/🏷️rename/🏷️renames-a |
      | replace-storage-requirement        | 🗄️storage-requirement/♻️replace/♻️replaces-a |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🌱️creates-a |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🗑️deletes-a |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🏷️renames-a |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/♻️replaces-a |
      | create-survey                      | 🗳️survey/🌱️create/🌱️creates-a |
      | delete-survey                      | 🗳️survey/🗑️delete/🗑️deletes-a |
      | rename-survey                      | 🗳️survey/🏷️rename/🏷️renames-a |
      | replace-survey                     | 🗳️survey/♻️replace/♻️replaces-a |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🌱️creates-a |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🗑️deletes-a |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🏷️renames-a |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/♻️replaces-a |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🌱️creates-a |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🗑️deletes-a |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🏷️renames-a |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/♻️replaces-a |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🌱️creates-a |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🗑️deletes-a |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🏷️renames-a |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/♻️replaces-a |
      | create-service-requirement         | 🛎️service-requirement/🌱️create/🌱️creates-a |
      | delete-service-requirement         | 🛎️service-requirement/🗑️delete/🗑️deletes-a |
      | rename-service-requirement         | 🛎️service-requirement/🏷️rename/🏷️renames-a |
      | replace-service-requirement        | 🛎️service-requirement/♻️replace/♻️replaces-a |
      | create-equipment                   | 🛠️equipment/🌱️create/🌱️creates-a |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🗑️deletes-a |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🏷️renames-a |
      | replace-equipment                  | 🛠️equipment/♻️replace/♻️replaces-a |
      | create-security-requirement        | 🛡️security-requirement/🌱️create/🌱️creates-a |
      | delete-security-requirement        | 🛡️security-requirement/🗑️delete/🗑️deletes-a |
      | rename-security-requirement        | 🛡️security-requirement/🏷️rename/🏷️renames-a |
      | replace-security-requirement       | 🛡️security-requirement/♻️replace/♻️replaces-a |
      | create-collaboration-record        | 🤝️collaboration-record/🌱️create/🌱️creates-a |
      | delete-collaboration-record        | 🤝️collaboration-record/🗑️delete/🗑️deletes-a |
      | rename-collaboration-record        | 🤝️collaboration-record/🏷️rename/🏷️renames-a |
      | replace-collaboration-record       | 🤝️collaboration-record/♻️replace/♻️replaces-a |
      | create-safety-requirement          | 🦺️safety-requirement/🌱️create/🌱️creates-a |
      | delete-safety-requirement          | 🦺️safety-requirement/🗑️delete/🗑️deletes-a |
      | rename-safety-requirement          | 🦺️safety-requirement/🏷️rename/🏷️renames-a |
      | replace-safety-requirement         | 🦺️safety-requirement/♻️replace/♻️replaces-a |
      | create-user-profile                | 🧑️user-profile/🌱️create/🌱️creates-a |
      | delete-user-profile                | 🧑️user-profile/🗑️delete/🗑️deletes-a |
      | rename-user-profile                | 🧑️user-profile/🏷️rename/🏷️renames-a |
      | replace-user-profile               | 🧑️user-profile/♻️replace/♻️replaces-a |
      | create-human-factor-requirement    | 🧠️human-factor-requirement/🌱️create/🌱️creates-a |
      | delete-human-factor-requirement    | 🧠️human-factor-requirement/🗑️delete/🗑️deletes-a |
      | rename-human-factor-requirement    | 🧠️human-factor-requirement/🏷️rename/🏷️renames-a |
      | replace-human-factor-requirement   | 🧠️human-factor-requirement/♻️replace/♻️replaces-a |
      | create-flexibility-requirement     | 🧩️flexibility-requirement/🌱️create/🌱️creates-a |
      | delete-flexibility-requirement     | 🧩️flexibility-requirement/🗑️delete/🗑️deletes-a |
      | rename-flexibility-requirement     | 🧩️flexibility-requirement/🏷️rename/🏷️renames-a |
      | replace-flexibility-requirement    | 🧩️flexibility-requirement/♻️replace/♻️replaces-a |
      | create-wayfinding-requirement      | 🧭️wayfinding-requirement/🌱️create/🌱️creates-a |
      | delete-wayfinding-requirement      | 🧭️wayfinding-requirement/🗑️delete/🗑️deletes-a |
      | rename-wayfinding-requirement      | 🧭️wayfinding-requirement/🏷️rename/🏷️renames-a |
      | replace-wayfinding-requirement     | 🧭️wayfinding-requirement/♻️replace/♻️replaces-a |
      | create-program-element             | 🧱️program-element/🌱️create/🌱️creates-a |
      | delete-program-element             | 🧱️program-element/🗑️delete/🗑️deletes-a |
      | rename-program-element             | 🧱️program-element/🏷️rename/🏷️renames-a |
      | replace-program-element            | 🧱️program-element/♻️replace/♻️replaces-a |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧲️reception-waiting |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🫷️reception-waiting |
      | connect-trace                      | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/✂️requirement-decision |
      | rename-meta                       | 🏷️meta/🏷️rename/🏷️title |
      | replace-meta                      | 🏷️meta/♻️replace/♻️block |
      | rename-project                    | 🏙️project/🏷️rename/🏷️code |
      | replace-project                   | 🏙️project/♻️replace/♻️definition |
      | rename-governance                 | 🏛️governance/🏷️rename/🏷️framework |
      | replace-governance                | 🏛️governance/♻️replace/♻️block |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🎬️demo/🗣️.dsl.semio
    When the artifact is parsed to a ProgramSnapshot, printed back to `.architect` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
