/** 🪚️ `change-chase-width` wire twin: the leaf payload `ChangeChaseWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseWidth {
  id: string;
  newWidthM: number;
}

export const parseChangeChaseWidth: NormWireReader<ChangeChaseWidth> = normWireObject<ChangeChaseWidth>({ id: normWireRequired(normWireString), newWidthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
