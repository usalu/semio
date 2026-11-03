/** ✏️ `change-foundation-bear-axial-n` wire twin: the leaf payload `ChangeFoundationBearAxialN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBearAxialN {
  mutation: "changeFoundationBearAxialN";
  id: string;
  newBearingAxialN: number;
}

export const parseChangeFoundationBearAxialN: NormWireReader<ChangeFoundationBearAxialN> = normWireObject<ChangeFoundationBearAxialN>({ mutation: normWireRequired(normWireLiteral("changeFoundationBearAxialN")), id: normWireRequired(normWireString), newBearingAxialN: normWireRequired(normWireNumber) });
