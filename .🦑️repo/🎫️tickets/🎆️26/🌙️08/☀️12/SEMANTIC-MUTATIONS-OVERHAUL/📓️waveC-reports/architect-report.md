# Wave C — architect facet report

`facet`: `✏️s/🔌️plugins/🏛️architect` (whole plugin; crate `semio-s-plugin-architect`)
`status`: **partial** — source-complete for Phases 1–2 and most of Phase 3 (verified by direct
inspection and by the last cargo check that actually reached this crate), but the final
confirming `cargo check`/`cargo test` pass was **not completed** by this lane — abandoned per the
coordinator's explicit instruction to stop waiting on the shared `.cargo-build-lock` and let the
coordinator's consolidated pass verify. See `gates` below for the exact, honest state.

---

## Phase 1 — compile

Fixed all 105 baseline errors (catalog.rs macro-CRUD rewrite with a per-register lookup table;
8 app-command files re-routed from `SetAdjacency`/`ClearAdjacency`/`Elements(CollectionMutation::
Add|Remove)`/`Reports(...)`/`Analyses(...)`/`SetSnapshot` to the real semantic variants; two stale
app-root tests updated to assert on `ConnectAdjacency`/`ReplaceProgramElement`/`LoadDocument`
instead of the deleted shapes; one foreign-file one-line import fix — see the audit item below).

Files touched (Phase 1, beyond what Phase 2 also touches):
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🗂️catalog/🦀️component.rs` — `add_register_item_operation`/
  `remove_register_item_operation`/`patch_register_item_operation` rewritten around per-register
  `create!`/`delete!` macros plus a new `merge_json_patch` helper (JSON-Value shallow merge onto
  the existing row's serialized form, since `EntityHeader` is `#[serde(flatten)]`'d into every
  register row — verified this makes the merge correct for identity-field patches like `name`).
  `patch_register_item_operation` gained a `program: &ProgramSnapshot` parameter (needed to look
  up the pre-patch row; the old `CollectionMutation::Patch{id,patch}` shape didn't need one because
  patch application happened later, inside diff-apply).
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs` — added `reset_document_effect`
  (see audit item (a) below); fixed 3 tests; dropped the now-dead `use protocol::CollectionMutation;`.
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/{↔️adjacency,🕸️graph,🏗️element,🔬️analysis,📤️exchange,📋️register}/🦀️component.rs`
  — every `SetAdjacency`/`ClearAdjacency`/`Elements(CollectionMutation::*)`/`Reports(...)`/
  `Analyses(...)`/`SetSnapshot` construction site rewritten to the semantic variant via
  `use crate::artifacts::program::schema::mutations as leaves;` + `leaves::<slug>::mutation::<Type>`.
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/⚙️engine/📐️template/🦀️component.rs`
  — wave-2's own in-boundary fix re-pointed from old flat module names (`leaves::stakeholders::…`)
  to the new one-per-verb module names (`leaves::create_stakeholder::…`) after Phase 2 restructuring.
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
  — see audit item (b) below.

## Phase 2 — directory/glue restructure

Split all 72 pre-migration noun-keyed triad directories (66 real + `🔀adjacencies`/`🖼️set-snapshot`
orphan stubs, deleted) into **266 one-triad-dir-per-variant directories**, 1:1 with the dispatch
enum — verified programmatically (dispatch-enum variant PascalCase names vs. triad-dir kebab stems,
both directions, zero mismatches).

Mechanism: this was scripted (not hand-authored file-by-file) — a Python generator
(`.🦑️repo/…/SEMANTIC-MUTATIONS-OVERHAUL/scratch-architect/{migrate.py,generate.py}`, left in the
ticket folder per house rules) parsed each old triad's `//#region 🔖️<Struct>` blocks and
`diff_<verb>`/`inverse_<verb>` function bodies (brace-matched, not regex-guessed), split them 1:1
into new files, renamed the delegate calls to the recipe's plain `diff`/`inverse` names, and
resolved cross-triad references (e.g. `delete-stakeholder`'s inverse constructing a
`create-stakeholder` payload) to fully-qualified `super::super::<module>::mutation::<Type>` paths —
the same inline-qualification convention the pre-existing `🗺️set-adjacency`/`🧹clear-adjacency`
dirs already used, now generalized. Verified brace-balance and no leftover `diff_<verb>`/
`inverse_<verb>` function names across all 798 generated `.rs` leaves.

**Emoji scheme**: `<verb-emoji><entity-emoji><kebab-slug>`. Verb emoji (6, shared across all dirs
of that verb): `create`=🌱 `delete`=🗑️ `rename`=✏️ `replace`=🔁 `connect`=🔗 `disconnect`=✂️. Entity
emoji (69, one per register/facet/edge-concept, mostly reused from the pre-migration 72-dir names
so they stay recognizable) is what actually carries uniqueness — verified **programmatically that
all 266 leading-emoji-prefixes are pairwise distinct** (script output: `total dirs: 266`,
`unique prefixes: 266`, `dupes: []`). Full 266-row table below for direct audit (no re-derivation
needed).

<details><summary>Full emoji/directory/variant table (266 rows)</summary>

