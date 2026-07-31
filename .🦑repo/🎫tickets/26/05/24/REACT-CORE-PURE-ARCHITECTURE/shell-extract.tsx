// Domain-neutral composite component providing a full application shell.
// An app has window kinds (rendered with golden-layout) and registers
// left/right side panel tabs and footer items.
// Every UI has a toolbar, a search (Ctrl+P command palette), panel toggles, and breadcrumb.
// Every app has a find (Ctrl+F scoped command palette).
// Every panel has a tree.

/**
 * Window kind classification for app windows.
 **/
export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
  SETTINGS = "settings",
  CHAT = "chat",
  WORKBENCH = "workbench",
  VEC_INPUT = "vec-input",
  /**3D placement deltas (gap, shift, rise) for move algorithms; not the 2D vec pad. */
  VECTOR_INPUT = "vector-input",
  PIECES_SELECTION_INPUT = "pieces-selection-input",
  SELECTION_INPUT = "selection-input",
  DESIGN_INPUT = "design-input",
  DESIGN_DIFF_OUTPUT = "design-diff-output",
  DESIGN_OUTPUT = "design-output",
}

/**
 * UI theme classification.
 **/
export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

/**
 * UI interaction mode.
 **/
export enum Mode {
  VIEW = "view",
  EDIT = "edit",
}

/**
 * A window control with kind, ID, icon, options, and change handler.
 **/
export interface UIWindowControl {
  kind: "toggle" | "dropdown";
  id: string;
  icon?: React.ReactNode;
  value?: string;
  options?: {
    id: string;
    value: string;
    icon?: React.ReactNode;
  }[];
  onChange?: (value: string) => void;
}

/**
 * 📐 Declarative `measure` entries for a window: read-only readouts (`display`, `reading`) or interactive controls; rendered as compact floats on the right.
 **/
