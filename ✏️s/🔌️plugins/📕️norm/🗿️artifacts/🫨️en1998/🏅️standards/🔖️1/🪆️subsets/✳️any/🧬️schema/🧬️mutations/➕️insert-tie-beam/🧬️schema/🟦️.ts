/** ➕️ `insert-tie-beam` wire twin: the leaf payload `InsertTieBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998TieBeam, parseEn1998TieBeam } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTieBeam {
  mutation: "insertTieBeam";
  index?: number | null;
  tieBeam: En1998TieBeam;
}

export const parseInsertTieBeam: NormWireReader<InsertTieBeam> = normWireObject<InsertTieBeam>({ mutation: normWireRequired(normWireLiteral("insertTieBeam")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), tieBeam: normWireRequired(parseEn1998TieBeam) });
