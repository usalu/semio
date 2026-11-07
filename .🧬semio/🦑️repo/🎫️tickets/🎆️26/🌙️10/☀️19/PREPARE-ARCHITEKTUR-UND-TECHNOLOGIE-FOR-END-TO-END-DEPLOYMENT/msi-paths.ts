/** 🗂️ Resolves MSI destination names into a safe relative path. */
export function msiDestination(rows: readonly (readonly string[])[], directory: string, file: string): string {
  const names = new Map<string, readonly string[]>();
  for (const row of rows) {
    if (row.length !== 3 || !row[0] || names.has(row[0])) throw Error("Invalid or duplicate MSI directory");
    names.set(row[0], row);
  }
  const segment = (value: string, directoryName: boolean): string => {
    const target = directoryName ? value.split(":", 1)[0]! : value;
    const parts = target.split("|");
    if (parts.length > 2) throw Error("Invalid MSI name");
    const name = parts.at(-1)!;
    if (directoryName && name === ".") return "";
    if (!name || name === "." || name === ".." || /[<>:"/\\|?*\x00-\x1f]/u.test(name) || /[. ]$/u.test(name) || /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(name)) throw Error("Unsafe MSI path segment");
    return name;
  };
  const path = [segment(file, false)], seen = new Set<string>();
  while (directory) {
    if (seen.has(directory) || seen.size >= 64) throw Error("MSI directory cycle or depth exceeded");
    seen.add(directory);
    const row = names.get(directory);
    if (!row) throw Error("Missing MSI directory");
    if (directory === "TARGETDIR") {
      if (row[1] !== "" || row[2] !== "SourceDir") throw Error("Invalid MSI target root");
      break;
    }
    const name = segment(row[2]!, true);
    if (name) path.unshift(name);
    directory = row[1]!;
  }
  return path.join("/");
}
