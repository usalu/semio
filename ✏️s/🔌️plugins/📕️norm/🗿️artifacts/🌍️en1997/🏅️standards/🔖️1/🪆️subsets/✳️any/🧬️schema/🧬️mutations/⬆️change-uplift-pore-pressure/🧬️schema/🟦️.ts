/** ⬆️ `change-uplift-pore-pressure` wire twin: the leaf payload `ChangeUpliftPorePressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUpliftPorePressure {
  mutation: "changeUpliftPorePressure";
  id: string;
  newPorePressure: number;
}

export const parseChangeUpliftPorePressure: NormWireReader<ChangeUpliftPorePressure> = normWireObject<ChangeUpliftPorePressure>({ mutation: normWireRequired(normWireLiteral("changeUpliftPorePressure")), id: normWireRequired(normWireString), newPorePressure: normWireRequired(normWireNumber) });
