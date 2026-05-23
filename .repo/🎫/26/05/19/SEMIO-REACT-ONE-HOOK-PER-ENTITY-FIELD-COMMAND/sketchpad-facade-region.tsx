// #region 🎨SketchpadFacade
export type { DesignDiff } from "../rendering/index";
export type ConnectionDiff = Readonly<Record<string, unknown>>;
export type PieceDiff = Readonly<Record<string, unknown>>;
export type TypeDiff = Readonly<Record<string, unknown>>;
export type KitDiff = Readonly<Record<string, unknown>>;
export type DiffStatus = string;

export { getKitPorts } from "../rendering/index";
export { SEMIO_IN_MEMORY_KIT_URI, kitStoreFromKitStoreClient } from "../../js";

/** @emoji 📏 Sketchpad geometric tolerance constant. */
export const TOLERANCE = 0.001;

export type ActiveKitTabScope = Readonly<{ id: string }>;
const ActiveKitTabContext = React.createContext<string | null>(null);
export { ActiveKitTabContext };

/** @emoji 📌 Tab-scoped kit id for sketchpad shell / scene bridge. */
export function ActiveKitTabContextProvider(props: Readonly<{ kitTabId: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(ActiveKitTabContext.Provider, { value: props.kitTabId }, props.children);
}

/** @emoji 📌 Active sketchpad kit tab id (`{ id }`) or `null` outside {@link ActiveKitTabContextProvider}. */
export function useActiveKitTab(): ActiveKitTabScope | null {
  const id = React.useContext(ActiveKitTabContext);
  return id == null || id === "" ? null : { id };
}

/** @emoji 📌 Whether {@link ActiveKitTabContextProvider} is mounted. */
export function useIsInActiveKitTab(): boolean {
  return useActiveKitTab() != null;
}

export type KitAlternativeSummary = Readonly<{ id: string; name: string }>;
const KIT_ALTERNATIVE_EMPTY: readonly KitAlternativeSummary[] = Object.freeze([]);
const KitAlternativeSelectionContext = React.createContext<Readonly<{ selectedAlternativeId: string | null; setSelectedAlternativeId: (id: string | null) => void; alternatives: readonly KitAlternativeSummary[] }>>({
  selectedAlternativeId: null,
  setSelectedAlternativeId: () => {},
  alternatives: KIT_ALTERNATIVE_EMPTY,
});

/** @emoji 🌱 Sketchpad alternative picker scope (WIP: empty until graph alternatives wire through). */
export function KitAlternativeSelectionProvider(props: Readonly<{ kitId?: string; children: ReactNode }>): React.ReactElement {
  void props.kitId;
  const value = React.useMemo(
    () => ({
      selectedAlternativeId: null as string | null,
      setSelectedAlternativeId: (_id: string | null) => {},
      alternatives: KIT_ALTERNATIVE_EMPTY,
    }),
    [],
  );
  return React.createElement(KitAlternativeSelectionContext.Provider, { value }, props.children);
}

/** @emoji 🌱 Selected alternative id + setter for host dropdowns. */
export function useKitAlternativeSelection(): readonly [string | null, (id: string | null) => void] {
  const v = React.useContext(KitAlternativeSelectionContext);
  return [v.selectedAlternativeId, v.setSelectedAlternativeId] as const;
}

/** @emoji 🌱 Alternatives list for host dropdowns. */
export function useKitAlternatives(): readonly KitAlternativeSummary[] {
  return React.useContext(KitAlternativeSelectionContext).alternatives;
}

//#region 🧾KitHostStore
const DEFAULT_KIT_SYNC = Object.freeze({ status: "idle", dirty: false, readonly: false, lastSyncedAt: null, error: null });

type KitPlain = Kit | Record<string, unknown>;

function isKitPlainDto(k: unknown): k is Record<string, unknown> {
  return k != null && typeof k === "object" && (Array.isArray((k as { designs?: unknown }).designs) || Array.isArray((k as { types?: unknown }).types));
}

function hostIdStr(x: unknown): string {
  if (x == null) return "";
  if (typeof x === "string") return x;
  if (typeof x === "object" && x !== null && "id" in x) return String((x as { id: unknown }).id);
  return String(x);
}

function hostPlainDto(store: KitHostStore): Record<string, unknown> {
  const k = store.getSnapshot().kit;
  if (isKitPlainDto(k)) return k;
  return { id: String((k as Kit).id ?? ""), designs: [], types: [], qualities: [], folders: [], files: [], concepts: [], tags: [], authors: [] };
}

export type KitHostSnap = Readonly<{ kit: Kit; sync: Readonly<{ status: string; dirty: boolean; readonly: boolean; lastSyncedAt: string | null; error: unknown }> }>;

export type KitHostStore = Readonly<{
  getSnapshot: () => KitHostSnap;
  subscribe: (onChange: () => void) => () => void;
  replace: (kit: KitPlain) => void;
  readonly name?: string;
}>;

/** @emoji 🧠 In-memory kit host for sketchpad temporary kits and imports. */
export class InMemoryKitStore implements KitHostStore {
  private readonly listeners = new Set<() => void>();
  private _kit: KitPlain;
  readonly name = "InMemoryKitStore";

  constructor(seed: KitPlain) {
    this._kit = seed;
  }

  getSnapshot(): KitHostSnap {
    return { kit: this._kit as Kit, sync: DEFAULT_KIT_SYNC };
  }

  subscribe(onChange: () => void): () => void {
    this.listeners.add(onChange);
    return () => {
      this.listeners.delete(onChange);
    };
  }

  replace(kit: KitPlain): void {
    this._kit = kit;
    for (const l of this.listeners) {
      try {
        l();
      } catch {
        /* ignore */
      }
    }
  }
}

export type KitJsonFileAdapter = Readonly<{ read: () => Promise<string>; write: (json: string) => Promise<void> }>;
export type KitFolderAdapter = Readonly<Record<string, unknown>>;

export async function createJsonFileKitStore(adapter: KitJsonFileAdapter): Promise<KitHostStore> {
  let text = "";
  try {
    text = await adapter.read();
  } catch {
    text = "";
  }
  let seed: Record<string, unknown> = { id: `kit-${Date.now()}`, name: "Untitled", designs: [], types: [], qualities: [], folders: [], files: [] };
  if (text.trim() !== "") {
    try {
      const parsed = JSON.parse(text) as Record<string, unknown>;
      if (parsed && typeof parsed === "object") seed = parsed;
    } catch {
      /* keep seed */
    }
  }
  const store = new InMemoryKitStore(seed);
  const persist = async (json: string) => {
    await adapter.write(json);
  };
  return Object.assign(store, { persistBundle: persist, initialBundleJson: text });
}

export async function createFolderKitStore(_adapter: KitFolderAdapter, initialKit?: unknown): Promise<KitHostStore> {
  const seed = (initialKit && typeof initialKit === "object" ? initialKit : { id: `kit-${Date.now()}`, name: "Untitled", designs: [], types: [], folders: [], files: [] }) as KitPlain;
  return new InMemoryKitStore(seed);
}

/** @emoji 🧾 Legacy string-command entry for DTO host stores; prefer {@link useKitCommand} on GraphQL kits. */
export async function executeSemioKitCommand(store: KitHostStore, command: string, _origin: string, ...args: unknown[]): Promise<unknown> {
  if (command === "semio.kit.undo" || command === "semio.kit.redo") {
    return { ok: false, error: { kind: "NotSupported", message: `${command}: no kit-store client bridge` } };
  }
  if (command === "semio.kit.import") {
    return { ok: false, error: "semio.kit.import: not wired in this build" };
  }
  if (command === "semio.kit.export") {
    return { ok: true };
  }
  if (command === "semio.kit.createFolder" && args[0]) {
    const snap = hostPlainDto(store);
    const nextFolders = [...((snap.folders as unknown[]) ?? []), args[0] as Record<string, unknown>];
    store.replace({ ...snap, folders: nextFolders });
    return { ok: true };
  }
  if (command === "semio.kit.moveToFolder" && args.length >= 3) {
    const entityId = hostIdStr(args[0]);
    const kind = String(args[1] ?? "");
    const folderId = hostIdStr(args[2]);
    const plain = JSON.parse(JSON.stringify(hostPlainDto(store))) as Record<string, unknown>;
    if (kind === "type") {
      for (const t of (plain.types as unknown[] | undefined) ?? []) {
        if (t && typeof t === "object" && (t as { id?: string }).id === entityId) (t as { folder?: string }).folder = folderId;
      }
    } else if (kind === "design") {
      for (const d of (plain.designs as unknown[] | undefined) ?? []) {
        if (d && typeof d === "object" && (d as { id?: string }).id === entityId) (d as { folder?: string }).folder = folderId;
      }
    } else if (kind === "quality") {
      for (const q of (plain.qualities as unknown[] | undefined) ?? []) {
        if (q && typeof q === "object" && (q as { id?: string }).id === entityId) (q as { folder?: string }).folder = folderId;
      }
    } else if (kind === "file") {
      for (const f of (plain.files as unknown[] | undefined) ?? []) {
        if (f && typeof f === "object" && (f as { id?: string }).id === entityId) (f as { folder?: { id: string } }).folder = { id: folderId };
      }
    } else if (kind === "folder") {
      for (const fo of (plain.folders as unknown[] | undefined) ?? []) {
        if (fo && typeof fo === "object" && (fo as { id?: string }).id === entityId) {
          (fo as { parent?: { id: string } }).parent = { id: folderId };
          delete (fo as { path?: string }).path;
        }
      }
    } else {
      return { ok: false, error: { kind: "InvalidValue", message: `moveToFolder: unknown kind ${kind}` } };
    }
    store.replace(plain);
    return { ok: true };
  }
  return { ok: false, error: { kind: "NotSupported", message: `unhandled ${command}` } };
}

/** @emoji 🧾 Memoized kit string-command engine for legacy sketchpad callers. */
export function useKitCommandEngineExplicitOrigin(store: KitHostStore): { execute: (...args: unknown[]) => Promise<unknown> } {
  return {
    execute: async (command: unknown, origin: unknown, ...rest: unknown[]) => executeSemioKitCommand(store, String(command), String(origin ?? ""), ...rest),
  };
}

export async function kitHostUndo(_store: KitHostStore): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "kitHostUndo: no kit-store client bridge", field: undefined, entity: undefined } };
}

