/** 🧈 `change-mortar-class` wire twin: the leaf payload `ChangeMortarClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996MortarClass, parseEn1996MortarClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMortarClass {
  id: string;
  newMortarClass: En1996MortarClass;
}

export const parseChangeMortarClass: NormWireReader<ChangeMortarClass> = normWireObject<ChangeMortarClass>({ id: normWireRequired(normWireString), newMortarClass: normWireRequired(parseEn1996MortarClass) });
