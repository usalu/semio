import React, { createContext, useContext, useMemo } from "react";
import { v4 as uuidv4 } from "uuid";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

// Import domain types
import { Connection, ConnectionId, DesignDiff, DesignId, DesignIdLike, designIdLikeToDesignId, Kit, KitId, KitIdLike, kitIdLikeToKitId, Piece, PieceId, Point, PortId, Type, Vector } from "./semio";

const sqlWasmUrl = "https://cdn.jsdelivr.net/npm/sql.js@1.8.0/dist/sql-wasm.wasm";

export enum Mode {
  USER = "user",
  GUEST = "guest",
}

export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export enum Layout {
  NORMAL = "normal",
  TOUCH = "touch",
}

// Domain interfaces for Design Editor
export interface DesignEditorSelection {
  selectedPieceIds: PieceId[];
  selectedConnections: ConnectionId[];
  selectedPiecePortId?: { pieceId: PieceId; portId: PortId };
}

export type DesignEditorFullscreenPanel = "none" | "type-library" | "file-browser" | "configuration" | "world";

export interface DesignEditorPresence {
  cursor?: { x: number; y: number };
  camera?: {
    position: Point;
    forward: Vector;
    up: Vector;
  };
}

export interface DesignEditorPresenceOther extends DesignEditorPresence {
  name: string;
}

export interface DesignEditorOperationStackEntry {
  undo: DesignDiff;
  redo: DesignDiff;
  selection: DesignEditorSelection;
}

export interface DesignEditorState {
  designId: DesignId;
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  operationStack: DesignEditorOperationStackEntry[];
  operationIndex: number;
  presence: DesignEditorPresence;
  others?: DesignEditorPresenceOther[];
}

export interface SketchpadState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
}

// Main store implementation
class InternalSketchpadStore {
  private id?: string;
  private sketchpadDoc: Y.Doc;
  private kitDocs: Map<string, Y.Doc> = new Map();
  private sketchpadIndexeddbProvider?: IndexeddbPersistence;
  private kitIndexeddbProviders: Map<string, IndexeddbPersistence> = new Map();
  private fileUrls: Map<string, string> = new Map();

