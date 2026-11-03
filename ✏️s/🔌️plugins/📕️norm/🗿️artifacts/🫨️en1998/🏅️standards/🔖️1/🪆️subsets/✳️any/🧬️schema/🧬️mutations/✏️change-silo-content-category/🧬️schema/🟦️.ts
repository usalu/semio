/** ✏️ `change-silo-content-category` wire twin: the leaf payload `ChangeSiloContentCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSiloContentCategory {
  mutation: "changeSiloContentCategory";
  id: string;
  newContentCategory: ImposedCategory;
}

export const parseChangeSiloContentCategory: NormWireReader<ChangeSiloContentCategory> = normWireObject<ChangeSiloContentCategory>({ mutation: normWireRequired(normWireLiteral("changeSiloContentCategory")), id: normWireRequired(normWireString), newContentCategory: normWireRequired(parseImposedCategory) });
