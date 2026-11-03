/** 🦴️ `change-transverse-width` wire twin: the leaf payload `ChangeTransverseWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTransverseWidth {
  mutation: "changeTransverseWidth";
  transverseStiffenerId: string;
  newWidth: number;
}

export const parseChangeTransverseWidth: NormWireReader<ChangeTransverseWidth> = normWireObject<ChangeTransverseWidth>({ mutation: normWireRequired(normWireLiteral("changeTransverseWidth")), transverseStiffenerId: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
