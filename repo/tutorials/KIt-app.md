# Kit App

## `semio/sketchpad/index.tsx` `MultiWindowApp`

```tsx
const MultiWindowApp: FC = () => {
  useKitAppYjsToXStateSync();
  const transaction = useKitAppTransaction();
  const actor = useSketchpadActor();
  const sketchpadStore = useSketchpadStore();
  const kitGuid = useKitScope()?.guid;
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const [activeWindow, setActiveWindow] = useState<string>(KitAppWindowKind.Table);
  const [activeTool, setActiveTool] = useKitAppActiveTool();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind === ToolKind.SELECTION_NORMAL && activeTool !== ToolKind.SELECTION_NORMAL) setActiveTool(ToolKind.SELECTION_NORMAL);
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  useEffect(() => {
    if (appType !== "kit") return;
    if (!kitGuid) return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.selection",
      specificity: 20,
      order: 10,
      toolbarGroup: {
        id: "selection",
        labelId: "semio.sketchpad.toolbar.parent.selection",
        order: 10,
      },
      content: () => (
        <KitScopeProvider guid={kitGuid}>
          <KitToolbarSelection />
        </KitScopeProvider>
      ),
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.filters",
      specificity: 20,
      order: 20,
      toolbarGroup: {
        id: "filter",
        labelId: "semio.sketchpad.toolbar.parent.filter",
        order: 20,
      },
      content: () => (
        <KitScopeProvider guid={kitGuid}>
          <KitFilters />
        </KitScopeProvider>
      ),
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.create",
      specificity: 20,
      order: 30,
      toolbarGroup: {
        id: "create",
        labelId: "semio.sketchpad.toolbar.parent.create",
        order: 30,
      },
      content: () => {
        return (
          <KitScopeProvider guid={kitGuid}>
            <KitCreateActions />
          </KitScopeProvider>
        );
      },
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.selection");
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.filters");
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.create");
      removeSection("toolbar", "semio.sketchpad.app.kit.kitApp.toolsGroup");
    };
  }, [appType, addSection, removeSection, kitGuid]);

  const hasKit = useHasKit(kitGuid || "");

  const store = useMemo(() => {
    if (!kitGuid || !sketchpadStore?.hasKitApp?.({ kit: kitGuid })) return null;
    return sketchpadStore.kitApp(kitGuid);
  }, [sketchpadStore, kitGuid]);
  const addSidePanelTab = useAddSidePanelTab();
  const removeSidePanelTab = useRemoveSidePanelTab();

  const storedWindowLayout = useSyncDeep<any, any>(store, (s: KitAppState | null) => s?.windowLayout);

  const defaultLayout = useMemo(
    () => ({
      root: {
        type: "row",
        content: [
          {
            type: "stack",
            size: "50%",
            content: [
              {
                type: "component",
                componentName: KitAppWindowKind.Table,
                title: "table",
                componentState: {},
              },
            ],
          },
          {
            type: "stack",
            size: "50%",
            content: [
              {
                type: "component",
                componentName: KitAppWindowKind.Diagram,
                title: "diagram",
                componentState: {},
              },
            ],
          },
        ],
      },
    }),
    [],
  );
  const windowLayout = useMemo(() => {
    if (!storedWindowLayout) return defaultLayout;
    const removeLegacySideTabsFromWindowLayout = (layoutNode: any): any => {
      if (!layoutNode || typeof layoutNode !== "object") return layoutNode;
      if (layoutNode.type === "component" && (layoutNode.componentName === "settings" || layoutNode.componentName === "chat")) {
        return null;
      }
      if (layoutNode.root && typeof layoutNode.root === "object") {
        const root = removeLegacySideTabsFromWindowLayout(layoutNode.root);
        if (!root) return undefined;
        return { ...layoutNode, root };
      }
      if (Array.isArray(layoutNode.content)) {
        const content = layoutNode.content.map((item: any) => removeLegacySideTabsFromWindowLayout(item)).filter(Boolean);
        if (content.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, content };
      }
      if (Array.isArray(layoutNode.contentItems)) {
        const contentItems = layoutNode.contentItems.map((item: any) => removeLegacySideTabsFromWindowLayout(item)).filter(Boolean);
        if (contentItems.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, contentItems };
      }
      return layoutNode;
    };
    return removeLegacySideTabsFromWindowLayout(storedWindowLayout) || defaultLayout;
  }, [storedWindowLayout, defaultLayout]);

  const windowConfig: AppWindowConfig = useMemo(
    () => ({
      windowKinds: [
        {
          id: KitAppWindowKind.Table,
          label: "table",
          component: () => <TableWindow />,
        },
        {
          id: KitAppWindowKind.Diagram,
          label: "diagram",
          component: () => <KitDiagramWindow />,
        },
      ],
      defaultLayout,
    }),
    [defaultLayout],
  );

  const handleLayoutChange = useCallback(
    (config: any) => {
      if (store && typeof store.change === "function") {
        store.change({ windowLayout: config });
      }
    },
    [store],
  );

  if (!hasKit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">Loading kit...</p>
      </div>
    );
  }

  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">Failed to load kit app</p>
        </div>
      }
    >
      <TransactionProvider transaction={transaction}>
        <KitDropZone>
          <Canvas id="semio.sketchpad.app.kit.canvas">
            <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
          </Canvas>
        </KitDropZone>
      </TransactionProvider>
    </ErrorBoundary>
  );
};
```

