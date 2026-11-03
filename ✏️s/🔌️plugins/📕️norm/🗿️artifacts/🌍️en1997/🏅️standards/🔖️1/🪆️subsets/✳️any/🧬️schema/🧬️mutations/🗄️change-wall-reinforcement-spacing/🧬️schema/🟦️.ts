/** 🗄️ `change-wall-reinforcement-spacing` wire twin: the leaf payload `ChangeWallReinforcementSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallReinforcementSpacing {
  mutation: "changeWallReinforcementSpacing";
  id: string;
  newReinforcementSpacing: number;
}

export const parseChangeWallReinforcementSpacing: NormWireReader<ChangeWallReinforcementSpacing> = normWireObject<ChangeWallReinforcementSpacing>({ mutation: normWireRequired(normWireLiteral("changeWallReinforcementSpacing")), id: normWireRequired(normWireString), newReinforcementSpacing: normWireRequired(normWireNumber) });
