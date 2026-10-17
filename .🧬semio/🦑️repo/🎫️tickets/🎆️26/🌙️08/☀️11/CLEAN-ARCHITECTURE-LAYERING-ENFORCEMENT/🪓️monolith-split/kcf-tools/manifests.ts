/** 🧾 KCF manifest check: every given Cargo.toml parses (Bun.TOML), every `path = "..."` dependency resolves to a Cargo.toml, no dependency name appears twice in one table.
 * usage: bun manifests.ts <repo-relative Cargo.toml>... */
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
const R = '/Users/ueli/Documents/semio';
let bad = 0;
for (const rel of process.argv.slice(2)) {
  const file = `${R}/${rel}`;
  const text = readFileSync(file, 'utf8');
  let t: any;
  try { t = Bun.TOML.parse(text); } catch (e) { console.log('TOML FAIL', rel, String(e)); bad++; continue; }
  const tabs: [string, any][] = [['', t], ...Object.entries<any>(t.target ?? {}).map(([k, v]) => [`target.${k}.`, v] as [string, any])];
  let paths = 0;
  for (const [prefix, tab] of tabs) for (const k of ['dependencies', 'dev-dependencies', 'build-dependencies']) for (const [name, spec] of Object.entries<any>(tab[k] ?? {})) {
    if (typeof spec === 'object' && spec.path) { paths++; const target = resolve(dirname(file), spec.path); if (!existsSync(`${target}/Cargo.toml`)) { console.log('MISSING PATH', rel, `${prefix}${k}`, name, '->', spec.path); bad++; } }
  }
  const lines = text.split('\n'); let section = ''; const seen = new Set<string>();
  for (const l of lines) { const m = l.match(/^\[(.+)\]\s*$/); if (m) { section = m[1]; continue; } const d = l.match(/^([A-Za-z0-9_.-]+)\s*=/); if (d && /dependencies$/.test(section)) { const key = `${section}:${d[1]}`; if (seen.has(key)) { console.log('DUPLICATE', rel, key); bad++; } seen.add(key); } }
  console.log('ok', paths, 'path deps checked:', rel.split('/').slice(-5, -2).join('/'));
}
console.log('problems', bad);
