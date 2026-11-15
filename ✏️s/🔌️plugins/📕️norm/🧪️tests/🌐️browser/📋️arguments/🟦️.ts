/** 🎛️ Requires one explicit hub, host, kind, locale and generated output directory; the shared journey runs the editor path. */
export function validateNormBrowserArgumentsV1(segments: readonly string[]): void {
  const allowed = new Set(["--hub", "--serve", "--serve-b", "--journey", "--locale", "--kinds", "--space", "--tag", "--out", "--admin-capability", "--users", "--max-create-to-mounted-ms", "--max-open-to-mounted-ms", "--mount-budget-ms"]), seen = new Set<string>();
  for (let index = 0; index < segments.length; index += 2) {
    const flag = segments[index], value = segments[index + 1];
    if (!flag || !allowed.has(flag) || seen.has(flag) || !value || value.startsWith("--")) throw Error("Invalid norm journey arguments");
    seen.add(flag);
  }
  for (const flag of ["--hub", "--serve", "--out", "--locale", "--kinds"]) if (!seen.has(flag)) throw Error("Missing " + flag);
  const value = (flag: string) => segments[segments.indexOf(flag) + 1]!;
  for (const flag of ["--hub", "--serve"]) { const url = new URL(value(flag)); if (!["http:", "https:"].includes(url.protocol) || url.username || url.password) throw Error("Invalid " + flag); }
  if (!["en", "de"].includes(value("--locale"))) throw Error("Invalid locale");
  if (!/^[a-zA-Z0-9._-]{1,40}$/u.test(value("--kinds"))) throw Error("Invalid kind");
  if (!value("--out").split(/[\\/]/u).includes("🗑️generated")) throw Error("Output must be inside a generated directory");
  if (seen.has("--journey") && value("--journey") !== "edit") throw Error("The norm journey requires the editor journey");
}
