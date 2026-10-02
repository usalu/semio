/** 🔺️ `Din4108Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireBoolean, normWireJson, normWireMap, normWireNumber, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din4108Diff {
  climateZone?: string;
  usage?: string;
  tIntC?: number;
  rhInt?: number;
  hasMechanicalVentilation?: boolean;
  airtightnessN50?: number;
  bb2DetailsConform?: boolean;
  zones?: { [key: string]: NormJson };
  elements?: { [key: string]: NormJson };
  thermalBridges?: { [key: string]: NormJson };
}

export const parseDin4108Diff: NormWireReader<Din4108Diff> = normWireObject<Din4108Diff>({ climateZone: normWireOptional(normWireString), usage: normWireOptional(normWireString), tIntC: normWireOptional(normWireNumber), rhInt: normWireOptional(normWireNumber), hasMechanicalVentilation: normWireOptional(normWireBoolean), airtightnessN50: normWireOptional(normWireNumber), bb2DetailsConform: normWireOptional(normWireBoolean), zones: normWireOptional(normWireMap(normWireJson)), elements: normWireOptional(normWireMap(normWireJson)), thermalBridges: normWireOptional(normWireMap(normWireJson)) }, false);
