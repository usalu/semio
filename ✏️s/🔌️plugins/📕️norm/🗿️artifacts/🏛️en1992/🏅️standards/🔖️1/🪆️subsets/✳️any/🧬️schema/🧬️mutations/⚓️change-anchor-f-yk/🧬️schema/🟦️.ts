/** ⚓️ `change-anchor-f-yk` wire twin: the leaf payload `ChangeAnchorFYk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFYk {
  anchorId: string;
  newFYk: number;
}

export const parseChangeAnchorFYk: NormWireReader<ChangeAnchorFYk> = normWireObject<ChangeAnchorFYk>({ anchorId: normWireRequired(normWireString), newFYk: normWireRequired(normWireNumber) });
