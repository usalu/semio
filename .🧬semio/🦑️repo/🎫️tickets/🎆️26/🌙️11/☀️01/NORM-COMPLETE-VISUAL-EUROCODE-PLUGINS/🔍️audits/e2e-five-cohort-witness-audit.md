# Five Non-Eurocode Browser Engineering Witnesses

Read-only source audit, 2026-11-04. Values below are independently derived from committed example text and native check formulas, not executed outputs. Three witnesses have current scalar edit rules; two expose concrete UI reachability gaps and are candidates, not ready passing browser journeys.

## Existing Direct Numeric Witnesses

| Variant / committed example | Field path and before -> after | Exact native check | Independent result |
| --- | --- | --- | --- |
| din4108 / demo | airtightnessN50: 1.5 -> 0.75 | din4108-7.n50 | Mechanical ventilation=true makes limit1.5/h; utilization n50/1.5:1 ->0.5 (100%->50%). |
| din16798 / compliant-office | zones[id=zone-office].co2Ppm:900 ->650 | din16798-1.co2.zone-office | Outdoor400; DE office absolute limit900, categoryII above-outdoor limit800; effective limit=min(900-400,800)=500. Utilization max(CO2-400,0)/500:1 ->0.5. |
| din18599 / compliant-detached | deltaUWbWM2k:0.03 ->0.15 | din18599.geg.ht-prime | Envelope area472.1m²; adjacency-weighted UA98.73W/K. H'T=98.73/472.1+delta; detached A_N140 limit0.40. Utilization0.5978235543317093 ->0.8978235543317094 (rounded row60%->90%). |

DIN18599 sumUA derivation: walls156*.2 + roof150*.14 + groundfloor140*.25*.6 + windows24*.95 + door2.1*1.3 =98.73. Envelope area156+150+140+24+2.1=472.1. The native balance-model uses these exact adjacency factors and adds delta times envelope area. This witness avoids uncertain full-year energy arithmetic and has a declared change-delta-u-wb leaf/edit rule.

DIN4108 sources: examples/demo ID and assets/demo DSL; inferences/checks/part-7-airtightness n50_limit/check_airtightness; edit-rules path airtightnessN50 -> change-airtightness-n50/newAirtightnessN50.
DIN16798 sources: examples/compliant-office ID and assets/compliant-office DSL; checks/zone-evaluation CO2 calculation; annex-parameters-and-parts DE absolute900 and CatII800; change-zone-co2 leaf.
DIN18599 sources: examples/compliant-detached ID and assets DSL; checks/balance-model and norm-tables; document-evaluation/envelope-and-heating-checks; edit-rules deltaUWbWM2k -> change-delta-u-wb/newDeltaUWbWM2k.

## Catalogue Engineering Candidates with Missing Typed Edit Routes

ISO16757 example demo: partNumberInputs.dn decimal50, script partNumberRule.source='dn * 10 + 50' produces deterministic part number550. Candidate edit dn50->60 produces650; exact check iso16757.5.6.10.partNumber exposes numeric computed/limit equal to550 or650 and explanatory text. However current EDIT_RULES has neither partNumberInputs.dn.value nor partNumberRule.source; only whole partNumberRule -> replace-part-number-rule. Shared NormEditRules resolves exact normalized templates, not arbitrary descendants. Therefore an inspector numeric/text child ID does not establish a working mutation. Do not author a browser test pretending this edit can commit until the actual typed route is implemented. Also the viewer table currently omits computed/explanation and only includes utilization/status/title; its always-pass ratio does not expose the changed part number. A viewer numeric-result witness needs an actual result surface.

VDI3805 example blatt-3: product RAD-600 configuration.attributes kind radiator has heatExponentN1.3; exact check vdi3805.3.n.RAD-600 divides by RADIATOR_N_MAX1.5 and requires range[1.1,1.5]. Candidate edit1.3->1.4 yields utilization0.8666666666666667->0.9333333333333333 (editor87%->93%, viewer0.87->0.93), remaining Pass. Current EDIT_RULES only owns catalog.products[].configuration as a whole -> change-product-configuration/newConfiguration, not catalog.products[].configuration.attributes.heatExponentN. That nested numeric control is not a proved current edit route. Source records duplicate typed configuration information in record210; additional consistency checks may change after the candidate edit, so target the exact n check and retain intended record/config consistency in any future fixture.

## Actual Input/Result/Viewer IDs and Routes

All five editor input windows use norm-<variant>-inputs, body norm.<variant>.play.inputs, render_inspector_for and local row key norm-inputs.<fieldPath>. Quantity child ID is .input; integer controls may be .stepper. Scope DOM/mirror lookup by actual owning window and exact path; do not depend on an arbitrary report index or nearest unrelated input.

