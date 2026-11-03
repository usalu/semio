/** 🏢 `change-floor-assumed-qk-conc` wire twin: the leaf payload `ChangeFloorAssumedQkConc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorAssumedQkConc {
  id: string;
  newAssumedQkConcentrated: number;
}

export const parseChangeFloorAssumedQkConc: NormWireReader<ChangeFloorAssumedQkConc> = normWireObject<ChangeFloorAssumedQkConc>({ id: normWireRequired(normWireString), newAssumedQkConcentrated: normWireRequired(normWireNumber) });
