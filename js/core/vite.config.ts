import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"
import path from "path"

export default defineConfig(async () => {
  // normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite")
  return {
    plugins: [
      tailwind.default(),
      react()
    ]
  }
})