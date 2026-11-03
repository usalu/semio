/** 🧵️ `change-weld-width` wire twin: the leaf payload `ChangeWeldWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldWidth {
  id: string;
  newWidth: number;
}

export const parseChangeWeldWidth: NormWireReader<ChangeWeldWidth> = normWireObject<ChangeWeldWidth>({ id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
