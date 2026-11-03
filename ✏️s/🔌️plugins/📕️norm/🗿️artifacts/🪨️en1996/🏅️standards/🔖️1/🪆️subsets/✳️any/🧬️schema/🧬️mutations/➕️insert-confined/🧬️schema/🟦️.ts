/** ➕️ `insert-confined` wire twin: the leaf payload `InsertConfined`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConfinedElement, parseConfinedElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConfined {
  wallId: string;
  index: number;
  confined: ConfinedElement;
}

export const parseInsertConfined: NormWireReader<InsertConfined> = normWireObject<InsertConfined>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), confined: normWireRequired(parseConfinedElement) });
