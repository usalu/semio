import factory from "../../../../../../../♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts";

/** 🧊️ Presentation host with the file watcher and HMR off, so concurrent edits by other agents cannot reload the probed page. */
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await (typeof factory === "function" ? factory(env) : factory);
  return { ...config, server: { ...(config.server ?? {}), watch: null, hmr: false } };
};
