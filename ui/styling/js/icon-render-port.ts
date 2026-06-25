// #region 🧲Header
/** @emoji 🖼️ Renderer-agnostic port for raster and vector icon export from three.js scenes. */
// #endregion 🧲Header

//#region 🔖IconRenderPort
export type IconRenderFormat = "svg" | "png";

export type IconRenderShape = "rectangle" | "ellipse";

export interface IconRenderCamera {
	readonly position: readonly [number, number, number];
	readonly target: readonly [number, number, number];
	readonly zoom: number;
	readonly fov?: number;
	readonly up?: readonly [number, number, number];
}

export interface IconRenderLights {
	readonly ambientIntensity: number;
	readonly ambientColor: string;
	readonly sunAzimuth: number;
	readonly sunElevation: number;
	readonly sunIntensity: number;
	readonly sunColor: string;
}

export interface IconRenderMaterial {
	readonly color?: string;
	readonly metalness?: number;
	readonly roughness?: number;
	readonly emissive?: string;
	readonly emissiveIntensity?: number;
}

export interface IconRenderRequest {
	readonly assetUrl: string;
	readonly camera: IconRenderCamera;
	readonly lights: IconRenderLights;
	readonly width: number;
	readonly height: number;
	readonly format: IconRenderFormat;
	readonly shape?: IconRenderShape;
	readonly background?: string;
	readonly shadowEnabled?: boolean;
	readonly material?: IconRenderMaterial;
}

export interface IconRenderResult {
	readonly dataUrl: string;
	readonly svgMarkup?: string;
}

export interface IconRenderPort {
	render(request: IconRenderRequest): Promise<IconRenderResult>;
}
//#endregion 🔖IconRenderPort
