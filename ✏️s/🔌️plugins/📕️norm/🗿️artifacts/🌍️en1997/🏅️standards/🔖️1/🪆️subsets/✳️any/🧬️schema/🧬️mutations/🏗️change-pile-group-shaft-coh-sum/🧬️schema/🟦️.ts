/** 🏗️ `change-pile-group-shaft-coh-sum` wire twin: the leaf payload `ChangePileGroupShaftCohSum`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupShaftCohSum {
  mutation: "changePileGroupShaftCohSum";
  id: string;
  newShaftCohesionSum: number;
}

export const parseChangePileGroupShaftCohSum: NormWireReader<ChangePileGroupShaftCohSum> = normWireObject<ChangePileGroupShaftCohSum>({ mutation: normWireRequired(normWireLiteral("changePileGroupShaftCohSum")), id: normWireRequired(normWireString), newShaftCohesionSum: normWireRequired(normWireNumber) });
