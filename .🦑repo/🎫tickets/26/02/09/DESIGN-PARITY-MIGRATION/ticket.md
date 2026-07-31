# Ticket

## Todos

## Changes

### Patch 1 (Objective 1: Details Panel Parity)

Summary:

- Implemented Details panel handlers using transactions + update commands with design-piece parsing and fix action wiring.
- Documented Design Details Editing in `README.md` and `AGENTS.md`.

Files:

- `js/compose/sketchpad/Design.tsx`
- `README.md`
- `AGENTS.md`

Tests:

- Not run (not requested).

Patch:

```diff
diff --git a/AGENTS.md b/AGENTS.md
index cfac1531..68b4ee97 100644
--- a/AGENTS.md
+++ b/AGENTS.md
@@ -76,6 +76,16 @@ Engine startup MUST support a pure stdio MCP server mode.

 App hover and selection state MUST be managed by the Sketchpad state machine.

+### Design Details
+
+Design details edits MUST apply to all selected pieces within a single transaction scope.
+
+Design piece name, variant, and view edits MUST support legacy `name-variant-view` identifiers and explicit design references.
+
+Clustered design pieces MUST reject name, variant, and view edits.
+
+Fixing pieces MUST detach selected pieces from their parent connections.
+
 ### Diagrams

 Diagrams MUST integrate D3 force-directed simulations with React Flow.
@@ -416,6 +426,8 @@ Stats provide computed or measured performance data for entire designs using the
 - Design diagram primary-pointer selection interactions (click, Shift+Click, lasso) MUST remain independent from panning-state tracking.
 - Design scene selection MUST resolve selected piece identity from traversable object ancestry metadata (`pieceId`/`id`) so click and box-selection stay synchronized.
 - Design scene piece render wrappers MUST expose selection identity metadata on transform ancestors so loaded model meshes propagate deterministic selection identity.
+- Design details panel edits MUST apply to all selected pieces and surface mixed-selection state for differing values.
+- Design details panel MUST treat clustered design pieces as read-only for name, variant, and view edits.

 ### Ticket UX

@@ -2572,6 +2584,7 @@ Sketchpad UI elements resolve transactions via React context (not props):
 - `js/compose/sketchpad/elements.tsx` defines `TransactionProvider` and `useTransaction()`.
 - `js/compose/sketchpad/elements.tsx` `Geometry` treats `color` as the base (non-interactive) color and uses selection/hover theme colors for the rendered material/edges when `selected`/`hovered` are true.
 - `js/compose/sketchpad/Design.tsx` diagram piece nodes use non-inset rings (`ring-*`, not `ring-inset`) so rings remain visible on `Avatar` nodes with full-size `AvatarFallback` backgrounds.
+- `js/compose/sketchpad/Design.tsx` details panel piece edits route through `updatePiece`/`updatePieces` transactions, parse legacy design variant strings, and apply fix actions via `fixPiecesInDesign`.
 - Elements such as `Input`, `Textarea`, `Select`, `Slider`, `Stepper`, `Combobox`, and `ActionDropdown` call `useTransaction()` internally and do not accept a `transaction` prop.
 - Apps are responsible for scoping transactions by wrapping their UI subtree with `TransactionProvider` using the appropriate transaction hook (per-app or kit-level), so all descendant elements participate consistently.

diff --git a/README.md b/README.md
index 3b2a37eb..cffdc466 100644
--- a/README.md
+++ b/README.md
@@ -1171,6 +1171,14 @@ The CLI sends unified diffs or file snapshots; the server parses them, reindexes
 HTTP endpoints cover ticket lifecycle commands, diff ingestion, precommit checks, indexing, and read-only queries for warnings, breachs, and scopes.
 Webhook receivers enrich GitHub issue events, and Discord notifications format prompt/summary headings to match ticket workflow conventions.

+## 🧾 Design Details Editing [↑](#-bundles-)
+
+The Design Details panel treats every change as a scoped edit inside the Design app transaction system so undo/redo groups related field updates into a single action.
+Piece edits resolve from the UI selection into `updatePiece` and `updatePieces` commands, applying the same change to all selected pieces while honoring mixed-value display when values differ.
+Design-piece edits interpret legacy design identifiers stored as `name-variant-view` strings and rebuild them when the user changes name, variant, or view, while design references stored as explicit design GUIDs update by selecting the matching design entry.
+Clustered design pieces are read-only in the Details panel, so rename, variant, and view edits stop before they can mutate connected design aggregates.
+The Fix Pieces action converts connected pieces into fixed placement by removing parent connections through `fixPiecesInDesign`, keeping the piece's plane and center consistent after the change.
+
 ## 🧭 Kit Diagram Simulation Sync [↑](#-bundles-)

 The Kit diagram keeps D3 in charge of layout while React Flow handles rendering and interaction.
diff --git a/js/compose/sketchpad/Design.tsx b/js/compose/sketchpad/Design.tsx
index e36ed200..b8e9687a 100644
--- a/js/compose/sketchpad/Design.tsx
+++ b/js/compose/sketchpad/Design.tsx
@@ -101,6 +101,7 @@ import {
   findModel,
   findPieceInDesign,
   findTypeInKit,
+  fixPiecesInDesign,
   generateUniqueName,
   getIncludedDesigns,
   guid,
@@ -185,6 +186,7 @@ import {
   useExplodeableDesignNodes,
   useFlatPiecePlane,
   useFocusSafe,
+  useIncludedDesigns,
   useIsConnectionHovered,
   useIsInDesignScope,
   useIsPieceHovered,
@@ -3416,6 +3418,9 @@ const PiecesSectionForm: FC = () => {
   const [updatePieces] = useDesignAppUpdatePieces();
   const design = useDesign() as Design;
   const kit = useKit() as Kit;
+  const kitCommands = useKitCommands();
+  const includedDesigns = useIncludedDesigns();
+  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((includedDesign) => [includedDesign.guid, includedDesign])), [includedDesigns]);
   const metadata = new Map();
   const [selection] = useDesignAppSelection();
   const pieces = usePiecesFromIds(selection.pieces || []);
@@ -3423,9 +3428,15 @@ const PiecesSectionForm: FC = () => {
   const isSingle = pieces.length === 1;
   const piece = isSingle ? pieces[0] : null;

-  const isDesignPiece = isSingle ? typeof piece?.type === "string" && piece?.type === "design" : pieces.every((p) => typeof p.type === "string" && p.type === "design");
-  const hasDesignPieces = pieces.some((p) => typeof p.type === "string" && p.type === "design");
-  const hasMixedTypes = hasDesignPieces && pieces.some((p) => typeof p.type === "string" && p.type !== "design");
+  const isDesignPieceEntry = (target: any) => {
+    if (target?.design) return true;
+    if (typeof target?.type === "string") return target.type === "design";
+    return target?.type?.name === "design";
+  };
+
+  const isDesignPiece = isSingle ? isDesignPieceEntry(piece) : pieces.every((p) => isDesignPieceEntry(p));
+  const hasDesignPieces = pieces.some((p) => isDesignPieceEntry(p));
+  const hasMixedTypes = hasDesignPieces && pieces.some((p) => !isDesignPieceEntry(p));

   const getCommonValue = <T,>(getter: (piece: any) => T | undefined): T | undefined => {
     const values = pieces.map(getter).filter((v) => v !== undefined);
@@ -3435,29 +3446,261 @@ const PiecesSectionForm: FC = () => {
   };

   const getPieceId = (p: any): string => (p as any).guid || (p as any).id_;
+  const isRealPiece = (p: any): boolean => typeof (p as any).guid === "string";
+  const parseDesignVariant = (variant: string) => {
+    const [name, variantPart, viewPart] = variant.split("-");
+    return { name, variant: variantPart || undefined, view: viewPart || undefined };
+  };
+  const buildDesignVariant = (name: string, variant?: string, view?: string) => {
+    const parts = [name, variant, view].filter((part) => part && part.length > 0) as string[];
+    return parts.join("-");
+  };

   const handleTypeNameChange = (value: string) => {
-    // TODO: Implement using updatePiece/updatePieces commands
+    if (!value) return;
+    if (isDesignPiece) return;
+    const match = availableTypes.find((t) => t.name === value) || allReplacableTypes.find((t) => t.name === value);
+    if (!match) return;
+    if (isSingle && piece && isRealPiece(piece)) {
+      transaction?.start();
+      updatePiece?.(getPieceId(piece), { type: { guid: match.guid } });
+      transaction?.finalize();
+      return;
+    }
+    const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { type: { guid: match.guid } } }));
+    if (updates.length === 0) return;
+    transaction?.start();
+    updatePieces?.(updates);
+    transaction?.finalize();
   };

   const handleTypeVariantChange = (value: string) => {
-    // TODO: Implement using updatePiece/updatePieces commands
+    if (isDesignPiece) return;
+    const variantValue = value || undefined;
+    const resolveType = (name: string, variant?: string) => {
+      const candidates = allReplacableTypes.filter((t) => t.name === name);
+      if (variant !== undefined) {
+        const exact = candidates.find((t) => ((t as any).variant || "") === variant);
+        if (exact) return exact;
+      } else {
+        const base = candidates.find((t) => !((t as any).variant));
+        if (base) return base;
+      }
+      return candidates[0];
+    };
+
+    if (isSingle && piece && isRealPiece(piece)) {
+      const currentType = piece.type && typeof piece.type === "string" ? findTypeInKit(kit, piece.type) : piece.type?.guid ? findTypeInKit(kit, piece.type.guid) : null;
+      if (!currentType) return;
+      const match = resolveType(currentType.name, variantValue);
+      if (!match) return;
+      transaction?.start();
+      updatePiece?.(getPieceId(piece), { type: { guid: match.guid } });
+      transaction?.finalize();
+      return;
+    }
+
+    const updates = pieces
+      .filter(isRealPiece)
+      .map((p) => {
+        const currentType = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : p.type?.guid ? findTypeInKit(kit, p.type.guid) : null;
+        if (!currentType) return null;
+        const match = resolveType(currentType.name, variantValue);
+        if (!match) return null;
+        return { id: getPieceId(p), diff: { type: { guid: match.guid } } };
+      })
+      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);
+
+    if (updates.length === 0) return;
+    transaction?.start();
+    updatePieces?.(updates);
+    transaction?.finalize();
   };

   const handleDesignNameChange = (value: string) => {
-    // TODO: Implement using updatePiece/updatePieces commands
+    if (!isDesignPiece || !value) return;
+    const updateDesignGuid = (targetPiece: any, name: string) => {
+      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
+      const variant = (currentDesign as any)?.variant || undefined;
+      const view = (currentDesign as any)?.view || undefined;
+      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
+      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
+      return match?.guid;
+    };
+
+    if (isSingle && piece) {
+      const pieceId = getPieceId(piece);
+      const includedDesign = includedDesignMap.get(pieceId);
+      if (includedDesign?.type === "connected") {
+        console.warn("Connected design pieces cannot be renamed - they represent clustered designs");
+        return;
+      }
+      if (!isRealPiece(piece)) return;
+      if (piece.design?.guid) {
+        const matchGuid = updateDesignGuid(piece, value);
+        if (!matchGuid) return;
+        transaction?.start();
+        updatePiece?.(pieceId, { design: { guid: matchGuid } });
+        transaction?.finalize();
+        return;
+      }
+      const current = parseDesignVariant((piece as any).type?.variant || "");
+      const newVariant = buildDesignVariant(value, current.variant, current.view);
+      transaction?.start();
+      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
+      transaction?.finalize();
+      return;
+    }
+
+    const updates = pieces
+      .filter(isRealPiece)
+      .map((p) => {
+        if (p.design?.guid) {
+          const matchGuid = updateDesignGuid(p, value);
+          if (!matchGuid) return null;
+          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
+        }
+        if ((p as any).type?.name === "design") {
+          const current = parseDesignVariant((p as any).type?.variant || "");
+          const newVariant = buildDesignVariant(value, current.variant, current.view);
+          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
+        }
+        return null;
+      })
+      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);
+
+    if (updates.length === 0) return;
+    transaction?.start();
+    updatePieces?.(updates);
+    transaction?.finalize();
   };

   const handleDesignVariantChange = (value: string) => {
-    // TODO: Implement using updatePiece/updatePieces commands
+    if (!isDesignPiece) return;
+    const nextVariant = value || undefined;
+    const updateDesignGuid = (targetPiece: any, variant?: string) => {
+      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
+      const name = currentDesign?.name || "";
+      const view = (currentDesign as any)?.view || undefined;
+      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
+      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
+      return match?.guid;
+    };
+
+    if (isSingle && piece) {
+      const pieceId = getPieceId(piece);
+      const includedDesign = includedDesignMap.get(pieceId);
+      if (includedDesign?.type === "connected") {
+        console.warn("Connected design pieces cannot have their variants changed - they represent clustered designs");
+        return;
+      }
+      if (!isRealPiece(piece)) return;
+      if (piece.design?.guid) {
+        const matchGuid = updateDesignGuid(piece, nextVariant);
+        if (!matchGuid) return;
+        transaction?.start();
+        updatePiece?.(pieceId, { design: { guid: matchGuid } });
+        transaction?.finalize();
+        return;
+      }
+      const current = parseDesignVariant((piece as any).type?.variant || "");
+      const newVariant = buildDesignVariant(current.name, nextVariant, current.view);
+      transaction?.start();
+      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
+      transaction?.finalize();
+      return;
+    }
+
+    const updates = pieces
+      .filter(isRealPiece)
+      .map((p) => {
+        if (p.design?.guid) {
+          const matchGuid = updateDesignGuid(p, nextVariant);
+          if (!matchGuid) return null;
+          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
+        }
+        if ((p as any).type?.name === "design") {
+          const current = parseDesignVariant((p as any).type?.variant || "");
+          const newVariant = buildDesignVariant(current.name, nextVariant, current.view);
+          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
+        }
+        return null;
+      })
+      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);
+
+    if (updates.length === 0) return;
+    transaction?.start();
+    updatePieces?.(updates);
+    transaction?.finalize();
   };

   const handleDesignViewChange = (value: string) => {
-    // TODO: Implement using updatePiece/updatePieces commands
+    if (!isDesignPiece) return;
+    const nextView = value || undefined;
+    const updateDesignGuid = (targetPiece: any, view?: string) => {
+      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
+      const name = currentDesign?.name || "";
+      const variant = (currentDesign as any)?.variant || undefined;
+      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
+      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
+      return match?.guid;
+    };
+
+    if (isSingle && piece) {
+      const pieceId = getPieceId(piece);
+      const includedDesign = includedDesignMap.get(pieceId);
+      if (includedDesign?.type === "connected") {
+        console.warn("Connected design pieces cannot have views changed - they represent clustered designs");
+        return;
+      }
+      if (!isRealPiece(piece)) return;
+      if (piece.design?.guid) {
+        const matchGuid = updateDesignGuid(piece, nextView);
+        if (!matchGuid) return;
+        transaction?.start();
+        updatePiece?.(pieceId, { design: { guid: matchGuid } });
+        transaction?.finalize();
+        return;
+      }
+      const current = parseDesignVariant((piece as any).type?.variant || "");
+      const newVariant = buildDesignVariant(current.name, current.variant, nextView);
+      transaction?.start();
+      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
+      transaction?.finalize();
+      return;
+    }
+
+    const updates = pieces
+      .filter(isRealPiece)
+      .map((p) => {
+        if (p.design?.guid) {
+          const matchGuid = updateDesignGuid(p, nextView);
+          if (!matchGuid) return null;
+          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
+        }
+        if ((p as any).type?.name === "design") {
+          const current = parseDesignVariant((p as any).type?.variant || "");
+          const newVariant = buildDesignVariant(current.name, current.variant, nextView);
+          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
+        }
+        return null;
+      })
+      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);
+
+    if (updates.length === 0) return;
+    transaction?.start();
+    updatePieces?.(updates);
+    transaction?.finalize();
   };

   const fixPieces = async () => {
-    // TODO: Implement using execute command
+    if (!design || !kit) return;
+    const pieceGuids = pieces.filter(isRealPiece).map((p) => getPieceId(p));
+    if (pieceGuids.length === 0) return;
+    const diff = fixPiecesInDesign(kit, design.guid, pieceGuids);
+    transaction?.start();
+    kitCommands?.updateDesign(design.guid, diff);
+    transaction?.finalize();
   };

   const handleCenterXChange = (value: number) => {
@@ -3780,7 +4023,7 @@ const PiecesSectionForm: FC = () => {
           <TreeContent>
             <div className="flex flex-col gap-single">
               <p className="text-sm text-muted-foreground">{useLabel("compose.sketchpad.app.design.piece.connectedPieceInfo")}</p>
-              <Button id="compose.sketchpad.app.design.piece.fixPiece">
+              <Button id="compose.sketchpad.app.design.piece.fixPiece" onClick={fixPieces}>
                 <DisconnectIcon className="size-tiny" />
                 {useLabel("compose.sketchpad.app.design.piece.fixPiece")}
               </Button>
```

