/** 🧵️ `change-weld-run-element-id` wire twin: the leaf payload `ChangeWeldRunElementId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunElementId {
  mutation: "changeWeldRunElementId";
  weldRunId: string;
  newElementId: string;
}

export const parseChangeWeldRunElementId: NormWireReader<ChangeWeldRunElementId> = normWireObject<ChangeWeldRunElementId>({ mutation: normWireRequired(normWireLiteral("changeWeldRunElementId")), weldRunId: normWireRequired(normWireString), newElementId: normWireRequired(normWireString) });
