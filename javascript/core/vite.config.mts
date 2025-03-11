// import tailwindcss from '@tailwindcss/vite'
import react from "@vitejs/plugin-react"
// import mdx from '@mdx-js/rollup'
import { defineConfig } from "vite"
// import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react({ include: /\.(mdx|js|jsx|ts|tsx)$/ })],
  // plugins: [{ enforce: 'pre', ...mdx() },react({ include: /\.(mdx|js|jsx|ts|tsx)$/ }), tailwindcss()], https://github.com/remix-run/react-router/issues/12168
  // plugins: [react({ include: /\.(mdx|js|jsx|ts|tsx)$/ }), tailwindcss()],
  // resolve: {
  //   alias: {
  //     'semio': resolve(__dirname, 'core/semio.ts')
  //   }
  // }
})