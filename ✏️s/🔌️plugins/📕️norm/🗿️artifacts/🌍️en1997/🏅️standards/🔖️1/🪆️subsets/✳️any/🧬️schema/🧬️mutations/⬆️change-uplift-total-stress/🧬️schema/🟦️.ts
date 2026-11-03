/** ⬆️ `change-uplift-total-stress` wire twin: the leaf payload `ChangeUpliftTotalStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUpliftTotalStress {
  mutation: "changeUpliftTotalStress";
  id: string;
  newTotalStress: number;
}

export const parseChangeUpliftTotalStress: NormWireReader<ChangeUpliftTotalStress> = normWireObject<ChangeUpliftTotalStress>({ mutation: normWireRequired(normWireLiteral("changeUpliftTotalStress")), id: normWireRequired(normWireString), newTotalStress: normWireRequired(normWireNumber) });
