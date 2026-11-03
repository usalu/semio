/** ⬇️ `change-patch-corner-radius` wire twin: the leaf payload `ChangePatchCornerRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchCornerRadius {
  mutation: "changePatchCornerRadius";
  patchLoadId: string;
  newCornerRadius: number;
}

export const parseChangePatchCornerRadius: NormWireReader<ChangePatchCornerRadius> = normWireObject<ChangePatchCornerRadius>({ mutation: normWireRequired(normWireLiteral("changePatchCornerRadius")), patchLoadId: normWireRequired(normWireString), newCornerRadius: normWireRequired(normWireNumber) });
