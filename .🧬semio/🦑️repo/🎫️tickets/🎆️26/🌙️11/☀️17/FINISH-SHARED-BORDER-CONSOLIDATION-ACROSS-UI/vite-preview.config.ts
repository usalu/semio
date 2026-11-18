import factory from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts";

/** 🔭️ Course dev server for border review without the assignment PDF transport. */
export default async (env: { command: "serve" | "build"; mode: string }) => {
  const config = await (typeof factory === "function" ? factory(env) : factory);
  const strip = (plugins: unknown[]): unknown[] => plugins.flatMap(p => Array.isArray(p) ? [strip(p)] : p && (p as { name?: string }).name === "architecture-course-assignment-pdfs" ? [] : [p]);
  return { ...config, plugins: strip(config.plugins ?? []) };
};
