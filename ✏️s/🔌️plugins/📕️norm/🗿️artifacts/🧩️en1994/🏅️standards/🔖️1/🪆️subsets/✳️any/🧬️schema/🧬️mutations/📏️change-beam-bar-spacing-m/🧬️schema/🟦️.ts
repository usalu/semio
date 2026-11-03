/** 📏️ `change-beam-bar-spacing-m` wire twin: the leaf payload `ChangeBeamBarSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamBarSpacingM {
  id: string;
  newBarSpacingM: number;
}

export const parseChangeBeamBarSpacingM: NormWireReader<ChangeBeamBarSpacingM> = normWireObject<ChangeBeamBarSpacingM>({ id: normWireRequired(normWireString), newBarSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
