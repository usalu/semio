/** 🧱️ `change-beam-steel-tf-m` wire twin: the leaf payload `ChangeBeamSteelTfM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelTfM {
  id: string;
  newTfM: number;
}

export const parseChangeBeamSteelTfM: NormWireReader<ChangeBeamSteelTfM> = normWireObject<ChangeBeamSteelTfM>({ id: normWireRequired(normWireString), newTfM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
