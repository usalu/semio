/** 🧮 `change-c-pe` wire twin: the leaf payload `ChangeCPe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCPe {
  wallId: string;
  id: string;
  newCPe: number;
}

export const parseChangeCPe: NormWireReader<ChangeCPe> = normWireObject<ChangeCPe>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCPe: normWireRequired(normWireRange(normWireNumber, {"minimum":-5,"maximum":5})) });
