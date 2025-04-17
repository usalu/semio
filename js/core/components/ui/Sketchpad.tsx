import { FC, Suspense, ReactNode, useState, useEffect, createContext, useContext } from 'react';
import { Folder, FlaskConical, ChevronDown, ChevronRight, Wrench, Terminal, Info, ChevronDownIcon, Share2, Minus, Square, X, MessageCircle, Home, Sun, Moon, Monitor, Sofa, Glasses, AppWindow } from 'lucide-react';
import {
    DndContext,
    DragEndEvent,
    DragOverlay,
    DragStartEvent,
    useDraggable,
    useDroppable
} from '@dnd-kit/core';
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable"
import { Avatar, AvatarFallback, AvatarImage } from "@semio/js/components/ui/Avatar";
import { default as Diagram } from "@semio/js/components/ui/Diagram";
import { default as Model } from "@semio/js/components/ui/Model";
import { Design, Kit, Type } from '@semio/js';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@semio/js/components/ui/Tooltip"
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup"
import { ToggleCycle } from "@semio/js/components/ui/ToggleCycle"
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { createPortal } from 'react-dom';
import { useAtomValue } from 'jotai';
import { useKit, useDesign, DesignProvider, KitProvider, StudioProvider } from '@semio/js/store';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@semio/js/components/ui/Breadcrumb';
import { Button } from "@semio/js/components/ui/Button";
import { useHotkeys } from 'react-hotkeys-hook';
import { Toggle } from '@semio/js/components/ui/Toggle';
import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';
import { default as nakaginCapsuleTower } from '@semio/assets/semio/design_nakagin-capsule-tower_flat.json';
import { Fingerprint } from 'lucide-react';

export enum Mode {
    USER = 'user',
    GUEST = 'guest',
}

export enum Theme {
    SYSTEM = 'system',
    LIGHT = 'light',
    DARK = 'dark',
}

export enum Layout {
    NORMAL = 'normal',
    TOUCH = 'touch',
}

interface SketchpadContextType {
    mode: Mode;
    layout: Layout;
    setLayout: (layout: Layout) => void;
    theme: Theme;
    setTheme: (theme: Theme) => void;
    setNavbarToolbar: (toolbar: ReactNode) => void;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);


export const useSketchpad = () => {
    const context = useContext(SketchpadContext);
    if (!context) {
        throw new Error('useSketchpad must be used within a SketchpadProvider');
    }
    return context;
};

interface TypeAvatarProps {
    type: Type
}
const TypeAvatar: FC<TypeAvatarProps> = ({ type }) => {
    const { attributes, listeners, setNodeRef, transform } = useDraggable({
        id: 'type-' + type.name + type.variant,
    });
    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <Avatar
                    ref={setNodeRef}
                    // className="cursor-pointer"
                    {...listeners}
                    {...attributes}>
                    {/* <AvatarImage src={"../../../../examples/metabolism/" + type.icon} /> */}
                    <AvatarImage src="https://github.com/semio-tech.png" />
                    {/* <AvatarFallback>{type.name}</AvatarFallback> */}
                </Avatar>
            </TooltipTrigger>
            <TooltipContent>
                {type.description}
            </TooltipContent>
        </Tooltip>
    );
}

const Types: FC = () => {
    const kit = useKit();
    if (!kit) return null;

    return (
        <div className="h-auto overflow-auto grid grid-cols-[repeat(auto-fill,minmax(40px,1fr))] auto-rows-[40px] p-1">
            {kit.types?.map((type) => (
                <TypeAvatar key={type.name} type={type} />
            ))}
        </div>
    );
}

interface DesignAvatarProps {
    design: Design
}
const DesignAvatar: FC<DesignAvatarProps> = ({ design }) => {
    const { attributes, listeners, setNodeRef, transform } = useDraggable({
        id: 'design-' + design.name,
    });
    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <Avatar
                    ref={setNodeRef}
                    // className="cursor-pointer"
                    {...listeners}
                    {...attributes}>
                    {/* <AvatarImage src={"../../../../examples/metabolism/" + type.icon} /> */}
                    <AvatarImage src="https://github.com/semio-tech.png" />
                    {/* <AvatarFallback>{type.name}</AvatarFallback> */}
                </Avatar>
            </TooltipTrigger>
            <TooltipContent>
                {design.description}
            </TooltipContent>
        </Tooltip>
    );
}

