/** 🧬️ `Din16798Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireJson, normWireNumber, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din16798Artifact {
  annex?: string;
  thetaRmC?: number;
  outdoorCo2Ppm?: number;
  zones?: NormJson[];
  ventSystems?: NormJson[];
  envelopeN50HInv?: number;
  envelopeVolumeM3?: number;
  cellarAreaM2?: number;
  cellarVentilationM3H?: number;
  nightSetbackK?: number;
}

export const parseDin16798Artifact: NormWireReader<Din16798Artifact> = normWireObject<Din16798Artifact>({ annex: normWireOptional(normWireString), thetaRmC: normWireOptional(normWireNumber), outdoorCo2Ppm: normWireOptional(normWireNumber), zones: normWireOptional(normWireArray(normWireJson)), ventSystems: normWireOptional(normWireArray(normWireJson)), envelopeN50HInv: normWireOptional(normWireNumber), envelopeVolumeM3: normWireOptional(normWireNumber), cellarAreaM2: normWireOptional(normWireNumber), cellarVentilationM3H: normWireOptional(normWireNumber), nightSetbackK: normWireOptional(normWireNumber) }, false);
