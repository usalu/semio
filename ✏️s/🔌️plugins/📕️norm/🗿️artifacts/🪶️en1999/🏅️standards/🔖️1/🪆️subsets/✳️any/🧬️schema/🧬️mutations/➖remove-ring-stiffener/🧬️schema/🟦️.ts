/** 🗂️ `remove-ring-stiffener` wire twin: the leaf payload `RemoveRingStiffener`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRingStiffener {
  mutation: "removeRingStiffener";
  id: string;
}

export const parseRemoveRingStiffener: NormWireReader<RemoveRingStiffener> = normWireObject<RemoveRingStiffener>({ mutation: normWireRequired(normWireLiteral("removeRingStiffener")), id: normWireRequired(normWireString) });
