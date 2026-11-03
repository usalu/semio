/** 🌉 `change-bridge-ballast-depth` wire twin: the leaf payload `ChangeBridgeBallastDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeBallastDepth {
  mutation: "changeBridgeBallastDepth";
  id: string;
  newBallastDepth: number;
}

export const parseChangeBridgeBallastDepth: NormWireReader<ChangeBridgeBallastDepth> = normWireObject<ChangeBridgeBallastDepth>({ mutation: normWireRequired(normWireLiteral("changeBridgeBallastDepth")), id: normWireRequired(normWireString), newBallastDepth: normWireRequired(normWireNumber) });
