/** ⚓️ `change-anchor-tendon` wire twin: the leaf payload `ChangeAnchorTendon`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TendonType, parseTendonType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnchorTendon {
  mutation: "changeAnchorTendon";
  id: string;
  newTendon: TendonType;
}

export const parseChangeAnchorTendon: NormWireReader<ChangeAnchorTendon> = normWireObject<ChangeAnchorTendon>({ mutation: normWireRequired(normWireLiteral("changeAnchorTendon")), id: normWireRequired(normWireString), newTendon: normWireRequired(parseTendonType) });
