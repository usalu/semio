/** ✏️ `change-foundation-passive-resistance-n` wire twin: the leaf payload `ChangeFoundationPassiveResistanceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPassiveResistanceN {
  mutation: "changeFoundationPassiveResistanceN";
  id: string;
  newPassiveResistanceN: number;
}

export const parseChangeFoundationPassiveResistanceN: NormWireReader<ChangeFoundationPassiveResistanceN> = normWireObject<ChangeFoundationPassiveResistanceN>({ mutation: normWireRequired(normWireLiteral("changeFoundationPassiveResistanceN")), id: normWireRequired(normWireString), newPassiveResistanceN: normWireRequired(normWireNumber) });
