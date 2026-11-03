/** 🦴️ `change-transverse-position` wire twin: the leaf payload `ChangeTransversePosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTransversePosition {
  mutation: "changeTransversePosition";
  transverseStiffenerId: string;
  newPosition: number;
}

export const parseChangeTransversePosition: NormWireReader<ChangeTransversePosition> = normWireObject<ChangeTransversePosition>({ mutation: normWireRequired(normWireLiteral("changeTransversePosition")), transverseStiffenerId: normWireRequired(normWireString), newPosition: normWireRequired(normWireNumber) });
