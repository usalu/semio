/** ⚙ `insert-dust-explosion` wire twin: the leaf payload `InsertDustExplosion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DustExplosion, parseDustExplosion } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDustExplosion {
  index: number;
  item: DustExplosion;
}

export const parseInsertDustExplosion: NormWireReader<InsertDustExplosion> = normWireObject<InsertDustExplosion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseDustExplosion) });
