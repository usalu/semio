/** ⚓️ `change-anchor-c1` wire twin: the leaf payload `ChangeAnchorC1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorC1 {
  anchorId: string;
  newC1: number;
}

export const parseChangeAnchorC1: NormWireReader<ChangeAnchorC1> = normWireObject<ChangeAnchorC1>({ anchorId: normWireRequired(normWireString), newC1: normWireRequired(normWireNumber) });
