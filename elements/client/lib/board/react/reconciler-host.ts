/** @emoji 🧩 `react-reconciler` host wiring for imperative {@link BoardRenderer} scene objects. */
import type { ReactElement } from "react";
import Reconciler from "react-reconciler";
import { ConcurrentRoot } from "react-reconciler/constants";

import {
	BoardRenderer,
	Edge as BoardEdgeObject,
	Handle as BoardHandleObject,
	Node as BoardNodeObject,
	type BoardEdgeProps,
	type BoardHandleProps,
	type BoardNodeProps,
} from "../js/index";
import { BOARD_RECONCILER_DEFAULTS } from "./reconciler-defaults";

const boardSchedulingHooks = {
	getCurrentUpdatePriority: BOARD_RECONCILER_DEFAULTS.getCurrentUpdatePriority as () => number,
	resolveUpdatePriority: BOARD_RECONCILER_DEFAULTS.resolveUpdatePriority as () => number,
	setCurrentUpdatePriority: BOARD_RECONCILER_DEFAULTS.setCurrentUpdatePriority as (p: number) => void,
};

//#region 🔖HostKinds
export const BOARD_HOST_NODE = "elements.board/node";
export const BOARD_HOST_HANDLE = "elements.board/handle";
export const BOARD_HOST_EDGE = "elements.board/edge";

export type BoardHostType = typeof BOARD_HOST_NODE | typeof BOARD_HOST_HANDLE | typeof BOARD_HOST_EDGE;

interface BoardHostNode {
	kind: "node";
	impl: BoardNodeObject;
	renderer: BoardRenderer;
	readonly handleChildren: Set<BoardHostHandle>;
}

interface BoardHostHandle {
	kind: "handle";
	impl: BoardHandleObject | null;
	props: BoardHandleProps;
	renderer: BoardRenderer;
}

interface BoardHostEdge {
	kind: "edge";
	impl: BoardEdgeObject | null;
	props: BoardEdgeProps;
	renderer: BoardRenderer;
}

export type BoardHostInstance = BoardHostNode | BoardHostHandle | BoardHostEdge;
//#endregion 🔖HostKinds

//#region 🔖PropApply
function newBoardNodeFromProps(props: BoardNodeProps): BoardNodeObject {
	if (props.shape === "rectangle") {
		return new BoardNodeObject({
			draggable: props.draggable ?? true,
			height: props.height,
			id: props.id,
			selected: props.selected,
			shape: "rectangle",
			style: props.style,
			text: props.text,
			userData: props.userData,
			visible: props.visible,
			width: props.width,
			x: props.x,
			y: props.y,
		});
	}
	return new BoardNodeObject({
		draggable: props.draggable ?? true,
		id: props.id,
		radius: props.radius,
		selected: props.selected,
		style: props.style,
		text: props.text,
		userData: props.userData,
		visible: props.visible,
		x: props.x,
		y: props.y,
	});
}

function applyNodeProps(instance: BoardNodeObject, props: BoardNodeProps): void {
	instance.draggable = props.draggable ?? true;
	instance.selected = props.selected ?? false;
	instance.style = props.style ?? null;
	instance.userData = { ...(props.userData ?? {}) };
	instance.visible = props.visible ?? true;
	instance.setPosition(props.x, props.y);
	instance.setText(props.text ?? null);
	if (props.shape === "rectangle") {
		instance.setRectangleSize(props.width, props.height);
	} else {
		instance.setRadius(props.radius);
	}
}

function applyHandleProps(instance: BoardHandleObject, props: BoardHandleProps, node: BoardNodeObject): void {
	if (instance.node !== node) {
		instance.node.detachHandle(instance);
		node.attachHandle(instance);
		instance.node = node;
	}
	instance.selected = props.selected ?? false;
	instance.style = props.style ?? null;
	instance.userData = { ...(props.userData ?? {}) };
	instance.visible = props.visible ?? true;
	instance.radius = props.radius ?? 8;
	instance.setAngle(props.angle);
}

function applyEdgeProps(instance: BoardEdgeObject, props: BoardEdgeProps, fromHandle: BoardHandleObject, toHandle: BoardHandleObject): void {
	instance.selected = props.selected ?? false;
	instance.style = props.style ?? null;
	instance.userData = { ...(props.userData ?? {}) };
	instance.visible = props.visible ?? true;
	instance.setEndpoints(fromHandle, toHandle);
}

