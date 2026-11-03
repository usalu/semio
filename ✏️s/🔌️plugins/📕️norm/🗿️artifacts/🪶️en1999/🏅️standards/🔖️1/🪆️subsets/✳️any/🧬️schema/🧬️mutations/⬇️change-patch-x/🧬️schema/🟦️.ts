/** ⬇️ `change-patch-x` wire twin: the leaf payload `ChangePatchX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchX {
  mutation: "changePatchX";
  patchLoadId: string;
  newX: number;
}

export const parseChangePatchX: NormWireReader<ChangePatchX> = normWireObject<ChangePatchX>({ mutation: normWireRequired(normWireLiteral("changePatchX")), patchLoadId: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
