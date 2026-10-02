/** 🧺️ `En1994Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeBeamStudCount, parseChangeBeamStudCount } from "./#️⃣change-beam-stud-count/🧬️schema/🟦️.ts";
import { type ChangeBeamTransverseAs, parseChangeBeamTransverseAs } from "./↔️change-beam-transverse-as/🧬️schema/🟦️.ts";
import { type ChangeColumnKind, parseChangeColumnKind } from "./↪️change-column-kind/🧬️schema/🟦️.ts";
import { parseRemoveColumn, type RemoveColumn } from "./⛔️remove-column/🧬️schema/🟦️.ts";
import { type ChangeBeamStudSpacingM, parseChangeBeamStudSpacingM } from "./✂️change-beam-stud-spacing-m/🧬️schema/🟦️.ts";
import { type InsertBeam, parseInsertBeam } from "./➕️insert-beam/🧬️schema/🟦️.ts";
import { type InsertSlab, parseInsertSlab } from "./➕insert-slab/🧬️schema/🟦️.ts";
import { parseRemoveBeam, type RemoveBeam } from "./➖️remove-beam/🧬️schema/🟦️.ts";
import { parseRemoveSlab, type RemoveSlab } from "./➖remove-slab/🧬️schema/🟦️.ts";
import { type InsertColumn, parseInsertColumn } from "./➗️insert-column/🧬️schema/🟦️.ts";
import { type ChangeColumnActionForceN, parseChangeColumnActionForceN } from "./⬇️change-column-action-force-n/🧬️schema/🟦️.ts";
import { type ChangeBeamStudDiameterM, parseChangeBeamStudDiameterM } from "./⭕️change-beam-stud-diameter-m/🧬️schema/🟦️.ts";
import { type ChangeBeamActionQAreaPa, parseChangeBeamActionQAreaPa } from "./🌀️change-beam-action-q-area-pa/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeSteelFYPa, parseChangeSteelFYPa } from "./🏋️change-steel-fy-pa/🧬️schema/🟦️.ts";
import { type ChangeStructureKind, parseChangeStructureKind } from "./🏗️change-structure-kind/🧬️schema/🟦️.ts";
import { type ChangeBeamStudFUPa, parseChangeBeamStudFUPa } from "./💪️change-beam-stud-fu-pa/🧬️schema/🟦️.ts";
import { type ChangeBeamSpanM, parseChangeBeamSpanM } from "./📏️change-beam-span-m/🧬️schema/🟦️.ts";
import { type ChangeSlabThicknessM, parseChangeSlabThicknessM } from "./📏change-slab-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeSlabActionQAreaPa, parseChangeSlabActionQAreaPa } from "./📐️change-slab-action-q-area-pa/🧬️schema/🟦️.ts";
import { type ChangeFatigueDetail, parseChangeFatigueDetail } from "./🔁️change-fatigue-detail/🧬️schema/🟦️.ts";
import { type ChangeFireRating, parseChangeFireRating } from "./🔥️change-fire-rating/🧬️schema/🟦️.ts";
import { type ChangeInsulationThicknessM, parseChangeInsulationThicknessM } from "./🧯️change-insulation-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeBeamSlabThicknessM, parseChangeBeamSlabThicknessM } from "./🧱change-beam-slab-thickness-m/🧬️schema/🟦️.ts";
import { type ChangeBeamConstruction, parseChangeBeamConstruction } from "./🛠️change-beam-construction/🧬️schema/🟦️.ts";

export type En1994Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeStructureKind: ChangeStructureKind }
  | { ChangeSteelFYPa: ChangeSteelFYPa }
  | { ChangeFireRating: ChangeFireRating }
  | { ChangeInsulationThicknessM: ChangeInsulationThicknessM }
  | { ChangeFatigueDetail: ChangeFatigueDetail }
  | { InsertBeam: InsertBeam }
  | { RemoveBeam: RemoveBeam }
  | { ChangeBeamActionQAreaPa: ChangeBeamActionQAreaPa }
  | { ChangeBeamStudSpacingM: ChangeBeamStudSpacingM }
  | { ChangeBeamSpanM: ChangeBeamSpanM }
  | { ChangeBeamSlabThicknessM: ChangeBeamSlabThicknessM }
  | { ChangeBeamStudDiameterM: ChangeBeamStudDiameterM }
  | { ChangeBeamStudCount: ChangeBeamStudCount }
  | { ChangeBeamStudFUPa: ChangeBeamStudFUPa }
  | { ChangeBeamTransverseAs: ChangeBeamTransverseAs }
  | { ChangeBeamConstruction: ChangeBeamConstruction }
  | { InsertColumn: InsertColumn }
  | { RemoveColumn: RemoveColumn }
  | { ChangeColumnActionForceN: ChangeColumnActionForceN }
  | { ChangeColumnKind: ChangeColumnKind }
  | { InsertSlab: InsertSlab }
  | { RemoveSlab: RemoveSlab }
  | { ChangeSlabActionQAreaPa: ChangeSlabActionQAreaPa }
  | { ChangeSlabThicknessM: ChangeSlabThicknessM };

export const parseEn1994Mutation: NormWireReader<En1994Mutation> = normWireExternal<En1994Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeStructureKind: parseChangeStructureKind,
  ChangeSteelFYPa: parseChangeSteelFYPa,
  ChangeFireRating: parseChangeFireRating,
  ChangeInsulationThicknessM: parseChangeInsulationThicknessM,
  ChangeFatigueDetail: parseChangeFatigueDetail,
  InsertBeam: parseInsertBeam,
  RemoveBeam: parseRemoveBeam,
  ChangeBeamActionQAreaPa: parseChangeBeamActionQAreaPa,
  ChangeBeamStudSpacingM: parseChangeBeamStudSpacingM,
  ChangeBeamSpanM: parseChangeBeamSpanM,
  ChangeBeamSlabThicknessM: parseChangeBeamSlabThicknessM,
  ChangeBeamStudDiameterM: parseChangeBeamStudDiameterM,
  ChangeBeamStudCount: parseChangeBeamStudCount,
  ChangeBeamStudFUPa: parseChangeBeamStudFUPa,
  ChangeBeamTransverseAs: parseChangeBeamTransverseAs,
  ChangeBeamConstruction: parseChangeBeamConstruction,
  InsertColumn: parseInsertColumn,
  RemoveColumn: parseRemoveColumn,
  ChangeColumnActionForceN: parseChangeColumnActionForceN,
  ChangeColumnKind: parseChangeColumnKind,
  InsertSlab: parseInsertSlab,
  RemoveSlab: parseRemoveSlab,
  ChangeSlabActionQAreaPa: parseChangeSlabActionQAreaPa,
  ChangeSlabThicknessM: parseChangeSlabThicknessM,
});
