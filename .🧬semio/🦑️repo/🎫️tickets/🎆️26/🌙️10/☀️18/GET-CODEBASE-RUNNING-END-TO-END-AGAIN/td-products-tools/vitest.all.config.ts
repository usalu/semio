import base from "./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts";
export default async (env: unknown) => {
  const official: any = typeof base === "function" ? await (base as (e: unknown) => unknown)(env) : base;
  const include = (process.env.TP_INCLUDE ?? "").split(",").filter(Boolean);
  return include.length ? { ...official, test: { ...official.test, include } } : official;
};
