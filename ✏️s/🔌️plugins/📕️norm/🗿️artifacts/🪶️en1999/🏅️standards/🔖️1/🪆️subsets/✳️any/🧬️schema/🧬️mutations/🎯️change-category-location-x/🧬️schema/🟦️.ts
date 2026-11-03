/** 🎯️ `change-category-location-x` wire twin: the leaf payload `ChangeCategoryLocationX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryLocationX {
  mutation: "changeCategoryLocationX";
  fatigueCategoryId: string;
  newLocationX: number;
}

export const parseChangeCategoryLocationX: NormWireReader<ChangeCategoryLocationX> = normWireObject<ChangeCategoryLocationX>({ mutation: normWireRequired(normWireLiteral("changeCategoryLocationX")), fatigueCategoryId: normWireRequired(normWireString), newLocationX: normWireRequired(normWireNumber) });
