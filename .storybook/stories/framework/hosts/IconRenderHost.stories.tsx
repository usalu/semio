// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/IconRenderHost.stories.tsx
// Specs: Host the framework renderer's `IconRenderHost` with zero WASM engine and zero dev-server asset route —
// the `framework/hosts` scope registers no static-dir for GLBs (unlike `framework/os`'s `/plugin-modules`), so
// `assetUrl` is a hand-built `data:model/gltf+json` fixture (`.storybook/framework/hosts/iconRenderFixture.ts`).
// Summary: The default `iconRenderPort` (`framework/ui/js/react/index.tsx`) still does the real three.js offscreen render —
// GLTFLoader + WebGLRenderer/SVGRenderer — against that fixture, so this exercises the real render pipeline, not
// a stub. `ToolbarFormat` reads `context.globals.iconRenderer` directly (webgl → `format: "png"`, svg → `format:
// "svg"`) per the toolbar's own description in `.storybook/preview.tsx` ("read directly from
// context.globals.iconRenderer, not applied by a decorator"); `SvgFixed`/`PngFixed` pin the format via args.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, type ReactElement } from "react";

import { IconRenderHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, UiComponentSceneNode } from "@semio-tech/framework-core";
import type { IconRenderRequest } from "@semio-tech/ui-react";
import { iconRenderPlaceholderAssetUrl } from "../../../framework/hosts/iconRenderFixture.ts";

//#region Fixtures
function buildStoryIconRenderRequest(format: IconRenderRequest["format"]): IconRenderRequest {
  return {
    assetUrl: iconRenderPlaceholderAssetUrl(),
    camera: { position: [1.6, -2.2, 1.4], target: [0.5, 0.5, 0], zoom: 1, fov: 42, up: [0, 0, 1] },
    lights: { ambientIntensity: 0.55, ambientColor: "#ffffff", sunAzimuth: 45, sunElevation: 55, sunIntensity: 1.3, sunColor: "#fff4e0" },
    width: 220,
    height: 220,
    format,
    shape: "rectangle",
    background: "#1b1e24",
    shadowEnabled: false,
    material: { color: "#8fa6c7", metalness: 0.15, roughness: 0.65 },
  };
}

const STORY_ICON_RENDER_CONTROLLER_ID = "icon-render-story";
//#endregion Fixtures

//#region StoryHost
function noopAction(_action: ActionDescriptor): void {
  // 🔇 IconRenderHost never dispatches — it's a read-only preview surface.
}

function IconRenderStoryHost({ format }: { readonly format: IconRenderRequest["format"] }): ReactElement {
  const node: UiComponentSceneNode = useMemo(
    () => ({
      type: "componentScene",
      surfaceId: "icon-render.story.preview",
      controllerId: STORY_ICON_RENDER_CONTROLLER_ID,
      componentKind: "icon-render",
      iconRender: { requestJson: JSON.stringify(buildStoryIconRenderRequest(format)), footer: `format: ${format}` },
    }),
    [format],
  );
  return (
    <div style={{ position: "relative", height: 320, width: 320 }}>
      <IconRenderHost node={node} onAction={noopAction} />
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/IconRenderHost",
  component: IconRenderStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof IconRenderStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎛️ Flip the "Icon Format" toolbar global to compare `iconRenderPort`'s WebGL-raster and SVGRenderer paths against the same fixture GLB. */
export const ToolbarFormat: Story = {
  render: (_args, context) => <IconRenderStoryHost format={context.globals.iconRenderer === "svg" ? "svg" : "png"} />,
};

/** 🖼️ Format pinned to `"svg"` (`SVGRenderer`), independent of the toolbar. */
export const SvgFixed: Story = {
  args: { format: "svg" },
};

/** 🖼️ Format pinned to `"png"` (`WebGLRenderer`), independent of the toolbar. */
export const PngFixed: Story = {
  args: { format: "png" },
};
