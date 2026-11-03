/** 🏢️ `change-storey-vertical-load` wire twin: the leaf payload `ChangeStoreyVerticalLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyVerticalLoad {
  id: string;
  newVerticalLoad: number;
}

export const parseChangeStoreyVerticalLoad: NormWireReader<ChangeStoreyVerticalLoad> = normWireObject<ChangeStoreyVerticalLoad>({ id: normWireRequired(normWireString), newVerticalLoad: normWireRequired(normWireNumber) });
