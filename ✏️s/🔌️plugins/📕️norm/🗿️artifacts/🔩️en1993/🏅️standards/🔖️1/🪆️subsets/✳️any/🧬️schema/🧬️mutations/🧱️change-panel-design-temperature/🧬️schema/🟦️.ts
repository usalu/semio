/** 🧱️ `change-panel-design-temperature` wire twin: the leaf payload `ChangePanelDesignTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelDesignTemperature {
  id: string;
  newDesignTemperature: number;
}

export const parseChangePanelDesignTemperature: NormWireReader<ChangePanelDesignTemperature> = normWireObject<ChangePanelDesignTemperature>({ id: normWireRequired(normWireString), newDesignTemperature: normWireRequired(normWireNumber) });
