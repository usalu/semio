// #region 🧲️Header
/** @emoji 📐️ `@semio-tech/cad-js` — barrel for the 6 folded cad TS domains (spatial factory runtime/model graph, R3F renderer, brepjs kernel, construct query language, XState machine adapter, and the runtime composition root). Namespaced (not `export *`) — several domains export same-named symbols (e.g. `core` and `brepjs` both export box-preview helpers), which `tsc` treats as a hard ambiguity error under a flat re-export. Import as `import { core, renderer, brepjs, query, stately, runtime } from "@semio-tech/cad-js"` — package.json `exports` targets may not escape the package directory, so per-domain subpath exports (`@semio-tech/cad-js/core` etc.) are not available; the domain files live in the owner-root `🔨️modules/*` tree, not under `📦️packages/`. */
// #endregion 🧲️Header

export * as core from "../../🔨️modules/🫀️core/🟦️component.ts";
export * as brepjs from "../../🔨️modules/📐️brepjs/🟦️component.ts";
export * as query from "../../🔨️modules/🔍️query/🟦️component.ts";
export * as stately from "../../🔨️modules/🎰️stately/🟦️component.ts";
export * as runtime from "../../🔨️modules/🏃️runtime/🟦️component.ts";
export * as renderer from "../../🔨️modules/📺️renderer/🟦️component.tsx";
