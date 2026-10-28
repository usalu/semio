// undeclared-g hybrid cover: the smallest Seatbelt rule set that grants EXACTLY the declared files when the plain cover (one literal per file) exceeds the kernel's 64 KB profile limit.
// Per directory two strategies compete: (literal) grant every declared entry on its own, or (hybrid) grant the whole directory with ONE subpath rule and deny every undeclared entry inside it (a denial after a grant wins in a Seatbelt profile, the last matching rule decides).
// The cheaper one (rules + denials) wins, ties go to literal; an undeclared entry is denied as ONE subpath however large it is. Pure: the directory layout comes in through `list`.

/** @typedef {{kind: "subpath" | "literal", path: string}} Rule */
/** @typedef {{rules: Rule[], denials: string[]}} HybridCover */

/** 🧩️ Builds the trie of declared relative paths. */
function trieOf(allowed) {
  const root = new Map();
  for (const path of new Set(allowed)) {
    let node = root;
    for (const part of path.split("/")) { let child = node.get(part); if (!child) node.set(part, (child = new Map())); node = child; }
  }
  return root;
}

/** 🧩️ The denials a fully granted directory needs: every undeclared entry once, then the same inside every declared directory. */
function inside(path, node, list) {
  const denials = [], names = list(path);
  if (names === undefined) return denials;
  for (const name of names) {
    const at = path ? `${path}/${name}` : name, child = node.get(name);
    if (child === undefined) denials.push(at);
    else if (child.size) denials.push(...inside(at, child, list));
  }
  return denials;
}

/**
 * 🧩️ Covers the declared files with grants and denials.
 * @param {readonly string[]} allowed workspace relative declared files (`/` separators)
 * @param {(directory: string) => readonly string[] | undefined} list entry names of a workspace directory (`""` is the root), `undefined` when it does not exist
 * @returns {HybridCover}
 */
export function hybridCoverV1(allowed, list) {
  const best = (path, node) => {
    const literal = { rules: [], denials: [] };
    for (const [name, child] of [...node].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))) {
      const at = path ? `${path}/${name}` : name;
      if (child.size) { const row = best(at, child); literal.rules.push(...row.rules); literal.denials.push(...row.denials); }
      else literal.rules.push({ kind: "literal", path: at });
    }
    const denials = inside(path, node, list), cost = 1 + denials.length;
    if (!node.size || cost >= literal.rules.length + literal.denials.length || list(path) === undefined) return literal;
    return { rules: [{ kind: "subpath", path: path === "" ? "." : path }], denials };
  };
  return best("", trieOf(allowed));
}

/** 🧩️ Whether a hybrid cover grants a workspace file: a grant matches the file or an ancestor, a denial after it removes the grant. */
export function hybridGrantsV1(cover, path) {
  const under = (base, file) => base === "." || file === base || file.startsWith(`${base}/`);
  const granted = cover.rules.some((rule) => (rule.kind === "literal" ? path === rule.path : under(rule.path, path)));
  return granted && !cover.denials.some((denial) => under(denial, path));
}