export async function kitHostRedo(_store: KitHostStore): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "kitHostRedo: no kit-store client bridge", field: undefined, entity: undefined } };
}

export async function applyKitHostGraphOperation(_store: KitHostStore, _op: unknown): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "applyKitHostGraphOperation: no kit-store client bridge", field: undefined, entity: undefined } };
}

export function attachSketchpadKitReadShell(_kitStore: KitHostStore): void {
  void _kitStore;
}

export function getOrCreateKitFileState(_kitId: string, _fileId: string): unknown {
  void _kitId;
  void _fileId;
  return null;
}
//#endregion 🧾KitHostStore

//#region 📦KitRegistry
export type KitPersistenceInfo = Readonly<{ kind: "temporary" | "file" | "folder" | "remote" }>;

export type KitRegistryEntry = Readonly<{
  store: KitHostStore;
  kitClient: unknown | null;
  refs: number;
  persistence: KitPersistenceInfo;
}>;

export type KitRegistryValue = Readonly<{
  activeKitId: string | undefined;
  setActiveKit: (id: string | undefined) => void;
  open: (id: string, init: Readonly<{ store?: KitHostStore; kitClient?: unknown | null; initialKit?: unknown }>) => Promise<void>;
  openTemporary: (initialKit?: unknown) => Promise<string>;
  openJsonFile: (kitId: string, adapter: KitJsonFileAdapter) => Promise<void>;
  openFolder: (kitId: string, adapter: KitFolderAdapter, initialKit?: unknown) => Promise<void>;
  openRemote: (kitId: string, config: Readonly<{ serverUrl: string }>) => Promise<void>;
  close: (id: string) => void;
  get: (id: string) => KitRegistryEntry | undefined;
  list: () => readonly string[];
  status: (id: string) => "idle" | "loading" | "ready" | "error";
}>;