function nodeShapeSyncKey(props: BoardNodeProps): "circle" | "rectangle" {
	return props.shape === "rectangle" ? "rectangle" : "circle";
}

function instanceShapeSyncKey(node: BoardNodeObject): "circle" | "rectangle" {
	return node.shape;
}

function propsEqualHandle(a: BoardHandleProps, b: BoardHandleProps): boolean {
	return (
		a.id === b.id &&
		a.angle === b.angle &&
		a.radius === b.radius &&
		a.selected === b.selected &&
		a.style === b.style &&
		a.visible === b.visible &&
		shallowEqualRecord(a.userData ?? {}, b.userData ?? {})
	);
}

function propsEqualEdge(a: BoardEdgeProps, b: BoardEdgeProps): boolean {
	return (
		a.id === b.id &&
		a.from === b.from &&
		a.to === b.to &&
		a.selected === b.selected &&
		a.style === b.style &&
		a.visible === b.visible &&
		shallowEqualRecord(a.userData ?? {}, b.userData ?? {})
	);
}

function shallowEqualRecord(left: Record<string, unknown>, right: Record<string, unknown>): boolean {
	const leftKeys = Object.keys(left);
	const rightKeys = Object.keys(right);
	if (leftKeys.length !== rightKeys.length) {
		return false;
	}
	for (const key of leftKeys) {
		if (left[key] !== right[key]) {
			return false;
		}
	}
	return true;
}

function propsEqualNode(a: BoardNodeProps, b: BoardNodeProps): boolean {
	if (a.id !== b.id || a.x !== b.x || a.y !== b.y || a.draggable !== b.draggable || a.selected !== b.selected || a.style !== b.style || a.visible !== b.visible || a.text !== b.text) {
		return false;
	}
	if (!shallowEqualRecord(a.userData ?? {}, b.userData ?? {})) {
		return false;
	}
	if (nodeShapeSyncKey(a) !== nodeShapeSyncKey(b)) {
		return false;
	}
	if (a.shape === "rectangle" && b.shape === "rectangle") {
		return a.width === b.width && a.height === b.height;
	}
	return (a as { radius: number }).radius === (b as { radius: number }).radius;
}
//#endregion 🔖PropApply

//#region 🔖MountHelpers
function mountHandleUnderNode(renderer: BoardRenderer, nodeHost: BoardHostNode, handleHost: BoardHostHandle): void {
	if (handleHost.impl?.parent) {
		return;
	}
	nodeHost.handleChildren.add(handleHost);
	const impl = new BoardHandleObject({ ...handleHost.props, node: nodeHost.impl });
	handleHost.impl = impl;
	renderer.batch(() => {
		renderer.scene.add(impl);
	});
	renderer.invalidate();
}

function mountNode(renderer: BoardRenderer, nodeHost: BoardHostNode): void {
	if (nodeHost.impl.parent) {
		return;
	}
	renderer.batch(() => {
		renderer.scene.add(nodeHost.impl);
	});
	renderer.invalidate();
}

function mountEdge(renderer: BoardRenderer, edgeHost: BoardHostEdge): void {
	if (edgeHost.impl?.parent) {
		return;
	}
	const from = renderer.scene.getObjectById(edgeHost.props.from);
	const to = renderer.scene.getObjectById(edgeHost.props.to);
	if (!(from instanceof BoardHandleObject) || !(to instanceof BoardHandleObject)) {
		return;
	}
	renderer.batch(() => {
		if (!edgeHost.impl) {
			edgeHost.impl = new BoardEdgeObject({ ...edgeHost.props, from, to });
			renderer.scene.add(edgeHost.impl);
		} else {
			applyEdgeProps(edgeHost.impl, edgeHost.props, from, to);
		}
	});
	renderer.invalidate();
}

