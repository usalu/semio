/** ↕️ `change-beam-steel-height-m` wire twin: the leaf payload `ChangeBeamSteelHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelHeightM {
  id: string;
  newHeightM: number;
}

export const parseChangeBeamSteelHeightM: NormWireReader<ChangeBeamSteelHeightM> = normWireObject<ChangeBeamSteelHeightM>({ id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
