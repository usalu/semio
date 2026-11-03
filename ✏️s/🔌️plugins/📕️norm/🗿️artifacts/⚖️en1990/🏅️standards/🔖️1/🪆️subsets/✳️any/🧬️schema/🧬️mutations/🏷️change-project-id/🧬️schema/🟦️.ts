/** 🏷️ `change-project-id` wire twin: the leaf payload `ChangeProjectId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeProjectId {
  mutation: "changeProjectId";
  newProjectId: string;
}

export const parseChangeProjectId: NormWireReader<ChangeProjectId> = normWireObject<ChangeProjectId>({ mutation: normWireRequired(normWireLiteral("changeProjectId")), newProjectId: normWireRequired(normWireString) });
