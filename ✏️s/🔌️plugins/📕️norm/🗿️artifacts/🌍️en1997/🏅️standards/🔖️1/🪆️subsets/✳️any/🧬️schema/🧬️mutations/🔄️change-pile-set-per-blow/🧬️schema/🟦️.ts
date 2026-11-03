/** 🔄️ `change-pile-set-per-blow` wire twin: the leaf payload `ChangePileSetPerBlow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileSetPerBlow {
  mutation: "changePileSetPerBlow";
  id: string;
  newSetPerBlow: number;
}

export const parseChangePileSetPerBlow: NormWireReader<ChangePileSetPerBlow> = normWireObject<ChangePileSetPerBlow>({ mutation: normWireRequired(normWireLiteral("changePileSetPerBlow")), id: normWireRequired(normWireString), newSetPerBlow: normWireRequired(normWireNumber) });
