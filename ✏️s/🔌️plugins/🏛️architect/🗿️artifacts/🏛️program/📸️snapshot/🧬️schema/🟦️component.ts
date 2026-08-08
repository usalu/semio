/** 🧬️ ProgramSnapshot snapshot schema — persistent fields only. */

export interface ProgramSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  meta: ProgramMeta;
  /** @state persistent */
  project: ProjectDefinition;
  /** @state persistent */
  stakeholders: Stakeholder[];
  /** @state persistent */
  users: UserProfile[];
  /** @state persistent */
  activities: Activity[];
  /** @state persistent */
  functions: Function[];
  /** @state persistent */
  elements: ProgramElement[];
  /** @state persistent */
  quantities: QuantityRequirement[];
  /** @state persistent */
  relationships: Relationship[];
  /** @state persistent */
  adjacencies: Adjacency[];
  /** @state persistent */
  processes: Process[];
  /** @state persistent */
  flows: FlowRequirement[];
  /** @state persistent */
  accessRules: AccessRule[];
  /** @state persistent */
  operations: OperationalRequirement[];
  /** @state persistent */
  equipment: Equipment[];
  /** @state persistent */
  resources: Resource[];
  /** @state persistent */
  storage: StorageRequirement[];
  /** @state persistent */
  environmental: EnvironmentalRequirement[];
  /** @state persistent */
  humanFactors: HumanFactorRequirement[];
  /** @state persistent */
  accessibility: AccessibilityRequirement[];
  /** @state persistent */
  privacy: PrivacyRequirement[];
  /** @state persistent */
  safety: SafetyRequirement[];
  /** @state persistent */
  security: SecurityRequirement[];
  /** @state persistent */
  regulatory: RegulatoryRequirement[];
  /** @state persistent */
  siteContext: SiteContext[];
  /** @state persistent */
  organizational: OrganizationalRequirement[];
  /** @state persistent */
  services: ServiceRequirement[];
  /** @state persistent */
  infrastructure: InfrastructureRequirement[];
  /** @state persistent */
  information: InformationRequirement[];
  /** @state persistent */
  communication: CommunicationRequirement[];
  /** @state persistent */
  wayfinding: WayfindingRequirement[];
  /** @state persistent */
  schedules: ScheduleRequirement[];
  /** @state persistent */
  flexibility: FlexibilityRequirement[];
  /** @state persistent */
  growth: GrowthPlan[];
  /** @state persistent */
  sustainability: SustainabilityRequirement[];
  /** @state persistent */
  resilience: ResilienceRequirement[];
  /** @state persistent */
  costs: CostRequirement[];
  /** @state persistent */
  delivery: DeliveryConstraint[];
  /** @state persistent */
  risks: Risk[];
  /** @state persistent */
  conflicts: Conflict[];
  /** @state persistent */
  requirements: Requirement[];
  /** @state persistent */
  priorities: PriorityRecord[];
  /** @state persistent */
  scenarios: Scenario[];
  /** @state persistent */
  options: OptionEvaluation[];
  /** @state persistent */
  decisions: Decision[];
  /** @state persistent */
  validations: ValidationRecord[];
  /** @state persistent */
  performance: PerformanceCriterion[];
  /** @state persistent */
  quality: QualityRecord[];
  /** @state persistent */
  documents: DocumentRecord[];
  /** @state persistent */
  assumptions: Assumption[];
  /** @state persistent */
  constraints: ConstraintRecord[];
  /** @state persistent */
  complianceRecords: ComplianceRecord[];
  /** @state persistent */
  approvals: ApprovalRecord[];
  /** @state persistent */
  meetings: MeetingRecord[];
  /** @state persistent */
  changes: ChangeRecord[];
  /** @state persistent */
  collaboration: CollaborationRecord[];
  /** @state persistent */
  analyses: AnalysisRecord[];
  /** @state persistent */
  reports: ReportRecord[];
  /** @state persistent */
  searchFilters: SearchFilter[];
  /** @state persistent */
  statusRecords: StatusRecord[];
  /** @state persistent */
  workshops: Workshop[];
  /** @state persistent */
  surveys: Survey[];
  /** @state persistent */
  issues: Issue[];
  /** @state persistent */
  auditEvents: AuditEvent[];
  /** @state persistent */
  templates: TemplateRecord[];
  /** @state persistent */
  knowledge: KnowledgeRecord[];
  /** @state persistent */
  benchmarks: BenchmarkRecord[];
  /** @state persistent */
  traces: TraceLink[];
  /** @state persistent */
  governance: Governance;
}

