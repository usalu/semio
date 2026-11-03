/** ⭕️ `change-ring-depth` wire twin: the leaf payload `ChangeRingDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRingDepth {
  mutation: "changeRingDepth";
  ringStiffenerId: string;
  newDepth: number;
}

export const parseChangeRingDepth: NormWireReader<ChangeRingDepth> = normWireObject<ChangeRingDepth>({ mutation: normWireRequired(normWireLiteral("changeRingDepth")), ringStiffenerId: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
