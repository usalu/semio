/** 📋️ ProgramSnapshot diff schema: the sparse deltas of the operational, resource, safety, site, service, schedule, sustainability, cost and delivery requirement registers. */
import type { AccessibilityRequirement, CommunicationRequirement, CostRequirement, DeliveryConstraint, EnvironmentalRequirement, Equipment, FlexibilityRequirement, GrowthPlan, HumanFactorRequirement, InformationRequirement, InfrastructureRequirement, OperationalRequirement, OrganizationalRequirement, PrivacyRequirement, RegulatoryRequirement, ResilienceRequirement, Resource, SafetyRequirement, ScheduleRequirement, SecurityRequirement, ServiceRequirement, SiteContext, StorageRequirement, SustainabilityRequirement, WayfindingRequirement } from "../../📋️requirements/🟦️.ts";
import type { AccessibilityRequirementPatch, CommunicationRequirementPatch, CostRequirementPatch, DeliveryConstraintPatch, EnvironmentalRequirementPatch, EquipmentPatch, FlexibilityRequirementPatch, GrowthPlanPatch, HumanFactorRequirementPatch, InformationRequirementPatch, InfrastructureRequirementPatch, OperationalRequirementPatch, OrganizationalRequirementPatch, PrivacyRequirementPatch, RegulatoryRequirementPatch, ResilienceRequirementPatch, ResourcePatch, SafetyRequirementPatch, ScheduleRequirementPatch, SecurityRequirementPatch, ServiceRequirementPatch, SiteContextPatch, StorageRequirementPatch, SustainabilityRequirementPatch, WayfindingRequirementPatch } from "../🩹️patch-types/🟦️.ts";

export interface ProgramOperationsDelta {
  removed: ProgramOperationsRemoval[];
  inserted: ProgramOperationsInsertion[];
  moved: ProgramOperationsRelocation[];
  modified: ProgramOperationsPatchEntry[];
}

export interface ProgramOperationsInsertion {
  index: number;
  row: OperationalRequirement;
}

export interface ProgramOperationsRemoval {
  id: string;
  index: number;
}

export interface ProgramOperationsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOperationsPatchEntry {
  id: string;
  patch: OperationalRequirementPatch;
}

export interface ProgramEquipmentDelta {
  removed: ProgramEquipmentRemoval[];
  inserted: ProgramEquipmentInsertion[];
  moved: ProgramEquipmentRelocation[];
  modified: ProgramEquipmentPatchEntry[];
}

export interface ProgramEquipmentInsertion {
  index: number;
  row: Equipment;
}

export interface ProgramEquipmentRemoval {
  id: string;
  index: number;
}

export interface ProgramEquipmentRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramEquipmentPatchEntry {
  id: string;
  patch: EquipmentPatch;
}

export interface ProgramResourcesDelta {
  removed: ProgramResourcesRemoval[];
  inserted: ProgramResourcesInsertion[];
  moved: ProgramResourcesRelocation[];
  modified: ProgramResourcesPatchEntry[];
}

export interface ProgramResourcesInsertion {
  index: number;
  row: Resource;
}

export interface ProgramResourcesRemoval {
  id: string;
  index: number;
}

export interface ProgramResourcesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramResourcesPatchEntry {
  id: string;
  patch: ResourcePatch;
}

export interface ProgramStorageDelta {
  removed: ProgramStorageRemoval[];
  inserted: ProgramStorageInsertion[];
  moved: ProgramStorageRelocation[];
  modified: ProgramStoragePatchEntry[];
}

export interface ProgramStorageInsertion {
  index: number;
  row: StorageRequirement;
}

export interface ProgramStorageRemoval {
  id: string;
  index: number;
}

export interface ProgramStorageRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStoragePatchEntry {
  id: string;
  patch: StorageRequirementPatch;
}

export interface ProgramEnvironmentalDelta {
  removed: ProgramEnvironmentalRemoval[];
  inserted: ProgramEnvironmentalInsertion[];
  moved: ProgramEnvironmentalRelocation[];
  modified: ProgramEnvironmentalPatchEntry[];
}

