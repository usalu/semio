/** 🗿️ En1990Artifact — hierarchical basis-of-design subject (SI: N, m, Hz). */

export type AnnexChoice = "En" | "De";

export interface PermanentAction {
  id: string;
  kind: string;
  gk: number;
}

export interface VariableAction {
  id: string;
  category: string;
  qk: number;
}

export interface AccidentalAction {
  id: string;
  ad: number;
}

export type ImportanceClass = "I" | "II" | "III" | "IV";

export interface SeismicAction {
  id: string;
  aEk: number;
  importanceClass: ImportanceClass;
}

export interface Member {
  id: string;
  labelEn: string;
  labelDe: string;
  rdStr: number;
  rdGeo: number;
  rdEquStab: number;
  rdEquDestab: number;
  rdFat: number;
  span: number;
  deflectionW: number;
  deflectionLimitRatio: number;
  vibrationFrequency: number;
  vibrationFrequencyMin: number;
}

export interface BridgeSls {
  id: string;
  memberId: string;
  deckAcceleration: number;
  deckAccelerationLimit: number;
  deckTwist: number;
  deckTwistLimit: number;
  bridgeDeflection: number;
  bridgeDeflectionLimit: number;
}

export interface MemberEffect {
  memberId: string;
  actionId: string;
  influence: number;
}

export interface En1990Artifact {
  annex: AnnexChoice;
  projectId: string;
  structureKind: string;
  altitudeM: number;
  consequenceClass: number;
  reliabilityClass: number;
  designWorkingLifeCategory: number;
  designWorkingLifeYears: number;
  referencePeriodYears: number;
  supervisionLevel: string;
  inspectionLevel: string;
  kFiDeclared: number;
  betaComputed: number;
  permanents: PermanentAction[];
  variables: VariableAction[];
  accidentals: AccidentalAction[];
  seismics: SeismicAction[];
  members: Member[];
  bridgeSls: BridgeSls[];
  effects: MemberEffect[];
}

export type En1990Snapshot = En1990Artifact;

/** 🧱 Typed structural parse error for En1990 artifact JSON. */
export class En1990ParseError extends Error {
  readonly path: string;
  readonly code: "not_object" | "missing" | "mistyped" | "invalid_enum";

  constructor(code: En1990ParseError["code"], path: string, detail: string) {
    super(`${code} at ${path}: ${detail}`);
    this.name = "En1990ParseError";
    this.path = path;
    this.code = code;
  }
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function expectObject(value: unknown, at: string): Record<string, unknown> {
  if (!isObject(value)) {
    throw new En1990ParseError("not_object", at, `expected object, got ${typeof value}`);
  }
  return value;
}

function expectString(value: unknown, at: string): string {
  if (typeof value !== "string") {
    throw new En1990ParseError("mistyped", at, `expected string, got ${typeof value}`);
  }
  return value;
}

function expectNumber(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new En1990ParseError("mistyped", at, `expected finite number, got ${typeof value}`);
  }
  return value;
}

function requireKey(obj: Record<string, unknown>, key: string, at: string): unknown {
  if (!(key in obj)) {
    throw new En1990ParseError("missing", `${at}.${key}`, `required field '${key}' is missing`);
  }
  return obj[key];
}

function expectArray(value: unknown, at: string): unknown[] {
  if (!Array.isArray(value)) {
    throw new En1990ParseError("mistyped", at, `expected array, got ${typeof value}`);
  }
  return value;
}

function parsePermanent(value: unknown, at: string): PermanentAction {
  const o = expectObject(value, at);
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    kind: expectString(requireKey(o, "kind", at), `${at}.kind`),
    gk: expectNumber(requireKey(o, "gk", at), `${at}.gk`),
  };
}

function parseVariable(value: unknown, at: string): VariableAction {
  const o = expectObject(value, at);
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    category: expectString(requireKey(o, "category", at), `${at}.category`),
    qk: expectNumber(requireKey(o, "qk", at), `${at}.qk`),
  };
}

function parseAccidental(value: unknown, at: string): AccidentalAction {
  const o = expectObject(value, at);
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    ad: expectNumber(requireKey(o, "ad", at), `${at}.ad`),
  };
}

function parseSeismic(value: unknown, at: string): SeismicAction {
  const o = expectObject(value, at);
  const importanceClass = expectString(requireKey(o, "importanceClass", at), `${at}.importanceClass`);
  if (!["I", "II", "III", "IV"].includes(importanceClass)) {
    throw new En1990ParseError("invalid_enum", `${at}.importanceClass`, `expected I|II|III|IV, got '${importanceClass}'`);
  }
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    aEk: expectNumber(requireKey(o, "aEk", at), `${at}.aEk`),
    importanceClass: importanceClass as ImportanceClass,
  };
}

