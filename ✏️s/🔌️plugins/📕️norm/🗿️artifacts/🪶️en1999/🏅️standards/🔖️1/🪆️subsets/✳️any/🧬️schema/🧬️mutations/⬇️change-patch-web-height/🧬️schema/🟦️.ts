/** ⬇️ `change-patch-web-height` wire twin: the leaf payload `ChangePatchWebHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchWebHeight {
  mutation: "changePatchWebHeight";
  patchLoadId: string;
  newWebHeight: number;
}

export const parseChangePatchWebHeight: NormWireReader<ChangePatchWebHeight> = normWireObject<ChangePatchWebHeight>({ mutation: normWireRequired(normWireLiteral("changePatchWebHeight")), patchLoadId: normWireRequired(normWireString), newWebHeight: normWireRequired(normWireNumber) });
