/** ⚓️ `change-anchor-min-free-length` wire twin: the leaf payload `ChangeAnchorMinFreeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorMinFreeLength {
  mutation: "changeAnchorMinFreeLength";
  id: string;
  newMinFreeLength: number;
}

export const parseChangeAnchorMinFreeLength: NormWireReader<ChangeAnchorMinFreeLength> = normWireObject<ChangeAnchorMinFreeLength>({ mutation: normWireRequired(normWireLiteral("changeAnchorMinFreeLength")), id: normWireRequired(normWireString), newMinFreeLength: normWireRequired(normWireNumber) });
