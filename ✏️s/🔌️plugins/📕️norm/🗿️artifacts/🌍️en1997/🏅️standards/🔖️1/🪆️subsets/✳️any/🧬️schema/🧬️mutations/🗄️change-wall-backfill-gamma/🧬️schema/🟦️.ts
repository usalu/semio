/** 🗄️ `change-wall-backfill-gamma` wire twin: the leaf payload `ChangeWallBackfillGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallBackfillGamma {
  mutation: "changeWallBackfillGamma";
  id: string;
  newBackfillGamma: number;
}

export const parseChangeWallBackfillGamma: NormWireReader<ChangeWallBackfillGamma> = normWireObject<ChangeWallBackfillGamma>({ mutation: normWireRequired(normWireLiteral("changeWallBackfillGamma")), id: normWireRequired(normWireString), newBackfillGamma: normWireRequired(normWireNumber) });