function replaceNodeImpl(renderer: BoardRenderer, host: BoardHostNode, nextProps: BoardNodeProps): void {
	if (instanceShapeSyncKey(host.impl) !== nodeShapeSyncKey(nextProps)) {
		renderer.batch(() => {
			for (const handleHost of host.handleChildren) {
				if (handleHost.impl?.parent) {
					renderer.scene.remove(handleHost.impl);
				}
				handleHost.impl = null;
			}
			renderer.scene.remove(host.impl);
			host.impl = newBoardNodeFromProps(nextProps);
			renderer.scene.add(host.impl);
			for (const handleHost of host.handleChildren) {
				mountHandleUnderNode(renderer, host, handleHost);
			}
		});
		renderer.invalidate();
		return;
	}
	renderer.batch(() => {
		applyNodeProps(host.impl, nextProps);
	});
	renderer.invalidate();
}

function isBoardRenderer(value: unknown): value is BoardRenderer {
	return value instanceof BoardRenderer;
}

function appendToBoardParent(parent: BoardRenderer | BoardHostInstance, child: BoardHostInstance): void {
	const renderer = child.renderer;
	if (isBoardRenderer(parent)) {
		if (child.kind === "node") {
			mountNode(renderer, child);
		} else if (child.kind === "edge") {
			mountEdge(renderer, child);
		}
		return;
	}
	if (parent.kind === "node" && child.kind === "handle") {
		mountHandleUnderNode(renderer, parent, child);
	}
}

function detachHandleFromNode(nodeHost: BoardHostNode, handleHost: BoardHostHandle): void {
	nodeHost.handleChildren.delete(handleHost);
}

const boardEmptyHostContext = Object.freeze({});
//#endregion 🔖MountHelpers

