/** ✂️ `change-beam-steel-shear-area-m2` wire twin: the leaf payload `ChangeBeamSteelShearAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelShearAreaM2 {
  id: string;
  newAVM2: number;
}

export const parseChangeBeamSteelShearAreaM2: NormWireReader<ChangeBeamSteelShearAreaM2> = normWireObject<ChangeBeamSteelShearAreaM2>({ id: normWireRequired(normWireString), newAVM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
