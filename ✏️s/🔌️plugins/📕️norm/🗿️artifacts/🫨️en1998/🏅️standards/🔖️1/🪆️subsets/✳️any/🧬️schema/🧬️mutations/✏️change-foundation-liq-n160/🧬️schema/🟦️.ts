/** ✏️ `change-foundation-liq-n160` wire twin: the leaf payload `ChangeFoundationLiqN160`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqN160 {
  mutation: "changeFoundationLiqN160";
  id: string;
  newLiqN160: number;
}

export const parseChangeFoundationLiqN160: NormWireReader<ChangeFoundationLiqN160> = normWireObject<ChangeFoundationLiqN160>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqN160")), id: normWireRequired(normWireString), newLiqN160: normWireRequired(normWireNumber) });
