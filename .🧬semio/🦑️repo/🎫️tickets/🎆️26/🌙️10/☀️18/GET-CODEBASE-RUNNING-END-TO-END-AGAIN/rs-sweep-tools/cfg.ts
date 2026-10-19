export type Env = { test: boolean; wasm: boolean };
export type Tri = boolean | null;
export type CfgExpr =
  | { k: "atom"; key: string; val: string | null }
  | { k: "all" | "any"; xs: CfgExpr[] }
  | { k: "not"; x: CfgExpr };

/** 🧮 Tokenizes and parses the argument text of a `cfg(...)` predicate into a tree. */
export function parseCfg(text: string): CfgExpr | null {
  const toks: string[] = [];
  const re = /\s*("(?:[^"\\]|\\.)*"|[A-Za-z_][A-Za-z0-9_]*|[(),=])/gy;
  let m: RegExpExecArray | null;
  let last = 0;
  while ((m = re.exec(text))) {
    toks.push(m[1]);
    last = re.lastIndex;
  }
  if (text.slice(last).trim() !== "") return null;
  let i = 0;
  const pred = (): CfgExpr | null => {
    const id = toks[i++];
    if (id === undefined) return null;
    if (toks[i] === "(") {
      i++;
      const xs: CfgExpr[] = [];
      while (toks[i] !== ")") {
        if (i >= toks.length) return null;
        const x = pred();
        if (!x) return null;
        xs.push(x);
        if (toks[i] === ",") i++;
      }
      i++;
      if (id === "not") return xs.length === 1 ? { k: "not", x: xs[0] } : null;
      if (id === "all" || id === "any") return { k: id, xs };
      return null;
    }
    if (toks[i] === "=") {
      i++;
      const v = toks[i++];
      return { k: "atom", key: id, val: v === undefined ? null : v.replace(/^"|"$/g, "") };
    }
    return { k: "atom", key: id, val: null };
  };
  const out = pred();
  return out && i === toks.length ? out : null;
}

const HOST: Record<string, string> = {
  target_os: "macos",
  target_arch: "aarch64",
  target_family: "unix",
  target_pointer_width: "64",
  target_endian: "little",
  target_vendor: "apple",
  target_env: "",
  panic: "unwind",
};
const WASM: Record<string, string> = {
  target_os: "unknown",
  target_arch: "wasm32",
  target_family: "wasm",
  target_pointer_width: "32",
  target_endian: "little",
  target_vendor: "unknown",
  target_env: "",
  panic: "unwind",
};

/** 🧭 Evaluates one atom under an environment; `null` means undecidable (features, custom cfgs). */
export function atom(key: string, val: string | null, env: Env, features: Map<string, boolean> | null): Tri {
  if (key === "test") return env.test;
  if (key === "feature") return features && val !== null && features.has(val) ? features.get(val)! : null;
  if (key === "unix") return !env.wasm;
  if (key === "windows") return false;
  if (key === "debug_assertions") return null;
  if (key === "doc" || key === "docsrs" || key === "miri" || key === "loom" || key === "fuzzing" || key === "doctest") return false;
  if (key === "target_has_atomic" || key === "target_feature" || key === "target_thread_local") return null;
  const table = env.wasm ? WASM : HOST;
  if (val !== null && key in table) return table[key] === val;
  return null;
}

/** 🧪 Three-valued evaluation of a cfg tree: unknown atoms stay unknown unless the connective decides. */
export function evalCfg(e: CfgExpr, env: Env, features: Map<string, boolean> | null = null): Tri {
  if (e.k === "atom") return atom(e.key, e.val, env, features);
  if (e.k === "not") {
    const v = evalCfg(e.x, env, features);
    return v === null ? null : !v;
  }
  let unknown = false;
  for (const x of e.xs) {
    const v = evalCfg(x, env, features);
    if (e.k === "all" && v === false) return false;
    if (e.k === "any" && v === true) return true;
    if (v === null) unknown = true;
  }
  if (unknown) return null;
  return e.k === "all";
}

const cache = new Map<string, CfgExpr | null>();

/** 🔎 Parses and memoizes the cfg argument text. */
export function cfgOf(text: string): CfgExpr | null {
  let v = cache.get(text);
  if (v === undefined) {
    v = parseCfg(text);
    cache.set(text, v);
  }
  return v;
}

/** ✅ An item chain is active unless some cfg is definitely false in the environment. */
export function active(chain: readonly string[], env: Env): boolean {
  for (const c of chain) {
    const e = cfgOf(c);
    if (e && evalCfg(e, env) === false) return false;
  }
  return true;
}

/** 🧷 Collects every `feature = "..."` atom name of a cfg tree. */
export function featureAtoms(e: CfgExpr, out: Set<string>): void {
  if (e.k === "atom") {
    if (e.key === "feature" && e.val !== null) out.add(e.val);
  } else if (e.k === "not") featureAtoms(e.x, out);
  else for (const x of e.xs) featureAtoms(x, out);
}

/** 🧩 True when two cfg chains can hold at once for some feature assignment in the environment. */
export function compatible(a: readonly string[], b: readonly string[], env: Env): boolean {
  const atoms = new Set<string>();
  const exprs: CfgExpr[] = [];
  for (const c of [...a, ...b]) {
    const e = cfgOf(c);
    if (!e) continue;
    exprs.push(e);
    featureAtoms(e, atoms);
  }
  const names = [...atoms];
  if (names.length > 10) return true;
  for (let mask = 0; mask < 1 << names.length; mask++) {
    const f = new Map<string, boolean>();
    names.forEach((n, i) => f.set(n, (mask >> i & 1) === 1));
    if (exprs.every((e) => evalCfg(e, env, f) !== false)) return true;
  }
  return false;
}
