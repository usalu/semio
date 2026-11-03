/** 🏠 `change-roof-ct` wire twin: the leaf payload `ChangeRoofCT`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofCT {
  id: string;
  newCT: number;
}

export const parseChangeRoofCT: NormWireReader<ChangeRoofCT> = normWireObject<ChangeRoofCT>({ id: normWireRequired(normWireString), newCT: normWireRequired(normWireNumber) });