function parseMember(value: unknown, at: string): Member {
  const o = expectObject(value, at);
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    labelEn: expectString(requireKey(o, "labelEn", at), `${at}.labelEn`),
    labelDe: expectString(requireKey(o, "labelDe", at), `${at}.labelDe`),
    rdStr: expectNumber(requireKey(o, "rdStr", at), `${at}.rdStr`),
    rdGeo: expectNumber(requireKey(o, "rdGeo", at), `${at}.rdGeo`),
    rdEquStab: expectNumber(requireKey(o, "rdEquStab", at), `${at}.rdEquStab`),
    rdEquDestab: expectNumber(requireKey(o, "rdEquDestab", at), `${at}.rdEquDestab`),
    rdFat: expectNumber(requireKey(o, "rdFat", at), `${at}.rdFat`),
    span: expectNumber(requireKey(o, "span", at), `${at}.span`),
    deflectionW: expectNumber(requireKey(o, "deflectionW", at), `${at}.deflectionW`),
    deflectionLimitRatio: expectNumber(requireKey(o, "deflectionLimitRatio", at), `${at}.deflectionLimitRatio`),
    vibrationFrequency: expectNumber(requireKey(o, "vibrationFrequency", at), `${at}.vibrationFrequency`),
    vibrationFrequencyMin: expectNumber(requireKey(o, "vibrationFrequencyMin", at), `${at}.vibrationFrequencyMin`),
  };
}

function parseBridgeSls(value: unknown, at: string): BridgeSls {
  const o = expectObject(value, at);
  return {
    id: expectString(requireKey(o, "id", at), `${at}.id`),
    memberId: expectString(requireKey(o, "memberId", at), `${at}.memberId`),
    deckAcceleration: expectNumber(requireKey(o, "deckAcceleration", at), `${at}.deckAcceleration`),
    deckAccelerationLimit: expectNumber(requireKey(o, "deckAccelerationLimit", at), `${at}.deckAccelerationLimit`),
    deckTwist: expectNumber(requireKey(o, "deckTwist", at), `${at}.deckTwist`),
    deckTwistLimit: expectNumber(requireKey(o, "deckTwistLimit", at), `${at}.deckTwistLimit`),
    bridgeDeflection: expectNumber(requireKey(o, "bridgeDeflection", at), `${at}.bridgeDeflection`),
    bridgeDeflectionLimit: expectNumber(requireKey(o, "bridgeDeflectionLimit", at), `${at}.bridgeDeflectionLimit`),
  };
}

function parseEffect(value: unknown, at: string): MemberEffect {
  const o = expectObject(value, at);
  return {
    memberId: expectString(requireKey(o, "memberId", at), `${at}.memberId`),
    actionId: expectString(requireKey(o, "actionId", at), `${at}.actionId`),
    influence: expectNumber(requireKey(o, "influence", at), `${at}.influence`),
  };
}

/** 🧱 Structurally validate and decode an En1990 artifact (rejects missing/mistyped required fields). */
export function parseEn1990Artifact(value: unknown, at = "$"): En1990Artifact {
  const o = expectObject(value, at);
  const annex = expectString(requireKey(o, "annex", at), `${at}.annex`);
  if (annex !== "En" && annex !== "De") {
    throw new En1990ParseError("invalid_enum", `${at}.annex`, `expected En|De, got '${annex}'`);
  }
  return {
    annex,
    projectId: expectString(requireKey(o, "projectId", at), `${at}.projectId`),
    structureKind: expectString(requireKey(o, "structureKind", at), `${at}.structureKind`),
    altitudeM: expectNumber(requireKey(o, "altitudeM", at), `${at}.altitudeM`),
    consequenceClass: expectNumber(requireKey(o, "consequenceClass", at), `${at}.consequenceClass`),
    reliabilityClass: expectNumber(requireKey(o, "reliabilityClass", at), `${at}.reliabilityClass`),
    designWorkingLifeCategory: expectNumber(requireKey(o, "designWorkingLifeCategory", at), `${at}.designWorkingLifeCategory`),
    designWorkingLifeYears: expectNumber(requireKey(o, "designWorkingLifeYears", at), `${at}.designWorkingLifeYears`),
    referencePeriodYears: expectNumber(requireKey(o, "referencePeriodYears", at), `${at}.referencePeriodYears`),
    supervisionLevel: expectString(requireKey(o, "supervisionLevel", at), `${at}.supervisionLevel`),
    inspectionLevel: expectString(requireKey(o, "inspectionLevel", at), `${at}.inspectionLevel`),
    kFiDeclared: expectNumber(requireKey(o, "kFiDeclared", at), `${at}.kFiDeclared`),
    betaComputed: expectNumber(requireKey(o, "betaComputed", at), `${at}.betaComputed`),
    permanents: expectArray(requireKey(o, "permanents", at), `${at}.permanents`).map((item, i) => parsePermanent(item, `${at}.permanents[${i}]`)),
    variables: expectArray(requireKey(o, "variables", at), `${at}.variables`).map((item, i) => parseVariable(item, `${at}.variables[${i}]`)),
    accidentals: expectArray(requireKey(o, "accidentals", at), `${at}.accidentals`).map((item, i) => parseAccidental(item, `${at}.accidentals[${i}]`)),
    seismics: expectArray(requireKey(o, "seismics", at), `${at}.seismics`).map((item, i) => parseSeismic(item, `${at}.seismics[${i}]`)),
    members: expectArray(requireKey(o, "members", at), `${at}.members`).map((item, i) => parseMember(item, `${at}.members[${i}]`)),
    bridgeSls: expectArray(requireKey(o, "bridgeSls", at), `${at}.bridgeSls`).map((item, i) => parseBridgeSls(item, `${at}.bridgeSls[${i}]`)),
    effects: expectArray(requireKey(o, "effects", at), `${at}.effects`).map((item, i) => parseEffect(item, `${at}.effects[${i}]`)),
  };
}

export function parseEn1990Fields(value: unknown, _partial: boolean, at = "$"): Partial<En1990Artifact> {
  return parseEn1990Artifact(value, at);
}
