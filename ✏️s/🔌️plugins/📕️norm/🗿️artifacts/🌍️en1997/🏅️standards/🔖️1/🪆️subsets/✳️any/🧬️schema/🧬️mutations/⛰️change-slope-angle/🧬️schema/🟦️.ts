/** ⛰️ `change-slope-angle` wire twin: the leaf payload `ChangeSlopeAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSlopeAngle {
  mutation: "changeSlopeAngle";
  id: string;
  newAngleDeg: number;
}

export const parseChangeSlopeAngle: NormWireReader<ChangeSlopeAngle> = normWireObject<ChangeSlopeAngle>({ mutation: normWireRequired(normWireLiteral("changeSlopeAngle")), id: normWireRequired(normWireString), newAngleDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":90})) });
