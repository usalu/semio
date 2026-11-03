/** 🗂️ `remove-transverse-stiffener` wire twin: the leaf payload `RemoveTransverseStiffener`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTransverseStiffener {
  mutation: "removeTransverseStiffener";
  id: string;
}

export const parseRemoveTransverseStiffener: NormWireReader<RemoveTransverseStiffener> = normWireObject<RemoveTransverseStiffener>({ mutation: normWireRequired(normWireLiteral("removeTransverseStiffener")), id: normWireRequired(normWireString) });
