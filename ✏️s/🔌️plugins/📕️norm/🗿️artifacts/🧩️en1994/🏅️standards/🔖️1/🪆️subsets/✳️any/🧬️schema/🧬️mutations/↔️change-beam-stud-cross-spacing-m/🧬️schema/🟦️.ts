/** ↔️ `change-beam-stud-cross-spacing-m` wire twin: the leaf payload `ChangeBeamStudCrossSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamStudCrossSpacingM {
  id: string;
  newStudTransverseSpacingM: number;
}

export const parseChangeBeamStudCrossSpacingM: NormWireReader<ChangeBeamStudCrossSpacingM> = normWireObject<ChangeBeamStudCrossSpacingM>({ id: normWireRequired(normWireString), newStudTransverseSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
