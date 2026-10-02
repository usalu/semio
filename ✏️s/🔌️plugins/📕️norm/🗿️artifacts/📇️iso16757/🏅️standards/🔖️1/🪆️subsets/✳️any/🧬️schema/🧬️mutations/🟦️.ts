/** 🧺️ `Iso16757Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseRetireSubject, type RetireSubject } from "./✂️retire-subject/🧬️schema/🟦️.ts";
import { type IntroduceSubject, parseIntroduceSubject } from "./🌳️introduce-subject/🧬️schema/🟦️.ts";
import { type ChangePartNumberInput, parseChangePartNumberInput } from "./🎛️change-part-number-input/🧬️schema/🟦️.ts";
import { type ChangeSelectionClass, parseChangeSelectionClass } from "./🎯️change-selection-class/🧬️schema/🟦️.ts";
import { parseRenameManufacturer, type RenameManufacturer } from "./🏭️rename-manufacturer/🧬️schema/🟦️.ts";
import { type IntroduceProductClass, parseIntroduceProductClass } from "./🏷️introduce-product-class/🧬️schema/🟦️.ts";
import { parseRenameProduct, type RenameProduct } from "./🏷️rename-product/🧬️schema/🟦️.ts";
import { parseRenameCatalogue, type RenameCatalogue } from "./📇️rename-catalogue/🧬️schema/🟦️.ts";
import { type IntroduceGeometryObject, parseIntroduceGeometryObject } from "./📐introduce-geometry-object/🧬️schema/🟦️.ts";
import { type IntroducePropertyDefinition, parseIntroducePropertyDefinition } from "./📐️introduce-property-definition/🧬️schema/🟦️.ts";
import { type IntroduceProductSeries, parseIntroduceProductSeries } from "./📚introduce-product-series/🧬️schema/🟦️.ts";
import { type IntroduceProduct, parseIntroduceProduct } from "./📦️introduce-product/🧬️schema/🟦️.ts";
import { type ChangeExchangeProcess, parseChangeExchangeProcess } from "./🔄️change-exchange-process/🧬️schema/🟦️.ts";
import { parseRemovePartNumberInput, type RemovePartNumberInput } from "./🔌️remove-part-number-input/🧬️schema/🟦️.ts";
import { type IntroduceProductIndex, parseIntroduceProductIndex } from "./🔎introduce-product-index/🧬️schema/🟦️.ts";
import { type AddSelectionConstraint, parseAddSelectionConstraint } from "./🔒️add-selection-constraint/🧬️schema/🟦️.ts";
import { parseRemoveSelectionConstraint, type RemoveSelectionConstraint } from "./🔓️remove-selection-constraint/🧬️schema/🟦️.ts";
import { parseRenameProductGroup, type RenameProductGroup } from "./🗂️rename-product-group/🧬️schema/🟦️.ts";
import { parseRetireGeometryObject, type RetireGeometryObject } from "./🗑️retire-geometry-object/🧬️schema/🟦️.ts";
import { parseRetireProductClass, type RetireProductClass } from "./🗑️retire-product-class/🧬️schema/🟦️.ts";
import { parseRetireProductIndex, type RetireProductIndex } from "./🗑️retire-product-index/🧬️schema/🟦️.ts";
import { parseRetireProductSeries, type RetireProductSeries } from "./🗑️retire-product-series/🧬️schema/🟦️.ts";
import { parseReplacePartNumberRule, type ReplacePartNumberRule } from "./🧮️replace-part-number-rule/🧬️schema/🟦️.ts";
import { type ChangeSelectionSeries, parseChangeSelectionSeries } from "./🧵️change-selection-series/🧬️schema/🟦️.ts";
import { parseRetireProductGroup, type RetireProductGroup } from "./🧹️retire-product-group/🧬️schema/🟦️.ts";
import { type IntroduceProductGroup, parseIntroduceProductGroup } from "./🧺️introduce-product-group/🧬️schema/🟦️.ts";
import { parseRetirePropertyDefinition, type RetirePropertyDefinition } from "./🧽️retire-property-definition/🧬️schema/🟦️.ts";
import { type ChangeScriptLimits, parseChangeScriptLimits } from "./🚦️change-script-limits/🧬️schema/🟦️.ts";
import { parseRetireProduct, type RetireProduct } from "./🚫️retire-product/🧬️schema/🟦️.ts";

export type Iso16757Mutation =
  | { ChangeExchangeProcess: ChangeExchangeProcess }
  | { ChangeScriptLimits: ChangeScriptLimits }
  | { ReplacePartNumberRule: ReplacePartNumberRule }
  | { ChangePartNumberInput: ChangePartNumberInput }
  | { RemovePartNumberInput: RemovePartNumberInput }
  | { ChangeSelectionClass: ChangeSelectionClass }
  | { ChangeSelectionSeries: ChangeSelectionSeries }
  | { AddSelectionConstraint: AddSelectionConstraint }
  | { RemoveSelectionConstraint: RemoveSelectionConstraint }
  | { RenameCatalogue: RenameCatalogue }
  | { RenameManufacturer: RenameManufacturer }
  | { IntroduceProductGroup: IntroduceProductGroup }
  | { RetireProductGroup: RetireProductGroup }
  | { RenameProductGroup: RenameProductGroup }
  | { IntroduceProduct: IntroduceProduct }
  | { RetireProduct: RetireProduct }
  | { RenameProduct: RenameProduct }
  | { IntroducePropertyDefinition: IntroducePropertyDefinition }
  | { RetirePropertyDefinition: RetirePropertyDefinition }
  | { IntroduceSubject: IntroduceSubject }
  | { RetireSubject: RetireSubject }
  | { IntroduceProductClass: IntroduceProductClass }
  | { RetireProductClass: RetireProductClass }
  | { IntroduceProductSeries: IntroduceProductSeries }
  | { RetireProductSeries: RetireProductSeries }
  | { IntroduceProductIndex: IntroduceProductIndex }
  | { RetireProductIndex: RetireProductIndex }
  | { IntroduceGeometryObject: IntroduceGeometryObject }
  | { RetireGeometryObject: RetireGeometryObject };

export const parseIso16757Mutation: NormWireReader<Iso16757Mutation> = normWireExternal<Iso16757Mutation>({
  ChangeExchangeProcess: parseChangeExchangeProcess,
  ChangeScriptLimits: parseChangeScriptLimits,
  ReplacePartNumberRule: parseReplacePartNumberRule,
  ChangePartNumberInput: parseChangePartNumberInput,
  RemovePartNumberInput: parseRemovePartNumberInput,
  ChangeSelectionClass: parseChangeSelectionClass,
  ChangeSelectionSeries: parseChangeSelectionSeries,
  AddSelectionConstraint: parseAddSelectionConstraint,
  RemoveSelectionConstraint: parseRemoveSelectionConstraint,
  RenameCatalogue: parseRenameCatalogue,
  RenameManufacturer: parseRenameManufacturer,
  IntroduceProductGroup: parseIntroduceProductGroup,
  RetireProductGroup: parseRetireProductGroup,
  RenameProductGroup: parseRenameProductGroup,
  IntroduceProduct: parseIntroduceProduct,
  RetireProduct: parseRetireProduct,
  RenameProduct: parseRenameProduct,
  IntroducePropertyDefinition: parseIntroducePropertyDefinition,
  RetirePropertyDefinition: parseRetirePropertyDefinition,
  IntroduceSubject: parseIntroduceSubject,
  RetireSubject: parseRetireSubject,
  IntroduceProductClass: parseIntroduceProductClass,
  RetireProductClass: parseRetireProductClass,
  IntroduceProductSeries: parseIntroduceProductSeries,
  RetireProductSeries: parseRetireProductSeries,
  IntroduceProductIndex: parseIntroduceProductIndex,
  RetireProductIndex: parseRetireProductIndex,
  IntroduceGeometryObject: parseIntroduceGeometryObject,
  RetireGeometryObject: parseRetireGeometryObject,
});
