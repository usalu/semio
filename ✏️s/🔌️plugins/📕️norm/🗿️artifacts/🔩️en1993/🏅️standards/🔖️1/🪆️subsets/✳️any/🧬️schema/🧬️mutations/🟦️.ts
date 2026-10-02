/** 🧺️ `En1993Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseUpdateThroughThicknessInputs, type UpdateThroughThicknessInputs } from "./↕️update-through-thickness-inputs/🧬️schema/🟦️.ts";
import { parseUpdateStainlessInputs, type UpdateStainlessInputs } from "./✨️update-stainless-inputs/🧬️schema/🟦️.ts";
import { type InsertBridgeFatigue, parseInsertBridgeFatigue } from "./➕️insert-bridge-fatigue/🧬️schema/🟦️.ts";
import { type InsertColdFormedMember, parseInsertColdFormedMember } from "./➕️insert-cold-formed-member/🧬️schema/🟦️.ts";
import { type InsertCraneRunway, parseInsertCraneRunway } from "./➕️insert-crane-runway/🧬️schema/🟦️.ts";
import { type InsertFatigueDetail, parseInsertFatigueDetail } from "./➕️insert-fatigue-detail/🧬️schema/🟦️.ts";
import { type InsertFireExposure, parseInsertFireExposure } from "./➕️insert-fire-exposure/🧬️schema/🟦️.ts";
import { type InsertJoint, parseInsertJoint } from "./➕️insert-joint/🧬️schema/🟦️.ts";
import { type InsertLoadCase, parseInsertLoadCase } from "./➕️insert-load-case/🧬️schema/🟦️.ts";
import { type InsertMaterial, parseInsertMaterial } from "./➕️insert-material/🧬️schema/🟦️.ts";
import { type InsertMemberAction, parseInsertMemberAction } from "./➕️insert-member-action/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { type InsertPile, parseInsertPile } from "./➕️insert-pile/🧬️schema/🟦️.ts";
import { type InsertPlatedPanel, parseInsertPlatedPanel } from "./➕️insert-plated-panel/🧬️schema/🟦️.ts";
import { type InsertSection, parseInsertSection } from "./➕️insert-section/🧬️schema/🟦️.ts";
import { type InsertSiloShell, parseInsertSiloShell } from "./➕️insert-silo-shell/🧬️schema/🟦️.ts";
import { type InsertTensionComponent, parseInsertTensionComponent } from "./➕️insert-tension-component/🧬️schema/🟦️.ts";
import { type InsertTowerLeg, parseInsertTowerLeg } from "./➕️insert-tower-leg/🧬️schema/🟦️.ts";
import { parseRemoveBridgeFatigue, type RemoveBridgeFatigue } from "./➖️remove-bridge-fatigue/🧬️schema/🟦️.ts";
import { parseRemoveColdFormedMember, type RemoveColdFormedMember } from "./➖️remove-cold-formed-member/🧬️schema/🟦️.ts";
import { parseRemoveCraneRunway, type RemoveCraneRunway } from "./➖️remove-crane-runway/🧬️schema/🟦️.ts";
import { parseRemoveFatigueDetail, type RemoveFatigueDetail } from "./➖️remove-fatigue-detail/🧬️schema/🟦️.ts";
import { parseRemoveFireExposure, type RemoveFireExposure } from "./➖️remove-fire-exposure/🧬️schema/🟦️.ts";
import { parseRemoveJoint, type RemoveJoint } from "./➖️remove-joint/🧬️schema/🟦️.ts";
import { parseRemoveLoadCase, type RemoveLoadCase } from "./➖️remove-load-case/🧬️schema/🟦️.ts";
import { parseRemoveMaterial, type RemoveMaterial } from "./➖️remove-material/🧬️schema/🟦️.ts";
import { parseRemoveMemberAction, type RemoveMemberAction } from "./➖️remove-member-action/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { parseRemovePile, type RemovePile } from "./➖️remove-pile/🧬️schema/🟦️.ts";
import { parseRemovePlatedPanel, type RemovePlatedPanel } from "./➖️remove-plated-panel/🧬️schema/🟦️.ts";
import { parseRemoveSection, type RemoveSection } from "./➖️remove-section/🧬️schema/🟦️.ts";
import { parseRemoveSiloShell, type RemoveSiloShell } from "./➖️remove-silo-shell/🧬️schema/🟦️.ts";
import { parseRemoveTensionComponent, type RemoveTensionComponent } from "./➖️remove-tension-component/🧬️schema/🟦️.ts";
import { parseRemoveTowerLeg, type RemoveTowerLeg } from "./➖️remove-tower-leg/🧬️schema/🟦️.ts";
import { parseUpdateHssInputs, type UpdateHssInputs } from "./⬜️update-hss-inputs/🧬️schema/🟦️.ts";
import { parseUpdateBridgeInputs, type UpdateBridgeInputs } from "./🌉️update-bridge-inputs/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { parseUpdateCraneInputs, type UpdateCraneInputs } from "./🏗️update-crane-inputs/🧬️schema/🟦️.ts";
import { parseUpdateMemberProperties, type UpdateMemberProperties } from "./📊️update-member-properties/🧬️schema/🟦️.ts";
import { parseUpdateFatigueInputs, type UpdateFatigueInputs } from "./🔁️update-fatigue-inputs/🧬️schema/🟦️.ts";
import { parseUpdateFireInputs, type UpdateFireInputs } from "./🔥️update-fire-inputs/🧬️schema/🟦️.ts";
import { parseUpdateBoltInputs, type UpdateBoltInputs } from "./🔩️update-bolt-inputs/🧬️schema/🟦️.ts";
import { parseUpdateTowerInputs, type UpdateTowerInputs } from "./🗼️update-tower-inputs/🧬️schema/🟦️.ts";
import { parseUpdateColdFormedInputs, type UpdateColdFormedInputs } from "./🥶️update-cold-formed-inputs/🧬️schema/🟦️.ts";
import { parseUpdatePlatedInputs, type UpdatePlatedInputs } from "./🧱️update-plated-inputs/🧬️schema/🟦️.ts";
import { parseUpdateWeldInputs, type UpdateWeldInputs } from "./🧲️update-weld-inputs/🧬️schema/🟦️.ts";
import { parseUpdateTensionComponentInputs, type UpdateTensionComponentInputs } from "./🪢️update-tension-component-inputs/🧬️schema/🟦️.ts";
import { parseUpdatePileInputs, type UpdatePileInputs } from "./🪵️update-pile-inputs/🧬️schema/🟦️.ts";
import { parseUpdateSiloShellInputs, type UpdateSiloShellInputs } from "./🛢️update-silo-shell-inputs/🧬️schema/🟦️.ts";

export type En1993Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { UpdateMemberProperties: UpdateMemberProperties }
  | { UpdateFireInputs: UpdateFireInputs }
  | { UpdateColdFormedInputs: UpdateColdFormedInputs }
  | { UpdateStainlessInputs: UpdateStainlessInputs }
  | { UpdatePlatedInputs: UpdatePlatedInputs }
  | { UpdateSiloShellInputs: UpdateSiloShellInputs }
  | { UpdateBoltInputs: UpdateBoltInputs }
  | { UpdateWeldInputs: UpdateWeldInputs }
  | { UpdateFatigueInputs: UpdateFatigueInputs }
  | { UpdateThroughThicknessInputs: UpdateThroughThicknessInputs }
  | { UpdateTensionComponentInputs: UpdateTensionComponentInputs }
  | { UpdateHssInputs: UpdateHssInputs }
  | { UpdateBridgeInputs: UpdateBridgeInputs }
  | { UpdateTowerInputs: UpdateTowerInputs }
  | { UpdatePileInputs: UpdatePileInputs }
  | { UpdateCraneInputs: UpdateCraneInputs }
  | { InsertMaterial: InsertMaterial }
  | { RemoveMaterial: RemoveMaterial }
  | { InsertSection: InsertSection }
  | { RemoveSection: RemoveSection }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { InsertLoadCase: InsertLoadCase }
  | { RemoveLoadCase: RemoveLoadCase }
  | { InsertMemberAction: InsertMemberAction }
  | { RemoveMemberAction: RemoveMemberAction }
  | { InsertJoint: InsertJoint }
  | { RemoveJoint: RemoveJoint }
  | { InsertFatigueDetail: InsertFatigueDetail }
  | { RemoveFatigueDetail: RemoveFatigueDetail }
  | { InsertFireExposure: InsertFireExposure }
  | { RemoveFireExposure: RemoveFireExposure }
  | { InsertColdFormedMember: InsertColdFormedMember }
  | { RemoveColdFormedMember: RemoveColdFormedMember }
  | { InsertPlatedPanel: InsertPlatedPanel }
  | { RemovePlatedPanel: RemovePlatedPanel }
  | { InsertSiloShell: InsertSiloShell }
  | { RemoveSiloShell: RemoveSiloShell }
  | { InsertTensionComponent: InsertTensionComponent }
  | { RemoveTensionComponent: RemoveTensionComponent }
  | { InsertBridgeFatigue: InsertBridgeFatigue }
  | { RemoveBridgeFatigue: RemoveBridgeFatigue }
  | { InsertTowerLeg: InsertTowerLeg }
  | { RemoveTowerLeg: RemoveTowerLeg }
  | { InsertPile: InsertPile }
  | { RemovePile: RemovePile }
  | { InsertCraneRunway: InsertCraneRunway }
  | { RemoveCraneRunway: RemoveCraneRunway };

export const parseEn1993Mutation: NormWireReader<En1993Mutation> = normWireExternal<En1993Mutation>({
  ChangeAnnex: parseChangeAnnex,
  UpdateMemberProperties: parseUpdateMemberProperties,
  UpdateFireInputs: parseUpdateFireInputs,
  UpdateColdFormedInputs: parseUpdateColdFormedInputs,
  UpdateStainlessInputs: parseUpdateStainlessInputs,
  UpdatePlatedInputs: parseUpdatePlatedInputs,
  UpdateSiloShellInputs: parseUpdateSiloShellInputs,
  UpdateBoltInputs: parseUpdateBoltInputs,
  UpdateWeldInputs: parseUpdateWeldInputs,
  UpdateFatigueInputs: parseUpdateFatigueInputs,
  UpdateThroughThicknessInputs: parseUpdateThroughThicknessInputs,
  UpdateTensionComponentInputs: parseUpdateTensionComponentInputs,
  UpdateHssInputs: parseUpdateHssInputs,
  UpdateBridgeInputs: parseUpdateBridgeInputs,
  UpdateTowerInputs: parseUpdateTowerInputs,
  UpdatePileInputs: parseUpdatePileInputs,
  UpdateCraneInputs: parseUpdateCraneInputs,
  InsertMaterial: parseInsertMaterial,
  RemoveMaterial: parseRemoveMaterial,
  InsertSection: parseInsertSection,
  RemoveSection: parseRemoveSection,
  InsertMember: parseInsertMember,
  RemoveMember: parseRemoveMember,
  InsertLoadCase: parseInsertLoadCase,
  RemoveLoadCase: parseRemoveLoadCase,
  InsertMemberAction: parseInsertMemberAction,
  RemoveMemberAction: parseRemoveMemberAction,
  InsertJoint: parseInsertJoint,
  RemoveJoint: parseRemoveJoint,
  InsertFatigueDetail: parseInsertFatigueDetail,
  RemoveFatigueDetail: parseRemoveFatigueDetail,
  InsertFireExposure: parseInsertFireExposure,
  RemoveFireExposure: parseRemoveFireExposure,
  InsertColdFormedMember: parseInsertColdFormedMember,
  RemoveColdFormedMember: parseRemoveColdFormedMember,
  InsertPlatedPanel: parseInsertPlatedPanel,
  RemovePlatedPanel: parseRemovePlatedPanel,
  InsertSiloShell: parseInsertSiloShell,
  RemoveSiloShell: parseRemoveSiloShell,
  InsertTensionComponent: parseInsertTensionComponent,
  RemoveTensionComponent: parseRemoveTensionComponent,
  InsertBridgeFatigue: parseInsertBridgeFatigue,
  RemoveBridgeFatigue: parseRemoveBridgeFatigue,
  InsertTowerLeg: parseInsertTowerLeg,
  RemoveTowerLeg: parseRemoveTowerLeg,
  InsertPile: parseInsertPile,
  RemovePile: parseRemovePile,
  InsertCraneRunway: parseInsertCraneRunway,
  RemoveCraneRunway: parseRemoveCraneRunway,
});