## `semio/sketchpad/index.tsx` `KitSection` / `KitSectionForm`

```tsx
export const KitSection: FC = () => {
  const isInKitScope = useIsInKitScope();
  if (!isInKitScope) return null;
  return <KitSectionForm />;
};

const KitSectionForm: FC = () => {
  const { t } = useTranslation();
  try {
    const kit = useKit() as Kit;
    if (!kit) {
      return (
        <HelperRow propertyAligned>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.notAvailable")}</p>
        </HelperRow>
      );
    }
    const kitDataSource = useKitAppStore() as any;
    return (
      <>
        <TreeRow>
          <Input lazy id="semio.sketchpad.app.kit.panel.details.section.kit.name" value={kit.name} onLazyChange={(value) => kitDataSource.change({ name: value })} showLabel />
        </TreeRow>
        <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.version"
            value={kit.version || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.versionPlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ version: value })}
            showLabel
          />
        </TreeRow>
        <TreeRow>
          <Textarea
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.description"
            value={kit.description || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.descriptionPlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ description: value })}
            showLabel
          />
        </TreeRow>
        <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.icon"
            value={kit.icon || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.iconPlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ icon: value })}
            showLabel
          />
        </TreeRow>
        <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.image"
            value={kit.image || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.imagePlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ image: value })}
            showLabel
          />
        </TreeRow>
        <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.homepage"
            value={kit.homepage || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.homepagePlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ homepage: value })}
            showLabel
          />
        </TreeRow>
        <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.kit.license"
            value={kit.license || ""}
            placeholder={useLabel("semio.sketchpad.app.kit.licensePlaceholder.label")}
            onLazyChange={(value) => kitDataSource.change({ license: value })}
            showLabel
          />
        </TreeRow>
      </>
    );
  } catch (error) {
    return (
      <HelperRow propertyAligned>
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.notFound")}</p>
      </HelperRow>
    );
  }
};
```

## `elements/ui/index.tsx` `Label`

```tsx
export function Label({ id, rowId, label, labelElementId, className, children, labelLayoutKind = "property" }: LabelProps) {
  const localizedLabel = useLabel(id);
  const resolvedLabel = label ?? localizedLabel;
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = React.useContext(TreeContext);
  const isInsideTreeRow = React.useContext(TreeRowAlignmentContext);
  const treePropertyRowOffsetPx = detailPanelIndentPx(level, indentMultiplier);

  const propertyLabelElement = (
    <Tooltip>
      <TooltipTrigger asChild>
        {isTree ? (
          <div data-slot="property-label-tree" className="min-w-0" style={{ paddingLeft: `${treePropertyRowOffsetPx}px` }}>
            <div className="inline-flex min-w-0 h-[22px]">
              <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px] pl-[4px]">
                {resolvedLabel}
              </span>
            </div>
          </div>
        ) : (
          <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px]">
            {resolvedLabel}
          </span>
        )}
      </TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );

  const propertyRowElement = (
    <div
      id={rowId}
      data-slot="property-row"
      style={isTree ? { marginLeft: `${-treePropertyRowOffsetPx}px`, width: treePropertyRowOffsetPx > 0 ? `calc(100% + ${treePropertyRowOffsetPx}px)` : "100%" } : undefined}
      className={cn(detailPanelPropertyRowClassName, isTree ? "grid-cols-[96px_minmax(0,1fr)]" : "w-full grid-cols-[96px_1fr]", className)}
    >
      {propertyLabelElement}
      <div data-slot="property-control" className={detailPanelPropertyControlClassName}>
        <PropertyValueColumnContext.Provider value={true}>{children}</PropertyValueColumnContext.Provider>
      </div>
    </div>
  );

  if (isTree) {
    if (isInsideTreeRow) {
      return propertyRowElement;
    }
    return (
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0}>
        {propertyRowElement}
      </TreeAlignedRow>
    );
  }

  return propertyRowElement;
}
```