export type UIWindowMeasure =
  | { kind: "display"; id: string; label?: string; content: React.ReactNode }
  | { kind: "reading"; id: string; label?: string; text: string; monospace?: boolean }
  | { kind: "section"; id: string; title: string }
  | { kind: "separator"; id: string }
  | { kind: "toggle"; id: string; label?: string; pressed?: boolean; defaultPressed?: boolean; icon?: React.ReactNode; text?: string; onPressedChange?: (pressed: boolean) => void }
  | { kind: "select"; id: string; label?: string; value?: string; defaultValue?: string; items: { id: string; value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "combobox"; id: string; label?: string; value?: string; placeholder?: string; choices: { value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "button"; id: string; label?: string; text: string; icon?: React.ReactNode; onClick?: () => void }
  | { kind: "buttonCycle"; id: string; label?: string; value?: string; items: { value: string; label: string; icon?: React.ReactNode; text?: string; id?: string }[]; onValueChange?: (value: string) => void }
  | { kind: "input"; id: string; label?: string; value?: string; placeholder?: string; onLazyChange?: (value: string) => void }
  | { kind: "textarea"; id: string; label?: string; value?: string; placeholder?: string; rows?: number; onLazyChange?: (value: string) => void }
  | { kind: "checkbox"; id: string; label?: string; checked?: boolean; defaultChecked?: boolean; onCheckedChange?: (checked: boolean) => void }
  | { kind: "radio"; id: string; label?: string; value: string; items: { value: string; label: string }[]; onChange?: (value: string) => void }
  | { kind: "slider"; id: string; label?: string; value?: number; min?: number; max?: number; step?: number; onValueChange?: (value: number) => void }
  | { kind: "number"; id: string; label?: string; value?: number; min?: number; max?: number; step?: number; onChange?: (value: number) => void }
  | { kind: "color"; id: string; label?: string; value?: string; onChange?: (value: string) => void };

/**
 * Definition of a window kind with label, icon, component, controls, and optional floating window measures.
 * Each app registers the window kinds it can render.
 **/
export interface UIWindowKindDefinition {
  id: string;
  label?: string;
  icon?: React.ReactNode;
  component: React.ComponentType<any>;
  controls?: UIWindowControl[];
  measures?: UIWindowMeasure[];
  contextMenu?: ContextMenuItem[];
  variants?: {
    id: string;
    icon?: React.ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

/**
 * A single window entry in the abstract UI layout tree.
 **/
export interface UIWindowLayoutWindowNode {
  kind: "window";
  windowKindId: string;
  title?: string;
}

/**
 * A tab stack in the abstract UI layout tree.
 **/
export interface UIWindowLayoutStackNode {
  kind: "stack";
  size?: number;
  children: UIWindowLayoutWindowNode[];
}

/**
 * A row or column branch in the abstract UI layout tree.
 **/
export interface UIWindowLayoutAxisNode {
  kind: "row" | "column";
  size?: number;
  children: Array<UIWindowLayoutAxisNode | UIWindowLayoutStackNode>;
}

/**
 * Root layout wrapper owned by an app instead of the Golden Layout runtime.
 **/
export interface UIWindowLayout {
  root: UIWindowLayoutAxisNode | UIWindowLayoutStackNode;
}

/**
 * Union of supported abstract UI layout nodes.
 **/
export type UIWindowLayoutNode = UIWindowLayout["root"];

/**
 * Alias for UIWindowLayout used by the sketchpad layer.
 **/
export type LayoutNode = UIWindowLayout;

/**
 * Alias for UIWindowLayoutStackNode used by the sketchpad layer.
 **/
export type LayoutStack = UIWindowLayoutStackNode;

/**
 * Alias for UIWindowLayoutAxisNode with kind "row" used by the sketchpad layer.
 **/
export type LayoutRow = UIWindowLayoutAxisNode & { kind: "row" };

/**
 * Alias for UIWindowLayoutAxisNode with kind "column" used by the sketchpad layer.
 **/
export type LayoutColumn = UIWindowLayoutAxisNode & { kind: "column" };

function isWindowLayoutWindowNode(value: unknown): value is UIWindowLayoutWindowNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<UIWindowLayoutWindowNode>;
  return candidate.kind === "window" && typeof candidate.windowKindId === "string";
}

function isWindowLayoutStackNode(value: unknown): value is UIWindowLayoutStackNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<UIWindowLayoutStackNode>;
  return candidate.kind === "stack" && Array.isArray(candidate.children) && candidate.children.every(isWindowLayoutWindowNode);
}

function isWindowLayoutAxisNode(value: unknown): value is UIWindowLayoutAxisNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<UIWindowLayoutAxisNode>;
  return (candidate.kind === "row" || candidate.kind === "column") && Array.isArray(candidate.children) && candidate.children.every((child) => isWindowLayoutAxisNode(child) || isWindowLayoutStackNode(child));
}

function isWindowLayout(value: unknown): value is UIWindowLayout {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<UIWindowLayout>;
  return isWindowLayoutAxisNode(candidate.root) || isWindowLayoutStackNode(candidate.root);
}

function convertLegacyGoldenNodeToWindowLayoutNode(value: unknown): UIWindowLayoutNode | UIWindowLayoutWindowNode | undefined {
  if (!value || typeof value !== "object") return undefined;
  const node = value as Record<string, unknown>;

  if (node.type === "component") {
    const componentName = typeof node.componentName === "string" ? node.componentName : undefined;
    if (!componentName) return undefined;
    return createWindowLayout(componentName, typeof node.title === "string" ? node.title : componentName);
  }

  if (node.type === "stack") {
    const children = Array.isArray(node.content) ? node.content.map(convertLegacyGoldenNodeToWindowLayoutNode).filter(isWindowLayoutWindowNode) : [];
    if (children.length === 0) return undefined;
    return {
      kind: "stack",
      ...(typeof node.size === "string" ? { size: Number.parseFloat(node.size) } : typeof node.size === "number" ? { size: node.size } : {}),
      children,
    };
  }

  if (node.type === "row" || node.type === "column") {
    const children = Array.isArray(node.content)
      ? node.content.map(convertLegacyGoldenNodeToWindowLayoutNode).filter((child): child is UIWindowLayoutAxisNode | UIWindowLayoutStackNode => isWindowLayoutAxisNode(child) || isWindowLayoutStackNode(child))
      : [];
    if (children.length === 0) return undefined;
    return {
      kind: node.type,
      ...(typeof node.size === "string" ? { size: Number.parseFloat(node.size) } : typeof node.size === "number" ? { size: node.size } : {}),
      children,
    };
  }

  return undefined;
}

/**
 * Parses a window layout from a string, object, or undefined input.
 * MUST return undefined for null, empty, or unparseable inputs.
 **/
export function parseWindowLayout(layout: unknown): UIWindowLayout | undefined {
  if (layout === undefined || layout === null) return undefined;
  if (typeof layout === "string") {
    const trimmed = layout.trim();
    if (!trimmed) return undefined;
    try {
      return parseWindowLayout(JSON.parse(trimmed));
    } catch {
      return undefined;
    }
  }
  if (isWindowLayout(layout)) return layout;
  if (typeof layout === "object") {
    const candidate = layout as Record<string, unknown>;
    const legacyRoot = convertLegacyGoldenNodeToWindowLayoutNode(candidate.root);
    if (legacyRoot && (isWindowLayoutAxisNode(legacyRoot) || isWindowLayoutStackNode(legacyRoot))) {
      return { root: legacyRoot };
    }
  }
  return undefined;
}

/**
 * Serializes a window layout to a JSON string.
 * MUST return undefined when serialization fails.
 **/
export function stringifyWindowLayout(layout: unknown): string | undefined {
  const parsedLayout = parseWindowLayout(layout);
  if (!parsedLayout) return undefined;
  try {
    return JSON.stringify(parsedLayout);
  } catch {
    return undefined;
  }
}

/**
 * Removes duplicate and disallowed window components from a layout.
 **/
export function deduplicateWindowLayout(layout: unknown, allowedWindowIds: string[]): UIWindowLayout | undefined {
  const parsedLayout = parseWindowLayout(layout);
  if (!parsedLayout) return undefined;

  const seenComponents = new Set<string>();

  const deduplicateNode = (node: UIWindowLayoutNode): UIWindowLayoutNode | undefined => {
    if (node.kind === "stack") {
      const children = node.children.filter((child) => {
        if (seenComponents.has(child.windowKindId) || !allowedWindowIds.includes(child.windowKindId)) return false;
        seenComponents.add(child.windowKindId);
        return true;
      });

      if (children.length === 0) return undefined;
      return { ...node, children };
    }

    const children = node.children.map((child) => deduplicateNode(child)).filter((child): child is UIWindowLayoutAxisNode | UIWindowLayoutStackNode => Boolean(child));

    if (children.length === 0) return undefined;
    return { ...node, children };
  };

  const deduplicatedRoot = deduplicateNode(parsedLayout.root);
  if (!deduplicatedRoot || isWindowLayoutWindowNode(deduplicatedRoot)) return undefined;
  return { root: deduplicatedRoot };
}

function convertWindowLayoutNodeToGoldenConfig(node: UIWindowLayoutNode): Record<string, unknown> {
  if (node.kind === "stack") {
    return {
      type: "stack",
      ...(node.size !== undefined ? { size: `${node.size}%` } : {}),
      content: node.children.map((child) => ({
        type: "component",
        componentName: child.windowKindId,
        title: child.title ?? child.windowKindId,
        componentState: {},
      })),
    };
  }

  return {
    type: node.kind,
    ...(node.size !== undefined ? { size: `${node.size}%` } : {}),
    content: node.children.map((child) => convertWindowLayoutNodeToGoldenConfig(child)),
  };
}

function convertWindowLayoutToGoldenConfig(layout: UIWindowLayout): Record<string, unknown> {
  return { root: convertWindowLayoutNodeToGoldenConfig(layout.root) };
}

/**
 * Alias for convertWindowLayoutToGoldenConfig used by the sketchpad layer.
 **/
export function layoutNodeToGoldenLayoutConfig(layout: UIWindowLayout): Record<string, unknown> {
  return convertWindowLayoutToGoldenConfig(layout);
}

/**
 * Window controls group component rendering toggle and dropdown controls.
 **/
const UIWindowControlsGroup: React.FC<{ controls: UIWindowControl[] }> = ({ controls }) => (
  <ActionGroup id="window-controls-group">
    {controls.map((control) => {
      if (control.kind === "toggle") {
        return (
          <ActionGroupItem key={control.id} id={control.id} onClick={() => control.onChange?.(control.value === "on" ? "off" : "on")}>
            {control.icon}
          </ActionGroupItem>
        );
      }
      return (
        <ActionGroupItem key={control.id} id={control.id}>
          {control.icon}
        </ActionGroupItem>
      );
    })}
  </ActionGroup>
);

// #region 🪟WindowMeasuresOverlay

const UIWindowMeasureFloat: React.FC<{ measureId: string; label?: string; children: React.ReactNode }> = ({ measureId, label, children }) => (
  <div data-slot="window-measure-float" data-measure-id={measureId} className="border-element/80 bg-window/90 max-w-[11rem] min-w-0 rounded-md border px-single py-half shadow-md backdrop-blur-sm">
    {label ? <span className="text-muted-foreground mb-half block max-w-full truncate text-[10px] font-semibold uppercase tracking-wide">{label}</span> : null}
    <div className="min-w-0 w-full">{children}</div>
  </div>
);

/**
 * 📐 Maps declarative `UIWindowMeasure` entries into compact floating tiles aligned to the right edge.
 **/
export const UIWindowMeasures: React.FC<{ measures: UIWindowMeasure[] }> = ({ measures }) => (
  <div data-slot="window-measures-stack-inner" className="flex flex-col items-end gap-half">
    {measures.map((measure) => {
      switch (measure.kind) {
        case "display":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className="text-foreground max-w-full text-xs leading-snug break-words">{measure.content}</div>
            </UIWindowMeasureFloat>
          );
        case "reading":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className={cn("text-foreground text-xs tabular-nums", measure.monospace && "font-mono")}>{measure.text}</div>
            </UIWindowMeasureFloat>
          );
        case "section":
          return (
            <div key={measure.id} data-slot="window-measure-heading" className="border-element/60 bg-window/85 max-w-[11rem] rounded-md border px-single py-tiny text-center shadow-sm backdrop-blur-sm">
              <span className="text-muted-foreground text-[10px] font-semibold uppercase tracking-wide">{measure.title}</span>
            </div>
          );
        case "separator":
          return <div key={measure.id} data-slot="window-measure-separator" className="bg-muted-foreground/35 my-half h-px w-8 shrink-0 rounded-full" aria-hidden />;
        case "toggle":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Toggle id={measure.id} pressed={measure.pressed} defaultPressed={measure.defaultPressed} onPressedChange={measure.onPressedChange} icon={measure.icon ?? <CheckIcon className="size-small" />} text={measure.text} />
            </UIWindowMeasureFloat>
          );
        case "select":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Select id={measure.id} value={measure.value} defaultValue={measure.defaultValue} onValueChange={measure.onValueChange}>
                <SelectTrigger id={measure.id} className="h-medium w-full min-w-0 max-w-[9.5rem]" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {measure.items.map((item) => (
                    <SelectItem key={item.id} value={item.value}>
                      {item.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </UIWindowMeasureFloat>
          );
        case "combobox":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Combobox id={measure.id} value={measure.value} options={measure.choices} placeholder={measure.placeholder} onValueChange={measure.onValueChange} className="w-full min-w-0 max-w-[9.5rem]" />
            </UIWindowMeasureFloat>
          );
        case "button":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Button id={measure.id} text={measure.text} icon={measure.icon} onClick={measure.onClick} />
            </UIWindowMeasureFloat>
          );
        case "buttonCycle":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <ButtonCycle id={measure.id} value={measure.value} onValueChange={measure.onValueChange} items={measure.items} />
            </UIWindowMeasureFloat>
          );
        case "input":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Input id={measure.id} lazy className="h-medium w-full min-w-0 max-w-[9.5rem]" value={measure.value} placeholder={measure.placeholder} onLazyChange={measure.onLazyChange} />
            </UIWindowMeasureFloat>
          );
        case "textarea":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Textarea id={measure.id} lazy className="min-h-[4rem] w-full min-w-0 max-w-[9.5rem]" value={measure.value} placeholder={measure.placeholder} rows={measure.rows} onLazyChange={measure.onLazyChange} />
            </UIWindowMeasureFloat>
          );
        case "checkbox":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id}>
              <div className="text-foreground flex w-full min-w-0 items-center gap-single text-xs">
                <input
                  id={measure.id}
                  type="checkbox"
                  className="border-element accent-foreground size-small shrink-0 rounded border"
                  {...(measure.checked !== undefined ? { checked: measure.checked } : { defaultChecked: measure.defaultChecked })}
                  onChange={(event) => measure.onCheckedChange?.(event.target.checked)}
                />
                {measure.label ? (
                  <label htmlFor={measure.id} className="cursor-pointer select-none">
                    {measure.label}
                  </label>
                ) : null}
              </div>
            </UIWindowMeasureFloat>
          );
        case "radio":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className="flex flex-col gap-half" role="radiogroup" aria-labelledby={measure.id}>
                {measure.items.map((item) => (
                  <button
                    key={item.value}
                    type="button"
                    data-slot="window-measure-radio-item"
                    className={cn("border-element/80 hover:bg-hover-window rounded border px-single py-half text-left text-xs transition-colors", measure.value === item.value && "bg-active-base text-active-foreground")}
                    onClick={() => measure.onChange?.(item.value)}
                  >
                    {item.label}
                  </button>
                ))}
              </div>
            </UIWindowMeasureFloat>
          );
        case "slider": {
          const min = measure.min ?? 0;
          const max = measure.max ?? 100;
          const v = measure.value ?? min;
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Slider id={measure.id} value={[v]} min={min} max={max} step={measure.step} onValueChange={(vals) => measure.onValueChange?.(vals[0] ?? min)} />
            </UIWindowMeasureFloat>
          );
        }
        case "number":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Stepper id={measure.id} value={measure.value} min={measure.min} max={measure.max} step={measure.step} onChange={measure.onChange} />
            </UIWindowMeasureFloat>
          );
        case "color":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Input id={measure.id} type="color" className="h-medium w-full min-w-0 max-w-[9.5rem] cursor-pointer" value={measure.value} onChange={(event) => measure.onChange?.(event.target.value)} />
            </UIWindowMeasureFloat>
          );
        default: {
          const _exhaustive: never = measure;
          return _exhaustive;
        }
      }
    })}
  </div>
);

