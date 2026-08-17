import { readFileSync, writeFileSync } from "fs";
const path = process.argv[2];
let t = readFileSync(path, "utf8");
t = t.replace(
  `import { createContext, lazy, memo, Suspense, useCallback, useContext, useMemo, useState, type ComponentType, type LazyExoticComponent, type ReactElement, type ReactNode } from "react";`,
  `import { createContext, memo, useCallback, useContext, useMemo, useState, type ComponentType, type ReactElement, type ReactNode } from "react";`,
);
const fallback = `function ComponentSceneFallback() {
  const loadingSurfaceLabel = useLabel("ui.common.loadingSurface");
  return (
    <p className={cn("text-muted-foreground p-2 text-xs", loadingBorderClass)} role="status">
      {loadingSurfaceLabel}
    </p>
  );
}

`;
if (t.includes(fallback)) t = t.replace(fallback, "");
t = t.replace(
  `// 🧭️ Dispatches through \`<Suspense>\` into one of 14 lazily-loaded host components (or`,
  `// 🧭️ Dispatches into one of 14 scene host components (or`,
);
writeFileSync(path, t);
console.log("cleaned", path);
