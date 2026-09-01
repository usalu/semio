/** 🌐️ Declaration mirror of `🟨️sequence-browser.js`'s real exported surface — the only export is
 *  `createSequenceBrowserFeatures`, whose resolved shape mirrors `🟨️sequence-host.js`'s
 *  `createSequenceFeatures` return value (`document`/`editing`/`execution`/`viewport`/`input`/
 *  `layout`/`selection`/`preview`/`playback`/`lifetime`), each method wrapping one `SequenceOperation`
 *  request over the linear-memory host and resolving with whatever decoder that call site passes to
 *  `task(...)` (`identity` when omitted, hence the raw `Uint8Array` return types below). */

/** 🧵️ One in-flight request over the Sequence Wasm bridge — `host.start(...)`'s own shape, spread by
 *  `mapTask` with `result` remapped through the call site's decoder. */
export interface SequenceTask<T> {
  requestId: bigint | undefined;
  result: Promise<T>;
  cancel(): boolean;
  subscribe(observer: (message: unknown) => void): () => void;
}

export interface SequenceDocumentFeatures {
  loadFixtureJson(json: string): SequenceTask<Uint8Array>;
  fixtureJson(): SequenceTask<string>;
  catalogueJson(): SequenceTask<string>;
}

export interface SequenceEditingFeatures {
  addStep(kind: string, x: number, y: number): SequenceTask<string>;
  addStepDropped(kind: string, x: number, y: number, picked?: string | null): SequenceTask<string>;
  addStepToSlot(kind: string, x: number, y: number, owner: string, name: string): SequenceTask<string>;
  setStepCollapsed(id: string, value: boolean): SequenceTask<boolean>;
  pickStepIdAtScreen(x: number, y: number): SequenceTask<string | undefined>;
  buildPathJson(): SequenceTask<string>;
  removeStep(id: string): SequenceTask<boolean>;
  setStepParamsJson(id: string, json: string): SequenceTask<Uint8Array>;
  connectSteps(from: string, to: string): SequenceTask<string>;
  disconnectSteps(from: string, to: string): SequenceTask<boolean>;
}

export interface SequenceExecutionFeatures {
  compileText(): SequenceTask<string>;
  compiledWireLiteral(): SequenceTask<string>;
  run(): SequenceTask<string>;
}

export interface SequenceViewportFeatures {
  attach(canvas?: unknown): SequenceTask<Uint8Array>;
  gpuReady(): SequenceTask<boolean>;
  setSize(width: number, height: number, dpr: number): SequenceTask<Uint8Array>;
  renderFrame(): SequenceTask<unknown>;
}

export interface SequenceInputFeatures {
  worldFromScreen(x: number, y: number): SequenceTask<string>;
  pointerDownScreen(x: number, y: number, button: number, shift: boolean, ctrl: boolean, alt: boolean): SequenceTask<Uint8Array>;
  pointerMoveScreen(x: number, y: number, shift: boolean, ctrl: boolean, alt: boolean): SequenceTask<Uint8Array>;
  pointerUpScreen(x: number, y: number, shift: boolean, ctrl: boolean, alt: boolean): SequenceTask<Uint8Array>;
  wheelScreen(x: number, y: number, deltaY: number): SequenceTask<Uint8Array>;
}

export interface SequenceLayoutFeatures {
  reorganize(json: string): SequenceTask<Uint8Array>;
  lodScaleJson(): SequenceTask<string>;
  setAutomaticLod(value: boolean): SequenceTask<Uint8Array>;
  setForcedDrawLodLabel(value: string): SequenceTask<Uint8Array>;
  drawLodLabel(): SequenceTask<string>;
  setCanvasThemeJson(json: string): SequenceTask<Uint8Array>;
}

export interface SequenceSelectionFeatures {
  selectedNodeIds(): SequenceTask<unknown>;
  setSelection(ids: readonly string[]): SequenceTask<Uint8Array>;
  labelOverlayPaintStateJson(): SequenceTask<string>;
  hoveredNodeId(): SequenceTask<string | undefined>;
}

export interface SequencePreviewFeatures {
  preselectNodeIdsJson(): SequenceTask<string>;
  selectionPreviewPointsJson(): SequenceTask<string>;
  selectionPreviewCrossing(): SequenceTask<boolean>;
  selectionPreviewMethod(): SequenceTask<string>;
  selectionUnionBoundsScreenJson(): SequenceTask<string>;
  setSelectionOptions(method: string, mode: string): SequenceTask<Uint8Array>;
  setGhostStep(kind: string, x: number, y: number): SequenceTask<Uint8Array>;
  clearGhostStep(): SequenceTask<Uint8Array>;
}

export interface SequencePlaybackFeatures {
  play(): SequenceTask<Uint8Array>;
  pause(): SequenceTask<Uint8Array>;
  stop(): SequenceTask<Uint8Array>;
}

/** 🧵️ `{ slot, generation }` — the same handle shape `readHandle`/`encodeHandle` codec on the wire. */
export interface SequenceHandle {
  slot: number;
  generation: number;
}

export interface SequenceLifetimeFeatures {
  session: SequenceHandle;
  close(): Promise<void>;
  terminalIsEmpty(): boolean;
}

export interface SequenceFeatures {
  document: SequenceDocumentFeatures;
  editing: SequenceEditingFeatures;
  execution: SequenceExecutionFeatures;
  viewport: SequenceViewportFeatures;
  input: SequenceInputFeatures;
  layout: SequenceLayoutFeatures;
  selection: SequenceSelectionFeatures;
  preview: SequencePreviewFeatures;
  playback: SequencePlaybackFeatures;
  lifetime: SequenceLifetimeFeatures;
}

/** 🌉️ `createSequenceBrowserFeatures`'s options — `source`/`imports`/`instantiate` drive
 *  instantiation, the rest passes straight through to `createSequenceHost`. */
export interface SequenceBrowserOptions {
  source: BufferSource | WebAssembly.Module | Response | PromiseLike<Response>;
  imports?: WebAssembly.Imports;
  instantiate?: (source: unknown, imports: WebAssembly.Imports) => Promise<WebAssembly.WebAssemblyInstantiatedSource | WebAssembly.Instance>;
  resolveCanvas?: () => unknown;
  render?: (canvas: unknown, state: unknown) => void;
  schedule?: (callback: () => void) => void;
  maximumInFlight?: number;
}

export function createSequenceBrowserFeatures(options: SequenceBrowserOptions): Promise<SequenceFeatures>;
