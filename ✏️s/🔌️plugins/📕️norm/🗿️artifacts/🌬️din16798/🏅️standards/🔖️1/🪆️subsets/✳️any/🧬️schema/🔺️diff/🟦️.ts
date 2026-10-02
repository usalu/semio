/** 🔺️ `Din16798Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireJson, normWireMap, normWireNumber, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din16798Diff {
  artifact?: { [key: string]: NormJson };
  annex?: string;
  thetaRmC?: number;
  outdoorCo2Ppm?: number;
  zones?: { [key: string]: NormJson };
  ventSystems?: { [key: string]: NormJson };
  envelopeN50HInv?: number;
  envelopeVolumeM3?: number;
  cellarAreaM2?: number;
  cellarVentilationM3H?: number;
  nightSetbackK?: number;
}

export const parseDin16798Diff: NormWireReader<Din16798Diff> = normWireObject<Din16798Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), thetaRmC: normWireOptional(normWireNumber), outdoorCo2Ppm: normWireOptional(normWireNumber), zones: normWireOptional(normWireMap(normWireJson)), ventSystems: normWireOptional(normWireMap(normWireJson)), envelopeN50HInv: normWireOptional(normWireNumber), envelopeVolumeM3: normWireOptional(normWireNumber), cellarAreaM2: normWireOptional(normWireNumber), cellarVentilationM3H: normWireOptional(normWireNumber), nightSetbackK: normWireOptional(normWireNumber) }, false);
