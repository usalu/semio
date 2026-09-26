/** 🧬️ Iso16757 diff schema — sparse field delta over typed snapshot lanes. */

import type {
  Catalogue,
  CatalogueValue,
  Dictionary,
  GeometryCatalogue,
  Iso16757Snapshot,
  PartNumberRule,
  ScriptLimits,
  SelectionRequest,
} from "../📸️snapshot/🟦️";

export type Iso16757Artifact = Iso16757Snapshot;

export interface Iso16757Diff {
  /** @state artifact */
  artifact?: Iso16757Artifact;
  /** @state artifact */
  catalogue?: Catalogue;
  /** @state artifact */
  dictionary?: Dictionary;
  /** @state artifact */
  geometry?: GeometryCatalogue;
  /** @state artifact */
  selection?: SelectionRequest;
  /** @state artifact */
  partNumberRule?: PartNumberRule;
  /** @state artifact */
  partNumberInputs?: Record<string, CatalogueValue>;
  /** @state artifact */
  scriptLimits?: ScriptLimits;
  /** @state artifact */
  exchangeProcess?: string;
  selectedCheckIndex?: number | null;
}