Editor results windows use norm-<variant>-results, body norm.<variant>.play.results, and shared render_checks. These are command-style check rows norm-checks-check-<report index>, not canonical check-ID node addresses. The row label contains utilization percentage and title; descriptions contain computed/limit context. Find by exact localized check title plus subject/clause, or add a source-owned stable identity surface before pinning a dynamic report index. Main-row percentage is available even for passing checks; expanded detail children are only guaranteed for Fail/Warning.

Every one of the five readonly report windows uses TableWindowKit KIND_ID/BODY_KEY framework.window.table. Its columns are part, clause, subject, status, utilization formatted to two decimals, title and first remedy. Native check.id is NOT included, and the computed/limit quantity is not included. Viewer table rows are positional; source check IDs cannot be used as current DOM IDs. Match localized part/clause/subject/title and assert two-decimal ratio, with uniqueness. This omission prevents the ISO calculated-part-number value witness from using the current viewer table.

| Variant | React URL base | Editor app / viewer app | Committed example |
| --- | --- | --- | --- |
| din4108 | http://127.0.0.1:6091/ | s.norm.din4108@1/*#editor / #viewer | demo |
| din16798 | http://127.0.0.1:6092/ | s.norm.din16798@1/*#editor / #viewer | compliant-office |
| din18599 | http://127.0.0.1:6093/ | s.norm.din18599@1/*#editor / #viewer | compliant-detached |
| iso16757 | http://127.0.0.1:6104/ | s.norm.iso16757@1/*#editor / #viewer | demo |
| vdi3805 | http://127.0.0.1:6105/ | s.norm.vdi3805@1/*#editor / #viewer | blatt-3 |

Append ?plugin=<variant>&role=editor|viewer&example=<exact ID>. Ports/app defaults are declared in generated Specific playground catalog; query plugin does not change a React server compiled for another variant. Use the registered dev-<variant>-react-prototype target and explicit locale policy from host-e2e-plan.md. No server was started.

The requested future gate can immediately specify three arithmetic witnesses, but ISO/VDI still need real typed leaf wiring and ISO a report-value surface. No placeholder test, invented check name, or source-equals-execution claim is made.

## Exact ISO/VDI Leaf Reachability Evidence

ISO metadata source: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs. It explicitly authors partNumberRule.source and partNumberInputs.*.value. iso16757_field_meta maps paths containing partNumberInputs. and ending .value to that wildcard metadata. The current decimal input is partNumberInputs.dn.value, dimensionless50. Proposed60 gives part number650 instead of550 with unchanged script dn*10+50. UI local row key is norm-inputs.partNumberInputs.dn.value; numeric child depends on resolved numeric control (.input for quantity/floating input, .stepper for integer). Script text row is norm-inputs.partNumberRule.source.input.

ISO exact rule source: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs. Only whole partNumberRule maps to replace-part-number-rule/newRule. Neither scalar source nor partNumberInputs.dn.value maps. The existing scalar operations are names, selection class/series and exchange process, not this arithmetic engineering input. No working scalar numeric engineering rule was identified in the current set_field roster.

VDI metadata source: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs. It explicitly authors catalog.products[].configuration.attributes.heatExponentN, label Heat exponent n / Heizexponent n; dimensionless baseline1.3. Proposed1.4, check vdi3805.3.n.RAD-600, max1.5 => exact ratios13/15 and14/15. Local row key norm-inputs.catalog.products[id=RAD-600].configuration.attributes.heatExponentN, corresponding numeric child .input for the floating field. StandardOutputW is separately authored with Watt and baseline1200 but likewise lacks a scalar rule.

VDI exact rule source: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs. Only catalog.products[].configuration maps to change-product-configuration/newConfiguration with SelectorKey::Id(id). Neither heatExponentN nor standardOutputW scalar descendant maps. Other scalar route strictMode is Boolean, not a numeric engineering property; remaining rules replace manufacturer file, title, date or limits. No working scalar numeric engineering rule was identified.

Shared rendering evidence: ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧾️properties/🦀️.rs defines field_row and entries_section; field_row unconditionally adds field_controls for every rendered scalar and render_inspector_for uses Tier::Expert. The controls module binds each control to the field verb/args. No check against NormEditRules or explicit read-only downgrade for these unsupported leaves was found on this path. This means metadata advertises apparently editable controls once their lazy sections open, but it does not prove the host accepted a field commit.

Shared refusal evidence: ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs, NormEditRules::set_rule, selects only rule.path == normalized template. NormEditRules::payload on SetField errors no kind sets <template> when no exact rule is found. There is no inspected ancestor-whole-object replacement fallback. A whole-object mutation's existence therefore cannot substantiate editing its numeric child. This is source evidence of an editing gap, not an executed browser failure; implementation was not changed.
