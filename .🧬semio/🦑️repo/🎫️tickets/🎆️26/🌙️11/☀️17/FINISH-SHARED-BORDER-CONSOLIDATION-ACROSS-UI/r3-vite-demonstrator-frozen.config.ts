import factory from "./vite-demonstrator.config.ts";

/** 🧊️ Demonstrator dev server with watcher and HMR off, so concurrent edits by others cannot reload the probed page. */
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await factory(env);
  return { ...config, server: { ...config.server, watch: null, hmr: false } };
};
