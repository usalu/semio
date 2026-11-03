/** 🗄️ `change-wall-backfill-phi-deg` wire twin: the leaf payload `ChangeWallBackfillPhiDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallBackfillPhiDeg {
  mutation: "changeWallBackfillPhiDeg";
  id: string;
  newBackfillPhiDeg: number;
}

export const parseChangeWallBackfillPhiDeg: NormWireReader<ChangeWallBackfillPhiDeg> = normWireObject<ChangeWallBackfillPhiDeg>({ mutation: normWireRequired(normWireLiteral("changeWallBackfillPhiDeg")), id: normWireRequired(normWireString), newBackfillPhiDeg: normWireRequired(normWireNumber) });
