import { FC, ReactNode, useState, useEffect, useReducer } from 'react';
import {
    DndContext,
    DragEndEvent,
    DragOverlay,
    DragStartEvent,
    useDraggable
} from '@dnd-kit/core';
import { createPortal } from 'react-dom';
import { useHotkeys } from 'react-hotkeys-hook';
import { Wrench, Terminal, Info, MessageCircle, ChevronDown, ChevronRight, Folder, Circle } from 'lucide-react';
import { ReactFlowProvider, useReactFlow } from '@xyflow/react';

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable";
import { Design, Type, Piece, flattenDesign, getPieceRepresentationUrls } from '@semio/js';
import { ICON_WIDTH } from '@semio/js/semio';
import { Avatar, AvatarFallback, AvatarImage } from "@semio/js/components/ui/Avatar";
import { default as Diagram } from "@semio/js/components/ui/Diagram";
import { default as Model } from "@semio/js/components/ui/Model";
import { Tooltip, TooltipContent, TooltipTrigger } from "@semio/js/components/ui/Tooltip";
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@semio/js/components/ui/Tabs";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { ScrollArea } from '@semio/js/components/ui/ScrollArea';
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@semio/js/components/ui/HoverCard";
import { Textarea } from '@semio/js/components/ui/Textarea';
import { Generator } from '@semio/js/lib/utils';
import { useDesignEditorStore, useStudioStore } from '@semio/js/store';
import { useSketchpad } from '@semio/js/components/ui/Sketchpad';
import { Input } from '@semio/js/components/ui/Input';
import { Slider } from '@semio/js/components/ui/Slider';

// Type for panel visibility toggles
interface PanelToggles {
    workbench: boolean;
    console: boolean;
    details: boolean;
    chat: boolean;
}

// Basic panel props
interface PanelProps {
    visible: boolean;
}

// Props for resizable panels
interface ResizablePanelProps extends PanelProps {
    onWidthChange?: (width: number) => void;
    width: number;
}

// TreeNode component for sidebar navigation
interface TreeNodeProps {
    label: ReactNode;
    icon?: ReactNode;
    children?: ReactNode;
    level?: number;
    collapsible?: boolean;
    defaultOpen?: boolean;
    isLeaf?: boolean;
}

// TreeSection component for top-level sections
interface TreeSectionProps {
    label: string;
    icon?: ReactNode;
    children?: ReactNode;
    defaultOpen?: boolean;
}