const Designs: FC = () => {
    const kit = useKit();
    if (!kit) return null;

    return (
        <div className="h-auto overflow-auto grid grid-cols-[repeat(auto-fill,minmax(40px,1fr))] auto-rows-[40px] p-1">
            {kit.designs?.map((design) => (
                <DesignAvatar key={design.name} design={design} />
            ))}
        </div>
    );
}


interface TreeSectionProps {
    name: string;
    children: ReactNode;
}


const TreeSection: FC<TreeSectionProps> = ({ name, children }) => {
    const [open, setOpen] = useState(true);
    return (
        <Collapsible className="p-3 border-b-thin font-thin uppercase"
            open={open}
            onOpenChange={setOpen}>
            <CollapsibleTrigger className="flex items-center justify-between">
                {open ? <ChevronDown size={14} className="w-3.5 h-3.5" /> : <ChevronRight size={14} className="w-3.5 h-3.5" />}
                {name}
            </CollapsibleTrigger>
            <CollapsibleContent>
                {children}
            </CollapsibleContent>
        </Collapsible>
    );
}

interface NavbarProps {
    toolbarContent?: ReactNode;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Navbar: FC<NavbarProps> = ({ toolbarContent, onWindowEvents }) => {
    const { mode, layout, setLayout, theme, setTheme } = useSketchpad();

    return (
        <div className={`w-full h-12 bg-background border-b flex items-center justify-between px-4`}>
            <div className="flex items-center">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/"><Home size={16} /></BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator items={[
                            { label: "Starter", href: "/metabolism/starter" },
                            { label: "Geometry", href: "/metabolism/geometry" }
                        ]} onNavigate={(href) => console.log('Navigate to:', href)} />
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator items={[
                            { label: "Types", href: "/designs/types" },
                            { label: "Representations", href: "/designs/representations" }
                        ]} onNavigate={(href) => console.log('Navigate to:', href)} />
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/designs">Designs</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator items={[
                            { label: "Capsule Dream", href: "/designs/nakagin/capsule-dream" }
                        ]} onNavigate={(href) => console.log('Navigate to:', href)} />
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/designs/nakagin">Nakagin Capsule Tower</BreadcrumbLink>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </div>
            <div className="flex items-center gap-4">
                {toolbarContent}
                <ToggleCycle
                    value={theme}
                    onValueChange={setTheme}
                    items={[
                        {
                            value: Theme.LIGHT,
                            tooltip: "Turn Theme Dark",
                            label: <Moon />
                        },
                        {
                            value: Theme.DARK,
                            tooltip: "Turn Theme Light",
                            label: <Sun />
                        }
                    ]}
                />
                <ToggleCycle
                    value={layout}
                    onValueChange={setLayout}
                    items={[
                        {
                            value: Layout.NORMAL,
                            tooltip: "Turn to Touch Layout",
                            label: <Fingerprint />
                        },
                        {
                            value: Layout.TOUCH,
                            tooltip: "Turn to Normal Layout",
                            label: <AppWindow />
                        }
                    ]}
                />

                <Avatar className="h-8 w-8">
                    <AvatarImage src="https://github.com/usalu.png" />
                    <AvatarFallback>US</AvatarFallback>
                </Avatar>

                <Toggle
                    variant="outline"
                    tooltip="Share"
                >
                    <Share2 />
                </Toggle>

                {onWindowEvents && (
                    <div className="flex items-center gap-2 ml-4">
                        <ToggleGroup type="single">
                            <ToggleGroupItem
                                value="minimize"
                                onClick={onWindowEvents.minimize}
                            >
                                <Minus size={16} />
                            </ToggleGroupItem>
                            <ToggleGroupItem
                                value="maximize"
                                onClick={onWindowEvents.maximize}
                            >
                                <Square size={16} />
                            </ToggleGroupItem>
                            <ToggleGroupItem
                                value="close"
                                onClick={onWindowEvents.close}
                                className="hover:bg-danger"
                            >
                                <X size={16} />
                            </ToggleGroupItem>
                        </ToggleGroup>
                    </div>
                )}
            </div>
        </div>
    );
};

interface PanelProps {
    visible: boolean;
}

