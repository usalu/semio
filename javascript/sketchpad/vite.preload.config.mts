// import { defineConfig } from 'vite'

// // https://vitejs.dev/config/
// export default defineConfig({
// })

import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

// https://vite.dev/config/
export default defineConfig({
    plugins: [react(
        { jsxRuntime: 'classic' }
    )],
})