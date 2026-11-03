/** 🏗️ `change-structure-id` wire twin: the leaf payload `ChangeStructureId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStructureId {
  mutation: "changeStructureId";
  newStructureId: string;
}

export const parseChangeStructureId: NormWireReader<ChangeStructureId> = normWireObject<ChangeStructureId>({ mutation: normWireRequired(normWireLiteral("changeStructureId")), newStructureId: normWireRequired(normWireString) });
