/** 👥️ App presence is empty; canonical window views and interaction carry live state. */
export interface NoPresence {}
export function parseNoPresence(value: unknown): NoPresence {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== 0) throw new Error("app presence must be empty" );
  return {};
}
