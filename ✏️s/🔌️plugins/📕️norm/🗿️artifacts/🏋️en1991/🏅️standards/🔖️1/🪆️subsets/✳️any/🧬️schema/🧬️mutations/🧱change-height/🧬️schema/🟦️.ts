/** 🧱 `change-height` wire twin: the leaf payload `ChangeHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHeight {
  newHeight: number;
}

export const parseChangeHeight: NormWireReader<ChangeHeight> = normWireObject<ChangeHeight>({ newHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
