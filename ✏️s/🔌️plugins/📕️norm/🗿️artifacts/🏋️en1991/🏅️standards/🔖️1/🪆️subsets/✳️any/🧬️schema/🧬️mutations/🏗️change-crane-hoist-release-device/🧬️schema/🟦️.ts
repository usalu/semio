/** 🏗 `change-crane-hoist-release-device` wire twin: the leaf payload `ChangeCraneHoistReleaseDevice`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneHoistReleaseDevice {
  id: string;
  newReleaseDevice: string;
}

export const parseChangeCraneHoistReleaseDevice: NormWireReader<ChangeCraneHoistReleaseDevice> = normWireObject<ChangeCraneHoistReleaseDevice>({ id: normWireRequired(normWireString), newReleaseDevice: normWireRequired(normWireString) });
