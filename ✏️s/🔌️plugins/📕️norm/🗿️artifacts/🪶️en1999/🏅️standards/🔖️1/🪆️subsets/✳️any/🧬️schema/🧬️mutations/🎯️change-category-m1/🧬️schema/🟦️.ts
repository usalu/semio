/** 🎯️ `change-category-m1` wire twin: the leaf payload `ChangeCategoryM1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryM1 {
  mutation: "changeCategoryM1";
  fatigueCategoryId: string;
  newM1: number;
}

export const parseChangeCategoryM1: NormWireReader<ChangeCategoryM1> = normWireObject<ChangeCategoryM1>({ mutation: normWireRequired(normWireLiteral("changeCategoryM1")), fatigueCategoryId: normWireRequired(normWireString), newM1: normWireRequired(normWireNumber) });
