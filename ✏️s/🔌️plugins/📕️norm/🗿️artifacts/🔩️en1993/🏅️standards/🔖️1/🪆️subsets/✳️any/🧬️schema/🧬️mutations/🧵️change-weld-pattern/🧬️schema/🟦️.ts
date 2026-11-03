/** 🧵️ `change-weld-pattern` wire twin: the leaf payload `ChangeWeldPattern`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldPattern, parseWeldPattern } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWeldPattern {
  id: string;
  newPattern: WeldPattern;
}

export const parseChangeWeldPattern: NormWireReader<ChangeWeldPattern> = normWireObject<ChangeWeldPattern>({ id: normWireRequired(normWireString), newPattern: normWireRequired(parseWeldPattern) });
