/** 🔺️ `En1997Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AnnexChoice, parseAnnexChoice, parsePile, parseRetainingWall, parseSlope, parseSoilLayer, parseSpreadFoundation, parseUpliftCase, type Pile, type RetainingWall, type Slope, type SoilLayer, type SpreadFoundation, type UpliftCase } from "../📸️snapshot/🟦️.ts";
import { type En1997Artifact, parseEn1997Artifact } from "../🟦️.ts";

export interface En1997Diff {
  /** @state artifact */
  artifact: En1997Artifact | null;
  /** @state artifact */
  structureId: string | null;
  /** @state artifact */
  geotechnicalCategory: number | null;
  /** @state artifact */
  designSituation: string | null;
  /** @state artifact */
  designApproach: string | null;
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  groundwaterLevel: number | null;
  /** @state artifact */
  investigationDepth: number | null;
  /** @state artifact */
  layers: { values: SoilLayer[]; } | null;
  /** @state artifact */
  footings: { values: SpreadFoundation[]; } | null;
  /** @state artifact */
  piles: { values: Pile[]; } | null;
  /** @state artifact */
  retainingWalls: { values: RetainingWall[]; } | null;
  /** @state artifact */
  slopes: { values: Slope[]; } | null;
  /** @state artifact */
  upliftCases: { values: UpliftCase[]; } | null;
}

export const parseEn1997Diff: NormWireReader<En1997Diff> = normWireObject<En1997Diff>({ artifact: normWireDefault(normWireNullable(parseEn1997Artifact), () => null), structureId: normWireDefault(normWireNullable(normWireString), () => null), geotechnicalCategory: normWireDefault(normWireNullable(normWireInteger), () => null), designSituation: normWireDefault(normWireNullable(normWireString), () => null), designApproach: normWireDefault(normWireNullable(normWireString), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), groundwaterLevel: normWireDefault(normWireNullable(normWireNumber), () => null), investigationDepth: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireNullable(normWireObject<{ values: SoilLayer[]; }>({ values: normWireRequired(normWireArray(parseSoilLayer)) })), () => null), footings: normWireDefault(normWireNullable(normWireObject<{ values: SpreadFoundation[]; }>({ values: normWireRequired(normWireArray(parseSpreadFoundation)) })), () => null), piles: normWireDefault(normWireNullable(normWireObject<{ values: Pile[]; }>({ values: normWireRequired(normWireArray(parsePile)) })), () => null), retainingWalls: normWireDefault(normWireNullable(normWireObject<{ values: RetainingWall[]; }>({ values: normWireRequired(normWireArray(parseRetainingWall)) })), () => null), slopes: normWireDefault(normWireNullable(normWireObject<{ values: Slope[]; }>({ values: normWireRequired(normWireArray(parseSlope)) })), () => null), upliftCases: normWireDefault(normWireNullable(normWireObject<{ values: UpliftCase[]; }>({ values: normWireRequired(normWireArray(parseUpliftCase)) })), () => null) });
