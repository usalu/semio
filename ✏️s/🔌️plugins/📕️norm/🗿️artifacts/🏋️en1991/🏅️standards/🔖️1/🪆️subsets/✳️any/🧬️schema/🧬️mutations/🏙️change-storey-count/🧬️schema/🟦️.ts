/** 🏙️ `change-storey-count` wire twin: the leaf payload `ChangeStoreyCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCount {
  newStoreyCount: number;
}

export const parseChangeStoreyCount: NormWireReader<ChangeStoreyCount> = normWireObject<ChangeStoreyCount>({ newStoreyCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
