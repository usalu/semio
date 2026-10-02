/** 🧺️ `En1997Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeFootingWidth, parseChangeFootingWidth } from "./↔️change-footing-width/🧬️schema/🟦️.ts";
import { type ChangeSlopeAngle, parseChangeSlopeAngle } from "./⛰️change-slope-angle/🧬️schema/🟦️.ts";
import { type InsertFooting, parseInsertFooting } from "./➕insert-footing/🧬️schema/🟦️.ts";
import { type InsertLayer, parseInsertLayer } from "./➕️insert-layer/🧬️schema/🟦️.ts";
import { parseRemoveFooting, type RemoveFooting } from "./➖remove-footing/🧬️schema/🟦️.ts";
import { parseRemoveLayer, type RemoveLayer } from "./➖️remove-layer/🧬️schema/🟦️.ts";
import { type ChangeFootingEmbedment, parseChangeFootingEmbedment } from "./⬇️change-footing-embedment/🧬️schema/🟦️.ts";
import { type ChangeLayerOedometricModulus, parseChangeLayerOedometricModulus } from "./🌀️change-layer-oedometric-modulus/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeGroundwaterLevel, parseChangeGroundwaterLevel } from "./💧change-groundwater-level/🧬️schema/🟦️.ts";
import { type ChangeDesignSituation, parseChangeDesignSituation } from "./📅️change-design-situation/🧬️schema/🟦️.ts";
import { type ChangePileLength, parseChangePileLength } from "./📏️change-pile-length/🧬️schema/🟦️.ts";
import { type ChangeLayerPhiPrime, parseChangeLayerPhiPrime } from "./📐️change-layer-phi-prime/🧬️schema/🟦️.ts";
import { parseRemovePile, type RemovePile } from "./📤remove-pile/🧬️schema/🟦️.ts";
import { type InsertPile, parseInsertPile } from "./📥insert-pile/🧬️schema/🟦️.ts";
import { type ChangeInvestigationDepth, parseChangeInvestigationDepth } from "./🔎️change-investigation-depth/🧬️schema/🟦️.ts";
import { type ChangePileCount, parseChangePileCount } from "./🔢change-pile-count/🧬️schema/🟦️.ts";
import { type ChangeGeotechnicalCategory, parseChangeGeotechnicalCategory } from "./🗂️change-geotechnical-category/🧬️schema/🟦️.ts";
import { type ChangeDesignApproach, parseChangeDesignApproach } from "./🧭️change-design-approach/🧬️schema/🟦️.ts";
import { type ChangeWallBaseWidth, parseChangeWallBaseWidth } from "./🧱change-wall-base-width/🧬️schema/🟦️.ts";

export type En1997Mutation =
  | ChangeAnnex
  | ChangeGeotechnicalCategory
  | ChangeDesignSituation
  | ChangeDesignApproach
  | ChangeGroundwaterLevel
  | ChangeInvestigationDepth
  | ChangeFootingWidth
  | ChangeFootingEmbedment
  | ChangePileLength
  | ChangePileCount
  | ChangeWallBaseWidth
  | ChangeSlopeAngle
  | ChangeLayerPhiPrime
  | ChangeLayerOedometricModulus
  | InsertLayer
  | RemoveLayer
  | InsertFooting
  | RemoveFooting
  | InsertPile
  | RemovePile;

export const parseEn1997Mutation: NormWireReader<En1997Mutation> = normWireTagged<En1997Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  changeGeotechnicalCategory: parseChangeGeotechnicalCategory,
  changeDesignSituation: parseChangeDesignSituation,
  changeDesignApproach: parseChangeDesignApproach,
  changeGroundwaterLevel: parseChangeGroundwaterLevel,
  changeInvestigationDepth: parseChangeInvestigationDepth,
  changeFootingWidth: parseChangeFootingWidth,
  changeFootingEmbedment: parseChangeFootingEmbedment,
  changePileLength: parseChangePileLength,
  changePileCount: parseChangePileCount,
  changeWallBaseWidth: parseChangeWallBaseWidth,
  changeSlopeAngle: parseChangeSlopeAngle,
  changeLayerPhiPrime: parseChangeLayerPhiPrime,
  changeLayerOedometricModulus: parseChangeLayerOedometricModulus,
  insertLayer: parseInsertLayer,
  removeLayer: parseRemoveLayer,
  insertFooting: parseInsertFooting,
  removeFooting: parseRemoveFooting,
  insertPile: parseInsertPile,
  removePile: parseRemovePile,
});
