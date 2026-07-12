import { useMemo } from "react";
import { HistoryTable, type HistoryColumn } from "@semio-tech/ui-react";
import type { ActionDescriptor, UiComponentSceneNode } from "../os-shell.tsx";

//#region VcsHistoryHost
export function VcsHistoryHost({ node, onAction }: { readonly node: UiComponentSceneNode; readonly onAction: (action: ActionDescriptor) => void }) {
  const scene = node.vcsHistory;
  const columns = useMemo(() => {
    if (!scene) return [] as HistoryColumn[];
    try {
      return JSON.parse(scene.columnsJson) as HistoryColumn[];
    } catch {
      return [];
    }
  }, [scene]);

  if (!scene) return <div className="semio-vcs-history-empty">No history scene</div>;

  return (
    <div className="semio-vcs-history-host h-full min-h-0 w-full overflow-auto p-single" data-surface-id={node.surfaceId}>
      <HistoryTable
        columns={columns}
        onSelectCheckpoint={(checkpointId) =>
          onAction({
            controllerId: node.controllerId,
            action: "checkoutCheckpoint",
            args: { checkpointId },
          })
        }
      />
    </div>
  );
}
//#endregion VcsHistoryHost
