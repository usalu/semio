/** 🔩 `change-as-vertical` wire twin: the leaf payload `ChangeAsVertical`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAsVertical {
  id: string;
  newAsVerticalM2: number;
}

export const parseChangeAsVertical: NormWireReader<ChangeAsVertical> = normWireObject<ChangeAsVertical>({ id: normWireRequired(normWireString), newAsVerticalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