//#region 🔖Reconciler
const boardReconciler = Reconciler({
	...boardSchedulingHooks,
	supportsMutation: true,
	supportsPersistence: false,
	supportsHydration: false,
	isPrimaryRenderer: false,
	warnsIfNotActing: true,
	supportsMicrotasks: true,
	scheduleMicrotask: (fn: () => unknown) => queueMicrotask(fn),
	noTimeout: -1,
	scheduleTimeout: setTimeout,
	cancelTimeout: clearTimeout,

	getRootHostContext: () => boardEmptyHostContext,
	getChildHostContext: () => boardEmptyHostContext,

	createInstance(type, props, rootContainer) {
		const renderer = rootContainer;
		if (type === BOARD_HOST_NODE) {
			return { kind: "node", handleChildren: new Set(), impl: newBoardNodeFromProps(props as BoardNodeProps), renderer };
		}
		if (type === BOARD_HOST_HANDLE) {
			return { kind: "handle", impl: null, props: props as BoardHandleProps, renderer };
		}
		if (type === BOARD_HOST_EDGE) {
			return { kind: "edge", impl: null, props: props as BoardEdgeProps, renderer };
		}
		throw new Error(`Unknown board host type: ${String(type)}`);
	},

	createTextInstance() {
		throw new Error("Text children are not supported inside the board host tree.");
	},

	shouldSetTextContent: () => false,

	appendInitialChild(parent, child) {
		appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
	},

	appendChild(parent, child) {
		appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
	},

	appendChildToContainer(container, child) {
		if (child.kind === "node") {
			mountNode(container, child);
		} else if (child.kind === "edge") {
			mountEdge(container, child);
		}
	},

	insertBefore(parent, child, _beforeChild) {
		appendToBoardParent(parent as BoardRenderer | BoardHostInstance, child);
	},

	insertInContainerBefore(container, child, _beforeChild) {
		if (child.kind === "node") {
			mountNode(container, child);
		} else if (child.kind === "edge") {
			mountEdge(container, child);
		}
	},

	removeChild(parent, child) {
		const renderer = child.renderer;
		if (!isBoardRenderer(parent) && parent.kind === "node" && child.kind === "handle") {
			detachHandleFromNode(parent, child);
		}
		if (child.impl?.parent) {
			renderer.scene.remove(child.impl);
		}
		if (child.kind === "handle" || child.kind === "edge") {
			child.impl = null;
		}
		renderer.invalidate();
	},

	removeChildFromContainer(container, child) {
		if (child.kind === "node") {
			const nh = child as BoardHostNode;
			for (const h of [...nh.handleChildren]) {
				detachHandleFromNode(nh, h);
				if (h.impl?.parent) {
					container.scene.remove(h.impl);
				}
				h.impl = null;
			}
			nh.handleChildren.clear();
			if (nh.impl.parent) {
				container.scene.remove(nh.impl);
			}
			container.invalidate();
			return;
		}
		if (child.impl?.parent) {
			container.scene.remove(child.impl);
		}
		if (child.kind === "handle" || child.kind === "edge") {
			child.impl = null;
		}
		container.invalidate();
	},

	clearContainer(container) {
		container.scene.clear();
		container.invalidate();
	},

	finalizeInitialChildren() {
		return false;
	},

	getPublicInstance(instance) {
		return instance;
	},

	prepareForCommit() {
		return null;
	},
	resetAfterCommit() {},
	preparePortalMount() {},

	prepareUpdate(instance, type, oldProps, newProps) {
		if (type === BOARD_HOST_NODE) {
			return !propsEqualNode(oldProps as BoardNodeProps, newProps as BoardNodeProps);
		}
		if (type === BOARD_HOST_HANDLE) {
			return !propsEqualHandle(oldProps as BoardHandleProps, newProps as BoardHandleProps);
		}
		if (type === BOARD_HOST_EDGE) {
			return !propsEqualEdge(oldProps as BoardEdgeProps, newProps as BoardEdgeProps);
		}
		return false;
	},

	commitUpdate(instance, _payload, type, _oldProps, nextProps) {
		const renderer = instance.renderer;
		if (type === BOARD_HOST_NODE) {
			const next = nextProps as BoardNodeProps;
			const host = instance as BoardHostNode;
			if (instanceShapeSyncKey(host.impl) !== nodeShapeSyncKey(next)) {
				replaceNodeImpl(renderer, host, next);
				return;
			}
			renderer.batch(() => {
				applyNodeProps(host.impl, next);
			});
			renderer.invalidate();
			return;
		}
		if (type === BOARD_HOST_HANDLE) {
			const h = instance as BoardHostHandle;
			h.props = nextProps as BoardHandleProps;
			if (!h.impl) {
				return;
			}
			const parentNode = h.impl.node;
			renderer.batch(() => {
				applyHandleProps(h.impl!, h.props, parentNode);
			});
			renderer.invalidate();
			return;
		}
		if (type === BOARD_HOST_EDGE) {
			const e = instance as BoardHostEdge;
			e.props = nextProps as BoardEdgeProps;
			const from = renderer.scene.getObjectById(e.props.from);
			const to = renderer.scene.getObjectById(e.props.to);
			if (!(from instanceof BoardHandleObject) || !(to instanceof BoardHandleObject)) {
				return;
			}
			renderer.batch(() => {
				if (!e.impl) {
					e.impl = new BoardEdgeObject({ ...e.props, from, to });
					renderer.scene.add(e.impl);
				} else {
					applyEdgeProps(e.impl, e.props, from, to);
				}
			});
			renderer.invalidate();
		}
	},

	commitMount() {},

	detachDeletedInstance() {},

	getInstanceFromNode: () => null,
	beforeActiveInstanceBlur() {},
	afterActiveInstanceBlur() {},
	prepareScopeUpdate() {},
	getInstanceFromScope: () => null,

	getCurrentEventPriority: () => DefaultEventPriority,
	requestPaint() {},
} as never);

export type BoardFiberRoot = ReturnType<typeof boardReconciler.createContainer>;

/** @emoji 🌱 Creates a concurrent board reconciler root bound to {@link BoardRenderer}. */
export function createBoardFiberRoot(renderer: BoardRenderer): BoardFiberRoot {
	return boardReconciler.createContainer(
		renderer,
		ConcurrentRoot,
		null,
		false,
		null,
		"board:",
		undefined,
		undefined,
		undefined,
		undefined,
	);
}

/** @emoji 🔄 Schedules reconciler work and ties post-commit to {@link BoardRenderer.invalidate}. */
export function updateBoardFiberRoot(root: BoardFiberRoot, element: ReactElement | null, parent: null): void {
	boardReconciler.updateContainer(element, root, parent, () => {
		const renderer = root.containerInfo;
		renderer.invalidate();
	});
}

/** @emoji 🧹 Unmounts the board reconciler subtree without disposing {@link BoardRenderer}. */
export function unmountBoardFiberRoot(root: BoardFiberRoot): void {
	updateBoardFiberRoot(root, null, null);
}

export { boardReconciler };
//#endregion 🔖Reconciler
