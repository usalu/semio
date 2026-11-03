/** ✏️ `change-pier-section-hm` wire twin: the leaf payload `ChangePierSectionHM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierSectionHM {
  mutation: "changePierSectionHM";
  id: string;
  newSectionHM: number;
}

export const parseChangePierSectionHM: NormWireReader<ChangePierSectionHM> = normWireObject<ChangePierSectionHM>({ mutation: normWireRequired(normWireLiteral("changePierSectionHM")), id: normWireRequired(normWireString), newSectionHM: normWireRequired(normWireNumber) });
