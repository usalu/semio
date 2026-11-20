/** 🖨️ The catalog file's own JSON style (two-space indent, every list expanded, scalar lists and short lists of scalar lists inline inside `volumes`), so a rewritten asset differs from the original only where its content changed.
 * @see ./r3-person-cooking-roots.ts */
type Json = any;

const scalar = (v: Json): boolean => v === null || typeof v !== "object";
function inline(v: Json): boolean {
  if (!Array.isArray(v)) return false;
  return v.every(scalar) || (v.every((x: Json) => Array.isArray(x) && x.every(scalar)) && pyJson(v).length <= 160);
}
/** 🔢 A number as Python's `repr` prints it: exponent notation below 1e-4 with a two-digit exponent. */
function pyNum(x: number): string {
  if (x === 0 || Math.abs(x) >= 1e-4 || !Number.isFinite(x)) return JSON.stringify(x);
  const [m, e] = x.toExponential().split("e");
  const n = Number(e);
  return `${m}e${n < 0 ? "-" : "+"}${String(Math.abs(n)).padStart(2, "0")}`;
}
/** 🐍 `json.dumps(value, ensure_ascii=False)` of Python: ", " and ": " separators, floats as Python prints them. */
export function pyJson(v: Json): string {
  if (Array.isArray(v)) return `[${v.map(pyJson).join(", ")}]`;
  if (v !== null && typeof v === "object") return `{${Object.entries(v).map(([k, x]) => `${JSON.stringify(k)}: ${pyJson(x)}`).join(", ")}}`;
  return typeof v === "number" ? pyNum(v) : JSON.stringify(v);
}
export function dump(v: Json, depth = 0, compact = false): string {
  const pad = "  ".repeat(depth);
  const inner = "  ".repeat(depth + 1);
  if (v !== null && typeof v === "object" && !Array.isArray(v)) {
    const keys = Object.keys(v);
    if (keys.length === 0) return "{}";
    return `{\n${keys.map((k) => `${inner}${JSON.stringify(k)}: ${dump(v[k], depth + 1, compact || (depth === 1 && k === "volumes"))}`).join(",\n")}\n${pad}}`;
  }
  if (Array.isArray(v)) {
    if (v.length === 0) return "[]";
    if (compact && inline(v)) return pyJson(v);
    return `[\n${v.map((x) => `${inner}${dump(x, depth + 1, compact)}`).join(",\n")}\n${pad}]`;
  }
  return typeof v === "number" ? pyNum(v) : JSON.stringify(v);
}
