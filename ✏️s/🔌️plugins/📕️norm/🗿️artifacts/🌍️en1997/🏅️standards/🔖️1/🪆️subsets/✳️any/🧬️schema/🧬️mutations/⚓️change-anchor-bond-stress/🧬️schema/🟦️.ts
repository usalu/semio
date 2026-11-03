/** ⚓️ `change-anchor-bond-stress` wire twin: the leaf payload `ChangeAnchorBondStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorBondStress {
  mutation: "changeAnchorBondStress";
  id: string;
  newBondStress: number;
}

export const parseChangeAnchorBondStress: NormWireReader<ChangeAnchorBondStress> = normWireObject<ChangeAnchorBondStress>({ mutation: normWireRequired(normWireLiteral("changeAnchorBondStress")), id: normWireRequired(normWireString), newBondStress: normWireRequired(normWireNumber) });
