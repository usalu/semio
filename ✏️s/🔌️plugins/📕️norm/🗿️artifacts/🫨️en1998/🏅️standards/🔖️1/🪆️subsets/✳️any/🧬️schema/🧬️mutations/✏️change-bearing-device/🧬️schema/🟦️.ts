/** ✏️ `change-bearing-device` wire twin: the leaf payload `ChangeBearingDevice`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BearingDevice, parseBearingDevice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBearingDevice {
  mutation: "changeBearingDevice";
  id: string;
  newDevice: BearingDevice;
}

export const parseChangeBearingDevice: NormWireReader<ChangeBearingDevice> = normWireObject<ChangeBearingDevice>({ mutation: normWireRequired(normWireLiteral("changeBearingDevice")), id: normWireRequired(normWireString), newDevice: normWireRequired(parseBearingDevice) });
