/** ⚙ `change-local-fire-heat-release-rate` wire twin: the leaf payload `ChangeLocalFireHeatReleaseRate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLocalFireHeatReleaseRate {
  id: string;
  newHeatReleaseRate: number;
}

export const parseChangeLocalFireHeatReleaseRate: NormWireReader<ChangeLocalFireHeatReleaseRate> = normWireObject<ChangeLocalFireHeatReleaseRate>({ id: normWireRequired(normWireString), newHeatReleaseRate: normWireRequired(normWireNumber) });
