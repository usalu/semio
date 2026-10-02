# Architect Program Typed Field Inventory

This read-only report inventories the current authoritative JSON schema; it does not generate SQL declarations or a provider. The eventual relational schema must be handwritten per domain and cannot infer tables/columns from this report at runtime. Common EntityHeader fields are listed once and each domain's additional fields are independently enumerated below. Optionality comes from each exact entity definition's required list, and order from the authored schema. Literal identifiers remain string domains rather than surrogate aliases.

## Root

- schema: string
- meta: ProgramMeta
- project: ProjectDefinition
- stakeholders: ordered<Stakeholder>
- users: ordered<UserProfile>
- activities: ordered<Activity>
- functions: ordered<Function>
- elements: ordered<ProgramElement>
- quantities: ordered<QuantityRequirement>
- relationships: ordered<Relationship>
- adjacencies: ordered<Adjacency>
- processes: ordered<Process>
- flows: ordered<FlowRequirement>
- accessRules: ordered<AccessRule>
- operations: ordered<OperationalRequirement>
- equipment: ordered<Equipment>
- resources: ordered<Resource>
- storage: ordered<StorageRequirement>
- environmental: ordered<EnvironmentalRequirement>
- humanFactors: ordered<HumanFactorRequirement>
- accessibility: ordered<AccessibilityRequirement>
- privacy: ordered<PrivacyRequirement>
- safety: ordered<SafetyRequirement>
- security: ordered<SecurityRequirement>
- regulatory: ordered<RegulatoryRequirement>
- siteContext: ordered<SiteContext>
- organizational: ordered<OrganizationalRequirement>
- services: ordered<ServiceRequirement>
- infrastructure: ordered<InfrastructureRequirement>
- information: ordered<InformationRequirement>
- communication: ordered<CommunicationRequirement>
- wayfinding: ordered<WayfindingRequirement>
- schedules: ordered<ScheduleRequirement>
- flexibility: ordered<FlexibilityRequirement>
- growth: ordered<GrowthPlan>
- sustainability: ordered<SustainabilityRequirement>
- resilience: ordered<ResilienceRequirement>
- costs: ordered<CostRequirement>
- delivery: ordered<DeliveryConstraint>
- risks: ordered<Risk>
- conflicts: ordered<Conflict>
- requirements: ordered<Requirement>
- priorities: ordered<PriorityRecord>
- scenarios: ordered<Scenario>
- options: ordered<OptionEvaluation>
- decisions: ordered<Decision>
- validations: ordered<ValidationRecord>
- performance: ordered<PerformanceCriterion>
- quality: ordered<QualityRecord>
- artifacts: ordered<ArtifactRecord>
- assumptions: ordered<Assumption>
- constraints: ordered<ConstraintRecord>
- complianceRecords: ordered<ComplianceRecord>
- approvals: ordered<ApprovalRecord>
- meetings: ordered<MeetingRecord>
- changes: ordered<ChangeRecord>
- collaboration: ordered<CollaborationRecord>
- analyses: ordered<AnalysisRecord>
- reports: ordered<ReportRecord>
- searchFilters: ordered<SearchFilter>
- statusRecords: ordered<StatusRecord>
- workshops: ordered<Workshop>
- surveys: ordered<Survey>
- issues: ordered<Issue>
- auditEvents: ordered<AuditEvent>
- templates: ordered<TemplateRecord>
- knowledgePayload: ordered<KnowledgeRecord>
- knowledge: https://json.schemas.assets.semio-tech.com/os/store/child/schema.json
- benchmarksPayload: ordered<BenchmarkRecord>
- benchmarks: https://json.schemas.assets.semio-tech.com/os/store/child/schema.json
- traces: ordered<TraceLink>
- governance: Governance

## TextField

- text: string
- format?: string

## TimestampMeta

- created: string
- updated: string
- createdBy?: string
- updatedBy?: string

## ProgramMeta

- schema: string
- documentId: string
- title: string
- subtitle: string | null
- purpose: TextField
- terminology: ordered<string>
- classification: ordered<string>
- industrySector: string
- projectType: string
- locale: string
- revision: string
- authorIds: ordered<string>
- sourceSystem: string | null
- exportProfile: string | null
- timestamps: TimestampMeta

## Priority

choice("mandatory"|"essential"|"preferred"|"optional"|"deferred"|"prohibited")

## Ownership

- ownerId: string | null
- authorityId: string | null
- consultantIds: ordered<string>
- participantIds: ordered<string>

## ProjectDefinition

- id: string
- code: string
- clientName: string
- ownerOrganization: string
- briefSummary: TextField
- problemStatement: TextField
- vision: TextField
- mission: TextField
- objectives: ordered<string>
- successCriteria: ordered<string>
- projectPriorities: ordered<Priority>
- completionCriteria: ordered<string>
- decisionCriteria: ordered<string>
- scopeInclusions: ordered<string>
- scopeExclusions: ordered<string>
- assumptions: ordered<string>
- constraintsSummary: ordered<string>
- dependencies: ordered<string>
- deliverables: ordered<string>
- phases: ordered<string>
- geographicContext: TextField
- developmentContext: TextField
- operationalContext: TextField
- regulatoryContext: ordered<string>
- fundingModel: string
- ownership: Ownership
- timestamps: TimestampMeta

## LifecycleStatus

choice("draft"|"proposed"|"underReview"|"validated"|"approved"|"rejected"|"deferred"|"superseded"|"archived"|"open"|"closed"|"atRisk"|"blocked"|"inProgress"|"complete")

## TaggedNote

- tag: string
- text: string

## EntityHeader

- id: string
- name: string
- description?: TextField
- status: LifecycleStatus
- priority: Priority
- ownership: Ownership
- tags?: ordered<string>
- notes?: ordered<TaggedNote>
- timestamps: TimestampMeta

## InfluenceLevel

choice("low"|"medium"|"high"|"critical")

## EngagementLevel

choice("unaware"|"resistant"|"neutral"|"supportive"|"leading")

## Stakeholder

Includes every EntityHeader field with this definition's independently authored optionality.

- role: string
- organization: string
- department: string | null
- contactEmail: string | null
- contactPhone: string | null
- influence: InfluenceLevel
- interest: InfluenceLevel
- engagement: EngagementLevel
- expectations: ordered<string>
- concerns: ordered<string>
- requirementIds: ordered<string>
- decisionAuthority: boolean
- communicationPreferences: ordered<string>
- reportingFrequency: string | null
- involvementPhases: ordered<string>
- availability: string | null
- representativeOf: string | null
- delegatedTo: string | null
- relationshipToClient: string | null
- powerInterestNotes: ordered<TaggedNote>
- stakeholderType: string
- influenceStrategy: string | null
- communicationChannels: ordered<string>
- successMetrics: ordered<string>

## UserCategory