const _kitRegistryListListeners = new Set<() => void>();

function emitKitRegistryListChanged(): void {
  for (const l of _kitRegistryListListeners) {
    try {
      l();
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 📣 Subscribe to registry open/close list changes. */
export function subscribeKitRegistryListChanged(onChange: () => void): () => void {
  _kitRegistryListListeners.add(onChange);
  return () => {
    _kitRegistryListListeners.delete(onChange);
  };
}

const KitRegistryContext = React.createContext<KitRegistryValue | null>(null);

let _semioKitRegistryBridge: KitRegistryValue | null = null;

/** @emoji 🧾 Registry bridge for {@link SketchpadStore} and other non-hook callers. */
export function getKitRegistryBridge(): KitRegistryValue | null {
  return _semioKitRegistryBridge;
}

/** @emoji 📦 Host registry for open kit tabs (sketchpad). */
export function KitRegistryProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const rowsRef = React.useRef(new Map<string, KitRegistryEntry & { unsub?: () => void }>());
  const loadingRef = React.useRef(new Set<string>());
  const errRef = React.useRef(new Map<string, Error>());
  const [registryEpoch, bump] = React.useReducer((x: number) => x + 1, 0);
  const [activeKitId, setActiveKitId] = React.useState<string | undefined>(undefined);

  const open = React.useCallback(
    async (kitId: string, init: Readonly<{ store?: KitHostStore; kitClient?: unknown | null; initialKit?: unknown }>) => {
      const cur = rowsRef.current.get(kitId);
      if (cur) {
        (cur as { refs: number }).refs += 1;
        bump();
        return;
      }
      if (init.store == null) {
        throw new Error("KitRegistry.open requires init.store");
      }
      loadingRef.current.add(kitId);
      errRef.current.delete(kitId);
      bump();
      try {
        const store = init.store;
        const kitClient = init.kitClient ?? null;
        const persistence: KitPersistenceInfo = { kind: store.name === "InMemoryKitStore" ? "temporary" : "file" };
        rowsRef.current.set(kitId, { store, kitClient, refs: 1, persistence });
        emitKitRegistryListChanged();
      } catch (e) {
        errRef.current.set(kitId, e instanceof Error ? e : new Error(String(e)));
        throw e;
      } finally {
        loadingRef.current.delete(kitId);
        bump();
      }
    },
    [],
  );

  const close = React.useCallback((kitId: string) => {
    const row = rowsRef.current.get(kitId);
    if (!row) return;
    row.refs -= 1;
    if (row.refs <= 0) {
      row.unsub?.();
      rowsRef.current.delete(kitId);
      setActiveKitId((cur) => (cur === kitId ? undefined : cur));
      emitKitRegistryListChanged();
    }
    bump();
  }, []);

  const value = React.useMemo<KitRegistryValue>(
    () => ({
      get activeKitId() {
        return activeKitId;
      },
      setActiveKit: setActiveKitId,
      open,
      async openTemporary(initialKit) {
        const k = `kit-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
        const seed = (initialKit && typeof initialKit === "object" ? initialKit : { id: k, name: "Untitled", designs: [], types: [], qualities: [], folders: [], files: [] }) as KitPlain;
        await open(k, { store: new InMemoryKitStore(seed) });
        return k;
      },
      async openJsonFile(kitId, adapter) {
        const store = await createJsonFileKitStore(adapter);
        await open(kitId, { store });
      },
      async openFolder(kitId, adapter, initialKit) {
        const store = await createFolderKitStore(adapter, initialKit);
        await open(kitId, { store });
      },
      async openRemote(kitId, config) {
        const seed = { id: kitId, name: config.serverUrl, remote: config.serverUrl, designs: [], types: [] };
        await open(kitId, { store: new InMemoryKitStore(seed) });
      },
      close,
      get(kitId) {
        return rowsRef.current.get(kitId);
      },
      list() {
        return Array.from(rowsRef.current.keys());
      },
      status(kitId) {
        if (loadingRef.current.has(kitId)) return "loading";
        if (errRef.current.has(kitId)) return "error";
        if (rowsRef.current.has(kitId)) return "ready";
        return "idle";
      },
    }),
    [activeKitId, open, close, registryEpoch],
  );

  _semioKitRegistryBridge = value;
  React.useLayoutEffect(() => {
    return () => {
      _semioKitRegistryBridge = null;
      _kitRegistryListListeners.clear();
    };
  }, []);

  return React.createElement(KitRegistryContext.Provider, { value }, props.children);
}

/** @emoji 📦 Sketchpad host root: {@link KitRegistryProvider}. */
export function KitStoreProvider(props: Readonly<{ children: ReactNode; initialKit?: unknown }>): React.ReactElement {
  void props.initialKit;
  return React.createElement(KitRegistryProvider, null, props.children);
}

export function useKitRegistry(): KitRegistryValue {
  const v = React.useContext(KitRegistryContext);
  if (v == null) throw new Error("useKitRegistry must be used within KitRegistryProvider.");
  return v;
}

/** @emoji 📦 Like {@link useKitRegistry} but returns null outside provider. */
export function useKitRegistrySafe(): KitRegistryValue | null {
  return React.useContext(KitRegistryContext);
}
//#endregion 📦KitRegistry

//#region 🔌KitRuntimeBridge
export type KitRuntimeContextValue = Readonly<{
  kitId: string;
  store: KitHostStore;
  kitClient: unknown | null;
}>;

const KitRuntimeContext = React.createContext<KitRuntimeContextValue | null>(null);

function useKitRuntimeSafe(): KitRuntimeContextValue | null {
  return React.useContext(KitRuntimeContext);
}

/** @emoji 📌 Live kit host snapshot for registry tab or {@link KitRuntimeContext}. */
export function useKitStoreSnapshot(explicitKitId?: string): KitHostSnap | null {
  const runtime = useKitRuntimeSafe();
  const tabId = useActiveKitTab()?.id;
  const regActive = getKitRegistryBridge()?.activeKitId;
  const kitId = explicitKitId != null && explicitKitId !== "" ? explicitKitId : tabId != null && tabId !== "" ? tabId : regActive;
  const [registryEpoch, setRegistryEpoch] = React.useState(0);
  React.useEffect(() => subscribeKitRegistryListChanged(() => setRegistryEpoch((n) => n + 1)), []);
  const reg = getKitRegistryBridge();
  const entry = kitId != null && reg != null ? reg.get(kitId) : undefined;
  const store = runtime?.store ?? entry?.store ?? null;
  const [snap, setSnap] = React.useState<KitHostSnap | null>(null);
  React.useEffect(() => {
    if (store == null) {
      setSnap(null);
      return;
    }
    const pull = () => setSnap(store.getSnapshot());
    pull();
    return store.subscribe(pull);
  }, [store, registryEpoch, runtime?.kitId, kitId]);
  if (store == null) {
    const gqlKit = React.useContext(KitHandleContext);
    if (gqlKit == null) return null;
    return { kit: gqlKit, sync: DEFAULT_KIT_SYNC };
  }
  return snap;
}

/** @emoji 📥 Active kit host store for the resolved tab / runtime scope. */
export function useKitStore(): KitHostStore | null {
  const runtime = useKitRuntimeSafe();
  if (runtime?.store) return runtime.store;
  const tabId = useActiveKitTab()?.id;
  const reg = getKitRegistryBridge();
  const kitId = tabId != null && tabId !== "" ? tabId : reg?.activeKitId;
  if (kitId == null || reg == null) return null;
  return reg.get(kitId)?.store ?? null;
}

/** @emoji 📂 Open kit ids from {@link KitRegistryProvider}. */
export function useOpenKits(): readonly string[] {
  const [registryEpoch, setRegistryEpoch] = React.useState(0);
  React.useEffect(() => subscribeKitRegistryListChanged(() => setRegistryEpoch((n) => n + 1)), []);
  const reg = getKitRegistryBridge();
  return reg != null ? Object.freeze(reg.list()) : EMPTY_IDS;
}

export function useRegistryHasKit(kitId: string): boolean {
  return getKitRegistryBridge()?.get(kitId) != null;
}

export function useRegistryKitPersistenceKind(kitId: string): KitPersistenceInfo["kind"] | null {
  return getKitRegistryBridge()?.get(kitId)?.persistence.kind ?? null;
}

export function useKitStoredFileUrls(_explicitKitId?: string): readonly [Readonly<Record<string, string>>] {
  void _explicitKitId;
  return [Object.freeze({})];
}

export function useKitFileUrl(_fileId?: string): string | undefined {
  void _fileId;
  return undefined;
}

export function useKitFileBlobUrl(_fileId?: string): string | undefined {
  void _fileId;
  return undefined;
}

export function createDefaultBrowserSketchpadFileKitStoreFactory(): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createDefaultBrowserSketchpadFileKitStoreFactory: use fileKitStoreFactory prop on Sketchpad");
  };
}

export function createDefaultBrowserSketchpadRemoteKitStoreFactory(): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createDefaultBrowserSketchpadRemoteKitStoreFactory: use remoteKitStoreFactory prop on Sketchpad");
  };
}

export function createVscodeWebviewSketchpadFileKitStoreFactory(_vscodeApi: { postMessage: (msg: unknown) => void }): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createVscodeWebviewSketchpadFileKitStoreFactory: use fileKitStoreFactory prop on Sketchpad");
  };
}

/** @emoji 📌 Kit `files` rows for sketchpad panels (`const [files] = useFilesFull()`). */
export function useFilesFull(_explicitKitId?: string): KitFieldBinding<readonly File[]> {
  void _explicitKitId;
  return [Object.freeze([]) as readonly File[], noopKitFieldSet, WRITE_STATUS_IDLE];
}

/** @emoji 📌 Kit `tags` rows for sketchpad panels (`const [tags] = useTagsFull()`). */
export function useTagsFull(_explicitKitId?: string): KitFieldBinding<readonly Tag[]> {
  void _explicitKitId;
  return [useKitTags(), noopKitFieldSet, WRITE_STATUS_IDLE];
}

/** @emoji 📌 Included design ids for explode / replace UI (WIP: empty without kit-store client). */
export function useExplodeableDesignNodes(_designId?: string): readonly [readonly string[]] {
  void _designId;
  return [EMPTY_IDS];
}

export type SchemaScopeEntityKind = string;
export const SchemaScopeContext = React.createContext<SchemaScopeEntityKind | null>(null);

/** @emoji 🧭 Maps entity id prefix to schema scope kind for sketchpad property panels. */
export function schemaScopeForEntityId(entityId: string): SchemaScopeEntityKind | null {
  if (entityId.startsWith("design-") || entityId.includes("/designs/")) return "design";
  if (entityId.startsWith("type-") || entityId.includes("/types/")) return "type";
  return null;
}

/** @emoji 🔗 Attach backbone via {@link useStoreCommand} (alias for sketchpad docs). */
export function useAttachBackbone(): (input: { dev?: { filePath: string }; local?: { folderPath: string }; remote?: { serverUrl: string } }) => Promise<SetResult> {
  const { run } = useStoreCommand();
  return React.useCallback(
    async (input) => {
      const uri = input.dev?.filePath ?? input.local?.folderPath ?? input.remote?.serverUrl;
      if (uri == null || uri === "") {
        return { ok: false, error: { kind: "InvalidValue", message: "useAttachBackbone: no backbone target.", field: undefined, entity: undefined } };
      }
      return run.attachBackbone(uri);
    },
    [run],
  );
}
//#endregion 🔌KitRuntimeBridge
// #endregion 🎨SketchpadFacade
