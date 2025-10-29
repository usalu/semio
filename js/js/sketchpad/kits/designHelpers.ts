// #region Header

// designHelpers.ts

// Helper functions that depend on design app integration
// Separate file to avoid circular dependency

// #endregion

import { Design, DiffStatus, Piece, Type } from "../../semio";
import { useMemo } from "react";
import { useDiffedKit, useDiffedPiece, usePieceStatus } from "./designAppIntegration";
import { useDesignAppDiff } from "./designAppHooks";
import { colorPortsForTypes } from "../../semio";
import { useConnectionScope, useDesign, useDesignScope, usePiece } from "./store";
import { findDesignInKit } from "../../semio";

export function usePortColoredTypes(): Type[] {
  const diffedKit = useDiffedKit();
  const typesWithColoredPorts = useMemo(() => {
    if (!diffedKit.types) return [];
    const colorDiff = colorPortsForTypes(diffedKit.types);
    return colorDiff.updated?.map((u) => u.value) || [];
  }, [diffedKit.types]);
  return typesWithColoredPorts;
}

export function usePieceWithDiff(): { original: Piece; diffed: Piece | null; hasDiff: boolean } {
  const originalPiece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const status = usePieceStatus();

  const hasDiff = status !== DiffStatus.Unchanged;

  return {
    original: originalPiece,
    diffed: hasDiff ? diffedPiece : null,
    hasDiff,
  };
}

export function useConnectionColor(): { stroke: string; fill: string } {
  const connection = useConnectionScope();
  const kitDiff = useDesignAppDiff();
  const designScope = useDesignScope();

  let diffStatus = DiffStatus.Unchanged;
  if (connection && designScope && kitDiff?.designs?.updated) {
    for (const designUpdate of kitDiff.designs.updated) {
      if (designUpdate.diff.connections?.added) {
        for (const conn of designUpdate.diff.connections.added) {
          if (conn.guid === connection.guid) {
            diffStatus = DiffStatus.Added;
            break;
          }
        }
      }
      if (designUpdate.diff.connections?.removed) {
        for (const removedConn of designUpdate.diff.connections.removed) {
          if (typeof removedConn === "string" && removedConn === connection.guid) {
            diffStatus = DiffStatus.Removed;
            break;
          }
        }
      }
      if (designUpdate.diff.connections?.updated) {
        for (const connUpdate of designUpdate.diff.connections.updated) {
          if (typeof connUpdate.id === "string" && connUpdate.id === connection.guid) {
            diffStatus = DiffStatus.Modified;
            break;
          }
        }
      }
    }
  }

  const stroke = diffStatus === DiffStatus.Added ? "#00ff00" : diffStatus === DiffStatus.Removed ? "#ff0000" : diffStatus === DiffStatus.Modified ? "#ffff00" : "#ffffff";
  const fill = diffStatus === DiffStatus.Added ? "#00ff0033" : diffStatus === DiffStatus.Removed ? "#ff000033" : diffStatus === DiffStatus.Modified ? "#ffff0033" : "#ffffff33";

  return { stroke, fill };
}

export function useDiffedDesign(): Design {
  const kit = useDiffedKit();
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useDiffedDesign must be called within a DesignScopeProvider");
  return findDesignInKit(kit, designScope.guid);
}
