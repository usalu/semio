/** 🧬️ En1997Mutation — mirrors Rust enum (semantic verbs). */
import type { AnnexChoice, SoilLayer, SpreadFoundation, Pile } from "../🟦️.ts";

export type En1997Mutation =
  | { mutation: "changeAnnex"; newAnnex: AnnexChoice }
  | { mutation: "changeGeotechnicalCategory"; newGeotechnicalCategory: number }
  | { mutation: "changeDesignSituation"; newDesignSituation: string }
  | { mutation: "changeDesignApproach"; newDesignApproach: string }
  | { mutation: "changeGroundwaterLevel"; newGroundwaterLevel: number }
  | { mutation: "changeInvestigationDepth"; newInvestigationDepth: number }
  | { mutation: "changeFootingWidth"; id: string; newWidth: number }
  | { mutation: "changeFootingEmbedment"; id: string; newEmbedment: number }
  | { mutation: "changePileLength"; id: string; newLength: number }
  | { mutation: "changePileCount"; id: string; newCount: number }
  | { mutation: "changeWallBaseWidth"; id: string; newBaseWidth: number }
  | { mutation: "changeSlopeAngle"; id: string; newAngleDeg: number }
  | { mutation: "changeLayerPhiPrime"; id: string; newPhiPrimeDeg: number }
  | { mutation: "changeLayerOedometricModulus"; id: string; newOedometricModulus: number }
  | { mutation: "insertLayer"; index: number; layer: SoilLayer }
  | { mutation: "removeLayer"; index: number }
  | { mutation: "insertFooting"; index: number; footing: SpreadFoundation }
  | { mutation: "removeFooting"; index: number }
  | { mutation: "insertPile"; index: number; pile: Pile }
  | { mutation: "removePile"; index: number };
