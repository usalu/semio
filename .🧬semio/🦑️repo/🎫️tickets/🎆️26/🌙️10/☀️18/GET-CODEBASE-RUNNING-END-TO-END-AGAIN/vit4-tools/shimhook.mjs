import { shimUrl } from "./shimcore.mjs";
export async function resolve(specifier, context, next) {
  try { return await next(specifier, context); } catch (e) {
    if (e && (e.code === "ERR_MODULE_NOT_FOUND" || e.code === "ERR_PACKAGE_PATH_NOT_EXPORTED")) { const u = shimUrl(specifier); if (u) return next(u, context); }
    throw e;
  }
}
