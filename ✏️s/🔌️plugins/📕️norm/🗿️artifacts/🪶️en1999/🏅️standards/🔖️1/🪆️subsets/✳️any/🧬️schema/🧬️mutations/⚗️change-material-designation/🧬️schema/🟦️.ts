/** ⚗️ `change-material-designation` wire twin: the leaf payload `ChangeMaterialDesignation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialDesignation {
  mutation: "changeMaterialDesignation";
  materialId: string;
  newDesignation: string;
}

export const parseChangeMaterialDesignation: NormWireReader<ChangeMaterialDesignation> = normWireObject<ChangeMaterialDesignation>({ mutation: normWireRequired(normWireLiteral("changeMaterialDesignation")), materialId: normWireRequired(normWireString), newDesignation: normWireRequired(normWireString) });
