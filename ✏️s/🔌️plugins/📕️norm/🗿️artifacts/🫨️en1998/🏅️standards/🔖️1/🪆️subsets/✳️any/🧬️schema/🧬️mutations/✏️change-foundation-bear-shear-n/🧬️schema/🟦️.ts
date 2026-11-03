/** ✏️ `change-foundation-bear-shear-n` wire twin: the leaf payload `ChangeFoundationBearShearN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBearShearN {
  mutation: "changeFoundationBearShearN";
  id: string;
  newBearingShearN: number;
}

export const parseChangeFoundationBearShearN: NormWireReader<ChangeFoundationBearShearN> = normWireObject<ChangeFoundationBearShearN>({ mutation: normWireRequired(normWireLiteral("changeFoundationBearShearN")), id: normWireRequired(normWireString), newBearingShearN: normWireRequired(normWireNumber) });
