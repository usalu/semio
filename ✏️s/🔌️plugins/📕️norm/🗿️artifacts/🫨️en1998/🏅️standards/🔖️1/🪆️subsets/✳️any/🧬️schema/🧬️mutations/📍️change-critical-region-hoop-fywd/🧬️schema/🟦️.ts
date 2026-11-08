/** 📍️ `change-critical-region-hoop-fywd` wire twin: the leaf payload `ChangeCriticalRegionHoopDesignYieldMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionHoopDesignYieldMpa {
  mutation: "changeCriticalRegionHoopDesignYieldMpa";
  id: string;
  newHoopDesignYieldMpa: number;
}

export const parseChangeCriticalRegionHoopDesignYieldMpa: NormWireReader<ChangeCriticalRegionHoopDesignYieldMpa> = normWireObject<ChangeCriticalRegionHoopDesignYieldMpa>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionHoopDesignYieldMpa")), id: normWireRequired(normWireString), newHoopDesignYieldMpa: normWireRequired(normWireNumber) });
