/** 🧵️ `change-weld-throat` wire twin: the leaf payload `ChangeWeldThroat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldThroat {
  id: string;
  newThroat: number;
}

export const parseChangeWeldThroat: NormWireReader<ChangeWeldThroat> = normWireObject<ChangeWeldThroat>({ id: normWireRequired(normWireString), newThroat: normWireRequired(normWireNumber) });
