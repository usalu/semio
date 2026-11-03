/** 🔄️ `change-profile-shaft-resistance` wire twin: the leaf payload `ChangeProfileShaftResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeProfileShaftResistance {
  mutation: "changeProfileShaftResistance";
  pileId: string;
  id: string;
  newShaftResistance: number;
}

export const parseChangeProfileShaftResistance: NormWireReader<ChangeProfileShaftResistance> = normWireObject<ChangeProfileShaftResistance>({ mutation: normWireRequired(normWireLiteral("changeProfileShaftResistance")), pileId: normWireRequired(normWireString), id: normWireRequired(normWireString), newShaftResistance: normWireRequired(normWireNumber) });
