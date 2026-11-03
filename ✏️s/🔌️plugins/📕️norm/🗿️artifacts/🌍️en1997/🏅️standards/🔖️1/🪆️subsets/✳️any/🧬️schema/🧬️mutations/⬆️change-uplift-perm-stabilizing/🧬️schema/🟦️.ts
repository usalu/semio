/** ⬆️ `change-uplift-perm-stabilizing` wire twin: the leaf payload `ChangeUpliftPermStabilizing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUpliftPermStabilizing {
  mutation: "changeUpliftPermStabilizing";
  id: string;
  newPermanentStabilizing: number;
}

export const parseChangeUpliftPermStabilizing: NormWireReader<ChangeUpliftPermStabilizing> = normWireObject<ChangeUpliftPermStabilizing>({ mutation: normWireRequired(normWireLiteral("changeUpliftPermStabilizing")), id: normWireRequired(normWireString), newPermanentStabilizing: normWireRequired(normWireNumber) });
