/** ⚓️ `change-anchor-free-length` wire twin: the leaf payload `ChangeAnchorFreeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFreeLength {
  mutation: "changeAnchorFreeLength";
  id: string;
  newFreeLength: number;
}

export const parseChangeAnchorFreeLength: NormWireReader<ChangeAnchorFreeLength> = normWireObject<ChangeAnchorFreeLength>({ mutation: normWireRequired(normWireLiteral("changeAnchorFreeLength")), id: normWireRequired(normWireString), newFreeLength: normWireRequired(normWireNumber) });
