/** ✏️ `change-pier-cover-m` wire twin: the leaf payload `ChangePierCoverM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierCoverM {
  mutation: "changePierCoverM";
  id: string;
  newCoverM: number;
}

export const parseChangePierCoverM: NormWireReader<ChangePierCoverM> = normWireObject<ChangePierCoverM>({ mutation: normWireRequired(normWireLiteral("changePierCoverM")), id: normWireRequired(normWireString), newCoverM: normWireRequired(normWireNumber) });