| # | emoji prefix | directory | variant |
|---|---|---|---|
| 1 | `🌱ℹ️` | `🌱ℹ️create-information-requirement` | `CreateInformationRequirement` |
| 2 | `🗑️ℹ️` | `🗑️ℹ️delete-information-requirement` | `DeleteInformationRequirement` |
| 3 | `✏️ℹ️` | `✏️ℹ️rename-information-requirement` | `RenameInformationRequirement` |
| 4 | `🔁ℹ️` | `🔁ℹ️replace-information-requirement` | `ReplaceInformationRequirement` |
| 5 | `🌱♻️` | `🌱♻️create-sustainability-requirement` | `CreateSustainabilityRequirement` |
| 6 | `🗑️♻️` | `🗑️♻️delete-sustainability-requirement` | `DeleteSustainabilityRequirement` |
| 7 | `✏️♻️` | `✏️♻️rename-sustainability-requirement` | `RenameSustainabilityRequirement` |
| 8 | `🔁♻️` | `🔁♻️replace-sustainability-requirement` | `ReplaceSustainabilityRequirement` |
| 9 | `🌱♿` | `🌱♿create-accessibility-requirement` | `CreateAccessibilityRequirement` |
| 10 | `🗑️♿` | `🗑️♿delete-accessibility-requirement` | `DeleteAccessibilityRequirement` |
| 11 | `✏️♿` | `✏️♿rename-accessibility-requirement` | `RenameAccessibilityRequirement` |
| 12 | `🔁♿` | `🔁♿replace-accessibility-requirement` | `ReplaceAccessibilityRequirement` |
| 13 | `🌱⚔️` | `🌱⚔️create-conflict` | `CreateConflict` |
| 14 | `🗑️⚔️` | `🗑️⚔️delete-conflict` | `DeleteConflict` |
| 15 | `✏️⚔️` | `✏️⚔️rename-conflict` | `RenameConflict` |
| 16 | `🔁⚔️` | `🔁⚔️replace-conflict` | `ReplaceConflict` |
| 17 | `🌱⚖️` | `🌱⚖️create-option-evaluation` | `CreateOptionEvaluation` |
| 18 | `🗑️⚖️` | `🗑️⚖️delete-option-evaluation` | `DeleteOptionEvaluation` |
| 19 | `✏️⚖️` | `✏️⚖️rename-option-evaluation` | `RenameOptionEvaluation` |
| 20 | `🔁⚖️` | `🔁⚖️replace-option-evaluation` | `ReplaceOptionEvaluation` |
| 21 | `🌱⚙️` | `🌱⚙️create-function` | `CreateFunction` |
| 22 | `🗑️⚙️` | `🗑️⚙️delete-function` | `DeleteFunction` |
| 23 | `✏️⚙️` | `✏️⚙️rename-function` | `RenameFunction` |
| 24 | `🔁⚙️` | `🔁⚙️replace-function` | `ReplaceFunction` |
| 25 | `🌱⚠️` | `🌱⚠️create-risk` | `CreateRisk` |
| 26 | `🗑️⚠️` | `🗑️⚠️delete-risk` | `DeleteRisk` |
| 27 | `✏️⚠️` | `✏️⚠️rename-risk` | `RenameRisk` |
| 28 | `🔁⚠️` | `🔁⚠️replace-risk` | `ReplaceRisk` |
| 29 | `🌱✅` | `🌱✅create-decision` | `CreateDecision` |
| 30 | `🗑️✅` | `🗑️✅delete-decision` | `DeleteDecision` |
| 31 | `✏️✅` | `✏️✅rename-decision` | `RenameDecision` |
| 32 | `🔁✅` | `🔁✅replace-decision` | `ReplaceDecision` |
| 33 | `🌱✔️` | `🌱✔️create-validation-record` | `CreateValidationRecord` |
| 34 | `🗑️✔️` | `🗑️✔️delete-validation-record` | `DeleteValidationRecord` |
| 35 | `✏️✔️` | `✏️✔️rename-validation-record` | `RenameValidationRecord` |
| 36 | `🔁✔️` | `🔁✔️replace-validation-record` | `ReplaceValidationRecord` |
| 37 | `🌱⭐` | `🌱⭐create-priority-record` | `CreatePriorityRecord` |
| 38 | `🗑️⭐` | `🗑️⭐delete-priority-record` | `DeletePriorityRecord` |
| 39 | `✏️⭐` | `✏️⭐rename-priority-record` | `RenamePriorityRecord` |
| 40 | `🔁⭐` | `🔁⭐replace-priority-record` | `ReplacePriorityRecord` |
| 41 | `🌱🌊` | `🌱🌊create-flow-requirement` | `CreateFlowRequirement` |
| 42 | `🗑️🌊` | `🗑️🌊delete-flow-requirement` | `DeleteFlowRequirement` |
| 43 | `✏️🌊` | `✏️🌊rename-flow-requirement` | `RenameFlowRequirement` |
| 44 | `🔁🌊` | `🔁🌊replace-flow-requirement` | `ReplaceFlowRequirement` |
| 45 | `🌱🌿` | `🌱🌿create-environmental-requirement` | `CreateEnvironmentalRequirement` |
| 46 | `🗑️🌿` | `🗑️🌿delete-environmental-requirement` | `DeleteEnvironmentalRequirement` |
| 47 | `✏️🌿` | `✏️🌿rename-environmental-requirement` | `RenameEnvironmentalRequirement` |
| 48 | `🔁🌿` | `🔁🌿replace-environmental-requirement` | `ReplaceEnvironmentalRequirement` |
| 49 | `🌱🎓` | `🌱🎓create-workshop` | `CreateWorkshop` |
| 50 | `🗑️🎓` | `🗑️🎓delete-workshop` | `DeleteWorkshop` |
| 51 | `✏️🎓` | `✏️🎓rename-workshop` | `RenameWorkshop` |
| 52 | `🔁🎓` | `🔁🎓replace-workshop` | `ReplaceWorkshop` |
| 53 | `🌱🎬` | `🌱🎬create-scenario` | `CreateScenario` |
| 54 | `🗑️🎬` | `🗑️🎬delete-scenario` | `DeleteScenario` |
| 55 | `✏️🎬` | `✏️🎬rename-scenario` | `RenameScenario` |
| 56 | `🔁🎬` | `🔁🎬replace-scenario` | `ReplaceScenario` |
| 57 | `🌱🏁` | `🌱🏁create-benchmark-record` | `CreateBenchmarkRecord` |
| 58 | `🗑️🏁` | `🗑️🏁delete-benchmark-record` | `DeleteBenchmarkRecord` |
| 59 | `✏️🏁` | `✏️🏁rename-benchmark-record` | `RenameBenchmarkRecord` |
| 60 | `🔁🏁` | `🔁🏁replace-benchmark-record` | `ReplaceBenchmarkRecord` |
| 61 | `🌱🏃` | `🌱🏃create-activity` | `CreateActivity` |
| 62 | `🗑️🏃` | `🗑️🏃delete-activity` | `DeleteActivity` |
| 63 | `✏️🏃` | `✏️🏃rename-activity` | `RenameActivity` |
| 64 | `🔁🏃` | `🔁🏃replace-activity` | `ReplaceActivity` |
| 65 | `🌱🏗️` | `🌱🏗️create-infrastructure-requirement` | `CreateInfrastructureRequirement` |
| 66 | `🗑️🏗️` | `🗑️🏗️delete-infrastructure-requirement` | `DeleteInfrastructureRequirement` |
| 67 | `✏️🏗️` | `✏️🏗️rename-infrastructure-requirement` | `RenameInfrastructureRequirement` |
| 68 | `🔁🏗️` | `🔁🏗️replace-infrastructure-requirement` | `ReplaceInfrastructureRequirement` |
| 69 | `🌱🏢` | `🌱🏢create-organizational-requirement` | `CreateOrganizationalRequirement` |
| 70 | `🗑️🏢` | `🗑️🏢delete-organizational-requirement` | `DeleteOrganizationalRequirement` |
| 71 | `✏️🏢` | `✏️🏢rename-organizational-requirement` | `RenameOrganizationalRequirement` |
| 72 | `🔁🏢` | `🔁🏢replace-organizational-requirement` | `ReplaceOrganizationalRequirement` |
| 73 | `🌱🐛` | `🌱🐛create-issue` | `CreateIssue` |
| 74 | `🗑️🐛` | `🗑️🐛delete-issue` | `DeleteIssue` |
| 75 | `✏️🐛` | `✏️🐛rename-issue` | `RenameIssue` |
| 76 | `🔁🐛` | `🔁🐛replace-issue` | `ReplaceIssue` |
| 77 | `🌱👍` | `🌱👍create-approval-record` | `CreateApprovalRecord` |
| 78 | `🗑️👍` | `🗑️👍delete-approval-record` | `DeleteApprovalRecord` |
| 79 | `✏️👍` | `✏️👍rename-approval-record` | `RenameApprovalRecord` |
| 80 | `🔁👍` | `🔁👍replace-approval-record` | `ReplaceApprovalRecord` |
| 81 | `🌱👥` | `🌱👥create-stakeholder` | `CreateStakeholder` |
| 82 | `🗑️👥` | `🗑️👥delete-stakeholder` | `DeleteStakeholder` |
| 83 | `✏️👥` | `✏️👥rename-stakeholder` | `RenameStakeholder` |
| 84 | `🔁👥` | `🔁👥replace-stakeholder` | `ReplaceStakeholder` |
| 85 | `🌱💎` | `🌱💎create-quality-record` | `CreateQualityRecord` |
| 86 | `🗑️💎` | `🗑️💎delete-quality-record` | `DeleteQualityRecord` |
| 87 | `✏️💎` | `✏️💎rename-quality-record` | `RenameQualityRecord` |
| 88 | `🔁💎` | `🔁💎replace-quality-record` | `ReplaceQualityRecord` |
| 89 | `🌱💪` | `🌱💪create-resilience-requirement` | `CreateResilienceRequirement` |
| 90 | `🗑️💪` | `🗑️💪delete-resilience-requirement` | `DeleteResilienceRequirement` |
| 91 | `✏️💪` | `✏️💪rename-resilience-requirement` | `RenameResilienceRequirement` |
| 92 | `🔁💪` | `🔁💪replace-resilience-requirement` | `ReplaceResilienceRequirement` |
| 93 | `🌱💭` | `🌱💭create-assumption` | `CreateAssumption` |
| 94 | `🗑️💭` | `🗑️💭delete-assumption` | `DeleteAssumption` |
| 95 | `✏️💭` | `✏️💭rename-assumption` | `RenameAssumption` |
| 96 | `🔁💭` | `🔁💭replace-assumption` | `ReplaceAssumption` |
| 97 | `🌱💰` | `🌱💰create-cost-requirement` | `CreateCostRequirement` |
| 98 | `🗑️💰` | `🗑️💰delete-cost-requirement` | `DeleteCostRequirement` |
| 99 | `✏️💰` | `✏️💰rename-cost-requirement` | `RenameCostRequirement` |
| 100 | `🔁💰` | `🔁💰replace-cost-requirement` | `ReplaceCostRequirement` |
| 101 | `🌱📄` | `🌱📄create-document` | `CreateDocument` |
| 102 | `🗑️📄` | `🗑️📄delete-document` | `DeleteDocument` |
| 103 | `✏️📄` | `✏️📄rename-document` | `RenameDocument` |
| 104 | `🔁📄` | `🔁📄replace-document` | `ReplaceDocument` |
| 105 | `🌱📅` | `🌱📅create-schedule-requirement` | `CreateScheduleRequirement` |
| 106 | `🗑️📅` | `🗑️📅delete-schedule-requirement` | `DeleteScheduleRequirement` |
| 107 | `✏️📅` | `✏️📅rename-schedule-requirement` | `RenameScheduleRequirement` |
| 108 | `🔁📅` | `🔁📅replace-schedule-requirement` | `ReplaceScheduleRequirement` |
| 109 | `🌱📈` | `🌱📈create-growth-plan` | `CreateGrowthPlan` |
| 110 | `🗑️📈` | `🗑️📈delete-growth-plan` | `DeleteGrowthPlan` |
| 111 | `✏️📈` | `✏️📈rename-growth-plan` | `RenameGrowthPlan` |
| 112 | `🔁📈` | `🔁📈replace-growth-plan` | `ReplaceGrowthPlan` |
| 113 | `🌱📊` | `🌱📊create-performance-criterion` | `CreatePerformanceCriterion` |
| 114 | `🗑️📊` | `🗑️📊delete-performance-criterion` | `DeletePerformanceCriterion` |
| 115 | `✏️📊` | `✏️📊rename-performance-criterion` | `RenamePerformanceCriterion` |
| 116 | `🔁📊` | `🔁📊replace-performance-criterion` | `ReplacePerformanceCriterion` |
| 117 | `🌱📋` | `🌱📋create-operational-requirement` | `CreateOperationalRequirement` |
| 118 | `🗑️📋` | `🗑️📋delete-operational-requirement` | `DeleteOperationalRequirement` |
| 119 | `✏️📋` | `✏️📋rename-operational-requirement` | `RenameOperationalRequirement` |
| 120 | `🔁📋` | `🔁📋replace-operational-requirement` | `ReplaceOperationalRequirement` |
| 121 | `🌱📌` | `🌱📌create-requirement` | `CreateRequirement` |
| 122 | `🗑️📌` | `🗑️📌delete-requirement` | `DeleteRequirement` |
| 123 | `✏️📌` | `✏️📌rename-requirement` | `RenameRequirement` |
| 124 | `🔁📌` | `🔁📌replace-requirement` | `ReplaceRequirement` |
| 125 | `🌱📍` | `🌱📍create-site-context` | `CreateSiteContext` |
| 126 | `🗑️📍` | `🗑️📍delete-site-context` | `DeleteSiteContext` |
| 127 | `✏️📍` | `✏️📍rename-site-context` | `RenameSiteContext` |
| 128 | `🔁📍` | `🔁📍replace-site-context` | `ReplaceSiteContext` |
| 129 | `🌱📐` | `🌱📐create-template-record` | `CreateTemplateRecord` |
| 130 | `🗑️📐` | `🗑️📐delete-template-record` | `DeleteTemplateRecord` |
| 131 | `✏️📐` | `✏️📐rename-template-record` | `RenameTemplateRecord` |
| 132 | `🔁📐` | `🔁📐replace-template-record` | `ReplaceTemplateRecord` |
| 133 | `🌱📑` | `🌱📑create-report-record` | `CreateReportRecord` |
| 134 | `🗑️📑` | `🗑️📑delete-report-record` | `DeleteReportRecord` |
| 135 | `✏️📑` | `✏️📑rename-report-record` | `RenameReportRecord` |
| 136 | `🔁📑` | `🔁📑replace-report-record` | `ReplaceReportRecord` |
| 137 | `🌱📒` | `🌱📒create-audit-event` | `CreateAuditEvent` |
| 138 | `🗑️📒` | `🗑️📒delete-audit-event` | `DeleteAuditEvent` |
| 139 | `✏️📒` | `✏️📒rename-audit-event` | `RenameAuditEvent` |
| 140 | `🔁📒` | `🔁📒replace-audit-event` | `ReplaceAuditEvent` |
| 141 | `🌱📚` | `🌱📚create-knowledge-record` | `CreateKnowledgeRecord` |
| 142 | `🗑️📚` | `🗑️📚delete-knowledge-record` | `DeleteKnowledgeRecord` |
| 143 | `✏️📚` | `✏️📚rename-knowledge-record` | `RenameKnowledgeRecord` |
| 144 | `🔁📚` | `🔁📚replace-knowledge-record` | `ReplaceKnowledgeRecord` |
| 145 | `🌱📜` | `🌱📜create-regulatory-requirement` | `CreateRegulatoryRequirement` |
| 146 | `🗑️📜` | `🗑️📜delete-regulatory-requirement` | `DeleteRegulatoryRequirement` |
| 147 | `✏️📜` | `✏️📜rename-regulatory-requirement` | `RenameRegulatoryRequirement` |
| 148 | `🔁📜` | `🔁📜replace-regulatory-requirement` | `ReplaceRegulatoryRequirement` |
| 149 | `🌱📝` | `🌱📝create-change-record` | `CreateChangeRecord` |
| 150 | `🗑️📝` | `🗑️📝delete-change-record` | `DeleteChangeRecord` |
| 151 | `✏️📝` | `✏️📝rename-change-record` | `RenameChangeRecord` |
| 152 | `🔁📝` | `🔁📝replace-change-record` | `ReplaceChangeRecord` |
| 153 | `🌱📡` | `🌱📡create-communication-requirement` | `CreateCommunicationRequirement` |
| 154 | `🗑️📡` | `🗑️📡delete-communication-requirement` | `DeleteCommunicationRequirement` |
| 155 | `✏️📡` | `✏️📡rename-communication-requirement` | `RenameCommunicationRequirement` |
| 156 | `🔁📡` | `🔁📡replace-communication-requirement` | `ReplaceCommunicationRequirement` |
| 157 | `🌱📦` | `🌱📦create-resource` | `CreateResource` |
| 158 | `🗑️📦` | `🗑️📦delete-resource` | `DeleteResource` |
| 159 | `✏️📦` | `✏️📦rename-resource` | `RenameResource` |
| 160 | `🔁📦` | `🔁📦replace-resource` | `ReplaceResource` |
| 161 | `🌱📶` | `🌱📶create-status-record` | `CreateStatusRecord` |
| 162 | `🗑️📶` | `🗑️📶delete-status-record` | `DeleteStatusRecord` |
| 163 | `✏️📶` | `✏️📶rename-status-record` | `RenameStatusRecord` |
| 164 | `🔁📶` | `🔁📶replace-status-record` | `ReplaceStatusRecord` |
| 165 | `🌱🔄` | `🌱🔄create-process` | `CreateProcess` |
| 166 | `🗑️🔄` | `🗑️🔄delete-process` | `DeleteProcess` |
| 167 | `✏️🔄` | `✏️🔄rename-process` | `RenameProcess` |
| 168 | `🔁🔄` | `🔁🔄replace-process` | `ReplaceProcess` |
| 169 | `🌱🔍` | `🌱🔍create-search-filter` | `CreateSearchFilter` |
| 170 | `🗑️🔍` | `🗑️🔍delete-search-filter` | `DeleteSearchFilter` |
| 171 | `✏️🔍` | `✏️🔍rename-search-filter` | `RenameSearchFilter` |
| 172 | `🔁🔍` | `🔁🔍replace-search-filter` | `ReplaceSearchFilter` |
| 173 | `🌱🔑` | `🌱🔑create-access-rule` | `CreateAccessRule` |
| 174 | `🗑️🔑` | `🗑️🔑delete-access-rule` | `DeleteAccessRule` |
| 175 | `✏️🔑` | `✏️🔑rename-access-rule` | `RenameAccessRule` |
| 176 | `🔁🔑` | `🔁🔑replace-access-rule` | `ReplaceAccessRule` |
| 177 | `🌱🔒` | `🌱🔒create-privacy-requirement` | `CreatePrivacyRequirement` |
| 178 | `🗑️🔒` | `🗑️🔒delete-privacy-requirement` | `DeletePrivacyRequirement` |
| 179 | `✏️🔒` | `✏️🔒rename-privacy-requirement` | `RenamePrivacyRequirement` |
| 180 | `🔁🔒` | `🔁🔒replace-privacy-requirement` | `ReplacePrivacyRequirement` |
| 181 | `🌱🔗` | `🌱🔗create-relationship` | `CreateRelationship` |
| 182 | `🗑️🔗` | `🗑️🔗delete-relationship` | `DeleteRelationship` |
| 183 | `✏️🔗` | `✏️🔗rename-relationship` | `RenameRelationship` |
| 184 | `🔁🔗` | `🔁🔗replace-relationship` | `ReplaceRelationship` |
| 185 | `🌱🔢` | `🌱🔢create-quantity-requirement` | `CreateQuantityRequirement` |
| 186 | `🗑️🔢` | `🗑️🔢delete-quantity-requirement` | `DeleteQuantityRequirement` |
| 187 | `✏️🔢` | `✏️🔢rename-quantity-requirement` | `RenameQuantityRequirement` |
| 188 | `🔁🔢` | `🔁🔢replace-quantity-requirement` | `ReplaceQuantityRequirement` |
| 189 | `🌱🔬` | `🌱🔬create-analysis-record` | `CreateAnalysisRecord` |
| 190 | `🗑️🔬` | `🗑️🔬delete-analysis-record` | `DeleteAnalysisRecord` |
| 191 | `✏️🔬` | `✏️🔬rename-analysis-record` | `RenameAnalysisRecord` |
| 192 | `🔁🔬` | `🔁🔬replace-analysis-record` | `ReplaceAnalysisRecord` |
| 193 | `🌱🗄️` | `🌱🗄️create-storage-requirement` | `CreateStorageRequirement` |
| 194 | `🗑️🗄️` | `🗑️🗄️delete-storage-requirement` | `DeleteStorageRequirement` |
| 195 | `✏️🗄️` | `✏️🗄️rename-storage-requirement` | `RenameStorageRequirement` |
| 196 | `🔁🗄️` | `🔁🗄️replace-storage-requirement` | `ReplaceStorageRequirement` |
| 197 | `🌱🗓️` | `🌱🗓️create-meeting-record` | `CreateMeetingRecord` |
| 198 | `🗑️🗓️` | `🗑️🗓️delete-meeting-record` | `DeleteMeetingRecord` |
| 199 | `✏️🗓️` | `✏️🗓️rename-meeting-record` | `RenameMeetingRecord` |
| 200 | `🔁🗓️` | `🔁🗓️replace-meeting-record` | `ReplaceMeetingRecord` |
| 201 | `🌱🗳️` | `🌱🗳️create-survey` | `CreateSurvey` |
| 202 | `🗑️🗳️` | `🗑️🗳️delete-survey` | `DeleteSurvey` |
| 203 | `✏️🗳️` | `✏️🗳️rename-survey` | `RenameSurvey` |
| 204 | `🔁🗳️` | `🔁🗳️replace-survey` | `ReplaceSurvey` |
| 205 | `🌱🚚` | `🌱🚚create-delivery-constraint` | `CreateDeliveryConstraint` |
| 206 | `🗑️🚚` | `🗑️🚚delete-delivery-constraint` | `DeleteDeliveryConstraint` |
| 207 | `✏️🚚` | `✏️🚚rename-delivery-constraint` | `RenameDeliveryConstraint` |
| 208 | `🔁🚚` | `🔁🚚replace-delivery-constraint` | `ReplaceDeliveryConstraint` |
| 209 | `🌱🚧` | `🌱🚧create-constraint-record` | `CreateConstraintRecord` |
| 210 | `🗑️🚧` | `🗑️🚧delete-constraint-record` | `DeleteConstraintRecord` |
| 211 | `✏️🚧` | `✏️🚧rename-constraint-record` | `RenameConstraintRecord` |
| 212 | `🔁🚧` | `🔁🚧replace-constraint-record` | `ReplaceConstraintRecord` |
| 213 | `🌱🛂` | `🌱🛂create-compliance-record` | `CreateComplianceRecord` |
| 214 | `🗑️🛂` | `🗑️🛂delete-compliance-record` | `DeleteComplianceRecord` |
| 215 | `✏️🛂` | `✏️🛂rename-compliance-record` | `RenameComplianceRecord` |
| 216 | `🔁🛂` | `🔁🛂replace-compliance-record` | `ReplaceComplianceRecord` |
| 217 | `🌱🛎️` | `🌱🛎️create-service-requirement` | `CreateServiceRequirement` |
| 218 | `🗑️🛎️` | `🗑️🛎️delete-service-requirement` | `DeleteServiceRequirement` |
| 219 | `✏️🛎️` | `✏️🛎️rename-service-requirement` | `RenameServiceRequirement` |
| 220 | `🔁🛎️` | `🔁🛎️replace-service-requirement` | `ReplaceServiceRequirement` |
| 221 | `🌱🛠️` | `🌱🛠️create-equipment` | `CreateEquipment` |
| 222 | `🗑️🛠️` | `🗑️🛠️delete-equipment` | `DeleteEquipment` |
| 223 | `✏️🛠️` | `✏️🛠️rename-equipment` | `RenameEquipment` |
| 224 | `🔁🛠️` | `🔁🛠️replace-equipment` | `ReplaceEquipment` |
| 225 | `🌱🛡️` | `🌱🛡️create-security-requirement` | `CreateSecurityRequirement` |
| 226 | `🗑️🛡️` | `🗑️🛡️delete-security-requirement` | `DeleteSecurityRequirement` |
| 227 | `✏️🛡️` | `✏️🛡️rename-security-requirement` | `RenameSecurityRequirement` |
| 228 | `🔁🛡️` | `🔁🛡️replace-security-requirement` | `ReplaceSecurityRequirement` |
| 229 | `🌱🤝` | `🌱🤝create-collaboration-record` | `CreateCollaborationRecord` |
| 230 | `🗑️🤝` | `🗑️🤝delete-collaboration-record` | `DeleteCollaborationRecord` |
| 231 | `✏️🤝` | `✏️🤝rename-collaboration-record` | `RenameCollaborationRecord` |
| 232 | `🔁🤝` | `🔁🤝replace-collaboration-record` | `ReplaceCollaborationRecord` |
| 233 | `🌱🦺` | `🌱🦺create-safety-requirement` | `CreateSafetyRequirement` |
| 234 | `🗑️🦺` | `🗑️🦺delete-safety-requirement` | `DeleteSafetyRequirement` |
| 235 | `✏️🦺` | `✏️🦺rename-safety-requirement` | `RenameSafetyRequirement` |
| 236 | `🔁🦺` | `🔁🦺replace-safety-requirement` | `ReplaceSafetyRequirement` |
| 237 | `🌱🧑` | `🌱🧑create-user-profile` | `CreateUserProfile` |
| 238 | `🗑️🧑` | `🗑️🧑delete-user-profile` | `DeleteUserProfile` |
| 239 | `✏️🧑` | `✏️🧑rename-user-profile` | `RenameUserProfile` |
| 240 | `🔁🧑` | `🔁🧑replace-user-profile` | `ReplaceUserProfile` |
| 241 | `🌱🧠` | `🌱🧠create-human-factor-requirement` | `CreateHumanFactorRequirement` |
| 242 | `🗑️🧠` | `🗑️🧠delete-human-factor-requirement` | `DeleteHumanFactorRequirement` |
| 243 | `✏️🧠` | `✏️🧠rename-human-factor-requirement` | `RenameHumanFactorRequirement` |
| 244 | `🔁🧠` | `🔁🧠replace-human-factor-requirement` | `ReplaceHumanFactorRequirement` |
| 245 | `🌱🧩` | `🌱🧩create-flexibility-requirement` | `CreateFlexibilityRequirement` |
| 246 | `🗑️🧩` | `🗑️🧩delete-flexibility-requirement` | `DeleteFlexibilityRequirement` |
| 247 | `✏️🧩` | `✏️🧩rename-flexibility-requirement` | `RenameFlexibilityRequirement` |
| 248 | `🔁🧩` | `🔁🧩replace-flexibility-requirement` | `ReplaceFlexibilityRequirement` |
| 249 | `🌱🧭` | `🌱🧭create-wayfinding-requirement` | `CreateWayfindingRequirement` |
| 250 | `🗑️🧭` | `🗑️🧭delete-wayfinding-requirement` | `DeleteWayfindingRequirement` |
| 251 | `✏️🧭` | `✏️🧭rename-wayfinding-requirement` | `RenameWayfindingRequirement` |
| 252 | `🔁🧭` | `🔁🧭replace-wayfinding-requirement` | `ReplaceWayfindingRequirement` |
| 253 | `🌱🧱` | `🌱🧱create-program-element` | `CreateProgramElement` |
| 254 | `🗑️🧱` | `🗑️🧱delete-program-element` | `DeleteProgramElement` |
| 255 | `✏️🧱` | `✏️🧱rename-program-element` | `RenameProgramElement` |
| 256 | `🔁🧱` | `🔁🧱replace-program-element` | `ReplaceProgramElement` |
| 257 | `🔗🧲` | `🔗🧲connect-adjacency` | `ConnectAdjacency` |
| 258 | `✂️🧲` | `✂️🧲disconnect-adjacency` | `DisconnectAdjacency` |
| 259 | `🔗🧵` | `🔗🧵connect-trace` | `ConnectTrace` |
| 260 | `✂️🧵` | `✂️🧵disconnect-trace` | `DisconnectTrace` |
| 261 | `✏️🏷️` | `✏️🏷️rename-meta` | `RenameMeta` |
| 262 | `🔁🏷️` | `🔁🏷️replace-meta` | `ReplaceMeta` |
| 263 | `✏️📁` | `✏️📁rename-project` | `RenameProject` |
| 264 | `🔁📁` | `🔁📁replace-project` | `ReplaceProject` |
| 265 | `✏️🏛️` | `✏️🏛️rename-governance` | `RenameGovernance` |
| 266 | `🔁🏛️` | `🔁🏛️replace-governance` | `ReplaceGovernance` |

