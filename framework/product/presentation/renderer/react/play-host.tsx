// #region 🧲Header
/** @emoji 🛝 Presentation app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution, UiPanelHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { useApp, CommandBus, useControllerStore, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, cn, Icon, Button, floatingFieldSurfaceClass, floatingMenuSurfaceClass, shellChromeTitleClassName } from "@semio-tech/ui-react";
import * as React from "react";
import {
    FIGURE_TILE_PDF_PAGE_ASPECT,
    NORMALIZED_RECT_MIN_FRACTION,
    figureTileMediaKindFromFile,
    moveNormalizedRect,
    resizeNormalizedRect,
    type DispositionPosition,
    type FigureTileMediaKind,
    type FigureTileSource,
    type NormalizedRectHandle,
} from "@semio-tech/framework-presentation-core";
import {
    PRESENTATION_PLAY_CONTROLLER_ID,
    PRESENTATION_PLAY_ICON_DETAILS,
    PRESENTATION_PLAY_ICON_HIERARCHY,
    PRESENTATION_PLAY_IDLE_SNAPSHOT,
    PRESENTATION_PLAY_STORE_ID,
    PRESENTATION_PLAY_SURFACE_ID,
    PresentationPlayController,
    type PresentationPlaySnapshot,
    presentationPlayWindowBodies,
    presentationPlaySidePanelBodies,
} from "@semio-tech/framework-presentation-core";

const PRESENTATION_TILE_HANDLES: readonly NormalizedRectHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const PRESENTATION_TILE_VIEWPORT_MIN_ZOOM = 0.2;
const PRESENTATION_TILE_VIEWPORT_MAX_ZOOM = 12;
const PRESENTATION_FIGURE_FILE_ACCEPT =
	"image/*,video/*,application/pdf,.pdf,.svg,.png,.jpg,.jpeg,.webp,.gif,.bmp,.avif,.mp4,.webm,.ogg,.ogv,.mov,.m4v,.mkv";

function clampFigureTileZoom(zoom: number): number {
	return Math.min(PRESENTATION_TILE_VIEWPORT_MAX_ZOOM, Math.max(PRESENTATION_TILE_VIEWPORT_MIN_ZOOM, zoom));
}

interface FigureTileViewportState {
	readonly zoom: number;
	readonly panX: number;
	readonly panY: number;
}

interface FigureTileContentLayout {
	readonly width: number;
	readonly height: number;
	readonly offsetX: number;
	readonly offsetY: number;
}

function figureTileContentLayout(viewportWidth: number, viewportHeight: number, aspect: number): FigureTileContentLayout {
	if (viewportWidth <= 0 || viewportHeight <= 0) {
		return { width: 1, height: 1, offsetX: 0, offsetY: 0 };
	}
	const viewportAspect = viewportWidth / viewportHeight;
	if (viewportAspect >= aspect) {
		const height = viewportHeight;
		const width = height * aspect;
		return { width, height, offsetX: (viewportWidth - width) / 2, offsetY: 0 };
	}
	const width = viewportWidth;
	const height = width / aspect;
	return { width, height, offsetX: 0, offsetY: (viewportHeight - height) / 2 };
}

function figureTileZoomAtClient(
	viewport: FigureTileViewportState,
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	layout: FigureTileContentLayout,
	deltaScale: number,
): FigureTileViewportState {
	const nextZoom = clampFigureTileZoom(viewport.zoom * deltaScale);
	if (nextZoom === viewport.zoom) {
		return viewport;
	}
	const anchorX = clientX - viewportRect.left;
	const anchorY = clientY - viewportRect.top;
	const contentX = (anchorX - layout.offsetX - viewport.panX) / viewport.zoom;
	const contentY = (anchorY - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		zoom: nextZoom,
		panX: anchorX - layout.offsetX - contentX * nextZoom,
		panY: anchorY - layout.offsetY - contentY * nextZoom,
	};
}

function revokeFigureObjectUrl(url: string | null): void {
	if (url?.startsWith("blob:")) {
		URL.revokeObjectURL(url);
	}
}

function probeFigureTileMediaAspect(
	src: string,
	kind: FigureTileMediaKind,
): Promise<number> {
	if (kind === "video") {
		return new Promise((resolve, reject) => {
			const video = document.createElement("video");
			video.preload = "metadata";
			video.onloadedmetadata = () => {
				const aspect = video.videoWidth > 0 && video.videoHeight > 0 ? video.videoWidth / video.videoHeight : 16 / 9;
				resolve(aspect);
			};
			video.onerror = () => reject(new Error("video metadata"));
			video.src = src;
		});
	}
	if (kind === "pdf") {
		return Promise.resolve(FIGURE_TILE_PDF_PAGE_ASPECT);
	}
	return new Promise((resolve, reject) => {
		const img = new Image();
		img.onload = () => {
			const aspect = img.naturalWidth > 0 && img.naturalHeight > 0 ? img.naturalWidth / img.naturalHeight : 1;
			resolve(aspect);
		};
		img.onerror = () => reject(new Error("image metadata"));
		img.src = src;
	});
}

function FigureTileMediaPreview(props: { readonly source: FigureTileSource }): ReactElement {
	const { source } = props;
	const kind = source.kind ?? "figure";
	if (kind === "video") {
		return (
			<video
				className="pointer-events-none absolute inset-0 h-full w-full object-contain"
				src={source.src}
				muted
				playsInline
				preload="metadata"
				controls={false}
			/>
		);
	}
	if (kind === "pdf") {
		const page = source.pdfPage ?? 1;
		const pdfSrc = `${source.src}#page=${page}&view=FitH`;
		return <iframe className="pointer-events-none absolute inset-0 h-full w-full border-0 bg-background" src={pdfSrc} title="PDF preview" />;
	}
	return <img alt="" className="pointer-events-none absolute inset-0 h-full w-full object-contain" draggable={false} src={source.src} />;
}

function FigureSourcePicker(props: {
	readonly onPickFile: (file: File) => void;
}): ReactElement {
	const { onPickFile } = props;
	const fileInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const [dragActive, setDragActive] = reactHostPort.useState(false);

	const onInputChange = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			if (file) {
				onPickFile(file);
			}
			event.target.value = "";
		},
		[onPickFile],
	);

	const onDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(true);
	}, []);

	const onDragLeave = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(false);
	}, []);

	const onDrop = reactHostPort.useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			event.preventDefault();
			setDragActive(false);
			const file = event.dataTransfer.files?.[0];
			if (file) {
				onPickFile(file);
			}
		},
		[onPickFile],
	);

	return (
		<div
			className={cn(
				"flex min-h-0 flex-1 flex-col items-center justify-center gap-3 border-dashed p-6 text-center",
				floatingFieldSurfaceClass,
				dragActive && "border-primary",
			)}
			onDragLeave={onDragLeave}
			onDragOver={onDragOver}
			onDrop={onDrop}
		>
			<Icon icon="image-up" size="large" className="text-muted-foreground" />
			<div className="flex flex-col gap-1">
				<p className={shellChromeTitleClassName}>Pick figure media</p>
				<p className="text-muted-foreground text-xs">Image, SVG, video, or PDF — drag and drop or choose a file</p>
			</div>
			<Button id="presentation.play.pick-figure" type="button" variant="secondary" onClick={() => fileInputRef.current?.click()}>
				Choose file…
			</Button>
			<input accept={PRESENTATION_FIGURE_FILE_ACCEPT} className="hidden" onChange={onInputChange} ref={fileInputRef} type="file" />
		</div>
	);
}

function usePresentationPlayController(): PresentationPlayController | undefined {
	const { runtime } = useApp();
	return runtime.getActiveApp()?.controller as PresentationPlayController | undefined;
}

function usePresentationPlaySnapshot(): PresentationPlaySnapshot {
	const ctrl = usePresentationPlayController();
	return useControllerStore(ctrl, PRESENTATION_PLAY_STORE_ID) ?? PRESENTATION_PLAY_IDLE_SNAPSHOT;
}

function clampUnit(value: number): number {
	return Math.min(1, Math.max(0, value));
}

function normalizedPointFromClient(
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	viewport: FigureTileViewportState,
	layout: FigureTileContentLayout,
): { readonly x: number; readonly y: number } {
	const localX = (clientX - viewportRect.left - layout.offsetX - viewport.panX) / viewport.zoom;
	const localY = (clientY - viewportRect.top - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		x: clampUnit(localX / layout.width),
		y: clampUnit(localY / layout.height),
	};
}

function normalizedRectFromDrag(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): DispositionPosition {
	const x = Math.min(start.x, end.x);
	const y = Math.min(start.y, end.y);
	const width = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.x - start.x));
	const height = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.y - start.y));
	return {
		x: clampUnit(x),
		y: clampUnit(y),
		width: Math.min(width, 1 - x),
		height: Math.min(height, 1 - y),
	};
}

function FigureTilesSurfaceHost({ node }: { readonly node: UiPanelHostSurfaceNode }): ReactElement {
	const { runtime } = useApp();
	const controller = usePresentationPlayController();
	const snapshot = usePresentationPlaySnapshot();
	const viewportRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const contentRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const figureObjectUrlRef = reactHostPort.useRef<string | null>(null);
	const spacePressedRef = reactHostPort.useRef(false);
	const [viewportSize, setViewportSize] = reactHostPort.useState({ width: 0, height: 0 });
	const [viewport, setViewport] = reactHostPort.useState<FigureTileViewportState>({ zoom: 1, panX: 0, panY: 0 });
	const [spacePressed, setSpacePressed] = reactHostPort.useState(false);
	const [isPanning, setIsPanning] = reactHostPort.useState(false);
	const [marquee, setMarquee] = reactHostPort.useState<{ readonly start: { readonly x: number; readonly y: number }; readonly end: { readonly x: number; readonly y: number } } | null>(null);
	const dragRef = reactHostPort.useRef<
		| {
				readonly kind: "move" | NormalizedRectHandle | "marquee" | "pan";
				readonly tileId?: string;
				readonly startClient: { readonly x: number; readonly y: number };
				readonly startCrop?: DispositionPosition;
				readonly marqueeStart?: { readonly x: number; readonly y: number };
				readonly startPan?: { readonly x: number; readonly y: number };
		  }
		| null
	>(null);

	reactHostPort.useEffect(() => {
		if (!snapshot.clipboardPrompt || snapshot.clipboardEpoch <= 0) {
			return;
		}
		void navigator.clipboard?.writeText(snapshot.clipboardPrompt).catch(() => undefined);
	}, [snapshot.clipboardEpoch, snapshot.clipboardPrompt]);

	const dispatch = reactHostPort.useCallback(
		(command: string, args?: unknown) => {
			if (!controller) {
				return;
			}
			runtime.commandBus.dispatch(controller.id, command, args);
		},
		[controller, runtime.commandBus],
	);

	reactHostPort.useEffect(() => () => revokeFigureObjectUrl(figureObjectUrlRef.current), []);

	reactHostPort.useEffect(() => {
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, [snapshot.source.src]);

	reactHostPort.useEffect(() => {
		const onKeyDown = (event: KeyboardEvent) => {
			if (event.code !== "Space" || event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
				return;
			}
			event.preventDefault();
			spacePressedRef.current = true;
			setSpacePressed(true);
		};
		const onKeyUp = (event: KeyboardEvent) => {
			if (event.code !== "Space") {
				return;
			}
			spacePressedRef.current = false;
			setSpacePressed(false);
		};
		window.addEventListener("keydown", onKeyDown);
		window.addEventListener("keyup", onKeyUp);
		return () => {
			window.removeEventListener("keydown", onKeyDown);
			window.removeEventListener("keyup", onKeyUp);
		};
	}, []);

	const applyFigureFile = reactHostPort.useCallback(
		(file: File) => {
			const kind = figureTileMediaKindFromFile(file.type, file.name);
			if (!kind) {
				return;
			}
			revokeFigureObjectUrl(figureObjectUrlRef.current);
			const url = URL.createObjectURL(file);
			figureObjectUrlRef.current = url;
			void probeFigureTileMediaAspect(url, kind)
				.then((sourceAspect) => {
					dispatch("setSource", {
						src: url,
						kind,
						sourceAspect,
						...(kind === "pdf" ? { pdfPage: 1 } : {}),
					});
				})
				.catch(() => {
					revokeFigureObjectUrl(url);
					if (figureObjectUrlRef.current === url) {
						figureObjectUrlRef.current = null;
					}
				});
		},
		[dispatch],
	);

	const hasFigure = snapshot.source.src.trim().length > 0;
	const aspect = snapshot.source.sourceAspect ?? 1;
	const contentLayout = reactHostPort.useMemo(
		() => figureTileContentLayout(viewportSize.width, viewportSize.height, aspect),
		[aspect, viewportSize.height, viewportSize.width],
	);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const observer = new ResizeObserver(([entry]) => {
			const { width, height } = entry.contentRect;
			setViewportSize({ width, height });
		});
		observer.observe(element);
		return () => observer.disconnect();
	}, [hasFigure]);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const onWheel = (event: WheelEvent) => {
			event.preventDefault();
			const rect = element.getBoundingClientRect();
			const layout = figureTileContentLayout(viewportSize.width, viewportSize.height, aspect);
			const deltaScale = event.deltaY < 0 ? 1.1 : 1 / 1.1;
			setViewport((current) => figureTileZoomAtClient(current, event.clientX, event.clientY, rect, layout, deltaScale));
		};
		element.addEventListener("wheel", onWheel, { passive: false });
		return () => element.removeEventListener("wheel", onWheel);
	}, [aspect, hasFigure, viewportSize.height, viewportSize.width]);

	const viewportPoint = reactHostPort.useCallback(
		(clientX: number, clientY: number) => {
			const rect = viewportRef.current?.getBoundingClientRect();
			if (!rect) {
				return { x: 0, y: 0 };
			}
			return normalizedPointFromClient(clientX, clientY, rect, viewport, contentLayout);
		},
		[contentLayout, viewport],
	);

	const onContentPointerDown = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!viewportRef.current) {
				return;
			}
			const target = event.target as HTMLElement;
			if (target.dataset.tileHandle || target.dataset.tileId) {
				return;
			}
			if (event.button === 1 || (event.button === 0 && (spacePressedRef.current || event.altKey))) {
				dragRef.current = {
					kind: "pan",
					startClient: { x: event.clientX, y: event.clientY },
					startPan: { x: viewport.panX, y: viewport.panY },
				};
				setIsPanning(true);
				event.currentTarget.setPointerCapture(event.pointerId);
				return;
			}
			if (event.button !== 0) {
				return;
			}
			const point = viewportPoint(event.clientX, event.clientY);
			dragRef.current = {
				kind: "marquee",
				startClient: { x: event.clientX, y: event.clientY },
				marqueeStart: point,
			};
			setMarquee({ start: point, end: point });
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[viewport.panX, viewport.panY, viewportPoint],
	);

	const onTilePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition) => (event: React.PointerEvent) => {
			event.stopPropagation();
			if (spacePressedRef.current || event.altKey) {
				return;
			}
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: "move",
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onHandlePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition, handle: NormalizedRectHandle) => (event: React.PointerEvent) => {
			event.stopPropagation();
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: handle,
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onPointerMove = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "pan" && drag.startPan) {
				setViewport((current) => ({
					...current,
					panX: drag.startPan!.x + (event.clientX - drag.startClient.x),
					panY: drag.startPan!.y + (event.clientY - drag.startClient.y),
				}));
				return;
			}
			const scaleX = contentLayout.width * viewport.zoom;
			const scaleY = contentLayout.height * viewport.zoom;
			const dx = scaleX > 0 ? (event.clientX - drag.startClient.x) / scaleX : 0;
			const dy = scaleY > 0 ? (event.clientY - drag.startClient.y) / scaleY : 0;
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				setMarquee({ start: drag.marqueeStart, end });
				return;
			}
			if (!drag.tileId || !drag.startCrop) {
				return;
			}
			const nextCrop =
				drag.kind === "move"
					? moveNormalizedRect(drag.startCrop, dx, dy)
					: resizeNormalizedRect(drag.startCrop, drag.kind, dx, dy);
			dispatch("setTileCrop", { id: drag.tileId, crop: nextCrop });
		},
		[contentLayout.height, contentLayout.width, dispatch, viewport.zoom, viewportPoint],
	);

	const onPointerUp = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				const crop = normalizedRectFromDrag(drag.marqueeStart, end);
				dispatch("addTile", { crop });
				setMarquee(null);
			}
			if (drag.kind === "pan") {
				setIsPanning(false);
			}
			dragRef.current = null;
			try {
				event.currentTarget.releasePointerCapture(event.pointerId);
			} catch {
				// pointer already released
			}
		},
		[dispatch, viewportPoint],
	);

	const onViewportDoubleClick = reactHostPort.useCallback((event: React.MouseEvent<HTMLDivElement>) => {
		const target = event.target as HTMLElement;
		if (target.dataset.tileHandle || target.dataset.tileId) {
			return;
		}
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, []);

	if (node.controllerId !== PRESENTATION_PLAY_CONTROLLER_ID || node.surfaceId !== PRESENTATION_PLAY_SURFACE_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid presentation tile surface binding</div>;
	}

	if (!hasFigure) {
		return (
			<div className="flex h-full min-h-0 w-full p-2">
				<FigureSourcePicker onPickFile={applyFigureFile} />
			</div>
		);
	}

	const viewportCursor = isPanning ? "grabbing" : spacePressed ? "grab" : undefined;

	return (
		<div className="flex h-full min-h-0 w-full flex-col">
			<div ref={viewportRef} className="relative min-h-0 flex-1 overflow-hidden bg-muted/30" style={{ cursor: viewportCursor }}>
				<div
					ref={contentRef}
					className="absolute touch-none select-none"
					style={{
						left: contentLayout.offsetX,
						top: contentLayout.offsetY,
						width: contentLayout.width,
						height: contentLayout.height,
						transform: `translate(${viewport.panX}px, ${viewport.panY}px) scale(${viewport.zoom})`,
						transformOrigin: "0 0",
					}}
					onPointerDown={onContentPointerDown}
					onPointerMove={onPointerMove}
					onPointerUp={onPointerUp}
					onPointerCancel={onPointerUp}
					onDoubleClick={onViewportDoubleClick}
				>
					<FigureTileMediaPreview source={snapshot.source} />
					{snapshot.tiles.map((tile) => {
						const selected = snapshot.selectedIds.includes(tile.id);
						return (
							<div
								key={tile.id}
								data-tile-id={tile.id}
								className={cn(
									"absolute box-border cursor-move border-2",
									selected ? "border-primary bg-primary/20" : "border-accent bg-accent/10",
								)}
								style={{
									left: `${tile.crop.x * 100}%`,
									top: `${tile.crop.y * 100}%`,
									width: `${tile.crop.width * 100}%`,
									height: `${tile.crop.height * 100}%`,
								}}
								onPointerDown={onTilePointerDown(tile.id, tile.crop)}
							>
								<span className={cn("pointer-events-none absolute left-0 top-0 max-w-full truncate px-1 text-2xs", floatingMenuSurfaceClass)}>{tile.name}</span>
								{selected
									? PRESENTATION_TILE_HANDLES.map((handle) => (
											<button
												key={handle}
												type="button"
												data-tile-handle={handle}
												className="bg-primary absolute z-10 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full border border-background"
												style={{
													left: handle.includes("w") ? "0%" : handle.includes("e") ? "100%" : "50%",
													top: handle.includes("n") ? "0%" : handle.includes("s") ? "100%" : "50%",
													cursor: `${handle}-resize`,
												}}
												onPointerDown={onHandlePointerDown(tile.id, tile.crop, handle)}
											/>
										))
									: null}
							</div>
						);
					})}
					{marquee ? (
						<div
							className="border-primary/80 bg-primary/10 pointer-events-none absolute border border-dashed"
							style={{
								left: `${Math.min(marquee.start.x, marquee.end.x) * 100}%`,
								top: `${Math.min(marquee.start.y, marquee.end.y) * 100}%`,
								width: `${Math.abs(marquee.end.x - marquee.start.x) * 100}%`,
								height: `${Math.abs(marquee.end.y - marquee.start.y) * 100}%`,
							}}
						/>
					) : null}
				</div>
			</div>
		</div>
	);
}

/** @emoji 🛝 Presentation app renderer for playground and OS shells. */
export const presentationAppRenderer: AppRendererContribution = {
  windowBodies: presentationPlayWindowBodies,
  sidePanelBodies: presentationPlaySidePanelBodies,
  surfaceHosts: {
    [PRESENTATION_PLAY_SURFACE_ID]: FigureTilesSurfaceHost,
  },
  tabIcons: {
    [PRESENTATION_PLAY_ICON_HIERARCHY]: "list-tree",
    [PRESENTATION_PLAY_ICON_DETAILS]: "clipboard-list",
  },
  examples: controllerBackedExampleContribution(PRESENTATION_PLAY_CONTROLLER_ID, []),
};