export interface ProgramEnvironmentalInsertion {
  index: number;
  row: EnvironmentalRequirement;
}

export interface ProgramEnvironmentalRemoval {
  id: string;
  index: number;
}

export interface ProgramEnvironmentalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramEnvironmentalPatchEntry {
  id: string;
  patch: EnvironmentalRequirementPatch;
}

export interface ProgramHumanFactorsDelta {
  removed: ProgramHumanFactorsRemoval[];
  inserted: ProgramHumanFactorsInsertion[];
  moved: ProgramHumanFactorsRelocation[];
  modified: ProgramHumanFactorsPatchEntry[];
}

export interface ProgramHumanFactorsInsertion {
  index: number;
  row: HumanFactorRequirement;
}

export interface ProgramHumanFactorsRemoval {
  id: string;
  index: number;
}

export interface ProgramHumanFactorsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramHumanFactorsPatchEntry {
  id: string;
  patch: HumanFactorRequirementPatch;
}

export interface ProgramAccessibilityDelta {
  removed: ProgramAccessibilityRemoval[];
  inserted: ProgramAccessibilityInsertion[];
  moved: ProgramAccessibilityRelocation[];
  modified: ProgramAccessibilityPatchEntry[];
}

export interface ProgramAccessibilityInsertion {
  index: number;
  row: AccessibilityRequirement;
}

export interface ProgramAccessibilityRemoval {
  id: string;
  index: number;
}

export interface ProgramAccessibilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAccessibilityPatchEntry {
  id: string;
  patch: AccessibilityRequirementPatch;
}

export interface ProgramPrivacyDelta {
  removed: ProgramPrivacyRemoval[];
  inserted: ProgramPrivacyInsertion[];
  moved: ProgramPrivacyRelocation[];
  modified: ProgramPrivacyPatchEntry[];
}

export interface ProgramPrivacyInsertion {
  index: number;
  row: PrivacyRequirement;
}

export interface ProgramPrivacyRemoval {
  id: string;
  index: number;
}

export interface ProgramPrivacyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPrivacyPatchEntry {
  id: string;
  patch: PrivacyRequirementPatch;
}

export interface ProgramSafetyDelta {
  removed: ProgramSafetyRemoval[];
  inserted: ProgramSafetyInsertion[];
  moved: ProgramSafetyRelocation[];
  modified: ProgramSafetyPatchEntry[];
}

export interface ProgramSafetyInsertion {
  index: number;
  row: SafetyRequirement;
}

export interface ProgramSafetyRemoval {
  id: string;
  index: number;
}

export interface ProgramSafetyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSafetyPatchEntry {
  id: string;
  patch: SafetyRequirementPatch;
}

export interface ProgramSecurityDelta {
  removed: ProgramSecurityRemoval[];
  inserted: ProgramSecurityInsertion[];
  moved: ProgramSecurityRelocation[];
  modified: ProgramSecurityPatchEntry[];
}

export interface ProgramSecurityInsertion {
  index: number;
  row: SecurityRequirement;
}

export interface ProgramSecurityRemoval {
  id: string;
  index: number;
}

export interface ProgramSecurityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSecurityPatchEntry {
  id: string;
  patch: SecurityRequirementPatch;
}

export interface ProgramRegulatoryDelta {
  removed: ProgramRegulatoryRemoval[];
  inserted: ProgramRegulatoryInsertion[];
  moved: ProgramRegulatoryRelocation[];
  modified: ProgramRegulatoryPatchEntry[];
}

export interface ProgramRegulatoryInsertion {
  index: number;
  row: RegulatoryRequirement;
}

export interface ProgramRegulatoryRemoval {
  id: string;
  index: number;
}

export interface ProgramRegulatoryRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRegulatoryPatchEntry {
  id: string;
  patch: RegulatoryRequirementPatch;
}

