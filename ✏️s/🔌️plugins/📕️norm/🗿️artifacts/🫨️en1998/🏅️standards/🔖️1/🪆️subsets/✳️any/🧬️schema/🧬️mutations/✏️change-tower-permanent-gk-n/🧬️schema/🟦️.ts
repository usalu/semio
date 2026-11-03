/** ✏️ `change-tower-permanent-gk-n` wire twin: the leaf payload `ChangeTowerPermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerPermanentGkN {
  mutation: "changeTowerPermanentGkN";
  id: string;
  newPermanentGkN: number;
}

export const parseChangeTowerPermanentGkN: NormWireReader<ChangeTowerPermanentGkN> = normWireObject<ChangeTowerPermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeTowerPermanentGkN")), id: normWireRequired(normWireString), newPermanentGkN: normWireRequired(normWireNumber) });
