/** 🧵️ `change-weld-depth` wire twin: the leaf payload `ChangeWeldDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldDepth {
  id: string;
  newDepth: number;
}

export const parseChangeWeldDepth: NormWireReader<ChangeWeldDepth> = normWireObject<ChangeWeldDepth>({ id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
