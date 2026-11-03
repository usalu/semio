/** ⚓️ `change-anchor-f-uk` wire twin: the leaf payload `ChangeAnchorFUk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFUk {
  anchorId: string;
  newFUk: number;
}

export const parseChangeAnchorFUk: NormWireReader<ChangeAnchorFUk> = normWireObject<ChangeAnchorFUk>({ anchorId: normWireRequired(normWireString), newFUk: normWireRequired(normWireNumber) });
