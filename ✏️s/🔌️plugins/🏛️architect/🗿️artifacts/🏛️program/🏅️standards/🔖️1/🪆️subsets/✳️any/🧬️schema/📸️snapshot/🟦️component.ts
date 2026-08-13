/** 🧬️ ProgramSnapshot snapshot schema — artifact-lane fields only. */

export interface ProgramSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  meta: ProgramMeta;
  /** @state artifact */
  project: ProjectDefinition;
  /** @state artifact */
  stakeholders: Stakeholder[];
  /** @state artifact */
  users: UserProfile[];
  /** @state artifact */
  activities: Activity[];
  /** @state artifact */
  functions: Function[];
  /** @state artifact */
  elements: ProgramElement[];
  /** @state artifact */
  quantities: QuantityRequirement[];
  /** @state artifact */
  relationships: Relationship[];
  /** @state artifact */
  adjacencies: Adjacency[];
  /** @state artifact */
  processes: Process[];
  /** @state artifact */
  flows: FlowRequirement[];
  /** @state artifact */
  accessRules: AccessRule[];
  /** @state artifact */
  operations: OperationalRequirement[];
  /** @state artifact */
  equipment: Equipment[];
  /** @state artifact */
  resources: Resource[];
  /** @state artifact */
  storage: StorageRequirement[];
  /** @state artifact */
  environmental: EnvironmentalRequirement[];
  /** @state artifact */
  humanFactors: HumanFactorRequirement[];
  /** @state artifact */
  accessibility: AccessibilityRequirement[];
  /** @state artifact */
  privacy: PrivacyRequirement[];
  /** @state artifact */
  safety: SafetyRequirement[];
  /** @state artifact */
  security: SecurityRequirement[];
  /** @state artifact */
  regulatory: RegulatoryRequirement[];
  /** @state artifact */
  siteContext: SiteContext[];
  /** @state artifact */
  organizational: OrganizationalRequirement[];
  /** @state artifact */
  services: ServiceRequirement[];
  /** @state artifact */
  infrastructure: InfrastructureRequirement[];
  /** @state artifact */
  information: InformationRequirement[];
  /** @state artifact */
  communication: CommunicationRequirement[];
  /** @state artifact */
  wayfinding: WayfindingRequirement[];
  /** @state artifact */
  schedules: ScheduleRequirement[];
  /** @state artifact */
  flexibility: FlexibilityRequirement[];
  /** @state artifact */
  growth: GrowthPlan[];
  /** @state artifact */
  sustainability: SustainabilityRequirement[];
  /** @state artifact */
  resilience: ResilienceRequirement[];
  /** @state artifact */
  costs: CostRequirement[];
  /** @state artifact */
  delivery: DeliveryConstraint[];
  /** @state artifact */
  risks: Risk[];
  /** @state artifact */
  conflicts: Conflict[];
  /** @state artifact */
  requirements: Requirement[];
  /** @state artifact */
  priorities: PriorityRecord[];
  /** @state artifact */
  scenarios: Scenario[];
  /** @state artifact */
  options: OptionEvaluation[];
  /** @state artifact */
  decisions: Decision[];
  /** @state artifact */
  validations: ValidationRecord[];
  /** @state artifact */
  performance: PerformanceCriterion[];
  /** @state artifact */
  quality: QualityRecord[];
  /** @state artifact */
  documents: DocumentRecord[];
  /** @state artifact */
  assumptions: Assumption[];
  /** @state artifact */
  constraints: ConstraintRecord[];
  /** @state artifact */
  complianceRecords: ComplianceRecord[];
  /** @state artifact */
  approvals: ApprovalRecord[];
  /** @state artifact */
  meetings: MeetingRecord[];
  /** @state artifact */
  changes: ChangeRecord[];
  /** @state artifact */
  collaboration: CollaborationRecord[];
  /** @state artifact */
  analyses: AnalysisRecord[];
  /** @state artifact */
  reports: ReportRecord[];
  /** @state artifact */
  searchFilters: SearchFilter[];
  /** @state artifact */
  statusRecords: StatusRecord[];
  /** @state artifact */
  workshops: Workshop[];
  /** @state artifact */
  surveys: Survey[];
  /** @state artifact */
  issues: Issue[];
  /** @state artifact */
  auditEvents: AuditEvent[];
  /** @state artifact */
  templates: TemplateRecord[];
  /** @state artifact */
  knowledge: KnowledgeRecord[];
  /** @state artifact */
  benchmarks: BenchmarkRecord[];
  /** @state artifact */
  traces: TraceLink[];
  /** @state artifact */
  governance: Governance;
}