choice("primary"|"secondary"|"occasional"|"service"|"visitor"|"staff"|"public")

## UserProfile

Includes every EntityHeader field with this definition's independently authored optionality.

- category: UserCategory
- demographic: string | null
- ageRange: string | null
- abilities: ordered<string>
- disabilities: ordered<string>
- occupation: string | null
- roleTitle: string | null
- department: string | null
- mobilityProfile: ordered<string>
- sensoryProfile: ordered<string>
- cognitiveProfile: ordered<string>
- behavioralPatterns: ordered<string>
- usageFrequency: string | null
- usageDuration: string | null
- peakUsageTimes: ordered<string>
- technologyProficiency: string | null
- preferences: ordered<string>
- painPoints: ordered<string>
- goals: ordered<string>
- activityIds: ordered<string>
- researchMethod: string | null
- personaArchetype: string | null
- validated: boolean
- stakeholderIds: ordered<string>

## QuantitySpec

- min?: number
- max?: number
- target?: number
- current?: number
- forecast?: number
- peak?: number
- average?: number
- unit: string

## Activity

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- category: string
- frequency: string | null
- duration: string | null
- intensity: string | null
- participants: QuantitySpec
- equipmentIds: ordered<string>
- spaceRequirements: ordered<string>
- environmentalNeeds: ordered<string>
- privacyNeeds: ordered<string>
- accessibilityNeeds: ordered<string>
- adjacentActivities: ordered<string>
- sequencing: ordered<string>
- peakPeriods: ordered<string>
- workflowSteps: ordered<string>
- inputs: ordered<string>
- outputs: ordered<string>
- userProfileIds: ordered<string>
- functionIds: ordered<string>
- performanceIndicators: ordered<string>
- activityType: string
- locationContext: string | null
- temporalPattern: string | null
- supervisionLevel: string | null

## FunctionKind

choice("primary"|"secondary"|"support"|"administrative"|"service"|"technical"|"public"|"private"|"shared"|"restricted"|"temporary"|"future"|"operational"|"circulation")

## Function

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- kind: FunctionKind
- purpose: TextField
- criticality: Priority
- performanceTargets: ordered<string>
- serviceLevel: string | null
- operatingHours: string | null
- staffing: QuantitySpec
- equipmentIds: ordered<string>
- resourceIds: ordered<string>
- activityIds: ordered<string>
- elementIds: ordered<string>
- dependencies: ordered<string>
- interfaces: ordered<string>
- constraints: ordered<string>
- qualityCriteria: ordered<string>
- regulatoryRefs: ordered<string>
- futureChanges: ordered<string>
- ownerStakeholderId: string | null
- successMetrics: ordered<string>
- hierarchyParentId: string | null
- conflictIds: ordered<string>

## ProgramElementKind

choice("building"|"campus"|"floor"|"zone"|"room"|"suite"|"department"|"system"|"circulation"|"support"|"outdoor"|"furnitureGroup"|"other")

## ProgramElement

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- kind: ProgramElementKind
- parentId: string | null
- level: string | null
- area: QuantitySpec
- volume: QuantitySpec
- height: QuantitySpec
- occupancy: QuantitySpec
- functionIds: ordered<string>
- activityIds: ordered<string>
- userProfileIds: ordered<string>
- adjacencyIds: ordered<string>
- quantityIds: ordered<string>
- requirementIds: ordered<string>
- locationHint: string | null
- orientation: string | null
- daylightRequirement: string | null
- acousticClass: string | null
- securityZone: string | null
- flexibilityNotes: ordered<string>
- growthAllocation: string | null
- circulationRole: string | null
- visibilityLevel: string | null
- adjacencyPreferences: ordered<string>
- environmentalZone: string | null

## QuantityRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- targetElementId: string
- metric: string
- quantity: QuantitySpec
- basis: string | null
- calculationMethod: string | null
- source: string | null
- benchmarkRef: string | null
- tolerancePercent: number | null
- peakFactor: number | null
- growthFactor: number | null
- unitCost: number | null
- currency: string | null
- verificationMethod: string | null
- relatedRequirementIds: ordered<string>
- assumptions: ordered<string>
- constraints: ordered<string>
- schedulePhase: string | null
- responsibleParty: string | null
- lastVerified: string | null
- varianceNotes: ordered<TaggedNote>

## RelationshipKind

choice("contains"|"serves"|"supports"|"dependsOn"|"conflictsWith"|"equivalentTo"|"adjacentTo"|"feeds"|"receives"|"controls"|"monitors"|"functional"|"operational"|"organizational"|"user"|"service"|"information"|"access"|"security"|"supervision"|"communication"|"dependency"|"sequential"|"sharedResource")

## TraceKind

choice("objectiveToRequirement"|"stakeholderToRequirement"|"userToActivity"|"activityToFunction"|"functionToProgramElement"|"requirementToDecision"|"requirementToRisk"|"requirementToStandard"|"requirementToValidation"|"requirementToApproval"|"requirementToChange"|"equipmentToActivity"|"processToResource"|"constraintToImpact"|"scenarioToDecision"|"issueToAction"|"actionToOwner"|"decisionToOutcome"|"versionToChange"|"fullAuditTrail")

## TraceLink

- id: string
- fromId: string
- toId: string
- kind: TraceKind
- label?: string

## SeparationKind

choice("acoustic"|"visual"|"security"|"olfactory"|"thermal"|"fire"|"hygienic"|"circulation"|"operational"|"infectionControl")

## Relationship

Includes every EntityHeader field with this definition's independently authored optionality.

- sourceId: string
- targetId: string
- kind: RelationshipKind
- strength: number | null
- directional: boolean
- rationale: TextField | null
- constraints: ordered<string>
- conditions: ordered<string>
- relationshipPriority: Priority
- validFrom: string | null
- validUntil: string | null
- evidence: ordered<string>
- conflictIds: ordered<string>
- traceLinks: ordered<TraceLink>
- bidirectional: boolean
- distanceConstraintM: number | null
- capacityConstraint: string | null
- regulatoryBasis: ordered<string>
- reviewCycle: string | null
- ownerId: string | null
- proximityRequirement: TextField | null
- compatibilityRequirement: TextField | null
- incompatibilityRequirement: TextField | null
- separationRequirements: ordered<SeparationKind>

## AdjacencyKind

choice("required"|"preferred"|"optional"|"prohibited")

## ConnectionKind

choice("direct"|"indirect"|"controlled"|"sharedAccess"|"none")

## ValidationStatus

choice("pending"|"passed"|"failed"|"waived"|"deferred")

## Adjacency

Includes every EntityHeader field with this definition's independently authored optionality.

