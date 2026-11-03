/** 🔄️ `change-pile-hammer-weight` wire twin: the leaf payload `ChangePileHammerWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileHammerWeight {
  mutation: "changePileHammerWeight";
  id: string;
  newHammerWeight: number;
}

export const parseChangePileHammerWeight: NormWireReader<ChangePileHammerWeight> = normWireObject<ChangePileHammerWeight>({ mutation: normWireRequired(normWireLiteral("changePileHammerWeight")), id: normWireRequired(normWireString), newHammerWeight: normWireRequired(normWireNumber) });