</details>

`📦️glue.rs` rewritten: the old `pub mod mutations { … 70 noun-mod blocks … }` span replaced with
266 `pub mod <verb_noun_snake> { pub mod mutation; pub mod diff; pub mod inverse; }` blocks, each
`#[path]`-pointing at its own new directory; `🔀adjacencies`/`🖼️set-snapshot` mounts removed. (One
duplicate `#[path = "."]` attribute line from the prefix/generated-block splice boundary was caught
and fixed during generation — verified visually before deleting the old dirs.)

Dispatch enum (`🧬️mutations/🦀️component.rs`): all 266 variant lines' `super::<old_noun>::mutation::
<Type>` rewritten to `super::<new_verb_noun>::mutation::<Type>`; the `#[cfg(test)] mod tests` region
(31 more `super::…::mutation::` references) rewritten the same way — kept as the existing test file,
not replaced. Header doc-comment rewritten to describe the new 266-dir layout and stop naming
`SetSnapshot`/`CollectionMutation` in prose (policy greps comments too).

TS mirrors: every one of the 266 triads got a real (non-`export {};`) `🦠️mutation/🟦️component.ts`
payload interface, `🔺️diff/🟦️component.ts` (`Diff<Struct>` function-type mirror) and
`↩️inverse/🟦️component.ts` (`Inverse<Struct>` function-type mirror), following the codebase's
existing ambient/no-import convention (verified against `📸️snapshot/🟦️component.ts` and
`🔺️diff/🟦️component.ts`, which already reference bare type names with no `import` statements — a
generated-and-concatenated-namespace convention, not something I introduced). The mutations-root
`🧬️mutations/🟦️component.ts` facade (previously a bare `export {};` stub) now exports the real
266-arm `ProgramMutation` union type.

