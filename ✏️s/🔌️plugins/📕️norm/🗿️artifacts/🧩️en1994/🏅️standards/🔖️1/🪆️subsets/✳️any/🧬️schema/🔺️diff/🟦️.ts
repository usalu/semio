/** 🔺️ `En1994Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireDefault, normWireJson, normWireMap, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1994Diff {
  artifact: { [key: string]: NormJson } | null;
  annex: string | null;
  structureKind: string | null;
  steelFYPa: number | null;
  beams: { [key: string]: NormJson } | null;
  columns: { [key: string]: NormJson } | null;
  slabs: { [key: string]: NormJson } | null;
  fireRating: string | null;
  insulationThicknessM: number | null;
  fatigueDetail: string | null;
}

export const parseEn1994Diff: NormWireReader<En1994Diff> = normWireObject<En1994Diff>({ artifact: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), annex: normWireDefault(normWireNullable(normWireString), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), steelFYPa: normWireDefault(normWireNullable(normWireNumber), () => null), beams: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), columns: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), slabs: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), fireRating: normWireDefault(normWireNullable(normWireString), () => null), insulationThicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), fatigueDetail: normWireDefault(normWireNullable(normWireString), () => null) });
