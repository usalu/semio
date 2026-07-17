import { useMemo } from "react";
import { HistoryTable, useLabel, type HistoryColumn } from "@semio-tech/ui-react";
import type { ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region GraphTimelineHost
export function GraphTimelineHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.graphTimeline;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const columns = useMemo(() => {
    if (!scene) return [] as HistoryColumn[];
    try {
      return JSON.parse(scene.columnsJson) as HistoryColumn[];
    } catch {
      return [];
    }
  }, [scene]);

  if (!scene) return <div className="semio-graph-timeline-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-graph-timeline-host h-full min-h-0 w-full overflow-auto p-single" data-surface-id={node.surfaceId}>
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
//#endregion GraphTimelineHost
