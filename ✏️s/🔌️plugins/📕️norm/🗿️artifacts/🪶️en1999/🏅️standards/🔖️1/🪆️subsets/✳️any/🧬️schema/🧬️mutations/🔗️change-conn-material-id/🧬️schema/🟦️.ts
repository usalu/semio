/** 🔗️ `change-conn-material-id` wire twin: the leaf payload `ChangeConnMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnMaterialId {
  mutation: "changeConnMaterialId";
  connectionId: string;
  newMaterialId: string;
}

export const parseChangeConnMaterialId: NormWireReader<ChangeConnMaterialId> = normWireObject<ChangeConnMaterialId>({ mutation: normWireRequired(normWireLiteral("changeConnMaterialId")), connectionId: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