## `elements/ui/index.tsx` `CollapsedFieldDisplay`

```tsx
function CollapsedFieldDisplay({ allowStackedOverflow = false, className, disabled, id, mixed, onActivate, placeholder, slot, value }: CollapsedFieldDisplayProps) {
  const isInPropertyValueColumn = React.useContext(PropertyValueColumnContext);
  const displayRef = React.useRef<HTMLDivElement>(null);
  const normalizedValue = React.useMemo(() => normalizeCollapsedFieldText(value), [value]);
  const stackedOverflowEnabled = isInPropertyValueColumn && allowStackedOverflow;
  const [displayState, setDisplayState] = React.useState<CollapsedFieldDisplayState>({
    value: normalizedValue,
    normalizedValue,
    isOverflowing: false,
    layoutKind: "single-line",
  });

  const updateCollapsedValue = React.useCallback(() => {
    const element = displayRef.current;
    if (!element) {
      return;
    }
    if (!normalizedValue) {
      setDisplayState({
        value: "",
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const computedStyle = window.getComputedStyle(element);
    const maxWidth = element.clientWidth - parseFloat(computedStyle.paddingLeft || "0") - parseFloat(computedStyle.paddingRight || "0");
    if (maxWidth <= 0) {
      setDisplayState({
        value: normalizedValue,
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const measurementElement = document.createElement("span");
    measurementElement.style.position = "absolute";
    measurementElement.style.visibility = "hidden";
    measurementElement.style.pointerEvents = "none";
    measurementElement.style.whiteSpace = "nowrap";
    measurementElement.style.font = computedStyle.font || `${computedStyle.fontStyle} ${computedStyle.fontVariant} ${computedStyle.fontWeight} ${computedStyle.fontSize} / ${computedStyle.lineHeight} ${computedStyle.fontFamily}`;
    measurementElement.style.letterSpacing = computedStyle.letterSpacing;
    measurementElement.style.textTransform = computedStyle.textTransform;
    measurementElement.style.textRendering = computedStyle.textRendering;
    document.body.appendChild(measurementElement);

    const measureText = (candidate: string) => {
      measurementElement.textContent = candidate;
      return measurementElement.getBoundingClientRect().width;
    };

    const nextState = resolveCollapsedFieldDisplayState({ allowStackedOverflow: stackedOverflowEnabled, value: normalizedValue, maxWidth, measureText });
    measurementElement.remove();

    setDisplayState((previousState) =>
      previousState.value === nextState.value && previousState.normalizedValue === nextState.normalizedValue && previousState.isOverflowing === nextState.isOverflowing && previousState.layoutKind === nextState.layoutKind ? previousState : nextState,
    );
  }, [normalizedValue, stackedOverflowEnabled]);

  React.useEffect(() => {
    updateCollapsedValue();
  }, [updateCollapsedValue]);

  React.useEffect(() => {
    const element = displayRef.current;
    if (!element || typeof ResizeObserver === "undefined") {
      return;
    }
    const resizeObserver = new ResizeObserver(() => updateCollapsedValue());
    resizeObserver.observe(element);
    return () => resizeObserver.disconnect();
  }, [updateCollapsedValue]);

  const activate = () => {
    if (!disabled) {
      onActivate();
    }
  };

  const showStackedOverflow = stackedOverflowEnabled && displayState.layoutKind === "stacked-overflow";

  return (
    <div
      ref={displayRef}
      data-slot={slot}
      data-collapsed="true"
      data-overflowing={displayState.isOverflowing ? "true" : undefined}
      data-overflow-layout={showStackedOverflow ? "stacked" : "single-line"}
      id={id}
      className={cn(
        "text-foreground flex w-full min-w-0 overflow-hidden border bg-transparent text-base transition-[color,border-color] outline-none md:text-sm",
        showStackedOverflow ? "grid h-auto min-h-[46px] grid-cols-1 grid-rows-[minmax(0,1fr)_14px] content-start gap-y-[2px] px-single py-[2px]" : "h-medium items-center px-single whitespace-nowrap",
        "aria-invalid:border-destructive flex-1 cursor-text",
        disabled && "cursor-not-allowed opacity-50",
        mixed && !displayState.value && "italic text-muted-foreground/70",
        className,
      )}
      tabIndex={disabled ? -1 : 0}
      role="textbox"
      aria-readonly="true"
      aria-disabled={disabled ? "true" : undefined}
      onClick={activate}
      onFocus={activate}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          activate();
        }
      }}
    >
      {displayState.value ? (
        showStackedOverflow ? (
          <>
            <span data-slot="collapsed-field-line" className="flex min-w-0 items-center overflow-hidden whitespace-nowrap leading-normal">
              <span className="block min-w-0 overflow-hidden text-ellipsis whitespace-nowrap">{displayState.value}</span>
            </span>
            <span data-slot="collapsed-field-overflow" aria-hidden="true" className="flex h-[14px] min-w-0 items-center justify-center overflow-hidden leading-none">
              <span data-slot="collapsed-field-indicator" className="inline-flex items-center justify-center text-muted-foreground/75 leading-none">
                <ChevronDownIcon data-slot="collapsed-field-indicator-chevron" className="size-[10px] shrink-0 stroke-[2.5]" />
              </span>
            </span>
          </>
        ) : (
          displayState.value
        )
      ) : (
        <span className={cn("truncate", mixed ? "italic text-muted-foreground/70" : "text-muted-foreground")}>{placeholder}</span>
      )}
    </div>
  );
}
```

