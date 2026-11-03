/** 🌀️ `change-beam-action-q-area-pa` wire twin: the leaf payload `ChangeBeamActionQAreaPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamActionQAreaPa {
  beamId: string;
  id: string;
  newQAreaPa: number;
}

export const parseChangeBeamActionQAreaPa: NormWireReader<ChangeBeamActionQAreaPa> = normWireObject<ChangeBeamActionQAreaPa>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQAreaPa: normWireRequired(normWireNumber) });