// #endregion 🪟WindowMeasuresOverlay

/**
 * Portal target for a golden-layout window kind.
 * Holds the DOM element, window kind definition, and a unique key.
 **/
interface UICanvasPortal {
  key: string;
  element: HTMLElement;
  windowKind: UIWindowKindDefinition;
}

interface UICanvasAsyncLifecycle {
  isDisposed: () => boolean;
  registerCleanup: (cleanup: () => void) => void;
  dispose: () => void;
}

function createUICanvasAsyncLifecycle(): UICanvasAsyncLifecycle {
  let disposed = false;
  let cleanup: (() => void) | undefined;

  return {
    isDisposed: () => disposed,
    registerCleanup: (nextCleanup) => {
      cleanup = nextCleanup;
      if (disposed) {
        cleanup();
      }
    },
    dispose: () => {
      disposed = true;
      if (cleanup) {
        const fn = cleanup;
        cleanup = undefined;
        fn();
      }
    },
  };
}

/**
 * Golden-layout canvas that renders window kinds using React portals.
 * Dynamically imports golden-layout and registers each window kind as a component.
 * Uses portals instead of createRoot so that parent React context flows into golden-layout windows.
 **/
const UICanvas: React.FC<{
  windowKinds: UIWindowKindDefinition[];
  defaultLayout: UIWindowLayout;
  layoutState?: unknown;
  onLayoutChange?: (layout: UIWindowLayout) => void;
  onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowKinds, defaultLayout, layoutState, onLayoutChange, onActiveWindowChange }) => {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const layoutRef = React.useRef<any>(null);
  const [portals, setPortals] = React.useState<UICanvasPortal[]>([]);
  const onLayoutChangeRef = React.useRef(onLayoutChange);
  const onActiveWindowChangeRef = React.useRef(onActiveWindowChange);
  onLayoutChangeRef.current = onLayoutChange;
  onActiveWindowChangeRef.current = onActiveWindowChange;
  /** @emoji 🪟 Stable registry key so measure/control-only `windowKinds` updates do not destroy Golden Layout. */
  const windowKindRegistryKey = React.useMemo(() => windowKinds.map((wk) => wk.id).join("\0"), [windowKinds]);

  /** @emoji 📐 Keeps floating measures/controls in sync when `windowKinds` change without tearing down Golden Layout. */
  React.useEffect(() => {
    if (!layoutRef.current) {
      return;
    }
    setPortals((prev) =>
      prev.map((portal) => {
        const next = windowKinds.find((wk) => wk.id === portal.windowKind.id);
        return next ? { ...portal, windowKind: next } : portal;
      }),
    );
  }, [windowKinds]);

  React.useEffect(() => {
    if (!containerRef.current || layoutRef.current) return;

    const lifecycle = createUICanvasAsyncLifecycle();

    const loadGoldenLayout = async () => {
      try {
        const goldenLayoutModule = await import("golden-layout");
        if (lifecycle.isDisposed()) return;
        const GoldenLayout = (goldenLayoutModule as any).GoldenLayout;
        if (!GoldenLayout || typeof GoldenLayout !== "function") {
          console.error("[UICanvas] GoldenLayout is not a constructor");
          return;
        }

        const rawLayout = parseWindowLayout(layoutState) ?? defaultLayout;
        const config = convertWindowLayoutToGoldenConfig(rawLayout);
        if (!config) {
          console.error("[UICanvas] No layout config");
          return;
        }

        const layout = new GoldenLayout(config, containerRef.current!);
        let isInitialized = false;
        let portalCounter = 0;

        windowKinds.forEach((windowKind) => {
          layout.registerComponent(windowKind.id, (container: any) => {
            if (lifecycle.isDisposed()) return;
            const element = container.getElement();
            let domElement: HTMLElement;
            if (element instanceof HTMLElement) {
              domElement = element;
            } else if (Array.isArray(element) && element[0] instanceof HTMLElement) {
              domElement = element[0];
            } else if (element?.[0] instanceof HTMLElement) {
              domElement = element[0];
            } else if (element?.nodeType === 1) {
              domElement = element as HTMLElement;
            } else {
              console.error("[UICanvas] Could not extract DOM element from container");
              return;
            }

            const portalKey = `${windowKind.id}-${portalCounter++}`;
            const portal: UICanvasPortal = { key: portalKey, element: domElement, windowKind };
            setPortals((prev) => [...prev, portal]);

            container.on("destroy", () => {
              setPortals((prev) => prev.filter((p) => p.key !== portalKey));
            });
          });
        });

        layout.on("stateChanged", () => {
          const onLayout = onLayoutChangeRef.current;
          if (!onLayout || !isInitialized) return;
          try {
            const nextLayout = parseWindowLayout(layout.toConfig());
            if (nextLayout) onLayout(nextLayout);
          } catch (error: any) {
            if (!error?.message?.includes("not yet initialised")) {
              console.warn("[UICanvas] Failed to get layout config:", error);
            }
          }
        });

        layout.on("tab", (tab: any) => {
          if (tab._header) {
            tab._header.on("click", () => {
              const componentName = tab._contentItem?.config?.componentName;
              const onActive = onActiveWindowChangeRef.current;
              if (componentName && onActive) onActive(componentName);
            });
          }
        });

        layout.init();
        isInitialized = true;
        layoutRef.current = layout;

        const handleResize = () => layout.updateSize();
        const bindings = new DOMEventBindingController();
        bindings.listen(window, "resize", handleResize);

        lifecycle.registerCleanup(() => {
          if (layoutRef.current === layout) {
            layoutRef.current = null;
          }
          bindings.dispose();
          setPortals([]);
          try {
            layout.destroy();
          } catch {}
          layoutRef.current = null;
        });
      } catch (error) {
        console.error("[UICanvas] Failed to load GoldenLayout:", error);
      }
    };

    void loadGoldenLayout();

    return () => {
      lifecycle.dispose();
    };
  }, [windowKindRegistryKey, defaultLayout, layoutState]);

  return (
    <>
      <div ref={containerRef} className="w-full h-full" />
      {portals.map((portal) => {
        const WindowComponent = portal.windowKind.component;

        const clickGoldenLayoutControl = (selector: string) => {
          const stackElement = portal.element.closest(".lm_item.lm_stack") as HTMLElement | null;
          const controlElement = queryElement<HTMLElement>(selector, stackElement);
          controlElement?.click();
        };

        return renderPortalInto(
          <Window
            key={portal.key}
            id={portal.windowKind.id}
            isVisible={true}
            showControls={true}
            onOpenInNewWindow={() => clickGoldenLayoutControl(".lm_popout")}
            onMaximize={() => clickGoldenLayoutControl(".lm_maximise")}
            onMinimize={() => clickGoldenLayoutControl(".lm_maximise")}
            onClose={() => clickGoldenLayoutControl(".lm_close")}
            controls={portal.windowKind.controls ? <UIWindowControlsGroup controls={portal.windowKind.controls} /> : undefined}
            measures={portal.windowKind.measures?.length ? <UIWindowMeasures measures={portal.windowKind.measures} /> : undefined}
          >
            <ContextMenu items={portal.windowKind.contextMenu}>
              <div className="flex min-h-0 min-w-0 flex-1 flex-col">
                <WindowComponent />
              </div>
            </ContextMenu>
          </Window>,
          portal.element,
        );
      })}
    </>
  );
};

