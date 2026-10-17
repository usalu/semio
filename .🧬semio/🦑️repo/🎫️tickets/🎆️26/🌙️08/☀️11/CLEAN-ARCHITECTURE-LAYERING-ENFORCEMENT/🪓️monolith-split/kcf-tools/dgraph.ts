/** 🕸️ KCF manifest graph (Bun.TOML): package index over git-known Cargo.toml files; reachability through normal/build (and dev) dependencies.
 * usage: bun dgraph.ts reach <from> <to> | deps <pkg> | check <pkg>... (every dependency of each pkg resolves to a known package) */
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
const R = '/Users/ueli/Documents/semio';
const git = (a: string[]) => spawnSync('git', ['-c', 'core.quotepath=false', ...a], { cwd: R, encoding: 'utf8', maxBuffer: 1 << 28 }).stdout.split('\n');
const files = [...new Set([...git(['ls-files', '--', '*Cargo.toml']), ...git(['ls-files', '--others', '--exclude-standard', '--', '*Cargo.toml'])])].filter((f) => f && !f.startsWith('.'));
const IDX = new Map<string, string[]>();
const MAN = new Map<string, any>();
for (const f of files) {
  try {
    const t: any = Bun.TOML.parse(readFileSync(`${R}/${f}`, 'utf8'));
    const n = t.package?.name;
    if (n) { IDX.set(n, [...(IDX.get(n) ?? []), f]); MAN.set(f, t); }
  } catch {}
}
function deps(pkg: string, kinds: string[]): Set<string> {
  const out = new Set<string>();
  for (const f of IDX.get(pkg) ?? []) {
    const t = MAN.get(f);
    const tabs = [t, ...Object.values<any>(t.target ?? {})];
    for (const tab of tabs) for (const k of kinds) for (const [name, spec] of Object.entries<any>(tab[k] ?? {})) out.add(typeof spec === 'object' && spec.package ? spec.package : name);
  }
  return out;
}
function reach(src: string, dst: string, kinds: string[]): string[] | null {
  const parent = new Map<string, string | null>([[src, null]]);
  const stack = [src];
  while (stack.length) {
    let n = stack.pop()!;
    if (n === dst) { const p: string[] = []; let c: string | null = n; while (c) { p.push(c); c = parent.get(c) ?? null; } return p.reverse(); }
    for (const d of deps(n, kinds)) if (IDX.has(d) && !parent.has(d)) { parent.set(d, n); stack.push(d); }
  }
  return null;
}
const [c, a, b] = process.argv.slice(2);
if (c === 'reach') {
  console.log('normal+build ->', reach(a, b, ['dependencies', 'build-dependencies']));
  console.log('with dev     ->', reach(a, b, ['dependencies', 'build-dependencies', 'dev-dependencies']));
} else if (c === 'deps') console.log([...deps(a, ['dependencies'])].sort().join('\n'));
else if (c === 'where') console.log(IDX.get(a));
console.error('packages', IDX.size, 'duplicate names', [...IDX].filter(([, v]) => v.length > 1).length);
