/** ↔️ `change-beam-steel-width-m` wire twin: the leaf payload `ChangeBeamSteelWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelWidthM {
  id: string;
  newWidthM: number;
}

export const parseChangeBeamSteelWidthM: NormWireReader<ChangeBeamSteelWidthM> = normWireObject<ChangeBeamSteelWidthM>({ id: normWireRequired(normWireString), newWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
