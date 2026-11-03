/** ✏️ `change-silo-permanent-gk-n` wire twin: the leaf payload `ChangeSiloPermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloPermanentGkN {
  mutation: "changeSiloPermanentGkN";
  id: string;
  newPermanentGkN: number;
}

export const parseChangeSiloPermanentGkN: NormWireReader<ChangeSiloPermanentGkN> = normWireObject<ChangeSiloPermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeSiloPermanentGkN")), id: normWireRequired(normWireString), newPermanentGkN: normWireRequired(normWireNumber) });
