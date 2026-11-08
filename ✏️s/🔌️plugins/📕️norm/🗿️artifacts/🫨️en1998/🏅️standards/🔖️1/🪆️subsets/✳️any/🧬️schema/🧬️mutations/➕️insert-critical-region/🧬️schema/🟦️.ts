/** ➕️ `insert-critical-region` wire twin: the leaf payload `InsertCriticalRegion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998CriticalRegion, parseEn1998CriticalRegion } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCriticalRegion {
  mutation: "insertCriticalRegion";
  index?: number | null;
  criticalRegion: En1998CriticalRegion;
}

export const parseInsertCriticalRegion: NormWireReader<InsertCriticalRegion> = normWireObject<InsertCriticalRegion>({ mutation: normWireRequired(normWireLiteral("insertCriticalRegion")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), criticalRegion: normWireRequired(parseEn1998CriticalRegion) });
