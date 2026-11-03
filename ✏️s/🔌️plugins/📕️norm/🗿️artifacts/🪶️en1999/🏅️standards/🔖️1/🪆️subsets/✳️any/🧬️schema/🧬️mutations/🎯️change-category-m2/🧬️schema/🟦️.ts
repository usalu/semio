/** 🎯️ `change-category-m2` wire twin: the leaf payload `ChangeCategoryM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryM2 {
  mutation: "changeCategoryM2";
  fatigueCategoryId: string;
  newM2: number;
}

export const parseChangeCategoryM2: NormWireReader<ChangeCategoryM2> = normWireObject<ChangeCategoryM2>({ mutation: normWireRequired(normWireLiteral("changeCategoryM2")), fatigueCategoryId: normWireRequired(normWireString), newM2: normWireRequired(normWireNumber) });
