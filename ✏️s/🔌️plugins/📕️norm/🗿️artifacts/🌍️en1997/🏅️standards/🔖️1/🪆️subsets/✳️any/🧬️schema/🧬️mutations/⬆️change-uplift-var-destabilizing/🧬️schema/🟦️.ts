/** ⬆️ `change-uplift-var-destabilizing` wire twin: the leaf payload `ChangeUpliftVarDestabilizing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUpliftVarDestabilizing {
  mutation: "changeUpliftVarDestabilizing";
  id: string;
  newVariableDestabilizing: number;
}

export const parseChangeUpliftVarDestabilizing: NormWireReader<ChangeUpliftVarDestabilizing> = normWireObject<ChangeUpliftVarDestabilizing>({ mutation: normWireRequired(normWireLiteral("changeUpliftVarDestabilizing")), id: normWireRequired(normWireString), newVariableDestabilizing: normWireRequired(normWireNumber) });
