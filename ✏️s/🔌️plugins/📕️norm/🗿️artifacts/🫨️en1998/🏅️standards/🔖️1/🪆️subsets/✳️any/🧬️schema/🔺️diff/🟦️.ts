/** 🔺️ `En1998Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireObject, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Assessment, type En1998Bridge, type En1998Building, type En1998Foundation, type En1998RetainingWall, type En1998Silo, type En1998Site, type En1998Tank, type En1998Tower, parseEn1998Assessment, parseEn1998Bridge, parseEn1998Building, parseEn1998Foundation, parseEn1998RetainingWall, parseEn1998Silo, parseEn1998Site, parseEn1998Tank, parseEn1998Tower } from "../📸️snapshot/🟦️.ts";
import { type En1998Artifact, parseEn1998Artifact } from "../🟦️.ts";

export interface En1998Diff {
  /** @state artifact */
  artifact: En1998Artifact | null;
  /** @state artifact */
  annex: string | null;
  /** @state artifact */
  site: En1998Site | null;
  /** @state artifact */
  buildings: En1998Building[] | null;
  /** @state artifact */
  bridges: En1998Bridge[] | null;
  /** @state artifact */
  assessments: En1998Assessment[] | null;
  /** @state artifact */
  silos: En1998Silo[] | null;
  /** @state artifact */
  tanks: En1998Tank[] | null;
  /** @state artifact */
  foundations: En1998Foundation[] | null;
  /** @state artifact */
  retainingWalls: En1998RetainingWall[] | null;
  /** @state artifact */
  towers: En1998Tower[] | null;
}

export const parseEn1998Diff: NormWireReader<En1998Diff> = normWireObject<En1998Diff>({ artifact: normWireDefault(normWireNullable(parseEn1998Artifact), () => null), annex: normWireDefault(normWireNullable(normWireString), () => null), site: normWireDefault(normWireNullable(parseEn1998Site), () => null), buildings: normWireDefault(normWireNullable(normWireArray(parseEn1998Building)), () => null), bridges: normWireDefault(normWireNullable(normWireArray(parseEn1998Bridge)), () => null), assessments: normWireDefault(normWireNullable(normWireArray(parseEn1998Assessment)), () => null), silos: normWireDefault(normWireNullable(normWireArray(parseEn1998Silo)), () => null), tanks: normWireDefault(normWireNullable(normWireArray(parseEn1998Tank)), () => null), foundations: normWireDefault(normWireNullable(normWireArray(parseEn1998Foundation)), () => null), retainingWalls: normWireDefault(normWireNullable(normWireArray(parseEn1998RetainingWall)), () => null), towers: normWireDefault(normWireNullable(normWireArray(parseEn1998Tower)), () => null) });