// #region 🎼UISearch

/**
 * A searchable item for the global UI command palette.
 * Consumers provide items; the UI renders them in a CommandDialog with fuzzy search.
 **/
export interface UISearchItem {
  id: string;
  label: string;
  description?: string;
  icon?: React.ReactNode;
  category?: string;
  onSelect: () => void;
}

/**
 * Global search command palette for the UI (Ctrl+P / Cmd+P).
 * Uses Fuse.js for fuzzy matching and CommandDialog for rendering.
 **/
const UISearch: React.FC<{
  items: UISearchItem[];
  open: boolean;
  onOpenChange: (open: boolean) => void;
  placeholder?: string;
  emptyMessage?: string;
}> = ({ items, open, onOpenChange, placeholder = "Search...", emptyMessage = "No results found." }) => {
  const [query, setQuery] = React.useState("");

  const fuse = React.useMemo(
    () =>
      new Fuse(items, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [items],
  );

  const results = React.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, query, items]);

  const grouped = React.useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = React.useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title="Search" description="Search for items..." open={open} onOpenChange={onOpenChange}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} onSelect={() => handleSelect(result.item)}>
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description && <span className="text-xs text-muted-foreground">{result.item.description}</span>}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
};

// #endregion 🎼UISearch

// #region 🌧️UIFind

