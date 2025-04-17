import { FC, ReactNode, useState, useEffect } from 'react';
import {
    DndContext,
    DragEndEvent,
    DragOverlay,
    DragStartEvent,
    useDraggable
} from '@dnd-kit/core';
import { createPortal } from 'react-dom';
import { useHotkeys } from 'react-hotkeys-hook';
import { Wrench, Terminal, Info, MessageCircle, ChevronDown, ChevronRight, Folder } from 'lucide-react';

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable";
import { Avatar, AvatarFallback, AvatarImage } from "@semio/js/components/ui/Avatar";
import { default as Diagram } from "@semio/js/components/ui/Diagram";
import { default as Model } from "@semio/js/components/ui/Model";
import { Tooltip, TooltipContent, TooltipTrigger } from "@semio/js/components/ui/Tooltip";
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { ScrollArea } from '@semio/js/components/ui/ScrollArea';
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@semio/js/components/ui/HoverCard";

import { Design, Type, Piece } from '@semio/js';
import { Generator } from '@semio/js/lib/utils';
import { useKit, useDesign } from '@semio/js/store';
import { useSketchpad } from './Sketchpad';

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
    type: Type
}

const TypeAvatar: FC<TypeAvatarProps> = ({ type }) => {
    const { attributes, listeners, setNodeRef } = useDraggable({
        id: `type-${type.name}-${type.variant || ''}`,
    });
    return (
        <HoverCard>
            <HoverCardTrigger asChild>
                <Avatar
                    ref={setNodeRef}
                    {...listeners}
                    {...attributes}
                    className="cursor-grab"
                >
                    <AvatarImage src="https://github.com/semio-tech.png" />
                    <AvatarFallback>{type.name.substring(0, 2).toUpperCase()}</AvatarFallback>
                </Avatar>
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

// Designs component for displaying designs in the workbench
const Designs: FC = () => {
    const { kit } = useKit();
    if (!kit?.designs) return null;

    // Group designs by name
    const designsByName = kit.designs.reduce((acc, design) => {
        const nameKey = design.name;
        acc[nameKey] = acc[nameKey] || [];
        acc[nameKey].push(design);
        return acc;
    }, {} as Record<string, Design[]>);

    return (
        <TreeNode label="Designs" collapsible={true} level={0} defaultOpen={true} icon={<Folder size={14} />}>
            {Object.entries(designsByName).map(([name, designs]) => (
                <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false} icon={<Folder size={14} />}>
                    <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                        style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}>
                        {designs.map((design) => (
                            <DesignAvatar key={`${design.name}-${design.variant}-${design.view}`} design={design} />
                        ))}
                    </div>
                </TreeNode>
            ))}
        </TreeNode>
    );
}

// Types component for displaying types in the workbench
const Types: FC = () => {
    const { kit } = useKit();
    if (!kit?.types) return null;

    const typesByName = kit.types.reduce((acc, type) => {
        acc[type.name] = acc[type.name] || [];
        acc[type.name].push(type);
        return acc;
    }, {} as Record<string, Type[]>);

    return (
        <TreeNode label="Types" collapsible={true} level={0} defaultOpen={true} icon={<Folder size={14} />}>
            {Object.entries(typesByName).map(([name, variants]) => (
                <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false} icon={<Folder size={14} />}>
                    <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1" style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}>
                        {variants.map((type) => (
                            <TypeAvatar key={`${type.name}-${type.variant}`} type={type} />
                        ))}
                    </div>
                </TreeNode>
            ))}
        </TreeNode>
    );
}

// Design Avatar component
interface DesignAvatarProps {
    design: Design
}

