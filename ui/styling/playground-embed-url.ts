// #region 🧲Header
/** @emoji 🌐 Browser-safe playground iframe hosts, dev ports, and embed URL resolver. */
// #endregion 🧲Header

//#region 🔖PlaygroundEmbedUrl
/** @emoji 🌐 Latest-only GitHub Pages hostnames for iframe-embeddable playground static sites. */
export const PLAYGROUND_SITE_HOSTS = {
	semio: "play.semio-tech.com",
	cad: "play.cad.semio-tech.com",
	"2d": "play.2d.semio-tech.com",
	"3d": "play.3d.semio-tech.com",
	"5d": "play.5d.semio-tech.com",
} as const;

export type PlaygroundSiteKind = keyof typeof PLAYGROUND_SITE_HOSTS;

/** @emoji 🔌 Local dev ports for iframe-embeddable playground static sites (match launch.json / project.json). */
export const PLAYGROUND_SITE_DEV_PORTS = {
	semio: "4000",
	cad: "6020",
	"2d": "6012",
	"3d": "6013",
	"5d": "6014",
} as const satisfies Record<PlaygroundSiteKind, string>;

/** @emoji 🌐 Playground iframe URL: localhost in dev, canonical host in production builds. */
export function playgroundEmbedUrl(kind: PlaygroundSiteKind, isDev: boolean): string {
	if (isDev) {
		return `http://localhost:${PLAYGROUND_SITE_DEV_PORTS[kind]}`;
	}
	return `https://${PLAYGROUND_SITE_HOSTS[kind]}`;
}
//#endregion 🔖PlaygroundEmbedUrl
