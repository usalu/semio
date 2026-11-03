/** 🌉️ `change-bridge-phi2` wire twin: the leaf payload `ChangeBridgePhi2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePhi2 {
  id: string;
  newPhi2: number;
}

export const parseChangeBridgePhi2: NormWireReader<ChangeBridgePhi2> = normWireObject<ChangeBridgePhi2>({ id: normWireRequired(normWireString), newPhi2: normWireRequired(normWireNumber) });
