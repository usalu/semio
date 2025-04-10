import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"
import path from "path"

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@semio/js": path.resolve(__dirname, "./"),
    },
  },
})