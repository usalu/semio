export type PluginBuildTarget = {
	readonly pluginId: string;
	readonly cratePath: string;
	readonly wasmOut: string;
};

export const PLUGIN_BUILD_TARGETS: readonly PluginBuildTarget[] = [
	{ pluginId: "draw", cratePath: "draw/plugin/rs", wasmOut: "draw_plugin.wasm" },
	{ pluginId: "note", cratePath: "note/plugin/rs", wasmOut: "note_plugin.wasm" },
	{ pluginId: "writer", cratePath: "writer/plugin/rs", wasmOut: "writer_plugin.wasm" },
	{ pluginId: "raster", cratePath: "raster/plugin/rs", wasmOut: "raster_plugin.wasm" },
	{ pluginId: "forms", cratePath: "forms/plugin/rs", wasmOut: "forms_plugin.wasm" },
	{ pluginId: "vcs", cratePath: "vcs/plugin/rs", wasmOut: "vcs_plugin.wasm" },
	{ pluginId: "flow", cratePath: "flow/plugin/rs", wasmOut: "flow_plugin.wasm" },
	{ pluginId: "dag", cratePath: "mathematical/graph/port/directed/dag/plugin/rs", wasmOut: "dag_plugin.wasm" },
	{ pluginId: "imperative", cratePath: "imperative/plugin/rs", wasmOut: "imperative_plugin.wasm" },
	{ pluginId: "sequence", cratePath: "sequence/plugin/rs", wasmOut: "sequence_plugin.wasm" },
	{ pluginId: "layout", cratePath: "layout/plugin/rs", wasmOut: "layout_plugin.wasm" },
	{ pluginId: "puzzle2d", cratePath: "puzzle/2d/plugin/rs", wasmOut: "puzzle2d_plugin.wasm" },
	{ pluginId: "gis2d", cratePath: "gis/2d/plugin/rs", wasmOut: "gis2d_plugin.wasm" },
	{ pluginId: "procedural2d", cratePath: "procedural/2d/plugin/rs", wasmOut: "procedural2d_plugin.wasm" },
	{ pluginId: "reasoning-wires", cratePath: "reasoning/mindmap/wires/plugin/rs", wasmOut: "reasoning_wires_plugin.wasm" },
	{ pluginId: "cad", cratePath: "cad/plugin/rs", wasmOut: "cad_plugin.wasm" },
	{ pluginId: "puzzle3d", cratePath: "puzzle/3d/plugin/rs", wasmOut: "puzzle3d_plugin.wasm" },
	{ pluginId: "puzzle5d", cratePath: "puzzle/5d/plugin/rs", wasmOut: "puzzle5d_plugin.wasm" },
	{ pluginId: "shooting", cratePath: "shooting/plugin/rs", wasmOut: "shooting_plugin.wasm" },
	{ pluginId: "lowpoly", cratePath: "lowpoly/plugin/rs", wasmOut: "lowpoly_plugin.wasm" },
	{ pluginId: "procedural3d", cratePath: "procedural/3d/plugin/rs", wasmOut: "procedural3d_plugin.wasm" },
	{ pluginId: "trinity", cratePath: "trinity/jack/plugin/rs", wasmOut: "trinity_plugin.wasm" },
	{ pluginId: "trinity-rewrite", cratePath: "trinity/rewrite/plugin/rs", wasmOut: "trinity_rewrite_plugin.wasm" },
	{ pluginId: "s", cratePath: "s/plugin/rs", wasmOut: "s_plugin.wasm" },
	{ pluginId: "presentation", cratePath: "framework/product/presentation/plugin/rs", wasmOut: "presentation_plugin.wasm" },
];

export const pluginModuleUrl = (pluginId: string, fileName: string) => `/plugins/${pluginId}/${fileName.replace(/\.wasm$/, ".js")}`;
