// #region 🧲Header
/** @emoji 🚀 Vite entry: {@link mountPlatform} + sketchpad {@link Platform} + docs MDX host. */
// #endregion 🧲Header

import "./globals.css";
import { MDXProvider } from "@mdx-js/react";
import type { Platform } from "@framework/core";
import type { UiPanelHostSurfaceNode } from "@framework/platform/core";
import { mountPlatform, registerUiPanelSurfaceHost } from "@framework/platform/renderer/react";
import { Aside, Card, CardGrid, FileTree, Steps, Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/react";
import React, { Suspense, useEffect, useState } from "react";
import {
	SKETCHPAD_SURFACE_DOCS_PAGE,
	ensureSketchpadPlatform,
	parseSketchpadRouteScopeFromPath,
	sketchpadLoadMdxModule,
	sketchpadMdxTitle,
	type SketchpadMdxModule,
} from "./index.ts";

const SKETCHPAD_MDX_COMPONENTS = {
	Aside,
	Card,
	CardGrid,
	FileTree,
	Steps,
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
	a: (props: React.AnchorHTMLAttributes<HTMLAnchorElement>) => {
		const href = props.href ?? "";
		if (href.startsWith("/docs/") || href.startsWith("docs/")) {
			const path = href.replace(/^\/?docs\//, "");
			return (
				<a
					{...props}
					href={`/docs/${path}`}
					onClick={(event) => {
						event.preventDefault();
						window.history.pushState({}, "", `/docs/${path}`);
						window.dispatchEvent(new PopStateEvent("popstate"));
					}}
				/>
			);
		}
		return <a {...props} />;
	},
};

function SketchpadDocsMdxHost({
	platform,
}: {
	readonly node: UiPanelHostSurfaceNode;
	readonly platform?: Platform;
}): React.ReactElement {
	const pathOnly = platform?.uri.split("?")[0] ?? "/";
	const docsPath = parseSketchpadRouteScopeFromPath(pathOnly).docsPath;
	const [state, setState] = useState<
		| { readonly status: "loading" }
		| { readonly status: "ready"; readonly module: SketchpadMdxModule }
		| { readonly status: "error"; readonly message: string }
	>({ status: "loading" });

	useEffect(() => {
		let cancelled = false;
		setState({ status: "loading" });
		void sketchpadLoadMdxModule(docsPath).then((module) => {
			if (cancelled) return;
			if (!module?.default) {
				setState({ status: "error", message: `No MDX page for "${docsPath}"` });
				return;
			}
			setState({ status: "ready", module });
		});
		return () => {
			cancelled = true;
		};
	}, [docsPath]);

	if (state.status === "loading") {
		return <div className="p-4 text-sm text-muted-foreground">Loading documentation…</div>;
	}
	if (state.status === "error") {
		return <div className="p-4 text-sm text-destructive">{state.message}</div>;
	}
	const Content = state.module.default as React.ComponentType<Record<string, never>>;
	const title = sketchpadMdxTitle(state.module, docsPath);
	return (
		<article className="prose prose-sm dark:prose-invert max-w-none p-4">
			<h1 className="not-prose mb-4 text-xl font-semibold">{title}</h1>
			<MDXProvider components={SKETCHPAD_MDX_COMPONENTS}>
				<Suspense fallback={<div className="text-muted-foreground">Rendering…</div>}>
					<Content />
				</Suspense>
			</MDXProvider>
		</article>
	);
}

registerUiPanelSurfaceHost(SKETCHPAD_SURFACE_DOCS_PAGE, SketchpadDocsMdxHost);

void mountPlatform(ensureSketchpadPlatform);
