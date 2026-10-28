# Independent Content Audit

Read root AGENTS.md, ticket plan/reports, all three quiz documents, their neutral fixtures and curriculum tests. No nested teaching AGENTS.md exists. All inspection used repo tools exec Python; source remained unchanged.

## Findings

P2 — Official curriculum tests omit semantic vector contracts. File: 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧩️curriculum/🟦️.ts reads classifications only, ignoring parametric axes/profiles and optimization pareto.baseline/minimize/cases. Existing profile assertions in 🧪️tests/🧪️course/🟦️.ts cover ventilation only. Add exact four-axis and complete category-profile assertions. Independently compute dominance from numeric Pareto vectors with no-worse-all and strictly-better-any, comparing computed result with fixture truth and authored category. Ticket verify-concepts.py guards parametric profiles but is not registered regression coverage.

P3 — Two German labels alter meaning. File: 🎓️teaching/🏛️architecture/📐️parametric/❓️quiz/🔣️.json. parameter-domains/profile-catalog says Auswahl eines Tragprofilkatalogs (selecting a catalog) while English and explanations select a profile entry; prefer Tragprofilauswahl aus einem Katalog. automation-levels/selected-variant-generation says Variantenerzeugung mit Personenauswahl (selection of people); prefer Variantenerzeugung mit menschlicher Auswahl. Short label and explanation already clarify the latter.

## Coverage Confirmed

BIM: data-information contrasts raw coordinate tables and semantic door objects with explicit context, units, relationships and requirements. object-oriented-model covers all six concepts with five alternatives each: class, instance, attribute, method, inheritance, owned composition. Schematic qualifications avoid false literal IFC hierarchy claims. model-coordination separates exchange contracts, shared cross-discipline decisions and isolated work correctly.

Parametric: parameter-domains covers integer/catalog/stepped and idealized continuous domains, machine precision and slider settings. input-output-consistency handles V=LWH, positive divisors, independent inputs, derived outputs and arbitrarily overdetermined controls without claiming all redundant prescribed tuples are numerically inconsistent. complexity-layers separates necessary design dependencies from implementation/environment overhead. design-tool-profiles defines four 1–5 axes and their beneficial/cost directions: versatility, internal complexity, usability, external dependency. Illustrative ratings are explicit. compact-pavilion-generator and redundant-box-controls contrast few independent inputs producing broad coherent variants with many redundant controls producing narrow variants. custom-simulation-network explicitly combines four independent inputs and broad consistent variants with spaghetti complexity. Script length or input count alone never determines versatility. automation-levels distinguishes manual, automated batch and required human checkpoints; human start alone does not make a batch semi-automated.

Optimization: optimization-objectives correctly separates objectives and feasibility constraints and treats fixed scalar weighting as one objective. pareto-dominance restricts each comparison to a feasible candidate/reference pair. Tradeoffs, equality, equal-one/worse-one, both worse and improvements are correct under no-worse-all/strictly-better-any dominance. equal-objectives remains Pareto-optimal within its pair; same-cost-more-carbon and more-cost-same-carbon are dominated; better-both and equal-one/better-one dominate the reference. No authored grading errors found.

## Fixture and Test Scope

All referenced fixture IDs exist with expected categories. BIM covers all six OO categories and all tasks; parametric covers all five tasks and complete four-profile mapping; optimization contains all 25 answer mappings and six numeric boundary/tradeoff vectors. Registered curriculum tests cover answer mappings, bilingual explanation presence, schema/domain validity, all challenge scoring and absence of raw category/explanation in shown items. Schema validity does not establish educational truth.

No builds/browser tests repeated because parent owns integration verification. Primary references are recorded by the author in concepts-research.md; numeric exercises are self-contained. Only this audit report was created.
