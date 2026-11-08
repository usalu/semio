/** 📍️ `change-critical-region-steel-fyd` wire twin: the leaf payload `ChangeCriticalRegionSteelDesignYieldMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionSteelDesignYieldMpa {
  mutation: "changeCriticalRegionSteelDesignYieldMpa";
  id: string;
  newSteelDesignYieldMpa: number;
}

export const parseChangeCriticalRegionSteelDesignYieldMpa: NormWireReader<ChangeCriticalRegionSteelDesignYieldMpa> = normWireObject<ChangeCriticalRegionSteelDesignYieldMpa>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionSteelDesignYieldMpa")), id: normWireRequired(normWireString), newSteelDesignYieldMpa: normWireRequired(normWireNumber) });
