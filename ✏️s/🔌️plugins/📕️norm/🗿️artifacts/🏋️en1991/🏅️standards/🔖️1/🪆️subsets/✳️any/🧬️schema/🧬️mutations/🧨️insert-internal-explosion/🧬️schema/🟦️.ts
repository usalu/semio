/** 🧨 `insert-internal-explosion` wire twin: the leaf payload `InsertInternalExplosion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type InternalExplosion, parseInternalExplosion } from "../../../📸️snapshot/🟦️.ts";

export interface InsertInternalExplosion {
  index: number;
  item: InternalExplosion;
}

export const parseInsertInternalExplosion: NormWireReader<InsertInternalExplosion> = normWireObject<InsertInternalExplosion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseInternalExplosion) });
