/** ⭕️ `change-ring-internal` wire twin: the leaf payload `ChangeRingInternal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRingInternal {
  mutation: "changeRingInternal";
  ringStiffenerId: string;
  newInternal: boolean;
}

export const parseChangeRingInternal: NormWireReader<ChangeRingInternal> = normWireObject<ChangeRingInternal>({ mutation: normWireRequired(normWireLiteral("changeRingInternal")), ringStiffenerId: normWireRequired(normWireString), newInternal: normWireRequired(normWireBoolean) });