const DesignAvatar: FC<DesignAvatarProps> = ({ design }) => {
    const { attributes, listeners, setNodeRef } = useDraggable({
        id: `design-${design.name}-${design.variant || ''}-${design.view || ''}`,
    });

    // Determine if this is the default variant and view
    const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

    return (
        <HoverCard>
            <HoverCardTrigger asChild>
                <Avatar
                    ref={setNodeRef}
                    {...listeners}
                    {...attributes}
                    className="cursor-grab"
                >
                    <AvatarImage src="https://github.com/semio-tech.png" />
                    <AvatarFallback>{design.name.substring(0, 2).toUpperCase()}</AvatarFallback>
                </Avatar>
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
    const [isDragging, setIsDragging] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsDragging(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth + (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsDragging(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 left-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-r-primary' : 'border-r'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div className="p-1">
                    <Types />
                    <Designs />
                </div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
        </div>
    );
}

// Details panel component
interface DetailsProps extends ResizablePanelProps { }

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isDragging, setIsDragging] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsDragging(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth - (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsDragging(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div className="font-semibold p-4">Details</div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
        </div>
    );
}

// Console panel component
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
    const [isDragging, setIsDragging] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsDragging(true);

        const startY = e.clientY;
        const startHeight = height;

        const handleMouseMove = (e: MouseEvent) => {
            const newHeight = startHeight - (e.clientY - startY);
            if (newHeight >= 100 && newHeight <= 600) {
                setHeight(newHeight);
            }
        };

        const handleMouseUp = () => {
            setIsDragging(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute z-30 bg-background-level-2 text-foreground border ${isDragging || isResizeHovered ? 'border-t-primary' : ''}`}
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
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
            <ScrollArea className="h-full">
                <div className="font-semibold p-4">Console</div>
            </ScrollArea>
        </div>
    );
}

// Chat panel component
interface ChatProps extends ResizablePanelProps { }

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const [isDragging, setIsDragging] = useState(false);

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();
        setIsDragging(true);

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth - (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            setIsDragging(false);
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
            style={{ width: `${width}px` }}
        >
            <ScrollArea className="h-full">
                <div className="font-semibold p-4">Chat</div>
            </ScrollArea>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
        </div>
    );
}

// Main DesignEditor component
interface DesignEditorProps { }

const DesignEditor: FC<DesignEditorProps> = () => {
    const [fullscreenPanel, setFullscreenPanel] = useState<'diagram' | 'model' | null>(null);
    const { setNavbarToolbar } = useSketchpad();
    const { kit } = useKit();
    const { design, createPiece } = useDesign();

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

    useHotkeys('ctrl+a', (e) => { e.preventDefault(); console.log('Select all pieces and connections'); });
    useHotkeys('ctrl+i', (e) => { e.preventDefault(); console.log('Invert selection'); });
    useHotkeys('ctrl+d', (e) => { e.preventDefault(); console.log('Select closest piece with same variant'); });
    useHotkeys('ctrl+shift+d', (e) => { e.preventDefault(); console.log('Select all pieces with same variant'); });
    useHotkeys('ctrl+c', (e) => { e.preventDefault(); console.log('Copy selected'); });

    useHotkeys('ctrl+v', (e) => { e.preventDefault(); console.log('Paste'); });
    useHotkeys('ctrl+x', (e) => { e.preventDefault(); console.log('Cut selected'); });
    useHotkeys('delete', (e) => { e.preventDefault(); console.log('Delete selected'); });
    useHotkeys('ctrl+z', (e) => { e.preventDefault(); console.log('Undo'); });
    useHotkeys('ctrl+y', (e) => { e.preventDefault(); console.log('Redo'); });
    useHotkeys('ctrl+s', (e) => { e.preventDefault(); console.log('Save stash'); });
    useHotkeys('ctrl+w', (e) => { e.preventDefault(); console.log('Close design'); });

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

    const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
    const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);

    const onDragStart = (event: DragStartEvent) => {
        const { active } = event;
        const idStr = active.id.toString();
        if (idStr.startsWith('type-')) {
            const [_, name, variant] = idStr.split('-');
            const type = kit?.types?.find((t: Type) => t.name === name && t.variant === (variant || undefined));
            setActiveDraggedType(type || null);
        } else if (idStr.startsWith('design-')) {
            const [_, name, variant, view] = idStr.split('-');
            const draggedDesign = kit?.designs?.find(d => d.name === name && d.variant === (variant || undefined) && d.view === (view || undefined));
            setActiveDraggedDesign(draggedDesign || null);
        }
    };

    const onDragEnd = (event: DragEndEvent) => {
        const { over } = event;
        if (over?.id === 'sketchpad-edgeless') {
            if (activeDraggedType) {
                const piece: Piece = {
                    id_: Generator.randomId(),
                    description: activeDraggedType.description || '',
                    type: {
                        name: activeDraggedType.name,
                        variant: activeDraggedType.variant
                    }
                };
                createPiece(piece);
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
                    createPiece(piece);
                } else {
                    console.warn(`Could not find corresponding Type for dragged Design: ${activeDraggedDesign.name} / ${activeDraggedDesign.variant}`);
                }
            }
        }
        setActiveDraggedType(null);
        setActiveDraggedDesign(null);
    };

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
                        {activeDraggedType && (
                            <Avatar>
                                <AvatarImage src="https://github.com/semio-tech.png" />
                                <AvatarFallback>{activeDraggedType.name.substring(0, 2).toUpperCase()}</AvatarFallback>
                            </Avatar>
                        )}
                        {activeDraggedDesign && (
                            <Avatar>
                                <AvatarImage src="https://github.com/semio-tech.png" />
                                <AvatarFallback>{activeDraggedDesign.name.substring(0, 2).toUpperCase()}</AvatarFallback>
                            </Avatar>
                        )}
                    </DragOverlay>,
                    document.body
                )}
            </div>
        </DndContext>
    );
};

export default DesignEditor;