/** ✏️ `change-tower-is-chimney` wire twin: the leaf payload `ChangeTowerIsChimney`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerIsChimney {
  mutation: "changeTowerIsChimney";
  id: string;
  newIsChimney: boolean;
}

export const parseChangeTowerIsChimney: NormWireReader<ChangeTowerIsChimney> = normWireObject<ChangeTowerIsChimney>({ mutation: normWireRequired(normWireLiteral("changeTowerIsChimney")), id: normWireRequired(normWireString), newIsChimney: normWireRequired(normWireBoolean) });
