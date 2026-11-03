import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: [
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx",
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/🧩️component/🟦️.tsx",
    ],
    environment: "jsdom",
  },
});
