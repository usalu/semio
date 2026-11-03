/** ↔️ `change-beam-spacing-m` wire twin: the leaf payload `ChangeBeamSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSpacingM {
  id: string;
  newSpacingM: number;
}

export const parseChangeBeamSpacingM: NormWireReader<ChangeBeamSpacingM> = normWireObject<ChangeBeamSpacingM>({ id: normWireRequired(normWireString), newSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
