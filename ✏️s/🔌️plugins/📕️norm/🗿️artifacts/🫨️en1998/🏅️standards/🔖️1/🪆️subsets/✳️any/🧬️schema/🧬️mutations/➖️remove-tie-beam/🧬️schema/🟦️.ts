/** ➖️ `remove-tie-beam` wire twin: the leaf payload `RemoveTieBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTieBeam {
  mutation: "removeTieBeam";
  id: string;
}

export const parseRemoveTieBeam: NormWireReader<RemoveTieBeam> = normWireObject<RemoveTieBeam>({ mutation: normWireRequired(normWireLiteral("removeTieBeam")), id: normWireRequired(normWireString) });
