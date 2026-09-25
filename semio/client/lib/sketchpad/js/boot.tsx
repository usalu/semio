// #region 🧲Header
/** @emoji 🚀 React entry of the sketchpad: surface hosts (docs MDX, feedback, hub), {@link bootSketchpad} for the full shell and the engine MCP App viewer mounts. */
// #endregion 🧲Header

import "./globals.css";
import { MDXProvider } from "@mdx-js/react";
import type { Platform } from "@framework/core";
import type { UiPanelHostSurfaceNode } from "@framework/platform/core";
import { mountPlatform, registerUiPanelSurfaceHost } from "@framework/platform/renderer/react";
import { Aside, Button, Card, CardGrid, FileTree, Input, Steps, Tabs, TabsContent, TabsList, TabsTrigger, Textarea } from "@ui/react";
import React, { Suspense, useEffect, useState } from "react";
import {
	SKETCHPAD_SHELL_CONTROLLER_ID,
	SKETCHPAD_SHELL_STORE_SHELL,
	SKETCHPAD_SURFACE_DOCS_PAGE,
	SKETCHPAD_SURFACE_FEEDBACK_FORM,
	ensureSketchpadPlatform,
	getSketchpadShellController,
	parseSketchpadRouteScopeFromPath,
	registerSketchpadDocsPages,
	seedSketchpadDevFixtureKitIfEmpty,
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
			<p className="text-sm text-muted-foreground">Share bugs, ideas, or questions about Semio Sketchpad.</p>
			<label className="flex flex-col gap-tiny text-sm">
				<span>Message</span>
				<Textarea
					id="semio.sketchpad.feedback.message"
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
					id="semio.sketchpad.feedback.contact"
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

// #region 🌐Hub
import { registerTabIcon } from "@framework/platform/renderer/react";
import { Avatar, AvatarFallback, BasicChatPanel, Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, type BasicChatMessage } from "@ui/react";
import { useSyncExternalStore } from "react";
import {
	SKETCHPAD_HUB_ACTIVITY_ICON_ID,
	SKETCHPAD_HUB_ICON_ID,
	SKETCHPAD_HUB_STORE,
	SKETCHPAD_SURFACE_HUB_ACTIVITY,
	SKETCHPAD_SURFACE_HUB_PANEL,
	getSketchpadHubController,
	setSketchpadHubFooterContent,
	sketchpadHubFocusLabel,
	sketchpadHubInitials,
	sketchpadHubStatusLabel,
	type SketchpadHubKitState,
	type SketchpadHubSnapshot,
} from "./index.ts";

const SKETCHPAD_HUB_STATE_COLOR: Readonly<Record<string, string>> = { synced: "#3cb44b", syncing: "#f58231", connecting: "#f58231", resyncing: "#4363d8", offline: "#e6194b", closed: "#94a3b8" };

/** @emoji 🪝 Live hub snapshot of the sketchpad hub controller. */
function useSketchpadHub(): SketchpadHubSnapshot | null {
	const store = getSketchpadHubController()?.getStore<SketchpadHubSnapshot>(SKETCHPAD_HUB_STORE);
	return useSyncExternalStore(
		(listener) => store?.subscribe(listener) ?? (() => undefined),
		() => store?.getSnapshot() ?? null,
	);
}

/** @emoji 🪝 Kit id of the active sketchpad route. */
function useSketchpadActiveKitId(): string | null {
	const store = getSketchpadShellController()?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
	const path = useSyncExternalStore(
		(listener) => store?.subscribe(listener) ?? (() => undefined),
		() => store?.getSnapshot().navigationPath ?? "/",
	);
	return parseSketchpadRouteScopeFromPath(path).kitId;
}

function sketchpadHubRun(command: string, args?: unknown): void {
	void getSketchpadHubController()?.execute(command, args).catch(() => undefined);
}

function sketchpadHubCopy(text: string): void {
	void navigator.clipboard?.writeText(text).catch(() => undefined);
}

/** @emoji 🧑‍🤝‍🧑 Colored participant avatar (🤖 for agents) with where-they-are tooltip. */
function SketchpadHubAvatar({ participant, where }: { readonly participant: SketchpadHubKitState["participants"][number]; readonly where: string }): React.ReactElement {
	return (
		<Avatar
			id={`sketchpad.hub.avatar.${participant.id}`}
			title={`${participant.name}${participant.kind === "agent" ? " (AI agent)" : ""} · ${where}`}
			data-testid="sketchpad-hub-avatar"
			data-participant-name={participant.name}
			data-participant-kind={participant.kind}
			className="size-[20px] border-2 border-background"
		>
			<AvatarFallback className="text-[9px] font-semibold text-white" style={{ backgroundColor: participant.color }}>
				{participant.kind === "agent" ? "🤖" : sketchpadHubInitials(participant.name)}
			</AvatarFallback>
		</Avatar>
	);
}

/** @emoji 👥 Footer chrome: sync status + avatar stack of the active shared kit. */
function SketchpadHubPresenceChrome(): React.ReactElement | null {
	const hub = useSketchpadHub();
	const kitId = useSketchpadActiveKitId();
	if (!hub) return null;
	const state = kitId ? hub.kits[kitId] : undefined;
	if (!state) {
		return (
			<span data-testid="sketchpad-hub-footer" className="px-single text-xs text-muted-foreground">
				{hub.person ? `Hub · ${hub.person.name}` : "Hub · signed out"}
			</span>
		);
	}
	const kit = getSketchpadShellController()?.getKitStore(state.kitId)?.getSnapshot().kit;
	return (
		<span data-testid="sketchpad-hub-footer" data-sync-state={state.status.state} data-sync-version={state.status.version} className="flex items-center gap-tight px-single text-xs">
			<span aria-hidden className="inline-block size-[8px] rounded-full" style={{ backgroundColor: SKETCHPAD_HUB_STATE_COLOR[state.status.state] ?? "#94a3b8" }} />
			<span data-testid="sketchpad-hub-status">{sketchpadHubStatusLabel(state.status)}</span>
			<span className="flex -space-x-[6px]">
				{state.participants.map((participant) => (
					<SketchpadHubAvatar key={participant.id} participant={participant} where={sketchpadHubFocusLabel(participant, kit)} />
				))}
			</span>
		</span>
	);
}

/** @emoji 🔑 Hub URL + sign in / register / sign out. */
function SketchpadHubAccount({ hub }: { readonly hub: SketchpadHubSnapshot }): React.ReactElement {
	const [url, setUrl] = useState(hub.url);
	const [name, setName] = useState("");
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	useEffect(() => setUrl(hub.url), [hub.url]);
	return (
		<section className="flex flex-col gap-tight" data-testid="sketchpad-hub-account">
			<label className="flex flex-col gap-tiny">
				<span className="text-muted-foreground">Hub URL</span>
				<span className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-tight">
					<Input id="sketchpad.hub.url" data-testid="sketchpad-hub-url" value={url} onChange={(event) => setUrl(event.target.value)} />
					<Button id="sketchpad.hub.url.save" type="button" text="Connect" onClick={() => sketchpadHubRun("setUrl", { url })} />
				</span>
			</label>
			{hub.person ? (
				<span className="flex items-center justify-between gap-tight">
					<span data-testid="sketchpad-hub-person">Signed in as {hub.person.name}</span>
					<Button id="sketchpad.hub.logout" type="button" text="Sign out" onClick={() => sketchpadHubRun("logout")} />
				</span>
			) : (
				<form
					className="flex flex-col gap-tight"
					data-testid="sketchpad-hub-auth"
					onSubmit={(event) => {
						event.preventDefault();
						const mode = ((event.nativeEvent as SubmitEvent).submitter as HTMLButtonElement | null)?.value;
						sketchpadHubRun(mode === "register" ? "register" : "login", { name, email, password });
					}}
				>
					<Input id="sketchpad.hub.name" data-testid="sketchpad-hub-name" placeholder="Name (register)" value={name} onChange={(event) => setName(event.target.value)} />
					<Input id="sketchpad.hub.email" data-testid="sketchpad-hub-email" type="email" placeholder="Email" value={email} onChange={(event) => setEmail(event.target.value)} />
					<Input id="sketchpad.hub.password" data-testid="sketchpad-hub-password" type="password" placeholder="Password" value={password} onChange={(event) => setPassword(event.target.value)} />
					<span className="flex gap-tight">
						<Button id="sketchpad.hub.login" type="submit" value="login" text="Sign in" disabled={hub.busy} />
						<Button id="sketchpad.hub.register" type="submit" value="register" text="Register" disabled={hub.busy || !name.trim()} />
					</span>
					{hub.pendingLink ? <span className="text-muted-foreground">Sign in to open the shared kit from your link.</span> : null}
				</form>
			)}
		</section>
	);
}

/** @emoji 🗂️ Home: hub sessions (shared kits) with open/delete plus create actions. */
function SketchpadHubSharedKits({ hub }: { readonly hub: SketchpadHubSnapshot }): React.ReactElement {
	const [name, setName] = useState("Shared kit");
	const localKitId = getSketchpadHubController()?.activeKitId();
	return (
		<section className="flex flex-col gap-tight" data-testid="sketchpad-hub-shared-kits">
			<span className="flex items-center justify-between">
				<span className="font-semibold">Shared kits</span>
				<Button id="sketchpad.hub.sessions.refresh" type="button" text="Refresh" onClick={() => sketchpadHubRun("refreshSessions")} />
			</span>
			{hub.sessions.length === 0 ? <span className="text-muted-foreground">No shared kits yet.</span> : null}
			{hub.sessions.map((session) => (
				<span key={session.id} data-testid="sketchpad-hub-session" data-session-name={session.name} className="flex items-center justify-between gap-tight rounded-[3px] border px-tight py-tiny">
					<span className="flex min-w-0 flex-col">
						<span className="truncate font-medium">{session.name}</span>
						<span className="text-muted-foreground">
							{session.role} · {session.owner?.name ?? "?"} · v{session.version} · {session.participantCount ?? 0} online
						</span>
					</span>
					<span className="flex gap-tight">
						<Button id={`sketchpad.hub.session.open.${session.id}`} type="button" text="Open" onClick={() => sketchpadHubRun("openSession", { sessionId: session.id })} />
						{session.role === "owner" ? <Button id={`sketchpad.hub.session.delete.${session.id}`} type="button" text="Delete" onClick={() => sketchpadHubRun("deleteSession", { sessionId: session.id })} /> : null}
					</span>
				</span>
			))}
			{localKitId ? (
				<Button
					id="sketchpad.hub.shareOpenKit"
					type="button"
					text={`Share ${getSketchpadShellController()?.getKitStore(localKitId)?.getSnapshot().kit.name ?? "open kit"}`}
					disabled={hub.busy}
					onClick={() => sketchpadHubRun("shareKit", { kitId: localKitId })}
				/>
			) : null}
			<span className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-tight">
				<Input id="sketchpad.hub.newSession.name" data-testid="sketchpad-hub-new-name" value={name} onChange={(event) => setName(event.target.value)} />
				<Button id="sketchpad.hub.newSession" type="button" text="Create empty" disabled={hub.busy} onClick={() => sketchpadHubRun("createEmptySession", { name })} />
			</span>
		</section>
	);
}

/** @emoji 🔗 Live session: status, participants (where they are), members and actions. */
function SketchpadHubSession({ state }: { readonly state: SketchpadHubKitState }): React.ReactElement {
	const kit = getSketchpadShellController()?.getKitStore(state.kitId)?.getSnapshot().kit;
	return (
		<section className="flex flex-col gap-tight" data-testid="sketchpad-hub-session-panel">
			<span className="font-semibold">{state.name}</span>
			<span data-testid="sketchpad-hub-session-status" data-sync-state={state.status.state}>
				{state.role} · {sketchpadHubStatusLabel(state.status)}
				{state.status.resyncs > 0 ? ` · ${state.status.resyncs} resync(s)` : ""}
			</span>
			<span className="text-muted-foreground">Participants</span>
			{state.participants.map((participant) => (
				<span key={participant.id} data-testid="sketchpad-hub-participant" data-participant-name={participant.name} className="flex items-center gap-tight">
					<SketchpadHubAvatar participant={participant} where={sketchpadHubFocusLabel(participant, kit)} />
					<span className="truncate">
						{participant.name}
						{participant.id === state.selfId ? " (you)" : ""}
						{participant.kind === "agent" ? " · AI" : ""}
					</span>
					<span className="ml-auto truncate text-muted-foreground">{sketchpadHubFocusLabel(participant, kit)}</span>
				</span>
			))}
			{state.members.length > 0 ? (
				<>
					<span className="text-muted-foreground">Members</span>
					{state.members.map((member) => (
						<span key={member.person.id} data-testid="sketchpad-hub-member" className="flex justify-between gap-tight">
							<span className="truncate">{member.person.name}</span>
							<span className="text-muted-foreground">{member.role}</span>
						</span>
					))}
				</>
			) : null}
			<span className="flex flex-wrap gap-tight">
				{state.role === "owner" ? <Button id="sketchpad.hub.share" type="button" text="Share…" onClick={() => sketchpadHubRun("openDialog", { dialog: "share" })} /> : null}
				<Button id="sketchpad.hub.connectAi" type="button" text="Connect AI…" onClick={() => sketchpadHubRun("openDialog", { dialog: "connectAi" })} />
				<Button id="sketchpad.hub.resync" type="button" text="Resync" onClick={() => sketchpadHubRun("resync", { kitId: state.kitId })} />
				<Button id="sketchpad.hub.members" type="button" text="Members" onClick={() => sketchpadHubRun("refreshMembers", { kitId: state.kitId })} />
			</span>
		</section>
	);
}

/** @emoji 📋 Read-only value with a copy button. */
function SketchpadHubCopyRow({ id, label, value }: { readonly id: string; readonly label: string; readonly value: string }): React.ReactElement {
	return (
		<div className="flex min-w-0 flex-col gap-tiny text-xs">
			<span className="flex items-center justify-between gap-tight">
				<span className="text-muted-foreground">{label}</span>
				<Button id={`${id}.copy`} type="button" text="Copy" onClick={() => sketchpadHubCopy(value)} />
			</span>
			<Textarea id={id} data-testid={id.replace(/\./g, "-")} className="break-all" value={value} readOnly rows={Math.min(10, Math.max(2, value.split("\n").length))} />
		</div>
	);
}

/** @emoji 🪟 Share links and Connect AI dialogs. */
function SketchpadHubDialogs({ hub, state }: { readonly hub: SketchpadHubSnapshot; readonly state: SketchpadHubKitState | undefined }): React.ReactElement {
	const controller = getSketchpadHubController();
	const [label, setLabel] = useState("Claude Code");
	const mcp = controller?.mcpSetup() ?? null;
	const close = (open: boolean) => {
		if (!open) sketchpadHubRun("closeDialog");
	};
	return (
		<>
			<Dialog open={hub.dialog === "share"} onOpenChange={close}>
				<DialogContent data-testid="sketchpad-hub-share-dialog">
					<DialogHeader>
						<DialogTitle>Share {state?.name ?? "kit"}</DialogTitle>
						<DialogDescription>Anyone with an editor link can change the kit live; viewers can follow along.</DialogDescription>
					</DialogHeader>
					<span className="flex gap-tight">
						<Button id="sketchpad.hub.share.editor" type="button" text="Create editor link" disabled={!state} onClick={() => sketchpadHubRun("createShare", { kitId: state?.kitId, role: "editor" })} />
						<Button id="sketchpad.hub.share.viewer" type="button" text="Create viewer link" disabled={!state} onClick={() => sketchpadHubRun("createShare", { kitId: state?.kitId, role: "viewer" })} />
					</span>
					{(state?.shares ?? []).map((share) => (
						<SketchpadHubCopyRow key={share.token} id={`sketchpad.hub.share.link.${share.role}`} label={`${share.role} link`} value={controller?.shareLink(share.token) ?? share.token} />
					))}
				</DialogContent>
			</Dialog>
			<Dialog open={hub.dialog === "connectAi"} onOpenChange={close}>
				<DialogContent data-testid="sketchpad-hub-ai-dialog">
					<DialogHeader>
						<DialogTitle>Connect AI</DialogTitle>
						<DialogDescription>Create an agent token and add the semio MCP server to your AI client. Its changes appear live for everyone in the session.</DialogDescription>
					</DialogHeader>
					<span className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-tight">
						<Input id="sketchpad.hub.ai.label" value={label} onChange={(event) => setLabel(event.target.value)} />
						<Button id="sketchpad.hub.ai.create" type="button" text="Create token" disabled={hub.busy} onClick={() => sketchpadHubRun("createAgentToken", { label })} />
					</span>
					{mcp ? (
						<>
							<SketchpadHubCopyRow id="sketchpad.hub.ai.endpoint" label="MCP endpoint" value={mcp.endpoint} />
							<SketchpadHubCopyRow id="sketchpad.hub.ai.claude" label="Claude Code" value={mcp.claudeCommand} />
							<SketchpadHubCopyRow id="sketchpad.hub.ai.json" label="mcpServers JSON" value={mcp.json} />
						</>
					) : null}
				</DialogContent>
			</Dialog>
		</>
	);
}

/** @emoji 🤝 Collaboration side panel: account, shared kits (home), live session and dialogs. */
function SketchpadHubPanelHost(): React.ReactElement {
	const hub = useSketchpadHub();
	const kitId = useSketchpadActiveKitId();
	if (!hub) return <div className="p-single text-xs text-muted-foreground">Hub unavailable</div>;
	const state = kitId ? hub.kits[kitId] : undefined;
	return (
		<div data-testid="sketchpad-hub-panel" className="flex flex-col gap-standard p-single text-xs">
			<SketchpadHubAccount hub={hub} />
			{hub.person && state ? <SketchpadHubSession state={state} /> : null}
			{hub.person && kitId && !state ? (
				<Button id="sketchpad.hub.shareActive" type="button" text="Share this kit on the hub" disabled={hub.busy} onClick={() => sketchpadHubRun("shareKit", { kitId })} />
			) : null}
			{hub.person && !kitId ? <SketchpadHubSharedKits hub={hub} /> : null}
			{hub.notice ? <span className="text-muted-foreground">{hub.notice}</span> : null}
			{hub.error ? (
				<span data-testid="sketchpad-hub-error" className="text-destructive">
					{hub.error}
				</span>
			) : null}
			<SketchpadHubDialogs hub={hub} state={state} />
		</div>
	);
}

/** @emoji 💬 Session activity feed (who did what, incl. AI agents, joins and leaves) in the chat side panel. */
function SketchpadHubActivityHost(): React.ReactElement {
	const hub = useSketchpadHub();
	const kitId = useSketchpadActiveKitId();
	const state = kitId ? hub?.kits[kitId] : undefined;
	const messages: readonly BasicChatMessage[] = (state?.activity ?? []).map((entry) => ({
		id: entry.id,
		author: entry.own ? `${entry.author} (you)` : entry.author,
		body: entry.text,
		color: entry.color,
		badge: entry.agent ? "AI" : undefined,
		timestamp: entry.at,
		own: entry.own,
	}));
	return (
		<BasicChatPanel
			id="sketchpad.hub.activity"
			title={state ? `Live activity · ${state.name}` : "Open a shared kit to follow its live activity."}
			messages={messages}
			emptyText={state ? "No activity yet — changes by collaborators and AI agents appear here." : "No shared kit open."}
		/>
	);
}

registerUiPanelSurfaceHost(SKETCHPAD_SURFACE_HUB_PANEL, SketchpadHubPanelHost);
registerUiPanelSurfaceHost(SKETCHPAD_SURFACE_HUB_ACTIVITY, SketchpadHubActivityHost);
registerTabIcon(SKETCHPAD_HUB_ICON_ID, "users");
registerTabIcon(SKETCHPAD_HUB_ACTIVITY_ICON_ID, "message-square");
setSketchpadHubFooterContent(<SketchpadHubPresenceChrome />);
// #endregion 🌐Hub

// #region 🤖McpApp
import { App as McpAppSdk } from "@modelcontextprotocol/ext-apps";
import { renderComponentHostSurface } from "@framework/platform/renderer/react";
import { createRoot } from "react-dom/client";
import {
	SKETCHPAD_MCP_VIEWER_TITLES,
	openSketchpadMcpPayload,
	sketchpadMcpPayloadFromToolResult,
	sketchpadMcpSelection,
	sketchpadMcpViewerBodies,
	type SketchpadMcpPayload,
	type SketchpadMcpViewerSurface,
} from "./index.ts";

/** @emoji 🔌 Host bridge of an MCP App viewer (hides the MCP Apps SDK behind the sketchpad). */
export interface SketchpadMcpHost {
	connect(onPayload: (payload: SketchpadMcpPayload) => void): Promise<void>;
	reportContext(text: string): void;
}

/** @emoji 🔌 {@link SketchpadMcpHost} over the MCP Apps SDK `App` (postMessage to the embedding host). */
export function createSketchpadMcpAppHost(surface: SketchpadMcpViewerSurface): SketchpadMcpHost {
	const app = new McpAppSdk({ name: SKETCHPAD_MCP_VIEWER_TITLES[surface], version: "1.0.0" }, {});
	return {
		async connect(onPayload) {
			app.ontoolresult = (result) => {
				const payload = sketchpadMcpPayloadFromToolResult(result);
				if (payload) onPayload(payload);
			};
			await app.connect();
		},
		reportContext(text) {
			void app.updateModelContext({ content: [{ type: "text", text }] }).catch(() => undefined);
		},
	};
}

/** @emoji 🤖 MCP App viewer: opens the tool payload kit through rs and renders the surface's sketchpad windows. */
function SketchpadMcpViewer({ surface, host }: { readonly surface: SketchpadMcpViewerSurface; readonly host: SketchpadMcpHost }): React.ReactElement {
	const [platform, setPlatform] = useState<Platform | null>(null);
	const [status, setStatus] = useState<string | null>("Waiting for a semio tool result…");

	useEffect(() => {
		document.title = SKETCHPAD_MCP_VIEWER_TITLES[surface];
		let active = true;
		let lastSelection = "";
		let detachShell: (() => void) | undefined;
		void (async () => {
			const next = await ensureSketchpadPlatform();
			if (!active) return;
			setPlatform(next);
			detachShell = getSketchpadShellController()
				?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)
				?.subscribe(() => {
					const selection = JSON.stringify({ selection: sketchpadMcpSelection() });
					if (selection === lastSelection) return;
					lastSelection = selection;
					host.reportContext(selection);
				});
			await host.connect((payload) => {
				setStatus("Opening kit…");
				void openSketchpadMcpPayload(payload, surface).then(
					() => active && setStatus(null),
					(error: unknown) => active && setStatus(error instanceof Error ? error.message : String(error)),
				);
			});
		})().catch((error: unknown) => active && setStatus(error instanceof Error ? error.message : String(error)));
		return () => {
			active = false;
			detachShell?.();
		};
	}, [host, surface]);

	return (
		<div className="relative flex h-full w-full min-h-0 gap-tight bg-background text-foreground" data-mcp-viewer-surface={surface}>
			{platform
				? sketchpadMcpViewerBodies(surface).map((body) => (
						<div key={body.surfaceId} className="relative min-h-0 min-w-0 flex-1">
							{renderComponentHostSurface(body, "canvas", platform)}
						</div>
					))
				: null}
			{status ? <div className="absolute inset-0 flex items-center justify-center p-4 text-sm text-muted-foreground">{status}</div> : null}
		</div>
	);
}

