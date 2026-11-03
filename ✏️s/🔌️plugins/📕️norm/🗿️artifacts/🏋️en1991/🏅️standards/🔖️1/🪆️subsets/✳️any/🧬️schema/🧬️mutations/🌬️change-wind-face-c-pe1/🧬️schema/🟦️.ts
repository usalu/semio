/** 🌬 `change-wind-face-c-pe1` wire twin: the leaf payload `ChangeWindFaceCPe1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceCPe1 {
  id: string;
  newCPe1: number;
}

export const parseChangeWindFaceCPe1: NormWireReader<ChangeWindFaceCPe1> = normWireObject<ChangeWindFaceCPe1>({ id: normWireRequired(normWireString), newCPe1: normWireRequired(normWireNumber) });