## `elements/ui/index.tsx` `Input`

```tsx
function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, interactionId, id, placeholderId, placeholder, showLabel, mixed, ...props }: InputProps) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = React.useContext(PropertyValueColumnContext);
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);
  const [isFocused, setIsFocused] = React.useState(false);
  const inputRef = React.useRef<HTMLInputElement>(null);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const placeholderLabel = useLabel(placeholderId || "");
  const mixedLabel = useLabel("semio.sketchpad.common.mixedValues");
  const computedPlaceholder = mixed ? mixedLabel || "—" : placeholderId ? placeholderLabel : placeholder;

  React.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  React.useEffect(() => {
    if (isFocused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocused]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(false);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (lazy) {
      setIsEditing(false);
      onLazyChange?.(localValue);
      transaction?.finalize?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (lazy) {
      if (e.key === "Enter") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        onLazyChange?.(localValue);
        transaction?.finalize?.();
        (e.target as HTMLInputElement).blur();
      } else if (e.key === "Escape") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        transaction?.abort?.();
        (e.target as HTMLInputElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const inputValue = lazy ? localValue : externalValue;

  const activeInteraction = useActiveInteraction();
  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;
  const inputDisplayValue = inputValue?.toString() || "";
  const showCollapsedDisplay = !!showLabel && !isFocused && isCollapsibleInputType(type);
  const allowStackedOverflow = isStackedOverflowInputType(type);

  const inputEmptyOpacity = isInPropertyValueColumn && !inputDisplayValue && !isFocused ? 0.6 : 1;
  const inputFinalOpacity = shouldFade ? 0 : inputEmptyOpacity;

  const inputElement = (
    <div data-slot="input-root" data-detail-panel-control="fill" className="flex min-w-0 w-full flex-1 items-stretch" style={{ opacity: inputFinalOpacity, transition: "opacity 150ms" }}>
      {showCollapsedDisplay ? (
        <CollapsedFieldDisplay
          allowStackedOverflow={allowStackedOverflow}
          className={className}
          disabled={props.disabled}
          id={id}
          mixed={mixed}
          onActivate={() => setIsFocused(true)}
          placeholder={computedPlaceholder}
          slot="input"
          value={mixed && !inputDisplayValue ? "" : inputDisplayValue}
        />
      ) : (
        <input
          ref={inputRef}
          type={type}
          data-slot="input"
          data-mixed={mixed ? "true" : undefined}
          id={id}
          className={cn(
            "file:text-foreground placeholder:text-muted-foreground text-foreground flex h-medium w-full min-w-0 border bg-transparent p-single text-base transition-[color,border-color] outline-none file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            "focus-visible:border-accent",
            "aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex-1",
            mixed && "placeholder:italic placeholder:text-muted-foreground/70",
            type === "number" && "[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]",
            className,
          )}
          value={mixed && !isFocused && !inputValue ? "" : inputValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          placeholder={computedPlaceholder}
          {...props}
        />
      )}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {inputElement}
      </Label>
    );
  }

  return inputElement;
}
```