  constructor(id?: string) {
    this.sketchpadDoc = new Y.Doc();
    if (id) {
      this.id = id;
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad:${id}`, this.sketchpadDoc);
    }

    const ySketchpad = this.getYSketchpad();
    if (!ySketchpad.has("mode")) ySketchpad.set("mode", Mode.USER);
    if (!ySketchpad.has("theme")) ySketchpad.set("theme", Theme.SYSTEM);
    if (!ySketchpad.has("layout")) ySketchpad.set("layout", Layout.NORMAL);
    if (!ySketchpad.has("activeDesignEditorId")) ySketchpad.set("activeDesignEditorId", "");
    this.sketchpadDoc.getMap("designEditors");
  }

  private getYSketchpad(): Y.Map<any> {
    return this.sketchpadDoc.getMap("sketchpad");
  }

  private getKitDoc(kitId: KitIdLike): Y.Doc {
    const kitKey = this.kitKey(kitId);
    let doc = this.kitDocs.get(kitKey);
    if (!doc) {
      doc = new Y.Doc();
      this.kitDocs.set(kitKey, doc);
      if (this.id) {
        this.kitIndexeddbProviders.set(kitKey, new IndexeddbPersistence(`semio-kit:${this.id}:${kitKey}`, doc));
      }
      doc.getMap("kit");
      doc.getMap("files");
    }
    return doc;
  }

  private kitKey(id: KitIdLike): string {
    const kitId = kitIdLikeToKitId(id);
    return `${kitId.name}::${kitId.version || ""}`;
  }

  // Public accessor for kit key
  getKitKey(id: KitIdLike): string {
    return this.kitKey(id);
  }

  // Sketchpad state methods
  getSketchpadState(): SketchpadState {
    const ySketchpad = this.getYSketchpad();
    return {
      mode: (ySketchpad.get("mode") as Mode) || Mode.USER,
      theme: (ySketchpad.get("theme") as Theme) || Theme.SYSTEM,
      layout: (ySketchpad.get("layout") as Layout) || Layout.NORMAL,
      activeDesignEditorId: ySketchpad.get("activeDesignEditorId") || undefined,
    };
  }

  setSketchpadMode(mode: Mode): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("mode", mode);
  }

  setSketchpadTheme(theme: Theme): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("theme", theme);
  }

  setSketchpadLayout(layout: Layout): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("layout", layout);
  }

  setActiveDesignEditorId(id?: string): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("activeDesignEditorId", id || "");
  }

  // Kit methods
  createKit(kit: Kit): void {
    const kitId = { name: kit.name, version: kit.version || "" };
    const doc = this.getKitDoc(kitId);
    const yKit = doc.getMap("kit");

    if (yKit.get("name")) {
      throw new Error(`Kit (${kit.name}, ${kit.version || ""}) already exists.`);
    }

    yKit.set("uuid", uuidv4());
    yKit.set("name", kit.name);
    yKit.set("description", kit.description || "");
    yKit.set("icon", kit.icon || "");
    yKit.set("image", kit.image || "");
    yKit.set("version", kit.version || "");
    yKit.set("preview", kit.preview || "");
    yKit.set("remote", kit.remote || "");
    yKit.set("homepage", kit.homepage || "");
    yKit.set("license", kit.license || "");
    yKit.set("created", new Date().toISOString());
    yKit.set("updated", new Date().toISOString());
    yKit.set("types", new Y.Map());
    yKit.set("designs", new Y.Map());
    yKit.set("qualities", new Y.Array());
  }

  getKit(kitId: KitIdLike): Kit {
    const doc = this.getKitDoc(kitId);
    const yKit = doc.getMap("kit");

    const name = yKit.get("name") as string;
    if (!name) {
      throw new Error(`Kit not found: ${JSON.stringify(kitIdLikeToKitId(kitId))}`);
    }

    return {
      name,
      description: (yKit.get("description") as string) || "",
      icon: (yKit.get("icon") as string) || "",
      image: (yKit.get("image") as string) || "",
      version: (yKit.get("version") as string) || "",
      preview: (yKit.get("preview") as string) || "",
      remote: (yKit.get("remote") as string) || "",
      homepage: (yKit.get("homepage") as string) || "",
      license: (yKit.get("license") as string) || "",
      created: new Date((yKit.get("created") as string) || new Date().toISOString()),
      updated: new Date((yKit.get("updated") as string) || new Date().toISOString()),
      types: [],
      designs: [],
      qualities: [],
    };
  }

  getKits(): Map<string, string[]> {
    const kitsMap = new Map<string, string[]>();

    this.kitDocs.forEach((doc, key) => {
      const yKit = doc.getMap("kit");
      const name = yKit.get("name") as string;
      const version = yKit.get("version") as string;

      if (name) {
        const arr = kitsMap.get(name) || [];
        arr.push(version || "");
        kitsMap.set(name, arr);
      }
    });

    return kitsMap;
  }

  // Design Editor methods
  getOrCreateDesignEditorId(kitId: KitIdLike, designId: DesignIdLike): string {
    const kitKey = this.kitKey(kitId);
    const designKey = this.designKey(designId);
    const id = `${kitKey}|${designKey}`;

    const designEditors = this.sketchpadDoc.getMap("designEditors");
    if (!designEditors.has(id)) {
      const yDesignEditor = new Y.Map();
      yDesignEditor.set("fullscreenPanel", "none");
      yDesignEditor.set("selectedPieceIds", new Y.Array());
      yDesignEditor.set("selectedConnections", new Y.Array());
      yDesignEditor.set("selectedPiecePortPieceId", "");
      yDesignEditor.set("selectedPiecePortPortId", "");
      yDesignEditor.set("isTransactionActive", false);
      yDesignEditor.set("presenceCursorX", 0);
      yDesignEditor.set("presenceCursorY", 0);
      designEditors.set(id, yDesignEditor);
    }

    return id;
  }

  private designKey(id: DesignIdLike): string {
    const designId = designIdLikeToDesignId(id);
    return `${designId.name}::${designId.variant || ""}::${designId.view || ""}`;
  }

  getDesignEditor(id: string): {
    getState: () => DesignEditorState;
    subscribe: (callback: () => void) => () => void;
  } | null {
    const designEditors = this.sketchpadDoc.getMap("designEditors");
    const yDesignEditor = designEditors.get(id);
    if (!yDesignEditor) return null;

    const parseEditorId = (editorId: string): { kitId: KitId; designId: DesignId } => {
      const [kitPart, designPart] = editorId.split("|");
      const [kitName, kitVersion = ""] = kitPart.split("::");
      const [designName, designVariant = "", designView = ""] = designPart.split("::");
      return {
        kitId: { name: kitName, version: kitVersion },
        designId: { name: designName, variant: designVariant, view: designView },
      };
    };

    const { designId } = parseEditorId(id);

    return {
      getState: () => ({
        designId,
        fullscreenPanel: (yDesignEditor as Y.Map<any>).get("fullscreenPanel") as DesignEditorFullscreenPanel,
        selection: {
          selectedPieceIds: [],
          selectedConnections: [],
        },
        designDiff: {
          pieces: { added: [], removed: [], updated: [] },
          connections: { added: [], removed: [], updated: [] },
        },
        isTransactionActive: (yDesignEditor as Y.Map<any>).get("isTransactionActive") as boolean,
        operationStack: [],
        operationIndex: -1,
        presence: {
          cursor: {
            x: (yDesignEditor as Y.Map<any>).get("presenceCursorX") as number,
            y: (yDesignEditor as Y.Map<any>).get("presenceCursorY") as number,
          },
        },
        others: [],
      }),
      subscribe: (callback: () => void) => {
        const observer = () => callback();
        (yDesignEditor as Y.Map<any>).observe(observer);
        return () => (yDesignEditor as Y.Map<any>).unobserve(observer);
      },
    };
  }

  // File methods
  createFile(kitId: KitIdLike, url: string, data: Uint8Array): void {
    const doc = this.getKitDoc(kitId);
    doc.getMap("files").set(url, data);
    const blob = new Blob([new Uint8Array(data)]);
    const blobUrl = URL.createObjectURL(blob);
    this.fileUrls.set(url, blobUrl);
  }

  getFileUrls(): Map<string, string> {
    return this.fileUrls;
  }

  // Stub methods for type compatibility
  createType(kitId: KitIdLike, type: Type): void {
    console.warn("createType not implemented");
  }

  createPiece(kitId: KitId, designId: DesignId, piece: Piece): void {
    console.warn("createPiece not implemented");
  }

  createConnection(kitId: KitId, designId: DesignId, connection: Connection): void {
    console.warn("createConnection not implemented");
  }

  async importKit(url: string, complete = false, force = false): Promise<void> {
    console.warn("importKit not implemented");
  }

  async exportKit(kitName: string, kitVersion: string, complete = false): Promise<Blob> {
    throw new Error("exportKit not implemented");
  }
}

// Zustand store interfaces
interface SketchpadZustandState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
  kits: Map<string, string[]>;
}

interface SketchpadZustandActions {
  setMode: (mode: Mode) => void;
  setTheme: (theme: Theme) => void;
  setLayout: (layout: Layout) => void;
  setActiveDesignEditorId: (id?: string) => void;
}

type SketchpadZustandStore = SketchpadZustandState & SketchpadZustandActions;

interface KitZustandState {
  kit: Kit | null;
}

type KitZustandStore = KitZustandState;

interface DesignEditorZustandState {
  designId: DesignId;
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  presence: DesignEditorPresence;
  others?: DesignEditorPresenceOther[];
}

interface DesignEditorZustandActions {
  setDesignId: (designId: DesignId) => void;
  setFullscreenPanel: (fullscreenPanel: DesignEditorFullscreenPanel) => void;
  setSelection: (selection: DesignEditorSelection) => void;
  setDesignDiff: (designDiff: DesignDiff) => void;
  setIsTransactionActive: (isTransactionActive: boolean) => void;
  setPresence: (presence: DesignEditorPresence) => void;
  setOthers: (others: DesignEditorPresenceOther[]) => void;
}

type DesignEditorZustandStore = DesignEditorZustandState & DesignEditorZustandActions;

// Store creators
const createSketchpadZustandStore = (store: InternalSketchpadStore) =>
  create<SketchpadZustandStore>()(
    subscribeWithSelector((set, get) => {
      const initialState = store.getSketchpadState();
      const initialKits = store.getKits();

      const state = {
        mode: initialState.mode,
        theme: initialState.theme,
        layout: initialState.layout,
        activeDesignEditorId: initialState.activeDesignEditorId,
        kits: initialKits,

        setMode: (mode: Mode) => {
          set({ mode });
          store.setSketchpadMode(mode);
        },
        setTheme: (theme: Theme) => {
          set({ theme });
          store.setSketchpadTheme(theme);
        },
        setLayout: (layout: Layout) => {
          set({ layout });
          store.setSketchpadLayout(layout);
        },
        setActiveDesignEditorId: (activeDesignEditorId?: string) => {
          set({ activeDesignEditorId });
          store.setActiveDesignEditorId(activeDesignEditorId);
        },
      };

      // Setup observer
      const ySketchpad = (store as any).getYSketchpad();
      ySketchpad.observeDeep(() => {
        const newState = store.getSketchpadState();
        const newKits = store.getKits();
        set({
          mode: newState.mode,
          theme: newState.theme,
          layout: newState.layout,
          activeDesignEditorId: newState.activeDesignEditorId,
          kits: newKits,
        });
      });

      return state;
    }),
  );

const createKitZustandStore = (store: InternalSketchpadStore, kitId: KitId) =>
  create<KitZustandStore>()(
    subscribeWithSelector((set, get) => {
      let initialKit: Kit | null = null;
      try {
        initialKit = store.getKit(kitId);
      } catch (e) {
        // Kit doesn't exist yet
      }

      const state = {
        kit: initialKit,
      };

      // Setup observer
      const doc = (store as any).getKitDoc(kitId);
      const yKit = doc.getMap("kit");
      yKit.observeDeep(() => {
        try {
          const newKit = store.getKit(kitId);
          set({ kit: newKit });
        } catch (e) {
          set({ kit: null });
        }
      });

      return state;
    }),
  );

const createDesignEditorZustandStore = (store: InternalSketchpadStore, id: string) =>
  create<DesignEditorZustandStore>()(
    subscribeWithSelector((set, get) => {
      const designEditorStore = store.getDesignEditor(id);
      if (!designEditorStore) {
        throw new Error(`Design editor store with id ${id} not found`);
      }

      const initialState = designEditorStore.getState();

      const state = {
        designId: initialState.designId,
        fullscreenPanel: initialState.fullscreenPanel,
        selection: initialState.selection,
        designDiff: initialState.designDiff,
        isTransactionActive: initialState.isTransactionActive,
        presence: initialState.presence,
        others: initialState.others,

        setDesignId: (designId: DesignId) => set({ designId }),
        setFullscreenPanel: (fullscreenPanel: DesignEditorFullscreenPanel) => set({ fullscreenPanel }),
        setSelection: (selection: DesignEditorSelection) => set({ selection }),
        setDesignDiff: (designDiff: DesignDiff) => set({ designDiff }),
        setIsTransactionActive: (isTransactionActive: boolean) => set({ isTransactionActive }),
        setPresence: (presence: DesignEditorPresence) => set({ presence }),
        setOthers: (others: DesignEditorPresenceOther[]) => set({ others }),
      };

      // Setup observer
      const designEditors = (store as any).sketchpadDoc.getMap("designEditors");
      designEditors.observeDeep(() => {
        const editorStore = store.getDesignEditor(id);
        if (editorStore) {
          const newState = editorStore.getState();
          set({
            designId: newState.designId,
            fullscreenPanel: newState.fullscreenPanel,
            selection: newState.selection,
            designDiff: newState.designDiff,
            isTransactionActive: newState.isTransactionActive,
            presence: newState.presence,
            others: newState.others,
          });
        }
      });

      return state;
    }),
  );

// Enhanced store class
class EnhancedSketchpadStore extends InternalSketchpadStore {
  private sketchpadZustandStore?: ReturnType<typeof createSketchpadZustandStore>;
  private kitZustandStores = new Map<string, ReturnType<typeof createKitZustandStore>>();
  private designEditorZustandStores = new Map<string, ReturnType<typeof createDesignEditorZustandStore>>();

  constructor(id?: string) {
    super(id);
    this.initializeSketchpadZustandStore();
  }

  private initializeSketchpadZustandStore() {
    this.sketchpadZustandStore = createSketchpadZustandStore(this);
  }

  getKitZustandStore(kitId: KitId): ReturnType<typeof createKitZustandStore> {
    const key = this.getKitKey(kitId);

    if (!this.kitZustandStores.has(key)) {
      const store = createKitZustandStore(this, kitId);
      this.kitZustandStores.set(key, store);
    }

    return this.kitZustandStores.get(key)!;
  }

  getDesignEditorZustandStore(id: string): ReturnType<typeof createDesignEditorZustandStore> {
    if (!this.designEditorZustandStores.has(id)) {
      const store = createDesignEditorZustandStore(this, id);
      this.designEditorZustandStores.set(id, store);
    }

    return this.designEditorZustandStores.get(id)!;
  }

  getSketchpadZustandStore() {
    return this.sketchpadZustandStore!;
  }

  destroy() {
    this.kitZustandStores.clear();
    this.designEditorZustandStores.clear();
  }
}

const SketchpadContext = createContext<EnhancedSketchpadStore | null>(null);

// Scoping contexts
type SketchpadScope = { id: string };
type KitScope = { id: KitId };
type DesignScope = { id: DesignId };
type DesignEditorScope = { id: string };

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
const KitScopeContext = createContext<KitScope | null>(null);
const DesignScopeContext = createContext<DesignScope | null>(null);
const DesignEditorScopeContext = createContext<DesignEditorScope | null>(null);

export const SketchpadScopeProvider = (props: { id?: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || "" }), [props.id]);
  return React.createElement(SketchpadScopeContext.Provider, { value }, props.children as any);
};

export const KitScopeProvider = (props: { id?: KitId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { name: "", version: "" } }), [props.id]);
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};

export const DesignScopeProvider = (props: { id?: DesignId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { name: "", variant: "", view: "" } }), [props.id]);
  return React.createElement(DesignScopeContext.Provider, { value }, props.children as any);
};

export const DesignEditorScopeProvider = (props: { id?: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || "" }), [props.id]);
  return React.createElement(DesignEditorScopeContext.Provider, { value }, props.children as any);
};

export const useSketchpadScope = () => useContext(SketchpadScopeContext);
export const useKitScope = () => useContext(KitScopeContext);
export const useDesignScope = () => useContext(DesignScopeContext);
export const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

export function SketchpadProvider(props: { id?: string; children: React.ReactNode }) {
  const store = useMemo(() => new EnhancedSketchpadStore(props.id), [props.id]);
  return React.createElement(SketchpadContext.Provider, { value: store }, props.children as any);
}

export function useSketchpadStore<T = EnhancedSketchpadStore>(selector?: (store: EnhancedSketchpadStore) => T): T {
  const ctx = useContext(SketchpadContext);
  const sScope = useSketchpadScope();
  if (!ctx) throw new Error("useSketchpadStore must be used within a SketchpadProvider");
  return selector ? selector(ctx) : (ctx as T);
}

export function useSketchpad(): ReturnType<typeof createSketchpadZustandStore> {
  const store = useSketchpadStore();
  return store.getSketchpadZustandStore();
}

export function useKits(): Map<string, string[]> {
  const store = useSketchpadStore();
  return store.getSketchpadZustandStore()((state) => state.kits);
}

export function useKit(id?: KitId): Kit {
  const store = useSketchpadStore();
  const kitScope = useKitScope();

  const kitId = id ?? kitScope?.id;
  if (!kitId) throw new Error("useKit requires a kit id or must be used within a KitScope");

  return store.getKitZustandStore(kitId)(
    (state) =>
      state.kit || {
        name: "",
        description: "",
        qualities: [],
        types: [],
        designs: [],
      },
  );
}

export function useDesignEditor(id?: string): DesignEditorState {
  const store = useSketchpadStore();
  const designEditorScope = useDesignEditorScope();
  const designEditorId = id ?? designEditorScope?.id;
  if (!designEditorId) throw new Error("useDesignEditor requires an id or must be used within a DesignEditorScope");

  return store.getDesignEditorZustandStore(designEditorId)((state) => ({
    designId: state.designId || { name: "", variant: "", view: "" },
    fullscreenPanel: state.fullscreenPanel,
    selection: state.selection,
    designDiff: state.designDiff,
    isTransactionActive: state.isTransactionActive,
    operationStack: [],
    operationIndex: 0,
    presence: state.presence,
    others: state.others,
  }));
}
