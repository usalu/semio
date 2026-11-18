import config from "./vite-play.config.ts";

const stub = { name: "frozen-refresh-stub", transformIndexHtml: () => [{ tag: "script", children: "window.$RefreshReg$=()=>{};window.$RefreshSig$=()=>t=>t;window.__vite_plugin_react_preamble_installed__=true;", injectTo: "head-prepend" as const }] };

/** 🧊️ Play harness with file watcher and HMR off, so concurrent edits by other agents cannot reload the probed page. */
export default { ...config, plugins: [stub, ...(config.plugins ?? [])], server: { ...(config.server ?? {}), watch: null, hmr: false } };
