// #region 🧲Header
// 💻 .storybook/stories/framework/os/os.stories.tsx
// Specs: Boot the real `FrameworkOsShell` filtered to one program from the generated registry — the composable "filters for starting apps" story mechanism.
// Summary: Args-driven `program` select (populated from `PROGRAM_BUILD_TARGETS`) + `appId` text control via `OsBootHost` (`.storybook/framework/os/index.tsx`). Serves prebuilt WASM from `/program-modules`; a missing artifact renders an instruction panel instead of blocking on cargo.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react-vite";

import { OsBootHost, PROGRAM_BUILD_TARGETS, type OsBootHostProps } from "../../../framework/os/index.tsx";

const meta = {
  title: "🛠️framework🖥️os/Shell",
  component: OsBootHost,
  parameters: { layout: "fullscreen" },
  tags: [], // heavy WASM boot — no autodocs page
  argTypes: {
    plugin: { control: "select", options: PROGRAM_BUILD_TARGETS.map((t) => t.programId) },
  },
} satisfies Meta<typeof OsBootHost>;

export default meta;
type Story = StoryObj<typeof meta>;

/** @emoji 🖥️ Studio host program — `PLUGIN_HOST_CONFIGS` (`framework/core/js/index.ts`) resolves `"s"` to studio mode. */
export const Studio: Story = {
  args: { plugin: "s" } satisfies OsBootHostProps,
};

/** @emoji 🧩 Puzzle program, no `appId` override — lands on its manifest's first app. */
export const Puzzle: Story = {
  args: { plugin: "puzzle" } satisfies OsBootHostProps,
};

/** @emoji 🚫 A registry programId with no prebuilt web artifact — exercises the artifact-missing panel deterministically offline (never triggers a cargo build). */
export const ArtifactMissing: Story = {
  args: { plugin: "architect" } satisfies OsBootHostProps,
};
