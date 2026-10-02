/** 🧺️ `En1996Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeSlabSpan, parseChangeSlabSpan } from "./↔️change-slab-span/🧬️schema/🟦️.ts";
import { type ChangeWallLength, parseChangeWallLength } from "./↔️change-wall-length/🧬️schema/🟦️.ts";
import { type ChangeWallHeight, parseChangeWallHeight } from "./↕️change-wall-height/🧬️schema/🟦️.ts";
import { type ChangeWallThickness, parseChangeWallThickness } from "./↕️change-wall-thickness/🧬️schema/🟦️.ts";
import { type ChangeEccentricityBottom, parseChangeEccentricityBottom } from "./↗️change-eccentricity-bottom/🧬️schema/🟦️.ts";
import { type ChangeEccentricityTop, parseChangeEccentricityTop } from "./↘️change-eccentricity-top/🧬️schema/🟦️.ts";
import { type ChangePhiInfinity, parseChangePhiInfinity } from "./♾️change-phi-infinity/🧬️schema/🟦️.ts";
import { type ChangeQKSnow, parseChangeQKSnow } from "./❄️change-qk-snow/🧬️schema/🟦️.ts";
import { type InsertConcentrated, parseInsertConcentrated } from "./➕️insert-concentrated/🧬️schema/🟦️.ts";
import { type InsertLoadCase, parseInsertLoadCase } from "./➕️insert-load-case/🧬️schema/🟦️.ts";
import { type InsertOpening, parseInsertOpening } from "./➕️insert-opening/🧬️schema/🟦️.ts";
import { type InsertWall, parseInsertWall } from "./➕️insert-wall/🧬️schema/🟦️.ts";
import { parseRemoveConcentrated, type RemoveConcentrated } from "./➖️remove-concentrated/🧬️schema/🟦️.ts";
import { parseRemoveLoadCase, type RemoveLoadCase } from "./➖️remove-load-case/🧬️schema/🟦️.ts";
import { parseRemoveOpening, type RemoveOpening } from "./➖️remove-opening/🧬️schema/🟦️.ts";
import { parseRemoveWall, type RemoveWall } from "./➖️remove-wall/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeQPWind, parseChangeQPWind } from "./🌬️change-qp-wind/🧬️schema/🟦️.ts";
import { type ChangeDesignSituation, parseChangeDesignSituation } from "./🎭️change-design-situation/🧬️schema/🟦️.ts";
import { type ChangeLoadCaseSituation, parseChangeLoadCaseSituation } from "./🎭️change-load-case-situation/🧬️schema/🟦️.ts";
import { type ChangeConcentratedForce, parseChangeConcentratedForce } from "./🏋️change-concentrated-force/🧬️schema/🟦️.ts";
import { type ChangeGKSlab, parseChangeGKSlab } from "./🏋️change-gk-slab/🧬️schema/🟦️.ts";
import { type ChangeQKImposed, parseChangeQKImposed } from "./🏋️change-qk-imposed/🧬️schema/🟦️.ts";
import { type ChangeIsBasement, parseChangeIsBasement } from "./🏗️change-is-basement/🧬️schema/🟦️.ts";
import { type ChangeStoreys, parseChangeStoreys } from "./🏢️change-storeys/🧬️schema/🟦️.ts";
import { type ChangeMasonryClass, parseChangeMasonryClass } from "./🏭️change-masonry-class/🧬️schema/🟦️.ts";
import { type ChangeImposedCategory, parseChangeImposedCategory } from "./🏷️change-imposed-category/🧬️schema/🟦️.ts";
import { type ChangeWallLabelDe, parseChangeWallLabelDe } from "./🏷️change-wall-label-de/🧬️schema/🟦️.ts";
import { type ChangeWallLabelEn, parseChangeWallLabelEn } from "./🏷️change-wall-label-en/🧬️schema/🟦️.ts";
import { type ChangeExposure, parseChangeExposure } from "./💧️change-exposure/🧬️schema/🟦️.ts";
import { type ChangeConcentratedBearingLength, parseChangeConcentratedBearingLength } from "./📏change-concentrated-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeConcentratedBearingArea, parseChangeConcentratedBearingArea } from "./📐️change-concentrated-bearing-area/🧬️schema/🟦️.ts";
import { type ChangeSlabBearingDepth, parseChangeSlabBearingDepth } from "./📐️change-slab-bearing-depth/🧬️schema/🟦️.ts";
import { type ChangeTributaryArea, parseChangeTributaryArea } from "./📐️change-tributary-area/🧬️schema/🟦️.ts";
import { type ChangeFireRei, parseChangeFireRei } from "./🔥️change-fire-rei/🧬️schema/🟦️.ts";
import { type ChangeAsHorizontal, parseChangeAsHorizontal } from "./🔩change-as-horizontal/🧬️schema/🟦️.ts";
import { type ChangeAsVertical, parseChangeAsVertical } from "./🔩change-as-vertical/🧬️schema/🟦️.ts";
import { type ChangeFYd, parseChangeFYd } from "./🔩change-f-yd/🧬️schema/🟦️.ts";
import { type ChangeReinforced, parseChangeReinforced } from "./🔩change-reinforced/🧬️schema/🟦️.ts";
import { type ChangeBedJointThickness, parseChangeBedJointThickness } from "./🥪️change-bed-joint-thickness/🧬️schema/🟦️.ts";
import { type ChangeFm, parseChangeFm } from "./🧈change-fm/🧬️schema/🟦️.ts";
import { type ChangeMortarClass, parseChangeMortarClass } from "./🧈change-mortar-class/🧬️schema/🟦️.ts";
import { type ChangeMortarType, parseChangeMortarType } from "./🧈change-mortar-type/🧬️schema/🟦️.ts";
import { type ChangeCPe, parseChangeCPe } from "./🧮change-c-pe/🧬️schema/🟦️.ts";
import { type ChangeDensity, parseChangeDensity } from "./🧱change-density/🧬️schema/🟦️.ts";
import { type ChangeSupportSides, parseChangeSupportSides } from "./🧱change-support-sides/🧬️schema/🟦️.ts";
import { type ChangeUnitFb, parseChangeUnitFb } from "./🧱change-unit-fb/🧬️schema/🟦️.ts";
import { type ChangeUnitGroup, parseChangeUnitGroup } from "./🧱change-unit-group/🧬️schema/🟦️.ts";
import { type ChangeUnitHeight, parseChangeUnitHeight } from "./🧱change-unit-height/🧬️schema/🟦️.ts";
import { type ChangeUnitLength, parseChangeUnitLength } from "./🧱change-unit-length/🧬️schema/🟦️.ts";
import { type ChangeUnitMaterial, parseChangeUnitMaterial } from "./🧱change-unit-material/🧬️schema/🟦️.ts";
import { type ChangeUnitWidth, parseChangeUnitWidth } from "./🧱change-unit-width/🧬️schema/🟦️.ts";
import { type ChangeWallType, parseChangeWallType } from "./🧱change-wall-type/🧬️schema/🟦️.ts";
import { type ChangeMu, parseChangeMu } from "./🧲️change-mu/🧬️schema/🟦️.ts";
import { type ChangeOpeningHeight, parseChangeOpeningHeight } from "./🪟change-opening-height/🧬️schema/🟦️.ts";
import { type ChangeOpeningSill, parseChangeOpeningSill } from "./🪟change-opening-sill/🧬️schema/🟦️.ts";
import { type ChangeOpeningWidth, parseChangeOpeningWidth } from "./🪟change-opening-width/🧬️schema/🟦️.ts";
import { type ChangeHKEarth, parseChangeHKEarth } from "./🪨change-hk-earth/🧬️schema/🟦️.ts";

export type En1996Mutation =
  | { ChangeConcentratedBearingLength: ChangeConcentratedBearingLength }
  | { ChangeSlabSpan: ChangeSlabSpan }
  | { ChangeWallLength: ChangeWallLength }
  | { ChangeWallHeight: ChangeWallHeight }
  | { ChangeWallThickness: ChangeWallThickness }
  | { ChangeEccentricityBottom: ChangeEccentricityBottom }
  | { ChangeEccentricityTop: ChangeEccentricityTop }
  | { ChangePhiInfinity: ChangePhiInfinity }
  | { ChangeQKSnow: ChangeQKSnow }
  | { InsertConcentrated: InsertConcentrated }
  | { InsertLoadCase: InsertLoadCase }
  | { InsertOpening: InsertOpening }
  | { InsertWall: InsertWall }
  | { RemoveConcentrated: RemoveConcentrated }
  | { RemoveLoadCase: RemoveLoadCase }
  | { RemoveOpening: RemoveOpening }
  | { RemoveWall: RemoveWall }
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeQPWind: ChangeQPWind }
  | { ChangeDesignSituation: ChangeDesignSituation }
  | { ChangeLoadCaseSituation: ChangeLoadCaseSituation }
  | { ChangeConcentratedForce: ChangeConcentratedForce }
  | { ChangeGKSlab: ChangeGKSlab }
  | { ChangeQKImposed: ChangeQKImposed }
  | { ChangeIsBasement: ChangeIsBasement }
  | { ChangeStoreys: ChangeStoreys }
  | { ChangeMasonryClass: ChangeMasonryClass }
  | { ChangeImposedCategory: ChangeImposedCategory }
  | { ChangeWallLabelDe: ChangeWallLabelDe }
  | { ChangeWallLabelEn: ChangeWallLabelEn }
  | { ChangeExposure: ChangeExposure }
  | { ChangeConcentratedBearingArea: ChangeConcentratedBearingArea }
  | { ChangeSlabBearingDepth: ChangeSlabBearingDepth }
  | { ChangeTributaryArea: ChangeTributaryArea }
  | { ChangeFireRei: ChangeFireRei }
  | { ChangeAsHorizontal: ChangeAsHorizontal }
  | { ChangeAsVertical: ChangeAsVertical }
  | { ChangeFYd: ChangeFYd }
  | { ChangeReinforced: ChangeReinforced }
  | { ChangeBedJointThickness: ChangeBedJointThickness }
  | { ChangeFm: ChangeFm }
  | { ChangeMortarClass: ChangeMortarClass }
  | { ChangeMortarType: ChangeMortarType }
  | { ChangeCPe: ChangeCPe }
  | { ChangeDensity: ChangeDensity }
  | { ChangeSupportSides: ChangeSupportSides }
  | { ChangeUnitFb: ChangeUnitFb }
  | { ChangeUnitGroup: ChangeUnitGroup }
  | { ChangeUnitHeight: ChangeUnitHeight }
  | { ChangeUnitLength: ChangeUnitLength }
  | { ChangeUnitMaterial: ChangeUnitMaterial }
  | { ChangeUnitWidth: ChangeUnitWidth }
  | { ChangeWallType: ChangeWallType }
  | { ChangeMu: ChangeMu }
  | { ChangeOpeningHeight: ChangeOpeningHeight }
  | { ChangeOpeningSill: ChangeOpeningSill }
  | { ChangeOpeningWidth: ChangeOpeningWidth }
  | { ChangeHKEarth: ChangeHKEarth };

export const parseEn1996Mutation: NormWireReader<En1996Mutation> = normWireExternal<En1996Mutation>({
  ChangeConcentratedBearingLength: parseChangeConcentratedBearingLength,
  ChangeSlabSpan: parseChangeSlabSpan,
  ChangeWallLength: parseChangeWallLength,
  ChangeWallHeight: parseChangeWallHeight,
  ChangeWallThickness: parseChangeWallThickness,
  ChangeEccentricityBottom: parseChangeEccentricityBottom,
  ChangeEccentricityTop: parseChangeEccentricityTop,
  ChangePhiInfinity: parseChangePhiInfinity,
  ChangeQKSnow: parseChangeQKSnow,
  InsertConcentrated: parseInsertConcentrated,
  InsertLoadCase: parseInsertLoadCase,
  InsertOpening: parseInsertOpening,
  InsertWall: parseInsertWall,
  RemoveConcentrated: parseRemoveConcentrated,
  RemoveLoadCase: parseRemoveLoadCase,
  RemoveOpening: parseRemoveOpening,
  RemoveWall: parseRemoveWall,
  ChangeAnnex: parseChangeAnnex,
  ChangeQPWind: parseChangeQPWind,
  ChangeDesignSituation: parseChangeDesignSituation,
  ChangeLoadCaseSituation: parseChangeLoadCaseSituation,
  ChangeConcentratedForce: parseChangeConcentratedForce,
  ChangeGKSlab: parseChangeGKSlab,
  ChangeQKImposed: parseChangeQKImposed,
  ChangeIsBasement: parseChangeIsBasement,
  ChangeStoreys: parseChangeStoreys,
  ChangeMasonryClass: parseChangeMasonryClass,
  ChangeImposedCategory: parseChangeImposedCategory,
  ChangeWallLabelDe: parseChangeWallLabelDe,
  ChangeWallLabelEn: parseChangeWallLabelEn,
  ChangeExposure: parseChangeExposure,
  ChangeConcentratedBearingArea: parseChangeConcentratedBearingArea,
  ChangeSlabBearingDepth: parseChangeSlabBearingDepth,
  ChangeTributaryArea: parseChangeTributaryArea,
  ChangeFireRei: parseChangeFireRei,
  ChangeAsHorizontal: parseChangeAsHorizontal,
  ChangeAsVertical: parseChangeAsVertical,
  ChangeFYd: parseChangeFYd,
  ChangeReinforced: parseChangeReinforced,
  ChangeBedJointThickness: parseChangeBedJointThickness,
  ChangeFm: parseChangeFm,
  ChangeMortarClass: parseChangeMortarClass,
  ChangeMortarType: parseChangeMortarType,
  ChangeCPe: parseChangeCPe,
  ChangeDensity: parseChangeDensity,
  ChangeSupportSides: parseChangeSupportSides,
  ChangeUnitFb: parseChangeUnitFb,
  ChangeUnitGroup: parseChangeUnitGroup,
  ChangeUnitHeight: parseChangeUnitHeight,
  ChangeUnitLength: parseChangeUnitLength,
  ChangeUnitMaterial: parseChangeUnitMaterial,
  ChangeUnitWidth: parseChangeUnitWidth,
  ChangeWallType: parseChangeWallType,
  ChangeMu: parseChangeMu,
  ChangeOpeningHeight: parseChangeOpeningHeight,
  ChangeOpeningSill: parseChangeOpeningSill,
  ChangeOpeningWidth: parseChangeOpeningWidth,
  ChangeHKEarth: parseChangeHKEarth,
});