## `semio/sketchpad/index.tsx` Diagram Node / Edge / Connection

```tsx
const KitArtifactNode: FC<NodeProps<Node<KitDiagramNode>>> = ({ data }) => {
  const [selection] = useKitAppSelection();
  const [hover] = useKitAppHover();
  const strategy = useMemo(() => getKitDiagramShapeStrategy(data.kind), [data.kind]);
  const frame = useMemo(() => getKitDiagramNodeFrameForKind(data.kind), [data.kind]);
  const renderPayload = useMemo<KitDiagramShapeRenderPayload>(() => strategy.getRenderPayload(), [strategy]);

  const isHovered = useMemo(() => {
    if (!hover) return false;
    if (data.kind === "type") return hover.type === data.guid;
    if (data.kind === "design") return hover.design === data.guid;
    if (data.kind === "quality") return hover.quality === data.guid;
    if (data.kind === "port") return hover.port === data.guid;
    if (data.kind === "file") return hover.file === data.guid;
    if (data.kind === "folder") return hover.folder === data.guid;
    if (data.kind === "author") return hover.author === data.guid;
    return false;
  }, [hover, data.kind, data.guid]);

  const isSelected = useMemo(() => {
    if (!selection) return false;
    switch (data.kind) {
      case "type":
        return selection.types?.includes(data.guid) ?? false;
      case "design":
        return selection.designs?.includes(data.guid) ?? false;
      case "quality":
        return selection.qualities?.includes(data.guid) ?? false;
      case "port":
        return selection.ports?.includes(data.guid) ?? false;
      case "file":
        return selection.files?.includes(data.guid) ?? false;
      case "folder":
        return selection.folders?.includes(data.guid) ?? false;
      case "author":
        return selection.authors?.includes(data.guid) ?? false;
      default:
        return false;
    }
  }, [selection, data.kind, data.guid]);

  return (
    <div
      data-kit-node="v3"
      data-kit-node-shape={strategy.id}
      data-kit-node-kind={data.kind}
      style={{
        width: `${frame.width}px`,
        height: `${frame.height}px`,
        position: "relative",
        background: "transparent",
        border: "0",
        outline: "0",
        boxShadow: "none",
        pointerEvents: "auto",
        padding: 0,
        margin: 0,
      }}
      title={data.name || data.guid.substring(0, 8)}
    >
      <Handle type="target" position={Position.Top} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Bottom} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="target" position={Position.Left} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Right} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <TableAvatar
        id="semio.sketchpad.app.kit.diagram.node.avatar"
        className={`!absolute !inset-0 ${renderPayload.className ?? ""}`}
        name={data.name}
        icon={data.icon}
        isSelected={isSelected}
        isHovered={isHovered}
        style={{ width: `${frame.width}px`, height: `${frame.height}px`, ...(renderPayload.style as React.CSSProperties | undefined) }}
      />
    </div>
  );
};

const resolveDiagramEdgeAnchors = (sourceNode: any, targetNode: any) => {
  const sourceKind = resolveDiagramNodeKind(sourceNode);
  const targetKind = resolveDiagramNodeKind(targetNode);
  const sourcePosition = resolveDiagramNodePosition(sourceNode);
  const targetPosition = resolveDiagramNodePosition(targetNode);
  const sourceFrame = resolveDiagramNodeFrame(sourceNode, sourceKind);
  const targetFrame = resolveDiagramNodeFrame(targetNode, targetKind);
  const anchors = resolveKitDiagramAnchorPair({ kind: sourceKind, position: sourcePosition, frame: sourceFrame }, { kind: targetKind, position: targetPosition, frame: targetFrame });
  return {
    sx: anchors.source.absolutePoint.x,
    sy: anchors.source.absolutePoint.y,
    tx: anchors.target.absolutePoint.x,
    ty: anchors.target.absolutePoint.y,
    sourcePos: toReactFlowPosition(anchors.source.localPoint.side),
    targetPos: toReactFlowPosition(anchors.target.localPoint.side),
  };
};

const FloatingEdge: FC<EdgeProps> = ({ id, source, target, markerEnd, style, selected, data }) => {
  const sourceNode = useInternalNode(source);
  const targetNode = useInternalNode(target);

  if (!sourceNode || !targetNode) {
    return null;
  }

  const { sx, sy, tx, ty, sourcePos: sPos, targetPos: tPos } = resolveDiagramEdgeAnchors(sourceNode, targetNode);

  const [edgePath] = getBezierPath({
    sourceX: sx,
    sourceY: sy,
    sourcePosition: sPos,
    targetX: tx,
    targetY: ty,
    targetPosition: tPos,
  });

  const relationship = data?.relationship as "part-of" | "reference";
  let stroke = relationship === "part-of" ? "var(--accent-secondary)" : "var(--foreground)";
  let strokeWidth = relationship === "reference" ? 1 : 3;
  let dasharray = relationship === "reference" ? "5 5" : undefined;
  let opacity = 1;

  if (selected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }

  return (
    <g>
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          ...style,
          stroke,
          strokeWidth,
          strokeDasharray: dasharray,
          opacity,
        }}
        className="transition-colors duration-200"
      />
    </g>
  );
};

const FloatingConnectionLine: FC<ConnectionLineComponentProps> = ({ fromX, fromY, toX, toY, fromNode, toNode, pointer }) => {
  const { getNodes } = useReactFlow();
  const fromKind = resolveDiagramNodeKind(fromNode);
  const fromPosition = resolveDiagramNodePosition(fromNode);
  const fromFrame = resolveDiagramNodeFrame(fromNode, fromKind);
  const targetPoint = pointer ?? { x: toX, y: toY };
  const sourceDirection = kitDiagramVector(kitDiagramToAbsolutePoint(fromPosition, { x: fromFrame.width / 2, y: fromFrame.height / 2 }), targetPoint);
  const sourceAnchor = getKitDiagramShapeStrategy(fromKind).resolveNearestPoint(sourceDirection, fromFrame);
  let sourceX = kitDiagramToAbsolutePoint(fromPosition, sourceAnchor).x;
  let sourceY = kitDiagramToAbsolutePoint(fromPosition, sourceAnchor).y;
  let sourcePosition = toReactFlowPosition(sourceAnchor.side);
  let targetX = toX;
  let targetY = toY;
  let targetPosition = toNode ? Position.Top : toY >= fromY ? Position.Bottom : Position.Top;

  if (toNode) {
    const resolved = resolveDiagramEdgeAnchors(fromNode, toNode);
    sourceX = resolved.sx;
    sourceY = resolved.sy;
    sourcePosition = resolved.sourcePos;
    targetX = resolved.tx;
    targetY = resolved.ty;
    targetPosition = resolved.targetPos;
  } else {
    const proximity = getNodes()
      .filter((node) => node.id !== fromNode.id)
      .map((node) => {
        const kind = resolveDiagramNodeKind(node);
        const position = resolveDiagramNodePosition(node);
        const frame = resolveDiagramNodeFrame(node, kind);
        return resolveKitDiagramProximityAnchor(node.id, { kind, position, frame }, targetPoint);
      })
      .sort((a, b) => a.distance - b.distance)[0];

    if (proximity && proximity.distance <= KIT_DIAGRAM_PROXIMITY_CONNECT_DISTANCE) {
      targetX = proximity.anchor.absolutePoint.x;
      targetY = proximity.anchor.absolutePoint.y;
      targetPosition = toReactFlowPosition(proximity.anchor.localPoint.side);
    } else {
      targetPosition = toReactFlowPosition(kitDiagramInferSnapSide({ x: toX - fromPosition.x, y: toY - fromPosition.y }, fromFrame, fromFrame));
    }
  }
  const edgePath = getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
  })[0];

  return <BaseEdge path={edgePath} style={{ stroke: "var(--active-base)", strokeWidth: 3 }} />;
};
```

