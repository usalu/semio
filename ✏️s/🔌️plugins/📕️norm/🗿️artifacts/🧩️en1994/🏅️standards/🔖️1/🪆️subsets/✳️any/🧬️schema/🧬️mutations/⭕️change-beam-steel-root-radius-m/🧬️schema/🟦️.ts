/** ✂️ `change-beam-steel-root-radius-m` wire twin: the leaf payload `ChangeBeamSteelRootRadiusM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelRootRadiusM {
  id: string;
  newRM: number;
}

export const parseChangeBeamSteelRootRadiusM: NormWireReader<ChangeBeamSteelRootRadiusM> = normWireObject<ChangeBeamSteelRootRadiusM>({ id: normWireRequired(normWireString), newRM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