### Patch 2 (Objective 2 + 3: Cluster/Expand Commands + Diagram Mapping Parity)

Summary:

- Added cluster/expand commands with transactional selection updates and deduped expansion merges.
- Aligned clustered design identifiers to design GUIDs for stable node selection and edge routing.
- Fixed design-node connector index mapping and explode targeting; added clustering test coverage.
- Documented clustering/expansion requirements and codebase notes.

Files:

- `js/compose/sketchpad/Design.tsx`
- `js/compose/sketchpad/Sketchpad.tsx`
- `js/compose/compose.ts`
- `js/compose/compose.test.ts`
- `README.md`
- `AGENTS.md`

Tests:

- Not run (not requested).

Patch:

```diff
diff --git a/js/compose/sketchpad/Design.tsx b/js/compose/sketchpad/Design.tsx
index b8e9687a..5dd842b1 100644
--- a/js/compose/sketchpad/Design.tsx
+++ b/js/compose/sketchpad/Design.tsx
@@ -93,7 +93,9 @@ import {
   Connection,
   Connector,
   Coord,
+  createClusteredDesign,
   Design,
   DiffStatus,
+  expandDesignPieces,
   findConnectionsInDesign,
   findConnectorInType,
   findDesignInKit,
@@ -102,7 +104,8 @@ import {
   findPieceInDesign,
   findTypeInKit,
   fixPiecesInDesign,
   generateUniqueName,
+  getDesignDiff,
   getIncludedDesigns,
   guid,
   ICON_WIDTH,
@@ -111,6 +114,7 @@ import {
   Piece,
   Plane,
   planeToMatrix,
+  replaceClusterWithDesign,
   selectBestModel,
   TOLERANCE,
   toComposeRotation,
@@ -896,6 +902,118 @@ export const commands: Record<string, (context: DesignAppCommandContext, ...args
       },
     };
   },
+  "compose.designApp.clusterPieces": (context: DesignAppCommandContext, pieceGuids: Guid[]): DesignAppCommandResult => {
+    if (!pieceGuids || pieceGuids.length === 0) {
+      return {};
+    }
+    const designPieceGuids = new Set((context.design.pieces || []).map((piece) => piece.guid));
+    const validPieceGuids = pieceGuids.filter((guid) => designPieceGuids.has(guid));
+    if (validPieceGuids.length === 0) {
+      return {};
+    }
+    const existingNames = (context.kit.designs || []).map((d) => d.name);
+    const clusterName = generateUniqueName(`${context.design.name} Cluster`, existingNames);
+    const { clusteredDesign, externalConnections } = createClusteredDesign(context.design, validPieceGuids, clusterName);
+    const designDiff = replaceClusterWithDesign(context.design, validPieceGuids, clusteredDesign, externalConnections);
+    const currentSelection = context.designApp.selection || {};
+    const piecesRemoved = currentSelection.pieces || [];
+    const connectionsRemoved = currentSelection.connections || [];
+    return {
+      diff: {
+        selection: {
+          pieces: {
+            removed: piecesRemoved,
+            added: [clusteredDesign.guid],
+          },
+          connections: {
+            removed: connectionsRemoved,
+          },
+        },
+      },
+      kitDiff: {
+        designs: {
+          added: [clusteredDesign],
+          updated: [
+            {
+              design: { guid: context.design.guid },
+              diff: designDiff,
+            },
+          ],
+        },
+      },
+    };
+  },
+  "compose.designApp.expandDesign": (context: DesignAppCommandContext, designGuid: Guid): DesignAppCommandResult => {
+    if (!designGuid) {
+      return {};
+    }
+    const referencedDesign = (context.kit.designs || []).find((d) => d.guid === designGuid);
+    if (!referencedDesign) {
+      return {};
+    }
+
+    const expandedReferencedDesign = expandDesignPieces(referencedDesign, context.kit);
+    const existingPieceGuids = new Set((context.design.pieces || []).map((piece) => piece.guid));
+    const addedPieces = (expandedReferencedDesign.pieces || []).filter((piece) => !existingPieceGuids.has(piece.guid));
+    const existingConnections = context.design.connections || [];
+    const addedConnections = (expandedReferencedDesign.connections || []).filter((connection) => !existingConnections.some((existing) => areSameConnection(existing, connection)));
+
+    const updatedExternalConnections = (context.design.connections || []).map((connection) => {
+      if (connection.connected.designPiece?.guid === designGuid) {
+        return {
+          ...connection,
+          connected: {
+            ...connection.connected,
+            designPiece: undefined,
+          },
+        };
+      }
+      if (connection.connecting.designPiece?.guid === designGuid) {
+        return {
+          ...connection,
+          connecting: {
+            ...connection.connecting,
+            designPiece: undefined,
+          },
+        };
+      }
+      return connection;
+    });
+
+    const expandedDesign: Design = {
+      ...context.design,
+      pieces: [...(context.design.pieces || []), ...addedPieces],
+      connections: [...updatedExternalConnections, ...addedConnections],
+    };
+
+    const designDiff = getDesignDiff(context.design, expandedDesign);
+    const currentSelection = context.designApp.selection || {};
+    const piecesRemoved = currentSelection.pieces || [];
+    const connectionsRemoved = currentSelection.connections || [];
+
+    return {
+      diff: {
+        selection: {
+          pieces: {
+            removed: piecesRemoved,
+          },
+          connections: {
+            removed: connectionsRemoved,
+          },
+        },
+      },
+      kitDiff: {
+        designs: {
+          updated: [
+            {
+              design: { guid: context.design.guid },
+              diff: designDiff,
+            },
+          ],
+        },
+      },
+    };
+  },
 };
@@ -2043,6 +2161,26 @@ export function useDesignAppUpdateConnections(): ActionHookResult<[updates: { id
   return [action, !!store];
 }

+export function useDesignAppClusterPieces(): ActionHookResult<[pieceGuids: Guid[]]> {
+  const store = useDesignStore() as DesignStore | null;
+  const getOrigin = useOrigin();
+  const action = useMemo(() => {
+    if (!store) return undefined;
+    return (pieceGuids: Guid[]) => store.execute("compose.designApp.clusterPieces", getOrigin(), pieceGuids);
+  }, [store, getOrigin]);
+  return [action, !!store];
+}
+
+export function useDesignAppExpandDesign(): ActionHookResult<[designGuid: Guid]> {
+  const store = useDesignStore() as DesignStore | null;
+  const getOrigin = useOrigin();
+  const action = useMemo(() => {
+    if (!store) return undefined;
+    return (designGuid: Guid) => store.execute("compose.designApp.expandDesign", getOrigin(), designGuid);
+  }, [store, getOrigin]);
+  return [action, !!store];
+}
+
@@ -4270,8 +4649,9 @@ const ExpandMenu: FC<ExpandMenuProps> = ({ nodes, edges, onExpand }) => {
       {explodeableDesignNodes.map((node) => {
         const boundingBox = getBoundingBoxForNode(node);
         const piece = node.data.piece as Piece;
-        const type = piece.type ? findTypeInKit(kit, piece.type.guid) : null;
-        const designName = type?.name ?? "";
+        const designGuid = piece.type?.guid;
+        const design = designGuid ? findDesignInKit(kit, designGuid) : null;
+        const designName = design?.name ?? "";

         return (
@@ -4286,7 +4666,7 @@ const ExpandMenu: FC<ExpandMenuProps> = ({ nodes, edges, onExpand }) => {
           >
             <div className="absolute inset-0 border-2 border-dashed border-accent/50 rounded-md" style={{ pointerEvents: "none" }} />
             <div className="absolute -top-10 -right-2 pointer-events-auto">
-              <Button id="compose.sketchpad.app.design.diagram.expandMenu.expand" className="px-3 py-single text-sm" onClick={() => onExpand(designName)}>
+              <Button id="compose.sketchpad.app.design.diagram.expandMenu.expand" className="px-3 py-single text-sm" onClick={() => designGuid && onExpand(designGuid)}>
                 Expand
               </Button>
             </div>
@@ -5153,40 +5533,42 @@ const connectionToEdge = (

   if (ComposeConnection.connecting.designPiece && allConnections) {
     const designPieceId = ComposeConnection.connecting.designPiece;
+    const designPieceGuid = designPieceId.guid;
     sourcePieceId = designPieceId;

     const externalConnections = allConnections.filter((conn) => {
-      const connectedToDesign = conn.connected.designPiece === ComposeConnection.connecting.designPiece;
-      const connectingToDesign = conn.connecting.designPiece === ComposeConnection.connecting.designPiece;
+      const connectedToDesign = conn.connected.designPiece?.guid === designPieceGuid;
+      const connectingToDesign = conn.connecting.designPiece?.guid === designPieceGuid;
       return connectedToDesign || connectingToDesign;
     });

     const connectorIndex = externalConnections.findIndex(
       (conn) =>
-        conn.connected.piece === ComposeConnection.connected.piece &&
-        conn.connecting.piece === ComposeConnection.connecting.piece &&
-        conn.connected.connector === ComposeConnection.connected.connector &&
-        conn.connecting.connector === ComposeConnection.connecting.connector,
+        conn.connected.piece.guid === ComposeConnection.connected.piece.guid &&
+        conn.connecting.piece.guid === ComposeConnection.connecting.piece.guid &&
+        conn.connected.connector?.guid === ComposeConnection.connected.connector?.guid &&
+        conn.connecting.connector?.guid === ComposeConnection.connecting.connector?.guid,
     );
     sourcePortId = connectorIndex >= 0 ? { guid: `connector-${connectorIndex}` } : { guid: "connector-0" };
   }

   if (ComposeConnection.connected.designPiece && allConnections) {
     const designPieceId = ComposeConnection.connected.designPiece;
+    const designPieceGuid = designPieceId.guid;
     targetPieceId = designPieceId;

     const externalConnections = allConnections.filter((conn) => {
-      const connectedToDesign = conn.connected.designPiece === ComposeConnection.connected.designPiece;
-      const connectingToDesign = conn.connecting.designPiece === ComposeConnection.connected.designPiece;
+      const connectedToDesign = conn.connected.designPiece?.guid === designPieceGuid;
+      const connectingToDesign = conn.connecting.designPiece?.guid === designPieceGuid;
       return connectedToDesign || connectingToDesign;
     });

     const connectorIndex = externalConnections.findIndex(
       (conn) =>
-        conn.connected.piece === ComposeConnection.connected.piece &&
-        conn.connecting.piece === ComposeConnection.connecting.piece &&
-        conn.connected.connector === ComposeConnection.connected.connector &&
-        conn.connecting.connector === ComposeConnection.connecting.connector,
+        conn.connected.piece.guid === ComposeConnection.connected.piece.guid &&
+        conn.connecting.piece.guid === ComposeConnection.connecting.piece.guid &&
+        conn.connected.connector?.guid === ComposeConnection.connected.connector?.guid &&
+        conn.connecting.connector?.guid === ComposeConnection.connecting.connector?.guid,
     );
     targetConnectorId = connectorIndex >= 0 ? { guid: `connector-${connectorIndex}` } : { guid: "connector-0" };
   }
@@ -5370,6 +5752,8 @@ const DesignDiagram: FC<DesignDiagramProps> = ({ reactFlowInstanceRef }) => {
   const [addConnection] = useDesignAppAddConnection();
   const [addConnections] = useDesignAppAddConnections();
   const [updateConnections] = useDesignAppUpdateConnections();
+  const [clusterPieces] = useDesignAppClusterPieces();
+  const [expandDesign] = useDesignAppExpandDesign();

@@ -5832,13 +6216,25 @@ const DesignDiagram: FC<DesignDiagramProps> = ({ reactFlowInstanceRef }) => {
     [toggleDiagramFullscreen],
   );

-  const onCluster = useCallback((clusterPieceIds: string[]) => {
-    // TODO: Implement cluster command
-  }, []);
+  const onCluster = useCallback(
+    (clusterPieceIds: string[]) => {
+      if (!clusterPieces || clusterPieceIds.length === 0) return;
+      transaction?.start();
+      clusterPieces(clusterPieceIds);
+      transaction?.finalize();
+    },
+    [clusterPieces, transaction],
+  );

-  const onExpand = useCallback((target: string) => {
-    // TODO: Implement explode command
-  }, []);
+  const onExpand = useCallback(
+    (target: string) => {
+      if (!expandDesign || !target) return;
+      transaction?.start();
+      expandDesign(target);
+      transaction?.finalize();
+    },
+    [expandDesign, transaction],
+  );

diff --git a/js/compose/sketchpad/Sketchpad.tsx b/js/compose/sketchpad/Sketchpad.tsx
index a310f2fe..30ce66d3 100644
--- a/js/compose/sketchpad/Sketchpad.tsx
+++ b/js/compose/sketchpad/Sketchpad.tsx
@@ -5511,11 +5511,11 @@ export function useExplodeableDesignNodes(nodes: any[], selection: any) {
   return useMemo(() => {
     return nodes.filter((node) => {
       if (node.type !== "design") return false;
-      const Guid = node.data.piece.id_;
+      const Guid = node.data.piece.guid;
       if (!selection.pieces?.includes(Guid)) return false;
-      const designName = (node.data.piece as any).type?.variant;
-      if (!designName) return false;
-      if (!kitDesigns?.find((d: any) => d.name === designName)) return false;
+      const designGuid = node.data.piece.type?.guid;
+      if (!designGuid) return false;
+      if (!kitDesigns?.find((d: any) => d.guid === designGuid)) return false;
       return true;
     });
   }, [nodes, selection.pieces, kitDesigns]);
diff --git a/js/compose/compose.ts b/js/compose/compose.ts
index 02fb4926..1641f505 100644
--- a/js/compose/compose.ts
+++ b/js/compose/compose.ts
@@ -3692,7 +3692,7 @@ export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds
         ...connection,
         connected: {
           ...connection.connected,
-          designPiece: { guid: connection.connected.piece.guid }, // Reference to the piece within nested design
+          designPiece: { guid: clusteredDesign.guid },
         },
       };
     } else if (connectingInCluster) {
@@ -3700,7 +3700,7 @@ export const replaceClusterWithDesign = (originalDesign: Design, clusterPieceIds
         ...connection,
         connecting: {
           ...connection.connecting,
-          designPiece: { guid: connection.connecting.piece.guid }, // Reference to the piece within nested design
+          designPiece: { guid: clusteredDesign.guid },
         },
       };
     }
@@ -3758,7 +3758,8 @@ export const getClusterableGroups = (design: Design, selectedPieceIds: string[])
     }
   }

-  const hasDesignNodes = selectedPieceIds.some((id) => id.startsWith("design-"));
+  const pieceGuidSet = new Set((design.pieces || []).map((piece) => piece.guid));
+  const hasDesignNodes = selectedPieceIds.some((id) => !pieceGuidSet.has(id));
   const hasMultipleComponents = connectedGroups.length > 1;
   const hasLargeConnectedGroup = connectedGroups.some((group) => group.length > 1);

@@ -3861,7 +3862,7 @@ export const getIncludedDesigns = (design: Design): IncludedDesignInfo[] => {
       }) ?? [];

     includedDesigns.push({
-      guid: guid(),
+      guid: designIdString,
       designGuid: designIdString,
       type: "connected",
       externalConnections,
diff --git a/js/compose/compose.test.ts b/js/compose/compose.test.ts
index 7ab6d3cd..914c955d 100644
--- a/js/compose/compose.test.ts
+++ b/js/compose/compose.test.ts
@@ -27,15 +27,18 @@ import {
   areKitDiffsEqual,
   areKitsEqual,
   areValidationResultsEqual,
+  createClusteredDesign,
   deserializeKit,
   exportKit,
   flattenDesign,
+  getIncludedDesigns,
   getKitDiff,
   hasErrors,
   importKit,
   inverseKitDiff,
   Kit,
   Plane,
+  replaceClusterWithDesign,
   serializeKit,
   toValidationResult,
   validateKit,
@@ -213,6 +216,47 @@ describe("Validation", () => {
   });
 });

+describe("Cluster", () => {
+  it("Cluster replacement uses design-guid designPiece and yields included design entry", () => {
+    const design = {
+      guid: "design-root",
+      name: "Root",
+      pieces: [
+        { guid: "piece-a", type: { guid: "type-a" } },
+        { guid: "piece-b", type: { guid: "type-b" } },
+        { guid: "piece-c", type: { guid: "type-c" } },
+      ],
+      connections: [
+        {
+          guid: "conn-ab",
+          connecting: { piece: { guid: "piece-a" } },
+          connected: { piece: { guid: "piece-b" } },
+        },
+        {
+          guid: "conn-bc",
+          connecting: { piece: { guid: "piece-b" } },
+          connected: { piece: { guid: "piece-c" } },
+        },
+      ],
+      createdAt: "2025-01-01T00:00:00.000Z",
+      updatedAt: "2025-01-01T00:00:00.000Z",
+    } as Kit["designs"][number];
+
+    const { clusteredDesign, externalConnections } = createClusteredDesign(design, ["piece-a", "piece-b"], "Cluster");
+    const diff = replaceClusterWithDesign(design, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
+    const updatedDesign = applyDesignDiff(design, diff);
+
+    const clusterConnection = updatedDesign.connections?.find((c) => c.guid === "conn-bc");
+    expect(clusterConnection?.connecting.designPiece?.guid).toBe(clusteredDesign.guid);
+    expect(clusterConnection?.connected.designPiece?.guid).toBeUndefined();
+
+    const included = getIncludedDesigns(updatedDesign);
+    expect(included.length).toBe(1);
+    expect(included[0].guid).toBe(clusteredDesign.guid);
+    expect(included[0].designGuid).toBe(clusteredDesign.guid);
+  });
+});
+
diff --git a/README.md b/README.md
index cffdc466..fed52ed5 100644
--- a/README.md
+++ b/README.md
@@ -1179,6 +1179,12 @@ The Fix Pieces action converts connected pieces into fixed placement by removing

 ## 🧭 Design Diagram Clustering [↑](#-bundles-)

 Clustering turns a selected, connected piece group into a nested design, adds that design to the kit, and rewrites external connections to reference the new design through `designPiece` markers while preserving the original piece IDs for expansion.
 Expanding reverses clustering by reinserting the nested design's pieces and connections into the parent design and clearing `designPiece` markers from affected connections, while filtering out duplicate connections by semantic equality.
 Design diagram nodes for clustered designs use the nested design GUID as their node identity so selection, edge routing, and connection lookups remain stable across renders.
 Design-node connector handles are indexed by external connection order, keeping port IDs and edge endpoints deterministic for clustered designs.
diff --git a/AGENTS.md b/AGENTS.md
index 68b4ee97..de57011d 100644
--- a/AGENTS.md
+++ b/AGENTS.md
@@ -92,6 +92,9 @@ Connection preview and proximity-connect targeting MUST reuse the same snap-poin
 Port visuals MUST use deterministic color assignment by compatibility family across Kit, Type, and Design diagrams.
 Ports without explicit compatibility mappings MUST keep deterministic per-port identity colors.
 Design app diagram simulation MUST prioritize overlap prevention with minimal-force collision-only resolution, no global centering forces, and pinned existing nodes.
+Design clustering MUST create nested designs and rewrite external connections to reference the nested design via `designPiece` identifiers.
+Design expansion MUST reinsert nested design pieces and clear `designPiece` references while preventing duplicate connections.
+Design diagram edge routing for clustered designs MUST use deterministic external-connection ordering for synthetic port indices.
@@ -418,6 +421,8 @@ Stats provide computed or measured performance data for entire designs using the
 - Design details panel edits MUST apply to all selected pieces and surface mixed-selection state for differing values.
 - Design details panel MUST treat clustered design pieces as read-only for name, variant, and view edits.
+- Design diagram cluster actions MUST select the clustered design node after grouping.
+- Design diagram expand actions MUST clear selection after removing clustered nodes.
@@ -4415,7 +4420,12 @@ Use this document for code organization (order of appearance of regions, classe
 - `Kit.tsx` enables React Flow element selection/focus on the diagram canvas so diagram click/lasso selection emits `onSelectionChange` updates into `KIT.SET_SELECTION`.
 - `Kit.tsx`, `Design.tsx`, and `Type.tsx` consume a shared compatibility-family port color strategy for port avatars, handle markers, and connector scene visuals.
 - Compatibility-family grouping merges explicitly compatible ports; ports without compatibility edges keep their own deterministic identity color.
+- `Design.tsx` routes cluster/expand actions through design app commands and maps clustered-design edges using `designPiece` GUIDs with deterministic connector indices.

 ## js/compose/sketchpad/portColor.ts
@@
 - Exports deterministic port tone resolution keyed by compatibility-family grouping, connector-port guid extraction helpers, and compatibility-state classification for selected-versus-target port interactions.
+
+## js/compose/compose.ts
+
+- Cluster helpers create nested designs, rewrite external connections with `designPiece` GUID markers, and expose included-design metadata keyed by design GUIDs.
```

## Log

- Implemented Details panel handlers and fix action wiring in `js/compose/sketchpad/Design.tsx` with transaction-backed updates, design-piece parsing, and fixPieces flow.
- Updated docs for Design Details Editing in `README.md` and `AGENTS.md`.
- Implemented cluster/expand commands, design-piece edge mapping parity, and designPiece GUID alignment; added tests and documentation for clustering behavior.

## Summary

Restored Details panel visibility/content by default in Design app state and unified navbar panel toggles to a single definition-driven icon strip with correct hover/active placement.
