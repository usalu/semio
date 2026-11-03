/** ⬇️ `change-patch-position` wire twin: the leaf payload `ChangePatchPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PatchLoadPosition, parsePatchLoadPosition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePatchPosition {
  mutation: "changePatchPosition";
  patchLoadId: string;
  newPosition: PatchLoadPosition;
}

export const parseChangePatchPosition: NormWireReader<ChangePatchPosition> = normWireObject<ChangePatchPosition>({ mutation: normWireRequired(normWireLiteral("changePatchPosition")), patchLoadId: normWireRequired(normWireString), newPosition: normWireRequired(parsePatchLoadPosition) });
