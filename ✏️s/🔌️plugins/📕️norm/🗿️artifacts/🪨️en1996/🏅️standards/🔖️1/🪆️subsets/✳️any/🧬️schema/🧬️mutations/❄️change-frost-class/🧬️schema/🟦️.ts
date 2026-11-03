/** ❄️ `change-frost-class` wire twin: the leaf payload `ChangeFrostClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996FrostClass, parseEn1996FrostClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrostClass {
  id: string;
  newFrostClass: En1996FrostClass;
}

export const parseChangeFrostClass: NormWireReader<ChangeFrostClass> = normWireObject<ChangeFrostClass>({ id: normWireRequired(normWireString), newFrostClass: normWireRequired(parseEn1996FrostClass) });
