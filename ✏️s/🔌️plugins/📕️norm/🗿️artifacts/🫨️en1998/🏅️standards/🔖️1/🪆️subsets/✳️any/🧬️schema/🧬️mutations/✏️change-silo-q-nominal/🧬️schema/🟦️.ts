/** ✏️ `change-silo-q-nominal` wire twin: the leaf payload `ChangeSiloQNominal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloQNominal {
  mutation: "changeSiloQNominal";
  id: string;
  newQNominal: number;
}

export const parseChangeSiloQNominal: NormWireReader<ChangeSiloQNominal> = normWireObject<ChangeSiloQNominal>({ mutation: normWireRequired(normWireLiteral("changeSiloQNominal")), id: normWireRequired(normWireString), newQNominal: normWireRequired(normWireNumber) });
