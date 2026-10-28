/** 🌳️ The owner of a project: the path before its first delivery scope directory (`📦️packages`), else the project root itself. */
export function ownerOfProjectV1(root, deliveryScopeNames) {
  const offsets = deliveryScopeNames.map((name) => root.indexOf(`/${name}/`)).filter((index) => index >= 0);
  return offsets.length > 0 ? root.slice(0, Math.min(...offsets)) : root;
}

/** 🌳️ The policy row of an owner whose subtrees are private: its own tasks key them, the tasks of its dependents do not. Framework files do mention products, so the premise is not that a framework module cannot depend on one but that every reach is covered another way: Cargo path dependencies are inferred project edges, Rust `#[path]` and `include*!` are followed by the crate source inputs, and a test is an own task that keys the private directory itself (`./🧪️tests/🏝️reach` pins the census; an uncovered reach must be listed there with its reason). */
export function ownerViewV1(owner, views) {
  return (views ?? []).find((row) => row.owner === owner);
}

/** 🌳️ Splits the owner glob of one project into the globs every dependent sees and the globs only the project's own tasks see: one glob per top-level directory of the owner (hidden entries never), the private directories apart, top-level files in one glob.
 * @param {string} owner @param {string} extensions brace list of the source extensions @param {readonly string[]} privateNames @param {readonly {name:string,directory:boolean}[]} entries the top-level entries of the owner */
export function ownerTreeGlobsV1(owner, extensions, privateNames, entries) {
  const shared = [], hidden = [];
  let files = false;
  for (const entry of [...entries].sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0))) {
    if (entry.name.startsWith(".")) continue;
    if (!entry.directory) { files = true; continue; }
    (privateNames.includes(entry.name) ? hidden : shared).push(`{workspaceRoot}/${owner}/${entry.name}/**/*.${extensions}`);
  }
  if (files) shared.push(`{workspaceRoot}/${owner}/*.${extensions}`);
  return { shared, private: hidden };
}
