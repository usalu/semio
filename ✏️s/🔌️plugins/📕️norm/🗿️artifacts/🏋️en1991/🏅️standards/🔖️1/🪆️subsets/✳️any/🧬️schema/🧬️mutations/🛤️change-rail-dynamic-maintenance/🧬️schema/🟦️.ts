/** 🛤 `change-rail-dynamic-maintenance` wire twin: the leaf payload `ChangeRailDynamicMaintenance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDynamicMaintenance {
  id: string;
  newMaintenance: string;
}

export const parseChangeRailDynamicMaintenance: NormWireReader<ChangeRailDynamicMaintenance> = normWireObject<ChangeRailDynamicMaintenance>({ id: normWireRequired(normWireString), newMaintenance: normWireRequired(normWireString) });