- elementAId: string
- elementBId: string
- kind: AdjacencyKind
- connection: ConnectionKind
- separations: ordered<SeparationKind>
- weight: number
- rationale: TextField | null
- distanceMaxM: number | null
- distanceMinM: number | null
- levelConstraint: string | null
- accessPath: string | null
- sharedWall: boolean
- sharedEntry: boolean
- trafficIsolation: boolean
- circulationOverlap: boolean
- conflictIds: ordered<string>
- normalized: boolean
- verificationStatus: ValidationStatus
- sourceRelationshipId: string | null
- internalExternalAccess: string | null

## Process

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- category: string
- trigger: string | null
- inputs: ordered<string>
- outputs: ordered<string>
- steps: ordered<string>
- actors: ordered<string>
- equipmentIds: ordered<string>
- elementIds: ordered<string>
- duration: string | null
- frequency: string | null
- criticalPath: boolean
- bottlenecks: ordered<string>
- dependencies: ordered<string>
- kpis: ordered<string>
- automationLevel: string | null
- failureModes: ordered<string>
- improvementOpportunities: ordered<string>
- regulatoryRefs: ordered<string>
- ownerId: string | null
- workflowType: string | null
- handoffPoints: ordered<string>
- qualityGates: ordered<string>

## FlowKind

choice("people"|"material"|"information"|"service"|"equipment"|"waste"|"emergency"|"vehicle")

## FlowDirection

choice("oneWay"|"twoWay"|"bidirectionalPeak"|"restricted")

## AccessLevel

choice("public"|"restricted"|"controlled"|"private"|"secure"|"emergencyOnly")

## FlowRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- fromElementId: string
- toElementId: string
- kind: FlowKind
- flowType: string
- direction: FlowDirection
- volume: QuantitySpec
- peakRate: number | null
- clearWidthM: number | null
- clearHeightM: number | null
- separationRequirements: ordered<SeparationKind>
- accessLevel: AccessLevel
- timeWindows: ordered<string>
- equipmentClearance: string | null
- signageRequired: boolean
- escortRequired: boolean
- emergencyRoute: boolean
- barrierFree: boolean
- monitoringRequired: boolean
- processId: string | null
- conflictIds: ordered<string>
- verificationMethod: string | null

## AccessMode

choice("unrestricted"|"cardControlled"|"biometric"|"keyed"|"escortRequired"|"timeRestricted"|"roleBased"|"emergencyOnly")

## AccessRule

Includes every EntityHeader field with this definition's independently authored optionality.

- subjectIds: ordered<string>
- resourceIds: ordered<string>
- accessLevel: AccessLevel
- accessMode: AccessMode
- authentication: ordered<string>
- authorization: ordered<string>
- timeRestrictions: ordered<string>
- escortPolicy: string | null
- visitorPolicy: string | null
- emergencyOverride: boolean
- auditRequired: boolean
- badgeRequired: boolean
- biometricRequired: boolean
- zoneIds: ordered<string>
- exceptions: ordered<string>
- regulatoryBasis: ordered<string>
- enforcementMethod: string | null
- revocationPolicy: string | null
- trainingRequired: boolean
- ownerId: string | null

## OperationalRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- operation: string
- serviceLevel: string | null
- operatingHours: string | null
- staffing: QuantitySpec
- maintenanceInterval: string | null
- cleaningRegime: string | null
- turnaroundTime: string | null
- redundancy: string | null
- uptimeTarget: number | null
- responseTime: string | null
- equipmentIds: ordered<string>
- elementIds: ordered<string>
- processIds: ordered<string>
- utilities: ordered<string>
- wasteStreams: ordered<string>
- contingencyPlan: ordered<string>
- trainingRequirements: ordered<string>
- sopReferences: ordered<string>
- kpiTargets: ordered<string>
- ownerId: string | null
- serviceCategory: string | null
- shiftPattern: string | null
- slaTarget: string | null
- escalationContactId: string | null

## Equipment

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- category: string
- manufacturer: string | null
- model: string | null
- quantity: QuantitySpec
- dimensions: string | null
- weightKg: number | null
- powerKw: number | null
- utilityConnections: ordered<string>
- ventilation: string | null
- noiseLevelDb: number | null
- clearance: string | null
- mounting: string | null
- elementIds: ordered<string>
- activityIds: ordered<string>
- maintenanceAccess: ordered<string>
- lifecycleYears: number | null
- replacementCost: number | null
- standards: ordered<string>
- supplier: string | null
- activityLinkIds: ordered<string>
- installationRequirements: ordered<string>
- commissioningNotes: ordered<string>
- spareParts: ordered<string>

## Resource

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- category: string
- resourceType: string
- quantity: QuantitySpec
- mobility: string | null
- sharingModel: string | null
- allocation: string | null
- elementIds: ordered<string>
- activityIds: ordered<string>
- userProfileIds: ordered<string>
- storageRequirementId: string | null
- durability: string | null
- cleaningRequirements: ordered<string>
- replacementCycle: string | null
- costPerUnit: number | null
- supplier: string | null
- standards: ordered<string>
- ergonomicNotes: ordered<string>
- customization: ordered<string>
- disposalNotes: ordered<string>
- furnitureClass: string | null
- ergonomicsRating: string | null
- sharingRatio: number | null

## StorageClass

choice("general"|"secure"|"climateControlled"|"hazardous"|"archive"|"mobile"|"fixed"|"shared"|"coldChain"|"flammable")

## StorageRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- storedItem: string
- storageClass: StorageClass
- quantity: QuantitySpec
- volumeM3: number | null
- weightKg: number | null
- temperatureRange: string | null
- humidityRange: string | null
- securityLevel: AccessLevel
- hazardClass: string | null
- retentionPeriod: string | null
- accessFrequency: string | null
- elementIds: ordered<string>
- equipmentIds: ordered<string>
- handlingEquipment: ordered<string>
- fireProtection: ordered<string>
- ventilation: string | null
- organizationSystem: string | null
- growthAllowance: number | null
- regulatoryRefs: ordered<string>
- ownerId: string | null

## EnvironmentalParameter

choice("temperature"|"humidity"|"airQuality"|"lighting"|"acoustics"|"ventilation"|"radiation"|"vibration"|"pressure"|"iaq")

## EnvironmentalRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- parameterKind: EnvironmentalParameter
- parameter: string
- targetValue: number | null
- unit: string | null
- minValue: number | null
- maxValue: number | null
- comfortBand: string | null
- measurementMethod: string | null
- monitoringFrequency: string | null
- elementIds: ordered<string>
- occupancyBasis: string | null
- seasonalVariation: ordered<string>
- energyImplications: ordered<string>
- standards: ordered<string>
- certificationTargets: ordered<string>
- outdoorConditions: ordered<string>
- ventilationStrategy: string | null
- daylightTarget: string | null
- acousticTarget: string | null
- iaqTarget: string | null
- verificationPlan: string | null

## HumanFactorAspect

choice("ergonomics"|"cognition"|"sensory"|"social"|"cultural"|"behavioral"|"physical"|"psychological"|"fatigue"|"stress")

## HumanFactorRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- aspect: HumanFactorAspect
- factor: string
- userProfileIds: ordered<string>
- activityIds: ordered<string>
- ergonomicCriteria: ordered<string>
- cognitiveLoad: string | null
- visualDemands: ordered<string>
- auditoryDemands: ordered<string>
- postureRequirements: ordered<string>
- reachEnvelope: string | null
- lightingForTasks: ordered<string>
- thermalComfort: ordered<string>
- privacyNeeds: ordered<string>
- socialInteraction: ordered<string>
- stressFactors: ordered<string>
- mitigationMeasures: ordered<string>
- trainingNeeds: ordered<string>
- standards: ordered<string>
- researchBasis: ordered<string>
- elementIds: ordered<string>
- verificationMethod: string | null

## AccessibilityRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- standard: string
- level: string | null
- userProfileIds: ordered<string>
- elementIds: ordered<string>
- routeIds: ordered<string>
- clearWidthM: number | null
- clearHeightM: number | null
- turningCircleM: number | null
- rampSlope: number | null
- liftRequired: boolean
- tactileGuidance: boolean
- hearingLoop: boolean
- visualContrast: boolean
- signageRequirements: ordered<string>
- controlsHeight: string | null
- emergencyEvacuation: ordered<string>
- serviceAnimalPolicy: string | null
- companionSeating: boolean
- verificationPlan: string | null
- exceptions: ordered<string>
- wcagConformance: string | null
- universalDesignPrinciples: ordered<string>

## PrivacyKind

choice("public"|"semiPublic"|"semiPrivate"|"private"|"confidential"|"restricted"|"anonymous")

## PrivacyRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- privacyKind: PrivacyKind
- privacyType: string
- level: string | null
- subjectIds: ordered<string>
- elementIds: ordered<string>
- visualPrivacy: ordered<string>
- acousticPrivacy: ordered<string>
- dataPrivacy: ordered<string>
- screeningRequired: boolean
- enclosureRequired: boolean
- accessRestrictions: ordered<string>
- observationRisk: string | null
- regulatoryBasis: ordered<string>
- culturalConsiderations: ordered<string>
- technologyControls: ordered<string>
- signage: ordered<string>
- monitoringRestrictions: ordered<string>
- retentionPolicy: string | null
- breachResponse: ordered<string>
- ownerId: string | null

## SafetyDomain

choice("lifeSafety"|"occupationalHealth"|"fire"|"structural"|"electrical"|"chemical"|"radiation"|"ergonomics"|"biological"|"environmental")

## RiskLevel

choice("negligible"|"low"|"medium"|"high"|"critical")

## SafetyRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- safetyDomain: SafetyDomain
- hazard: string
- riskLevel: RiskLevel
- affectedElementIds: ordered<string>
- affectedUserIds: ordered<string>
- mitigationMeasures: ordered<string>
- ppeRequirements: ordered<string>
- emergencyProcedures: ordered<string>
- evacuationRequirements: ordered<string>
- fireProtection: ordered<string>
- structuralSafety: ordered<string>
- slipTripFall: ordered<string>
- chemicalSafety: ordered<string>
- electricalSafety: ordered<string>
- machinerySafety: ordered<string>
- standards: ordered<string>
- inspectionFrequency: string | null
- trainingRequirements: ordered<string>
- incidentReporting: ordered<string>
- residualRisk: string | null

## SecurityControlKind

choice("accessControl"|"surveillance"|"perimeter"|"cyber"|"personnel"|"information"|"physical"|"procedural"|"screening"|"keyManagement")

## SecurityRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- controlKind: SecurityControlKind
- threat: string
- riskLevel: RiskLevel
- assetIds: ordered<string>
- zoneIds: ordered<string>
- accessLevel: AccessLevel
- perimeterControls: ordered<string>
- surveillance: ordered<string>
- intrusionDetection: ordered<string>
- cybersecurity: ordered<string>
- screening: ordered<string>
- visitorManagement: ordered<string>
- keyManagement: ordered<string>
- standards: ordered<string>
- responseProcedures: ordered<string>
- drillFrequency: string | null
- liaisonContacts: ordered<string>
- classifiedLevel: string | null
- redundancy: ordered<string>
- auditRequirements: ordered<string>

## RegulatoryRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- jurisdiction: string
- code: string
- clause: string | null
- title: string
- requirementText: TextField
- applicability: ordered<string>
- elementIds: ordered<string>
- complianceMethod: string | null
- evidenceRequired: ordered<string>
- authority: string | null
- effectiveDate: string | null
- expiryDate: string | null
- penalties: ordered<string>
- exemptions: ordered<string>
- relatedRequirementIds: ordered<string>
- interpretationNotes: ordered<TaggedNote>
- verificationStatus: ValidationStatus
- consultantRefs: ordered<string>
- updateSource: string | null

## SiteContext

Includes every EntityHeader field with this definition's independently authored optionality.

- siteName: string
- address: string | null
- latitude: number | null
- longitude: number | null
- elevationM: number | null
- climateZone: string | null
- seismicZone: string | null
- floodRisk: string | null
- soilConditions: ordered<string>
- utilitiesAvailable: ordered<string>
- accessRoads: ordered<string>
- publicTransit: ordered<string>
- neighbors: ordered<string>
- views: ordered<string>
- noiseSources: ordered<string>
- environmentalConstraints: ordered<string>
- heritageConstraints: ordered<string>
- zoning: string | null
- maxHeightM: number | null
- maxCoverage: number | null

## OrganizationalRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- department: string
- reportingLine: string | null
- headcount: QuantitySpec
- growthPlanId: string | null
- workPatterns: ordered<string>
- collaborationModel: string | null
- hierarchyLevels: ordered<string>
- decisionMaking: ordered<string>
- cultureNotes: ordered<string>
- changeReadiness: string | null
- unionConsiderations: ordered<string>
- trainingNeeds: ordered<string>
- elementIds: ordered<string>
- stakeholderIds: ordered<string>
- serviceRequirementIds: ordered<string>
- brandingRequirements: ordered<string>
- wellnessPlugins: ordered<string>
- diversityGoals: ordered<string>
- ownerId: string | null

## ServiceRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- serviceName: string
- serviceType: string
- provider: string | null
- serviceLevel: string | null
- operatingHours: string | null
- capacity: QuantitySpec
- responseTime: string | null
- queueManagement: ordered<string>
- customerProfiles: ordered<string>
- elementIds: ordered<string>
- equipmentIds: ordered<string>
- staffing: QuantitySpec
- qualityMetrics: ordered<string>
- costModel: string | null
- contractRefs: ordered<string>
- dependencies: ordered<string>
- failureImpact: string | null
- backupService: ordered<string>
- feedbackChannels: ordered<string>

## InfrastructureRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- system: string
- category: string
- capacity: QuantitySpec
- redundancy: string | null
- distribution: ordered<string>
- entryPoints: ordered<string>
- utilitySource: string | null
- standbyPower: boolean
- monitoring: ordered<string>
- maintenanceAccess: ordered<string>
- standards: ordered<string>
- elementIds: ordered<string>
- peakDemand: number | null
- diversityFactor: number | null
- futureExpansion: ordered<string>
- interfaceRequirements: ordered<string>
- commissioning: ordered<string>
- lifecycleCost: number | null
- ownerId: string | null

## InformationRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- informationType: string
- format: string | null
- sourceSystem: string | null
- destinationSystems: ordered<string>
- updateFrequency: string | null
- retentionPeriod: string | null
- accessControls: ordered<string>
- classification: string | null
- qualityCriteria: ordered<string>
- metadataRequirements: ordered<string>
- integrationPoints: ordered<string>
- backupRequirements: ordered<string>
- disasterRecovery: ordered<string>
- privacyControls: ordered<string>
- auditTrail: boolean
- elementIds: ordered<string>
- stakeholderIds: ordered<string>
- standards: ordered<string>
- ownerId: string | null

## CommunicationRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- channel: string
- audienceIds: ordered<string>
- messageTypes: ordered<string>
- frequency: string | null
- medium: ordered<string>
- language: ordered<string>
- accessibility: ordered<string>
- emergencyUse: boolean
- twoWay: boolean
- recordingPolicy: string | null
- signageLocations: ordered<string>
- technology: ordered<string>
- escalationPath: ordered<string>
- feedbackLoop: boolean
- privacyControls: ordered<string>
- elementIds: ordered<string>
- standards: ordered<string>
- ownerId: string | null
- templates: ordered<string>

## WayfindingRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- userProfileIds: ordered<string>
- elementIds: ordered<string>
- destinationTypes: ordered<string>
- signageTypes: ordered<string>
- languages: ordered<string>
- tactileRequired: boolean
- audioRequired: boolean
- digitalWayfinding: boolean
- landmarkStrategy: ordered<string>
- colorCoding: ordered<string>
- symbolStandards: ordered<string>
- decisionPoints: ordered<string>
- maximumSignageDistanceM: number | null
- lightingRequirements: ordered<string>
- maintenancePlan: string | null
- emergencyEgress: ordered<string>
- visitorJourney: ordered<string>
- staffJourney: ordered<string>
- brandIntegration: ordered<string>

## DeliveryPhase

choice("concept"|"schematic"|"designDevelopment"|"constructionDocuments"|"procurement"|"construction"|"commissioning"|"occupancy")

## ScheduleRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- milestone: string
- phase: DeliveryPhase
- startDate: string | null
- endDate: string | null
- duration: string | null
- dependencies: ordered<string>
- predecessors: ordered<string>
- successors: ordered<string>
- critical: boolean
- floatDays: number | null
- resourceRequirements: ordered<string>
- occupancyImpact: ordered<string>
- phasingStrategy: string | null
- decantRequirements: ordered<string>
- commissioningWindow: string | null
- stakeholderIds: ordered<string>
- riskIds: ordered<string>
- contingencyDays: number | null
- reportingCadence: string | null
- ownerId: string | null

## FlexibilityRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- flexibilityType: string
- elementIds: ordered<string>
- adaptationScenarios: ordered<string>
- modularityLevel: string | null
- reconfigurationTime: string | null
- costOfChange: number | null
- technologyReadiness: string | null
- futureFunctionIds: ordered<string>
- demountablePartitions: boolean
- raisedFloor: boolean
- overheadServices: boolean
- expansionDirection: ordered<string>
- contractionScenario: ordered<string>
- multiUsePotential: ordered<string>
- furnitureStrategy: ordered<string>
- infrastructureSpareCapacity: ordered<string>
- leaseImplications: ordered<string>
- ownerId: string | null

## GrowthPlan

Includes every EntityHeader field with this definition's independently authored optionality.

- horizonYears: number
- growthRate: number | null
- headcountGrowth: QuantitySpec
- areaGrowth: QuantitySpec
- phases: ordered<string>
- triggerEvents: ordered<string>
- expansionElementIds: ordered<string>
- reserveAreas: ordered<string>
- infrastructureHeadroom: ordered<string>
- budgetEnvelope: number | null
- fundingSources: ordered<string>
- riskFactors: ordered<string>
- decisionPoints: ordered<string>
- scenarioIds: ordered<string>
- decommissionPlan: ordered<string>
- relocationStrategy: ordered<string>
- stakeholderImpact: ordered<string>
- regulatoryConsiderations: ordered<string>
- ownerId: string | null

## SustainabilityRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- topic: string
- target: string | null
- metric: string | null
- baseline: number | null
- targetValue: number | null
- unit: string | null
- certification: ordered<string>
- standards: ordered<string>
- elementIds: ordered<string>
- strategies: ordered<string>
- materialsPreferences: ordered<string>
- energyStrategy: ordered<string>
- waterStrategy: ordered<string>
- wasteStrategy: ordered<string>
- biodiversity: ordered<string>
- embodiedCarbon: number | null
- operationalCarbon: number | null
- reportingRequirements: ordered<string>
- verificationPlan: string | null
- ownerId: string | null

## ResilienceRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- hazard: string
- riskLevel: RiskLevel
- scenario: string | null
- recoveryTime: string | null
- recoveryPoint: string | null
- redundancy: ordered<string>
- hardeningMeasures: ordered<string>
- backupSystems: ordered<string>
- alternateSites: ordered<string>
- supplyChain: ordered<string>
- communicationPlan: ordered<string>
- drillRequirements: ordered<string>
- elementIds: ordered<string>
- infrastructureIds: ordered<string>
- standards: ordered<string>
- insuranceImplications: ordered<string>
- climateAdaptation: ordered<string>
- ownerId: string | null
- verificationPlan: string | null

## CostBasis

choice("capital"|"operational"|"lifecycle"|"replacement"|"maintenance")

## CostRequirement

Includes every EntityHeader field with this definition's independently authored optionality.

- costItem: string
- basis: CostBasis
- amount: number | null
- currency: string
- quantityBasis: string | null
- unitCost: number | null
- contingencyPercent: number | null
- escalationRate: number | null
- fundingSource: string | null
- elementIds: ordered<string>
- requirementIds: ordered<string>
- phase: DeliveryPhase | null
- cashFlowProfile: ordered<string>
- valueEngineeringNotes: ordered<string>
- benchmarkRef: string | null
- approvalStatus: ValidationStatus
- ownerId: string | null
- assumptions: ordered<string>
- sensitivityFactors: ordered<string>

## DeliveryConstraint

Includes every EntityHeader field with this definition's independently authored optionality.

