/** 🏗️ `change-pile-group-cap-moment-y` wire twin: the leaf payload `ChangePileGroupCapMomentY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupCapMomentY {
  mutation: "changePileGroupCapMomentY";
  id: string;
  newCapMomentY: number;
}

export const parseChangePileGroupCapMomentY: NormWireReader<ChangePileGroupCapMomentY> = normWireObject<ChangePileGroupCapMomentY>({ mutation: normWireRequired(normWireLiteral("changePileGroupCapMomentY")), id: normWireRequired(normWireString), newCapMomentY: normWireRequired(normWireNumber) });
