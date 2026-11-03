/** ⭕️ `change-ring-x` wire twin: the leaf payload `ChangeRingX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRingX {
  mutation: "changeRingX";
  ringStiffenerId: string;
  newX: number;
}

export const parseChangeRingX: NormWireReader<ChangeRingX> = normWireObject<ChangeRingX>({ mutation: normWireRequired(normWireLiteral("changeRingX")), ringStiffenerId: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
