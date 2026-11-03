/** 🌉️ `change-bridge-method` wire twin: the leaf payload `ChangeBridgeMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueMethod, parseFatigueMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBridgeMethod {
  id: string;
  newMethod: FatigueMethod;
}

export const parseChangeBridgeMethod: NormWireReader<ChangeBridgeMethod> = normWireObject<ChangeBridgeMethod>({ id: normWireRequired(normWireString), newMethod: normWireRequired(parseFatigueMethod) });
