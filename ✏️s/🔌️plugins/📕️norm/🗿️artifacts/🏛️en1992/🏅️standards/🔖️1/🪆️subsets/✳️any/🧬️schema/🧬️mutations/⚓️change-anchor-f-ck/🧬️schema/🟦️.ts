/** ⚓️ `change-anchor-f-ck` wire twin: the leaf payload `ChangeAnchorFCk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFCk {
  anchorId: string;
  newFCk: number;
}

export const parseChangeAnchorFCk: NormWireReader<ChangeAnchorFCk> = normWireObject<ChangeAnchorFCk>({ anchorId: normWireRequired(normWireString), newFCk: normWireRequired(normWireNumber) });
