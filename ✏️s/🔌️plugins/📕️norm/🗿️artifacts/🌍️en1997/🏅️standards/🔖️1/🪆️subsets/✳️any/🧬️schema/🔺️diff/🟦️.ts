/** 🔺️ `En1997Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireDefault, normWireInteger, normWireJson, normWireMap, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1997Diff {
  artifact: { [key: string]: NormJson } | null;
  structureId: string | null;
  geotechnicalCategory: number | null;
  designSituation: string | null;
  designApproach: string | null;
  annex: string | null;
  groundwaterLevel: number | null;
  investigationDepth: number | null;
  layers: { [key: string]: NormJson } | null;
  footings: { [key: string]: NormJson } | null;
  piles: { [key: string]: NormJson } | null;
  retainingWalls: { [key: string]: NormJson } | null;
  slopes: { [key: string]: NormJson } | null;
  upliftCases: { [key: string]: NormJson } | null;
}

export const parseEn1997Diff: NormWireReader<En1997Diff> = normWireObject<En1997Diff>({ artifact: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), structureId: normWireDefault(normWireNullable(normWireString), () => null), geotechnicalCategory: normWireDefault(normWireNullable(normWireInteger), () => null), designSituation: normWireDefault(normWireNullable(normWireString), () => null), designApproach: normWireDefault(normWireNullable(normWireString), () => null), annex: normWireDefault(normWireNullable(normWireString), () => null), groundwaterLevel: normWireDefault(normWireNullable(normWireNumber), () => null), investigationDepth: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), footings: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), piles: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), retainingWalls: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), slopes: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), upliftCases: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null) });