const TreeSection: FC<TreeSectionProps> = ({ label, icon, children, defaultOpen = true }) => {
    const [open, setOpen] = useState(defaultOpen);

    return (
        <Collapsible open={open} onOpenChange={setOpen}>
            <CollapsibleTrigger asChild>
                <div className="flex items-center gap-2 py-1.5 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden">
                    {open ? <ChevronDown size={14} className="flex-shrink-0" /> : <ChevronRight size={14} className="flex-shrink-0" />}
                    {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
                    <span className="flex-1 text-sm text-muted-foreground uppercase tracking-wide truncate">{label}</span>
                </div>
            </CollapsibleTrigger>
            <CollapsibleContent>
                {children}
            </CollapsibleContent>
        </Collapsible>
    );
};

const TreeNode: FC<TreeNodeProps> = ({ label, icon, children, level = 0, collapsible = false, defaultOpen = true, isLeaf = false }) => {
    const [open, setOpen] = useState(defaultOpen);
    const indentStyle = { paddingLeft: `${level * 1.25}rem` };

    const Trigger = collapsible ? CollapsibleTrigger : 'div';
    const Content = collapsible ? CollapsibleContent : 'div';

    const triggerContent = (
        <div className="flex items-center gap-2 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden" style={indentStyle}>
            {collapsible && !isLeaf && (open ? <ChevronDown size={14} className="flex-shrink-0" /> : <ChevronRight size={14} className="flex-shrink-0" />)}
            {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-sm font-normal truncate">{label}</span>
        </div>
    );

    if (collapsible) {
        return (
            <Collapsible open={open} onOpenChange={setOpen}>
                <Trigger asChild>
                    {triggerContent}
                </Trigger>
                <Content>
                    {children}
                </Content>
            </Collapsible>
        );
    } else if (isLeaf) {
        return triggerContent;
    } else {
        return (
            <>
                {triggerContent}
                {children}
            </>
        );
    }
};

// Type Avatar component
interface TypeAvatarProps {
    type: Type;
    showHoverCard?: boolean;
}

const TypeAvatar: FC<TypeAvatarProps> = ({ type, showHoverCard = false }) => {
    const { attributes, listeners, setNodeRef } = useDraggable({
        id: `type-${type.name}-${type.variant || ''}`,
    });

    const avatar = (
        <Avatar
            ref={setNodeRef}
            {...listeners}
            {...attributes}
            className="cursor-grab"
        >
            {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
            <AvatarFallback>{type.variant.substring(0, 2).toUpperCase()}</AvatarFallback>
        </Avatar>
    );

    if (!showHoverCard) {
        return avatar;
    }

    return (
        <HoverCard>
            <HoverCardTrigger asChild>
                {avatar}
            </HoverCardTrigger>
            <HoverCardContent className="w-80">
                <div className="space-y-1">
                    {type.variant ? (
                        <>
                            <h4 className="text-sm font-semibold">{type.variant}</h4>
                            <p className="text-sm">
                                {type.description || 'No description available.'}
                            </p>
                        </>
                    ) : (
                        <p className="text-sm">
                            {type.description || 'No description available.'}
                        </p>
                    )}
                </div>
            </HoverCardContent>
        </HoverCard>
    );
}

// Design Avatar component
interface DesignAvatarProps {
    design: Design;
    showHoverCard?: boolean;
}

const DesignAvatar: FC<DesignAvatarProps> = ({ design, showHoverCard = false }) => {
    const { attributes, listeners, setNodeRef } = useDraggable({
        id: `design-${design.name}-${design.variant || ''}-${design.view || ''}`,
    });

    // Determine if this is the default variant and view
    const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

    const avatar = (
        <Avatar
            ref={setNodeRef}
            {...listeners}
            {...attributes}
            className="cursor-grab"
        >
            {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
            <AvatarFallback>{design.variant.substring(0, 2).toUpperCase()}</AvatarFallback>
        </Avatar>
    );

    if (!showHoverCard) {
        return avatar;
    }

    return (
        <HoverCard>
            <HoverCardTrigger asChild>
                {avatar}
            </HoverCardTrigger>
            <HoverCardContent className="w-80">
                <div className="space-y-1">
                    {!isDefault && (
                        <h4 className="text-sm font-semibold">
                            {design.variant || design.name}
                            {design.view && design.view !== "Default" && ` (${design.view})`}
                        </h4>
                    )}
                    <p className="text-sm">
                        {design.description || 'No description available.'}
                    </p>
                </div>
            </HoverCardContent>
        </HoverCard>
    );
}

// Workbench panel component
interface WorkbenchProps extends ResizablePanelProps { }

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isResizing, setIsResizing] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsResizing(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth + (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsResizing(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    // const { kit } = useKit();
    const kit = useStudioStore().getKit("Metabolism", "r25.03-1");
    if (!kit?.types) return null;

    const typesByName = kit.types.reduce((acc, type) => {
        acc[type.name] = acc[type.name] || [];
        acc[type.name].push(type);
        return acc;
    }, {} as Record<string, Type[]>);

    const designsByName = kit.designs.reduce((acc, design) => {
        const nameKey = design.name;
        acc[nameKey] = acc[nameKey] || [];
        acc[nameKey].push(design);
        return acc;
    }, {} as Record<string, Design[]>);

    return (
        <div
            className={`absolute top-4 left-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-r-primary' : 'border-r'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div className="p-1">
                    <TreeSection label="Types" defaultOpen={true} >
                        {Object.entries(typesByName).map(([name, variants]) => (
                            <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
                                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                                    style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}>
                                    {variants.map((type) => (
                                        <TypeAvatar
                                            key={`${type.name}-${type.variant}`}
                                            type={type}
                                            showHoverCard={true}
                                        />
                                    ))}
                                </div>
                            </TreeNode>
                        ))}
                    </TreeSection>
                    <TreeSection label="Designs" defaultOpen={true}>
                        {Object.entries(designsByName).map(([name, designs]) => (
                            <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
                                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                                    style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}>
                                    {designs.map((design) => (
                                        <DesignAvatar
                                            key={`${design.name}-${design.variant}-${design.view}`}
                                            design={design}
                                            showHoverCard={true}
                                        />
                                    ))}
                                </div>
                            </TreeNode>
                        ))}
                    </TreeSection>
                </div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
            />
        </div>
    );
}

interface ConsoleProps {
    visible: boolean;
    leftPanelVisible: boolean;
    rightPanelVisible: boolean;
    leftPanelWidth?: number;
    rightPanelWidth?: number;
    height: number;
    setHeight: (height: number) => void;
}

const Console: FC<ConsoleProps> = ({ visible, leftPanelVisible, rightPanelVisible, leftPanelWidth = 230, rightPanelWidth = 230, height, setHeight }) => {
    if (!visible) return null;

    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isResizing, setIsResizing] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsResizing(true);

        const startY = e.clientY;
        const startHeight = height;

        const handleMouseMove = (e: MouseEvent) => {
            const newHeight = startHeight - (e.clientY - startY);
            if (newHeight >= 100 && newHeight <= 600) {
                setHeight(newHeight);
            }
        };

        const handleMouseUp = () => {
            setIsResizing(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute z-30 bg-background-level-2 text-foreground border ${isResizing || isResizeHovered ? 'border-t-primary' : ''}`}
            style={{
                left: leftPanelVisible ? `calc(${leftPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
                right: rightPanelVisible ? `calc(${rightPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
                bottom: `calc(var(--spacing) * 4)`,
                height: `${height}px`,
            }}
        >
            <div
                className={`absolute top-0 left-0 right-0 h-1 cursor-ns-resize`}
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
            />
            <ScrollArea className="h-full">
                <div className="font-semibold p-4">Console</div>
            </ScrollArea>
        </div>
    );
}

interface DetailsProps extends ResizablePanelProps { }

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isResizing, setIsResizing] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsResizing(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth - (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsResizing(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div id="type-properties" className="p-4 space-y-4">
                    <div className="space-y-2">
                        <label className="text-sm">Name</label>
                        <Input placeholder="Name" />
                    </div>
                    <div className="space-y-2">
                        <label className="text-sm">Rotation</label>
                        <Slider defaultValue={[33]} max={100} min={0} step={1} />
                    </div>
                    <div className="space-y-2">
                        <label className="text-sm">Description</label>
                        <Textarea />
                    </div>
                </div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
            />
        </div>
    );
}

interface ChatProps extends ResizablePanelProps { }

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isResizing, setIsResizing] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsResizing(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth - (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsResizing(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div className="font-semibold p-4">Chat</div>
                <div className="p-4 space-y-4">
                    <Textarea />
                </div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
            />
        </div>
    );
}

const DesignEditorCore: FC = () => {
    const [, forceUpdate] = useReducer(x => x + 1, 0);

    const studioStore = useStudioStore();
    const designEditorStore = useDesignEditorStore();

    useEffect(() => {
        const unsubscribe = designEditorStore.subscribe(() => {
            forceUpdate();
        });
        return () => unsubscribe();
    }, [designEditorStore]);

    const [fullscreenPanel, setFullscreenPanel] = useState<'diagram' | 'model' | null>(null);
    const { setNavbarToolbar } = useSketchpad();
    const [kitName, kitVersion] = designEditorStore.getKitId();
    const kit = studioStore.getKit(kitName, kitVersion);
    if (!kit) return null;

    if (!kit.types) throw new Error(`Kit ${kitName} ${kitVersion} has no types`);

    const [designName, designVariant, designView] = designEditorStore.getDesignId();
    const design = studioStore.getDesign(kitName, kitVersion, designName, designVariant, designView);

    const selection = designEditorStore.getState().selection;

    const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
        workbench: false,
        console: false,
        details: false,
        chat: false,
    });
    const [workbenchWidth, setWorkbenchWidth] = useState(230);
    const [detailsWidth, setDetailsWidth] = useState(230);
    const [consoleHeight, setConsoleHeight] = useState(200);
    const [chatWidth, setChatWidth] = useState(230);

    const togglePanel = (panel: keyof PanelToggles) => {
        setVisiblePanels(prev => {
            const newState = { ...prev };
            if (panel === 'chat' && !prev.chat) {
                newState.details = false;
            }
            if (panel === 'details' && !prev.details) {
                newState.chat = false;
            }
            newState[panel] = !prev[panel];
            return newState;
        });
    };

    useHotkeys('mod+j', (e) => { e.preventDefault(); e.stopPropagation(); togglePanel('workbench'); });
    useHotkeys('mod+k', (e) => { e.preventDefault(); e.stopPropagation(); togglePanel('console'); });
    useHotkeys('mod+l', (e) => { e.preventDefault(); e.stopPropagation(); togglePanel('details'); });
    useHotkeys(['mod+[', 'mod+semicolon', 'mod+ö'], (e) => { e.preventDefault(); e.stopPropagation(); togglePanel('chat'); });

    useHotkeys('mod+a', (e) => { e.preventDefault(); console.log('Select all pieces and connections'); });
    useHotkeys('mod+i', (e) => { e.preventDefault(); console.log('Invert selection'); });
    useHotkeys('mod+d', (e) => { e.preventDefault(); console.log('Select closest piece with same variant'); });
    useHotkeys('mod+shift+d', (e) => { e.preventDefault(); console.log('Select all pieces with same variant'); });
    useHotkeys('mod+c', (e) => { e.preventDefault(); console.log('Copy selected'); });

    useHotkeys('mod+v', (e) => { e.preventDefault(); console.log('Paste'); });
    useHotkeys('mod+x', (e) => { e.preventDefault(); console.log('Cut selected'); });
    useHotkeys('delete', (e) => {
        e.preventDefault();
        designEditorStore.transact(() => {
            designEditorStore.deleteSelectedPiecesAndConnections();
        });
    });
    useHotkeys('mod+y', (e) => {
        e.preventDefault(); designEditorStore.undo();
    });
    useHotkeys('mod+z', (e) => {
        e.preventDefault(); designEditorStore.redo();
    });
    useHotkeys('mod+w', (e) => { e.preventDefault(); console.log('Close design'); });

    const handlePanelDoubleClick = (panel: 'diagram' | 'model') => {
        setFullscreenPanel(currentPanel => currentPanel === panel ? null : panel);
    };

    const rightPanelVisible = visiblePanels.details || visiblePanels.chat;

    const designEditorToolbar = (
        <ToggleGroup
            type="multiple"
            value={Object.entries(visiblePanels)
                .filter(([_, isVisible]) => isVisible)
                .map(([key]) => key)}
            onValueChange={(values) => {
                Object.keys(visiblePanels).forEach(key => {
                    const isCurrentlyVisible = visiblePanels[key as keyof PanelToggles];
                    const shouldBeVisible = values.includes(key);
                    if (isCurrentlyVisible !== shouldBeVisible) {
                        togglePanel(key as keyof PanelToggles);
                    }
                });
            }}
        >
            <ToggleGroupItem value="workbench" tooltip="Workbench" hotkey="⌘J"><Wrench /></ToggleGroupItem>
            <ToggleGroupItem value="console" tooltip="Console" hotkey="⌘K"><Terminal /></ToggleGroupItem>
            <ToggleGroupItem value="details" tooltip="Details" hotkey="⌘L"><Info /></ToggleGroupItem>
            <ToggleGroupItem value="chat" tooltip="Chat" hotkey="⌘["><MessageCircle /></ToggleGroupItem>
        </ToggleGroup>
    );

    useEffect(() => {
        setNavbarToolbar(designEditorToolbar);
        return () => setNavbarToolbar(null);
    }, [setNavbarToolbar, visiblePanels]);

    const { screenToFlowPosition } = useReactFlow();
    const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
    const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);

    const createPiece = (piece: Piece) => {
        designEditorStore.transact(() => {
            studioStore.createPiece(kitName, kitVersion, designName, designVariant, designView, piece);
        });
    };

    const onDragStart = (event: DragStartEvent) => {
        const { active } = event;
        const id = active.id.toString();
        if (id.startsWith('type-')) {
            const [_, name, variant] = id.split('-');
            const type = kit?.types?.find((t: Type) => t.name === name && t.variant === (variant || undefined));
            setActiveDraggedType(type || null);
        } else if (id.startsWith('design-')) {
            const [_, name, variant, view] = id.split('-');
            const draggedDesign = kit?.designs?.find(d => d.name === name && d.variant === (variant || undefined) && d.view === (view || undefined));
            setActiveDraggedDesign(draggedDesign || null);
        }
    };

    const onDragEnd = (event: DragEndEvent) => {
        const { over } = event;
        if (over?.id === 'diagram') {
            if (!(event.activatorEvent instanceof PointerEvent)) {
                return;
            }
            if (activeDraggedType) {
                const { x, y } = screenToFlowPosition({
                    x: event.activatorEvent.clientX + event.delta.x,
                    y: event.activatorEvent.clientY + event.delta.y
                });
                const piece: Piece = {
                    id_: Generator.randomId(),
                    type: {
                        name: activeDraggedType.name,
                        variant: activeDraggedType.variant
                    },
                    plane: {
                        origin: { x: 0, y: 0, z: 0 },
                        xAxis: { x: 1, y: 0, z: 0 },
                        yAxis: { x: 0, y: 1, z: 0 }
                    },
                    center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 }
                };
                designEditorStore.transact(() => {
                    createPiece(piece);
                });
            } else if (activeDraggedDesign) {
                const correspondingType = kit?.types?.find(t => t.name === activeDraggedDesign.name && t.variant === activeDraggedDesign.variant);
                if (correspondingType) {
                    const piece: Piece = {
                        id_: Generator.randomId(),
                        description: activeDraggedDesign.description || '',
                        type: {
                            name: correspondingType.name,
                            variant: correspondingType.variant
                        }
                    };
                    designEditorStore.transact(() => {
                        createPiece(piece);
                    });
                } else {
                    console.warn(`Could not find corresponding Type for dragged Design: ${activeDraggedDesign.name} / ${activeDraggedDesign.variant}`);
                }
            }
        }
        setActiveDraggedType(null);
        setActiveDraggedDesign(null);
    };

    const fileUrls = studioStore.getFileUrls()
    return (
        <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>

            <div className="canvas flex-1 relative">
                <div id="sketchpad-edgeless" className="h-full">
                    <ResizablePanelGroup direction="horizontal">
                        <ResizablePanel
                            defaultSize={fullscreenPanel === 'diagram' ? 100 : 50}
                            className={`${fullscreenPanel === 'model' ? 'hidden' : 'block'}`}
                            onDoubleClick={() => handlePanelDoubleClick('diagram')}
                        >
                            {design ? (
                                <Diagram
                                    fullscreen={fullscreenPanel === 'diagram'}
                                    onPanelDoubleClick={() => handlePanelDoubleClick('diagram')}
                                    design={design}
                                    types={kit?.types ?? []}
                                    selection={selection}
                                    fileUrls={fileUrls}
                                    onSelectionChange={designEditorStore.updateDesignEditorSelection}
                                />
                            ) : (
                                <div className="flex items-center justify-center h-full text-muted-foreground">No design loaded</div>
                            )}
                        </ResizablePanel>
                        <ResizableHandle className={`border-r ${fullscreenPanel !== null ? 'hidden' : 'block'}`} />
                        <ResizablePanel
                            defaultSize={fullscreenPanel === 'model' ? 100 : 50}
                            className={`${fullscreenPanel === 'diagram' ? 'hidden' : 'block'}`}
                            onDoubleClick={() => handlePanelDoubleClick('model')}
                        >
                            {design ? (
                                <Model
                                    fullscreen={fullscreenPanel === 'model'}
                                    onPanelDoubleClick={() => handlePanelDoubleClick('model')}
                                    design={design}
                                    types={kit?.types ?? []}
                                    selection={selection}
                                    fileUrls={fileUrls}
                                    onSelectionChange={designEditorStore.updateDesignEditorSelection}
                                />
                            ) : (
                                <div className="flex items-center justify-center h-full text-muted-foreground">No design loaded</div>
                            )}
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>
                <Workbench
                    visible={visiblePanels.workbench}
                    onWidthChange={setWorkbenchWidth}
                    width={workbenchWidth}
                />
                <Details
                    visible={visiblePanels.details}
                    onWidthChange={setDetailsWidth}
                    width={detailsWidth}
                />
                <Console
                    visible={visiblePanels.console}
                    leftPanelVisible={visiblePanels.workbench}
                    rightPanelVisible={rightPanelVisible}
                    leftPanelWidth={workbenchWidth}
                    rightPanelWidth={detailsWidth}
                    height={consoleHeight}
                    setHeight={setConsoleHeight}
                />
                <Chat
                    visible={visiblePanels.chat}
                    onWidthChange={setChatWidth}
                    width={chatWidth}
                />
                {createPortal(
                    <DragOverlay>
                        {activeDraggedType && (<TypeAvatar type={activeDraggedType} />)}
                        {activeDraggedDesign && (<DesignAvatar design={activeDraggedDesign} />)}
                    </DragOverlay>,
                    document.body
                )}
            </div>
        </DndContext>
    )
}

interface DesignEditorProps { }

const DesignEditor: FC<DesignEditorProps> = () => {

    return (
        <ReactFlowProvider>
            <DesignEditorCore />
        </ReactFlowProvider>
    );
};

export default DesignEditor;