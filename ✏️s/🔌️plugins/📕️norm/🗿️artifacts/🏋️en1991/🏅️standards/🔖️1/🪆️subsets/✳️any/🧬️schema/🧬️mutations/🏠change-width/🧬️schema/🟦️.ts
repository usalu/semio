/** 🏠 `change-width` wire twin: the leaf payload `ChangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWidth {
  newWidth: number;
}

export const parseChangeWidth: NormWireReader<ChangeWidth> = normWireObject<ChangeWidth>({ newWidth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
