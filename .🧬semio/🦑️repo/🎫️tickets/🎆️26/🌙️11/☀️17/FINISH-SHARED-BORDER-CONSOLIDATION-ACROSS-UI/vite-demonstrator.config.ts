import factory from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts";

/** 🔭️ Demonstrator dev server for border review without lane activation: the build-command plugin set, serve-mode transport. */
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await (typeof factory === "function" ? factory({ command: "build", mode: env.mode }) : factory);
  const strip = (plugins: unknown[]): unknown[] => plugins.flatMap(p => Array.isArray(p) ? [strip(p)] : p && /browser-artifact|build-only/.test((p as { name?: string }).name ?? "") ? [] : [p]);
  return { ...config, plugins: strip(config.plugins ?? []), server: { ...config.server, port: 5393 } };
};