interface ResizablePanelProps extends PanelProps {
    onWidthChange?: (width: number) => void;
    width: number;
}

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
            className={`absolute top-4 left-4 bottom-4 z-100 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-r-primary' : 'border-r-border'}`}
            style={{ width: `${width}px` }}
        >
            <TreeSection name="Types">
                <Types />
            </TreeSection>
            <TreeSection name="Designs">
                <Designs />
            </TreeSection>
            <div
                className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
        </div>
    );
}

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
            className={`absolute top-4 right-4 bottom-4 z-100 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-l-primary' : 'border-l-border'}`}
            style={{ width: `${width}px` }}
        >
            <div className="font-semibold p-4">Details</div>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
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
            className={`absolute z-[150] bg-background-level-2 text-foreground border ${isDragging || isResizeHovered ? 'border-t-primary' : ''}`}
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
            <div className="font-semibold p-4">Console</div>
        </div>
    );
}

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
            className={`absolute top-4 right-4 bottom-4 z-100 bg-background-level-2 text-foreground border
                ${isDragging || isResizeHovered ? 'border-l-primary' : 'border-l-border'}`}
            style={{ width: `${width}px` }}
        >
            <div className="font-semibold p-4">Chat</div>
            <div
                className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => !isDragging && setIsResizeHovered(false)}
            />
        </div>
    );
}

interface PanelToggles {
    workbench: boolean;
    console: boolean;
    details: boolean;
    chat: boolean;
}

interface DesignEditorProps {
}
const DesignEditor: FC<DesignEditorProps> = ({ }) => {
    const [fullscreenPanel, setFullscreenPanel] = useState<'diagram' | 'model' | null>(null);
    const { setNavbarToolbar } = useSketchpad();

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

    const design = useDesign();

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
        console.log('onDragStart', event);
    };

    const onDragEnd = (event: DragEndEvent) => {
        console.log('onDragEnd', event);
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
                            <Diagram
                                fullscreen={fullscreenPanel === 'diagram'}
                                onPanelDoubleClick={() => handlePanelDoubleClick('diagram')}
                                design={design}
                            />
                        </ResizablePanel>
                        <ResizableHandle className={`border-r ${fullscreenPanel !== null ? 'hidden' : 'block'}`} />
                        <ResizablePanel
                            defaultSize={fullscreenPanel === 'model' ? 100 : 50}
                            className={`${fullscreenPanel === 'diagram' ? 'hidden' : 'block'}`}
                            onDoubleClick={() => handlePanelDoubleClick('model')}
                        >
                            <Model
                                fullscreen={fullscreenPanel === 'model'}
                                onPanelDoubleClick={() => handlePanelDoubleClick('model')}
                                design={design}
                            />
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
                        {activeDraggedType && (<TypeAvatar type={activeDraggedType} />
                        {activeDraggedDesign && (<DesignAvatar design={activeDraggedDesign} />)}
                    </DragOverlay>,
                    document.body
                )}
            </div>
        </DndContext>
    );
};

interface SketchpadProps {
    mode?: Mode;
    theme?: Theme;
    layout?: Layout;
    readonly?: boolean;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.USER, theme, layout = Layout.NORMAL, onWindowEvents }) => {
    const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);
    const [currentLayout, setCurrentLayout] = useState<Layout>(layout);

    const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
        if (theme) return theme;
        if (typeof window !== 'undefined') {
            return window.matchMedia('(prefers-color-scheme: dark)').matches ? Theme.DARK : Theme.LIGHT;
        }
        return Theme.LIGHT;
    });

    useEffect(() => {
        const root = window.document.documentElement;
        root.classList.remove(Theme.DARK);
        if (currentTheme === Theme.DARK) {
            root.classList.add(Theme.DARK);
        }
    }, [currentTheme]);

    useEffect(() => {
        const root = window.document.documentElement;
        root.classList.remove(Layout.TOUCH);
        if (currentLayout === Layout.TOUCH) {
            root.classList.add(Layout.TOUCH);
        }
    }, [currentLayout]);

    const ActiveView = DesignEditor;


    return (
        <TooltipProvider>
            <StudioProvider>
                <SketchpadContext.Provider value={{
                    mode: mode,
                    layout: currentLayout,
                    setLayout: setCurrentLayout,
                    theme: currentTheme,
                    setTheme: setCurrentTheme,
                    setNavbarToolbar: setNavbarToolbar,
                }}>
                    <KitProvider kit={metabolism}>
                        <DesignProvider design={nakaginCapsuleTower}>
                            <div
                                key={`layout-${currentLayout}`} // Force unmount/remount on layout change because some components are not responsive
                                className="h-full w-full flex flex-col bg-background text-foreground"
                            >
                                <Navbar
                                    toolbarContent={navbarToolbar}
                                    onWindowEvents={onWindowEvents}
                                />
                                <ActiveView />
                            </div>
                        </DesignProvider>
                    </KitProvider>
                </SketchpadContext.Provider>
            </StudioProvider>
        </TooltipProvider>
    );
};

export default Sketchpad;