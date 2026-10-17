// #region Header
/** 🌱️ Closed browser twin of the schema-owned host artifact-creation contract. */
// #endregion Header

export const SPACE_ARTIFACT_CREATION_MAX_BYTES = 4096;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES = 65_536;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS = 64;
export const SPACE_ARTIFACT_CREATE_SCHEMA_V1 = "semio.hub.space-artifact-create/v1";
export const SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1 = "semio.hub.space-artifact-creation-catalog/v1";
export const SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1 = "semio.hub.space-artifact-creation-status/v1";

export type SpaceArtifactCreateV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATE_SCHEMA_V1;
  requestId: string;
  expectedCatalogGenerationId: string;
  kindId: string;
  name: string;
}>;

export type SpaceArtifactCreationPhaseV1 = "accepted" | "preparing" | "ready" | "indeterminate" | "failed" | "cancelled";

export type SpaceArtifactCreationDialectV1 = Readonly<{
  artifactKind: string;
  standard: string;
  subset: string;
}>;

export type SpaceArtifactCreationReadyV1 = Readonly<{
  artifactId: string;
  kindId: string;
  artifactSchema: string;
  parentDialect: SpaceArtifactCreationDialectV1;
}>;

/** 📈️ Where a running creation is; only `accepted`/`preparing` report it, and `completedUnits <= totalUnits`. */
export type SpaceArtifactCreationStageV1 = "queued" | "compiling-guest" | "genesis" | "publishing";

export type SpaceArtifactCreationProgressV1 = Readonly<{
  stage: SpaceArtifactCreationStageV1;
  completedUnits: number;
  totalUnits: number;
}>;

export type SpaceArtifactCreationStatusV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1;
  requestId: string;
  spaceId: string;
  catalogGenerationId: string;
  phase: SpaceArtifactCreationPhaseV1;
  ready?: SpaceArtifactCreationReadyV1;
  progress?: SpaceArtifactCreationProgressV1;
}>;

export type SpaceArtifactCreationKindV1 = Readonly<{
  kindId: string;
  schema: string;
  dialect: SpaceArtifactCreationDialectV1;
  label: Readonly<{ en: string; de: string }>;
}>;

export type SpaceArtifactCreationCatalogV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;

export function record(value: unknown): Readonly<Record<string, unknown>> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Readonly<Record<string, unknown>>) : null;
}

export function exactFields(value: Readonly<Record<string, unknown>>, fields: readonly string[]): boolean {
  return Object.keys(value).sort().join(",") === [...fields].sort().join(",");
}

export function identity(value: unknown): string | null {
  return typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value) ? value : null;
}

/** 🃏️ A dialect subset is an identity **or** the single wildcard `*` that a dialect coordinate
 * writes for a standard's whole subset space (`"s.gis.gismap@1/*"`). Every shipped trusted-catalog
 * open target declares `"*"`, so an identity-only predicate refuses the real catalog. */
function subsetIdentity(value: unknown): string | null {
  return value === "*" ? "*" : identity(value);
}

export function requestId(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{32}$)[0-9a-f]{32}$/u.test(value) ? value : null;
}

export function digest(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value) ? value : null;
}

export function name(value: unknown): string | null {
  return typeof value === "string" && value.length > 0 && [...value].length <= 128 && !value.startsWith(" ") && !value.endsWith(" ") && !/[\u0000-\u001f\u007f-\u009f]/u.test(value) && !/[\ud800-\udfff]/u.test(value) ? value : null;
}

export function ready(value: unknown): SpaceArtifactCreationReadyV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["artifactId", "kindId", "artifactSchema", "parentDialect"])) return null;
  const dialect = record(row.parentDialect);
  if (dialect === null || !exactFields(dialect, ["artifactKind", "standard", "subset"])) return null;
  const artifactId = typeof row.artifactId === "string" && /^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(row.artifactId) ? row.artifactId : null,
    kindId = identity(row.kindId),
    artifactSchema = identity(row.artifactSchema),
    artifactKind = identity(dialect.artifactKind),
    standard = identity(dialect.standard),
    subset = subsetIdentity(dialect.subset);
  return artifactId !== null && kindId !== null && artifactSchema !== null && artifactKind !== null && standard !== null && subset !== null
    ? { artifactId, kindId, artifactSchema, parentDialect: { artifactKind, standard, subset } }
    : null;
}

export function creationKind(value: unknown): SpaceArtifactCreationKindV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["kindId", "schema", "dialect", "label"])) return null;
  const dialect = record(row.dialect),
    labels = record(row.label);
  if (dialect === null || !exactFields(dialect, ["artifactKind", "standard", "subset"]) || labels === null || !exactFields(labels, ["en", "de"])) return null;
  const kindId = identity(row.kindId),
    schema = identity(row.schema),
    artifactKind = identity(dialect.artifactKind),
    standard = identity(dialect.standard),
    subset = subsetIdentity(dialect.subset),
    en = name(labels.en),
    de = name(labels.de);
  return kindId !== null && schema !== null && artifactKind !== null && standard !== null && subset !== null && en !== null && de !== null
    ? { kindId, schema, dialect: { artifactKind, standard, subset }, label: { en, de } }
    : null;
}

/** 📥️ Seals the only client-supplied creation fields. */
export function sealSpaceArtifactCreateV1(value: Readonly<{ requestId: string; expectedCatalogGenerationId: string; kindId: string; name: string }>): SpaceArtifactCreateV1 {
  const parsedRequestId = requestId(value.requestId),
    expectedCatalogGenerationId = digest(value.expectedCatalogGenerationId),
    parsedKindId = identity(value.kindId),
    parsedName = name(value.name);
  if (parsedRequestId === null || expectedCatalogGenerationId === null || parsedKindId === null || parsedName === null) throw new Error("space artifact creation: invalid intent");
  return { schema: SPACE_ARTIFACT_CREATE_SCHEMA_V1, requestId: parsedRequestId, expectedCatalogGenerationId, kindId: parsedKindId, name: parsedName };
}

/** 🧾️ Parses a canonical client request without admitting extra authority fields. */


/** 📈️ One exact progress record, or `null`. */
export function creationProgress(value: unknown): SpaceArtifactCreationProgressV1 | null {
  const row = record(value);
  const stages: readonly SpaceArtifactCreationStageV1[] = ["queued", "compiling-guest", "genesis", "publishing"];
  if (row === null || !exactFields(row, ["stage", "completedUnits", "totalUnits"]) || !stages.includes(row.stage as SpaceArtifactCreationStageV1)) return null;
  const completedUnits = row.completedUnits,
    totalUnits = row.totalUnits;
  if (!Number.isSafeInteger(completedUnits) || !Number.isSafeInteger(totalUnits) || (totalUnits as number) < 1 || (completedUnits as number) < 0 || (completedUnits as number) > (totalUnits as number)) return null;
  return { stage: row.stage as SpaceArtifactCreationStageV1, completedUnits: completedUnits as number, totalUnits: totalUnits as number };
}

/** 📤️ Decodes one canonical, bounded server status and withholds incomplete ready tuples; only a running
 * creation carries progress. */


/** 🗂️ Decodes only the exact selected-current, bounded and sorted creation presentation. */

