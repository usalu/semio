// #region Header

// designAppIntegration.ts

// Hooks that integrate kit store with design app store
// Separate file to avoid circular dependency between store.tsx and apps/design/store

// #endregion

import { useMemo } from "react";
import { applyKitDiff, Design, DiffStatus, getClusterableGroups, Kit, Piece } from "../../semio";
import { useDesignAppDiff, useDesignAppHover, useDesignAppIsPieceTransitiveHovered, useDesignAppSelection, useDesignAppStore } from "./designAppHooks";
import { identitySelector, useConnectionScope, useDesignScope, useKit, usePiece, usePieceScope } from "./store";
import * as storeExports from "./store";

export function useIsPieceSelected(): boolean {
  const piece = usePieceScope();
  const selection = useDesignAppSelection();
  return selection.pieces?.includes(piece?.guid ?? "") ?? false;
}

export function useIsPieceHovered(): boolean {
  const hover = useDesignAppHover();
  const pieceScope = usePieceScope();
  if (!pieceScope || !hover) return false;
  return hover.pieces?.includes(pieceScope.guid) ?? false;
}

export function useIsPieceTransitiveHovered(): boolean {
  const pieceScope = usePieceScope();
  if (!pieceScope) return false;
  return useDesignAppIsPieceTransitiveHovered(undefined, pieceScope.guid);
}

export function usePieceStatus(): DiffStatus {
  const piece = usePieceScope();
  const designScope = useDesignScope();
  const designAppStore = useDesignAppStore(identitySelector) as any;

  if (!designAppStore || !piece || !designScope) {
    return DiffStatus.Unchanged;
  }

  const currentStack = designAppStore?.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) {
    return DiffStatus.Unchanged;
  }

  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.pieces?.added) {
          for (const addedPiece of designUpdate.diff.pieces.added) {
            if (addedPiece.guid === piece.guid) {
              return DiffStatus.Added;
            }
          }
        }
        if (designUpdate.diff.pieces?.removed) {
          for (const removedId of designUpdate.diff.pieces.removed) {
            if (removedId === piece.guid) {
              return DiffStatus.Removed;
            }
          }
        }
        if (designUpdate.diff.pieces?.updated) {
          for (const pieceUpdate of designUpdate.diff.pieces.updated) {
            if (pieceUpdate.id === piece.guid) {
              return DiffStatus.Modified;
            }
          }
        }
      }
    }
  }
  return DiffStatus.Unchanged;
}

export function useDiffedPiece<T>(selector?: (piece: Piece) => T, id?: string, deep: boolean = false): T | Piece {
  const originalPiece = usePiece(identitySelector, id, deep) as Piece;
  const pieceScope = usePieceScope();
  const designScope = useDesignScope();
  const designAppStore = useDesignAppStore(identitySelector) as any;

  if (!designAppStore || !pieceScope || !designScope) {
    return selector ? selector(originalPiece) : originalPiece;
  }

  const currentStack = designAppStore?.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) {
    return selector ? selector(originalPiece) : originalPiece;
  }

  let diffedPiece = { ...originalPiece };
  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.pieces?.updated) {
          for (const pieceUpdate of designUpdate.diff.pieces.updated) {
            if (pieceUpdate.id === pieceScope.guid) {
              diffedPiece = { ...diffedPiece, ...pieceUpdate.diff };
            }
          }
        }
      }
    }
  }

  return selector ? selector(diffedPiece) : diffedPiece;
}

export function useIsConnectionSelected(): boolean {
  const connectionScope = useConnectionScope();
  const selection = useDesignAppSelection();
  if (!connectionScope) return false;
  return selection.connections?.some((guid) => guid === connectionScope.guid) ?? false;
}

export function useIsConnectionHovered(): boolean {
  const hover = useDesignAppHover();
  const connectionScope = useConnectionScope();
  if (!connectionScope || !hover) return false;
  return hover.connections?.includes(connectionScope.guid) ?? false;
}

export function useConnectionStatus(): DiffStatus {
  const connection = useConnectionScope();
  const kitDiff = useDesignAppDiff();
  const designScope = useDesignScope();

  if (!connection || !designScope || !kitDiff?.designs?.updated) {
    return DiffStatus.Unchanged;
  }

  for (const designUpdate of kitDiff.designs.updated) {
    if (designUpdate.diff.connections?.added) {
      for (const conn of designUpdate.diff.connections.added) {
        if (conn.guid === connection.guid) {
          return DiffStatus.Added;
        }
      }
    }
    if (designUpdate.diff.connections?.removed) {
      for (const removedConn of designUpdate.diff.connections.removed) {
        if (typeof removedConn === "string" && removedConn === connection.guid) {
          return DiffStatus.Removed;
        }
      }
    }
    if (designUpdate.diff.connections?.updated) {
      for (const connUpdate of designUpdate.diff.connections.updated) {
        if (typeof connUpdate.id === "string" && connUpdate.id === connection.guid) {
          return DiffStatus.Modified;
        }
      }
    }
  }

  return DiffStatus.Unchanged;
}

export function useClusterableGroups() {
  const design = storeExports.useDesign() as Design;
  const selection = useDesignAppSelection();
  return useMemo(() => {
    if (!design) return [];
    return getClusterableGroups(design, selection.pieces ?? []);
  }, [design, selection.pieces]);
}

export function useDiffedKit(): Kit {
  const kit = useKit() as Kit;
  const diff = useDesignAppDiff();
  return diff ? applyKitDiff(kit, diff) : kit;
}
