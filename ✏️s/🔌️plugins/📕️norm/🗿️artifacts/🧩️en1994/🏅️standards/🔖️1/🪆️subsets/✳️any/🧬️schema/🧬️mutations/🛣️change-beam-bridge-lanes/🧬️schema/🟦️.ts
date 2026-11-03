/** 🛣️ `change-beam-bridge-lanes` wire twin: the leaf payload `ChangeBeamBridgeLanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamBridgeLanes {
  id: string;
  newBridgeLanes: number;
}

export const parseChangeBeamBridgeLanes: NormWireReader<ChangeBeamBridgeLanes> = normWireObject<ChangeBeamBridgeLanes>({ id: normWireRequired(normWireString), newBridgeLanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
