/** 🧺️ `En1998Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeTowerMRdNm, parseChangeTowerMRdNm } from "./↪️change-tower-m-rd-nm/🧬️schema/🟦️.ts";
import { type ChangeStoreyPermanentGkN, parseChangeStoreyPermanentGkN } from "./⚖️change-storey-permanent-gk-n/🧬️schema/🟦️.ts";
import { type ChangeMemberDetailing, parseChangeMemberDetailing } from "./✅️change-member-detailing/🧬️schema/🟦️.ts";
import { type InsertBuilding, parseInsertBuilding } from "./➕️insert-building/🧬️schema/🟦️.ts";
import { parseRemoveAssessment, type RemoveAssessment } from "./➖️remove-assessment/🧬️schema/🟦️.ts";
import { parseRemoveBridge, type RemoveBridge } from "./➖️remove-bridge/🧬️schema/🟦️.ts";
import { parseRemoveBuilding, type RemoveBuilding } from "./➖️remove-building/🧬️schema/🟦️.ts";
import { parseRemoveFoundation, type RemoveFoundation } from "./➖️remove-foundation/🧬️schema/🟦️.ts";
import { parseRemoveRetainingWall, type RemoveRetainingWall } from "./➖️remove-retaining-wall/🧬️schema/🟦️.ts";
import { parseRemoveSilo, type RemoveSilo } from "./➖️remove-silo/🧬️schema/🟦️.ts";
import { parseRemoveTank, type RemoveTank } from "./➖️remove-tank/🧬️schema/🟦️.ts";
import { parseRemoveTower, type RemoveTower } from "./➖️remove-tower/🧬️schema/🟦️.ts";
import { type InsertBridge, parseInsertBridge } from "./🌉insert-bridge/🧬️schema/🟦️.ts";
import { parseUpdateSite, type UpdateSite } from "./🌚️update-site/🧬️schema/🟦️.ts";
import { type ChangeAssessmentRKN, parseChangeAssessmentRKN } from "./🏋️change-assessment-rkn/🧬️schema/🟦️.ts";
import { type ChangeSystemVRdN, parseChangeSystemVRdN } from "./💪️change-system-v-rd-n/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./📎️change-annex/🧬️schema/🟦️.ts";
import { type ChangeElevationRegular, parseChangeElevationRegular } from "./📏️change-elevation-regular/🧬️schema/🟦️.ts";
import { type ChangeStoreyDriftXM, parseChangeStoreyDriftXM } from "./📏️change-storey-drift-xm/🧬️schema/🟦️.ts";
import { type ChangeStoreyStiffnessX, parseChangeStoreyStiffnessX } from "./📐️change-storey-stiffness-x/🧬️schema/🟦️.ts";
import { type InsertAssessment, parseInsertAssessment } from "./🔧insert-assessment/🧬️schema/🟦️.ts";
import { type InsertTower, parseInsertTower } from "./🗼insert-tower/🧬️schema/🟦️.ts";
import { type ChangeBuildingPlanRegular, parseChangeBuildingPlanRegular } from "./🧭️change-building-plan-regular/🧬️schema/🟦️.ts";
import { type ChangeMasonryWallRatio, parseChangeMasonryWallRatio } from "./🧱️change-masonry-wall-ratio/🧬️schema/🟦️.ts";
import { type InsertRetainingWall, parseInsertRetainingWall } from "./🧱️insert-retaining-wall/🧬️schema/🟦️.ts";
import { type InsertFoundation, parseInsertFoundation } from "./🪨insert-foundation/🧬️schema/🟦️.ts";
import { type InsertSilo, parseInsertSilo } from "./🫙insert-silo/🧬️schema/🟦️.ts";
import { type ChangeBridgeVRdN, parseChangeBridgeVRdN } from "./🛑️change-bridge-v-rd-n/🧬️schema/🟦️.ts";
import { type InsertTank, parseInsertTank } from "./🛢insert-tank/🧬️schema/🟦️.ts";

export type En1998Mutation =
  | ChangeAnnex
  | UpdateSite
  | InsertBuilding
  | RemoveBuilding
  | ChangeSystemVRdN
  | ChangeStoreyPermanentGkN
  | ChangeStoreyStiffnessX
  | ChangeStoreyDriftXM
  | ChangeBuildingPlanRegular
  | ChangeElevationRegular
  | ChangeMemberDetailing
  | ChangeMasonryWallRatio
  | InsertBridge
  | ChangeBridgeVRdN
  | InsertAssessment
  | ChangeAssessmentRKN
  | InsertSilo
  | InsertTank
  | InsertFoundation
  | InsertRetainingWall
  | InsertTower
  | ChangeTowerMRdNm
  | RemoveBridge
  | RemoveAssessment
  | RemoveSilo
  | RemoveTank
  | RemoveFoundation
  | RemoveRetainingWall
  | RemoveTower;

export const parseEn1998Mutation: NormWireReader<En1998Mutation> = normWireTagged<En1998Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  updateSite: parseUpdateSite,
  insertBuilding: parseInsertBuilding,
  removeBuilding: parseRemoveBuilding,
  changeSystemVRdN: parseChangeSystemVRdN,
  changeStoreyPermanentGkN: parseChangeStoreyPermanentGkN,
  changeStoreyStiffnessX: parseChangeStoreyStiffnessX,
  changeStoreyDriftXM: parseChangeStoreyDriftXM,
  changeBuildingPlanRegular: parseChangeBuildingPlanRegular,
  changeElevationRegular: parseChangeElevationRegular,
  changeMemberDetailing: parseChangeMemberDetailing,
  changeMasonryWallRatio: parseChangeMasonryWallRatio,
  insertBridge: parseInsertBridge,
  changeBridgeVRdN: parseChangeBridgeVRdN,
  insertAssessment: parseInsertAssessment,
  changeAssessmentRKN: parseChangeAssessmentRKN,
  insertSilo: parseInsertSilo,
  insertTank: parseInsertTank,
  insertFoundation: parseInsertFoundation,
  insertRetainingWall: parseInsertRetainingWall,
  insertTower: parseInsertTower,
  changeTowerMRdNm: parseChangeTowerMRdNm,
  removeBridge: parseRemoveBridge,
  removeAssessment: parseRemoveAssessment,
  removeSilo: parseRemoveSilo,
  removeTank: parseRemoveTank,
  removeFoundation: parseRemoveFoundation,
  removeRetainingWall: parseRemoveRetainingWall,
  removeTower: parseRemoveTower,
});
