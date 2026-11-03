/** ⚓️ `change-anchor-fixed-length` wire twin: the leaf payload `ChangeAnchorFixedLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorFixedLength {
  mutation: "changeAnchorFixedLength";
  id: string;
  newFixedLength: number;
}

export const parseChangeAnchorFixedLength: NormWireReader<ChangeAnchorFixedLength> = normWireObject<ChangeAnchorFixedLength>({ mutation: normWireRequired(normWireLiteral("changeAnchorFixedLength")), id: normWireRequired(normWireString), newFixedLength: normWireRequired(normWireNumber) });
