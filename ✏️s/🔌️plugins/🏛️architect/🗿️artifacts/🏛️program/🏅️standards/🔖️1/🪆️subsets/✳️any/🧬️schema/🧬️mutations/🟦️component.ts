/** 🧩 ProgramMutation dispatch union — mirrors 🦀️component.rs's `ProgramMutation` enum. Each
 *  arm's payload interface is declared beside its own triad's 🦠️mutation/component.ts. */
export interface CreateInformationRequirementOp { mutation: "CreateInformationRequirement"; payload: CreateInformationRequirement; }
export interface DeleteInformationRequirementOp { mutation: "DeleteInformationRequirement"; payload: DeleteInformationRequirement; }
export interface RenameInformationRequirementOp { mutation: "RenameInformationRequirement"; payload: RenameInformationRequirement; }
export interface ReplaceInformationRequirementOp { mutation: "ReplaceInformationRequirement"; payload: ReplaceInformationRequirement; }
export interface CreateSustainabilityRequirementOp { mutation: "CreateSustainabilityRequirement"; payload: CreateSustainabilityRequirement; }
export interface DeleteSustainabilityRequirementOp { mutation: "DeleteSustainabilityRequirement"; payload: DeleteSustainabilityRequirement; }
export interface RenameSustainabilityRequirementOp { mutation: "RenameSustainabilityRequirement"; payload: RenameSustainabilityRequirement; }
export interface ReplaceSustainabilityRequirementOp { mutation: "ReplaceSustainabilityRequirement"; payload: ReplaceSustainabilityRequirement; }
export interface CreateAccessibilityRequirementOp { mutation: "CreateAccessibilityRequirement"; payload: CreateAccessibilityRequirement; }
export interface DeleteAccessibilityRequirementOp { mutation: "DeleteAccessibilityRequirement"; payload: DeleteAccessibilityRequirement; }
export interface RenameAccessibilityRequirementOp { mutation: "RenameAccessibilityRequirement"; payload: RenameAccessibilityRequirement; }
export interface ReplaceAccessibilityRequirementOp { mutation: "ReplaceAccessibilityRequirement"; payload: ReplaceAccessibilityRequirement; }
export interface CreateConflictOp { mutation: "CreateConflict"; payload: CreateConflict; }
export interface DeleteConflictOp { mutation: "DeleteConflict"; payload: DeleteConflict; }
export interface RenameConflictOp { mutation: "RenameConflict"; payload: RenameConflict; }
export interface ReplaceConflictOp { mutation: "ReplaceConflict"; payload: ReplaceConflict; }
export interface CreateOptionEvaluationOp { mutation: "CreateOptionEvaluation"; payload: CreateOptionEvaluation; }
export interface DeleteOptionEvaluationOp { mutation: "DeleteOptionEvaluation"; payload: DeleteOptionEvaluation; }
export interface RenameOptionEvaluationOp { mutation: "RenameOptionEvaluation"; payload: RenameOptionEvaluation; }
export interface ReplaceOptionEvaluationOp { mutation: "ReplaceOptionEvaluation"; payload: ReplaceOptionEvaluation; }
export interface CreateFunctionOp { mutation: "CreateFunction"; payload: CreateFunction; }
export interface DeleteFunctionOp { mutation: "DeleteFunction"; payload: DeleteFunction; }
export interface RenameFunctionOp { mutation: "RenameFunction"; payload: RenameFunction; }
export interface ReplaceFunctionOp { mutation: "ReplaceFunction"; payload: ReplaceFunction; }
export interface CreateRiskOp { mutation: "CreateRisk"; payload: CreateRisk; }
export interface DeleteRiskOp { mutation: "DeleteRisk"; payload: DeleteRisk; }
export interface RenameRiskOp { mutation: "RenameRisk"; payload: RenameRisk; }
export interface ReplaceRiskOp { mutation: "ReplaceRisk"; payload: ReplaceRisk; }
export interface CreateDecisionOp { mutation: "CreateDecision"; payload: CreateDecision; }
export interface DeleteDecisionOp { mutation: "DeleteDecision"; payload: DeleteDecision; }
export interface RenameDecisionOp { mutation: "RenameDecision"; payload: RenameDecision; }
export interface ReplaceDecisionOp { mutation: "ReplaceDecision"; payload: ReplaceDecision; }
export interface CreateValidationRecordOp { mutation: "CreateValidationRecord"; payload: CreateValidationRecord; }
export interface DeleteValidationRecordOp { mutation: "DeleteValidationRecord"; payload: DeleteValidationRecord; }
export interface RenameValidationRecordOp { mutation: "RenameValidationRecord"; payload: RenameValidationRecord; }
export interface ReplaceValidationRecordOp { mutation: "ReplaceValidationRecord"; payload: ReplaceValidationRecord; }
export interface CreatePriorityRecordOp { mutation: "CreatePriorityRecord"; payload: CreatePriorityRecord; }
export interface DeletePriorityRecordOp { mutation: "DeletePriorityRecord"; payload: DeletePriorityRecord; }
export interface RenamePriorityRecordOp { mutation: "RenamePriorityRecord"; payload: RenamePriorityRecord; }
export interface ReplacePriorityRecordOp { mutation: "ReplacePriorityRecord"; payload: ReplacePriorityRecord; }
export interface CreateFlowRequirementOp { mutation: "CreateFlowRequirement"; payload: CreateFlowRequirement; }
export interface DeleteFlowRequirementOp { mutation: "DeleteFlowRequirement"; payload: DeleteFlowRequirement; }
export interface RenameFlowRequirementOp { mutation: "RenameFlowRequirement"; payload: RenameFlowRequirement; }
export interface ReplaceFlowRequirementOp { mutation: "ReplaceFlowRequirement"; payload: ReplaceFlowRequirement; }
export interface CreateEnvironmentalRequirementOp { mutation: "CreateEnvironmentalRequirement"; payload: CreateEnvironmentalRequirement; }
export interface DeleteEnvironmentalRequirementOp { mutation: "DeleteEnvironmentalRequirement"; payload: DeleteEnvironmentalRequirement; }
export interface RenameEnvironmentalRequirementOp { mutation: "RenameEnvironmentalRequirement"; payload: RenameEnvironmentalRequirement; }
export interface ReplaceEnvironmentalRequirementOp { mutation: "ReplaceEnvironmentalRequirement"; payload: ReplaceEnvironmentalRequirement; }
export interface CreateWorkshopOp { mutation: "CreateWorkshop"; payload: CreateWorkshop; }
export interface DeleteWorkshopOp { mutation: "DeleteWorkshop"; payload: DeleteWorkshop; }
export interface RenameWorkshopOp { mutation: "RenameWorkshop"; payload: RenameWorkshop; }
export interface ReplaceWorkshopOp { mutation: "ReplaceWorkshop"; payload: ReplaceWorkshop; }
export interface CreateScenarioOp { mutation: "CreateScenario"; payload: CreateScenario; }
export interface DeleteScenarioOp { mutation: "DeleteScenario"; payload: DeleteScenario; }
export interface RenameScenarioOp { mutation: "RenameScenario"; payload: RenameScenario; }
export interface ReplaceScenarioOp { mutation: "ReplaceScenario"; payload: ReplaceScenario; }
export interface CreateBenchmarkRecordOp { mutation: "CreateBenchmarkRecord"; payload: CreateBenchmarkRecord; }
export interface DeleteBenchmarkRecordOp { mutation: "DeleteBenchmarkRecord"; payload: DeleteBenchmarkRecord; }
export interface RenameBenchmarkRecordOp { mutation: "RenameBenchmarkRecord"; payload: RenameBenchmarkRecord; }
export interface ReplaceBenchmarkRecordOp { mutation: "ReplaceBenchmarkRecord"; payload: ReplaceBenchmarkRecord; }
export interface CreateActivityOp { mutation: "CreateActivity"; payload: CreateActivity; }
export interface DeleteActivityOp { mutation: "DeleteActivity"; payload: DeleteActivity; }
export interface RenameActivityOp { mutation: "RenameActivity"; payload: RenameActivity; }
export interface ReplaceActivityOp { mutation: "ReplaceActivity"; payload: ReplaceActivity; }
export interface CreateInfrastructureRequirementOp { mutation: "CreateInfrastructureRequirement"; payload: CreateInfrastructureRequirement; }
export interface DeleteInfrastructureRequirementOp { mutation: "DeleteInfrastructureRequirement"; payload: DeleteInfrastructureRequirement; }
export interface RenameInfrastructureRequirementOp { mutation: "RenameInfrastructureRequirement"; payload: RenameInfrastructureRequirement; }
export interface ReplaceInfrastructureRequirementOp { mutation: "ReplaceInfrastructureRequirement"; payload: ReplaceInfrastructureRequirement; }
export interface CreateOrganizationalRequirementOp { mutation: "CreateOrganizationalRequirement"; payload: CreateOrganizationalRequirement; }
export interface DeleteOrganizationalRequirementOp { mutation: "DeleteOrganizationalRequirement"; payload: DeleteOrganizationalRequirement; }
export interface RenameOrganizationalRequirementOp { mutation: "RenameOrganizationalRequirement"; payload: RenameOrganizationalRequirement; }
export interface ReplaceOrganizationalRequirementOp { mutation: "ReplaceOrganizationalRequirement"; payload: ReplaceOrganizationalRequirement; }
export interface CreateIssueOp { mutation: "CreateIssue"; payload: CreateIssue; }
export interface DeleteIssueOp { mutation: "DeleteIssue"; payload: DeleteIssue; }
export interface RenameIssueOp { mutation: "RenameIssue"; payload: RenameIssue; }
export interface ReplaceIssueOp { mutation: "ReplaceIssue"; payload: ReplaceIssue; }
export interface CreateApprovalRecordOp { mutation: "CreateApprovalRecord"; payload: CreateApprovalRecord; }
export interface DeleteApprovalRecordOp { mutation: "DeleteApprovalRecord"; payload: DeleteApprovalRecord; }
export interface RenameApprovalRecordOp { mutation: "RenameApprovalRecord"; payload: RenameApprovalRecord; }
export interface ReplaceApprovalRecordOp { mutation: "ReplaceApprovalRecord"; payload: ReplaceApprovalRecord; }
export interface CreateStakeholderOp { mutation: "CreateStakeholder"; payload: CreateStakeholder; }
export interface DeleteStakeholderOp { mutation: "DeleteStakeholder"; payload: DeleteStakeholder; }
export interface RenameStakeholderOp { mutation: "RenameStakeholder"; payload: RenameStakeholder; }
export interface ReplaceStakeholderOp { mutation: "ReplaceStakeholder"; payload: ReplaceStakeholder; }
export interface CreateQualityRecordOp { mutation: "CreateQualityRecord"; payload: CreateQualityRecord; }
export interface DeleteQualityRecordOp { mutation: "DeleteQualityRecord"; payload: DeleteQualityRecord; }
export interface RenameQualityRecordOp { mutation: "RenameQualityRecord"; payload: RenameQualityRecord; }
export interface ReplaceQualityRecordOp { mutation: "ReplaceQualityRecord"; payload: ReplaceQualityRecord; }
export interface CreateResilienceRequirementOp { mutation: "CreateResilienceRequirement"; payload: CreateResilienceRequirement; }
export interface DeleteResilienceRequirementOp { mutation: "DeleteResilienceRequirement"; payload: DeleteResilienceRequirement; }
export interface RenameResilienceRequirementOp { mutation: "RenameResilienceRequirement"; payload: RenameResilienceRequirement; }
export interface ReplaceResilienceRequirementOp { mutation: "ReplaceResilienceRequirement"; payload: ReplaceResilienceRequirement; }
export interface CreateAssumptionOp { mutation: "CreateAssumption"; payload: CreateAssumption; }
export interface DeleteAssumptionOp { mutation: "DeleteAssumption"; payload: DeleteAssumption; }
export interface RenameAssumptionOp { mutation: "RenameAssumption"; payload: RenameAssumption; }
export interface ReplaceAssumptionOp { mutation: "ReplaceAssumption"; payload: ReplaceAssumption; }
export interface CreateCostRequirementOp { mutation: "CreateCostRequirement"; payload: CreateCostRequirement; }
export interface DeleteCostRequirementOp { mutation: "DeleteCostRequirement"; payload: DeleteCostRequirement; }
export interface RenameCostRequirementOp { mutation: "RenameCostRequirement"; payload: RenameCostRequirement; }
export interface ReplaceCostRequirementOp { mutation: "ReplaceCostRequirement"; payload: ReplaceCostRequirement; }
export interface CreateDocumentOp { mutation: "CreateDocument"; payload: CreateDocument; }
export interface DeleteDocumentOp { mutation: "DeleteDocument"; payload: DeleteDocument; }
export interface RenameDocumentOp { mutation: "RenameDocument"; payload: RenameDocument; }
export interface ReplaceDocumentOp { mutation: "ReplaceDocument"; payload: ReplaceDocument; }
export interface CreateScheduleRequirementOp { mutation: "CreateScheduleRequirement"; payload: CreateScheduleRequirement; }
export interface DeleteScheduleRequirementOp { mutation: "DeleteScheduleRequirement"; payload: DeleteScheduleRequirement; }
export interface RenameScheduleRequirementOp { mutation: "RenameScheduleRequirement"; payload: RenameScheduleRequirement; }
export interface ReplaceScheduleRequirementOp { mutation: "ReplaceScheduleRequirement"; payload: ReplaceScheduleRequirement; }
export interface CreateGrowthPlanOp { mutation: "CreateGrowthPlan"; payload: CreateGrowthPlan; }
export interface DeleteGrowthPlanOp { mutation: "DeleteGrowthPlan"; payload: DeleteGrowthPlan; }
export interface RenameGrowthPlanOp { mutation: "RenameGrowthPlan"; payload: RenameGrowthPlan; }
export interface ReplaceGrowthPlanOp { mutation: "ReplaceGrowthPlan"; payload: ReplaceGrowthPlan; }
export interface CreatePerformanceCriterionOp { mutation: "CreatePerformanceCriterion"; payload: CreatePerformanceCriterion; }
export interface DeletePerformanceCriterionOp { mutation: "DeletePerformanceCriterion"; payload: DeletePerformanceCriterion; }
export interface RenamePerformanceCriterionOp { mutation: "RenamePerformanceCriterion"; payload: RenamePerformanceCriterion; }
export interface ReplacePerformanceCriterionOp { mutation: "ReplacePerformanceCriterion"; payload: ReplacePerformanceCriterion; }
export interface CreateOperationalRequirementOp { mutation: "CreateOperationalRequirement"; payload: CreateOperationalRequirement; }
export interface DeleteOperationalRequirementOp { mutation: "DeleteOperationalRequirement"; payload: DeleteOperationalRequirement; }
export interface RenameOperationalRequirementOp { mutation: "RenameOperationalRequirement"; payload: RenameOperationalRequirement; }
export interface ReplaceOperationalRequirementOp { mutation: "ReplaceOperationalRequirement"; payload: ReplaceOperationalRequirement; }
export interface CreateRequirementOp { mutation: "CreateRequirement"; payload: CreateRequirement; }
export interface DeleteRequirementOp { mutation: "DeleteRequirement"; payload: DeleteRequirement; }
export interface RenameRequirementOp { mutation: "RenameRequirement"; payload: RenameRequirement; }
export interface ReplaceRequirementOp { mutation: "ReplaceRequirement"; payload: ReplaceRequirement; }
export interface CreateSiteContextOp { mutation: "CreateSiteContext"; payload: CreateSiteContext; }
export interface DeleteSiteContextOp { mutation: "DeleteSiteContext"; payload: DeleteSiteContext; }
export interface RenameSiteContextOp { mutation: "RenameSiteContext"; payload: RenameSiteContext; }
export interface ReplaceSiteContextOp { mutation: "ReplaceSiteContext"; payload: ReplaceSiteContext; }
export interface CreateTemplateRecordOp { mutation: "CreateTemplateRecord"; payload: CreateTemplateRecord; }
export interface DeleteTemplateRecordOp { mutation: "DeleteTemplateRecord"; payload: DeleteTemplateRecord; }
export interface RenameTemplateRecordOp { mutation: "RenameTemplateRecord"; payload: RenameTemplateRecord; }
export interface ReplaceTemplateRecordOp { mutation: "ReplaceTemplateRecord"; payload: ReplaceTemplateRecord; }
export interface CreateReportRecordOp { mutation: "CreateReportRecord"; payload: CreateReportRecord; }
export interface DeleteReportRecordOp { mutation: "DeleteReportRecord"; payload: DeleteReportRecord; }
export interface RenameReportRecordOp { mutation: "RenameReportRecord"; payload: RenameReportRecord; }
export interface ReplaceReportRecordOp { mutation: "ReplaceReportRecord"; payload: ReplaceReportRecord; }
export interface CreateAuditEventOp { mutation: "CreateAuditEvent"; payload: CreateAuditEvent; }
export interface DeleteAuditEventOp { mutation: "DeleteAuditEvent"; payload: DeleteAuditEvent; }
export interface RenameAuditEventOp { mutation: "RenameAuditEvent"; payload: RenameAuditEvent; }
export interface ReplaceAuditEventOp { mutation: "ReplaceAuditEvent"; payload: ReplaceAuditEvent; }
export interface CreateKnowledgeRecordOp { mutation: "CreateKnowledgeRecord"; payload: CreateKnowledgeRecord; }
export interface DeleteKnowledgeRecordOp { mutation: "DeleteKnowledgeRecord"; payload: DeleteKnowledgeRecord; }
export interface RenameKnowledgeRecordOp { mutation: "RenameKnowledgeRecord"; payload: RenameKnowledgeRecord; }
export interface ReplaceKnowledgeRecordOp { mutation: "ReplaceKnowledgeRecord"; payload: ReplaceKnowledgeRecord; }
export interface CreateRegulatoryRequirementOp { mutation: "CreateRegulatoryRequirement"; payload: CreateRegulatoryRequirement; }
export interface DeleteRegulatoryRequirementOp { mutation: "DeleteRegulatoryRequirement"; payload: DeleteRegulatoryRequirement; }
export interface RenameRegulatoryRequirementOp { mutation: "RenameRegulatoryRequirement"; payload: RenameRegulatoryRequirement; }
export interface ReplaceRegulatoryRequirementOp { mutation: "ReplaceRegulatoryRequirement"; payload: ReplaceRegulatoryRequirement; }
export interface CreateChangeRecordOp { mutation: "CreateChangeRecord"; payload: CreateChangeRecord; }
export interface DeleteChangeRecordOp { mutation: "DeleteChangeRecord"; payload: DeleteChangeRecord; }
export interface RenameChangeRecordOp { mutation: "RenameChangeRecord"; payload: RenameChangeRecord; }
export interface ReplaceChangeRecordOp { mutation: "ReplaceChangeRecord"; payload: ReplaceChangeRecord; }
export interface CreateCommunicationRequirementOp { mutation: "CreateCommunicationRequirement"; payload: CreateCommunicationRequirement; }
export interface DeleteCommunicationRequirementOp { mutation: "DeleteCommunicationRequirement"; payload: DeleteCommunicationRequirement; }
export interface RenameCommunicationRequirementOp { mutation: "RenameCommunicationRequirement"; payload: RenameCommunicationRequirement; }
export interface ReplaceCommunicationRequirementOp { mutation: "ReplaceCommunicationRequirement"; payload: ReplaceCommunicationRequirement; }
export interface CreateResourceOp { mutation: "CreateResource"; payload: CreateResource; }
export interface DeleteResourceOp { mutation: "DeleteResource"; payload: DeleteResource; }
export interface RenameResourceOp { mutation: "RenameResource"; payload: RenameResource; }
export interface ReplaceResourceOp { mutation: "ReplaceResource"; payload: ReplaceResource; }
export interface CreateStatusRecordOp { mutation: "CreateStatusRecord"; payload: CreateStatusRecord; }
export interface DeleteStatusRecordOp { mutation: "DeleteStatusRecord"; payload: DeleteStatusRecord; }
export interface RenameStatusRecordOp { mutation: "RenameStatusRecord"; payload: RenameStatusRecord; }
export interface ReplaceStatusRecordOp { mutation: "ReplaceStatusRecord"; payload: ReplaceStatusRecord; }
export interface CreateProcessOp { mutation: "CreateProcess"; payload: CreateProcess; }
export interface DeleteProcessOp { mutation: "DeleteProcess"; payload: DeleteProcess; }
export interface RenameProcessOp { mutation: "RenameProcess"; payload: RenameProcess; }
export interface ReplaceProcessOp { mutation: "ReplaceProcess"; payload: ReplaceProcess; }
export interface CreateSearchFilterOp { mutation: "CreateSearchFilter"; payload: CreateSearchFilter; }
export interface DeleteSearchFilterOp { mutation: "DeleteSearchFilter"; payload: DeleteSearchFilter; }
export interface RenameSearchFilterOp { mutation: "RenameSearchFilter"; payload: RenameSearchFilter; }
export interface ReplaceSearchFilterOp { mutation: "ReplaceSearchFilter"; payload: ReplaceSearchFilter; }
export interface CreateAccessRuleOp { mutation: "CreateAccessRule"; payload: CreateAccessRule; }
export interface DeleteAccessRuleOp { mutation: "DeleteAccessRule"; payload: DeleteAccessRule; }
export interface RenameAccessRuleOp { mutation: "RenameAccessRule"; payload: RenameAccessRule; }
export interface ReplaceAccessRuleOp { mutation: "ReplaceAccessRule"; payload: ReplaceAccessRule; }
export interface CreatePrivacyRequirementOp { mutation: "CreatePrivacyRequirement"; payload: CreatePrivacyRequirement; }
export interface DeletePrivacyRequirementOp { mutation: "DeletePrivacyRequirement"; payload: DeletePrivacyRequirement; }
export interface RenamePrivacyRequirementOp { mutation: "RenamePrivacyRequirement"; payload: RenamePrivacyRequirement; }
export interface ReplacePrivacyRequirementOp { mutation: "ReplacePrivacyRequirement"; payload: ReplacePrivacyRequirement; }
export interface CreateRelationshipOp { mutation: "CreateRelationship"; payload: CreateRelationship; }
export interface DeleteRelationshipOp { mutation: "DeleteRelationship"; payload: DeleteRelationship; }
export interface RenameRelationshipOp { mutation: "RenameRelationship"; payload: RenameRelationship; }
export interface ReplaceRelationshipOp { mutation: "ReplaceRelationship"; payload: ReplaceRelationship; }
export interface CreateQuantityRequirementOp { mutation: "CreateQuantityRequirement"; payload: CreateQuantityRequirement; }
export interface DeleteQuantityRequirementOp { mutation: "DeleteQuantityRequirement"; payload: DeleteQuantityRequirement; }
export interface RenameQuantityRequirementOp { mutation: "RenameQuantityRequirement"; payload: RenameQuantityRequirement; }
export interface ReplaceQuantityRequirementOp { mutation: "ReplaceQuantityRequirement"; payload: ReplaceQuantityRequirement; }
export interface CreateAnalysisRecordOp { mutation: "CreateAnalysisRecord"; payload: CreateAnalysisRecord; }
export interface DeleteAnalysisRecordOp { mutation: "DeleteAnalysisRecord"; payload: DeleteAnalysisRecord; }
export interface RenameAnalysisRecordOp { mutation: "RenameAnalysisRecord"; payload: RenameAnalysisRecord; }
export interface ReplaceAnalysisRecordOp { mutation: "ReplaceAnalysisRecord"; payload: ReplaceAnalysisRecord; }
export interface CreateStorageRequirementOp { mutation: "CreateStorageRequirement"; payload: CreateStorageRequirement; }
export interface DeleteStorageRequirementOp { mutation: "DeleteStorageRequirement"; payload: DeleteStorageRequirement; }
export interface RenameStorageRequirementOp { mutation: "RenameStorageRequirement"; payload: RenameStorageRequirement; }
export interface ReplaceStorageRequirementOp { mutation: "ReplaceStorageRequirement"; payload: ReplaceStorageRequirement; }
export interface CreateMeetingRecordOp { mutation: "CreateMeetingRecord"; payload: CreateMeetingRecord; }
export interface DeleteMeetingRecordOp { mutation: "DeleteMeetingRecord"; payload: DeleteMeetingRecord; }
export interface RenameMeetingRecordOp { mutation: "RenameMeetingRecord"; payload: RenameMeetingRecord; }
export interface ReplaceMeetingRecordOp { mutation: "ReplaceMeetingRecord"; payload: ReplaceMeetingRecord; }
export interface CreateSurveyOp { mutation: "CreateSurvey"; payload: CreateSurvey; }
export interface DeleteSurveyOp { mutation: "DeleteSurvey"; payload: DeleteSurvey; }
export interface RenameSurveyOp { mutation: "RenameSurvey"; payload: RenameSurvey; }
export interface ReplaceSurveyOp { mutation: "ReplaceSurvey"; payload: ReplaceSurvey; }
export interface CreateDeliveryConstraintOp { mutation: "CreateDeliveryConstraint"; payload: CreateDeliveryConstraint; }
export interface DeleteDeliveryConstraintOp { mutation: "DeleteDeliveryConstraint"; payload: DeleteDeliveryConstraint; }
export interface RenameDeliveryConstraintOp { mutation: "RenameDeliveryConstraint"; payload: RenameDeliveryConstraint; }
export interface ReplaceDeliveryConstraintOp { mutation: "ReplaceDeliveryConstraint"; payload: ReplaceDeliveryConstraint; }
export interface CreateConstraintRecordOp { mutation: "CreateConstraintRecord"; payload: CreateConstraintRecord; }
export interface DeleteConstraintRecordOp { mutation: "DeleteConstraintRecord"; payload: DeleteConstraintRecord; }
export interface RenameConstraintRecordOp { mutation: "RenameConstraintRecord"; payload: RenameConstraintRecord; }
export interface ReplaceConstraintRecordOp { mutation: "ReplaceConstraintRecord"; payload: ReplaceConstraintRecord; }
export interface CreateComplianceRecordOp { mutation: "CreateComplianceRecord"; payload: CreateComplianceRecord; }
export interface DeleteComplianceRecordOp { mutation: "DeleteComplianceRecord"; payload: DeleteComplianceRecord; }
export interface RenameComplianceRecordOp { mutation: "RenameComplianceRecord"; payload: RenameComplianceRecord; }
export interface ReplaceComplianceRecordOp { mutation: "ReplaceComplianceRecord"; payload: ReplaceComplianceRecord; }
export interface CreateServiceRequirementOp { mutation: "CreateServiceRequirement"; payload: CreateServiceRequirement; }
export interface DeleteServiceRequirementOp { mutation: "DeleteServiceRequirement"; payload: DeleteServiceRequirement; }
export interface RenameServiceRequirementOp { mutation: "RenameServiceRequirement"; payload: RenameServiceRequirement; }
export interface ReplaceServiceRequirementOp { mutation: "ReplaceServiceRequirement"; payload: ReplaceServiceRequirement; }
export interface CreateEquipmentOp { mutation: "CreateEquipment"; payload: CreateEquipment; }
export interface DeleteEquipmentOp { mutation: "DeleteEquipment"; payload: DeleteEquipment; }
export interface RenameEquipmentOp { mutation: "RenameEquipment"; payload: RenameEquipment; }
export interface ReplaceEquipmentOp { mutation: "ReplaceEquipment"; payload: ReplaceEquipment; }
export interface CreateSecurityRequirementOp { mutation: "CreateSecurityRequirement"; payload: CreateSecurityRequirement; }
export interface DeleteSecurityRequirementOp { mutation: "DeleteSecurityRequirement"; payload: DeleteSecurityRequirement; }
export interface RenameSecurityRequirementOp { mutation: "RenameSecurityRequirement"; payload: RenameSecurityRequirement; }
export interface ReplaceSecurityRequirementOp { mutation: "ReplaceSecurityRequirement"; payload: ReplaceSecurityRequirement; }
export interface CreateCollaborationRecordOp { mutation: "CreateCollaborationRecord"; payload: CreateCollaborationRecord; }
export interface DeleteCollaborationRecordOp { mutation: "DeleteCollaborationRecord"; payload: DeleteCollaborationRecord; }
export interface RenameCollaborationRecordOp { mutation: "RenameCollaborationRecord"; payload: RenameCollaborationRecord; }
export interface ReplaceCollaborationRecordOp { mutation: "ReplaceCollaborationRecord"; payload: ReplaceCollaborationRecord; }
export interface CreateSafetyRequirementOp { mutation: "CreateSafetyRequirement"; payload: CreateSafetyRequirement; }
export interface DeleteSafetyRequirementOp { mutation: "DeleteSafetyRequirement"; payload: DeleteSafetyRequirement; }
export interface RenameSafetyRequirementOp { mutation: "RenameSafetyRequirement"; payload: RenameSafetyRequirement; }
export interface ReplaceSafetyRequirementOp { mutation: "ReplaceSafetyRequirement"; payload: ReplaceSafetyRequirement; }
export interface CreateUserProfileOp { mutation: "CreateUserProfile"; payload: CreateUserProfile; }
export interface DeleteUserProfileOp { mutation: "DeleteUserProfile"; payload: DeleteUserProfile; }
export interface RenameUserProfileOp { mutation: "RenameUserProfile"; payload: RenameUserProfile; }
export interface ReplaceUserProfileOp { mutation: "ReplaceUserProfile"; payload: ReplaceUserProfile; }
export interface CreateHumanFactorRequirementOp { mutation: "CreateHumanFactorRequirement"; payload: CreateHumanFactorRequirement; }
export interface DeleteHumanFactorRequirementOp { mutation: "DeleteHumanFactorRequirement"; payload: DeleteHumanFactorRequirement; }
export interface RenameHumanFactorRequirementOp { mutation: "RenameHumanFactorRequirement"; payload: RenameHumanFactorRequirement; }
export interface ReplaceHumanFactorRequirementOp { mutation: "ReplaceHumanFactorRequirement"; payload: ReplaceHumanFactorRequirement; }
export interface CreateFlexibilityRequirementOp { mutation: "CreateFlexibilityRequirement"; payload: CreateFlexibilityRequirement; }
export interface DeleteFlexibilityRequirementOp { mutation: "DeleteFlexibilityRequirement"; payload: DeleteFlexibilityRequirement; }
export interface RenameFlexibilityRequirementOp { mutation: "RenameFlexibilityRequirement"; payload: RenameFlexibilityRequirement; }
export interface ReplaceFlexibilityRequirementOp { mutation: "ReplaceFlexibilityRequirement"; payload: ReplaceFlexibilityRequirement; }
export interface CreateWayfindingRequirementOp { mutation: "CreateWayfindingRequirement"; payload: CreateWayfindingRequirement; }
export interface DeleteWayfindingRequirementOp { mutation: "DeleteWayfindingRequirement"; payload: DeleteWayfindingRequirement; }
export interface RenameWayfindingRequirementOp { mutation: "RenameWayfindingRequirement"; payload: RenameWayfindingRequirement; }
export interface ReplaceWayfindingRequirementOp { mutation: "ReplaceWayfindingRequirement"; payload: ReplaceWayfindingRequirement; }
export interface CreateProgramElementOp { mutation: "CreateProgramElement"; payload: CreateProgramElement; }
export interface DeleteProgramElementOp { mutation: "DeleteProgramElement"; payload: DeleteProgramElement; }
export interface RenameProgramElementOp { mutation: "RenameProgramElement"; payload: RenameProgramElement; }
export interface ReplaceProgramElementOp { mutation: "ReplaceProgramElement"; payload: ReplaceProgramElement; }
export interface ConnectAdjacencyOp { mutation: "ConnectAdjacency"; payload: ConnectAdjacency; }
export interface DisconnectAdjacencyOp { mutation: "DisconnectAdjacency"; payload: DisconnectAdjacency; }
export interface ConnectTraceOp { mutation: "ConnectTrace"; payload: ConnectTrace; }
export interface DisconnectTraceOp { mutation: "DisconnectTrace"; payload: DisconnectTrace; }
export interface RenameMetaOp { mutation: "RenameMeta"; payload: RenameMeta; }
export interface ReplaceMetaOp { mutation: "ReplaceMeta"; payload: ReplaceMeta; }
export interface RenameProjectOp { mutation: "RenameProject"; payload: RenameProject; }
export interface ReplaceProjectOp { mutation: "ReplaceProject"; payload: ReplaceProject; }
export interface RenameGovernanceOp { mutation: "RenameGovernance"; payload: RenameGovernance; }
export interface ReplaceGovernanceOp { mutation: "ReplaceGovernance"; payload: ReplaceGovernance; }

