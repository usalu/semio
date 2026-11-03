/** ✏️ `change-bearing-xm` wire twin: the leaf payload `ChangeBearingXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingXM {
  mutation: "changeBearingXM";
  id: string;
  newXM: number;
}

export const parseChangeBearingXM: NormWireReader<ChangeBearingXM> = normWireObject<ChangeBearingXM>({ mutation: normWireRequired(normWireLiteral("changeBearingXM")), id: normWireRequired(normWireString), newXM: normWireRequired(normWireNumber) });
