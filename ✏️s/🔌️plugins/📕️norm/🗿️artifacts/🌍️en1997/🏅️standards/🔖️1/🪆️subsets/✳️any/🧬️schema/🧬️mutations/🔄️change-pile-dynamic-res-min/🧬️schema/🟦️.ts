/** 🔄️ `change-pile-dynamic-res-min` wire twin: the leaf payload `ChangePileDynamicResMin`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDynamicResMin {
  mutation: "changePileDynamicResMin";
  id: string;
  newDynamicResistanceMin: number;
}

export const parseChangePileDynamicResMin: NormWireReader<ChangePileDynamicResMin> = normWireObject<ChangePileDynamicResMin>({ mutation: normWireRequired(normWireLiteral("changePileDynamicResMin")), id: normWireRequired(normWireString), newDynamicResistanceMin: normWireRequired(normWireNumber) });