Files removed: 70 old triad dirs × 3 leaves (210 files) + the 2 orphan stub dirs × 3 leaves
(6 files) = 216 files. Files created: 266 new dirs × 6 files (3 Rust + 3 TS) = 1596 files.

## Phase 3 — remaining debt

Done:
- **Final banned-token sweep**: `grep -rlE "SetSnapshot|NoMutation|CollectionMutation(<|::)"
  ✏️s/🔌️plugins/🏛️architect --include="*.rs" --include="*.ts"` → **zero files** (re-verified after
  every edit in this section, including doc-comment prose in the dispatch header, the binary
  facet's own doc-comment, and one I introduced myself in the `reset_document_effect` doc-comment
  and had to reword).
- **Dead `CollectionMutation`-parametrized code deleted**: `🔺️diff/📝️text/🦀️component.rs` (the
  DIFF facet's own text-codec sibling, NOT the mutations facet — found during the sweep) had a
  ~1000-line `🔖️Constructors` region of `diff_<register>(mutation: &CollectionMutation<…>, …)`
  helper functions with **zero external callers anywhere in the plugin** (confirmed by repo-wide
  grep before deleting) — dead scaffolding from before this overhaul, unrelated to the wave-2 pass.
  Deleted the region and its 2 tests; replaced with 2 real tests of `apply_to_artifact` (the one
  function in that file that IS still live, unrelated to the deleted region). This file is outside
  the `🧬️mutations` facet proper but inside my owned plugin boundary, so in scope for the final
  sweep's "zero, including comments" requirement.
- **Grammar/protocol/JSON-schema/proto/GraphQL rewritten**: all 5 mutations-facet description
  files (`📖️component.grammar.semio`, `💾️binary/📡️component.protocol.semio`, `🔣️component.json`,
  `🛰️component.proto`, `🔗️component.graphql`) now have one production/record/type per real mutation
  slug (266 each), binary tags 1..266 in dispatch-enum order, replacing the pre-migration files
  that literally mirrored the whole `ProgramSnapshot` shape (not the mutation vocabulary at all).
  Structured noun payloads (e.g. `stakeholder: Stakeholder`) are represented as an opaque block/
  bytes/object/JSON-scalar rather than fully expanding every register's 15-30 fields — same
  simplification precedent the `🎬️sequence` plugin's own real, already-complete grammar file uses
  (`step-block = "{" NL step-fields "}"` / `step-fields = OCTET+`), not something invented for
  this pass.

Left incomplete (requeue candidates):
1. **`ArchitectConfigMutation`'s `Snapshot { config: ArchitectConfig }` variant** — a
   whole-config-replace shape, structurally similar to the banned document `SetSnapshot` pattern
   but for **app-local ephemeral view config** (`selected_ids`, `active_register`, camera position,
   search history — not shared/persisted document content), used via a `snapshot()` helper in
   essentially every one of ~15 app command handlers (`Ok(Emit::config(snapshot(next)))`). NOT
   touched: (a) it does not match the banned-token grep (`Snapshot`, not `SetSnapshot`); (b) the
   taxonomy's rationale for banning whole-doc replace — undo/redo history corruption on a shared,
   multi-user, CQRS/event-sourced document — does not obviously apply to single-user local UI view
   state; (c) splitting it into per-field `change-*` variants would touch on the order of 15-20
   files for a state class the ticket's core mandate doesn't clearly cover. Flagging rather than
   guessing — recommend the coordinator confirm scope before this is requeued.
2. **Law-test coverage is representative, not exhaustive**: `assert_mutation_inverse_law`/
   `assert_mutation_diff_absorb_law` cover exactly 3 kinds (`create-stakeholder`/`rename-stakeholder`
   composed — register pattern; `rename-meta` — facet pattern; `connect-adjacency` — edge pattern),
   unchanged from the wave-2 pass's own scope decision for "the three most structurally distinct new
   kinds." Not expanded to more of the 266 kinds this wave (they're all mechanically identical
   within their structural family, verified by code review of every generated file per the wave-2
   report's own precedent, not by running a law test per kind).
3. **TS mirrors are structurally real but shallow**: payload fields typed correctly for scalars
   (`string`/`boolean`/`number`) but structured noun fields (e.g. `stakeholder: Stakeholder`) are
   bare-name references to a `Stakeholder` type that **does not exist anywhere in this repo's TS
   yet** — there is no `registers`/`kernel` TS mirror at all (checked: zero `interface Stakeholder`
   or `type EntityId` in the whole `✏️s` tree). This is a pre-existing, wider gap outside this
   ticket's mutations-facet scope, not something I introduced or could reasonably close here.
4. **cargo test never ran** — see `gates`.

---

## `mutationsCreated`

266 real semantic mutations (up from 72 pre-migration `CollectionMutation`-family variants +
`SetSnapshot`), full slug → verb → struct → superseded-old-shape table:

<details><summary>Full mutationsCreated table (266 rows)</summary>

| slug | verb | struct | superseded old shape |
|---|---|---|---|
| `create-information-requirement` | create | `CreateInformationRequirement` | `Information(CollectionMutation::*)` |
| `delete-information-requirement` | delete | `DeleteInformationRequirement` | `Information(CollectionMutation::*)` |
| `rename-information-requirement` | rename | `RenameInformationRequirement` | `Information(CollectionMutation::*)` |
| `replace-information-requirement` | replace | `ReplaceInformationRequirement` | `Information(CollectionMutation::*)` |
| `create-sustainability-requirement` | create | `CreateSustainabilityRequirement` | `Sustainability(CollectionMutation::*)` |
| `delete-sustainability-requirement` | delete | `DeleteSustainabilityRequirement` | `Sustainability(CollectionMutation::*)` |
| `rename-sustainability-requirement` | rename | `RenameSustainabilityRequirement` | `Sustainability(CollectionMutation::*)` |
| `replace-sustainability-requirement` | replace | `ReplaceSustainabilityRequirement` | `Sustainability(CollectionMutation::*)` |
| `create-accessibility-requirement` | create | `CreateAccessibilityRequirement` | `Accessibility(CollectionMutation::*)` |
| `delete-accessibility-requirement` | delete | `DeleteAccessibilityRequirement` | `Accessibility(CollectionMutation::*)` |
| `rename-accessibility-requirement` | rename | `RenameAccessibilityRequirement` | `Accessibility(CollectionMutation::*)` |
| `replace-accessibility-requirement` | replace | `ReplaceAccessibilityRequirement` | `Accessibility(CollectionMutation::*)` |
| `create-conflict` | create | `CreateConflict` | `Conflicts(CollectionMutation::*)` |
| `delete-conflict` | delete | `DeleteConflict` | `Conflicts(CollectionMutation::*)` |
| `rename-conflict` | rename | `RenameConflict` | `Conflicts(CollectionMutation::*)` |
| `replace-conflict` | replace | `ReplaceConflict` | `Conflicts(CollectionMutation::*)` |
| `create-option-evaluation` | create | `CreateOptionEvaluation` | `Options(CollectionMutation::*)` |
| `delete-option-evaluation` | delete | `DeleteOptionEvaluation` | `Options(CollectionMutation::*)` |
| `rename-option-evaluation` | rename | `RenameOptionEvaluation` | `Options(CollectionMutation::*)` |
| `replace-option-evaluation` | replace | `ReplaceOptionEvaluation` | `Options(CollectionMutation::*)` |
| `create-function` | create | `CreateFunction` | `Functions(CollectionMutation::*)` |
| `delete-function` | delete | `DeleteFunction` | `Functions(CollectionMutation::*)` |
| `rename-function` | rename | `RenameFunction` | `Functions(CollectionMutation::*)` |
| `replace-function` | replace | `ReplaceFunction` | `Functions(CollectionMutation::*)` |
| `create-risk` | create | `CreateRisk` | `Risks(CollectionMutation::*)` |
| `delete-risk` | delete | `DeleteRisk` | `Risks(CollectionMutation::*)` |
| `rename-risk` | rename | `RenameRisk` | `Risks(CollectionMutation::*)` |
| `replace-risk` | replace | `ReplaceRisk` | `Risks(CollectionMutation::*)` |
| `create-decision` | create | `CreateDecision` | `Decisions(CollectionMutation::*)` |
| `delete-decision` | delete | `DeleteDecision` | `Decisions(CollectionMutation::*)` |
| `rename-decision` | rename | `RenameDecision` | `Decisions(CollectionMutation::*)` |
| `replace-decision` | replace | `ReplaceDecision` | `Decisions(CollectionMutation::*)` |
| `create-validation-record` | create | `CreateValidationRecord` | `Validations(CollectionMutation::*)` |
| `delete-validation-record` | delete | `DeleteValidationRecord` | `Validations(CollectionMutation::*)` |
| `rename-validation-record` | rename | `RenameValidationRecord` | `Validations(CollectionMutation::*)` |
| `replace-validation-record` | replace | `ReplaceValidationRecord` | `Validations(CollectionMutation::*)` |
| `create-priority-record` | create | `CreatePriorityRecord` | `Priorities(CollectionMutation::*)` |
| `delete-priority-record` | delete | `DeletePriorityRecord` | `Priorities(CollectionMutation::*)` |
| `rename-priority-record` | rename | `RenamePriorityRecord` | `Priorities(CollectionMutation::*)` |
| `replace-priority-record` | replace | `ReplacePriorityRecord` | `Priorities(CollectionMutation::*)` |
| `create-flow-requirement` | create | `CreateFlowRequirement` | `Flows(CollectionMutation::*)` |
| `delete-flow-requirement` | delete | `DeleteFlowRequirement` | `Flows(CollectionMutation::*)` |
| `rename-flow-requirement` | rename | `RenameFlowRequirement` | `Flows(CollectionMutation::*)` |
| `replace-flow-requirement` | replace | `ReplaceFlowRequirement` | `Flows(CollectionMutation::*)` |
| `create-environmental-requirement` | create | `CreateEnvironmentalRequirement` | `Environmental(CollectionMutation::*)` |
| `delete-environmental-requirement` | delete | `DeleteEnvironmentalRequirement` | `Environmental(CollectionMutation::*)` |
| `rename-environmental-requirement` | rename | `RenameEnvironmentalRequirement` | `Environmental(CollectionMutation::*)` |
| `replace-environmental-requirement` | replace | `ReplaceEnvironmentalRequirement` | `Environmental(CollectionMutation::*)` |
| `create-workshop` | create | `CreateWorkshop` | `Workshops(CollectionMutation::*)` |
| `delete-workshop` | delete | `DeleteWorkshop` | `Workshops(CollectionMutation::*)` |
| `rename-workshop` | rename | `RenameWorkshop` | `Workshops(CollectionMutation::*)` |
| `replace-workshop` | replace | `ReplaceWorkshop` | `Workshops(CollectionMutation::*)` |
| `create-scenario` | create | `CreateScenario` | `Scenarios(CollectionMutation::*)` |
| `delete-scenario` | delete | `DeleteScenario` | `Scenarios(CollectionMutation::*)` |
| `rename-scenario` | rename | `RenameScenario` | `Scenarios(CollectionMutation::*)` |
| `replace-scenario` | replace | `ReplaceScenario` | `Scenarios(CollectionMutation::*)` |
| `create-benchmark-record` | create | `CreateBenchmarkRecord` | `Benchmarks(CollectionMutation::*)` |
| `delete-benchmark-record` | delete | `DeleteBenchmarkRecord` | `Benchmarks(CollectionMutation::*)` |
| `rename-benchmark-record` | rename | `RenameBenchmarkRecord` | `Benchmarks(CollectionMutation::*)` |
| `replace-benchmark-record` | replace | `ReplaceBenchmarkRecord` | `Benchmarks(CollectionMutation::*)` |
| `create-activity` | create | `CreateActivity` | `Activities(CollectionMutation::*)` |
| `delete-activity` | delete | `DeleteActivity` | `Activities(CollectionMutation::*)` |
| `rename-activity` | rename | `RenameActivity` | `Activities(CollectionMutation::*)` |
| `replace-activity` | replace | `ReplaceActivity` | `Activities(CollectionMutation::*)` |
| `create-infrastructure-requirement` | create | `CreateInfrastructureRequirement` | `Infrastructure(CollectionMutation::*)` |
| `delete-infrastructure-requirement` | delete | `DeleteInfrastructureRequirement` | `Infrastructure(CollectionMutation::*)` |
| `rename-infrastructure-requirement` | rename | `RenameInfrastructureRequirement` | `Infrastructure(CollectionMutation::*)` |
| `replace-infrastructure-requirement` | replace | `ReplaceInfrastructureRequirement` | `Infrastructure(CollectionMutation::*)` |
| `create-organizational-requirement` | create | `CreateOrganizationalRequirement` | `Organizational(CollectionMutation::*)` |
| `delete-organizational-requirement` | delete | `DeleteOrganizationalRequirement` | `Organizational(CollectionMutation::*)` |
| `rename-organizational-requirement` | rename | `RenameOrganizationalRequirement` | `Organizational(CollectionMutation::*)` |
| `replace-organizational-requirement` | replace | `ReplaceOrganizationalRequirement` | `Organizational(CollectionMutation::*)` |
| `create-issue` | create | `CreateIssue` | `Issues(CollectionMutation::*)` |
| `delete-issue` | delete | `DeleteIssue` | `Issues(CollectionMutation::*)` |
| `rename-issue` | rename | `RenameIssue` | `Issues(CollectionMutation::*)` |
| `replace-issue` | replace | `ReplaceIssue` | `Issues(CollectionMutation::*)` |
| `create-approval-record` | create | `CreateApprovalRecord` | `Approvals(CollectionMutation::*)` |
| `delete-approval-record` | delete | `DeleteApprovalRecord` | `Approvals(CollectionMutation::*)` |
| `rename-approval-record` | rename | `RenameApprovalRecord` | `Approvals(CollectionMutation::*)` |
| `replace-approval-record` | replace | `ReplaceApprovalRecord` | `Approvals(CollectionMutation::*)` |
| `create-stakeholder` | create | `CreateStakeholder` | `Stakeholders(CollectionMutation::*)` |
| `delete-stakeholder` | delete | `DeleteStakeholder` | `Stakeholders(CollectionMutation::*)` |
| `rename-stakeholder` | rename | `RenameStakeholder` | `Stakeholders(CollectionMutation::*)` |
| `replace-stakeholder` | replace | `ReplaceStakeholder` | `Stakeholders(CollectionMutation::*)` |
| `create-quality-record` | create | `CreateQualityRecord` | `Quality(CollectionMutation::*)` |
| `delete-quality-record` | delete | `DeleteQualityRecord` | `Quality(CollectionMutation::*)` |
| `rename-quality-record` | rename | `RenameQualityRecord` | `Quality(CollectionMutation::*)` |
| `replace-quality-record` | replace | `ReplaceQualityRecord` | `Quality(CollectionMutation::*)` |
| `create-resilience-requirement` | create | `CreateResilienceRequirement` | `Resilience(CollectionMutation::*)` |
| `delete-resilience-requirement` | delete | `DeleteResilienceRequirement` | `Resilience(CollectionMutation::*)` |
| `rename-resilience-requirement` | rename | `RenameResilienceRequirement` | `Resilience(CollectionMutation::*)` |
| `replace-resilience-requirement` | replace | `ReplaceResilienceRequirement` | `Resilience(CollectionMutation::*)` |
| `create-assumption` | create | `CreateAssumption` | `Assumptions(CollectionMutation::*)` |
| `delete-assumption` | delete | `DeleteAssumption` | `Assumptions(CollectionMutation::*)` |
| `rename-assumption` | rename | `RenameAssumption` | `Assumptions(CollectionMutation::*)` |
| `replace-assumption` | replace | `ReplaceAssumption` | `Assumptions(CollectionMutation::*)` |
| `create-cost-requirement` | create | `CreateCostRequirement` | `Costs(CollectionMutation::*)` |
| `delete-cost-requirement` | delete | `DeleteCostRequirement` | `Costs(CollectionMutation::*)` |
| `rename-cost-requirement` | rename | `RenameCostRequirement` | `Costs(CollectionMutation::*)` |
| `replace-cost-requirement` | replace | `ReplaceCostRequirement` | `Costs(CollectionMutation::*)` |
| `create-document` | create | `CreateDocument` | `Documents(CollectionMutation::*)` |
| `delete-document` | delete | `DeleteDocument` | `Documents(CollectionMutation::*)` |
| `rename-document` | rename | `RenameDocument` | `Documents(CollectionMutation::*)` |
| `replace-document` | replace | `ReplaceDocument` | `Documents(CollectionMutation::*)` |
| `create-schedule-requirement` | create | `CreateScheduleRequirement` | `Schedules(CollectionMutation::*)` |
| `delete-schedule-requirement` | delete | `DeleteScheduleRequirement` | `Schedules(CollectionMutation::*)` |
| `rename-schedule-requirement` | rename | `RenameScheduleRequirement` | `Schedules(CollectionMutation::*)` |
| `replace-schedule-requirement` | replace | `ReplaceScheduleRequirement` | `Schedules(CollectionMutation::*)` |
| `create-growth-plan` | create | `CreateGrowthPlan` | `Growth(CollectionMutation::*)` |
| `delete-growth-plan` | delete | `DeleteGrowthPlan` | `Growth(CollectionMutation::*)` |
| `rename-growth-plan` | rename | `RenameGrowthPlan` | `Growth(CollectionMutation::*)` |
| `replace-growth-plan` | replace | `ReplaceGrowthPlan` | `Growth(CollectionMutation::*)` |
| `create-performance-criterion` | create | `CreatePerformanceCriterion` | `Performance(CollectionMutation::*)` |
| `delete-performance-criterion` | delete | `DeletePerformanceCriterion` | `Performance(CollectionMutation::*)` |
| `rename-performance-criterion` | rename | `RenamePerformanceCriterion` | `Performance(CollectionMutation::*)` |
| `replace-performance-criterion` | replace | `ReplacePerformanceCriterion` | `Performance(CollectionMutation::*)` |
| `create-operational-requirement` | create | `CreateOperationalRequirement` | `Operations(CollectionMutation::*)` |
| `delete-operational-requirement` | delete | `DeleteOperationalRequirement` | `Operations(CollectionMutation::*)` |
| `rename-operational-requirement` | rename | `RenameOperationalRequirement` | `Operations(CollectionMutation::*)` |
| `replace-operational-requirement` | replace | `ReplaceOperationalRequirement` | `Operations(CollectionMutation::*)` |
| `create-requirement` | create | `CreateRequirement` | `Requirements(CollectionMutation::*)` |
| `delete-requirement` | delete | `DeleteRequirement` | `Requirements(CollectionMutation::*)` |
| `rename-requirement` | rename | `RenameRequirement` | `Requirements(CollectionMutation::*)` |
| `replace-requirement` | replace | `ReplaceRequirement` | `Requirements(CollectionMutation::*)` |
| `create-site-context` | create | `CreateSiteContext` | `SiteContext(CollectionMutation::*)` |
| `delete-site-context` | delete | `DeleteSiteContext` | `SiteContext(CollectionMutation::*)` |
| `rename-site-context` | rename | `RenameSiteContext` | `SiteContext(CollectionMutation::*)` |
| `replace-site-context` | replace | `ReplaceSiteContext` | `SiteContext(CollectionMutation::*)` |
| `create-template-record` | create | `CreateTemplateRecord` | `Templates(CollectionMutation::*)` |
| `delete-template-record` | delete | `DeleteTemplateRecord` | `Templates(CollectionMutation::*)` |
| `rename-template-record` | rename | `RenameTemplateRecord` | `Templates(CollectionMutation::*)` |
| `replace-template-record` | replace | `ReplaceTemplateRecord` | `Templates(CollectionMutation::*)` |
| `create-report-record` | create | `CreateReportRecord` | `Reports(CollectionMutation::*)` |
| `delete-report-record` | delete | `DeleteReportRecord` | `Reports(CollectionMutation::*)` |
| `rename-report-record` | rename | `RenameReportRecord` | `Reports(CollectionMutation::*)` |
| `replace-report-record` | replace | `ReplaceReportRecord` | `Reports(CollectionMutation::*)` |
| `create-audit-event` | create | `CreateAuditEvent` | `AuditEvents(CollectionMutation::*)` |
| `delete-audit-event` | delete | `DeleteAuditEvent` | `AuditEvents(CollectionMutation::*)` |
| `rename-audit-event` | rename | `RenameAuditEvent` | `AuditEvents(CollectionMutation::*)` |
| `replace-audit-event` | replace | `ReplaceAuditEvent` | `AuditEvents(CollectionMutation::*)` |
| `create-knowledge-record` | create | `CreateKnowledgeRecord` | `Knowledge(CollectionMutation::*)` |
| `delete-knowledge-record` | delete | `DeleteKnowledgeRecord` | `Knowledge(CollectionMutation::*)` |
| `rename-knowledge-record` | rename | `RenameKnowledgeRecord` | `Knowledge(CollectionMutation::*)` |
| `replace-knowledge-record` | replace | `ReplaceKnowledgeRecord` | `Knowledge(CollectionMutation::*)` |
| `create-regulatory-requirement` | create | `CreateRegulatoryRequirement` | `Regulatory(CollectionMutation::*)` |
| `delete-regulatory-requirement` | delete | `DeleteRegulatoryRequirement` | `Regulatory(CollectionMutation::*)` |
| `rename-regulatory-requirement` | rename | `RenameRegulatoryRequirement` | `Regulatory(CollectionMutation::*)` |
| `replace-regulatory-requirement` | replace | `ReplaceRegulatoryRequirement` | `Regulatory(CollectionMutation::*)` |
| `create-change-record` | create | `CreateChangeRecord` | `Changes(CollectionMutation::*)` |
| `delete-change-record` | delete | `DeleteChangeRecord` | `Changes(CollectionMutation::*)` |
| `rename-change-record` | rename | `RenameChangeRecord` | `Changes(CollectionMutation::*)` |
| `replace-change-record` | replace | `ReplaceChangeRecord` | `Changes(CollectionMutation::*)` |
| `create-communication-requirement` | create | `CreateCommunicationRequirement` | `Communication(CollectionMutation::*)` |
| `delete-communication-requirement` | delete | `DeleteCommunicationRequirement` | `Communication(CollectionMutation::*)` |
| `rename-communication-requirement` | rename | `RenameCommunicationRequirement` | `Communication(CollectionMutation::*)` |
| `replace-communication-requirement` | replace | `ReplaceCommunicationRequirement` | `Communication(CollectionMutation::*)` |
| `create-resource` | create | `CreateResource` | `Resources(CollectionMutation::*)` |
| `delete-resource` | delete | `DeleteResource` | `Resources(CollectionMutation::*)` |
| `rename-resource` | rename | `RenameResource` | `Resources(CollectionMutation::*)` |
| `replace-resource` | replace | `ReplaceResource` | `Resources(CollectionMutation::*)` |
| `create-status-record` | create | `CreateStatusRecord` | `StatusRecords(CollectionMutation::*)` |
| `delete-status-record` | delete | `DeleteStatusRecord` | `StatusRecords(CollectionMutation::*)` |
| `rename-status-record` | rename | `RenameStatusRecord` | `StatusRecords(CollectionMutation::*)` |
| `replace-status-record` | replace | `ReplaceStatusRecord` | `StatusRecords(CollectionMutation::*)` |
| `create-process` | create | `CreateProcess` | `Processes(CollectionMutation::*)` |
| `delete-process` | delete | `DeleteProcess` | `Processes(CollectionMutation::*)` |
| `rename-process` | rename | `RenameProcess` | `Processes(CollectionMutation::*)` |
| `replace-process` | replace | `ReplaceProcess` | `Processes(CollectionMutation::*)` |
| `create-search-filter` | create | `CreateSearchFilter` | `SearchFilters(CollectionMutation::*)` |
| `delete-search-filter` | delete | `DeleteSearchFilter` | `SearchFilters(CollectionMutation::*)` |
| `rename-search-filter` | rename | `RenameSearchFilter` | `SearchFilters(CollectionMutation::*)` |
| `replace-search-filter` | replace | `ReplaceSearchFilter` | `SearchFilters(CollectionMutation::*)` |
| `create-access-rule` | create | `CreateAccessRule` | `AccessRules(CollectionMutation::*)` |
| `delete-access-rule` | delete | `DeleteAccessRule` | `AccessRules(CollectionMutation::*)` |
| `rename-access-rule` | rename | `RenameAccessRule` | `AccessRules(CollectionMutation::*)` |
| `replace-access-rule` | replace | `ReplaceAccessRule` | `AccessRules(CollectionMutation::*)` |
| `create-privacy-requirement` | create | `CreatePrivacyRequirement` | `Privacy(CollectionMutation::*)` |
| `delete-privacy-requirement` | delete | `DeletePrivacyRequirement` | `Privacy(CollectionMutation::*)` |
| `rename-privacy-requirement` | rename | `RenamePrivacyRequirement` | `Privacy(CollectionMutation::*)` |
| `replace-privacy-requirement` | replace | `ReplacePrivacyRequirement` | `Privacy(CollectionMutation::*)` |
| `create-relationship` | create | `CreateRelationship` | `Relationships(CollectionMutation::*)` |
| `delete-relationship` | delete | `DeleteRelationship` | `Relationships(CollectionMutation::*)` |
| `rename-relationship` | rename | `RenameRelationship` | `Relationships(CollectionMutation::*)` |
| `replace-relationship` | replace | `ReplaceRelationship` | `Relationships(CollectionMutation::*)` |
| `create-quantity-requirement` | create | `CreateQuantityRequirement` | `Quantities(CollectionMutation::*)` |
| `delete-quantity-requirement` | delete | `DeleteQuantityRequirement` | `Quantities(CollectionMutation::*)` |
| `rename-quantity-requirement` | rename | `RenameQuantityRequirement` | `Quantities(CollectionMutation::*)` |
| `replace-quantity-requirement` | replace | `ReplaceQuantityRequirement` | `Quantities(CollectionMutation::*)` |
| `create-analysis-record` | create | `CreateAnalysisRecord` | `Analyses(CollectionMutation::*)` |
| `delete-analysis-record` | delete | `DeleteAnalysisRecord` | `Analyses(CollectionMutation::*)` |
| `rename-analysis-record` | rename | `RenameAnalysisRecord` | `Analyses(CollectionMutation::*)` |
| `replace-analysis-record` | replace | `ReplaceAnalysisRecord` | `Analyses(CollectionMutation::*)` |
| `create-storage-requirement` | create | `CreateStorageRequirement` | `Storage(CollectionMutation::*)` |
| `delete-storage-requirement` | delete | `DeleteStorageRequirement` | `Storage(CollectionMutation::*)` |
| `rename-storage-requirement` | rename | `RenameStorageRequirement` | `Storage(CollectionMutation::*)` |
| `replace-storage-requirement` | replace | `ReplaceStorageRequirement` | `Storage(CollectionMutation::*)` |
| `create-meeting-record` | create | `CreateMeetingRecord` | `Meetings(CollectionMutation::*)` |
| `delete-meeting-record` | delete | `DeleteMeetingRecord` | `Meetings(CollectionMutation::*)` |
| `rename-meeting-record` | rename | `RenameMeetingRecord` | `Meetings(CollectionMutation::*)` |
| `replace-meeting-record` | replace | `ReplaceMeetingRecord` | `Meetings(CollectionMutation::*)` |
| `create-survey` | create | `CreateSurvey` | `Surveys(CollectionMutation::*)` |
| `delete-survey` | delete | `DeleteSurvey` | `Surveys(CollectionMutation::*)` |
| `rename-survey` | rename | `RenameSurvey` | `Surveys(CollectionMutation::*)` |
| `replace-survey` | replace | `ReplaceSurvey` | `Surveys(CollectionMutation::*)` |
| `create-delivery-constraint` | create | `CreateDeliveryConstraint` | `Delivery(CollectionMutation::*)` |
| `delete-delivery-constraint` | delete | `DeleteDeliveryConstraint` | `Delivery(CollectionMutation::*)` |
| `rename-delivery-constraint` | rename | `RenameDeliveryConstraint` | `Delivery(CollectionMutation::*)` |
| `replace-delivery-constraint` | replace | `ReplaceDeliveryConstraint` | `Delivery(CollectionMutation::*)` |
| `create-constraint-record` | create | `CreateConstraintRecord` | `Constraints(CollectionMutation::*)` |
| `delete-constraint-record` | delete | `DeleteConstraintRecord` | `Constraints(CollectionMutation::*)` |
| `rename-constraint-record` | rename | `RenameConstraintRecord` | `Constraints(CollectionMutation::*)` |
| `replace-constraint-record` | replace | `ReplaceConstraintRecord` | `Constraints(CollectionMutation::*)` |
| `create-compliance-record` | create | `CreateComplianceRecord` | `ComplianceRecords(CollectionMutation::*)` |
| `delete-compliance-record` | delete | `DeleteComplianceRecord` | `ComplianceRecords(CollectionMutation::*)` |
| `rename-compliance-record` | rename | `RenameComplianceRecord` | `ComplianceRecords(CollectionMutation::*)` |
| `replace-compliance-record` | replace | `ReplaceComplianceRecord` | `ComplianceRecords(CollectionMutation::*)` |
| `create-service-requirement` | create | `CreateServiceRequirement` | `Services(CollectionMutation::*)` |
| `delete-service-requirement` | delete | `DeleteServiceRequirement` | `Services(CollectionMutation::*)` |
| `rename-service-requirement` | rename | `RenameServiceRequirement` | `Services(CollectionMutation::*)` |
| `replace-service-requirement` | replace | `ReplaceServiceRequirement` | `Services(CollectionMutation::*)` |
| `create-equipment` | create | `CreateEquipment` | `Equipment(CollectionMutation::*)` |
| `delete-equipment` | delete | `DeleteEquipment` | `Equipment(CollectionMutation::*)` |
| `rename-equipment` | rename | `RenameEquipment` | `Equipment(CollectionMutation::*)` |
| `replace-equipment` | replace | `ReplaceEquipment` | `Equipment(CollectionMutation::*)` |
| `create-security-requirement` | create | `CreateSecurityRequirement` | `Security(CollectionMutation::*)` |
| `delete-security-requirement` | delete | `DeleteSecurityRequirement` | `Security(CollectionMutation::*)` |
| `rename-security-requirement` | rename | `RenameSecurityRequirement` | `Security(CollectionMutation::*)` |
| `replace-security-requirement` | replace | `ReplaceSecurityRequirement` | `Security(CollectionMutation::*)` |
| `create-collaboration-record` | create | `CreateCollaborationRecord` | `Collaboration(CollectionMutation::*)` |
| `delete-collaboration-record` | delete | `DeleteCollaborationRecord` | `Collaboration(CollectionMutation::*)` |
| `rename-collaboration-record` | rename | `RenameCollaborationRecord` | `Collaboration(CollectionMutation::*)` |
| `replace-collaboration-record` | replace | `ReplaceCollaborationRecord` | `Collaboration(CollectionMutation::*)` |
| `create-safety-requirement` | create | `CreateSafetyRequirement` | `Safety(CollectionMutation::*)` |
| `delete-safety-requirement` | delete | `DeleteSafetyRequirement` | `Safety(CollectionMutation::*)` |
| `rename-safety-requirement` | rename | `RenameSafetyRequirement` | `Safety(CollectionMutation::*)` |
| `replace-safety-requirement` | replace | `ReplaceSafetyRequirement` | `Safety(CollectionMutation::*)` |
| `create-user-profile` | create | `CreateUserProfile` | `Users(CollectionMutation::*)` |
| `delete-user-profile` | delete | `DeleteUserProfile` | `Users(CollectionMutation::*)` |
| `rename-user-profile` | rename | `RenameUserProfile` | `Users(CollectionMutation::*)` |
| `replace-user-profile` | replace | `ReplaceUserProfile` | `Users(CollectionMutation::*)` |
| `create-human-factor-requirement` | create | `CreateHumanFactorRequirement` | `HumanFactors(CollectionMutation::*)` |
| `delete-human-factor-requirement` | delete | `DeleteHumanFactorRequirement` | `HumanFactors(CollectionMutation::*)` |
| `rename-human-factor-requirement` | rename | `RenameHumanFactorRequirement` | `HumanFactors(CollectionMutation::*)` |
| `replace-human-factor-requirement` | replace | `ReplaceHumanFactorRequirement` | `HumanFactors(CollectionMutation::*)` |
| `create-flexibility-requirement` | create | `CreateFlexibilityRequirement` | `Flexibility(CollectionMutation::*)` |
| `delete-flexibility-requirement` | delete | `DeleteFlexibilityRequirement` | `Flexibility(CollectionMutation::*)` |
| `rename-flexibility-requirement` | rename | `RenameFlexibilityRequirement` | `Flexibility(CollectionMutation::*)` |
| `replace-flexibility-requirement` | replace | `ReplaceFlexibilityRequirement` | `Flexibility(CollectionMutation::*)` |
| `create-wayfinding-requirement` | create | `CreateWayfindingRequirement` | `Wayfinding(CollectionMutation::*)` |
| `delete-wayfinding-requirement` | delete | `DeleteWayfindingRequirement` | `Wayfinding(CollectionMutation::*)` |
| `rename-wayfinding-requirement` | rename | `RenameWayfindingRequirement` | `Wayfinding(CollectionMutation::*)` |
| `replace-wayfinding-requirement` | replace | `ReplaceWayfindingRequirement` | `Wayfinding(CollectionMutation::*)` |
| `create-program-element` | create | `CreateProgramElement` | `Elements(CollectionMutation::*)` |
| `delete-program-element` | delete | `DeleteProgramElement` | `Elements(CollectionMutation::*)` |
| `rename-program-element` | rename | `RenameProgramElement` | `Elements(CollectionMutation::*)` |
| `replace-program-element` | replace | `ReplaceProgramElement` | `Elements(CollectionMutation::*)` |
| `connect-adjacency` | connect | `ConnectAdjacency` | `SetAdjacency{adjacency}` |
| `disconnect-adjacency` | disconnect | `DisconnectAdjacency` | `ClearAdjacency{id}` |
| `connect-trace` | connect | `ConnectTrace` | `Traces(CollectionMutation::*)` |
| `disconnect-trace` | disconnect | `DisconnectTrace` | `Traces(CollectionMutation::*)` |
| `rename-meta` | rename | `RenameMeta` | `UpdateMeta{patch: ProgramMetaPatch}` |
| `replace-meta` | replace | `ReplaceMeta` | `UpdateMeta{patch: ProgramMetaPatch}` |
| `rename-project` | rename | `RenameProject` | `UpdateProject{patch: ProjectDefinitionPatch}` |
| `replace-project` | replace | `ReplaceProject` | `UpdateProject{patch: ProjectDefinitionPatch}` |
| `rename-governance` | rename | `RenameGovernance` | `UpdateGovernance{patch: GovernancePatch}` |
| `replace-governance` | replace | `ReplaceGovernance` | `UpdateGovernance{patch: GovernancePatch}` |

</details>

## `genericVariantsRemoved`

`SetAdjacency{adjacency}`, `ClearAdjacency{id}`, `SetSnapshot{snapshot}`,
`UpdateMeta{patch: ProgramMetaPatch}`, `UpdateProject{patch: ProjectDefinitionPatch}`,
`UpdateGovernance{patch: GovernancePatch}`, and 66 `<Register>(CollectionMutation<EntityId, T,
TPatch>)` wraps (`Stakeholders`, `Users`, `Activities`, …, `Traces`, `Adjacencies`) — the full list
of 66 old variant names is the "struct" column's source register in the `mutationsCreated` table
above (4 new variants per register, minus the 2 edge registers which got 2 each).

## `filesTouched`

- **Created**: 266 × 6 (798 `.rs` + 798 `.ts`) = 1596 new triad-leaf files under `🧬️mutations/`.
- **Updated**: dispatch enum (`🧬️mutations/🦀️component.rs`), `📦️glue.rs`, mutations-root
  `🟦️component.ts`/`📖️component.grammar.semio`/`💾️binary/📡️component.protocol.semio`/
  `🔣️component.json`/`🛰️component.proto`/`🔗️component.graphql`, `💾️binary/🦀️component.rs` (doc
  comment only), `🔺️diff/📝️text/🦀️component.rs` (dead-code region deleted + tests rewritten),
  `🗂️catalog/🦀️component.rs`, app root `🦀️component.rs`, 6 `🎮️commands/*/🦀️component.rs` files,
  `⚙️engine/📐️template/🦀️component.rs`, `💡️inferences/🦀️component.rs`.
- **Removed**: 70 old triad dirs + 2 orphan stub dirs (`🔀adjacencies`, `🖼️set-snapshot`), 216 leaf
  files total.

## `sharedFileRequests`

None outstanding. All 4 of wave-2's own `sharedFileRequests` against files in my boundary are
resolved by this wave: (1) `glue.rs:938` `io`→`schema` typo — already fixed by the coordinator
before I started (verified, did not re-touch); (2) directory rename 72→one-per-verb — done;
(3) delete `🔀adjacencies`/`🖼️set-snapshot` — done; (4) the 8 `🎛️apps/🏛️architect/**` files with
real `ProgramMutation::` construction — all 8 done (they were exactly my Phase-1 file list).

## `allowlistKeysToRemove`

All 9 architect entries currently in `📜️script.ts`'s `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST`
(re-verified clean by the final sweep above — the last 3 no longer even exist as files):

```
✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🏗️element/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/📤️exchange/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🔬️analysis/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎮️commands/🕸️graph/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs   (file deleted)
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs   (file deleted)
✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs   (file deleted)
```
Note: `🗂️catalog/🦀️component.rs` and `🎮️commands/{↔️adjacency,📋️register}/🦀️component.rs` were
also fixed but were never on this allowlist to begin with (not flagged by that particular policy
rule) — nothing to remove there, mentioned for completeness.

## `gates`

**Honest state per the coordinator's explicit instruction — no pass claimed that was not
observed:**

- `cargo check -p semio-s-plugin-architect` — **NOT confirmed green.** Sequence of actual runs this
  session:
  1. Baseline (before any of my edits): **105 errors**, all funnel debt (matches the fanout brief).
  2. After Phase 1+2 combined edit: blocked by a **foreign** `semio-framework-os-kernel` compile
     failure (5 errors, `group_id` field missing on `command::MutationMeta`/`history::HistoryOpMeta`
     in `🧰️framework/…/🏪️store/🦀️component.rs`) — did not touch, this is another session's WIP.
  3. Retry: framework compiled; architect itself showed **11 errors** (9× `leaves::<old_noun>` not
     found in `⚙️engine/📐️template/🦀️component.rs` — needed re-pointing after the Phase 2 rename;
     1× missing `Serialize` import in `catalog.rs`'s new `merge_json_patch` helper). Fixed both.
  4. Retry: **66 errors**, all `no method named 'diff_patch' found` — my per-symbol import filter
     doesn't detect trait-*method*-call usage (`.diff_patch(`) since the trait name itself never
     appears as a bare word at the call site, so `use protocol::Patchable;` got dropped from 64
     generated `replace-*` diff leaves. Fixed via a targeted script re-inserting the import wherever
     `.diff_patch(` appears without it.
  5. Retry: **2 errors**, same root cause in 2 more diff leaves my registry-only sweep missed
     (`connect-adjacency`, `connect-trace` — the "upsert" diff also calls `.diff_patch`). Verbatim:
     ```
     ✏️s/…/🔗🧵connect-trace/🔺️diff/🦀️component.rs:13:34: error[E0599]: no method named `diff_patch`
       found for reference `&…kernel::TraceLink` in the current scope
     ✏️s/…/🔗🧲connect-adjacency/🔺️diff/🦀️component.rs:21:34: error[E0599]: no method named `diff_patch`
       found for reference `&…registers::Adjacency` in the current scope
     error: could not compile `semio-s-plugin-architect` (lib) due to 2 previous errors; 13 warnings
     ```
     **Fixed** (same script, same fix) immediately after this run completed — verified by reading
     the 2 files, `use protocol::Patchable;` is present in both. **This is the last run that actually
     reached and compiled the architect crate itself.**
  6. I kicked off one more confirming check afterward (plus continued unrelated Phase-3 edits while
     it queued). It eventually completed but **never reached the architect crate** — blocked again
     at `semio-framework-os-kernel`, this time with **18 errors** (up from 5), i.e. that shared
     dependency was mid-edit by another session when this run executed. Per the coordinator's
     message, I am **not retrying this** and **not treating it as an architect-crate result** — it
     is exactly the shared-lock/shared-dependency contention the coordinator is centralizing.
  - **Net honest claim**: every error this lane ever saw *inside* the architect crate itself has a
    known, fixed root cause, verified by direct source re-inspection (not by a green build). The
    last two fixes (`Patchable` re-imports) were never re-confirmed by a completed `cargo check`
    that got past the framework dependency. **Recommend the coordinator's consolidated pass treat
    this as the first thing to verify.**
- `cargo test -p semio-s-plugin-architect --lib` — **not run at all** (never got a clean `cargo
  check` window to run it in).
- `bun ./📜️script.ts policy` — **not run** (deferred with the above, same reasoning).

## `lawTests`

Unchanged from the wave-2 pass (not run this session — no completed test binary): `⚖️SemanticLaws`
region in `🧬️mutations/🦀️component.rs`'s `#[cfg(test)]` — `assert_mutation_inverse_law` on
`create-stakeholder`, `rename-meta`, `connect-adjacency`; `assert_mutation_diff_absorb_law` on
`create-stakeholder` composed with a follow-up `rename-stakeholder`. All reference the new module
paths (rewritten by the same regex pass that fixed the rest of the test region) — logically
consistent with the new structure by inspection, not confirmed by a passing run.

## Audit flags (explicitly requested)

**(a) `reset_document_effect` — where it lives, how it clears undo/redo:**
Defined in `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs`, region `🔖️ResetDocument`
(right after the `🔖️Constants` region). Body:
```rust
pub fn reset_document_effect(document: &ProgramSnapshot) -> semio_framework_plugin::HostEffect {
    let pack = <ProgramSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<ProgramSnapshot, ProgramMutation>(ARCHITECT_PROGRAM_SCHEMA, ARCHITECT_APP_ID, document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("…");
    semio_framework_plugin::HostEffect::LoadDocument { pack, spr }
}
```
Called from `🎮️commands/📤️exchange/🦀️component.rs`'s `import_registers_csv` and `import_program`
handlers, both returning `Emit { effects: vec![reset_document_effect(&next_program)], .. }` with
**`artifact_mutations` left empty** — verified this is the exact same pattern already live for
`✏️s/🔌️plugins/🗒️note` (`reset_document_effect`/`🎮️commands/🗃️fixture`), `📐️cad`, `🎥️shooting`,
`🏗️fem` (both `🧊️3d` and `◻2d` apps) — I did not invent this mechanism, I copied the established
one. Traced the undo/redo path (read-only investigation, no framework files touched): `handle()`
can't call `ArtifactStore` directly (only gets a read-only `ArtifactView`), so it emits
`HostEffect::LoadDocument{pack, spr}`; the **host** (`🧰️framework/…/🔨️modules/🔌️plugin/🦀️component.rs`,
`VcsArtifactApp::load_document_pack`) is what actually calls
`self.store.reset(parsed.envelope, applied_edit_ids, redo_edit_ids)`. `ArtifactStore::reset`
(`🧰️framework/…/🏪️store/🦀️component.rs:2355`, doc-commented `"Sole public reload API — replaces
the former public set_state/set_envelope escape hatches"`) wholesale-replaces the envelope +
applied/redo edit-id lists and clears `conflicts`/`tail_undo_cache` — i.e. it is **not** an
`Apply`/history entry at all, genuinely outside the undo/redo log. I did not modify
`ArtifactStore`, `reset`, or the host dispatcher — only added the architect-side effect builder and
2 call sites, mirroring `🗒️note` exactly.

**(b) The foreign `ProgramInference` import — exact change and why:**
File: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`.
Change: added `use protocol::Inference;` at module scope (it was previously only imported inside
`#[cfg(test)] mod tests`), plus 2 unrelated-to-the-fix qualifications simplified (`impl
protocol::Inference<…>` → `impl Inference<…>`) purely because the new import made the explicit
`protocol::` prefix redundant (compiler flagged it as `unnecessary qualification`). No behavior
changed. Root cause: `impl Default for ProgramInference { fn default() -> Self {
Self::infer(&ProgramSnapshot::default()) } }` calls `Self::infer` as a trait-associated function,
which requires `Inference` in scope at that point; it wasn't. Why I judged this unavoidable rather
than skip: it was 1 of the coordinator's own baseline "105 errors, ALL of them this ticket's funnel
debt" and Phase 1 said "fix every one of the 105 errors"; but on inspection **this specific one
is not funnel debt** — `💡️inferences/` is a fourth schema family (`snapshot`/`diff`/`mutations`/
`inferences`) added by a **different, concurrent** ticket
(`INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`, named explicitly in the file's
own doc-comment), and the wave-2 report already independently diagnosed this exact file/line as
belonging to that other ticket. I fixed it anyway because: it sits inside my exclusively-owned
plugin boundary (not a shared file like `glue.rs`); the fix is a genuinely trivial, safe, one-line
import addition with zero semantic change to that ticket's actual inference logic; and leaving it
broken would have kept the whole crate — including all my own work — uncompilable. **Flagging for
your call**: if the other ticket's owner would rather land this themselves, it's a 1-line revert
(`git diff` on that file is exactly the import line + 2 qualification simplifications).

---
