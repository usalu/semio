/** ⬇️ `change-patch-web-angle` wire twin: the leaf payload `ChangePatchWebAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchWebAngle {
  mutation: "changePatchWebAngle";
  patchLoadId: string;
  newWebAngle: number;
}

export const parseChangePatchWebAngle: NormWireReader<ChangePatchWebAngle> = normWireObject<ChangePatchWebAngle>({ mutation: normWireRequired(normWireLiteral("changePatchWebAngle")), patchLoadId: normWireRequired(normWireString), newWebAngle: normWireRequired(normWireNumber) });
