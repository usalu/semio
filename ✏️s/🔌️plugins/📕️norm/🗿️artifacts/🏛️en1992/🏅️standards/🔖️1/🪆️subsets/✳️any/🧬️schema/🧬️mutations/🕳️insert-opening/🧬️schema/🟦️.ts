/** 🕳️ `insert-opening` wire twin: the leaf payload `InsertOpening`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Opening, parseOpening } from "../../../📸️snapshot/🟦️.ts";

export interface InsertOpening {
  memberId: string;
  index: number;
  opening: Opening;
}

export const parseInsertOpening: NormWireReader<InsertOpening> = normWireObject<InsertOpening>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), opening: normWireRequired(parseOpening) });
