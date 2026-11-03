/** 🏠 `change-roof-ce` wire twin: the leaf payload `ChangeRoofCE`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofCE {
  id: string;
  newCE: number;
}

export const parseChangeRoofCE: NormWireReader<ChangeRoofCE> = normWireObject<ChangeRoofCE>({ id: normWireRequired(normWireString), newCE: normWireRequired(normWireNumber) });
