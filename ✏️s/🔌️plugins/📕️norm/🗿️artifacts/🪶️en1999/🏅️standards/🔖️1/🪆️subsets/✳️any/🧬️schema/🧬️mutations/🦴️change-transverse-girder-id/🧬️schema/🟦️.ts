/** 🦴️ `change-transverse-girder-id` wire twin: the leaf payload `ChangeTransverseGirderId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTransverseGirderId {
  mutation: "changeTransverseGirderId";
  transverseStiffenerId: string;
  newGirderId: string;
}

export const parseChangeTransverseGirderId: NormWireReader<ChangeTransverseGirderId> = normWireObject<ChangeTransverseGirderId>({ mutation: normWireRequired(normWireLiteral("changeTransverseGirderId")), transverseStiffenerId: normWireRequired(normWireString), newGirderId: normWireRequired(normWireString) });
