/** ⭕️ `change-ring-thickness` wire twin: the leaf payload `ChangeRingThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRingThickness {
  mutation: "changeRingThickness";
  ringStiffenerId: string;
  newThickness: number;
}

export const parseChangeRingThickness: NormWireReader<ChangeRingThickness> = normWireObject<ChangeRingThickness>({ mutation: normWireRequired(normWireLiteral("changeRingThickness")), ringStiffenerId: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
