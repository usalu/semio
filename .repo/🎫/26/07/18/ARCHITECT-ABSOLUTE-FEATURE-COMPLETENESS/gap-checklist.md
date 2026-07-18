# Architect Feature Completeness — Gap Closure

## Phase 1 — Typed fields
- [ ] ProjectDefinition §1 missing fields
- [ ] FunctionKind enum + Function fields
- [ ] RelationshipKind expansion + Relationship proximity fields
- [ ] Adjacency internal_external_access
- [ ] Governance §56 missing fields
- [ ] TraceKind 19 variants
- [ ] AnalysisKind + ReportKind expansion
- [ ] Assumption, Constraint, Compliance, Approval, Meeting registers
- [ ] Remaining entity bullet fields per audit

## Phase 2 — Behavior
- [ ] validate.rs full cross-ref
- [ ] analyze.rs all kinds + persist
- [ ] report.rs all 20 summaries + matrices
- [ ] search.rs all registers + filters
- [ ] trace.rs kind-specific chains
- [ ] exchange.rs CSV/TSV all registers
- [ ] template.rs all entity kinds
- [ ] status_summary.rs all registers
- [ ] adjacency.rs separation/distance conflicts
- [ ] outputs.rs §65 builders

## Phase 3 — Plugin
- [ ] REGISTER_IDS all registers
- [ ] Generic CRUD actions
- [ ] Editable inspectors
- [ ] Catalogue: templates, import, all analysis/report, search, trace
- [ ] Formatted report body

## Phase 4 — Verify
- [ ] architect_program tests (isolated target)
- [ ] architect-plugin tests (isolated target)
