/** ✏️ `change-building-t1-method` wire twin: the leaf payload `ChangeBuildingT1Method`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type T1Method, parseT1Method } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBuildingT1Method {
  mutation: "changeBuildingT1Method";
  id: string;
  newT1Method: T1Method;
}

export const parseChangeBuildingT1Method: NormWireReader<ChangeBuildingT1Method> = normWireObject<ChangeBuildingT1Method>({ mutation: normWireRequired(normWireLiteral("changeBuildingT1Method")), id: normWireRequired(normWireString), newT1Method: normWireRequired(parseT1Method) });
