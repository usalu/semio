/** 🌨️ `change-roof-assumed-sk` wire twin: the leaf payload `ChangeRoofAssumedSk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofAssumedSk {
  id: string;
  newAssumedSk: number;
}

export const parseChangeRoofAssumedSk: NormWireReader<ChangeRoofAssumedSk> = normWireObject<ChangeRoofAssumedSk>({ id: normWireRequired(normWireString), newAssumedSk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
