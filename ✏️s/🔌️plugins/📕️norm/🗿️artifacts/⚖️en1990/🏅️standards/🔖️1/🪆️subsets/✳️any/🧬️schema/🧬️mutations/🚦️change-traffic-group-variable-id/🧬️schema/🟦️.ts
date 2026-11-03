/** 🚦 `change-traffic-group-variable-id` wire twin: the leaf payload `ChangeTrafficGroupVariableId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrafficGroupVariableId {
  mutation: "changeTrafficGroupVariableId";
  id: string;
  newVariableId: string;
}

export const parseChangeTrafficGroupVariableId: NormWireReader<ChangeTrafficGroupVariableId> = normWireObject<ChangeTrafficGroupVariableId>({ mutation: normWireRequired(normWireLiteral("changeTrafficGroupVariableId")), id: normWireRequired(normWireString), newVariableId: normWireRequired(normWireString) });