export interface ProgramSiteContextDelta {
  removed: ProgramSiteContextRemoval[];
  inserted: ProgramSiteContextInsertion[];
  moved: ProgramSiteContextRelocation[];
  modified: ProgramSiteContextPatchEntry[];
}

export interface ProgramSiteContextInsertion {
  index: number;
  row: SiteContext;
}

export interface ProgramSiteContextRemoval {
  id: string;
  index: number;
}

export interface ProgramSiteContextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSiteContextPatchEntry {
  id: string;
  patch: SiteContextPatch;
}

export interface ProgramOrganizationalDelta {
  removed: ProgramOrganizationalRemoval[];
  inserted: ProgramOrganizationalInsertion[];
  moved: ProgramOrganizationalRelocation[];
  modified: ProgramOrganizationalPatchEntry[];
}

export interface ProgramOrganizationalInsertion {
  index: number;
  row: OrganizationalRequirement;
}

export interface ProgramOrganizationalRemoval {
  id: string;
  index: number;
}

export interface ProgramOrganizationalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOrganizationalPatchEntry {
  id: string;
  patch: OrganizationalRequirementPatch;
}

export interface ProgramServicesDelta {
  removed: ProgramServicesRemoval[];
  inserted: ProgramServicesInsertion[];
  moved: ProgramServicesRelocation[];
  modified: ProgramServicesPatchEntry[];
}

export interface ProgramServicesInsertion {
  index: number;
  row: ServiceRequirement;
}

export interface ProgramServicesRemoval {
  id: string;
  index: number;
}

export interface ProgramServicesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramServicesPatchEntry {
  id: string;
  patch: ServiceRequirementPatch;
}

export interface ProgramInfrastructureDelta {
  removed: ProgramInfrastructureRemoval[];
  inserted: ProgramInfrastructureInsertion[];
  moved: ProgramInfrastructureRelocation[];
  modified: ProgramInfrastructurePatchEntry[];
}

export interface ProgramInfrastructureInsertion {
  index: number;
  row: InfrastructureRequirement;
}

export interface ProgramInfrastructureRemoval {
  id: string;
  index: number;
}

export interface ProgramInfrastructureRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramInfrastructurePatchEntry {
  id: string;
  patch: InfrastructureRequirementPatch;
}

export interface ProgramInformationDelta {
  removed: ProgramInformationRemoval[];
  inserted: ProgramInformationInsertion[];
  moved: ProgramInformationRelocation[];
  modified: ProgramInformationPatchEntry[];
}

export interface ProgramInformationInsertion {
  index: number;
  row: InformationRequirement;
}

export interface ProgramInformationRemoval {
  id: string;
  index: number;
}

export interface ProgramInformationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramInformationPatchEntry {
  id: string;
  patch: InformationRequirementPatch;
}

export interface ProgramCommunicationDelta {
  removed: ProgramCommunicationRemoval[];
  inserted: ProgramCommunicationInsertion[];
  moved: ProgramCommunicationRelocation[];
  modified: ProgramCommunicationPatchEntry[];
}

export interface ProgramCommunicationInsertion {
  index: number;
  row: CommunicationRequirement;
}

export interface ProgramCommunicationRemoval {
  id: string;
  index: number;
}

export interface ProgramCommunicationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCommunicationPatchEntry {
  id: string;
  patch: CommunicationRequirementPatch;
}

export interface ProgramWayfindingDelta {
  removed: ProgramWayfindingRemoval[];
  inserted: ProgramWayfindingInsertion[];
  moved: ProgramWayfindingRelocation[];
  modified: ProgramWayfindingPatchEntry[];
}

export interface ProgramWayfindingInsertion {
  index: number;
  row: WayfindingRequirement;
}

export interface ProgramWayfindingRemoval {
  id: string;
  index: number;
}

export interface ProgramWayfindingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramWayfindingPatchEntry {
  id: string;
  patch: WayfindingRequirementPatch;
}

