/** 🔄️ `change-pile-dynamic-res-mean` wire twin: the leaf payload `ChangePileDynamicResMean`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDynamicResMean {
  mutation: "changePileDynamicResMean";
  id: string;
  newDynamicResistanceMean: number;
}

export const parseChangePileDynamicResMean: NormWireReader<ChangePileDynamicResMean> = normWireObject<ChangePileDynamicResMean>({ mutation: normWireRequired(normWireLiteral("changePileDynamicResMean")), id: normWireRequired(normWireString), newDynamicResistanceMean: normWireRequired(normWireNumber) });
