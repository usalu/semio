import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import mdx from '@mdx-js/rollup'
import { defineConfig } from "vite"

// https://vite.dev/config/
export default defineConfig({
  plugins: [{ enforce: 'pre', ...mdx() },
  react({ include: /\.(mdx|js|jsx|ts|tsx)$/ }), tailwindcss()],
})