/** 🗄️ `change-wall-reinforcement-width` wire twin: the leaf payload `ChangeWallReinforcementWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallReinforcementWidth {
  mutation: "changeWallReinforcementWidth";
  id: string;
  newReinforcementWidth: number;
}

export const parseChangeWallReinforcementWidth: NormWireReader<ChangeWallReinforcementWidth> = normWireObject<ChangeWallReinforcementWidth>({ mutation: normWireRequired(normWireLiteral("changeWallReinforcementWidth")), id: normWireRequired(normWireString), newReinforcementWidth: normWireRequired(normWireNumber) });
