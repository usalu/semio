/** ➖️ `remove-tower-variable` wire twin: the leaf payload `RemoveTowerVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTowerVariable {
  mutation: "removeTowerVariable";
  towerId: string;
  id: string;
}

export const parseRemoveTowerVariable: NormWireReader<RemoveTowerVariable> = normWireObject<RemoveTowerVariable>({ mutation: normWireRequired(normWireLiteral("removeTowerVariable")), towerId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