export interface ProgramSchedulesDelta {
  removed: ProgramSchedulesRemoval[];
  inserted: ProgramSchedulesInsertion[];
  moved: ProgramSchedulesRelocation[];
  modified: ProgramSchedulesPatchEntry[];
}

export interface ProgramSchedulesInsertion {
  index: number;
  row: ScheduleRequirement;
}

export interface ProgramSchedulesRemoval {
  id: string;
  index: number;
}

export interface ProgramSchedulesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSchedulesPatchEntry {
  id: string;
  patch: ScheduleRequirementPatch;
}

export interface ProgramFlexibilityDelta {
  removed: ProgramFlexibilityRemoval[];
  inserted: ProgramFlexibilityInsertion[];
  moved: ProgramFlexibilityRelocation[];
  modified: ProgramFlexibilityPatchEntry[];
}

export interface ProgramFlexibilityInsertion {
  index: number;
  row: FlexibilityRequirement;
}

export interface ProgramFlexibilityRemoval {
  id: string;
  index: number;
}

export interface ProgramFlexibilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFlexibilityPatchEntry {
  id: string;
  patch: FlexibilityRequirementPatch;
}

export interface ProgramGrowthDelta {
  removed: ProgramGrowthRemoval[];
  inserted: ProgramGrowthInsertion[];
  moved: ProgramGrowthRelocation[];
  modified: ProgramGrowthPatchEntry[];
}

export interface ProgramGrowthInsertion {
  index: number;
  row: GrowthPlan;
}

export interface ProgramGrowthRemoval {
  id: string;
  index: number;
}

export interface ProgramGrowthRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramGrowthPatchEntry {
  id: string;
  patch: GrowthPlanPatch;
}

export interface ProgramSustainabilityDelta {
  removed: ProgramSustainabilityRemoval[];
  inserted: ProgramSustainabilityInsertion[];
  moved: ProgramSustainabilityRelocation[];
  modified: ProgramSustainabilityPatchEntry[];
}

export interface ProgramSustainabilityInsertion {
  index: number;
  row: SustainabilityRequirement;
}

export interface ProgramSustainabilityRemoval {
  id: string;
  index: number;
}

export interface ProgramSustainabilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSustainabilityPatchEntry {
  id: string;
  patch: SustainabilityRequirementPatch;
}

export interface ProgramResilienceDelta {
  removed: ProgramResilienceRemoval[];
  inserted: ProgramResilienceInsertion[];
  moved: ProgramResilienceRelocation[];
  modified: ProgramResiliencePatchEntry[];
}

export interface ProgramResilienceInsertion {
  index: number;
  row: ResilienceRequirement;
}

export interface ProgramResilienceRemoval {
  id: string;
  index: number;
}

export interface ProgramResilienceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramResiliencePatchEntry {
  id: string;
  patch: ResilienceRequirementPatch;
}

export interface ProgramCostsDelta {
  removed: ProgramCostsRemoval[];
  inserted: ProgramCostsInsertion[];
  moved: ProgramCostsRelocation[];
  modified: ProgramCostsPatchEntry[];
}

export interface ProgramCostsInsertion {
  index: number;
  row: CostRequirement;
}

export interface ProgramCostsRemoval {
  id: string;
  index: number;
}

export interface ProgramCostsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCostsPatchEntry {
  id: string;
  patch: CostRequirementPatch;
}

export interface ProgramDeliveryDelta {
  removed: ProgramDeliveryRemoval[];
  inserted: ProgramDeliveryInsertion[];
  moved: ProgramDeliveryRelocation[];
  modified: ProgramDeliveryPatchEntry[];
}

export interface ProgramDeliveryInsertion {
  index: number;
  row: DeliveryConstraint;
}

export interface ProgramDeliveryRemoval {
  id: string;
  index: number;
}

export interface ProgramDeliveryRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramDeliveryPatchEntry {
  id: string;
  patch: DeliveryConstraintPatch;
}
