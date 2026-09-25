import base from "/home/user/semio/semio/client/lib/sketchpad/js/vite.config.ts";
/** 🧪 F2 E2E dev server: sketchpad config on :5174 without HMR/file watching (stable against concurrent edits). */
export default async (env: { mode: string; command: string }) => {
	const config = await (typeof base === "function" ? (base as (e: typeof env) => Promise<Record<string, any>>)(env) : base);
	return { ...config, root: "/home/user/semio/semio/client/lib/sketchpad/js", server: { ...config.server, host: "127.0.0.1", port: 5174, strictPort: true, hmr: false, watch: { ignored: ["**/*"] } } };
};
