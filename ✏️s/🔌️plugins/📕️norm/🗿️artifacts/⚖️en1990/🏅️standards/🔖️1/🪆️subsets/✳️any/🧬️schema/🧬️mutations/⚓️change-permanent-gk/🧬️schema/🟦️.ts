/** ⚓️ `change-permanent-gk` wire twin: the leaf payload `ChangePermanentGk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePermanentGk {
  mutation: "changePermanentGk";
  id: string;
  newGk: number;
}

export const parseChangePermanentGk: NormWireReader<ChangePermanentGk> = normWireObject<ChangePermanentGk>({ mutation: normWireRequired(normWireLiteral("changePermanentGk")), id: normWireRequired(normWireString), newGk: normWireRequired(normWireNumber) });
