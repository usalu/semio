import factory from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts";
import { resolve } from "node:path";

/** 🔭️ Dev server for the round 3 element story sweep: the course build config rooted at the ticket's own page. */
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await (typeof factory === "function" ? factory(env) : factory);
  const strip = (plugins: unknown[]): unknown[] => plugins.flatMap(p => Array.isArray(p) ? [strip(p)] : p && /architecture-course-assignment-pdfs|html|course/i.test((p as { name?: string }).name ?? "") ? [] : [p]);
  return { ...config, root: resolve(import.meta.dirname, "site"), plugins: strip(config.plugins ?? []), server: { ...config.server, watch: null, fs: { strict: false } } };
};