- constraintType: string
- constraintDetails: TextField
- phase: DeliveryPhase
- hardDeadline: string | null
- softDeadline: string | null
- impactedElementIds: ordered<string>
- impactedRequirementIds: ordered<string>
- workHours: string | null
- noiseRestrictions: ordered<string>
- accessRestrictions: ordered<string>
- siteLogistics: ordered<string>
- procurementLeadTime: string | null
- approvalGates: ordered<string>
- occupancyConstraints: ordered<string>
- weatherWindows: ordered<string>
- penaltyClauses: ordered<string>
- mitigationOptions: ordered<string>
- ownerId: string | null
- riskIds: ordered<string>
- constraintStatus: LifecycleStatus

## Risk

Includes every EntityHeader field with this definition's independently authored optionality.

- riskStatement: TextField
- category: string
- probability: RiskLevel
- impact: RiskLevel
- riskScore: number | null
- causes: ordered<string>
- effects: ordered<string>
- affectedElementIds: ordered<string>
- affectedRequirementIds: ordered<string>
- mitigation: ordered<string>
- contingency: ordered<string>
- ownerId: string | null
- reviewDate: string | null
- triggerIndicators: ordered<string>
- residualProbability: RiskLevel | null
- residualImpact: RiskLevel | null
- relatedConflictIds: ordered<string>
- escalationPath: ordered<string>
- monitoringPlan: string | null

## ConflictKind

choice("adjacency"|"capacity"|"schedule"|"budget"|"regulatory"|"operational"|"environmental"|"security"|"priority")

## IssueSeverity

choice("cosmetic"|"minor"|"major"|"critical"|"blocker")

## Conflict

Includes every EntityHeader field with this definition's independently authored optionality.

- kind: ConflictKind
- summary: TextField
- entityAId: string
- entityBId: string
- severity: IssueSeverity
- detectedBy: string | null
- detectionDate: string | null
- tradeOffOptions: ordered<string>
- recommendedResolution: TextField | null
- decisionId: string | null
- stakeholderIds: ordered<string>
- requirementIds: ordered<string>
- costImpact: number | null
- scheduleImpact: string | null
- qualityImpact: ordered<string>
- resolutionStatus: ValidationStatus
- ownerId: string | null
- escalationLevel: string | null
- relatedRiskIds: ordered<string>

## RequirementKind

choice("functional"|"spatial"|"performance"|"regulatory"|"operational"|"technical"|"aesthetic"|"sustainability")

## Requirement

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- kind: RequirementKind
- statement: TextField
- rationale: TextField | null
- source: string | null
- stakeholderIds: ordered<string>
- elementIds: ordered<string>
- functionIds: ordered<string>
- parentRequirementId: string | null
- childRequirementIds: ordered<string>
- acceptanceCriteria: ordered<string>
- verificationMethod: string | null
- validationStatus: ValidationStatus
- conflictIds: ordered<string>
- riskIds: ordered<string>
- costEstimate: number | null
- scheduleConstraint: string | null
- regulatoryRefs: ordered<string>
- traceLinks: ordered<TraceLink>
- supersededBy: string | null

## PriorityRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- subjectId: string
- subjectKind: string
- rankedPriority: Priority
- rank: number | null
- weight: number | null
- rationale: TextField | null
- decisionId: string | null
- stakeholderIds: ordered<string>
- effectiveFrom: string | null
- effectiveUntil: string | null
- reviewCycle: string | null
- dependencies: ordered<string>
- conflicts: ordered<string>
- scoringMethod: string | null
- score: number | null
- criteria: ordered<string>
- approvedBy: string | null
- approvalDate: string | null
- rankingNotes: ordered<TaggedNote>

## Scenario

Includes every EntityHeader field with this definition's independently authored optionality.

- code: string
- hypothesis: TextField
- assumptions: ordered<string>
- variables: ordered<string>
- elementIds: ordered<string>
- requirementIds: ordered<string>
- growthPlanId: string | null
- probability: number | null
- impactSummary: TextField | null
- costDelta: number | null
- areaDelta: number | null
- headcountDelta: number | null
- scheduleDelta: string | null
- riskIds: ordered<string>
- optionIds: ordered<string>
- baseline: boolean
- preferred: boolean
- analysisIds: ordered<string>
- ownerId: string | null

## OptionEvaluation

Includes every EntityHeader field with this definition's independently authored optionality.

- optionName: string
- optionDescription: TextField
- scenarioId: string | null
- criteriaIds: ordered<string>
- scores: ordered<number>
- weightedScore: number | null
- costEstimate: number | null
- scheduleEstimate: string | null
- riskSummary: ordered<string>
- benefits: ordered<string>
- drawbacks: ordered<string>
- assumptions: ordered<string>
- dependencies: ordered<string>
- stakeholderFeedback: ordered<TaggedNote>
- recommendation: string | null
- decisionId: string | null
- evaluationStatus: ValidationStatus
- evaluatorIds: ordered<string>
- evaluationDate: string | null

## Decision

Includes every EntityHeader field with this definition's independently authored optionality.

- decisionStatement: TextField
- context: TextField
- optionsConsidered: ordered<string>
- selectedOptionId: string | null
- rationale: TextField
- decisionMakerIds: ordered<string>
- consultedIds: ordered<string>
- informedIds: ordered<string>
- decisionDate: string | null
- effectiveDate: string | null
- reversalConditions: ordered<string>
- impactedRequirementIds: ordered<string>
- impactedElementIds: ordered<string>
- costImpact: number | null
- scheduleImpact: string | null
- riskImpact: ordered<string>
- approvalStatus: ValidationStatus
- meetingRef: string | null
- artifactRefs: ordered<string>

## ValidationRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- subjectId: string
- subjectKind: string
- validationType: string
- method: string | null
- criteria: ordered<string>
- result: ValidationStatus
- evidence: ordered<string>
- validatorIds: ordered<string>
- validationDate: string | null
- nextReviewDate: string | null
- findings: ordered<string>
- nonConformities: ordered<string>
- correctiveActions: ordered<string>
- waivers: ordered<string>
- standards: ordered<string>
- traceLinks: ordered<TraceLink>
- reportId: string | null
- confidenceLevel: string | null
- validationNotes: ordered<TaggedNote>

## PerformanceCriterion

Includes every EntityHeader field with this definition's independently authored optionality.

- criterion: string
- metric: string
- target: number | null
- unit: string | null
- minimum: number | null
- maximum: number | null
- measurementMethod: string | null
- frequency: string | null
- requirementIds: ordered<string>
- elementIds: ordered<string>
- baseline: number | null
- benchmarkRef: string | null
- weight: number | null
- dataSource: string | null
- reportingCadence: string | null
- ownerId: string | null
- verificationPlan: string | null
- penaltyThreshold: number | null
- incentiveThreshold: number | null

## QualityRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- qualityTopic: string
- standard: string | null
- targetLevel: string | null
- inspectionPoints: ordered<string>
- acceptanceCriteria: ordered<string>
- testingRequirements: ordered<string>
- sampleRate: string | null
- defectCategories: ordered<string>
- correctiveActionProcess: ordered<string>
- elementIds: ordered<string>
- requirementIds: ordered<string>
- supplierRequirements: ordered<string>
- documentationRequirements: ordered<string>
- trainingRequirements: ordered<string>
- auditSchedule: string | null
- kpis: ordered<string>
- ownerId: string | null
- certificationTargets: ordered<string>
- continuousImprovement: ordered<string>

## ArtifactRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- documentType: string
- title: string
- version: string
- fileRef: string | null
- format: string | null
- authorIds: ordered<string>
- reviewerIds: ordered<string>
- approverIds: ordered<string>
- issueDate: string | null
- revisionDate: string | null
- distributionList: ordered<string>
- relatedEntityIds: ordered<string>
- classification: string | null
- retentionPeriod: string | null
- accessControls: ordered<string>
- supersedes: string | null
- documentStatus: LifecycleStatus
- checksum: string | null
- sourceSystem: string | null

## Assumption

Includes every EntityHeader field with this definition's independently authored optionality.

- statement: TextField
- basis: TextField | null
- confidenceLevel: string | null
- impactIfFalse: TextField | null
- relatedEntityIds: ordered<string>
- validationStatus: ValidationStatus
- validatedBy: string | null
- validationDate: string | null
- ownerId: string | null
- reviewCycle: string | null
- source: string | null
- category: string | null
- dependencies: ordered<string>
- mitigation: ordered<string>
- linkedRequirementIds: ordered<string>
- linkedRiskIds: ordered<string>
- expirationDate: string | null
- statusNotes: ordered<TaggedNote>
- artifactRefs: ordered<string>

## ConstraintRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- constraintType: string
- summary: TextField
- severity: RiskLevel
- affectedEntityIds: ordered<string>
- source: string | null
- regulatoryBasis: ordered<string>
- mitigationOptions: ordered<string>
- ownerId: string | null
- effectiveDate: string | null
- expiryDate: string | null
- waiverStatus: string | null
- waiverApprover: string | null
- impactAssessment: TextField | null
- resolutionPlan: ordered<string>
- relatedRequirementIds: ordered<string>
- relatedDecisionIds: ordered<string>
- monitoringFrequency: string | null
- complianceStatus: ValidationStatus
- exceptions: ordered<string>
- traceLinks: ordered<TraceLink>
- escalationContactId: string | null

## ComplianceRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- standardRef: string
- obligation: TextField
- complianceStatus: ValidationStatus
- evidenceRefs: ordered<string>
- auditorId: string | null
- auditDate: string | null
- nextReview: string | null
- affectedEntityIds: ordered<string>
- gapAnalysis: ordered<string>
- remediationPlan: ordered<string>
- ownerId: string | null
- severity: RiskLevel
- regulatoryBody: string | null
- certificationTarget: string | null
- waiverStatus: string | null
- relatedRequirementIds: ordered<string>
- monitoringMethod: string | null
- reportingFrequency: string | null
- penalties: ordered<string>
- correctiveActions: ordered<string>
- artifactRefs: ordered<string>

## ApprovalRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- approvalType: string
- subjectId: string
- approverIds: ordered<string>
- approvalDate: string | null
- conditions: ordered<string>
- approvalStatus: LifecycleStatus
- expiryDate: string | null
- delegationChain: ordered<string>
- evidenceRefs: ordered<string>
- relatedDecisionId: string | null
- relatedChangeId: string | null
- authorityBasis: ordered<string>
- signatureMethod: string | null
- rejectionReason: TextField | null
- resubmissionDate: string | null
- notificationList: ordered<string>
- workflowStep: string | null
- version: string | null
- auditTrailRef: string | null

## MeetingRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- meetingType: string
- scheduledDate: string | null
- duration: string | null
- location: string | null
- chairId: string | null
- attendeeIds: ordered<string>
- agendaItems: ordered<string>
- minutes: TextField | null
- actionItems: ordered<string>
- decisionsMade: ordered<string>
- artifactRefs: ordered<string>
- followUpDate: string | null
- recordingRef: string | null
- quorumMet: boolean
- meetingStatus: LifecycleStatus
- workshopId: string | null
- stakeholderIds: ordered<string>
- requirementIds: ordered<string>
- issueIds: ordered<string>
- approvalIds: ordered<string>

## ChangeRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- changeType: string
- summary: TextField
- reason: TextField
- requestedBy: string | null
- approvedBy: string | null
- changeDate: string | null
- effectiveDate: string | null
- impactedEntityIds: ordered<string>
- beforeSnapshot: string | null
- afterSnapshot: string | null
- costImpact: number | null
- scheduleImpact: string | null
- riskImpact: ordered<string>
- approvalStatus: ValidationStatus
- rollbackPlan: ordered<string>
- communicationPlan: ordered<string>
- versionFrom: string | null
- versionTo: string | null
- auditEventIds: ordered<string>

## CollaborationRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- sessionType: string
- title: string
- participants: ordered<string>
- facilitatorId: string | null
- startTime: string | null
- endTime: string | null
- location: string | null
- agenda: ordered<string>
- outcomes: ordered<string>
- actionItems: ordered<string>
- decisionIds: ordered<string>
- issueIds: ordered<string>
- documentIds: ordered<string>
- recordingRef: string | null
- feedback: ordered<TaggedNote>
- followUpDate: string | null
- workshopId: string | null
- surveyId: string | null

## AnalysisKind

choice("gap"|"conflict"|"dependency"|"capacity"|"demand"|"utilization"|"workflow"|"risk"|"cost"|"scenario"|"sensitivity"|"impact"|"trend"|"requirementComparison"|"requirementClustering"|"requirementFiltering"|"requirementSorting"|"requirementScoring"|"requirementWeighting"|"relationshipAnalysis")

## AnalysisRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- kind: AnalysisKind
- title: string
- parameters: ordered<string>
- inputEntityIds: ordered<string>
- outputSummary: TextField
- findings: ordered<string>
- metrics: ordered<string>
- charts: ordered<string>
- runBy: string | null
- runAt: string | null
- durationMs: number | null
- toolVersion: string | null
- scenarioId: string | null
- reportId: string | null
- confidence: string | null
- limitations: ordered<string>
- recommendations: ordered<string>
- rawResultRef: string | null

## ReportKind

choice("executiveSummary"|"programOverview"|"stakeholderSummary"|"requirementsMatrix"|"adjacencyMatrix"|"gapAnalysis"|"riskRegister"|"decisionLog"|"validationSummary"|"recommendation"|"userSummary"|"functionalSummary"|"capacitySummary"|"workflowSummary"|"complianceSummary"|"costSummary"|"scheduleSummary"|"changeSummary"|"openIssueSummary"|"prioritySummary"|"scenarioSummary")

## ReportRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- kind: ReportKind
- title: string
- audience: ordered<string>
- sections: ordered<string>
- generatedAt: string | null
- generatedBy: string | null
- analysisIds: ordered<string>
- format: string | null
- fileRef: string | null
- distributionList: ordered<string>
- approvalStatus: ValidationStatus
- approverId: string | null
- version: string
- templateId: string | null
- parameters: ordered<string>
- confidentiality: string | null
- expiryDate: string | null
- relatedDecisionIds: ordered<string>

## SearchFilter

Includes every EntityHeader field with this definition's independently authored optionality.

- filterName: string
- filterDescription: TextField | null
- keywords: ordered<string>
- categories: ordered<string>
- ownerIds: ordered<string>
- statuses: ordered<LifecycleStatus>
- priorities: ordered<Priority>
- sources: ordered<string>
- dateFrom: string | null
- dateTo: string | null
- entityKinds: ordered<string>
- tagFilters: ordered<string>
- sortField: string | null
- sortDirection: string | null
- isPublic: boolean
- createdBy: string | null
- lastUsed: string | null
- useCount: number
- pinned: boolean

## StatusRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- subjectId: string
- subjectKind: string
- recordStatus: LifecycleStatus
- previousStatus: LifecycleStatus | null
- changedBy: string | null
- changedAt: string | null
- reason: TextField | null
- blockers: ordered<string>
- nextActions: ordered<string>
- dueDate: string | null
- progressPercent: number | null
- health: string | null
- escalationLevel: string | null
- relatedIssueIds: ordered<string>
- relatedRiskIds: ordered<string>
- milestoneId: string | null
- reportingPeriod: string | null
- statusNotes: ordered<TaggedNote>

## Workshop

Includes every EntityHeader field with this definition's independently authored optionality.

- workshopType: string
- objectives: ordered<string>
- agenda: ordered<string>
- facilitatorId: string | null
- participants: ordered<string>
- scheduledStart: string | null
- scheduledEnd: string | null
- location: string | null
- materials: ordered<string>
- methods: ordered<string>
- outputs: ordered<string>
- decisions: ordered<string>
- issues: ordered<string>
- followUpActions: ordered<string>
- feedback: ordered<TaggedNote>
- recordingRef: string | null
- budget: number | null
- workshopStatus: LifecycleStatus
- surveyIds: ordered<string>

## Survey

Includes every EntityHeader field with this definition's independently authored optionality.

- surveyType: string
- title: string
- objectives: ordered<string>
- questions: ordered<string>
- targetAudience: ordered<string>
- distributionChannels: ordered<string>
- launchDate: string | null
- closeDate: string | null
- responseCount: number
- responseRate: number | null
- findings: ordered<string>
- themes: ordered<string>
- recommendations: ordered<string>
- confidentiality: string | null
- consentProcess: ordered<string>
- analysisId: string | null
- workshopId: string | null
- ownerId: string | null
- surveyStatus: LifecycleStatus

## Issue

Includes every EntityHeader field with this definition's independently authored optionality.

- issueType: string
- summary: TextField
- issueDescription: TextField
- severity: IssueSeverity
- issuePriority: Priority
- reporterId: string | null
- assigneeId: string | null
- affectedEntityIds: ordered<string>
- rootCause: TextField | null
- resolution: TextField | null
- workaround: TextField | null
- dueDate: string | null
- resolvedDate: string | null
- relatedConflictIds: ordered<string>
- relatedRiskIds: ordered<string>
- decisionId: string | null
- comments: ordered<TaggedNote>
- attachments: ordered<string>
- escalationLevel: string | null

## AuditAction

choice("created"|"updated"|"deleted"|"reviewed"|"approved"|"rejected"|"exported"|"imported"|"merged"|"archived")

## AuditEvent

Includes every EntityHeader field with this definition's independently authored optionality.

- action: AuditAction
- actorId: string | null
- subjectId: string
- subjectKind: string
- timestamp: string
- details: TextField
- beforeState: string | null
- afterState: string | null
- ipAddress: string | null
- client: string | null
- sessionId: string | null
- changeRecordId: string | null
- traceLink: TraceLink | null
- success: boolean
- errorMessage: string | null
- correlationId: string | null
- complianceTags: ordered<string>
- retentionUntil: string | null

## TemplateRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- templateType: string
- sector: string | null
- projectType: string | null
- version: string
- contentRef: string | null
- entityKinds: ordered<string>
- defaultFields: ordered<string>
- checklists: ordered<string>
- standards: ordered<string>
- applicability: ordered<string>
- authorId: string | null
- approvalStatus: ValidationStatus
- usageCount: number
- lastApplied: string | null
- customizationNotes: ordered<string>
- relatedKnowledgeIds: ordered<string>
- benchmarkIds: ordered<string>
- license: string | null
- sourceOrganization: string | null

## KnowledgeRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- topic: string
- category: string
- summary: TextField
- content: TextField
- sources: ordered<string>
- references: ordered<string>
- lessonsLearned: ordered<string>
- bestPractices: ordered<string>
- applicableSectors: ordered<string>
- relatedEntityKinds: ordered<string>
- authorIds: ordered<string>
- expertiseLevel: string | null
- validationStatus: ValidationStatus
- lastReviewed: string | null
- keywords: ordered<string>
- attachments: ordered<string>
- citations: ordered<string>
- usageCount: number

## BenchmarkRecord

Includes every EntityHeader field with this definition's independently authored optionality.

- benchmarkName: string
- sector: string
- metric: string
- value: number
- unit: string
- sampleSize: number | null
- source: string | null
- collectionYear: number | null
- geography: string | null
- buildingType: string | null
- confidence: string | null
- methodology: string | null
- applicableElementKinds: ordered<string>
- relatedRequirementIds: ordered<string>
- comparisonNotes: ordered<string>
- limitations: ordered<string>
- license: string | null
- knowledgeId: string | null
- lastVerified: string | null

## Governance

- id: string
- framework: string
- roles: ordered<string>
- responsibilities: ordered<string>
- approvalMatrix: ordered<string>
- escalationPaths: ordered<string>
- meetingCadence: ordered<string>
- decisionRights: ordered<string>
- changeControlProcess: ordered<string>
- qualityPolicy: TextField
- riskAppetite: string | null
- complianceObligations: ordered<string>
- auditSchedule: string | null
- documentControl: ordered<string>
- stakeholderEngagementPlan: ordered<string>
- ethicsPolicy: ordered<string>
- dataGovernance: ordered<string>
- ownerId: string | null
- reviewCycle: string | null
- reviewHierarchy: ordered<string>
- policyOwnershipId: string | null
- requirementOwnershipId: string | null
- riskOwnershipId: string | null
- reportingFrequency: string | null
- accountabilityRules: ordered<string>
- exceptionManagement: ordered<string>
- governancePerformance: ordered<string>