/**
 * A findable item scoped to an app for the per-app find palette.
 **/
export interface UIFindItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

/**
 * Context value for per-app find functionality.
 * Apps set find items and a callback; the UI renders the find palette.
 **/
export interface UIFindContextValue {
  findItems: UIFindItem[];
  setFindItems: (items: UIFindItem[]) => void;
  setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  triggerFindItem: (itemId: string) => void;
}

const UIFindContext = React.createContext<UIFindContextValue | null>(null);
const EMPTY_UI_FIND_ITEMS: UIFindItem[] = [];

function areFindItemsShallowEqual(previousItems: UIFindItem[], nextItems: UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let i = 0; i < nextItems.length; i++) {
    if (previousItems[i] !== nextItems[i]) return false;
  }
  return true;
}

/**
 * Provider for per-app find functionality.
 * Wraps children and exposes find items + trigger via context.
 **/
export const UIFindProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [findItems, setFindItems] = React.useState<UIFindItem[]>([]);
  const onFindItemCallbackRef = React.useRef<((itemId: string) => void) | undefined>(undefined);

  const setFindItemsStable = React.useCallback((items: UIFindItem[]) => {
    setFindItems((previousItems) => {
      return areFindItemsShallowEqual(previousItems, items) ? previousItems : items;
    });
  }, []);

  const setOnFindItem = React.useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);

  const triggerFindItem = React.useCallback((itemId: string) => {
    if (onFindItemCallbackRef.current) {
      onFindItemCallbackRef.current(itemId);
    }
  }, []);

  const contextValue = React.useMemo(() => ({ findItems, setFindItems: setFindItemsStable, setOnFindItem, triggerFindItem }), [findItems, setFindItemsStable, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
};

/**
 * Hook to access the find context. Throws if used outside UIFindProvider.
 **/
export function useUIFind(): UIFindContextValue {
  const context = React.useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

/**
 * Hook to access the find context. Returns null if outside UIFindProvider.
 **/
export function useUIFindSafe(): UIFindContextValue | null {
  return React.useContext(UIFindContext);
}

/**
 * Per-app find command palette (Ctrl+F / Cmd+F).
 * Renders a CommandDialog with fuzzy search over the active app's find items.
 **/
const UIFind: React.FC<{
  open: boolean;
  onOpenChange: (open: boolean) => void;
  placeholder?: string;
  emptyMessage?: string;
}> = ({ open, onOpenChange, placeholder = "Find...", emptyMessage = "No results found." }) => {
  const [query, setQuery] = React.useState("");
  const findContext = React.useContext(UIFindContext);
  const findItems = findContext?.findItems || [];
  const triggerFindItem = findContext?.triggerFindItem;

  const fuse = React.useMemo(
    () =>
      new Fuse(findItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [findItems],
  );

  const results = React.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [fuse, query, findItems]);

  const grouped = React.useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = React.useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      if (triggerFindItem) triggerFindItem(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext || findItems.length === 0) return null;

  return (
    <CommandDialog title="Find" description="Find items in this app..." open={open} onOpenChange={onOpenChange}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} onSelect={() => handleSelect(result.item)}>
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description && <span className="text-xs text-muted-foreground">{result.item.description}</span>}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
};

// #endregion 🌧️UIFind

// #region 📔UIToolbar

/**
 * A toolbar action item registered by an app or the UI.
 **/
export interface UIToolbarItem {
  id: string;
  icon?: React.ReactNode;
  label?: string;
  text?: string;
  onClick?: () => void;
  kind?: "button" | "toggle" | "separator";
  pressed?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  order?: number;
}

/** @emoji 🧰 Toolbar category ids shared by every App registration surface. */
export type AppToolCategory = "history" | "hand" | "selection" | "lasso" | "filter" | "open" | "create" | "view" | "actions" | "settings";

/** @emoji 📋 Default toolbar category order (history and hand first when present). */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = ["history", "hand", "selection", "lasso", "filter", "open", "create", "view", "actions", "settings"];

/** @emoji 🗂️ Per-category toolbar tools registered by an app or global UI shell. */
export type AppTools = Partial<Record<AppToolCategory, UIToolbarItem[]>>;

function sortToolbarItems(items: readonly UIToolbarItem[]): UIToolbarItem[] {
  return [...items].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

function hasAppToolCategoryItems(items: readonly UIToolbarItem[] | undefined): boolean {
  return Boolean(items?.some((item) => item.kind !== "separator"));
}

/** @emoji 🔢 Counts registered toolbar items across all populated categories. */
export function countAppTools(tools?: AppTools): number {
  if (!tools) return 0;
  return APP_TOOL_CATEGORY_ORDER.reduce((sum, category) => sum + (tools[category]?.length ?? 0), 0);
}

/** @emoji 🔀 Merges base and extension tool maps per category (extension appends within each category). */
export function mergeAppTools(base?: AppTools, extension?: AppTools): AppTools | undefined {
  if (!base && !extension) return undefined;
  const merged: AppTools = {};
  for (const category of APP_TOOL_CATEGORY_ORDER) {
    const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
    if (combined.length > 0) merged[category] = combined;
  }
  return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 📂 Lists categories that have at least one non-separator tool. */
export function listPopulatedAppToolCategories(tools?: AppTools): AppToolCategory[] {
  if (!tools) return [];
  return APP_TOOL_CATEGORY_ORDER.filter((category) => hasAppToolCategoryItems(tools[category]));
}

function resolveAppToolCategoryIcon(category: AppToolCategory): React.ReactNode {
  switch (category) {
    case "hand":
      return <HandIcon className="size-tiny" aria-hidden />;
    case "selection":
      return <MousePointerIcon className="size-tiny" aria-hidden />;
    case "lasso":
      return <LassoIcon className="size-tiny" aria-hidden />;
    case "filter":
      return <FilterIcon className="size-tiny" aria-hidden />;
    case "open":
      return <FolderOpenIcon className="size-tiny" aria-hidden />;
    case "create":
      return <PlusIcon className="size-tiny" aria-hidden />;
    case "view":
      return <LayoutGridIcon className="size-tiny" aria-hidden />;
    case "actions":
      return <MoreHorizontalIcon className="size-tiny" aria-hidden />;
    case "settings":
      return <Settings2Icon className="size-tiny" aria-hidden />;
    default:
      return <SearchIcon className="size-tiny" aria-hidden />;
  }
}

const UIToolbarItems: React.FC<{ items: readonly UIToolbarItem[] }> = ({ items }) => {
  const sorted = React.useMemo(() => sortToolbarItems(items), [items]);
  return (
    <>
      {sorted.map((item) => {
        if (item.kind === "separator") {
          return <ToolbarDivider key={item.id} />;
        }
        if (item.kind === "toggle") {
          return (
            <ToolbarItem key={item.id}>
              <Toggle kind={item.icon && !item.text && !item.label ? "icon" : "default"} id={item.id} pressed={item.pressed ?? false} onPressedChange={(pressed) => item.onPressedChange?.(pressed)} icon={item.icon} text={item.text ?? item.label} />
            </ToolbarItem>
          );
        }
        return (
          <ToolbarItem key={item.id}>
            <button onClick={item.onClick} className="flex items-center gap-single px-single py-tiny hover:bg-hover-panel rounded text-sm cursor-pointer">
              {item.icon}
              {(item.text ?? item.label) && <span>{item.text ?? item.label}</span>}
            </button>
          </ToolbarItem>
        );
      })}
    </>
  );
};

/**
 * Renders a floating toolbar with category toggles; only categories with registered tools are shown.
 **/
const UIToolbar: React.FC<{
  tools: AppTools;
  className?: string;
}> = ({ tools, className }) => {
  const { t } = useTranslation();
  const populatedCategories = React.useMemo(() => listPopulatedAppToolCategories(tools), [tools]);
  const [activeCategory, setActiveCategory] = React.useState<AppToolCategory | null>(null);

  React.useEffect(() => {
    if (populatedCategories.length === 0) {
      setActiveCategory(null);
      return;
    }
    setActiveCategory((previousValue) => {
      if (previousValue && populatedCategories.includes(previousValue)) return previousValue;
      return populatedCategories.find((category) => category !== "history" && category !== "hand") ?? populatedCategories[0] ?? null;
    });
  }, [populatedCategories]);

  if (populatedCategories.length === 0) return null;

  const activeItems = activeCategory ? (tools[activeCategory] ?? []) : [];
  const showCategoryNav = populatedCategories.length > 1;

  return (
    <div className={cn("flex items-center justify-center pointer-events-none", className)}>
      <div role="toolbar" id="ui.toolbar" className={cn("pointer-events-auto flex max-w-full items-center gap-single", showCategoryNav && "relative h-[var(--toolbar-item-height)] w-full max-w-[min(100%,48rem)] px-2")}>
        {showCategoryNav ? (
          <>
            <ToolbarZone id="ui.toolbar.zone.categories" className="shrink-0">
              {populatedCategories.map((category) => (
                <Toggle
                  key={category}
                  kind="single"
                  id={`ui.toolbar.group.${category}`}
                  pressed={activeCategory === category}
                  onPressedChange={() => setActiveCategory((previousValue) => (previousValue === category ? null : category))}
                  icon={resolveAppToolCategoryIcon(category)}
                  text={resolveTranslationLabel(t(`compose.sketchpad.toolbar.parent.${category}`))}
                />
              ))}
            </ToolbarZone>
            {activeCategory && hasAppToolCategoryItems(activeItems) ? (
              <ToolbarZone id="ui.toolbar.zone.tools" className="min-w-0 flex-1 flex-wrap h-auto min-h-[var(--toolbar-item-height)] overflow-visible p-half">
                <UIToolbarItems items={activeItems} />
              </ToolbarZone>
            ) : null}
          </>
        ) : (
          <ToolbarZone className="max-w-full flex-wrap h-auto min-h-[var(--toolbar-item-height)] overflow-visible p-half">
            <UIToolbarItems items={tools[populatedCategories[0]!] ?? []} />
          </ToolbarZone>
        )}
      </div>
    </div>
  );
};

export { UICanvas, UISearch, UIFind, UIToolbar };

// #endregion 📔UIToolbar
