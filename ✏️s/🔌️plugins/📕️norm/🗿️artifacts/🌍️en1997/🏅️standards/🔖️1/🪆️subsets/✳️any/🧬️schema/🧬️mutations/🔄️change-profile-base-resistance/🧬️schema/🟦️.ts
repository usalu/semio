/** 🔄️ `change-profile-base-resistance` wire twin: the leaf payload `ChangeProfileBaseResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeProfileBaseResistance {
  mutation: "changeProfileBaseResistance";
  pileId: string;
  id: string;
  newBaseResistance: number;
}

export const parseChangeProfileBaseResistance: NormWireReader<ChangeProfileBaseResistance> = normWireObject<ChangeProfileBaseResistance>({ mutation: normWireRequired(normWireLiteral("changeProfileBaseResistance")), pileId: normWireRequired(normWireString), id: normWireRequired(normWireString), newBaseResistance: normWireRequired(normWireNumber) });
