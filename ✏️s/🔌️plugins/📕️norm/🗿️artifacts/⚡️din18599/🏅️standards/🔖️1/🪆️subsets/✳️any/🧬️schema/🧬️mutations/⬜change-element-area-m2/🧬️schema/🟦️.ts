/** ⬜ `change-element-area-m2` wire twin: the leaf payload `ChangeElementAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementAreaM2 {
  mutation: "changeelementaream2";
  elementId: string;
  newAreaM2: number;
}

export const parseChangeElementAreaM2: NormWireReader<ChangeElementAreaM2> = normWireObject<ChangeElementAreaM2>({ mutation: normWireRequired(normWireLiteral("changeelementaream2")), elementId: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireRange(normWireNumber, {"maximum":100000,"exclusiveMinimum":0})) });
