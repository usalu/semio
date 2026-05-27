// #region Header
/** @emoji 🧭 `@spatial/js-core` implements the rewritten spatial core around hashed primitives, persistent models, declarative model definitions, a headless action interpreter, and a static interaction runtime. */
// #endregion Header

type JsonModule<T> = { readonly default: T } | T;

function moduleValue<T>(input: JsonModule<T>): T {
  return (typeof input === "object" && input !== null && "default" in input ? input.default : input) as T;
}

function readCatalog<T>(modules: Record<string, JsonModule<T>>): readonly T[] {
  return Object.values(modules).map((entry) => moduleValue(entry));
}

export type Vec3 = readonly [number, number, number];

export const primitiveKinds = {
  geometry: {
    point: "geometry.point",
    curve: {
      line: "geometry.curve.line",
      circle: "geometry.curve.circle",
      ellipse: "geometry.curve.ellipse",
      parabola: "geometry.curve.parabola",
      hyperbola: "geometry.curve.hyperbola",
      bspline: "geometry.curve.bspline",
      bezier: "geometry.curve.bezier",
    },
    surface: {
      plane: "geometry.surface.plane",
      cylinder: "geometry.surface.cylinder",
      cone: "geometry.surface.cone",
      sphere: "geometry.surface.sphere",
      torus: "geometry.surface.torus",
      bspline: "geometry.surface.bspline",
      bezier: "geometry.surface.bezier",
    },
  },
  topology: {
    vertex: "topology.vertex",
    edge: "topology.edge",
    wire: "topology.wire",
    face: "topology.face",
    shell: "topology.shell",
    solid: "topology.solid",
    compSolid: "topology.compSolid",
    compound: "topology.compound",
  },
} as const;

export type PrimitiveKind =
  | typeof primitiveKinds.geometry.point
  | (typeof primitiveKinds.geometry.curve)[keyof typeof primitiveKinds.geometry.curve]
  | (typeof primitiveKinds.geometry.surface)[keyof typeof primitiveKinds.geometry.surface]
  | (typeof primitiveKinds.topology)[keyof typeof primitiveKinds.topology];

export interface SpatialPrimitive<TData = unknown> {
  readonly hash: string;
  readonly kind: PrimitiveKind;
  readonly data: TData;
}

export interface SpatialAttribute {
  readonly id: string;
  readonly definitionId: string;
  readonly primitiveHash: string;
  readonly value: unknown;
}

export interface SpatialProperty {
  readonly definitionId: string;
  readonly value: unknown;
}

export interface SpatialObject {
  readonly id: string;
  readonly typologyId: string;
  readonly primitiveHashes: readonly string[];
  readonly attributes: readonly SpatialAttribute[];
  readonly properties: Readonly<Record<string, SpatialProperty>>;
}

export interface SpatialModelSnapshot {
  readonly id: string;
  readonly definitionId?: string;
  readonly revision: number;
  readonly objects: readonly SpatialObject[];
}

export interface ModelLink {
  readonly fromModelId: string;
  readonly toModelId: string;
  readonly relation: string;
}

