/** 🔷 `change-element-tilt-deg` wire twin: the leaf payload `ChangeElementTiltDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementTiltDeg {
  mutation: "changeelementtiltdeg";
  elementId: string;
  newTiltDeg: number;
}

export const parseChangeElementTiltDeg: NormWireReader<ChangeElementTiltDeg> = normWireObject<ChangeElementTiltDeg>({ mutation: normWireRequired(normWireLiteral("changeelementtiltdeg")), elementId: normWireRequired(normWireString), newTiltDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":180})) });
