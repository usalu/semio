/** 🧪 `change-cement-type` wire twin: the leaf payload `ChangeCementType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CementClass, parseCementClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCementType {
  newCementType: CementClass;
}

export const parseChangeCementType: NormWireReader<ChangeCementType> = normWireObject<ChangeCementType>({ newCementType: normWireRequired(parseCementClass) });
