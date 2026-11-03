/** ↕️ `change-beam-end-plate-height-m` wire twin: the leaf payload `ChangeBeamEndPlateHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamEndPlateHeightM {
  id: string;
  newEndPlateHeightM: number;
}

export const parseChangeBeamEndPlateHeightM: NormWireReader<ChangeBeamEndPlateHeightM> = normWireObject<ChangeBeamEndPlateHeightM>({ id: normWireRequired(normWireString), newEndPlateHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
