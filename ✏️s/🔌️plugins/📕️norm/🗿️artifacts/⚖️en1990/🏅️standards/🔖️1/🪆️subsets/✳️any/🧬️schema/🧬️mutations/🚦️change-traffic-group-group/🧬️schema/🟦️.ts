/** 🚦 `change-traffic-group-group` wire twin: the leaf payload `ChangeTrafficGroupGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrafficGroupGroup {
  mutation: "changeTrafficGroupGroup";
  id: string;
  newGroup: "GR1A" | "GR1B" | "GR2" | "GR3" | "GR4" | "GR5" | "GR6";
}

export const parseChangeTrafficGroupGroup: NormWireReader<ChangeTrafficGroupGroup> = normWireObject<ChangeTrafficGroupGroup>({ mutation: normWireRequired(normWireLiteral("changeTrafficGroupGroup")), id: normWireRequired(normWireString), newGroup: normWireRequired(normWireLiteral("GR1A", "GR1B", "GR2", "GR3", "GR4", "GR5", "GR6")) });
