/** 🏢 `change-floor-assumed-qk` wire twin: the leaf payload `ChangeFloorAssumedQk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorAssumedQk {
  id: string;
  newAssumedQk: number;
}

export const parseChangeFloorAssumedQk: NormWireReader<ChangeFloorAssumedQk> = normWireObject<ChangeFloorAssumedQk>({ id: normWireRequired(normWireString), newAssumedQk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
