/** 🔄️ `change-pile-hammer-efficiency` wire twin: the leaf payload `ChangePileHammerEfficiency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileHammerEfficiency {
  mutation: "changePileHammerEfficiency";
  id: string;
  newHammerEfficiency: number;
}

export const parseChangePileHammerEfficiency: NormWireReader<ChangePileHammerEfficiency> = normWireObject<ChangePileHammerEfficiency>({ mutation: normWireRequired(normWireLiteral("changePileHammerEfficiency")), id: normWireRequired(normWireString), newHammerEfficiency: normWireRequired(normWireNumber) });