## `semio/sketchpad/index.tsx` `buildKitDiagramData`

```tsx
const buildKitDiagramData = (kit: Kit): { nodes: Node<KitDiagramNode>[]; edges: Edge[] } => {
  const nodes: Node<KitDiagramNode>[] = [];
  const edges: Edge[] = [];

  const kindGroups: KitDiagramNodeKind[] = ["type", "design", "quality", "port", "file", "folder", "author"];

  for (const kind of kindGroups) {
    let items: Array<{ guid: string; name: string; icon?: any; parentGuid?: string }> = [];

    switch (kind) {
      case "type":
        items = (kit.types ?? []).filter(Boolean).map((t) => ({
          guid: t.guid,
          name: t.name,
          icon: t.icon,
          parentGuid: t.parent?.guid,
        }));
        break;
      case "design":
        items = (kit.designs ?? []).filter(Boolean).map((d) => ({
          guid: d.guid,
          name: d.name,
          icon: d.icon,
          parentGuid: d.parent?.guid,
        }));
        break;
      case "quality":
        items = (kit.qualities ?? []).filter(Boolean).map((q) => ({ guid: q.guid, name: q.name, icon: q.icon }));
        break;
      case "port":
        items = (kit.ports ?? []).filter(Boolean).map((i) => ({ guid: i.guid, name: i.name, icon: i.icon }));
        break;
      case "file":
        items = (kit.files ?? []).filter(Boolean).map((f) => ({ guid: f.guid, name: f.name, icon: getFileIcon(f.name), parentGuid: f.folder?.guid }));
        break;
      case "folder":
        items = (kit.folders ?? []).filter(Boolean).map((f) => ({ guid: f.guid, name: f.name, icon: <FolderIcon className="size-tiny" />, parentGuid: f.parent?.guid }));
        break;
      case "author":
        items = (kit.authors ?? []).filter(Boolean).map((a) => ({ guid: a.guid, name: a.name, icon: <UserIcon className="size-tiny" /> }));
        break;
    }

    for (const item of items) {
      const nodeId = `${kind}:${item.guid}`;
      const frame = getKitDiagramNodeFrameForKind(kind);
      nodes.push({
        id: nodeId,
        type: "artifact",
        position: { x: 0, y: 0 },
        width: frame.width,
        height: frame.height,
        data: {
          guid: item.guid,
          name: item.name,
          kind,
          icon: item.icon,
          parentGuid: item.parentGuid,
        },
      });

      if (item.parentGuid) {
        let parentKind = kind;
        if (kind === "file") parentKind = "folder";
        edges.push({
          id: `${kind}-${item.parentGuid}-${item.guid}`,
          source: `${parentKind}:${item.parentGuid}`,
          target: nodeId,
          type: "floating",
          style: edgeStyle["part-of"],
          data: { relationship: "part-of" },
        });
      }
    }
  }

  for (const design of kit.designs ?? []) {
    for (const piece of design.pieces ?? []) {
      if (piece.type?.guid) {
        const typeGuid = piece.type.guid;
        const sourceId = `type:${typeGuid}`;
        const targetId = `design:${design.guid}`;
        const edgeId = `ref-${sourceId}-${targetId}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });
        }
      }
      if (piece.design?.guid) {
        const nestedDesignGuid = piece.design.guid;
        const sourceId = `design:${nestedDesignGuid}`;
        const targetId = `design:${design.guid}`;
        const edgeId = `ref-${sourceId}-${targetId}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });
        }
      }
    }
  }

  return { nodes, edges };
};
```