function mountSketchpadMcpViewer(surface: SketchpadMcpViewerSurface, root: HTMLElement, host: SketchpadMcpHost): void {
	createRoot(root).render(<SketchpadMcpViewer surface={surface} host={host} />);
}

/** @emoji 📦 Mounts the kit viewer (file system + wires) of the engine MCP App. */
export function mountMcpKitViewer(root: HTMLElement, host: SketchpadMcpHost = createSketchpadMcpAppHost("kit")): void {
	mountSketchpadMcpViewer("kit", root, host);
}

/** @emoji 🏗️ Mounts the design viewer (scene + diagram) of the engine MCP App. */
export function mountMcpDesignViewer(root: HTMLElement, host: SketchpadMcpHost = createSketchpadMcpAppHost("design")): void {
	mountSketchpadMcpViewer("design", root, host);
}

/** @emoji 🎬 Mounts the 3D scene viewer of the engine MCP App. */
export function mountMcpSceneViewer(root: HTMLElement, host: SketchpadMcpHost = createSketchpadMcpAppHost("scene")): void {
	mountSketchpadMcpViewer("scene", root, host);
}

/** @emoji 📐 Mounts the 2D diagram viewer of the engine MCP App. */
export function mountMcpDiagramViewer(root: HTMLElement, host: SketchpadMcpHost = createSketchpadMcpAppHost("diagram")): void {
	mountSketchpadMcpViewer("diagram", root, host);
}
// #endregion 🤖McpApp

/** @emoji 🚀 Mounts the full sketchpad {@link Platform} shell into `#root` (sketchpad, play and docs entries); dev servers seed the metabolism fixture kit. */
export function bootSketchpad(): Promise<void> {
	registerSketchpadDocsPages(import.meta.glob<SketchpadMdxModule>("./pages/**/*.mdx"));
	return mountPlatform(async () => {
		const platform = await ensureSketchpadPlatform();
		if (import.meta.env.DEV && !import.meta.env.SEMIO_SKETCHPAD_E2E) void seedSketchpadDevFixtureKitIfEmpty();
		return platform;
	});
}