export function stableJsonStringify(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map((entry) => stableJsonStringify(entry)).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.entries(value as Record<string, unknown>)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([key, entry]) => `${JSON.stringify(key)}:${stableJsonStringify(entry)}`)
      .join(",")}}`;
  }
  return JSON.stringify(value);
}

export function hashValue(value: unknown): string {
  const input = stableJsonStringify(value);
  let hash = 0x811c9dc5;
  for (let index = 0; index < input.length; index += 1) {
    hash ^= input.charCodeAt(index);
    hash = Math.imul(hash, 0x01000193);
  }
  return `h${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

export function hashPrimitive<TData>(kind: PrimitiveKind, data: TData): string {
  return hashValue({ kind, data });
}

export function createPrimitive<TData>(kind: PrimitiveKind, data: TData): SpatialPrimitive<TData> {
  return { hash: hashPrimitive(kind, data), kind, data };
}

export class Model {
  readonly id: string;
  readonly definitionId?: string;
  private revisionValue = 0;
  private readonly objectsById = new Map<string, SpatialObject>();

  constructor(input: { readonly id: string; readonly definitionId?: string; readonly objects?: readonly SpatialObject[] }) {
    this.id = input.id;
    this.definitionId = input.definitionId;
    for (const object of input.objects ?? []) this.objectsById.set(object.id, object);
  }

  get revision(): number {
    return this.revisionValue;
  }

  listObjects(): readonly SpatialObject[] {
    return [...this.objectsById.values()];
  }

  getObject(id: string): SpatialObject | null {
    return this.objectsById.get(id) ?? null;
  }

  upsertObject(object: SpatialObject): SpatialObject {
    this.objectsById.set(object.id, object);
    this.revisionValue += 1;
    return object;
  }

  toSnapshot(): SpatialModelSnapshot {
    return { id: this.id, definitionId: this.definitionId, revision: this.revisionValue, objects: this.listObjects() };
  }
}

export class ModelSpace {
  private readonly primitivesByHash = new Map<string, SpatialPrimitive>();
  private readonly modelsById = new Map<string, Model>();
  private readonly links: ModelLink[] = [];

  addPrimitive<TData>(kind: PrimitiveKind, data: TData): SpatialPrimitive<TData> {
    const primitive = createPrimitive(kind, data);
    if (!this.primitivesByHash.has(primitive.hash)) this.primitivesByHash.set(primitive.hash, primitive);
    return this.primitivesByHash.get(primitive.hash) as SpatialPrimitive<TData>;
  }

  getPrimitive(hash: string): SpatialPrimitive | null {
    return this.primitivesByHash.get(hash) ?? null;
  }

  listPrimitives(): readonly SpatialPrimitive[] {
    return [...this.primitivesByHash.values()];
  }

  addModel(model: Model): Model {
    this.modelsById.set(model.id, model);
    return model;
  }

  createModel(input: { readonly id: string; readonly definitionId?: string }): Model {
    const model = new Model(input);
    return this.addModel(model);
  }

  getModel(id: string): Model | null {
    return this.modelsById.get(id) ?? null;
  }

  listModels(): readonly Model[] {
    return [...this.modelsById.values()];
  }

  linkModels(link: ModelLink): ModelLink {
    this.links.push(link);
    return link;
  }

  listLinks(): readonly ModelLink[] {
    return [...this.links];
  }
}

export function createSpatialObject(input: {
  readonly id: string;
  readonly typologyId: string;
  readonly primitiveHashes: readonly string[];
  readonly attributes?: readonly SpatialAttribute[];
  readonly properties?: Readonly<Record<string, SpatialProperty>>;
}): SpatialObject {
  return {
    id: input.id,
    typologyId: input.typologyId,
    primitiveHashes: [...input.primitiveHashes],
    attributes: [...(input.attributes ?? [])],
    properties: { ...(input.properties ?? {}) },
  };
}

export interface ExtensionDefinition {
  readonly schema: "spatial.extension/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly ("action" | "attribute" | "interaction" | "property" | "typology" | "view")[];
}

export interface ModelDefinitionManifest {
  readonly schema?: string;
  readonly id?: string;
  readonly label?: string;
  readonly version?: string;
  readonly description?: string;
}

export interface TypologyDefinitionDocument {
  readonly schema: "spatial.typology/v1";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly actions: readonly string[];
  readonly interactions: readonly string[];
  readonly attributes?: readonly string[];
  readonly properties?: readonly string[];
  readonly primitiveKinds?: readonly PrimitiveKind[];
}

export interface AttributeDefinition {
  readonly schema?: string;
  readonly id?: string;
  readonly [key: string]: unknown;
}

export interface PropertyDefinition {
  readonly schema?: string;
  readonly id?: string;
  readonly [key: string]: unknown;
}

export interface TransformationDefinition {
  readonly schema?: string;
  readonly id?: string;
  readonly [key: string]: unknown;
}

export interface ViewDefinition {
  readonly schema?: string;
  readonly id?: string;
  readonly [key: string]: unknown;
}

export interface PathFieldSegment {
  readonly kind: "field";
  readonly name: string;
}

export interface PathIndexSegment {
  readonly kind: "index";
  readonly index: number;
}

export type PathSegment = PathFieldSegment | PathIndexSegment;
export type PathRoot = "context" | "event" | "params";

export interface PathTarget {
  readonly root: PathRoot;
  readonly segments: readonly PathSegment[];
}

export type SpatialExpression =
  | { readonly kind: "const"; readonly value: unknown }
  | { readonly kind: "path"; readonly root: PathRoot; readonly segments: readonly PathSegment[] }
  | { readonly kind: "var"; readonly name: string }
  | { readonly kind: "exists"; readonly target: PathTarget }
  | { readonly kind: "all"; readonly args: readonly SpatialExpression[] }
  | { readonly kind: "binop"; readonly op: "==" | "!=" | ">" | "<" | ">=" | "<=" | "+" | "-" | "*" | "/"; readonly left: SpatialExpression; readonly right: SpatialExpression };

export type ActionStep =
  | { readonly op: "setContext"; readonly values: Readonly<Record<string, SpatialExpression>> }
  | { readonly op: "guard"; readonly condition: SpatialExpression; readonly message?: string }
  | { readonly op: "action"; readonly action: string; readonly params?: Readonly<Record<string, SpatialExpression>>; readonly assignTo?: string }
  | { readonly op: "return"; readonly diff?: SpatialExpression; readonly data?: SpatialExpression; readonly patch?: SpatialExpression; readonly result?: SpatialExpression };

export interface ActionDefinition {
  readonly schema: "spatial.action/v1";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly variables?: readonly { readonly name: string; readonly value: SpatialExpression }[];
  readonly steps: readonly ActionStep[];
}

export interface DisplayDefinition {
  readonly kind: "point" | "label" | "segment";
  readonly id: string;
  readonly role?: string;
  readonly position?: SpatialExpression;
  readonly from?: SpatialExpression;
  readonly to?: SpatialExpression;
  readonly text?: string;
}

export type InteractionEffect =
  | { readonly op: "assign"; readonly target: PathTarget; readonly value: SpatialExpression }
  | { readonly op: "append"; readonly target: PathTarget; readonly value: SpatialExpression }
  | { readonly op: "action"; readonly action: string; readonly params?: Readonly<Record<string, SpatialExpression>>; readonly assignTo?: PathTarget };

export interface InteractionTransition {
  readonly target?: string;
  readonly guard?: string;
  readonly effects?: readonly InteractionEffect[];
}

export interface InteractionState {
  readonly name: string;
  readonly final?: boolean;
  readonly on?: readonly { readonly event: string; readonly transitions: readonly InteractionTransition[] }[];
}

export interface InteractionDefinition {
  readonly schema: "spatial.interaction/v1";
  readonly id: string;
  readonly version: string;
  readonly label?: string;
  readonly guards?: readonly { readonly name: string; readonly expr: SpatialExpression }[];
  readonly machine: { readonly initial: string; readonly states: readonly InteractionState[] };
  readonly display?: { readonly states?: readonly { readonly state: string; readonly items: readonly DisplayDefinition[] }[] };
  readonly commit: { readonly when?: string; readonly fromStates?: readonly string[]; readonly outputDataPath?: PathTarget; readonly operation: { readonly kind: "action"; readonly action: string; readonly params?: Readonly<Record<string, SpatialExpression>> } };
}

export interface DisplayItem {
  readonly kind: string;
  readonly id: string;
  readonly role?: string;
  readonly params: Readonly<Record<string, unknown>>;
}

export interface ModelDefinitionCatalog {
  readonly extensions: readonly ExtensionDefinition[];
  readonly manifests: readonly ModelDefinitionManifest[];
  readonly typologies: readonly TypologyDefinitionDocument[];
  readonly actions: readonly ActionDefinition[];
  readonly interactions: readonly InteractionDefinition[];
  readonly attributeDefinitions: readonly AttributeDefinition[];
  readonly propertyDefinitions: readonly PropertyDefinition[];
  readonly transformations: readonly TransformationDefinition[];
  readonly views: readonly ViewDefinition[];
}

const extensionModules = import.meta.glob("../../assets/modelDefinition/**/extension.json", { eager: true, import: "default" }) as Record<string, JsonModule<ExtensionDefinition>>;
const manifestModules = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", { eager: true, import: "default" }) as Record<string, JsonModule<ModelDefinitionManifest>>;
const typologyModules = import.meta.glob("../../assets/modelDefinition/**/typology.json", { eager: true, import: "default" }) as Record<string, JsonModule<TypologyDefinitionDocument>>;
const actionModules = import.meta.glob("../../assets/modelDefinition/**/action/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<ActionDefinition>>;
const interactionModules = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<InteractionDefinition>>;
const attributeDefinitionModules = import.meta.glob("../../assets/modelDefinition/**/attributeDefinition/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<AttributeDefinition>>;
const propertyDefinitionModules = import.meta.glob("../../assets/modelDefinition/**/propertyDefinition/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<PropertyDefinition>>;
const propertyModules = import.meta.glob("../../assets/modelDefinition/**/property/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<PropertyDefinition>>;
const transformationModules = import.meta.glob("../../assets/modelDefinition/**/transformation/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<TransformationDefinition>>;
const viewModules = import.meta.glob("../../assets/modelDefinition/**/view/**/*.json", { eager: true, import: "default" }) as Record<string, JsonModule<ViewDefinition>>;

let builtinCatalogCache: ModelDefinitionCatalog | null = null;

export function loadBuiltinCatalog(): ModelDefinitionCatalog {
  if (builtinCatalogCache) return builtinCatalogCache;
  builtinCatalogCache = {
    extensions: readCatalog(extensionModules),
    manifests: readCatalog(manifestModules),
    typologies: readCatalog(typologyModules),
    actions: readCatalog(actionModules),
    interactions: readCatalog(interactionModules),
    attributeDefinitions: readCatalog(attributeDefinitionModules),
    propertyDefinitions: [...readCatalog(propertyDefinitionModules), ...readCatalog(propertyModules)],
    transformations: readCatalog(transformationModules),
    views: readCatalog(viewModules),
  };
  return builtinCatalogCache;
}

export function listBuiltinTypologies(): readonly TypologyDefinitionDocument[] {
  return loadBuiltinCatalog().typologies;
}

export function listBuiltinActions(): readonly ActionDefinition[] {
  return loadBuiltinCatalog().actions;
}

export function listBuiltinInteractions(): readonly InteractionDefinition[] {
  return loadBuiltinCatalog().interactions;
}

export function loadActionDefinition(id: string): ActionDefinition | null {
  return loadBuiltinCatalog().actions.find((entry) => entry.id === id) ?? null;
}

export function loadInteractionDefinition(id: string): InteractionDefinition | null {
  return loadBuiltinCatalog().interactions.find((entry) => entry.id === id) ?? null;
}

export function loadTypologyDefinition(id: string): TypologyDefinitionDocument | null {
  return loadBuiltinCatalog().typologies.find((entry) => entry.id === id) ?? null;
}

export function validateObjectAgainstTypology(object: SpatialObject, typology: TypologyDefinitionDocument, space: ModelSpace): readonly string[] {
  if (!typology.primitiveKinds || typology.primitiveKinds.length === 0) return [];
  const allowed = new Set(typology.primitiveKinds);
  const errors: string[] = [];
  for (const hash of object.primitiveHashes) {
    const primitive = space.getPrimitive(hash);
    if (!primitive) errors.push(`Missing primitive ${hash}.`);
    else if (!allowed.has(primitive.kind)) errors.push(`Primitive ${primitive.kind} is not allowed for ${typology.id}.`);
  }
  return errors;
}

function readPathSegments(root: unknown, segments: readonly PathSegment[]): unknown {
  let cursor = root;
  for (const segment of segments) {
    if (cursor === null || cursor === undefined) return undefined;
    cursor = segment.kind === "field" ? (cursor as Record<string, unknown>)[segment.name] : (cursor as readonly unknown[])[segment.index];
  }
  return cursor;
}

function writePathSegments(root: Record<string, unknown>, segments: readonly PathSegment[], value: unknown): void {
  if (segments.length === 0) return;
  let cursor: Record<string, unknown> | unknown[] = root;
  for (let index = 0; index < segments.length - 1; index += 1) {
    const current = segments[index]!;
    const next = segments[index + 1]!;
    if (current.kind === "field") {
      const holder = cursor as Record<string, unknown>;
      if (holder[current.name] === undefined) holder[current.name] = next.kind === "index" ? [] : {};
      cursor = holder[current.name] as Record<string, unknown> | unknown[];
    } else {
      const holder = cursor as unknown[];
      if (holder[current.index] === undefined) holder[current.index] = next.kind === "index" ? [] : {};
      cursor = holder[current.index] as Record<string, unknown> | unknown[];
    }
  }
  const last = segments[segments.length - 1]!;
  if (last.kind === "field") (cursor as Record<string, unknown>)[last.name] = value;
  else (cursor as unknown[])[last.index] = value;
}

export interface SpatialKernel {
  readonly id?: string;
  call?(name: string, args: Readonly<Record<string, unknown>>, runtime: RuntimeContext): unknown | Promise<unknown>;
}

export interface RuntimeContext {
  readonly modelSpace: ModelSpace;
  readonly model: Model;
  readonly kernel: SpatialKernel;
}

export interface ExpressionEnvironment {
  readonly context: Record<string, unknown>;
  readonly event: Record<string, unknown>;
  readonly params: Record<string, unknown>;
  readonly variables: Record<string, unknown>;
  readonly runtime: RuntimeContext;
}

export async function evalExpression(expression: SpatialExpression, environment: ExpressionEnvironment): Promise<unknown> {
  switch (expression.kind) {
    case "const":
      return expression.value;
    case "path": {
      const root = expression.root === "context" ? environment.context : expression.root === "event" ? environment.event : environment.params;
      return readPathSegments(root, expression.segments);
    }
    case "var":
      return environment.variables[expression.name];
    case "exists": {
      const root = expression.target.root === "context" ? environment.context : expression.target.root === "event" ? environment.event : environment.params;
      return readPathSegments(root, expression.target.segments) !== undefined;
    }
    case "all":
      for (const argument of expression.args) if (!(await evalExpression(argument, environment))) return false;
      return true;
    case "binop": {
      const left = await evalExpression(expression.left, environment);
      const right = await evalExpression(expression.right, environment);
      switch (expression.op) {
        case "==":
          return left === right;
        case "!=":
          return left !== right;
        case ">":
          return Number(left) > Number(right);
        case "<":
          return Number(left) < Number(right);
        case ">=":
          return Number(left) >= Number(right);
        case "<=":
          return Number(left) <= Number(right);
        case "+":
          return Number(left) + Number(right);
        case "-":
          return Number(left) - Number(right);
        case "*":
          return Number(left) * Number(right);
        case "/":
          return Number(left) / Number(right);
      }
    }
  }
}

async function evalExpressionMap(input: Readonly<Record<string, SpatialExpression>>, environment: ExpressionEnvironment): Promise<Record<string, unknown>> {
  const output: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(input)) output[key] = await evalExpression(value, environment);
  return output;
}

export interface ActionExecutionResult {
  readonly ok: boolean;
  readonly context: Record<string, unknown>;
  readonly data: unknown;
  readonly diff: unknown;
  readonly patch: unknown;
  readonly result: unknown;
  readonly errors: readonly string[];
}

export interface ActionExecutionInput {
  readonly action: string | ActionDefinition;
  readonly params?: Readonly<Record<string, unknown>>;
  readonly context?: Record<string, unknown>;
  readonly runtime: RuntimeContext;
}

export class ActionRegistry {
  private readonly actions = new Map<string, ActionDefinition>();

  constructor(input?: readonly ActionDefinition[]) {
    for (const action of input ?? []) this.register(action);
  }

  static withBuiltins(): ActionRegistry {
    return new ActionRegistry(listBuiltinActions());
  }

  register(action: ActionDefinition): ActionDefinition {
    this.actions.set(action.id, action);
    return action;
  }

  get(id: string): ActionDefinition | null {
    return this.actions.get(id) ?? null;
  }

  async run(input: ActionExecutionInput): Promise<ActionExecutionResult> {
    return runActionDefinition(input, this);
  }
}

export async function runActionDefinition(input: ActionExecutionInput, registry: ActionRegistry = ActionRegistry.withBuiltins()): Promise<ActionExecutionResult> {
  const action = typeof input.action === "string" ? registry.get(input.action) : input.action;
  if (!action) return { ok: false, context: { ...(input.context ?? {}) }, data: null, diff: null, patch: null, result: null, errors: ["Unknown action."] };
  const context = { ...(input.context ?? {}) };
  const variables: Record<string, unknown> = {};
  const environment: ExpressionEnvironment = { context, event: {}, params: { ...(input.params ?? {}) }, variables, runtime: input.runtime };
  for (const variable of action.variables ?? []) variables[variable.name] = await evalExpression(variable.value, environment);
  let data: unknown = null;
  let diff: unknown = null;
  let patch: unknown = null;
  let result: unknown = null;
  try {
    for (const step of action.steps) {
      if (step.op === "setContext") {
        Object.assign(context, await evalExpressionMap(step.values, environment));
        continue;
      }
      if (step.op === "guard") {
        if (!(await evalExpression(step.condition, environment))) throw new Error(step.message ?? `Guard failed in ${action.id}.`);
        continue;
      }
      if (step.op === "action") {
        const nested = await registry.run({ action: step.action, params: await evalExpressionMap(step.params ?? {}, environment), context, runtime: input.runtime });
        if (!nested.ok) throw new Error(nested.errors.join("\n"));
        if (step.assignTo) variables[step.assignTo] = nested.result ?? nested.data;
        data = nested.data;
        diff = nested.diff;
        patch = nested.patch;
        result = nested.result;
        continue;
      }
      diff = step.diff ? await evalExpression(step.diff, environment) : diff;
      data = step.data ? await evalExpression(step.data, environment) : data;
      patch = step.patch ? await evalExpression(step.patch, environment) : patch;
      result = step.result ? await evalExpression(step.result, environment) : result;
      return { ok: true, context, data, diff, patch, result, errors: [] };
    }
    return { ok: true, context, data, diff, patch, result, errors: [] };
  } catch (error) {
    return { ok: false, context, data, diff, patch, result, errors: [error instanceof Error ? error.message : String(error)] };
  }
}

export interface InteractionEvent {
  readonly kind: string;
  readonly [key: string]: unknown;
}

export interface InteractionSnapshot {
  readonly interactionId: string;
  readonly state: string;
  readonly context: Record<string, unknown>;
  readonly display: readonly DisplayItem[];
  readonly lastResult: ActionExecutionResult | null;
}

export interface InteractionRuntimeOptions {
  readonly interaction: string | InteractionDefinition;
  readonly runtime: RuntimeContext;
  readonly context?: Record<string, unknown>;
  readonly actionRegistry?: ActionRegistry;
}

export interface InteractionRuntime {
  getSnapshot(): InteractionSnapshot;
  send(event: InteractionEvent): Promise<InteractionSnapshot>;
}

function stateByName(spec: InteractionDefinition, name: string): InteractionState | null {
  return spec.machine.states.find((state) => state.name === name) ?? null;
}

function guardByName(spec: InteractionDefinition, name: string): SpatialExpression | null {
  return spec.guards?.find((guard) => guard.name === name)?.expr ?? null;
}

async function evaluateDisplay(spec: InteractionDefinition, snapshot: InteractionSnapshot, options: InteractionRuntimeOptions): Promise<readonly DisplayItem[]> {
  const stateDisplay = spec.display?.states?.find((entry) => entry.state === snapshot.state);
  if (!stateDisplay) return [];
  const environment: ExpressionEnvironment = { context: snapshot.context, event: {}, params: {}, variables: {}, runtime: options.runtime };
  const items: DisplayItem[] = [];
  for (const item of stateDisplay.items) {
    if (item.kind === "point") items.push({ kind: item.kind, id: item.id, role: item.role, params: { position: await evalExpression(item.position!, environment) } });
    if (item.kind === "label") items.push({ kind: item.kind, id: item.id, role: item.role, params: { text: item.text, position: await evalExpression(item.position!, environment) } });
    if (item.kind === "segment") items.push({ kind: item.kind, id: item.id, role: item.role, params: { from: await evalExpression(item.from!, environment), to: await evalExpression(item.to!, environment) } });
  }
  return items;
}

export class InteractionRegistry {
  private readonly interactions = new Map<string, InteractionDefinition>();

  constructor(input?: readonly InteractionDefinition[]) {
    for (const interaction of input ?? []) this.register(interaction);
  }

  static withBuiltins(): InteractionRegistry {
    return new InteractionRegistry(listBuiltinInteractions());
  }

  register(interaction: InteractionDefinition): InteractionDefinition {
    this.interactions.set(interaction.id, interaction);
    return interaction;
  }

  get(id: string): InteractionDefinition | null {
    return this.interactions.get(id) ?? null;
  }
}

export function createInteractionRuntime(options: InteractionRuntimeOptions): InteractionRuntime {
  const interactionRegistry = InteractionRegistry.withBuiltins();
  const spec = typeof options.interaction === "string" ? interactionRegistry.get(options.interaction) : options.interaction;
  if (!spec) throw new Error(`Unknown interaction ${String(options.interaction)}.`);
  const actionRegistry = options.actionRegistry ?? ActionRegistry.withBuiltins();
  let snapshot: InteractionSnapshot = { interactionId: spec.id, state: spec.machine.initial, context: { ...(options.context ?? {}) }, display: [], lastResult: null };

  async function refreshDisplay(): Promise<void> {
    snapshot = { ...snapshot, display: await evaluateDisplay(spec, snapshot, options) };
  }

  async function maybeCommit(fromState: string, event: InteractionEvent): Promise<void> {
    const shouldCommitByEvent = spec.commit.when ? spec.commit.when === event.kind : false;
    const shouldCommitByState = spec.commit.fromStates ? spec.commit.fromStates.includes(fromState) : Boolean(stateByName(spec, snapshot.state)?.final);
    if (!shouldCommitByEvent && !shouldCommitByState) return;
    const environment: ExpressionEnvironment = { context: snapshot.context, event: event as Record<string, unknown>, params: {}, variables: {}, runtime: options.runtime };
    const params = await evalExpressionMap(spec.commit.operation.params ?? {}, environment);
    const result = await actionRegistry.run({ action: spec.commit.operation.action, params, context: snapshot.context, runtime: options.runtime });
    if (spec.commit.outputDataPath) writePathSegments(snapshot.context, spec.commit.outputDataPath.segments, result.data);
    snapshot = { ...snapshot, lastResult: result };
  }

  async function executeEffect(effect: InteractionEffect, event: InteractionEvent): Promise<void> {
    const environment: ExpressionEnvironment = { context: snapshot.context, event: event as Record<string, unknown>, params: {}, variables: {}, runtime: options.runtime };
    if (effect.op === "assign") {
      writePathSegments(snapshot.context, effect.target.segments, await evalExpression(effect.value, environment));
      return;
    }
    if (effect.op === "append") {
      const existing = readPathSegments(snapshot.context, effect.target.segments);
      const next = Array.isArray(existing) ? [...existing] : [];
      next.push(await evalExpression(effect.value, environment));
      writePathSegments(snapshot.context, effect.target.segments, next);
      return;
    }
    const result = await actionRegistry.run({ action: effect.action, params: await evalExpressionMap(effect.params ?? {}, environment), context: snapshot.context, runtime: options.runtime });
    if (effect.assignTo) writePathSegments(snapshot.context, effect.assignTo.segments, result.result ?? result.data);
    snapshot = { ...snapshot, lastResult: result };
  }

  const runtime: InteractionRuntime = {
    getSnapshot(): InteractionSnapshot {
      return snapshot;
    },
    async send(event: InteractionEvent): Promise<InteractionSnapshot> {
      const currentState = stateByName(spec, snapshot.state);
      if (!currentState) throw new Error(`Unknown state ${snapshot.state}.`);
      const handler = currentState.on?.find((entry) => entry.event === event.kind);
      if (!handler) {
        await refreshDisplay();
        return snapshot;
      }
      for (const transition of handler.transitions) {
        if (transition.guard) {
          const guard = guardByName(spec, transition.guard);
          if (!guard) continue;
          if (!(await evalExpression(guard, { context: snapshot.context, event: event as Record<string, unknown>, params: {}, variables: {}, runtime: options.runtime }))) continue;
        }
        for (const effect of transition.effects ?? []) await executeEffect(effect, event);
        snapshot = { ...snapshot, state: transition.target ?? snapshot.state };
        await maybeCommit(currentState.name, event);
        await refreshDisplay();
        return snapshot;
      }
      await refreshDisplay();
      return snapshot;
    },
  };

  void refreshDisplay();
  return runtime;
}

export function listModelObjectsByTypology(model: Model, typologyId: string): readonly SpatialObject[] {
  return model.listObjects().filter((object) => object.typologyId === typologyId);
}

export function listObjectPrimitives(space: ModelSpace, object: SpatialObject): readonly SpatialPrimitive[] {
  return object.primitiveHashes.map((hash) => space.getPrimitive(hash)).filter((entry): entry is SpatialPrimitive => entry !== null);
}

export function listObjectsByAttribute(model: Model, definitionId: string): readonly SpatialObject[] {
  return model.listObjects().filter((object) => object.attributes.some((attribute) => attribute.definitionId === definitionId));
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@spatial/js-core hashing", () => {
    it("hashes identical primitives stably", () => {
      const first = createPrimitive(primitiveKinds.geometry.point, { position: [1, 2, 3] as Vec3 });
      const second = createPrimitive(primitiveKinds.geometry.point, { position: [1, 2, 3] as Vec3 });
      expect(first.hash).toBe(second.hash);
    });
  });

  describe("@spatial/js-core model space", () => {
    it("deduplicates hashed primitives across models", () => {
      const space = new ModelSpace();
      const a = space.addPrimitive(primitiveKinds.topology.vertex, { position: [0, 0, 0] as Vec3 });
      const b = space.addPrimitive(primitiveKinds.topology.vertex, { position: [0, 0, 0] as Vec3 });
      expect(a.hash).toBe(b.hash);
      expect(space.listPrimitives()).toHaveLength(1);
    });
  });

  describe("@spatial/js-core catalog", () => {
    it("loads builtin model definition assets", () => {
      const catalog = loadBuiltinCatalog();
      expect(catalog.extensions.length).toBeGreaterThan(0);
      expect(catalog.typologies.some((entry) => entry.id === "builtin.curve.line")).toBe(true);
      expect(catalog.interactions.some((entry) => entry.id === "curve.line")).toBe(true);
      expect(catalog.actions.length).toBeGreaterThan(0);
    });
  });

  describe("@spatial/js-core actions", () => {
    it("interprets declarative actions and nested actions", async () => {
      const registry = new ActionRegistry();
      registry.register({ schema: "spatial.action/v1", id: "child", version: "1.0.0", steps: [{ op: "return", data: { kind: "const", value: 5 }, result: { kind: "const", value: "child-result" } }] });
      registry.register({
        schema: "spatial.action/v1",
        id: "parent",
        version: "1.0.0",
        variables: [{ name: "origin", value: { kind: "path", root: "params", segments: [{ kind: "field", name: "origin" }] } }],
        steps: [
          { op: "guard", condition: { kind: "exists", target: { root: "params", segments: [{ kind: "field", name: "origin" }] } } },
          { op: "setContext", values: { cursor: { kind: "var", name: "origin" } } },
          { op: "action", action: "child", assignTo: "childResult" },
          { op: "return", data: { kind: "var", name: "childResult" }, result: { kind: "path", root: "context", segments: [{ kind: "field", name: "cursor" }] } },
        ],
      });
      const result = await registry.run({ action: "parent", params: { origin: [1, 2, 3] }, runtime: { kernel: {}, model: new Model({ id: "m" }), modelSpace: new ModelSpace() } });
      expect(result.ok).toBe(true);
      expect(result.data).toBe("child-result");
      expect(result.result).toEqual([1, 2, 3]);
    });
  });

  describe("@spatial/js-core interactions", () => {
    it("executes a static state machine and commits via action", async () => {
      const registry = new ActionRegistry();
      registry.register({ schema: "spatial.action/v1", id: "commit.line", version: "1.0.0", steps: [{ op: "return", data: { kind: "path", root: "params", segments: [{ kind: "field", name: "points" }] } }] });
      const runtime = createInteractionRuntime({
        interaction: {
          schema: "spatial.interaction/v1",
          id: "line",
          version: "1.0.0",
          machine: {
            initial: "idle",
            states: [
              { name: "idle", on: [{ event: "pick.start", transitions: [{ target: "picked", effects: [{ op: "assign", target: { root: "context", segments: [{ kind: "field", name: "points" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "point" }] } }] }] }] },
              { name: "picked", final: true, on: [{ event: "pick.end", transitions: [{ target: "picked", effects: [{ op: "append", target: { root: "context", segments: [{ kind: "field", name: "points" }] }, value: { kind: "path", root: "event", segments: [{ kind: "field", name: "point" }] } }] }] }] },
            ],
          },
          commit: { when: "pick.end", operation: { kind: "action", action: "commit.line", params: { points: { kind: "path", root: "context", segments: [{ kind: "field", name: "points" }] } } } },
        },
        runtime: { kernel: {}, model: new Model({ id: "m" }), modelSpace: new ModelSpace() },
        actionRegistry: registry,
      });
      await runtime.send({ kind: "pick.start", point: [0, 0, 0] });
      const snapshot = await runtime.send({ kind: "pick.end", point: [2, 0, 0] });
      expect(snapshot.state).toBe("picked");
      expect(snapshot.lastResult?.data).toEqual([[0, 0, 0], [2, 0, 0]]);
    });
  });
}
