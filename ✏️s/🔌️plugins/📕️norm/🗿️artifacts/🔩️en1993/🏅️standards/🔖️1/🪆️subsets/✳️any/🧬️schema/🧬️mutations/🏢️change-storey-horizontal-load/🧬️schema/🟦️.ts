/** 🏢️ `change-storey-horizontal-load` wire twin: the leaf payload `ChangeStoreyHorizontalLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyHorizontalLoad {
  id: string;
  newHorizontalLoad: number;
}

export const parseChangeStoreyHorizontalLoad: NormWireReader<ChangeStoreyHorizontalLoad> = normWireObject<ChangeStoreyHorizontalLoad>({ id: normWireRequired(normWireString), newHorizontalLoad: normWireRequired(normWireNumber) });
