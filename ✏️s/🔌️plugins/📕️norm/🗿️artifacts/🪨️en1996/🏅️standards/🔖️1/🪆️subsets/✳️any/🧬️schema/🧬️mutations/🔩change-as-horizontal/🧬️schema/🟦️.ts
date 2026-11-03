/** 🔩 `change-as-horizontal` wire twin: the leaf payload `ChangeAsHorizontal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAsHorizontal {
  id: string;
  newAsHorizontalM2: number;
}

export const parseChangeAsHorizontal: NormWireReader<ChangeAsHorizontal> = normWireObject<ChangeAsHorizontal>({ id: normWireRequired(normWireString), newAsHorizontalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
