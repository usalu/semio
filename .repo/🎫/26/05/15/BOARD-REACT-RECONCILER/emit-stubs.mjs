import fs from "node:fs";

const s = fs.readFileSync("c:/git/compose/elements/client/lib/board/node_modules/react-reconciler/cjs/react-reconciler.development.js", "utf8");
const re = /([a-zA-Z0-9$]+) = \$\$\$config\.([a-zA-Z0-9]+)/g;
const keys = new Set();
let m;
while ((m = re.exec(s))) {
	keys.add(m[2]);
}

const lines = [];
for (const k of [...keys].sort()) {
	if (k === "extraDevToolsConfig") {
		lines.push(`\t${k}: {},`);
		continue;
	}
	if (k === "rendererVersion") {
		lines.push(`\t${k}: "0.1.0",`);
		continue;
	}
	if (k === "rendererPackageName") {
		lines.push(`\t${k}: "@elements/board",`);
		continue;
	}
	if (k === "HostTransitionContext") {
		lines.push(`\t${k}: React.createContext(null) as never,`);
		continue;
	}
	if (k === "NotPendingTransition") {
		lines.push(`\t${k}: null,`);
		continue;
	}
	if (k === "supportsMutation" || k === "supportsPersistence" || k === "supportsHydration") {
		continue;
	}
	if (k === "isPrimaryRenderer") {
		continue;
	}
	if (k === "noTimeout") {
		continue;
	}
	if (k === "scheduleTimeout" || k === "cancelTimeout") {
		continue;
	}
	if (k === "supportsMicrotasks" || k === "scheduleMicrotask") {
		continue;
	}
	if (k === "createInstance" || k === "appendInitialChild" || k === "appendChild" || k === "appendChildToContainer") {
		continue;
	}
	if (k === "insertBefore" || k === "insertInContainerBefore") {
		continue;
	}
	if (k === "removeChild" || k === "removeChildFromContainer" || k === "clearContainer") {
		continue;
	}
	if (k === "finalizeInitialChildren" || k === "prepareUpdate" || k === "commitUpdate" || k === "commitMount") {
		continue;
	}
	if (k === "getPublicInstance" || k === "prepareForCommit" || k === "resetAfterCommit" || k === "preparePortalMount") {
		continue;
	}
	if (k === "getRootHostContext" || k === "getChildHostContext") {
		continue;
	}
	if (k === "createTextInstance" || k === "shouldSetTextContent") {
		continue;
	}
	if (k === "detachDeletedInstance") {
		continue;
	}
	if (k === "getInstanceFromNode") {
		continue;
	}
	if (k === "setCurrentUpdatePriority" || k === "getCurrentUpdatePriority" || k === "resolveUpdatePriority") {
		continue;
	}
	// default stub
	if (
		k.startsWith("can") ||
		k.startsWith("is") ||
		k === "supportsTestSelectors" ||
		k === "supportsResources" ||
		k === "supportsSingletons" ||
		k === "shouldAttemptEagerTransition" ||
		k === "shouldSetTextContent" ||
		k === "shouldDeleteUnhydratedTailInstances"
	) {
		lines.push(`\t${k}: () => false,`);
		continue;
	}
	if (k.includes("Hydrate") || k.includes("hydrate") || k.includes("Hydration")) {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k === "getTextContent" || k === "getBoundingRect" || k === "findFiberRoot") {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k === "setupIntersectionObserver") {
		lines.push(`\t${k}: () => () => undefined,`);
		continue;
	}
	if (k === "bindToConsole") {
		lines.push(`\t${k}: () => () => undefined,`);
		continue;
	}
	if (k === "trackSchedulerEvent" || k === "resetFormInstance" || k === "suspendInstance" || k === "preloadInstance") {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k === "resolveEventType") {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k === "resolveEventTimeStamp") {
		lines.push(`\t${k}: () => -1.1,`);
		continue;
	}
	if (k === "maySuspendCommit" || k === "maySuspendCommitOnUpdate" || k === "maySuspendCommitInSyncRender") {
		lines.push(`\t${k}: () => false,`);
		continue;
	}
	if (k === "startSuspendingCommit" || k === "waitForCommitToBeReady" || k === "getSuspendedCommitReason") {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k === "preloadResource" || k === "suspendResource" || k === "mayResourceSuspendCommit") {
		lines.push(`\t${k}: () => false,`);
		continue;
	}
	if (k.includes("ContainerChild") || k.includes("Children") || k.includes("Boundary") || k.includes("flush")) {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k.includes("diff") || k.includes("describe") || k.includes("validate")) {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k.startsWith("hide") || k.startsWith("unhide")) {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k.includes("clone") || k === "replaceContainerChildren") {
		lines.push(`\t${k}: () => { throw new Error("Board host: ${k} unsupported"); },`);
		continue;
	}
	if (k.startsWith("acquire") || k.startsWith("release") || k.startsWith("mount") || k.startsWith("unmount") || k.startsWith("create") || k.startsWith("prepareTo")) {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k.startsWith("get") || k.startsWith("resolve")) {
		lines.push(`\t${k}: () => null,`);
		continue;
	}
	if (k.startsWith("register")) {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k === "commitTextUpdate" || k === "resetTextContent") {
		lines.push(`\t${k}: () => {},`);
		continue;
	}
	if (k === "setFocusIfFocusable" || k === "matchAccessibilityRole" || k === "isHiddenSubtree") {
		lines.push(`\t${k}: () => false,`);
		continue;
	}
	lines.push(`\t${k}: () => { throw new Error("Board host stub missing: ${k}"); },`);
}
console.log(lines.join("\n"));
