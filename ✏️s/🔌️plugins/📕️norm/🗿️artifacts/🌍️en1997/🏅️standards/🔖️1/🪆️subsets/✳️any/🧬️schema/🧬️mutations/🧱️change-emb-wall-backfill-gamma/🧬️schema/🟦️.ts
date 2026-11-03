/** 🧱 `change-emb-wall-backfill-gamma` wire twin: the leaf payload `ChangeEmbWallBackfillGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallBackfillGamma {
  mutation: "changeEmbWallBackfillGamma";
  id: string;
  newBackfillGamma: number;
}

export const parseChangeEmbWallBackfillGamma: NormWireReader<ChangeEmbWallBackfillGamma> = normWireObject<ChangeEmbWallBackfillGamma>({ mutation: normWireRequired(normWireLiteral("changeEmbWallBackfillGamma")), id: normWireRequired(normWireString), newBackfillGamma: normWireRequired(normWireNumber) });
