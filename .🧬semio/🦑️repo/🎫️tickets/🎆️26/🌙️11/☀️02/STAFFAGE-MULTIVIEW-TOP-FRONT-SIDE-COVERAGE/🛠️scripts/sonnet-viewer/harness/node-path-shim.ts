export const resolve = (...parts: string[]): string => parts.join("/");
export const dirname = (path: string): string => path.split("/").slice(0, -1).join("/");
export const join = (...parts: string[]): string => parts.join("/");
export const basename = (path: string): string => path.split("/").at(-1) ?? "";
export const sep = "/";
export default { resolve, dirname, join, basename, sep };
