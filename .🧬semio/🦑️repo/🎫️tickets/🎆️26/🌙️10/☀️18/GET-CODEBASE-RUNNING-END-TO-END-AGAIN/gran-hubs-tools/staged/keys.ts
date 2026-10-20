/** 🔎️ Third-party oracle for the manifest plans: Bun's TOML parser lists, for each (before, after) text pair, the dependency tables with their entries. stdin: JSON [[before, after], ...]; stdout: JSON [[beforeTables, afterTables] | null, ...]. */
const pairs: [string, string][] = JSON.parse(await Bun.stdin.text());
const tab = (t: string) => {
  const d: any = Bun.TOML.parse(t);
  const out: Record<string, Record<string, any>> = {};
  for (const k of ["dependencies", "dev-dependencies", "build-dependencies"]) out[k] = d[k] ?? {};
  for (const [cfg, v] of Object.entries((d.target ?? {}) as Record<string, any>)) for (const k of ["dependencies", "dev-dependencies", "build-dependencies"]) if (v[k]) out[`target.${cfg}.${k}`] = v[k];
  return out;
};
console.log(JSON.stringify(pairs.map(([a, b]) => { try { return [tab(a), tab(b)]; } catch { return null; } })));
