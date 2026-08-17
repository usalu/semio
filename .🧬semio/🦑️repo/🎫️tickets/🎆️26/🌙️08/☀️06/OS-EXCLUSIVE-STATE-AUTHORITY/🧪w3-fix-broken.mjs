import fs from "fs";

const fixes = [
  {
    file: "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx",
    from: `const surfaceChromeCleanup = ephemeralBox<(()>("s.plugins.animate.apps.present.renderer.react.component.tsx.surfaceChromeCleanup", > void) | null = null);`,
    to: `const surfaceChromeCleanup = ephemeralBox<(() => void) | null>("s.plugins.animate.apps.present.renderer.react.component.tsx.surfaceChromeCleanup", null);`,
  },
  {
    file: "✏️s/🔌️plugins/📐️cad/🔨️modules/