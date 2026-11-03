/** 📍️ `change-beam-action-point-force-n` wire twin: the leaf payload `ChangeBeamActionPointForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamActionPointForceN {
  beamId: string;
  id: string;
  newFKN: number;
}

export const parseChangeBeamActionPointForceN: NormWireReader<ChangeBeamActionPointForceN> = normWireObject<ChangeBeamActionPointForceN>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString), newFKN: normWireRequired(normWireNumber) });
