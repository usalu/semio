/** ✏️ `change-foundation-p-rd-pa` wire twin: the leaf payload `ChangeFoundationPRdPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPRdPa {
  mutation: "changeFoundationPRdPa";
  id: string;
  newPRdPa: number;
}

export const parseChangeFoundationPRdPa: NormWireReader<ChangeFoundationPRdPa> = normWireObject<ChangeFoundationPRdPa>({ mutation: normWireRequired(normWireLiteral("changeFoundationPRdPa")), id: normWireRequired(normWireString), newPRdPa: normWireRequired(normWireNumber) });
