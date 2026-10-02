/** ⛰️ `change-altitude-m` wire twin: the leaf payload `ChangeAltitudeM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAltitudeM {
  mutation: "changeAltitudeM";
  newAltitudeM: number;
}

export const parseChangeAltitudeM: NormWireReader<ChangeAltitudeM> = normWireObject<ChangeAltitudeM>({ mutation: normWireRequired(normWireLiteral("changeAltitudeM")), newAltitudeM: normWireRequired(normWireNumber) });
