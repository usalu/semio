/** 🦴️ `change-transverse-thickness` wire twin: the leaf payload `ChangeTransverseThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTransverseThickness {
  mutation: "changeTransverseThickness";
  transverseStiffenerId: string;
  newThickness: number;
}

export const parseChangeTransverseThickness: NormWireReader<ChangeTransverseThickness> = normWireObject<ChangeTransverseThickness>({ mutation: normWireRequired(normWireLiteral("changeTransverseThickness")), transverseStiffenerId: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
