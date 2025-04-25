import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig(async () => {
  // normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite")
  return {
    plugins: [
      tailwind.default(),
      react(),
      wasm(),
      topLevelAwait() // needed for older browsers to run wasm
    ]
  }
})