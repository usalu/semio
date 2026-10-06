export interface IconShortcodeTables { emoji: Record<string, string>; themed: Record<string, string>; catalog: Record<string, string>; }
export interface IconShortcodeMatch { kind: "emoji" | "themed" | "catalog"; value: string; }

/** 🔎️ Resolves one trimmed code through the explicit emoji, themed and catalog precedence. */
export function resolveIconShortcode(code: string, tables: IconShortcodeTables): IconShortcodeMatch | null {
  const key = code.replace(/^\p{White_Space}+|\p{White_Space}+$/gu, "");
  if (!key) return null;
  for (const kind of ["emoji", "themed", "catalog"] as const) if (Object.prototype.hasOwnProperty.call(tables[kind], key)) return { kind, value: tables[kind][key] };
  return null;
}
