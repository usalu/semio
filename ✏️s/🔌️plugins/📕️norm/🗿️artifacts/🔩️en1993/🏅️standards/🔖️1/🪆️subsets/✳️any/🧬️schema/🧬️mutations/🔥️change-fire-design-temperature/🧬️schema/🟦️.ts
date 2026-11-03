/** 🔥️ `change-fire-design-temperature` wire twin: the leaf payload `ChangeFireDesignTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireDesignTemperature {
  id: string;
  newDesignTemperature: number;
}

export const parseChangeFireDesignTemperature: NormWireReader<ChangeFireDesignTemperature> = normWireObject<ChangeFireDesignTemperature>({ id: normWireRequired(normWireString), newDesignTemperature: normWireRequired(normWireNumber) });
