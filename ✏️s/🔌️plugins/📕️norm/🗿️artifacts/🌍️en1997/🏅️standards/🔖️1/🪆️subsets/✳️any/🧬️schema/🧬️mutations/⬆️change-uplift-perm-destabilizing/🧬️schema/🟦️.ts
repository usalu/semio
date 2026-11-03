/** ⬆️ `change-uplift-perm-destabilizing` wire twin: the leaf payload `ChangeUpliftPermDestabilizing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUpliftPermDestabilizing {
  mutation: "changeUpliftPermDestabilizing";
  id: string;
  newPermanentDestabilizing: number;
}

export const parseChangeUpliftPermDestabilizing: NormWireReader<ChangeUpliftPermDestabilizing> = normWireObject<ChangeUpliftPermDestabilizing>({ mutation: normWireRequired(normWireLiteral("changeUpliftPermDestabilizing")), id: normWireRequired(normWireString), newPermanentDestabilizing: normWireRequired(normWireNumber) });
