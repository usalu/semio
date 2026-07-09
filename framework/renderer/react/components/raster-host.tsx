import type { CommandDescriptor, UiComponentSceneNode } from "../os-shell.tsx";

//#region RasterHost
export function RasterHost({ node, onCommand }: { readonly node: UiComponentSceneNode; readonly onCommand: (command: CommandDescriptor) => void }) {
  const scene = node.raster;
  if (!scene) return <div className="semio-raster-empty">No raster scene</div>;
  const src = scene.pixelsBase64 ? `data:image/png;base64,${scene.pixelsBase64}` : undefined;
  return (
    <div className="semio-raster-host flex h-full min-h-0 w-full items-center justify-center overflow-auto bg-canvas p-2" data-surface-id={node.surfaceId}>
      {src ? (
        <img
          alt="Raster viewport"
          className="max-h-full max-w-full object-contain"
          height={scene.height}
          src={src}
          width={scene.width}
          onClick={() =>
            onCommand({
              controllerId: node.controllerId,
              command: "rasterClick",
              args: { surfaceId: node.surfaceId },
            })
          }
        />
      ) : (
        <div className="text-muted-foreground text-xs">
          {scene.width}×{scene.height} raster (no pixels)
        </div>
      )}
    </div>
  );
}
//#endregion RasterHost
