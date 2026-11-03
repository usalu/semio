/** 🛡️ `change-diaphragm-method` wire twin: the leaf payload `ChangeDiaphragmMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DiaphragmMethod, parseDiaphragmMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDiaphragmMethod {
  diaphragmId: string;
  newValue: DiaphragmMethod;
}

export const parseChangeDiaphragmMethod: NormWireReader<ChangeDiaphragmMethod> = normWireObject<ChangeDiaphragmMethod>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(parseDiaphragmMethod) });
