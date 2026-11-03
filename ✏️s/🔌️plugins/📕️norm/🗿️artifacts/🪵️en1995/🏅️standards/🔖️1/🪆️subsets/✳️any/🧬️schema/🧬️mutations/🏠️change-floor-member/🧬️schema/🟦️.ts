/** 🏠️ `change-floor-member` wire twin: the leaf payload `ChangeFloorMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorMember {
  floorId: string;
  newValue: string;
}

export const parseChangeFloorMember: NormWireReader<ChangeFloorMember> = normWireObject<ChangeFloorMember>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
