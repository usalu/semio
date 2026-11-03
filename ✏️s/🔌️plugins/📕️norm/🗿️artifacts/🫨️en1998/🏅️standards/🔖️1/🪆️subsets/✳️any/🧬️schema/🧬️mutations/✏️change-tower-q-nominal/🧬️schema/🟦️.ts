/** ✏️ `change-tower-q-nominal` wire twin: the leaf payload `ChangeTowerQNominal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerQNominal {
  mutation: "changeTowerQNominal";
  id: string;
  newQNominal: number;
}

export const parseChangeTowerQNominal: NormWireReader<ChangeTowerQNominal> = normWireObject<ChangeTowerQNominal>({ mutation: normWireRequired(normWireLiteral("changeTowerQNominal")), id: normWireRequired(normWireString), newQNominal: normWireRequired(normWireNumber) });
