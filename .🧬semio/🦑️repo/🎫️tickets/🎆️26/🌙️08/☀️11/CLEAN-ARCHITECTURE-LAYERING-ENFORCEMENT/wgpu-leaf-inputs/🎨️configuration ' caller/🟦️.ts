import { fileURLToPath } from "node:url";
export default {
  cacheDir: fileURLToPath(new URL("../../🗑️generated/wgpu-leaf-vite/cache", import.meta.url)),
  plugins: [{
    name: "semio-owned-composition-probe",
    configureServer(server: { middlewares: { use: (path: string, handler: (_request: unknown, response: { setHeader: (key: string, value: string) => void; end: (body: string) => void }) => void) => void } }) {
      server.middlewares.use("/__semio_composition_probe", (_request, response) => {
        response.setHeader("content-type", "application/json");
        response.end(JSON.stringify({ variant: process.env.SEMIO_PLUGIN, renderer: process.env.SEMIO_RENDERER, mode: process.env.SEMIO_BUILD_MODE, port: process.env.S_OS_PORT }));
      });
    }
  }]
};
