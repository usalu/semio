import { useEffect, useMemo, useState } from "react";
import { IconShotFrame, iconRenderPort, type IconRenderRequest } from "@semio-tech/ui-react";
import type { ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region IconRenderHost
/** @emoji 🖼️ Renders an icon-render scene: offscreen GLB shot preview inside a shot frame, see https://threejs.org/docs/#examples/en/renderers/SVGRenderer. */
export function IconRenderHost({ node }: ComponentSceneHostProps) {
  const scene = node.iconRender;
  const requestJson = scene?.requestJson;
  const request = useMemo<IconRenderRequest | null>(() => {
    if (!requestJson) return null;
    try {
      return JSON.parse(requestJson) as IconRenderRequest;
    } catch {
      return null;
    }
  }, [requestJson]);
  const [preview, setPreview] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    setPreview(null);
    setError(null);
    if (!request) return;
    let cancelled = false;
    void iconRenderPort
      .render(request)
      .then((result) => {
        if (!cancelled) setPreview(result.dataUrl);
      })
      .catch((renderError: unknown) => {
        if (!cancelled) setError(renderError instanceof Error ? renderError.message : String(renderError));
      });
    return () => {
      cancelled = true;
    };
  }, [request]);
  if (!scene || !request) {
    return <div className="flex h-full items-center justify-center text-sm opacity-60">No shot</div>;
  }
  const content = error ? (
    <div className="flex h-full items-center justify-center p-4 text-sm text-destructive">{error}</div>
  ) : preview ? (
    <img alt={scene.footer ?? "Icon shot"} className="block h-full w-full" src={preview} />
  ) : (
    <div className="flex h-full items-center justify-center text-sm opacity-60">Rendering…</div>
  );
  return (
    <div className="semio-icon-render-host absolute inset-0 flex flex-col" data-surface-id={node.surfaceId}>
      <div className="relative min-h-0 flex-1">
        <IconShotFrame background={request.background} height={request.height} shape={request.shape ?? "rectangle"} width={request.width}>
          {content}
        </IconShotFrame>
      </div>
      {scene.footer ? <div className="shrink-0 px-3 pb-2 text-center text-xs opacity-60">{scene.footer}</div> : null}
    </div>
  );
}
//#endregion IconRenderHost
