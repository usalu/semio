// #region 🧲Header
/** @emoji 🚀 Vite entry: sketchpad {@link Platform} shell + docs MDX host. */
// #endregion 🧲Header

import "./globals.css";
import { MDXProvider } from "@mdx-js/react";
import type { Platform } from "@framework/core";
import type { UiPanelHostSurfaceNode } from "@framework/platform/core";
import { mountReactApp, PlatformShell, PlatformViewWithHistory, registerUiPanelSurfaceHost } from "@framework/platform/renderer/react";
import { Aside, Button, Card, CardGrid, FileTree, Input, NavbarFixtureSelect, NAVBAR_NO_FIXTURE_ID, Steps, Tabs, TabsContent, TabsList, TabsTrigger, Textarea } from "@ui/react";
import React, { Suspense, useEffect, useState } from "react";
import {
	SKETCHPAD_SHELL_CONTROLLER_ID,
	SKETCHPAD_SHELL_STORE_SHELL,
	SKETCHPAD_SURFACE_DOCS_PAGE,
	SKETCHPAD_SURFACE_FEEDBACK_FORM,
	ensureSketchpadPlatform,
	getSketchpadShellController,
	parseSketchpadRouteScopeFromPath,
	sketchpadFeedbackMailtoUri,
	sketchpadLoadMdxModule,
	sketchpadMdxTitle,
	type SketchpadFeedbackDraft,
	type SketchpadMdxModule,
	type SketchpadShellSnapshot,
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
		if (href.startsWith("/doc/") || href.startsWith("docs/")) {
			const path = href.replace(/^\/?docs\//, "");
			return (
				<a
					{...props}
					href={`/doc/${path}`}
					onClick={(event) => {
						event.preventDefault();
						window.history.pushState({}, "", `/doc/${path}`);
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

function readFeedbackDraft(): SketchpadFeedbackDraft {
	const shell = getSketchpadShellController()?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
	return shell?.feedback ?? { message: "", contact: "" };
}

function SketchpadFeedbackFormHost({
	platform,
}: {
	readonly node: UiPanelHostSurfaceNode;
	readonly platform?: Platform;
}): React.ReactElement {
	const [draft, setDraft] = useState<SketchpadFeedbackDraft>(() => readFeedbackDraft());
	const [submitted, setSubmitted] = useState(false);

	useEffect(() => {
		const ctrl = getSketchpadShellController();
		const shellStore = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (!shellStore) return;
		return shellStore.subscribe(() => {
			setDraft(readFeedbackDraft());
		});
	}, []);

	const dispatchDraft = (next: SketchpadFeedbackDraft) => {
		setDraft(next);
		platform?.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "setFeedbackDraft", next);
	};

	return (
		<form
			className="flex h-full min-h-0 flex-col gap-standard p-4"
			onSubmit={(event) => {
				event.preventDefault();
				platform?.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "submitFeedback");
				setSubmitted(true);
			}}
		>
			<h1 className="text-lg font-semibold">Feedback</h1>
			<p className="text-sm text-muted-foreground">Share bugs, ideas, or questions about Compose Sketchpad.</p>
			<label className="flex flex-col gap-tiny text-sm">
				<span>Message</span>
				<Textarea
					value={draft.message}
					onChange={(event) => dispatchDraft({ ...draft, message: event.target.value })}
					placeholder="What should we know?"
					rows={8}
					required
				/>
			</label>
			<label className="flex flex-col gap-tiny text-sm">
				<span>Contact (optional)</span>
				<Input
					value={draft.contact}
					onChange={(event) => dispatchDraft({ ...draft, contact: event.target.value })}
					placeholder="email@example.com"
				/>
			</label>
			<div className="flex flex-wrap items-center gap-tight">
				<Button type="submit">Send feedback</Button>
				{submitted && !sketchpadFeedbackMailtoUri(draft) ? (
					<span className="text-sm text-destructive">Enter a message before sending.</span>
				) : null}
				{submitted && sketchpadFeedbackMailtoUri(draft) ? (
					<span className="text-sm text-muted-foreground">Opening your mail client…</span>
				) : null}
			</div>
		</form>
	);
}

registerUiPanelSurfaceHost(SKETCHPAD_SURFACE_DOCS_PAGE, SketchpadDocsMdxHost);
registerUiPanelSurfaceHost(SKETCHPAD_SURFACE_FEEDBACK_FORM, SketchpadFeedbackFormHost);

function SketchpadKitFixtureNavbar({ platform }: { readonly platform: Platform }): React.ReactElement {
	const [tick, setTick] = useState(0);
	useEffect(() => {
		const ctrl = getSketchpadShellController();
		const shellStore = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (!shellStore) return;
		return shellStore.subscribe(() => setTick((n) => n + 1));
	}, []);
	void tick;
	const ctrl = getSketchpadShellController();
	const shell = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
	const openKitIds = shell?.openKitIds ?? [];
	const pathOnly = platform.uri.split("?")[0] ?? "/";
	const routeKitId = parseSketchpadRouteScopeFromPath(pathOnly).kitId;
	const activeKitId = routeKitId ?? NAVBAR_NO_FIXTURE_ID;
	const options = openKitIds.map((kitId) => {
		const kit = ctrl?.getKitStore(kitId)?.getSnapshot().kit;
		return { id: kitId, label: kit?.name ?? kitId };
	});
	return (
		<NavbarFixtureSelect
			id="compose.sketchpad.navbar.fixture"
			label="Kit"
			value={activeKitId}
			options={options}
			onValueChange={(kitId) => {
				if (kitId === NAVBAR_NO_FIXTURE_ID) {
					platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "navigate", { path: "/" });
					return;
				}
				platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "navigate", { path: `/kits/${kitId}` });
			}}
		/>
	);
}

void ensureSketchpadPlatform().then((platform) => {
	mountReactApp(
		<PlatformShell>
			<PlatformViewWithHistory platform={platform} slotNavbarCenter={<SketchpadKitFixtureNavbar platform={platform} />} />
		</PlatformShell>,
	);
});
