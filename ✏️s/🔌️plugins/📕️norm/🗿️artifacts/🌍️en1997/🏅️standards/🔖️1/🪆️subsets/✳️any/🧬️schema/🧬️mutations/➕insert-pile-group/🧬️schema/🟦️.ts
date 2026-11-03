/** ➕ `insert-pile-group` wire twin: the leaf payload `InsertPileGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997PileGroup, parseEn1997PileGroup } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPileGroup {
  mutation: "insertPileGroup";
  index: number;
  pileGroup: En1997PileGroup;
}

export const parseInsertPileGroup: NormWireReader<InsertPileGroup> = normWireObject<InsertPileGroup>({ mutation: normWireRequired(normWireLiteral("insertPileGroup")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pileGroup: normWireRequired(parseEn1997PileGroup) });
