import base from "../../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts";
export default (env: { command: string; mode: string }) => {
  const config = (base as any)(env);
  config.plugins = [...config.plugins, { name: "qe2e-sizes", generateBundle(_: unknown, bundle: Record<string, any>) {
    for (const [name, chunk] of Object.entries(bundle)) {
      if (chunk.type !== "chunk" || !chunk.isEntry) continue;
      const rows = Object.entries(chunk.modules as Record<string, { renderedLength: number }>).map(([id, m]) => [m.renderedLength, id] as const).sort((a, b) => b[0] - a[0]);
      const byPackage = new Map<string, number>();
      for (const [size, id] of rows) {
        const match = /node_modules\/\.bun\/([^/]+)\//u.exec(id);
        const key = match ? match[1]! : id.replace(/^.*\/semio\//u, "").split("/").slice(0, 6).join("/");
        byPackage.set(key, (byPackage.get(key) ?? 0) + size);
      }
      console.log("[DEBUG] entry", name, chunk.code.length);
      for (const [key, size] of [...byPackage].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log("[DEBUG]", size, key);
    }
  } }];
  return config;
};
