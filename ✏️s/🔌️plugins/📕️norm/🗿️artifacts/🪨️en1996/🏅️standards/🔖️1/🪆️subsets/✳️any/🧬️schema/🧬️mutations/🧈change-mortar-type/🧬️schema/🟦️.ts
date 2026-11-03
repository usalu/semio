/** 🧈 `change-mortar-type` wire twin: the leaf payload `ChangeMortarType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996MortarType, parseEn1996MortarType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMortarType {
  id: string;
  newMortarType: En1996MortarType;
}

export const parseChangeMortarType: NormWireReader<ChangeMortarType> = normWireObject<ChangeMortarType>({ id: normWireRequired(normWireString), newMortarType: normWireRequired(parseEn1996MortarType) });
