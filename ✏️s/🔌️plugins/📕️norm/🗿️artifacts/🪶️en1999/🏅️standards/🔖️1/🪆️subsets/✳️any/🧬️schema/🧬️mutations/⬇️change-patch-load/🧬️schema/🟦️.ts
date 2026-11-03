/** ⬇️ `change-patch-load` wire twin: the leaf payload `ChangePatchLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchLoad {
  mutation: "changePatchLoad";
  patchLoadId: string;
  newLoad: number;
}

export const parseChangePatchLoad: NormWireReader<ChangePatchLoad> = normWireObject<ChangePatchLoad>({ mutation: normWireRequired(normWireLiteral("changePatchLoad")), patchLoadId: normWireRequired(normWireString), newLoad: normWireRequired(normWireNumber) });
