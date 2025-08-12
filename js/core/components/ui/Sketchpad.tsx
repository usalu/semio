// #region Header

// Sketchpad.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import { TooltipProvider } from "@semio/js/components/ui/Tooltip";
import { createContext, FC, ReactNode, useContext, useEffect, useMemo, useRef, useState } from "react";
import DesignEditor, { DesignEditorDispatcher, DesignEditorState as UIDesignEditorState, DesignEditorAction, reduceSelection } from "./DesignEditor";

import { Connection, Design, DesignId, Kit, Piece } from "@semio/js";
import { SketchpadProvider as StoreProvider, useDesigns, useKit, useSketchpadStore, DesignEditorState as StoreDesignEditorState, DesignEditorFullscreenPanel } from "../../store";

// Helper
const keyOf = (d: DesignId) => `${d.name}::${d.variant || ""}::${d.view || ""}`;

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

interface SketchpadContextType {
  mode: Mode;
  layout: Layout;
  setLayout: (layout: Layout) => void;
  theme: Theme;
  setTheme: (theme: Theme) => void;
  setNavbarToolbar: (toolbar: ReactNode) => void;
  kit: Kit | null;
  designEditorState: UIDesignEditorState | null;
  designEditorDispatch: DesignEditorDispatcher | null;
  availableDesigns: DesignId[];
  activeDesignId: DesignId | null;
  setActiveDesignId: (id: DesignId) => void;
  clusterDesign: () => void;
  expandDesign: (id: DesignId) => void;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);

export const useSketchpad = () => {
  const context = useContext(SketchpadContext);
  if (!context) {
    throw new Error("useSketchpad must be used within a SketchpadProvider");
  }
  return context;
};

interface ViewProps {}

const View = () => {
  const { kit, designEditorState, designEditorDispatch, availableDesigns } = useSketchpad();

  const onDesignIdChange = (newDesignId: DesignId) => {
    // handled by context provider via setActiveDesignId
  };

  if (!kit || !designEditorState) return null;

  return (
    <DesignEditor
      designId={designEditorState.designId}
  fileUrls={designEditorState.fileUrls}
  externalState={designEditorState}
  externalDispatch={designEditorDispatch}
      onDesignIdChange={onDesignIdChange}
      availableDesigns={availableDesigns}
      onToolbarChange={() => {}}
    />
  );
};

interface SketchpadProps {
  mode?: Mode;
  theme?: Theme;
  layout?: Layout;
  readonly?: boolean;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
  userId: string;
}

