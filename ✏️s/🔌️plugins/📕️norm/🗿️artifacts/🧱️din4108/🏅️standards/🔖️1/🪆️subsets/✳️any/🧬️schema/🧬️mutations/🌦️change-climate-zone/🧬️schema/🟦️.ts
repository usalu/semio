/** 🌦️ `change-climate-zone` wire twin: the leaf payload `ChangeClimateZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ClimateZoneDe, parseDin4108ClimateZoneDe } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeClimateZone {
  newClimateZone: Din4108ClimateZoneDe;
}

export const parseChangeClimateZone: NormWireReader<ChangeClimateZone> = normWireObject<ChangeClimateZone>({ newClimateZone: normWireRequired(parseDin4108ClimateZoneDe) });
