/** ➕ `insert-embedded-wall` wire twin: the leaf payload `InsertEmbeddedWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997EmbeddedWall, parseEn1997EmbeddedWall } from "../../../📸️snapshot/🟦️.ts";

export interface InsertEmbeddedWall {
  mutation: "insertEmbeddedWall";
  index: number;
  embeddedWall: En1997EmbeddedWall;
}

export const parseInsertEmbeddedWall: NormWireReader<InsertEmbeddedWall> = normWireObject<InsertEmbeddedWall>({ mutation: normWireRequired(normWireLiteral("insertEmbeddedWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), embeddedWall: normWireRequired(parseEn1997EmbeddedWall) });
