/** 🧺️ `En1990Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeReferencePeriodYears, parseChangeReferencePeriodYears } from "./⏱️change-reference-period-years/🧬️schema/🟦️.ts";
import { type ChangePermanents, parseChangePermanents } from "./⚓️change-permanents/🧬️schema/🟦️.ts";
import { type ChangeConsequenceClass, parseChangeConsequenceClass } from "./⚠️change-consequence-class/🧬️schema/🟦️.ts";
import { type ChangeAltitudeM, parseChangeAltitudeM } from "./⛰️change-altitude-m/🧬️schema/🟦️.ts";
import { parseRemoveEffect, type RemoveEffect } from "./✂️remove-effect/🧬️schema/🟦️.ts";
import { type InsertPermanent, parseInsertPermanent } from "./➕insert-permanent/🧬️schema/🟦️.ts";
import { parseRemovePermanent, type RemovePermanent } from "./➖remove-permanent/🧬️schema/🟦️.ts";
import { type ChangeBridgeSls, parseChangeBridgeSls } from "./🌉change-bridge-sls/🧬️schema/🟦️.ts";
import { type ChangeSeismics, parseChangeSeismics } from "./🌋️change-seismics/🧬️schema/🟦️.ts";
import { type InsertSeismic, parseInsertSeismic } from "./🌋insert-seismic/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeReliabilityClass, parseChangeReliabilityClass } from "./🎯change-reliability-class/🧬️schema/🟦️.ts";
import { type ChangeVariables, parseChangeVariables } from "./🏋️change-variables/🧬️schema/🟦️.ts";
import { type ChangeMembers, parseChangeMembers } from "./🏗️change-members/🧬️schema/🟦️.ts";
import { type ChangeProjectId, parseChangeProjectId } from "./🏷️change-project-id/🧬️schema/🟦️.ts";
import { type ChangeSupervisionLevel, parseChangeSupervisionLevel } from "./👁️change-supervision-level/🧬️schema/🟦️.ts";
import { type InsertAccidental, parseInsertAccidental } from "./💣insert-accidental/🧬️schema/🟦️.ts";
import { type ChangeAccidentals, parseChangeAccidentals } from "./💥change-accidentals/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLifeCategory, parseChangeDesignWorkingLifeCategory } from "./📅change-design-working-life-category/🧬️schema/🟦️.ts";
import { type ChangeDesignWorkingLifeYears, parseChangeDesignWorkingLifeYears } from "./📆change-design-working-life-years/🧬️schema/🟦️.ts";
import { type InsertEffect, parseInsertEffect } from "./📎insert-effect/🧬️schema/🟦️.ts";
import { type ChangeBetaComputed, parseChangeBetaComputed } from "./📐change-beta-computed/🧬️schema/🟦️.ts";
import { parseRemoveVariable, type RemoveVariable } from "./📤remove-variable/🧬️schema/🟦️.ts";
import { type InsertVariable, parseInsertVariable } from "./📥insert-variable/🧬️schema/🟦️.ts";
import { type ChangeInspectionLevel, parseChangeInspectionLevel } from "./🔍change-inspection-level/🧬️schema/🟦️.ts";
import { type ChangeEffects, parseChangeEffects } from "./🔗change-effects/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./🔩insert-member/🧬️schema/🟦️.ts";
import { parseRemoveSeismic, type RemoveSeismic } from "./🕳️remove-seismic/🧬️schema/🟦️.ts";
import { parseRemoveAccidental, type RemoveAccidental } from "./🧯remove-accidental/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "./🪚remove-member/🧬️schema/🟦️.ts";

export type En1990Mutation =
  | ChangeAnnex
  | ChangeProjectId
  | ChangeAltitudeM
  | ChangeConsequenceClass
  | ChangeReliabilityClass
  | ChangeDesignWorkingLifeCategory
  | ChangeDesignWorkingLifeYears
  | ChangeReferencePeriodYears
  | ChangeSupervisionLevel
  | ChangeInspectionLevel
  | ChangeBetaComputed
  | ChangePermanents
  | ChangeVariables
  | ChangeAccidentals
  | ChangeSeismics
  | ChangeMembers
  | ChangeBridgeSls
  | ChangeEffects
  | RemoveEffect
  | RemoveMember
  | RemoveSeismic
  | RemoveAccidental
  | RemoveVariable
  | RemovePermanent
  | InsertEffect
  | InsertMember
  | InsertSeismic
  | InsertAccidental
  | InsertVariable
  | InsertPermanent;

export const parseEn1990Mutation: NormWireReader<En1990Mutation> = normWireTagged<En1990Mutation, "mutation">("mutation", {
  changeAnnex: parseChangeAnnex,
  changeProjectId: parseChangeProjectId,
  changeAltitudeM: parseChangeAltitudeM,
  changeConsequenceClass: parseChangeConsequenceClass,
  changeReliabilityClass: parseChangeReliabilityClass,
  changeDesignWorkingLifeCategory: parseChangeDesignWorkingLifeCategory,
  changeDesignWorkingLifeYears: parseChangeDesignWorkingLifeYears,
  changeReferencePeriodYears: parseChangeReferencePeriodYears,
  changeSupervisionLevel: parseChangeSupervisionLevel,
  changeInspectionLevel: parseChangeInspectionLevel,
  changeBetaComputed: parseChangeBetaComputed,
  changePermanents: parseChangePermanents,
  changeVariables: parseChangeVariables,
  changeAccidentals: parseChangeAccidentals,
  changeSeismics: parseChangeSeismics,
  changeMembers: parseChangeMembers,
  changeBridgeSls: parseChangeBridgeSls,
  changeEffects: parseChangeEffects,
  removeEffect: parseRemoveEffect,
  removeMember: parseRemoveMember,
  removeSeismic: parseRemoveSeismic,
  removeAccidental: parseRemoveAccidental,
  removeVariable: parseRemoveVariable,
  removePermanent: parseRemovePermanent,
  insertEffect: parseInsertEffect,
  insertMember: parseInsertMember,
  insertSeismic: parseInsertSeismic,
  insertAccidental: parseInsertAccidental,
  insertVariable: parseInsertVariable,
  insertPermanent: parseInsertPermanent,
});
