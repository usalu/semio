/** 🔺️ `En1993Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireDefault, normWireJson, normWireMap, normWireNullable, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1993Diff {
  artifact: { [key: string]: NormJson } | null;
  annex: string | null;
  materials: { values?: NormJson[]; } | null;
  sections: { values?: NormJson[]; } | null;
  members: { values?: NormJson[]; } | null;
  loadCases: { values?: NormJson[]; } | null;
  memberActions: { values?: NormJson[]; } | null;
  joints: { values?: NormJson[]; } | null;
  fatigueDetails: { values?: NormJson[]; } | null;
  fireExposures: { values?: NormJson[]; } | null;
  coldFormedMembers: { values?: NormJson[]; } | null;
  platedPanels: { values?: NormJson[]; } | null;
  siloShells: { values?: NormJson[]; } | null;
  tensionComponents: { values?: NormJson[]; } | null;
  bridgeFatigue: { values?: NormJson[]; } | null;
  towerLegs: { values?: NormJson[]; } | null;
  piles: { values?: NormJson[]; } | null;
  craneRunways: { values?: NormJson[]; } | null;
}

export const parseEn1993Diff: NormWireReader<En1993Diff> = normWireObject<En1993Diff>({ artifact: normWireDefault(normWireNullable(normWireMap(normWireJson)), () => null), annex: normWireDefault(normWireNullable(normWireString), () => null), materials: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), sections: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), members: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), loadCases: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), memberActions: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), joints: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), fatigueDetails: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), fireExposures: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), coldFormedMembers: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), platedPanels: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), siloShells: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), tensionComponents: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), bridgeFatigue: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), towerLegs: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), piles: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null), craneRunways: normWireDefault(normWireNullable(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), () => null) });
