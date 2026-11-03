/** 🪜️ `change-built-up-label` wire twin: the leaf payload `ChangeBuiltUpLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpLabel {
  id: string;
  newLabel: string;
}

export const parseChangeBuiltUpLabel: NormWireReader<ChangeBuiltUpLabel> = normWireObject<ChangeBuiltUpLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
