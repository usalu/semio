import { createRequire } from "node:module";

/** 📦️ Returns a `require` that loads from the declared parser package first; a checkout whose parser package is not installed yet falls back to the resolution of the requiring file, so the same code runs before and after the install. */
export function parserRequireV1(from) {
  const own = createRequire(new URL("./package.json", import.meta.url)), fallback = createRequire(from);
  const attempt = (action) => (name) => { try { return own[action](name); } catch (error) { if (error?.code !== "MODULE_NOT_FOUND") throw error; return fallback[action](name); } };
  const load = (name) => { try { return own(name); } catch (error) { if (error?.code !== "MODULE_NOT_FOUND") throw error; return fallback(name); } };
  return Object.assign(load, { resolve: attempt("resolve") });
}
