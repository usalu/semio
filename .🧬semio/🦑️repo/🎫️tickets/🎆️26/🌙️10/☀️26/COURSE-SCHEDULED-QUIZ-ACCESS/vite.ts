import path from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwind from "@tailwindcss/vite";
import { aliases, root, ticket } from "./aliases";

export default defineConfig({
  root: path.resolve(root, "🎓️teaching/🏛️architecture/📚️course"),
  cacheDir: path.resolve(ticket, "🗑️generated/vite"),
  publicDir: false,
  plugins: [react(), tailwind(), { name: "ticket-host", configureServer(server) { server.middlewares.use((request, _response, next) => { if (request.url?.split("?")[0] === "/") request.url = "/" + encodeURIComponent("🌐️.html") + (request.url.includes("?") ? request.url.slice(request.url.indexOf("?")) : ""); next(); }); } }],
  resolve: { alias: aliases, dedupe: ["react", "react-dom"] },
  server: { host: "127.0.0.1", port: 6067, strictPort: true, fs: { allow: [root] }, hmr: false, watch: null },
});
