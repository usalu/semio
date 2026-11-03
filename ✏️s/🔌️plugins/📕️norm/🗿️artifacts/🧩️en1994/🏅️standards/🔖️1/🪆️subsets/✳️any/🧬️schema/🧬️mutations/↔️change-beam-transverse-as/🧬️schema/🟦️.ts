/** ↔️ `change-beam-transverse-as` wire twin: the leaf payload `ChangeBeamTransverseAs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamTransverseAs {
  id: string;
  newTransverseAsM2PerM: number;
}

export const parseChangeBeamTransverseAs: NormWireReader<ChangeBeamTransverseAs> = normWireObject<ChangeBeamTransverseAs>({ id: normWireRequired(normWireString), newTransverseAsM2PerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
