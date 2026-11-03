/** ✏️ `change-pier-stiffness-basis` wire twin: the leaf payload `ChangePierStiffnessBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PierStiffnessBasis, parsePierStiffnessBasis } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePierStiffnessBasis {
  mutation: "changePierStiffnessBasis";
  id: string;
  newStiffnessBasis: PierStiffnessBasis;
}

export const parseChangePierStiffnessBasis: NormWireReader<ChangePierStiffnessBasis> = normWireObject<ChangePierStiffnessBasis>({ mutation: normWireRequired(normWireLiteral("changePierStiffnessBasis")), id: normWireRequired(normWireString), newStiffnessBasis: normWireRequired(parsePierStiffnessBasis) });
