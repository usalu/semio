/** ✏️ `change-pier-section-bm` wire twin: the leaf payload `ChangePierSectionBM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierSectionBM {
  mutation: "changePierSectionBM";
  id: string;
  newSectionBM: number;
}

export const parseChangePierSectionBM: NormWireReader<ChangePierSectionBM> = normWireObject<ChangePierSectionBM>({ mutation: normWireRequired(normWireLiteral("changePierSectionBM")), id: normWireRequired(normWireString), newSectionBM: normWireRequired(normWireNumber) });
