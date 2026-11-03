/** 🏛️ `change-pier-width` wire twin: the leaf payload `ChangePierWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierWidth {
  wallId: string;
  id: string;
  newWidthM: number;
}

export const parseChangePierWidth: NormWireReader<ChangePierWidth> = normWireObject<ChangePierWidth>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
