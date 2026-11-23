import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
const root = resolve(ui, "🎯️targets/⚛️react/📦️packages/🟦️typescript");
const el = (p: string) => resolve(ui, "🧱️elements", p);

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: /^@semio-tech\/ui-react\/introduction$/u, replacement: resolve(root, "👋️introduction.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(root, "🟦️.tsx") },
    ],
  },
  test: {
    root,
    name: "elements-lane",
    environment: "jsdom",
    include: [
      el("🌈️Surface/🧪️tests/frame-press.test.tsx"),
      el("🃏️CardStack/🟦️.test.tsx"),
      el("🃏️CardStack/bands.test.ts"),
      el("🃏️CardStack/chip-row.test.ts"),
      el("🖱️ContextMenu/🧪️tests/🧩️component/placement.tsx"),
      el("🖱️ContextMenu/🧪️tests/🧩️component/🟦️.tsx"),
      el("🃏️OverviewCard/🧪️tests/🧩️component/🟦️.tsx"),
      el("🦴️Skeletons/🧪️tests/🧩️component/🟦️.tsx"),
      el("📑️Tabs/🧪️tests/🧩️component/boundary.tsx"),
      el("📑️Tabs/🧪️tests/🧩️component/🟦️.tsx"),
      el("🪟️Window/🧪️tests/🧩️component/🟦️.tsx"),
      el("🗨️Popover/🧪️tests/🧩️component/placement.tsx"),
      el("🔽️Select/🧪️tests/🧩️component/boundary.tsx"),
      el("🎨️Canvas/🧪️tests/shared-border-dock.test.tsx"),
      el("🧭️PanelTabBar/shared-border.test.tsx"),
      el("🪢️SharedBorderRow/round3-framework.test.tsx"),
      el("🖼️Panel/🧪️tests/🧩️component/🟦️.tsx"),
      el("📐️Layout/🧪️tests/🧩️component/🟦️.tsx"),
    ],
    setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")],
  },
});
