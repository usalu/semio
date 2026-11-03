/** 🧪️ `change-lab-test-design-life` wire twin: the leaf payload `ChangeLabTestDesignLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestDesignLife {
  mutation: "changeLabTestDesignLife";
  id: string;
  newDesignLife: number;
}

export const parseChangeLabTestDesignLife: NormWireReader<ChangeLabTestDesignLife> = normWireObject<ChangeLabTestDesignLife>({ mutation: normWireRequired(normWireLiteral("changeLabTestDesignLife")), id: normWireRequired(normWireString), newDesignLife: normWireRequired(normWireNumber) });
