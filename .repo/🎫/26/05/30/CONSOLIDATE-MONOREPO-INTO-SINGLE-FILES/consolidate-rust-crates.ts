import { readFileSync, writeFileSync, unlinkSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

type ModSpec = {
	libPath: string;
	mods: Array<{ name: string; pub?: boolean; cfg?: string }>;
	includes?: string[];
};

function inlineMod(lib: string, libPath: string, name: string, pub: boolean, cfg?: string): string {
	const dir = dirname(libPath);
	const modPath = join(dir, `${name}.rs`);
	if (!existsSync(modPath)) throw new Error(`missing ${modPath}`);
	const body = readFileSync(modPath, "utf8").trimEnd();
	const decl = cfg
		? `#[cfg(${cfg})]\n${pub ? "pub " : ""}mod ${name};`
		: `${pub ? "pub " : ""}mod ${name};`;
	if (!lib.includes(decl)) throw new Error(`${libPath}: missing declaration:\n${decl}`);
	const inline = cfg
		? `#[cfg(${cfg})]\n${pub ? "pub " : ""}mod ${name} {\n// #region ${name}\n${body}\n// #endregion ${name}\n}\n`
		: `${pub ? "pub " : ""}mod ${name} {\n// #region ${name}\n${body}\n// #endregion ${name}\n}\n`;
	lib = lib.replace(decl, inline);
	unlinkSync(modPath);
	return lib;
}

function inlineInclude(lib: string, libPath: string, fileName: string): string {
	const dir = dirname(libPath);
	const modPath = join(dir, fileName);
	if (!existsSync(modPath)) throw new Error(`missing ${modPath}`);
	const body = readFileSync(modPath, "utf8").trimEnd();
	const decl = `include!("${fileName}");`;
	if (!lib.includes(decl)) throw new Error(`${libPath}: missing ${decl}`);
	lib = lib.replace(decl, `// #region ${fileName.replace(/\.rs$/, "")}\n${body}\n// #endregion ${fileName.replace(/\.rs$/, "")}`);
	unlinkSync(modPath);
	return lib;
}

function consolidate(spec: ModSpec): void {
	let lib = readFileSync(spec.libPath, "utf8");
	for (const include of spec.includes ?? []) {
		lib = inlineInclude(lib, spec.libPath, include);
	}
	for (const mod of spec.mods) {
		lib = inlineMod(lib, spec.libPath, mod.name, mod.pub ?? false, mod.cfg);
	}
	writeFileSync(spec.libPath, lib);
	console.log(`consolidated ${spec.libPath}`);
}

const root = join(import.meta.dirname, "../../../../../../");

const specs: ModSpec[] = [
	{
		libPath: join(root, "ui/wgpu/rs/lib.rs"),
		mods: [
			{ name: "chrome", pub: true },
			{ name: "cursor", pub: true },
			{ name: "draw", pub: true },
			{ name: "geometry", pub: true },
			{ name: "gpu", pub: true },
			{ name: "input", pub: true },
			{ name: "layout", pub: true },
			{ name: "shaders", pub: true },
			{ name: "text", pub: true },
			{ name: "theme", pub: true },
			{ name: "widgets", pub: true },
		],
	},
	{
		libPath: join(root, "framework/renderer/wgpu/rs/lib.rs"),
		mods: [
			{ name: "dock", pub: true },
			{ name: "engine_canvas", pub: true },
			{ name: "interpreter", pub: true },
			{ name: "plugin_bridge", pub: true },
			{ name: "scenes", pub: true },
			{ name: "shell", pub: true },
		],
	},
	{
		libPath: join(root, "framework/core/rs/lib.rs"),
		mods: [
			{ name: "command_bus", pub: true },
			{ name: "layout", pub: true },
			{ name: "mesh", pub: true },
			{ name: "platform", pub: true },
			{ name: "tools", pub: true },
			{ name: "ui", pub: true },
		],
	},
	{
		libPath: join(root, "framework/product/os/core/rs/lib.rs"),
		mods: [
			{ name: "host", pub: true },
			{ name: "instance", pub: true },
			{ name: "media_export_raster", pub: true },
			{ name: "media_export_simple", pub: true },
			{ name: "media_graph", pub: true },
			{ name: "registry", pub: true },
		],
	},
	{
		libPath: join(root, "framework/plugin/rs/lib.rs"),
		mods: [
			{ name: "app", pub: true },
			{ name: "generate_mode", pub: true },
			{ name: "plugin_runtime", pub: true },
			{ name: "scaffold", pub: true },
			{ name: "world3d_host", pub: true },
		],
	},
	{
		libPath: join(root, "layout/rs/lib.rs"),
		mods: [
			{ name: "document" },
			{ name: "display" },
			{ name: "engine" },
			{ name: "export" },
			{ name: "wasm_session", cfg: "target_arch = \"wasm32\"" },
		],
	},
	{
		libPath: join(root, "mathematical/graph/dsl/rs/lib.rs"),
		includes: ["jack_impl.rs"],
		mods: [
			{ name: "queryable", pub: true },
			{ name: "wire", pub: true },
		],
	},
	{
		libPath: join(root, "trinity/jack/core/rs/lib.rs"),
		mods: [{ name: "queryable", pub: true }],
	},
	{
		libPath: join(root, "writer/rs/lib.rs"),
		mods: [{ name: "document_vcs" }],
	},
	{
		libPath: join(root, "writer/plugin/rs/lib.rs"),
		mods: [{ name: "grammar" }],
	},
];

for (const spec of specs) consolidate(spec);
