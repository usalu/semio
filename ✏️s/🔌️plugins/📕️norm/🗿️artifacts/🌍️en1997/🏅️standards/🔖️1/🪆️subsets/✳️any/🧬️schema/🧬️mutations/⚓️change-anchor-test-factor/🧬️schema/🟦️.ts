/** ⚓️ `change-anchor-test-factor` wire twin: the leaf payload `ChangeAnchorTestFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorTestFactor {
  mutation: "changeAnchorTestFactor";
  id: string;
  newTestFactor: number;
}

export const parseChangeAnchorTestFactor: NormWireReader<ChangeAnchorTestFactor> = normWireObject<ChangeAnchorTestFactor>({ mutation: normWireRequired(normWireLiteral("changeAnchorTestFactor")), id: normWireRequired(normWireString), newTestFactor: normWireRequired(normWireNumber) });
