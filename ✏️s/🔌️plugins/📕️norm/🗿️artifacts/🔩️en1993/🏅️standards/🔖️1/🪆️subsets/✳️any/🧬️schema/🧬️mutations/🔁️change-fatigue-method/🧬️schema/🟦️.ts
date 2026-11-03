/** 🔁️ `change-fatigue-method` wire twin: the leaf payload `ChangeFatigueMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueMethod, parseFatigueMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFatigueMethod {
  id: string;
  newMethod: FatigueMethod;
}

export const parseChangeFatigueMethod: NormWireReader<ChangeFatigueMethod> = normWireObject<ChangeFatigueMethod>({ id: normWireRequired(normWireString), newMethod: normWireRequired(parseFatigueMethod) });
