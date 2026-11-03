/** 🧵️ `change-weld-run-member-id` wire twin: the leaf payload `ChangeWeldRunMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunMemberId {
  mutation: "changeWeldRunMemberId";
  weldRunId: string;
  newMemberId: string;
}

export const parseChangeWeldRunMemberId: NormWireReader<ChangeWeldRunMemberId> = normWireObject<ChangeWeldRunMemberId>({ mutation: normWireRequired(normWireLiteral("changeWeldRunMemberId")), weldRunId: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
