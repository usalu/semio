/** ✏️ `change-pier-reinforcement-ratio` wire twin: the leaf payload `ChangePierReinforcementRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierReinforcementRatio {
  mutation: "changePierReinforcementRatio";
  id: string;
  newReinforcementRatio: number;
}

export const parseChangePierReinforcementRatio: NormWireReader<ChangePierReinforcementRatio> = normWireObject<ChangePierReinforcementRatio>({ mutation: normWireRequired(normWireLiteral("changePierReinforcementRatio")), id: normWireRequired(normWireString), newReinforcementRatio: normWireRequired(normWireNumber) });
