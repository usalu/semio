/** 🌬 `change-wind-face-c-pe10` wire twin: the leaf payload `ChangeWindFaceCPe10`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceCPe10 {
  id: string;
  newCPe10: number;
}

export const parseChangeWindFaceCPe10: NormWireReader<ChangeWindFaceCPe10> = normWireObject<ChangeWindFaceCPe10>({ id: normWireRequired(normWireString), newCPe10: normWireRequired(normWireNumber) });
