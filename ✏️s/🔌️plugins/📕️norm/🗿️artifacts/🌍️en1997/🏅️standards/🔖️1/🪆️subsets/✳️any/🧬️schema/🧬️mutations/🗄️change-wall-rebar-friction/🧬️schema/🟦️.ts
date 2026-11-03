/** 🗄️ `change-wall-rebar-friction` wire twin: the leaf payload `ChangeWallRebarFriction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallRebarFriction {
  mutation: "changeWallRebarFriction";
  id: string;
  newReinforcementFriction: number;
}

export const parseChangeWallRebarFriction: NormWireReader<ChangeWallRebarFriction> = normWireObject<ChangeWallRebarFriction>({ mutation: normWireRequired(normWireLiteral("changeWallRebarFriction")), id: normWireRequired(normWireString), newReinforcementFriction: normWireRequired(normWireNumber) });
