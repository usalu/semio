/** 🗂️ `insert-fire-insulation` wire twin: the leaf payload `InsertFireInsulation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FireInsulation, parseFireInsulation } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFireInsulation {
  mutation: "insertFireInsulation";
  index: number;
  fireInsulation: FireInsulation;
}

export const parseInsertFireInsulation: NormWireReader<InsertFireInsulation> = normWireObject<InsertFireInsulation>({ mutation: normWireRequired(normWireLiteral("insertFireInsulation")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fireInsulation: normWireRequired(parseFireInsulation) });
