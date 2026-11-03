/** 🗂️ `remove-material` wire twin: the leaf payload `RemoveMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMaterial {
  mutation: "removeMaterial";
  id: string;
}

export const parseRemoveMaterial: NormWireReader<RemoveMaterial> = normWireObject<RemoveMaterial>({ mutation: normWireRequired(normWireLiteral("removeMaterial")), id: normWireRequired(normWireString) });
