/** 🔶 `change-element-orientation-deg` wire twin: the leaf payload `ChangeElementOrientationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementOrientationDeg {
  mutation: "changeelementorientationdeg";
  elementId: string;
  newOrientationDeg: number;
}

export const parseChangeElementOrientationDeg: NormWireReader<ChangeElementOrientationDeg> = normWireObject<ChangeElementOrientationDeg>({ mutation: normWireRequired(normWireLiteral("changeelementorientationdeg")), elementId: normWireRequired(normWireString), newOrientationDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":360})) });
