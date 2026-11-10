/** ⬜ `change-element-area-m2` wire twin: the leaf payload `ChangeElementAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementAreaM2 {
  mutation: "changeelementaream2";
  elementId: string;
  newAreaM2: number;
}

export const parseChangeElementAreaM2: NormWireReader<ChangeElementAreaM2> = normWireObject<ChangeElementAreaM2>({ mutation: normWireRequired(normWireLiteral("changeelementaream2")), elementId: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireRange(normWireNumber, {"maximum":100000.0,"exclusiveMinimum":0.0})) });
