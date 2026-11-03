/** ⚙ `change-local-fire-assumed-flux` wire twin: the leaf payload `ChangeLocalFireAssumedFlux`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLocalFireAssumedFlux {
  id: string;
  newAssumedFlux: number;
}

export const parseChangeLocalFireAssumedFlux: NormWireReader<ChangeLocalFireAssumedFlux> = normWireObject<ChangeLocalFireAssumedFlux>({ id: normWireRequired(normWireString), newAssumedFlux: normWireRequired(normWireNumber) });
