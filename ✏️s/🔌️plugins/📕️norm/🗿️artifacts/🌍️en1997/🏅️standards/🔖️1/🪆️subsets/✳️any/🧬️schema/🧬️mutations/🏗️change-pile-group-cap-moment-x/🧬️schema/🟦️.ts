/** 🏗️ `change-pile-group-cap-moment-x` wire twin: the leaf payload `ChangePileGroupCapMomentX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupCapMomentX {
  mutation: "changePileGroupCapMomentX";
  id: string;
  newCapMomentX: number;
}

export const parseChangePileGroupCapMomentX: NormWireReader<ChangePileGroupCapMomentX> = normWireObject<ChangePileGroupCapMomentX>({ mutation: normWireRequired(normWireLiteral("changePileGroupCapMomentX")), id: normWireRequired(normWireString), newCapMomentX: normWireRequired(normWireNumber) });
