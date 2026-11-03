/** ✏️ `change-foundation-area-m2` wire twin: the leaf payload `ChangeFoundationAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationAreaM2 {
  mutation: "changeFoundationAreaM2";
  id: string;
  newAreaM2: number;
}

export const parseChangeFoundationAreaM2: NormWireReader<ChangeFoundationAreaM2> = normWireObject<ChangeFoundationAreaM2>({ mutation: normWireRequired(normWireLiteral("changeFoundationAreaM2")), id: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireNumber) });
