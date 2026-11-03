/** ➡️ `change-beam-end-zone-length-m` wire twin: the leaf payload `ChangeBeamEndZoneLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamEndZoneLengthM {
  id: string;
  newEndZoneLengthM: number;
}

export const parseChangeBeamEndZoneLengthM: NormWireReader<ChangeBeamEndZoneLengthM> = normWireObject<ChangeBeamEndZoneLengthM>({ id: normWireRequired(normWireString), newEndZoneLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
