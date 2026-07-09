import { useCallback, useEffect, useLayoutEffect, useRef } from "react";
import { useCanvasThemeSync } from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import type { CommandDescriptor, Puzzle2dBoardScene, Puzzle2dBoardWasmSession, UiComponentSceneNode } from "../os-shell.tsx";
import { createPuzzle2dBoardSession } from "../os-shell.tsx";

//#region Types
type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
//#endregion Types

//#region Parsing
function parseBoardCamera(json: string): BoardCamera | null {
	try {
		const parsed = JSON.parse(json) as Partial<BoardCamera>;
		if (typeof parsed.x !== "number" || typeof parsed.y !== "number" || typeof parsed.zoom !== "number") return null;
		return { x: parsed.x, y: parsed.y, zoom: parsed.zoom };
	} catch {
		return null;
	}
}

export function puzzle2dBoardCameraCommandArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
	const camera = parseBoardCamera(cameraJson);
	return camera ? { camera } : null;
}
//#endregion Parsing

//#region Sync
function syncBoardSessionFromScene(session: Puzzle2dBoardWasmSession, scene: Puzzle2dBoardScene): void {
	try {
		session.parseFixtureJson(scene.fixtureJson);
		session.setKindCatalogsJson(scene.kindCatalogsJson);
		session.setSelectionIdsJson(scene.selectionJson);
		const camera = parseBoardCamera(scene.cameraJson);
		if (camera) session.setCamera(camera.x, camera.y, camera.zoom);
	} catch {
		/* session not ready */
	}
}
//#endregion Sync

//#region Puzzle2dBoardHost
export function Puzzle2dBoardHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.puzzle2dBoard;
	const containerRef = useRef<HTMLDivElement>(null);
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const sessionRef = useRef<Puzzle2dBoardWasmSession | null>(null);
	const sceneSignature = scene ? JSON.stringify(scene) : "";

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId: node.controllerId, command, args: { surfaceId: node.surfaceId, ...args } });
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const drainBoardEvents = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			const eventsJson = session.drainEventsJson();
			if (eventsJson && eventsJson !== "[]") {
				dispatch("applyBoardEvents", { eventsJson });
			}
		} catch {
			/* session not ready */
		}
	}, [dispatch]);

	const readContainerSize = useCallback((): { w: number; h: number } => {
		const container = containerRef.current;
		if (!container) return { w: 1, h: 1 };
		const rect = container.getBoundingClientRect();
		return {
			w: Math.max(1, Math.round(rect.width || container.clientWidth)),
			h: Math.max(1, Math.round(rect.height || container.clientHeight)),
		};
	}, []);

	useLayoutEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container || !scene) return;
		let disposed = false;
		let resizeObserver: ResizeObserver | null = null;
		let raf = 0;

		void createPuzzle2dBoardSession().then((session) => {
			if (disposed) {
				session.free();
				return;
			}
			sessionRef.current = session;

			const applySize = (): void => {
				const nextDpr = globalThis.devicePixelRatio || 1;
				const { w, h } = readContainerSize();
				session.setSize(w, h, nextDpr);
			};

			const boot = async (): Promise<void> => {
				let { w, h } = readContainerSize();
				for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
					await new Promise<void>((resolve) => {
						if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
						else queueMicrotask(resolve);
					});
					if (disposed) return;
					({ w, h } = readContainerSize());
				}
				const dpr = globalThis.devicePixelRatio || 1;
				await session.attach_canvas(canvas, w, h, dpr);
				if (disposed) {
					session.free();
					return;
				}
				applySize();
				syncBoardSessionFromScene(session, scene);
				syncSessionCanvasTheme(session);
				const tick = () => {
					if (disposed) return;
					try {
						session.renderFrame();
					} catch {
						/* gpu not ready */
					}
					raf = requestAnimationFrame(tick);
				};
				raf = requestAnimationFrame(tick);
			};

			resizeObserver =
				typeof ResizeObserver === "undefined"
					? null
					: new ResizeObserver(() => {
							applySize();
						});
			resizeObserver?.observe(container);
			void boot();
		});

		return () => {
			disposed = true;
			resizeObserver?.disconnect();
			if (raf) cancelAnimationFrame(raf);
			sessionRef.current?.free();
			sessionRef.current = null;
		};
	}, [readContainerSize, scene?.fixtureJson, scene?.interactive]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session || !scene) return;
		syncBoardSessionFromScene(session, scene);
		try {
			session.renderFrame();
		} catch {
			/* gpu not ready */
		}
	}, [sceneSignature, scene]);

	useCanvasThemeSync(() => {
		syncSessionCanvasTheme(sessionRef.current);
		try {
			sessionRef.current?.renderFrame();
		} catch {
			/* gpu not ready */
		}
	});

	useEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container || !scene?.interactive) return undefined;

		const clientToLocal = (clientX: number, clientY: number): { x: number; y: number } => {
			const rect = canvas.getBoundingClientRect();
			return { x: clientX - rect.left, y: clientY - rect.top };
		};

		const onPointerDown = (event: PointerEvent): void => {
			event.stopPropagation();
			const session = sessionRef.current;
			if (!session) return;
			const point = clientToLocal(event.clientX, event.clientY);
			if (event.button === 0 || event.button === 1) {
				canvas.setPointerCapture?.(event.pointerId);
			}
			session.pointerDownScreen(
				point.x,
				point.y,
				event.button,
				event.shiftKey,
				event.metaKey || event.ctrlKey,
			);
			try {
				session.renderFrame();
			} catch {
				/* gpu not ready */
			}
		};

		const onPointerMove = (event: PointerEvent): void => {
			const session = sessionRef.current;
			if (!session) return;
			const point = clientToLocal(event.clientX, event.clientY);
			session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
			try {
				session.renderFrame();
			} catch {
				/* gpu not ready */
			}
		};

		const onPointerUp = (event: PointerEvent): void => {
			const session = sessionRef.current;
			if (!session) return;
			const point = clientToLocal(event.clientX, event.clientY);
			session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
			if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
			try {
				session.renderFrame();
			} catch {
				/* gpu not ready */
			}
			drainBoardEvents();
		};

		const onWheel = (event: WheelEvent): void => {
			event.preventDefault();
			event.stopPropagation();
			const session = sessionRef.current;
			if (!session) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const delta =
				event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
			session.wheelScreen(point.x, point.y, delta);
			try {
				session.renderFrame();
			} catch {
				/* gpu not ready */
			}
			const cameraArgs = puzzle2dBoardCameraCommandArgs(session.cameraJson());
			if (cameraArgs) dispatch("setCamera", cameraArgs);
			drainBoardEvents();
		};

		canvas.addEventListener("pointerdown", onPointerDown);
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp);
		container.addEventListener("wheel", onWheel, { passive: false });
		return () => {
			canvas.removeEventListener("pointerdown", onPointerDown);
			window.removeEventListener("pointermove", onPointerMove);
			window.removeEventListener("pointerup", onPointerUp);
			container.removeEventListener("wheel", onWheel);
		};
	}, [drainBoardEvents, scene?.interactive]);

	if (!scene) return <div className="semio-puzzle2d-board-empty text-muted-foreground p-2 text-xs">No puzzle board scene</div>;

	return (
		<div
			ref={containerRef}
			className="semio-puzzle2d-board-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
			data-surface-id={node.surfaceId}
			style={{ touchAction: "none" }}
		>
			<canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
		</div>
	);
}
//#endregion Puzzle2dBoardHost
