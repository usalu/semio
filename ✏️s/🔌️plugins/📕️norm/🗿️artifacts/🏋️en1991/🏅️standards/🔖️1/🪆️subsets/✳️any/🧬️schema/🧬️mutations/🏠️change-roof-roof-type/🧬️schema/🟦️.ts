/** 🏠 `change-roof-roof-type` wire twin: the leaf payload `ChangeRoofRoofType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofRoofType {
  id: string;
  newRoofType: string;
}

export const parseChangeRoofRoofType: NormWireReader<ChangeRoofRoofType> = normWireObject<ChangeRoofRoofType>({ id: normWireRequired(normWireString), newRoofType: normWireRequired(normWireString) });
