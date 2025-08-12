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
import { FC, ReactNode, createContext, useContext, useEffect, useMemo, useRef, useState } from "react";
import DesignEditor, { DesignEditorAction, DesignEditorDispatcher, DesignEditorState as UIDesignEditorState, reduceSelection } from "./DesignEditor";

import {
  Connection,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  Kit,
  Layout,
  Mode,
  Piece,
  Plane,
  Theme,
  addConnectionToDesignDiff,
  addConnectionsToDesignDiff,
  addPieceToDesignDiff,
  addPiecesToDesignDiff,
  removeConnectionFromDesignDiff,
  removeConnectionsFromDesignDiff,
  removePieceFromDesignDiff,
  removePiecesFromDesignDiff,
  setConnectionInDesignDiff,
  setConnectionsInDesignDiff,
  setPieceInDesignDiff,
  setPiecesInDesignDiff,
} from "@semio/js";
import { orientDesign } from "../../semio";
import { DesignEditorFullscreenPanel, KitScopeProvider, SketchpadScopeProvider, DesignEditorState as StoreDesignEditorState, SketchpadProvider as StoreProvider, useDesigns, useKit, useKits } from "../../store";

// Helper
const keyOf = (d: DesignId) => `${d.name}::${d.variant || ""}::${d.view || ""}`;

interface SketchpadContextType {
  mode: Mode;
  layout: Layout;
  setLayout: (layout: Layout) => void;
  theme: Theme;
  setTheme: (theme: Theme) => void;
  navbarToolbar: ReactNode | null;
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
  const { kit, designEditorState } = useSketchpad();
  if (!kit || !designEditorState) return null;
  return <DesignEditor />;
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

  const [isImporting, setIsImporting] = useState<boolean>(true);
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

  if (isImporting) return null;

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

