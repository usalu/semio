/** 🔄️ `change-pile-lateral-head-load` wire twin: the leaf payload `ChangePileLateralHeadLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileLateralHeadLoad {
  mutation: "changePileLateralHeadLoad";
  id: string;
  newLateralHeadLoad: number;
}

export const parseChangePileLateralHeadLoad: NormWireReader<ChangePileLateralHeadLoad> = normWireObject<ChangePileLateralHeadLoad>({ mutation: normWireRequired(normWireLiteral("changePileLateralHeadLoad")), id: normWireRequired(normWireString), newLateralHeadLoad: normWireRequired(normWireNumber) });
