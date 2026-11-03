/** 🧷️ `change-restraint-design-moment` wire twin: the leaf payload `ChangeRestraintDesignMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintDesignMoment {
  id: string;
  newDesignMoment: number;
}

export const parseChangeRestraintDesignMoment: NormWireReader<ChangeRestraintDesignMoment> = normWireObject<ChangeRestraintDesignMoment>({ id: normWireRequired(normWireString), newDesignMoment: normWireRequired(normWireNumber) });
