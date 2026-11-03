/** 🏋️ `change-beam-action-xm` wire twin: the leaf payload `ChangeBeamActionXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamActionXM {
  beamId: string;
  id: string;
  newXM: number;
}

export const parseChangeBeamActionXM: NormWireReader<ChangeBeamActionXM> = normWireObject<ChangeBeamActionXM>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString), newXM: normWireRequired(normWireNumber) });