const SketchpadInner: FC<SketchpadProps> = ({ mode = Mode.USER, theme, layout = Layout.NORMAL, onWindowEvents, userId }) => {
  const store = useSketchpadStore();
  const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);
  const [currentLayout, setCurrentLayout] = useState<Layout>(layout);
  const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
    if (theme) return theme;
    if (typeof window !== "undefined") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? Theme.DARK : Theme.LIGHT;
    }
    return Theme.LIGHT;
  });
  const [activeDesignId, setActiveDesignId] = useState<DesignId | null>(null);
  const editorStoreIds = useRef<Map<string, string>>(new Map());
  const [designEditorState, setDesignEditorState] = useState<StoreDesignEditorState | null>(null);
  const [isImporting, setIsImporting] = useState<boolean>(true);

  // Import default kit and files into store once
  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        await store.importKit("metabolism.zip", true);
      } catch (e) {
      } finally {
        if (mounted) setIsImporting(false);
      }
    })();
    return () => {
      mounted = false;
    };
  }, []);

  // Read kit and designs from store
  const defaultKit = useKit("Metabolism", "");
  const designs = useDesigns("Metabolism", "");

  // Set active design when designs are available
  useEffect(() => {
    if (!activeDesignId && designs && designs.length > 0) setActiveDesignId({ name: designs[0].name, variant: designs[0].variant, view: designs[0].view });
  }, [designs]);

  // Ensure a design editor store exists and subscribe to its state
  useEffect(() => {
    if (!activeDesignId) return;
    const key = keyOf(activeDesignId);
    let id = editorStoreIds.current.get(key);
    if (!id) {
      const kitName = defaultKit?.name || "Metabolism";
      const kitVersion = defaultKit?.version || "";
      id = store.createDesignEditorStore(kitName, kitVersion, activeDesignId.name, activeDesignId.variant || "", activeDesignId.view || "");
      editorStoreIds.current.set(key, id);
    }
    const editor = store.getDesignEditorStore(id);
    if (!editor) return;
  const unsubscribe = editor.subscribe(() => setDesignEditorState(editor.getState()));
  setDesignEditorState(editor.getState());
    return () => unsubscribe();
  }, [activeDesignId, defaultKit]);

  // Bridge dispatch to store operations
  const transactionQueue = useRef<(() => void)[] | null>(null);
  const designEditorDispatch: DesignEditorDispatcher = (action) => {
    if (!activeDesignId) return;
    const id = editorStoreIds.current.get(keyOf(activeDesignId));
    if (!id) return;
    const editor = store.getDesignEditorStore(id);
    if (!editor) return;
    const kitId = editor.getKitId();
    const designId = editor.getDesignId();
    const commit = (fn: () => void) => {
      if (transactionQueue.current) transactionQueue.current.push(fn);
      else store.transact(fn);
    };
  const setEphemeral = (updater: (s: StoreDesignEditorState) => StoreDesignEditorState) => editor.setState(updater(editor.getState()));
    switch (action.type) {
      // Selection-only
      case DesignEditorAction.SetSelection:
      case DesignEditorAction.SelectAll:
      case DesignEditorAction.DeselectAll:
      case DesignEditorAction.InvertSelection:
      case DesignEditorAction.InvertPiecesSelection:
      case DesignEditorAction.InvertConnectionsSelection:
      case DesignEditorAction.AddAllPiecesToSelection:
      case DesignEditorAction.RemoveAllPiecesFromSelection:
      case DesignEditorAction.AddAllConnectionsToSelection:
      case DesignEditorAction.RemoveAllConnectionsFromSelection:
      case DesignEditorAction.SelectPiece:
      case DesignEditorAction.AddPieceToSelection:
      case DesignEditorAction.RemovePieceFromSelection:
      case DesignEditorAction.SelectPieces:
      case DesignEditorAction.AddPiecesToSelection:
      case DesignEditorAction.RemovePiecesFromSelection:
      case DesignEditorAction.SelectConnection:
      case DesignEditorAction.AddConnectionToSelection:
      case DesignEditorAction.RemoveConnectionFromSelection:
      case DesignEditorAction.SelectConnections:
      case DesignEditorAction.AddConnectionsToSelection:
      case DesignEditorAction.RemoveConnectionsFromSelection:
      case DesignEditorAction.SelectPiecePort:
      case DesignEditorAction.DeselectPiecePort: {
        const kit = store.getKit(kitId.name, kitId.version || "");
        const design = (kit.designs || []).find((d) => d.name === designId.name && (d.variant || "") === (designId.variant || "") && (d.view || "") === (designId.view || ""))!;
        const nextSel = reduceSelection(editor.getState().selection, design, action);
        editor.updateDesignEditorSelection(nextSel);
        break;
      }
      // Ephemeral UI
      case DesignEditorAction.SetFullscreen:
      case DesignEditorAction.ToggleDiagramFullscreen:
      case DesignEditorAction.ToggleModelFullscreen:
      case DesignEditorAction.SetCursor:
      case DesignEditorAction.SetCamera:
      case DesignEditorAction.StepIn:
      case DesignEditorAction.StepOut:
      case DesignEditorAction.UpdatePresence: {
        setEphemeral((s) => {
          if (action.type === DesignEditorAction.SetFullscreen) return { ...s, fullscreenPanel: action.payload as DesignEditorFullscreenPanel } as StoreDesignEditorState;
          if (action.type === DesignEditorAction.ToggleDiagramFullscreen)
            return { ...s, fullscreenPanel: s.fullscreenPanel === DesignEditorFullscreenPanel.Diagram ? DesignEditorFullscreenPanel.None : DesignEditorFullscreenPanel.Diagram } as StoreDesignEditorState;
          if (action.type === DesignEditorAction.ToggleModelFullscreen)
            return { ...s, fullscreenPanel: s.fullscreenPanel === DesignEditorFullscreenPanel.Model ? DesignEditorFullscreenPanel.None : DesignEditorFullscreenPanel.Model } as StoreDesignEditorState;
          if (action.type === DesignEditorAction.SetCursor) return { ...s, cursor: action.payload } as any;
          if (action.type === DesignEditorAction.SetCamera) return { ...s, camera: action.payload } as any;
          if (action.type === DesignEditorAction.StepIn) return { ...s, others: [...(s.others || []), action.payload] } as any;
          if (action.type === DesignEditorAction.StepOut) return { ...s, others: (s.others || []).filter((p) => p.name !== action.payload.name) } as any;
          if (action.type === DesignEditorAction.UpdatePresence) return { ...s, others: (s.others || []).map((p) => (p.name === action.payload.name ? { ...p, ...action.payload } : p)) } as any;
          return s;
        });
        break;
      }
      // Undo/Redo
      case DesignEditorAction.Undo:
        editor.undo();
        break;
      case DesignEditorAction.Redo:
        editor.redo();
        break;
      // Transactions
      case DesignEditorAction.StartTransaction:
        transactionQueue.current = [];
        setEphemeral((s) => ({ ...s, isTransactionActive: true } as any));
        break;
      case DesignEditorAction.FinalizeTransaction: {
        const ops = transactionQueue.current || [];
        transactionQueue.current = null;
        store.transact(() => ops.forEach((fn) => fn()));
        setEphemeral((s) => ({ ...s, isTransactionActive: false } as any));
        break;
      }
      case DesignEditorAction.AbortTransaction:
        transactionQueue.current = null;
        setEphemeral((s) => ({ ...s, isTransactionActive: false } as any));
        break;
      // Design changes
      case DesignEditorAction.SetDesign: {
        const d = action.payload as Design;
        commit(() => store.updateDesign(kitId.name, kitId.version || "", d));
        break;
      }
      case DesignEditorAction.AddPiece: {
        const p = action.payload as Piece;
        commit(() => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
        break;
      }
      case DesignEditorAction.SetPiece: {
        const p = action.payload as Piece;
        commit(() => store.updatePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
        break;
      }
      case DesignEditorAction.RemovePiece: {
        const p = action.payload as Piece | string;
        const pid = typeof p === "string" ? p : p.id_;
        commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
        break;
      }
      case DesignEditorAction.AddPieces:
        (action.payload as Piece[]).forEach((p: Piece) => commit(() => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p)));
        break;
      case DesignEditorAction.SetPieces:
        (action.payload as Piece[]).forEach((p: Piece) => commit(() => store.updatePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p)));
        break;
      case DesignEditorAction.RemovePieces:
        (action.payload as (Piece | string)[]).forEach((p: any) => {
          const pid = typeof p === "string" ? p : p.id_;
          commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
        });
        break;
      case DesignEditorAction.AddConnection:
        commit(() => store.createConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", action.payload as Connection));
        break;
      case DesignEditorAction.SetConnection:
        commit(() => store.updateConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", action.payload as Connection));
        break;
      case DesignEditorAction.RemoveConnection: {
        const c = action.payload as any;
        const connectedId = c.connected?.piece?.id_ || c.connectedPieceId || c;
        const connectingId = c.connecting?.piece?.id_ || c.connectingPieceId || c;
        commit(() => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", connectedId, connectingId));
        break;
      }
      case DesignEditorAction.AddConnections:
        (action.payload as Connection[]).forEach((c) => commit(() => store.createConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", c)));
        break;
      case DesignEditorAction.SetConnections:
        (action.payload as Connection[]).forEach((c) => commit(() => store.updateConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", c)));
        break;
      case DesignEditorAction.RemoveConnections:
        (action.payload as any[]).forEach((c) => {
          const connectedId = c.connected?.piece?.id_ || c.connectedPieceId || c;
          const connectingId = c.connecting?.piece?.id_ || c.connectingPieceId || c;
          commit(() => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", connectedId, connectingId));
        });
        break;
      case DesignEditorAction.RemovePiecesAndConnections: {
        const { pieceIds, connectionIds } = action.payload as { pieceIds: (Piece | string)[]; connectionIds: any[] };
        pieceIds.forEach((p) => {
          const pid = typeof p === "string" ? p : p.id_;
          commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
        });
        connectionIds.forEach((c) => {
          const connectedId = c.connected?.piece?.id_ || c.connectedPieceId || c;
          const connectingId = c.connecting?.piece?.id_ || c.connectingPieceId || c;
          commit(() => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", connectedId, connectingId));
        });
        break;
      }
      case DesignEditorAction.DeleteSelected:
        editor.deleteSelectedPiecesAndConnections();
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Theme.DARK);
    if (currentTheme === Theme.DARK) {
      root.classList.add(Theme.DARK);
    }
  }, [currentTheme]);
  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(Layout.TOUCH);
    if (currentLayout === Layout.TOUCH) {
      root.classList.add(Layout.TOUCH);
    }
  }, [currentLayout]);

  const kit = defaultKit || null;
  const availableDesigns: DesignId[] = useMemo(() => (designs || []).map((d: Design) => ({ name: d.name, variant: d.variant, view: d.view })), [designs]);
  const fileUrls = store.getFileUrls();
  const externalState: UIDesignEditorState | null = designEditorState
    ? { ...(designEditorState as any), kit: kit as Kit, fileUrls, operationStack: [], operationIndex: 0 }
    : null;

  return (
    <TooltipProvider>
      <SketchpadContext.Provider
        value={{
          mode: mode,
          layout: currentLayout,
          setLayout: setCurrentLayout,
          theme: currentTheme,
          setTheme: setCurrentTheme,
          setNavbarToolbar: setNavbarToolbar,
          kit: kit,
          designEditorState: externalState,
          designEditorDispatch: designEditorDispatch,
          availableDesigns,
          activeDesignId,
          setActiveDesignId,
          clusterDesign: () => {},
          expandDesign: () => {},
        }}
      >
        <div key={`layout-${currentLayout}`} className="h-full w-full flex flex-col bg-background text-foreground">
          <View />
        </div>
      </SketchpadContext.Provider>
    </TooltipProvider>
  );
};

const Sketchpad: FC<SketchpadProps> = (props) => (
  <StoreProvider>
    <SketchpadInner {...props} />
  </StoreProvider>
);

export default Sketchpad;

// Export Sketchpad state management types for external use
export {};
