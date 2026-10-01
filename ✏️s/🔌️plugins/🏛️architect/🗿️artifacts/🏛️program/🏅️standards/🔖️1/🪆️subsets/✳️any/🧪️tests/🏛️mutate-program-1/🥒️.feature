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
      | create-information-requirement     | ℹ️information/🌱️create/🌱️creates |
      | delete-information-requirement     | ℹ️information/🗑️delete/🗑️deletes |
      | rename-information-requirement     | ℹ️information/🏷️rename/🏷️renames |
      | replace-information-requirement    | ℹ️information/♻️replace/♻️replaces |
      | create-sustainability-requirement  | ♻️sustainability/🌱️create/🌱️creates |
      | delete-sustainability-requirement  | ♻️sustainability/🗑️delete/🗑️deletes |
      | rename-sustainability-requirement  | ♻️sustainability/🏷️rename/🏷️renames |
      | replace-sustainability-requirement | ♻️sustainability/♻️replace/♻️replaces |
      | create-accessibility-requirement   | ♿️accessibility/🌱️create/🌱️creates |
      | delete-accessibility-requirement   | ♿️accessibility/🗑️delete/🗑️deletes |
      | rename-accessibility-requirement   | ♿️accessibility/🏷️rename/🏷️renames |
      | replace-accessibility-requirement  | ♿️accessibility/♻️replace/♻️replaces |
      | create-conflict                    | ⚔️conflict/🌱️create/🌱️creates |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🗑️deletes |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🏷️renames |
      | replace-conflict                   | ⚔️conflict/♻️replace/♻️replaces |
      | create-option-evaluation           | ⚖️option/🌱️create/🌱️creates |
      | delete-option-evaluation           | ⚖️option/🗑️delete/🗑️deletes |
      | rename-option-evaluation           | ⚖️option/🏷️rename/🏷️renames |
      | replace-option-evaluation          | ⚖️option/♻️replace/♻️replaces |
      | create-function                    | ⚙️function/🌱️create/🌱️creates |
      | delete-function                    | ⚙️function/🗑️delete/🗑️deletes |
      | rename-function                    | ⚙️function/🏷️rename/🏷️renames |
      | replace-function                   | ⚙️function/♻️replace/♻️replaces |
      | create-risk                        | ⚠️risk/🌱️create/🌱️creates |
      | delete-risk                        | ⚠️risk/🗑️delete/🗑️deletes |
      | rename-risk                        | ⚠️risk/🏷️rename/🏷️renames |
      | replace-risk                       | ⚠️risk/♻️replace/♻️replaces |
      | create-decision                    | ✅️decision/🌱️create/🌱️creates |
      | delete-decision                    | ✅️decision/🗑️delete/🗑️deletes |
      | rename-decision                    | ✅️decision/🏷️rename/🏷️renames |
      | replace-decision                   | ✅️decision/♻️replace/♻️replaces |
      | create-validation-record           | ✔️validation-record/🌱️create/🌱️creates |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🗑️deletes |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🏷️renames |
      | replace-validation-record          | ✔️validation-record/♻️replace/♻️replaces |
      | create-priority-record             | ⭐️priority-record/🌱️create/🌱️creates |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🗑️deletes |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🏷️renames |
      | replace-priority-record            | ⭐️priority-record/♻️replace/♻️replaces |
      | create-flow-requirement            | 🌊️flow/🌱️create/🌱️creates |
      | delete-flow-requirement            | 🌊️flow/🗑️delete/🗑️deletes |
      | rename-flow-requirement            | 🌊️flow/🏷️rename/🏷️renames |
      | replace-flow-requirement           | 🌊️flow/♻️replace/♻️replaces |
      | create-environmental-requirement   | 🌿️environmental/🌱️create/🌱️creates |
      | delete-environmental-requirement   | 🌿️environmental/🗑️delete/🗑️deletes |
      | rename-environmental-requirement   | 🌿️environmental/🏷️rename/🏷️renames |
      | replace-environmental-requirement  | 🌿️environmental/♻️replace/♻️replaces |
      | create-workshop                    | 🎓️workshop/🌱️create/🌱️creates |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🗑️deletes |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🏷️renames |
      | replace-workshop                   | 🎓️workshop/♻️replace/♻️replaces |
      | create-scenario                    | 🎬️scenario/🌱️create/🌱️creates |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🗑️deletes |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🏷️renames |
      | replace-scenario                   | 🎬️scenario/♻️replace/♻️replaces |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🌱️creates |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🌱️creates                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🗑️deletes                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🏷️renames                                            |
      | replace-activity                   | 🏃️activity/♻️replace/♻️replaces                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure/🌱️create/🌱️creates |
      | delete-infrastructure-requirement  | 🏗️infrastructure/🗑️delete/🗑️deletes |
      | rename-infrastructure-requirement  | 🏗️infrastructure/🏷️rename/🏷️renames |
      | replace-infrastructure-requirement | 🏗️infrastructure/♻️replace/♻️replaces |
      | create-organizational-requirement  | 🏢️organizational/🌱️create/🌱️creates |
      | delete-organizational-requirement  | 🏢️organizational/🗑️delete/🗑️deletes |
      | rename-organizational-requirement  | 🏢️organizational/🏷️rename/🏷️renames |
      | replace-organizational-requirement | 🏢️organizational/♻️replace/♻️replaces |
      | create-issue                       | 🐛️issue/🌱️create/🌱️creates |
      | delete-issue                       | 🐛️issue/🗑️delete/🗑️deletes |
      | rename-issue                       | 🐛️issue/🏷️rename/🏷️renames |
      | replace-issue                      | 🐛️issue/♻️replace/♻️replaces |
      | create-approval-record             | 👍️approval-record/🌱️create/🌱️creates |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🗑️deletes |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🏷️renames |
      | replace-approval-record            | 👍️approval-record/♻️replace/♻️replaces |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🌱️creates |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🗑️deletes |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🏷️renames |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/♻️replaces |
      | create-quality-record              | 💎️quality-record/🌱️create/🌱️creates |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🗑️deletes |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🏷️renames |
      | replace-quality-record             | 💎️quality-record/♻️replace/♻️replaces |
      | create-resilience-requirement      | 💪️resilience/🌱️create/🌱️creates |
      | delete-resilience-requirement      | 💪️resilience/🗑️delete/🗑️deletes |
      | rename-resilience-requirement      | 💪️resilience/🏷️rename/🏷️renames |
      | replace-resilience-requirement     | 💪️resilience/♻️replace/♻️replaces |
      | create-assumption                  | 💭️assumption/🌱️create/🌱️creates |
      | delete-assumption                  | 💭️assumption/🗑️delete/🗑️deletes |
      | rename-assumption                  | 💭️assumption/🏷️rename/🏷️renames |
      | replace-assumption                 | 💭️assumption/♻️replace/♻️replaces |
      | create-cost-requirement            | 💰️cost/🌱️create/🌱️creates |
      | delete-cost-requirement            | 💰️cost/🗑️delete/🗑️deletes |
      | rename-cost-requirement            | 💰️cost/🏷️rename/🏷️renames |
      | replace-cost-requirement           | 💰️cost/♻️replace/♻️replaces |
      | create-document                    | 📃️document/🌱️create/🌱️creates |
      | delete-document                    | 📃️document/🗑️delete/🗑️deletes |
      | rename-document                    | 📃️document/🏷️rename/🏷️renames |
      | replace-document                   | 📃️document/♻️replace/♻️replaces |
      | create-schedule-requirement        | 📅️schedule/🌱️create/🌱️creates |
      | delete-schedule-requirement        | 📅️schedule/🗑️delete/🗑️deletes |
      | rename-schedule-requirement        | 📅️schedule/🏷️rename/🏷️renames |
      | replace-schedule-requirement       | 📅️schedule/♻️replace/♻️replaces |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🌱️creates |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🗑️deletes |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🏷️renames |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/♻️replaces |
      | create-performance-criterion       | 📊️performance/🌱️create/🌱️creates |
      | delete-performance-criterion       | 📊️performance/🗑️delete/🗑️deletes |
      | rename-performance-criterion       | 📊️performance/🏷️rename/🏷️renames |
      | replace-performance-criterion      | 📊️performance/♻️replace/♻️replaces |
      | create-operational-requirement     | 📋️operational/🌱️create/🌱️creates |
      | delete-operational-requirement     | 📋️operational/🗑️delete/🗑️deletes |
      | rename-operational-requirement     | 📋️operational/🏷️rename/🏷️renames |
      | replace-operational-requirement    | 📋️operational/♻️replace/♻️replaces |
      | create-requirement                 | 📌️requirement/🌱️create/🌱️creates |
      | delete-requirement                 | 📌️requirement/🗑️delete/🗑️deletes |
      | rename-requirement                 | 📌️requirement/🏷️rename/🏷️renames |
      | replace-requirement                | 📌️requirement/♻️replace/♻️replaces |
      | create-site-context                | 📍️site/🌱️create/🌱️creates |
      | delete-site-context                | 📍️site/🗑️delete/🗑️deletes |
      | rename-site-context                | 📍️site/🏷️rename/🏷️renames |
      | replace-site-context               | 📍️site/♻️replace/♻️replaces |
      | create-template-record             | 📐️template-record/🌱️create/🌱️creates |
      | delete-template-record             | 📐️template-record/🗑️delete/🗑️deletes |
      | rename-template-record             | 📐️template-record/🏷️rename/🏷️renames |
      | replace-template-record            | 📐️template-record/♻️replace/♻️replaces |
      | create-report-record               | 📑️report-record/🌱️create/🌱️creates |
      | delete-report-record               | 📑️report-record/🗑️delete/🗑️deletes |
      | rename-report-record               | 📑️report-record/🏷️rename/🏷️renames |
      | replace-report-record              | 📑️report-record/♻️replace/♻️replaces |
      | create-audit-event                 | 📒️audit/🌱️create/🌱️creates |
      | delete-audit-event                 | 📒️audit/🗑️delete/🗑️deletes |
      | rename-audit-event                 | 📒️audit/🏷️rename/🏷️renames |
      | replace-audit-event                | 📒️audit/♻️replace/♻️replaces |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🌱️creates |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory/🌱️create/🌱️creates |
      | delete-regulatory-requirement      | 📜️regulatory/🗑️delete/🗑️deletes |
      | rename-regulatory-requirement      | 📜️regulatory/🏷️rename/🏷️renames |
      | replace-regulatory-requirement     | 📜️regulatory/♻️replace/♻️replaces |
      | create-change-record               | 🔀️change-record/🌱️create/🌱️creates |
      | delete-change-record               | 🔀️change-record/🗑️delete/🗑️deletes |
      | rename-change-record               | 🔀️change-record/🏷️rename/🏷️renames |
      | replace-change-record              | 🔀️change-record/♻️replace/♻️replaces |
      | create-communication-requirement   | 📡️communication/🌱️create/🌱️creates |
      | delete-communication-requirement   | 📡️communication/🗑️delete/🗑️deletes |
      | rename-communication-requirement   | 📡️communication/🏷️rename/🏷️renames |
      | replace-communication-requirement  | 📡️communication/♻️replace/♻️replaces |
      | create-resource                    | 📦️resource/🌱️create/🌱️creates |
      | delete-resource                    | 📦️resource/🗑️delete/🗑️deletes |
      | rename-resource                    | 📦️resource/🏷️rename/🏷️renames |
      | replace-resource                   | 📦️resource/♻️replace/♻️replaces |
      | create-status-record               | 📶️status-record/🌱️create/🌱️creates |
      | delete-status-record               | 📶️status-record/🗑️delete/🗑️deletes |
      | rename-status-record               | 📶️status-record/🏷️rename/🏷️renames |
      | replace-status-record              | 📶️status-record/♻️replace/♻️replaces |
      | create-process                     | 🔄️process/🌱️create/🌱️creates |
      | delete-process                     | 🔄️process/🗑️delete/🗑️deletes |
      | rename-process                     | 🔄️process/🏷️rename/🏷️renames |
      | replace-process                    | 🔄️process/♻️replace/♻️replaces |
      | create-search-filter               | 🔍️search-filter/🌱️create/🌱️creates |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🗑️deletes |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🏷️renames |
      | replace-search-filter              | 🔍️search-filter/♻️replace/♻️replaces |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🌱️creates |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🗑️deletes |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🏷️renames |
      | replace-access-rule                | 🔑️access-rule/♻️replace/♻️replaces |
      | create-privacy-requirement         | 🔒️privacy/🌱️create/🌱️creates |
      | delete-privacy-requirement         | 🔒️privacy/🗑️delete/🗑️deletes |
      | rename-privacy-requirement         | 🔒️privacy/🏷️rename/🏷️renames |
      | replace-privacy-requirement        | 🔒️privacy/♻️replace/♻️replaces |
      | create-relationship                | 🕸️relationship/🌱️create/🌱️creates |
      | delete-relationship                | 🕸️relationship/🗑️delete/🗑️deletes |
      | rename-relationship                | 🕸️relationship/🏷️rename/🏷️renames |
      | replace-relationship               | 🕸️relationship/♻️replace/♻️replaces |
      | create-quantity-requirement        | 🔢️quantity/🌱️create/🌱️creates |
      | delete-quantity-requirement        | 🔢️quantity/🗑️delete/🗑️deletes |
      | rename-quantity-requirement        | 🔢️quantity/🏷️rename/🏷️renames |
      | replace-quantity-requirement       | 🔢️quantity/♻️replace/♻️replaces |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🌱️creates |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🗑️deletes |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🏷️renames |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/♻️replaces |
      | create-storage-requirement         | 🗄️storage/🌱️create/🌱️creates |
      | delete-storage-requirement         | 🗄️storage/🗑️delete/🗑️deletes |
      | rename-storage-requirement         | 🗄️storage/🏷️rename/🏷️renames |
      | replace-storage-requirement        | 🗄️storage/♻️replace/♻️replaces |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🌱️creates |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🗑️deletes |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🏷️renames |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/♻️replaces |
      | create-survey                      | 🗳️survey/🌱️create/🌱️creates |
      | delete-survey                      | 🗳️survey/🗑️delete/🗑️deletes |
      | rename-survey                      | 🗳️survey/🏷️rename/🏷️renames |
      | replace-survey                     | 🗳️survey/♻️replace/♻️replaces |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🌱️creates |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🗑️deletes |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🏷️renames |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/♻️replaces |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🌱️creates |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🗑️deletes |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🏷️renames |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/♻️replaces |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🌱️creates |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🗑️deletes |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🏷️renames |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/♻️replaces |
      | create-service-requirement         | 🛎️service/🌱️create/🌱️creates |
      | delete-service-requirement         | 🛎️service/🗑️delete/🗑️deletes |
      | rename-service-requirement         | 🛎️service/🏷️rename/🏷️renames |
      | replace-service-requirement        | 🛎️service/♻️replace/♻️replaces |
      | create-equipment                   | 🛠️equipment/🌱️create/🌱️creates |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🗑️deletes |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🏷️renames |
      | replace-equipment                  | 🛠️equipment/♻️replace/♻️replaces |
      | create-security-requirement        | 🛡️security/🌱️create/🌱️creates |
      | delete-security-requirement        | 🛡️security/🗑️delete/🗑️deletes |
      | rename-security-requirement        | 🛡️security/🏷️rename/🏷️renames |
      | replace-security-requirement       | 🛡️security/♻️replace/♻️replaces |
      | create-collaboration-record        | 🤝️collaboration/🌱️create/🌱️creates |
      | delete-collaboration-record        | 🤝️collaboration/🗑️delete/🗑️deletes |
      | rename-collaboration-record        | 🤝️collaboration/🏷️rename/🏷️renames |
      | replace-collaboration-record       | 🤝️collaboration/♻️replace/♻️replaces |
      | create-safety-requirement          | 🦺️safety/🌱️create/🌱️creates |
      | delete-safety-requirement          | 🦺️safety/🗑️delete/🗑️deletes |
      | rename-safety-requirement          | 🦺️safety/🏷️rename/🏷️renames |
      | replace-safety-requirement         | 🦺️safety/♻️replace/♻️replaces |
      | create-user-profile                | 🧑️user/🌱️create/🌱️creates |
      | delete-user-profile                | 🧑️user/🗑️delete/🗑️deletes |
      | rename-user-profile                | 🧑️user/🏷️rename/🏷️renames |
      | replace-user-profile               | 🧑️user/♻️replace/♻️replaces |
      | create-human-factor-requirement    | 🧠️human/🌱️create/🌱️creates |
      | delete-human-factor-requirement    | 🧠️human/🗑️delete/🗑️deletes |
      | rename-human-factor-requirement    | 🧠️human/🏷️rename/🏷️renames |
      | replace-human-factor-requirement   | 🧠️human/♻️replace/♻️replaces |
      | create-flexibility-requirement     | 🧩️flexibility/🌱️create/🌱️creates |
      | delete-flexibility-requirement     | 🧩️flexibility/🗑️delete/🗑️deletes |
      | rename-flexibility-requirement     | 🧩️flexibility/🏷️rename/🏷️renames |
      | replace-flexibility-requirement    | 🧩️flexibility/♻️replace/♻️replaces |
      | create-wayfinding-requirement      | 🧭️wayfinding/🌱️create/🌱️creates |
      | delete-wayfinding-requirement      | 🧭️wayfinding/🗑️delete/🗑️deletes |
      | rename-wayfinding-requirement      | 🧭️wayfinding/🏷️rename/🏷️renames |
      | replace-wayfinding-requirement     | 🧭️wayfinding/♻️replace/♻️replaces |
      | create-program-element             | 🧱️program-element/🌱️create/🌱️creates |
      | delete-program-element             | 🧱️program-element/🗑️delete/🗑️deletes |
      | rename-program-element             | 🧱️program-element/🏷️rename/🏷️renames |
      | replace-program-element            | 🧱️program-element/♻️replace/♻️replaces |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧲️reception |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🫷️reception |
      | connect-trace                      | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/✂️requirement |
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
      | create-information-requirement     | ℹ️information/🌱️create/🌱️creates |
      | delete-information-requirement     | ℹ️information/🗑️delete/🗑️deletes |
      | rename-information-requirement     | ℹ️information/🏷️rename/🏷️renames |
      | replace-information-requirement    | ℹ️information/♻️replace/♻️replaces |
      | create-sustainability-requirement  | ♻️sustainability/🌱️create/🌱️creates |
      | delete-sustainability-requirement  | ♻️sustainability/🗑️delete/🗑️deletes |
      | rename-sustainability-requirement  | ♻️sustainability/🏷️rename/🏷️renames |
      | replace-sustainability-requirement | ♻️sustainability/♻️replace/♻️replaces |
      | create-accessibility-requirement   | ♿️accessibility/🌱️create/🌱️creates |
      | delete-accessibility-requirement   | ♿️accessibility/🗑️delete/🗑️deletes |
      | rename-accessibility-requirement   | ♿️accessibility/🏷️rename/🏷️renames |
      | replace-accessibility-requirement  | ♿️accessibility/♻️replace/♻️replaces |
      | create-conflict                    | ⚔️conflict/🌱️create/🌱️creates |
      | delete-conflict                    | ⚔️conflict/🗑️delete/🗑️deletes |
      | rename-conflict                    | ⚔️conflict/🏷️rename/🏷️renames |
      | replace-conflict                   | ⚔️conflict/♻️replace/♻️replaces |
      | create-option-evaluation           | ⚖️option/🌱️create/🌱️creates |
      | delete-option-evaluation           | ⚖️option/🗑️delete/🗑️deletes |
      | rename-option-evaluation           | ⚖️option/🏷️rename/🏷️renames |
      | replace-option-evaluation          | ⚖️option/♻️replace/♻️replaces |
      | create-function                    | ⚙️function/🌱️create/🌱️creates |
      | delete-function                    | ⚙️function/🗑️delete/🗑️deletes |
      | rename-function                    | ⚙️function/🏷️rename/🏷️renames |
      | replace-function                   | ⚙️function/♻️replace/♻️replaces |
      | create-risk                        | ⚠️risk/🌱️create/🌱️creates |
      | delete-risk                        | ⚠️risk/🗑️delete/🗑️deletes |
      | rename-risk                        | ⚠️risk/🏷️rename/🏷️renames |
      | replace-risk                       | ⚠️risk/♻️replace/♻️replaces |
      | create-decision                    | ✅️decision/🌱️create/🌱️creates |
      | delete-decision                    | ✅️decision/🗑️delete/🗑️deletes |
      | rename-decision                    | ✅️decision/🏷️rename/🏷️renames |
      | replace-decision                   | ✅️decision/♻️replace/♻️replaces |
      | create-validation-record           | ✔️validation-record/🌱️create/🌱️creates |
      | delete-validation-record           | ✔️validation-record/🗑️delete/🗑️deletes |
      | rename-validation-record           | ✔️validation-record/🏷️rename/🏷️renames |
      | replace-validation-record          | ✔️validation-record/♻️replace/♻️replaces |
      | create-priority-record             | ⭐️priority-record/🌱️create/🌱️creates |
      | delete-priority-record             | ⭐️priority-record/🗑️delete/🗑️deletes |
      | rename-priority-record             | ⭐️priority-record/🏷️rename/🏷️renames |
      | replace-priority-record            | ⭐️priority-record/♻️replace/♻️replaces |
      | create-flow-requirement            | 🌊️flow/🌱️create/🌱️creates |
      | delete-flow-requirement            | 🌊️flow/🗑️delete/🗑️deletes |
      | rename-flow-requirement            | 🌊️flow/🏷️rename/🏷️renames |
      | replace-flow-requirement           | 🌊️flow/♻️replace/♻️replaces |
      | create-environmental-requirement   | 🌿️environmental/🌱️create/🌱️creates |
      | delete-environmental-requirement   | 🌿️environmental/🗑️delete/🗑️deletes |
      | rename-environmental-requirement   | 🌿️environmental/🏷️rename/🏷️renames |
      | replace-environmental-requirement  | 🌿️environmental/♻️replace/♻️replaces |
      | create-workshop                    | 🎓️workshop/🌱️create/🌱️creates |
      | delete-workshop                    | 🎓️workshop/🗑️delete/🗑️deletes |
      | rename-workshop                    | 🎓️workshop/🏷️rename/🏷️renames |
      | replace-workshop                   | 🎓️workshop/♻️replace/♻️replaces |
      | create-scenario                    | 🎬️scenario/🌱️create/🌱️creates |
      | delete-scenario                    | 🎬️scenario/🗑️delete/🗑️deletes |
      | rename-scenario                    | 🎬️scenario/🏷️rename/🏷️renames |
      | replace-scenario                   | 🎬️scenario/♻️replace/♻️replaces |
      | create-benchmark-record            | 🏁️benchmark-record/🌱️create/🌱️creates |
      | delete-benchmark-record            | 🏁️benchmark-record/🗑️delete/🚫️absent-a |
      | rename-benchmark-record            | 🏁️benchmark-record/🏷️rename/🚫️absent-a |
      | replace-benchmark-record           | 🏁️benchmark-record/♻️replace/🚫️absent-a |
      | create-activity                    | 🏃️activity/🌱️create/🌱️creates                                            |
      | delete-activity                    | 🏃️activity/🗑️delete/🗑️deletes                                            |
      | rename-activity                    | 🏃️activity/🏷️rename/🏷️renames                                            |
      | replace-activity                   | 🏃️activity/♻️replace/♻️replaces                                           |
      | create-infrastructure-requirement  | 🏗️infrastructure/🌱️create/🌱️creates |
      | delete-infrastructure-requirement  | 🏗️infrastructure/🗑️delete/🗑️deletes |
      | rename-infrastructure-requirement  | 🏗️infrastructure/🏷️rename/🏷️renames |
      | replace-infrastructure-requirement | 🏗️infrastructure/♻️replace/♻️replaces |
      | create-organizational-requirement  | 🏢️organizational/🌱️create/🌱️creates |
      | delete-organizational-requirement  | 🏢️organizational/🗑️delete/🗑️deletes |
      | rename-organizational-requirement  | 🏢️organizational/🏷️rename/🏷️renames |
      | replace-organizational-requirement | 🏢️organizational/♻️replace/♻️replaces |
      | create-issue                       | 🐛️issue/🌱️create/🌱️creates |
      | delete-issue                       | 🐛️issue/🗑️delete/🗑️deletes |
      | rename-issue                       | 🐛️issue/🏷️rename/🏷️renames |
      | replace-issue                      | 🐛️issue/♻️replace/♻️replaces |
      | create-approval-record             | 👍️approval-record/🌱️create/🌱️creates |
      | delete-approval-record             | 👍️approval-record/🗑️delete/🗑️deletes |
      | rename-approval-record             | 👍️approval-record/🏷️rename/🏷️renames |
      | replace-approval-record            | 👍️approval-record/♻️replace/♻️replaces |
      | create-stakeholder                 | 👥️stakeholder/🌱️create/🌱️creates |
      | delete-stakeholder                 | 👥️stakeholder/🗑️delete/🗑️deletes |
      | rename-stakeholder                 | 👥️stakeholder/🏷️rename/🏷️renames |
      | replace-stakeholder                | 👥️stakeholder/♻️replace/♻️replaces |
      | create-quality-record              | 💎️quality-record/🌱️create/🌱️creates |
      | delete-quality-record              | 💎️quality-record/🗑️delete/🗑️deletes |
      | rename-quality-record              | 💎️quality-record/🏷️rename/🏷️renames |
      | replace-quality-record             | 💎️quality-record/♻️replace/♻️replaces |
      | create-resilience-requirement      | 💪️resilience/🌱️create/🌱️creates |
      | delete-resilience-requirement      | 💪️resilience/🗑️delete/🗑️deletes |
      | rename-resilience-requirement      | 💪️resilience/🏷️rename/🏷️renames |
      | replace-resilience-requirement     | 💪️resilience/♻️replace/♻️replaces |
      | create-assumption                  | 💭️assumption/🌱️create/🌱️creates |
      | delete-assumption                  | 💭️assumption/🗑️delete/🗑️deletes |
      | rename-assumption                  | 💭️assumption/🏷️rename/🏷️renames |
      | replace-assumption                 | 💭️assumption/♻️replace/♻️replaces |
      | create-cost-requirement            | 💰️cost/🌱️create/🌱️creates |
      | delete-cost-requirement            | 💰️cost/🗑️delete/🗑️deletes |
      | rename-cost-requirement            | 💰️cost/🏷️rename/🏷️renames |
      | replace-cost-requirement           | 💰️cost/♻️replace/♻️replaces |
      | create-document                    | 📃️document/🌱️create/🌱️creates |
      | delete-document                    | 📃️document/🗑️delete/🗑️deletes |
      | rename-document                    | 📃️document/🏷️rename/🏷️renames |
      | replace-document                   | 📃️document/♻️replace/♻️replaces |
      | create-schedule-requirement        | 📅️schedule/🌱️create/🌱️creates |
      | delete-schedule-requirement        | 📅️schedule/🗑️delete/🗑️deletes |
      | rename-schedule-requirement        | 📅️schedule/🏷️rename/🏷️renames |
      | replace-schedule-requirement       | 📅️schedule/♻️replace/♻️replaces |
      | create-growth-plan                 | 📈️growth-plan/🌱️create/🌱️creates |
      | delete-growth-plan                 | 📈️growth-plan/🗑️delete/🗑️deletes |
      | rename-growth-plan                 | 📈️growth-plan/🏷️rename/🏷️renames |
      | replace-growth-plan                | 📈️growth-plan/♻️replace/♻️replaces |
      | create-performance-criterion       | 📊️performance/🌱️create/🌱️creates |
      | delete-performance-criterion       | 📊️performance/🗑️delete/🗑️deletes |
      | rename-performance-criterion       | 📊️performance/🏷️rename/🏷️renames |
      | replace-performance-criterion      | 📊️performance/♻️replace/♻️replaces |
      | create-operational-requirement     | 📋️operational/🌱️create/🌱️creates |
      | delete-operational-requirement     | 📋️operational/🗑️delete/🗑️deletes |
      | rename-operational-requirement     | 📋️operational/🏷️rename/🏷️renames |
      | replace-operational-requirement    | 📋️operational/♻️replace/♻️replaces |
      | create-requirement                 | 📌️requirement/🌱️create/🌱️creates |
      | delete-requirement                 | 📌️requirement/🗑️delete/🗑️deletes |
      | rename-requirement                 | 📌️requirement/🏷️rename/🏷️renames |
      | replace-requirement                | 📌️requirement/♻️replace/♻️replaces |
      | create-site-context                | 📍️site/🌱️create/🌱️creates |
      | delete-site-context                | 📍️site/🗑️delete/🗑️deletes |
      | rename-site-context                | 📍️site/🏷️rename/🏷️renames |
      | replace-site-context               | 📍️site/♻️replace/♻️replaces |
      | create-template-record             | 📐️template-record/🌱️create/🌱️creates |
      | delete-template-record             | 📐️template-record/🗑️delete/🗑️deletes |
      | rename-template-record             | 📐️template-record/🏷️rename/🏷️renames |
      | replace-template-record            | 📐️template-record/♻️replace/♻️replaces |
      | create-report-record               | 📑️report-record/🌱️create/🌱️creates |
      | delete-report-record               | 📑️report-record/🗑️delete/🗑️deletes |
      | rename-report-record               | 📑️report-record/🏷️rename/🏷️renames |
      | replace-report-record              | 📑️report-record/♻️replace/♻️replaces |
      | create-audit-event                 | 📒️audit/🌱️create/🌱️creates |
      | delete-audit-event                 | 📒️audit/🗑️delete/🗑️deletes |
      | rename-audit-event                 | 📒️audit/🏷️rename/🏷️renames |
      | replace-audit-event                | 📒️audit/♻️replace/♻️replaces |
      | create-knowledge-record            | 📚️knowledge-record/🌱️create/🌱️creates |
      | delete-knowledge-record            | 📚️knowledge-record/🗑️delete/🚫️absent-a |
      | rename-knowledge-record            | 📚️knowledge-record/🏷️rename/🚫️absent-a |
      | replace-knowledge-record           | 📚️knowledge-record/♻️replace/🚫️absent-a |
      | create-regulatory-requirement      | 📜️regulatory/🌱️create/🌱️creates |
      | delete-regulatory-requirement      | 📜️regulatory/🗑️delete/🗑️deletes |
      | rename-regulatory-requirement      | 📜️regulatory/🏷️rename/🏷️renames |
      | replace-regulatory-requirement     | 📜️regulatory/♻️replace/♻️replaces |
      | create-change-record               | 🔀️change-record/🌱️create/🌱️creates |
      | delete-change-record               | 🔀️change-record/🗑️delete/🗑️deletes |
      | rename-change-record               | 🔀️change-record/🏷️rename/🏷️renames |
      | replace-change-record              | 🔀️change-record/♻️replace/♻️replaces |
      | create-communication-requirement   | 📡️communication/🌱️create/🌱️creates |
      | delete-communication-requirement   | 📡️communication/🗑️delete/🗑️deletes |
      | rename-communication-requirement   | 📡️communication/🏷️rename/🏷️renames |
      | replace-communication-requirement  | 📡️communication/♻️replace/♻️replaces |
      | create-resource                    | 📦️resource/🌱️create/🌱️creates |
      | delete-resource                    | 📦️resource/🗑️delete/🗑️deletes |
      | rename-resource                    | 📦️resource/🏷️rename/🏷️renames |
      | replace-resource                   | 📦️resource/♻️replace/♻️replaces |
      | create-status-record               | 📶️status-record/🌱️create/🌱️creates |
      | delete-status-record               | 📶️status-record/🗑️delete/🗑️deletes |
      | rename-status-record               | 📶️status-record/🏷️rename/🏷️renames |
      | replace-status-record              | 📶️status-record/♻️replace/♻️replaces |
      | create-process                     | 🔄️process/🌱️create/🌱️creates |
      | delete-process                     | 🔄️process/🗑️delete/🗑️deletes |
      | rename-process                     | 🔄️process/🏷️rename/🏷️renames |
      | replace-process                    | 🔄️process/♻️replace/♻️replaces |
      | create-search-filter               | 🔍️search-filter/🌱️create/🌱️creates |
      | delete-search-filter               | 🔍️search-filter/🗑️delete/🗑️deletes |
      | rename-search-filter               | 🔍️search-filter/🏷️rename/🏷️renames |
      | replace-search-filter              | 🔍️search-filter/♻️replace/♻️replaces |
      | create-access-rule                 | 🔑️access-rule/🌱️create/🌱️creates |
      | delete-access-rule                 | 🔑️access-rule/🗑️delete/🗑️deletes |
      | rename-access-rule                 | 🔑️access-rule/🏷️rename/🏷️renames |
      | replace-access-rule                | 🔑️access-rule/♻️replace/♻️replaces |
      | create-privacy-requirement         | 🔒️privacy/🌱️create/🌱️creates |
      | delete-privacy-requirement         | 🔒️privacy/🗑️delete/🗑️deletes |
      | rename-privacy-requirement         | 🔒️privacy/🏷️rename/🏷️renames |
      | replace-privacy-requirement        | 🔒️privacy/♻️replace/♻️replaces |
      | create-relationship                | 🕸️relationship/🌱️create/🌱️creates |
      | delete-relationship                | 🕸️relationship/🗑️delete/🗑️deletes |
      | rename-relationship                | 🕸️relationship/🏷️rename/🏷️renames |
      | replace-relationship               | 🕸️relationship/♻️replace/♻️replaces |
      | create-quantity-requirement        | 🔢️quantity/🌱️create/🌱️creates |
      | delete-quantity-requirement        | 🔢️quantity/🗑️delete/🗑️deletes |
      | rename-quantity-requirement        | 🔢️quantity/🏷️rename/🏷️renames |
      | replace-quantity-requirement       | 🔢️quantity/♻️replace/♻️replaces |
      | create-analysis-record             | 🔬️analysis-record/🌱️create/🌱️creates |
      | delete-analysis-record             | 🔬️analysis-record/🗑️delete/🗑️deletes |
      | rename-analysis-record             | 🔬️analysis-record/🏷️rename/🏷️renames |
      | replace-analysis-record            | 🔬️analysis-record/♻️replace/♻️replaces |
      | create-storage-requirement         | 🗄️storage/🌱️create/🌱️creates |
      | delete-storage-requirement         | 🗄️storage/🗑️delete/🗑️deletes |
      | rename-storage-requirement         | 🗄️storage/🏷️rename/🏷️renames |
      | replace-storage-requirement        | 🗄️storage/♻️replace/♻️replaces |
      | create-meeting-record              | 🗓️meeting-record/🌱️create/🌱️creates |
      | delete-meeting-record              | 🗓️meeting-record/🗑️delete/🗑️deletes |
      | rename-meeting-record              | 🗓️meeting-record/🏷️rename/🏷️renames |
      | replace-meeting-record             | 🗓️meeting-record/♻️replace/♻️replaces |
      | create-survey                      | 🗳️survey/🌱️create/🌱️creates |
      | delete-survey                      | 🗳️survey/🗑️delete/🗑️deletes |
      | rename-survey                      | 🗳️survey/🏷️rename/🏷️renames |
      | replace-survey                     | 🗳️survey/♻️replace/♻️replaces |
      | create-delivery-constraint         | 🚚️delivery-constraint/🌱️create/🌱️creates |
      | delete-delivery-constraint         | 🚚️delivery-constraint/🗑️delete/🗑️deletes |
      | rename-delivery-constraint         | 🚚️delivery-constraint/🏷️rename/🏷️renames |
      | replace-delivery-constraint        | 🚚️delivery-constraint/♻️replace/♻️replaces |
      | create-constraint-record           | 🚧️constraint-record/🌱️create/🌱️creates |
      | delete-constraint-record           | 🚧️constraint-record/🗑️delete/🗑️deletes |
      | rename-constraint-record           | 🚧️constraint-record/🏷️rename/🏷️renames |
      | replace-constraint-record          | 🚧️constraint-record/♻️replace/♻️replaces |
      | create-compliance-record           | 🛂️compliance-record/🌱️create/🌱️creates |
      | delete-compliance-record           | 🛂️compliance-record/🗑️delete/🗑️deletes |
      | rename-compliance-record           | 🛂️compliance-record/🏷️rename/🏷️renames |
      | replace-compliance-record          | 🛂️compliance-record/♻️replace/♻️replaces |
      | create-service-requirement         | 🛎️service/🌱️create/🌱️creates |
      | delete-service-requirement         | 🛎️service/🗑️delete/🗑️deletes |
      | rename-service-requirement         | 🛎️service/🏷️rename/🏷️renames |
      | replace-service-requirement        | 🛎️service/♻️replace/♻️replaces |
      | create-equipment                   | 🛠️equipment/🌱️create/🌱️creates |
      | delete-equipment                   | 🛠️equipment/🗑️delete/🗑️deletes |
      | rename-equipment                   | 🛠️equipment/🏷️rename/🏷️renames |
      | replace-equipment                  | 🛠️equipment/♻️replace/♻️replaces |
      | create-security-requirement        | 🛡️security/🌱️create/🌱️creates |
      | delete-security-requirement        | 🛡️security/🗑️delete/🗑️deletes |
      | rename-security-requirement        | 🛡️security/🏷️rename/🏷️renames |
      | replace-security-requirement       | 🛡️security/♻️replace/♻️replaces |
      | create-collaboration-record        | 🤝️collaboration/🌱️create/🌱️creates |
      | delete-collaboration-record        | 🤝️collaboration/🗑️delete/🗑️deletes |
      | rename-collaboration-record        | 🤝️collaboration/🏷️rename/🏷️renames |
      | replace-collaboration-record       | 🤝️collaboration/♻️replace/♻️replaces |
      | create-safety-requirement          | 🦺️safety/🌱️create/🌱️creates |
      | delete-safety-requirement          | 🦺️safety/🗑️delete/🗑️deletes |
      | rename-safety-requirement          | 🦺️safety/🏷️rename/🏷️renames |
      | replace-safety-requirement         | 🦺️safety/♻️replace/♻️replaces |
      | create-user-profile                | 🧑️user/🌱️create/🌱️creates |
      | delete-user-profile                | 🧑️user/🗑️delete/🗑️deletes |
      | rename-user-profile                | 🧑️user/🏷️rename/🏷️renames |
      | replace-user-profile               | 🧑️user/♻️replace/♻️replaces |
      | create-human-factor-requirement    | 🧠️human/🌱️create/🌱️creates |
      | delete-human-factor-requirement    | 🧠️human/🗑️delete/🗑️deletes |
      | rename-human-factor-requirement    | 🧠️human/🏷️rename/🏷️renames |
      | replace-human-factor-requirement   | 🧠️human/♻️replace/♻️replaces |
      | create-flexibility-requirement     | 🧩️flexibility/🌱️create/🌱️creates |
      | delete-flexibility-requirement     | 🧩️flexibility/🗑️delete/🗑️deletes |
      | rename-flexibility-requirement     | 🧩️flexibility/🏷️rename/🏷️renames |
      | replace-flexibility-requirement    | 🧩️flexibility/♻️replace/♻️replaces |
      | create-wayfinding-requirement      | 🧭️wayfinding/🌱️create/🌱️creates |
      | delete-wayfinding-requirement      | 🧭️wayfinding/🗑️delete/🗑️deletes |
      | rename-wayfinding-requirement      | 🧭️wayfinding/🏷️rename/🏷️renames |
      | replace-wayfinding-requirement     | 🧭️wayfinding/♻️replace/♻️replaces |
      | create-program-element             | 🧱️program-element/🌱️create/🌱️creates |
      | delete-program-element             | 🧱️program-element/🗑️delete/🗑️deletes |
      | rename-program-element             | 🧱️program-element/🏷️rename/🏷️renames |
      | replace-program-element            | 🧱️program-element/♻️replace/♻️replaces |
      | connect-adjacency                  | 🧲️adjacency/🧲️connect/🧲️reception |
      | disconnect-adjacency               | 🧲️adjacency/🫷️disconnect/🫷️reception |
      | connect-trace                      | 🧵️trace/🧵️connect/🧵️requirement-decision |
      | disconnect-trace                   | 🧵️trace/✂️disconnect/✂️requirement |
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