  // Read kit and designs (strict: must be inside KitScope)
  const kits = useKits();
  const firstKitEntry = (() => {
    const it = kits.entries().next();
    return it && !it.done ? it.value : ["Metabolism", [""]];
  })();
  const kitName = firstKitEntry[0] as string;
  const kitVersion = ((firstKitEntry[1] as string[]) || [""])[0] || "";
  const defaultKit = useKit();
  const designs = useDesigns();

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
    const pushUndoRedo = (undo: DesignDiff, forward: DesignDiff) => editor.pushOperation(undo, forward, editor.getState().selection);
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
        const kit = store.getKit(kitId);
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
      // Clipboard
      case DesignEditorAction.CopyToClipboard: {
        const s = editor.getState();
        const kit = store.getKit(kitId);
        const design = (kit.designs || []).find((d) => d.name === designId.name && (d.variant || "") === (designId.variant || "") && (d.view || "") === (designId.view || ""));
        if (!design) break;
        const subDesign: Design = {
          ...design,
          pieces: (design.pieces || []).filter((p) => s.selection.selectedPieceIds.includes(p.id_)),
          connections: (design.connections || []).filter((c) => s.selection.selectedConnections.some((sc) => sc.connectedPieceId === c.connected.piece.id_ && sc.connectingPieceId === c.connecting.piece.id_)),
        } as any;
        const { plane, center } = (action.payload || {}) as { plane?: Plane; center?: DiagramPoint };
        navigator.clipboard?.writeText(JSON.stringify(orientDesign(subDesign, plane, center))).then(() => {});
        break;
      }
      case DesignEditorAction.PasteFromClipboard: {
        (async () => {
          try {
            const text = await navigator.clipboard?.readText();
            if (!text) return;
            const pasted = JSON.parse(text) as Design;
            const pieces = pasted.pieces || [];
            const connections = pasted.connections || [];
            const empty: DesignDiff = { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any;
            const forward = addConnectionsToDesignDiff(addPiecesToDesignDiff(empty, pieces), connections);
            const undo = editor.invertDiff(forward);
            store.transact(() => {
              pieces.forEach((p) => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
              connections.forEach((c) => store.createConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", c));
            });
            pushUndoRedo(undo, forward);
          } catch (e) {}
        })();
        break;
      }
      case DesignEditorAction.CutToClipboard: {
        const s = editor.getState();
        const kit = store.getKit(kitId);
        const design = (kit.designs || []).find((d) => d.name === designId.name && (d.variant || "") === (designId.variant || "") && (d.view || "") === (designId.view || ""));
        if (design) {
          const subDesign: Design = {
            ...design,
            pieces: (design.pieces || []).filter((p) => s.selection.selectedPieceIds.includes(p.id_)),
            connections: (design.connections || []).filter((c) => s.selection.selectedConnections.some((sc) => sc.connectedPieceId === c.connected.piece.id_ && sc.connectingPieceId === c.connecting.piece.id_)),
          } as any;
          const { plane, center } = (action.payload || {}) as { plane?: Plane; center?: DiagramPoint };
          navigator.clipboard?.writeText(JSON.stringify(orientDesign(subDesign, plane, center))).then(() => {});
        }
        const pieceIds = s.selection.selectedPieceIds.map((id_) => ({ id_ }));
        const connectionIds = s.selection.selectedConnections.map((c) => ({ connected: { piece: { id_: c.connectedPieceId } }, connecting: { piece: { id_: c.connectingPieceId } } }));
        const forward = removeConnectionsFromDesignDiff(removePiecesFromDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, pieceIds), connectionIds);
        const undo = editor.invertDiff(forward);
        store.transact(() => {
          pieceIds.forEach(({ id_ }) => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", id_));
          connectionIds.forEach((cid: any) => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", cid.connected.piece.id_, cid.connecting.piece.id_));
        });
        pushUndoRedo(undo, forward);
        editor.updateDesignEditorSelection({ selectedPieceIds: [], selectedConnections: [] });
        break;
      }
      // Transactions
      case DesignEditorAction.StartTransaction:
        transactionQueue.current = [];
        setEphemeral((s) => ({ ...s, isTransactionActive: true, designDiff: { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } }) as any);
        break;
      case DesignEditorAction.FinalizeTransaction: {
        const ops = transactionQueue.current || [];
        transactionQueue.current = null;
        const before = editor.getState();
        const forward = before.designDiff as any;
        const undo = editor.invertDiff(forward);
        store.transact(() => ops.forEach((fn) => fn()));
        const hasChanges = !!(forward.pieces?.added?.length || forward.pieces?.removed?.length || forward.pieces?.updated?.length || forward.connections?.added?.length || forward.connections?.removed?.length || forward.connections?.updated?.length);
        if (hasChanges) pushUndoRedo(undo, forward);
        setEphemeral((s) => ({ ...s, isTransactionActive: false, designDiff: { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } }) as any);
        break;
      }
      case DesignEditorAction.AbortTransaction:
        transactionQueue.current = null;
        setEphemeral((s) => ({ ...s, isTransactionActive: false, designDiff: { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } }) as any);
        break;
      // Design changes
      case DesignEditorAction.SetDesign: {
        const d = action.payload as Design;
        commit(() => store.updateDesign(kitId.name, kitId.version || "", d));
        break;
      }
      case DesignEditorAction.AddPiece: {
        const p = action.payload as Piece;
        if (!editor.getState().isTransactionActive) {
          const forward = addPieceToDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, p);
          const undo = editor.invertDiff(forward);
          commit(() => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
          pushUndoRedo(undo, forward);
        } else commit(() => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
        break;
      }
      case DesignEditorAction.SetPiece: {
        const p = action.payload as Piece;
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: setPieceInDesignDiff(editor.getState().designDiff as any, p as any) as any } as any);
        else {
          const forward = setPieceInDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, p as any);
          const undo = editor.invertDiff(forward);
          commit(() => store.updatePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p));
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.RemovePiece: {
        const p = action.payload as Piece | string;
        const pid = typeof p === "string" ? p : p.id_;
        if (!editor.getState().isTransactionActive) {
          const forward = removePieceFromDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, { id_: pid } as any);
          const undo = editor.invertDiff(forward);
          commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
          pushUndoRedo(undo, forward);
        } else commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
        break;
      }
      case DesignEditorAction.AddPieces:
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: addPiecesToDesignDiff(editor.getState().designDiff as any, action.payload as Piece[]) as any } as any);
        else {
          const forward = addPiecesToDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as Piece[]);
          const undo = editor.invertDiff(forward);
          (action.payload as Piece[]).forEach((p: Piece) => commit(() => store.createPiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p)));
          pushUndoRedo(undo, forward);
        }
        break;
      case DesignEditorAction.SetPieces:
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: setPiecesInDesignDiff(editor.getState().designDiff as any, action.payload as any) as any } as any);
        else {
          const forward = setPiecesInDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as any);
          const undo = editor.invertDiff(forward);
          (action.payload as Piece[]).forEach((p: Piece) => commit(() => store.updatePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", p)));
          pushUndoRedo(undo, forward);
        }
        break;
      case DesignEditorAction.RemovePieces: {
        (action.payload as (Piece | string)[]).forEach((p: any) => {
          const pid = typeof p === "string" ? p : p.id_;
          commit(() => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", pid));
        });
        if (editor.getState().isTransactionActive)
          editor.setState({
            ...editor.getState(),
            designDiff: removePiecesFromDesignDiff(
              editor.getState().designDiff as any,
              (action.payload as any[]).map((pp) => ({ id_: typeof pp === "string" ? pp : pp.id_ })),
            ) as any,
          } as any);
        else {
          const forward = removePiecesFromDesignDiff(
            { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any,
            (action.payload as any[]).map((pp) => ({ id_: typeof pp === "string" ? pp : pp.id_ })),
          );
          const undo = editor.invertDiff(forward);
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.AddConnection: {
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: addConnectionToDesignDiff(editor.getState().designDiff as any, action.payload as Connection) as any } as any);
        else {
          const forward = addConnectionToDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as Connection);
          const undo = editor.invertDiff(forward);
          commit(() => store.createConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", action.payload as Connection));
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.SetConnection: {
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: setConnectionInDesignDiff(editor.getState().designDiff as any, action.payload as any) as any } as any);
        else {
          const forward = setConnectionInDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as any);
          const undo = editor.invertDiff(forward);
          commit(() => store.updateConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", action.payload as Connection));
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.RemoveConnection: {
        const c = action.payload as any;
        const connectedId = c.connected?.piece?.id_ || c.connectedPieceId || c;
        const connectingId = c.connecting?.piece?.id_ || c.connectingPieceId || c;
        commit(() => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", connectedId, connectingId));
        if (editor.getState().isTransactionActive)
          editor.setState({
            ...editor.getState(),
            designDiff: removeConnectionFromDesignDiff(editor.getState().designDiff as any, { connected: { piece: { id_: connectedId } }, connecting: { piece: { id_: connectingId } } } as any) as any,
          } as any);
        else {
          const forward = removeConnectionFromDesignDiff(
            { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any,
            { connected: { piece: { id_: connectedId } }, connecting: { piece: { id_: connectingId } } } as any,
          );
          const undo = editor.invertDiff(forward);
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.AddConnections: {
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: addConnectionsToDesignDiff(editor.getState().designDiff as any, action.payload as Connection[]) as any } as any);
        else {
          const forward = addConnectionsToDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as Connection[]);
          const undo = editor.invertDiff(forward);
          (action.payload as Connection[]).forEach((c) => commit(() => store.createConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", c)));
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.SetConnections: {
        if (editor.getState().isTransactionActive) editor.setState({ ...editor.getState(), designDiff: setConnectionsInDesignDiff(editor.getState().designDiff as any, action.payload as any) as any } as any);
        else {
          const forward = setConnectionsInDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, action.payload as any);
          const undo = editor.invertDiff(forward);
          (action.payload as Connection[]).forEach((c) => commit(() => store.updateConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", c)));
          pushUndoRedo(undo, forward);
        }
        break;
      }
      case DesignEditorAction.RemoveConnections: {
        (action.payload as any[]).forEach((c) => {
          const connectedId = c.connected?.piece?.id_ || c.connectedPieceId || c;
          const connectingId = c.connecting?.piece?.id_ || c.connectingPieceId || c;
          commit(() => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", connectedId, connectingId));
        });
        if (editor.getState().isTransactionActive)
          editor.setState({
            ...editor.getState(),
            designDiff: removeConnectionsFromDesignDiff(
              editor.getState().designDiff as any,
              (action.payload as any[]).map((c) => ({ connected: { piece: { id_: c.connected?.piece?.id_ || c.connectedPieceId || c } }, connecting: { piece: { id_: c.connecting?.piece?.id_ || c.connectingPieceId || c } } }) as any),
            ) as any,
          } as any);
        else {
          const forward = removeConnectionsFromDesignDiff(
            { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any,
            (action.payload as any[]).map((c) => ({ connected: { piece: { id_: c.connected?.piece?.id_ || c.connectedPieceId || c } }, connecting: { piece: { id_: c.connecting?.piece?.id_ || c.connectingPieceId || c } } }) as any) as any,
          ) as any;
          const undo = editor.invertDiff(forward);
          pushUndoRedo(undo, forward);
        }
        break;
      }
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
        if (editor.getState().isTransactionActive) {
          let d = editor.getState().designDiff as any;
          d = removePiecesFromDesignDiff(d, pieceIds.map((p) => ({ id_: typeof p === "string" ? p : p.id_ })) as any) as any;
          d = removeConnectionsFromDesignDiff(
            d,
            connectionIds.map((c) => ({ connected: { piece: { id_: c.connected?.piece?.id_ || c.connectedPieceId || c } }, connecting: { piece: { id_: c.connecting?.piece?.id_ || c.connectingPieceId || c } } }) as any) as any,
          ) as any;
          editor.setState({ ...editor.getState(), designDiff: d } as any);
        } else {
          let d: DesignDiff = { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any;
          d = removePiecesFromDesignDiff(d, pieceIds.map((p) => ({ id_: typeof p === "string" ? p : p.id_ })) as any) as any;
          d = removeConnectionsFromDesignDiff(
            d,
            connectionIds.map((c) => ({ connected: { piece: { id_: c.connected?.piece?.id_ || c.connectedPieceId || c } }, connecting: { piece: { id_: c.connecting?.piece?.id_ || c.connectingPieceId || c } } }) as any) as any,
          ) as any;
          const undo = editor.invertDiff(d);
          pushUndoRedo(undo, d);
        }
        break;
      }
      case DesignEditorAction.DeleteSelected: {
        const s = editor.getState();
        const pieceIds = s.selection.selectedPieceIds.map((id_) => ({ id_ }));
        const connectionIds = s.selection.selectedConnections.map((c) => ({ connected: { piece: { id_: c.connectedPieceId } }, connecting: { piece: { id_: c.connectingPieceId } } }));
        const forward = removeConnectionsFromDesignDiff(removePiecesFromDesignDiff({ pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any, pieceIds), connectionIds);
        const undo = editor.invertDiff(forward);
        store.transact(() => {
          pieceIds.forEach(({ id_ }) => store.deletePiece(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", id_));
          connectionIds.forEach((cid: any) => store.deleteConnection(kitId.name, kitId.version || "", designId.name, designId.variant || "", designId.view || "", cid.connected.piece.id_, cid.connecting.piece.id_));
        });
        pushUndoRedo(undo, forward);
        editor.updateDesignEditorSelection({ selectedPieceIds: [], selectedConnections: [] });
        break;
      }
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
  const externalState: UIDesignEditorState | null = designEditorState ? { ...(designEditorState as any), kit: kit as Kit, fileUrls } : null;

  return (
    <TooltipProvider>
      <SketchpadScopeProvider>
        <KitScopeProvider id={{ name: kitName, version: kitVersion }}>
          <SketchpadContext.Provider
            value={{
              mode: mode,
              layout: currentLayout,
              setLayout: setCurrentLayout,
              theme: currentTheme,
              setTheme: setCurrentTheme,
              navbarToolbar: navbarToolbar,
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
        </KitScopeProvider>
      </SketchpadScopeProvider>
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