export type ProgramMutation = CreateInformationRequirementOp | DeleteInformationRequirementOp |
   RenameInformationRequirementOp | ReplaceInformationRequirementOp | CreateSustainabilityRequirementOp |
   DeleteSustainabilityRequirementOp | RenameSustainabilityRequirementOp | ReplaceSustainabilityRequirementOp |
   CreateAccessibilityRequirementOp | DeleteAccessibilityRequirementOp | RenameAccessibilityRequirementOp |
   ReplaceAccessibilityRequirementOp | CreateConflictOp | DeleteConflictOp | RenameConflictOp | ReplaceConflictOp |
   CreateOptionEvaluationOp | DeleteOptionEvaluationOp | RenameOptionEvaluationOp | ReplaceOptionEvaluationOp |
   CreateFunctionOp | DeleteFunctionOp | RenameFunctionOp | ReplaceFunctionOp | CreateRiskOp | DeleteRiskOp |
   RenameRiskOp | ReplaceRiskOp | CreateDecisionOp | DeleteDecisionOp | RenameDecisionOp | ReplaceDecisionOp |
   CreateValidationRecordOp | DeleteValidationRecordOp | RenameValidationRecordOp | ReplaceValidationRecordOp |
   CreatePriorityRecordOp | DeletePriorityRecordOp | RenamePriorityRecordOp | ReplacePriorityRecordOp |
   CreateFlowRequirementOp | DeleteFlowRequirementOp | RenameFlowRequirementOp | ReplaceFlowRequirementOp |
   CreateEnvironmentalRequirementOp | DeleteEnvironmentalRequirementOp | RenameEnvironmentalRequirementOp |
   ReplaceEnvironmentalRequirementOp | CreateWorkshopOp | DeleteWorkshopOp | RenameWorkshopOp | ReplaceWorkshopOp |
   CreateScenarioOp | DeleteScenarioOp | RenameScenarioOp | ReplaceScenarioOp | CreateBenchmarkRecordOp |
   DeleteBenchmarkRecordOp | RenameBenchmarkRecordOp | ReplaceBenchmarkRecordOp | CreateActivityOp |
   DeleteActivityOp | RenameActivityOp | ReplaceActivityOp | CreateInfrastructureRequirementOp |
   DeleteInfrastructureRequirementOp | RenameInfrastructureRequirementOp | ReplaceInfrastructureRequirementOp |
   CreateOrganizationalRequirementOp | DeleteOrganizationalRequirementOp | RenameOrganizationalRequirementOp |
   ReplaceOrganizationalRequirementOp | CreateIssueOp | DeleteIssueOp | RenameIssueOp | ReplaceIssueOp |
   CreateApprovalRecordOp | DeleteApprovalRecordOp | RenameApprovalRecordOp | ReplaceApprovalRecordOp |
   CreateStakeholderOp | DeleteStakeholderOp | RenameStakeholderOp | ReplaceStakeholderOp | CreateQualityRecordOp |
   DeleteQualityRecordOp | RenameQualityRecordOp | ReplaceQualityRecordOp | CreateResilienceRequirementOp |
   DeleteResilienceRequirementOp | RenameResilienceRequirementOp | ReplaceResilienceRequirementOp |
   CreateAssumptionOp | DeleteAssumptionOp | RenameAssumptionOp | ReplaceAssumptionOp | CreateCostRequirementOp |
   DeleteCostRequirementOp | RenameCostRequirementOp | ReplaceCostRequirementOp | CreateDocumentOp |
   DeleteDocumentOp | RenameDocumentOp | ReplaceDocumentOp | CreateScheduleRequirementOp |
   DeleteScheduleRequirementOp | RenameScheduleRequirementOp | ReplaceScheduleRequirementOp | CreateGrowthPlanOp |
   DeleteGrowthPlanOp | RenameGrowthPlanOp | ReplaceGrowthPlanOp | CreatePerformanceCriterionOp |
   DeletePerformanceCriterionOp | RenamePerformanceCriterionOp | ReplacePerformanceCriterionOp |
   CreateOperationalRequirementOp | DeleteOperationalRequirementOp | RenameOperationalRequirementOp |
   ReplaceOperationalRequirementOp | CreateRequirementOp | DeleteRequirementOp | RenameRequirementOp |
   ReplaceRequirementOp | CreateSiteContextOp | DeleteSiteContextOp | RenameSiteContextOp | ReplaceSiteContextOp |
   CreateTemplateRecordOp | DeleteTemplateRecordOp | RenameTemplateRecordOp | ReplaceTemplateRecordOp |
   CreateReportRecordOp | DeleteReportRecordOp | RenameReportRecordOp | ReplaceReportRecordOp | CreateAuditEventOp |
   DeleteAuditEventOp | RenameAuditEventOp | ReplaceAuditEventOp | CreateKnowledgeRecordOp | DeleteKnowledgeRecordOp |
   RenameKnowledgeRecordOp | ReplaceKnowledgeRecordOp | CreateRegulatoryRequirementOp |
   DeleteRegulatoryRequirementOp | RenameRegulatoryRequirementOp | ReplaceRegulatoryRequirementOp |
   CreateChangeRecordOp | DeleteChangeRecordOp | RenameChangeRecordOp | ReplaceChangeRecordOp |
   CreateCommunicationRequirementOp | DeleteCommunicationRequirementOp | RenameCommunicationRequirementOp |
   ReplaceCommunicationRequirementOp | CreateResourceOp | DeleteResourceOp | RenameResourceOp | ReplaceResourceOp |
   CreateStatusRecordOp | DeleteStatusRecordOp | RenameStatusRecordOp | ReplaceStatusRecordOp | CreateProcessOp |
   DeleteProcessOp | RenameProcessOp | ReplaceProcessOp | CreateSearchFilterOp | DeleteSearchFilterOp |
   RenameSearchFilterOp | ReplaceSearchFilterOp | CreateAccessRuleOp | DeleteAccessRuleOp | RenameAccessRuleOp |
   ReplaceAccessRuleOp | CreatePrivacyRequirementOp | DeletePrivacyRequirementOp | RenamePrivacyRequirementOp |
   ReplacePrivacyRequirementOp | CreateRelationshipOp | DeleteRelationshipOp | RenameRelationshipOp |
   ReplaceRelationshipOp | CreateQuantityRequirementOp | DeleteQuantityRequirementOp | RenameQuantityRequirementOp |
   ReplaceQuantityRequirementOp | CreateAnalysisRecordOp | DeleteAnalysisRecordOp | RenameAnalysisRecordOp |
   ReplaceAnalysisRecordOp | CreateStorageRequirementOp | DeleteStorageRequirementOp | RenameStorageRequirementOp |
   ReplaceStorageRequirementOp | CreateMeetingRecordOp | DeleteMeetingRecordOp | RenameMeetingRecordOp |
   ReplaceMeetingRecordOp | CreateSurveyOp | DeleteSurveyOp | RenameSurveyOp | ReplaceSurveyOp |
   CreateDeliveryConstraintOp | DeleteDeliveryConstraintOp | RenameDeliveryConstraintOp |
   ReplaceDeliveryConstraintOp | CreateConstraintRecordOp | DeleteConstraintRecordOp | RenameConstraintRecordOp |
   ReplaceConstraintRecordOp | CreateComplianceRecordOp | DeleteComplianceRecordOp | RenameComplianceRecordOp |
   ReplaceComplianceRecordOp | CreateServiceRequirementOp | DeleteServiceRequirementOp | RenameServiceRequirementOp |
   ReplaceServiceRequirementOp | CreateEquipmentOp | DeleteEquipmentOp | RenameEquipmentOp | ReplaceEquipmentOp |
   CreateSecurityRequirementOp | DeleteSecurityRequirementOp | RenameSecurityRequirementOp |
   ReplaceSecurityRequirementOp | CreateCollaborationRecordOp | DeleteCollaborationRecordOp |
   RenameCollaborationRecordOp | ReplaceCollaborationRecordOp | CreateSafetyRequirementOp |
   DeleteSafetyRequirementOp | RenameSafetyRequirementOp | ReplaceSafetyRequirementOp | CreateUserProfileOp |
   DeleteUserProfileOp | RenameUserProfileOp | ReplaceUserProfileOp | CreateHumanFactorRequirementOp |
   DeleteHumanFactorRequirementOp | RenameHumanFactorRequirementOp | ReplaceHumanFactorRequirementOp |
   CreateFlexibilityRequirementOp | DeleteFlexibilityRequirementOp | RenameFlexibilityRequirementOp |
   ReplaceFlexibilityRequirementOp | CreateWayfindingRequirementOp | DeleteWayfindingRequirementOp |
   RenameWayfindingRequirementOp | ReplaceWayfindingRequirementOp | CreateProgramElementOp | DeleteProgramElementOp |
   RenameProgramElementOp | ReplaceProgramElementOp | ConnectAdjacencyOp | DisconnectAdjacencyOp | ConnectTraceOp |
   DisconnectTraceOp | RenameMetaOp | ReplaceMetaOp | RenameProjectOp | ReplaceProjectOp | RenameGovernanceOp |
   ReplaceGovernanceOp;
