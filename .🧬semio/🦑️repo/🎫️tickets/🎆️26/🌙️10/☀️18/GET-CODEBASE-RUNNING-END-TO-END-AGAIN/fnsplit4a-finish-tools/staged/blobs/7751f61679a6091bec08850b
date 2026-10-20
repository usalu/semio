import { WasmCanvas } from "@semio-tech/framework/canvas/react-renderer";
import { ContextMenuController } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { Canvas2dGumballOverlay } from "./🧭️gumball/🟦️.tsx";
import { CanvasPresenceOverlayV1 } from "../👕️canvas-presence/🟦️.tsx";
import { useSceneState } from "./🔶️scene/🟦️.tsx";
import { usePublishCatalogueDragState } from "./🔸️publish-catalogue-drag/🟦️.tsx";
import { useContextMenu } from "./🟠️context-menu/🟦️.tsx";

export function Canvas2dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const { windowInstanceId, containerRef, cameraRef, readLayers, dragOverStateRef, dispatch, mapContextMenu, shellContextMenuFallback, setContextMenu, setViewportSize, scene, emptySceneLabel, onPresencePointerMove, onPresencePointerLeave, sessionFactory, activeUtility, viewportSize, presenceRuntimeKey, presenceWindowId, localPresenceActor, presenceCamera, contextMenuTitleLabel, contextMenu } = useSceneState({ node, onAction });
  const { handleDragOver, handleDragLeave, handleDrop } = usePublishCatalogueDragState({ windowInstanceId, containerRef, cameraRef, readLayers, dragOverStateRef, dispatch });
  const { onContextMenu } = useContextMenu({ requestContextMenu, node, windowInstanceId, mapContextMenu, shellContextMenuFallback, setContextMenu, containerRef, setViewportSize });
  if (!scene) return <div className="semio-canvas-2d-empty">{emptySceneLabel}</div>;
  return (
    <div
      ref={containerRef}
      className="semio-canvas-2d-host relative h-full min-h-[24rem] w-full ui-surface"
      data-level="base"
      data-controller-id={node.controllerId}
      data-surface-id={node.surfaceId}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      onContextMenu={onContextMenu}
      onPointerMove={onPresencePointerMove}
      onPointerLeave={onPresencePointerLeave}
    >
      <WasmCanvas className="h-full w-full" sessionFactory={sessionFactory} />
      <Canvas2dGumballOverlay
        layersJson={scene.layersJson}
        activeUtility={activeUtility}
        camera={cameraRef.current}
        viewportWidth={viewportSize.width}
        viewportHeight={viewportSize.height}
        onDispatch={(action, args) => dispatch(action, args)}
      />
      <CanvasPresenceOverlayV1
        runtimeKey={presenceRuntimeKey}
        windowId={presenceWindowId}
        space="canvas"
        myActor={localPresenceActor ?? ""}
        locale={typeof document !== "undefined" ? document.documentElement.lang : undefined}
        localCanvas={presenceCamera}
        localSizePx={[viewportSize.width, viewportSize.height]}
        scenePath={`canvas/${presenceWindowId}`}
      />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
