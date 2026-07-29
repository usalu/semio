// #region 🧲Header
// 💻 .storybook/stories/framework/os/Wgpu.stories.tsx
// Specs: Boot the real `@semio-tech/framework-renderer-wgpu` raw-wgpu host for a registry program — the "no React DOM, canvas-only" renderer story.
// Summary: Args-driven `program` select via `WgpuBootHost` (`.storybook/framework/os/index.tsx`), which dynamically imports the wgpu package, resolves its Trunk-hashed bundle filename from the served `index.html`, and boots into a `#root` container. Falls back to a "WebGPU unavailable" message when `navigator.gpu` is undefined (e.g. headless Chromium without `--enable-unsafe-webgpu`, Safari, Firefox) and to an artifact-missing panel when the plugin has no prebuilt WASM — never triggers a cargo build.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react-vite";

import { WgpuBootHost, PLUGIN_BUILD_TARGETS, type WgpuBootHostProps } from "../../../framework/os/index.tsx";

const meta = {
  title: "🛠️framework🖥️os/Wgpu",
  component: WgpuBootHost,
  parameters: { layout: "fullscreen" },
  tags: [], // heavy WASM + WebGPU boot — no autodocs page
  argTypes: {
    plugin: { control: "select", options: PLUGIN_BUILD_TARGETS.map((t) => t.pluginId) },
  },
} satisfies Meta<typeof WgpuBootHost>;

export default meta;
type Story = StoryObj<typeof meta>;

/** @emoji 🖥️ Studio host program through the wgpu renderer instead of the React renderer. */
export const Studio: Story = {
  args: { plugin: "s" } satisfies WgpuBootHostProps,
};

/** @emoji 🧩 Puzzle program through the wgpu renderer. */
export const Puzzle: Story = {
  args: { plugin: "puzzle" } satisfies WgpuBootHostProps,
};

/** @emoji 🚫 A registry pluginId with no prebuilt web artifact — exercises the artifact-missing panel deterministically offline. */
export const ArtifactMissing: Story = {
  args: { plugin: "architect" } satisfies WgpuBootHostProps,
};
