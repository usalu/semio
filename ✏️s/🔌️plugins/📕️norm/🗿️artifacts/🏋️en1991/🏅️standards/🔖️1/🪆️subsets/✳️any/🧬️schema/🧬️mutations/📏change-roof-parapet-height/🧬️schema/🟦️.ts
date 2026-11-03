/** 📏 `change-roof-parapet-height` wire twin: the leaf payload `ChangeRoofParapetHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofParapetHeight {
  id: string;
  newParapetHeight: number;
}

export const parseChangeRoofParapetHeight: NormWireReader<ChangeRoofParapetHeight> = normWireObject<ChangeRoofParapetHeight>({ id: normWireRequired(normWireString), newParapetHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
