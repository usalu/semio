/** ⬇️ `change-patch-bearing-length` wire twin: the leaf payload `ChangePatchBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePatchBearingLength {
  mutation: "changePatchBearingLength";
  patchLoadId: string;
  newBearingLength: number;
}

export const parseChangePatchBearingLength: NormWireReader<ChangePatchBearingLength> = normWireObject<ChangePatchBearingLength>({ mutation: normWireRequired(normWireLiteral("changePatchBearingLength")), patchLoadId: normWireRequired(normWireString), newBearingLength: normWireRequired(normWireNumber) });
