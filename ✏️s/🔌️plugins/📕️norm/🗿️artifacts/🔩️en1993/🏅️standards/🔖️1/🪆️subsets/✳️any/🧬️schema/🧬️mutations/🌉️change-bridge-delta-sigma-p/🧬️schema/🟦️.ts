/** 🌉️ `change-bridge-delta-sigma-p` wire twin: the leaf payload `ChangeBridgeDeltaSigmaP`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeltaSigmaP {
  id: string;
  newDeltaSigmaP: number;
}

export const parseChangeBridgeDeltaSigmaP: NormWireReader<ChangeBridgeDeltaSigmaP> = normWireObject<ChangeBridgeDeltaSigmaP>({ id: normWireRequired(normWireString), newDeltaSigmaP: normWireRequired(normWireNumber) });
