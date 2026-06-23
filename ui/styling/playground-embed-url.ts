// #region 🧲Header
/** @emoji 🌐 Browser-safe playground iframe hosts, dev ports, and embed URL resolver. */
// #endregion 🧲Header

import { PLAYGROUND_EMBED_SITE_DEV_PORTS, type PlaygroundEmbedSiteKind } from "./playground-dev-ports.ts";

//#region 🔖PlaygroundEmbedUrl
/** @emoji 🌐 Latest-only GitHub Pages hostnames for iframe-embeddable playground static sites. */
export const PLAYGROUND_SITE_HOSTS = {
	compose: "play.semio-tech.com",
	cad: "play.cad.semio-tech.com",
	"2d": "play.2d.semio-tech.com",
	"3d": "play.3d.semio-tech.com",
	"5d": "play.5d.semio-tech.com",
} as const;

export type PlaygroundSiteKind = PlaygroundEmbedSiteKind;

/** @emoji 🔌 Local dev ports for iframe-embeddable playground static sites (from `playground-dev-ports.ts`). */
export const PLAYGROUND_SITE_DEV_PORTS = PLAYGROUND_EMBED_SITE_DEV_PORTS;

/** @emoji 🌐 Playground iframe URL: localhost in dev, canonical host in production builds. */
export function playgroundEmbedUrl(kind: PlaygroundSiteKind, isDev: boolean): string {
	if (isDev) {
		return `http://localhost:${PLAYGROUND_SITE_DEV_PORTS[kind]}`;
	}
	return `https://${PLAYGROUND_SITE_HOSTS[kind]}`;
}
//#endregion 🔖PlaygroundEmbedUrl
