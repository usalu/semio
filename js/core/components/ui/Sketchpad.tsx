import { FC, Suspense, ReactNode, useState, useEffect, createContext, useContext } from 'react';
import { Provider as JotaiProvider } from 'jotai';
import { Folder, FlaskConical, ChevronDown, ChevronRight, Wrench, Terminal, Info, ChevronDownIcon, Share2, Minus, Square, X, MessageCircle, Home, Sun, Moon, Monitor } from 'lucide-react';
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
import { useTypes } from '@semio/js/store';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@semio/js/components/ui/Breadcrumb';
import { Button } from "@semio/js/components/ui/Button";
import { useHotkeys } from 'react-hotkeys-hook';
import { Toggle } from '@semio/js/components/ui/Toggle';

interface SketchpadContextType {
    setNavbarToolbar: (toolbar: ReactNode) => void;
}

const SketchpadContext = createContext<SketchpadContextType | null>(null);

const useSketchpad = () => {
    const context = useContext(SketchpadContext);
    if (!context) {
        throw new Error('useSketchpad must be used within a SketchpadProvider');
    }
    return context;
};

type TreeSection = {
    name: string;
    children: ReactNode;
}

type Tree = {
    id: string;
    name: string;
    icon: ReactNode;
    sections: TreeSection[];
}

interface TypeAvatarProps {
    type: Type
}
const TypeAvatar: FC<TypeAvatarProps> = ({ type }) => {
    const { attributes, listeners, setNodeRef, transform } = useDraggable({
        id: 'type-' + type.name + type.variant,
    });
    return (
        <Avatar
            ref={setNodeRef}
            // className="cursor-pointer"
            {...listeners}
            {...attributes}>
            {/* <AvatarImage src={"../../../../examples/metabolism/" + type.icon} /> */}
            <AvatarImage src="https://github.com/shadcn.png" />
            {/* <AvatarFallback>{type.name}</AvatarFallback> */}
        </Avatar>
    );
}

const Types: FC = () => {
    const types = useTypes();
    if (!types) return null;

    return (
        <div className="h-auto overflow-auto grid grid-cols-[repeat(auto-fill,minmax(40px,1fr))] auto-rows-[40px] p-1">
            {Array.from(types.entries()).map(([name, variantMap]) => (
                Array.from(variantMap.entries()).map(([variant, type]: [string, Type]) => (
                    <TypeAvatar key={`${name}-${variant}`} type={type} />
                ))
            ))}
        </div>
    );
}

const ExplorerTree: Tree = {
    id: 'explorer',
    name: 'Explorer',
    icon: <Folder size={14} className="w-3.5 h-3.5" />,
    sections: [
        {
            name: 'Types',
            children: <Types />,
        },
    ],
}

const TestTree: Tree = {
    id: 'test',
    name: 'Test',
    icon: <FlaskConical size={14} className="w-3.5 h-3.5" />,
    sections: [],
}

const trees = [
    ExplorerTree,
    TestTree,
]

const TreeSectionComponent: FC<{ section: TreeSection }> = ({ section }) => {
    const [open, setOpen] = useState(true);
    return (
        <Collapsible className="p-3 border-b-thin font-thin uppercase"
            open={open}
            onOpenChange={setOpen}>
            <CollapsibleTrigger className="flex items-center justify-between">
                {open ? <ChevronDown size={14} className="w-3.5 h-3.5" /> : <ChevronRight size={14} className="w-3.5 h-3.5" />}
                {section.name}
            </CollapsibleTrigger>
            <CollapsibleContent>
                {section.children}
            </CollapsibleContent>
        </Collapsible>
    );
};

interface TreeProps {
    treeId: string;
}
const TreeComponent: FC<TreeProps> = ({ treeId }) => {
    const tree = trees.find(tree => tree.id === treeId);
    if (!tree) return null;

    return (
        <ResizablePanel defaultSize={15}>
            {tree.sections.map((section, index) => (
                <TreeSectionComponent key={index} section={section} />
            ))}
        </ResizablePanel>
    );
};


interface TreeBarProps {
    activeTreeId?: string;
    onTreeSelect?: (treeId: string) => void;
}
const TreeBar: FC<TreeBarProps> = ({ activeTreeId, onTreeSelect }) => {
    return (
        <div className="flex h-full w-12 flex-col items-end justify-top border-r">
            {trees.map((tree) => (
                <div
                    key={tree.id}
                    className={`w-12 h-12 flex items-center justify-center cursor-pointer ${tree.id === activeTreeId ? 'border-l-3 border-primary' : ''
                        }`}
                    onClick={() => onTreeSelect?.(tree.id)}
                >
                    {tree.icon}
                </div>
            ))}
        </div>
    );
};

interface TreeSiderProps {
}
const TreeSider: FC<TreeSiderProps> = ({ }) => {

    const [activeTreeId, setActiveTreeId] = useState('explorer');

    return (
        <>
            <TreeBar activeTreeId={activeTreeId} onTreeSelect={setActiveTreeId} />
            <TreeComponent treeId={activeTreeId} />
        </>
    );
};

interface NavbarProps {
    toolbarContent?: ReactNode;
    readonly?: boolean;
    currentTheme: Theme;
    onToggleTheme: () => void;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Navbar: FC<NavbarProps> = ({ toolbarContent, onWindowEvents, readonly, currentTheme, onToggleTheme }) => {
    const handleThemeChange = (value: string) => {
        onToggleTheme();
    };

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
            </div>

            <div className="flex items-center gap-4">
                <ToggleCycle
                    value={currentTheme}
                    onValueChange={handleThemeChange}
                    items={[
                        {
                            value: Theme.LIGHT,
                            tooltip: "Turn Dark",
                            label: <Moon />
                        },
                        {
                            value: Theme.DARK,
                            tooltip: "Turn Light",
                            label: <Sun />
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

interface WorkbenchProps extends PanelProps {
    onWidthChange?: (width: number) => void;
    width: number;
}

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width }) => {
    if (!visible) return null;
    const [isResizeHovered, setIsResizeHovered] = useState(false);
    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();

        const startX = e.clientX;
        const startWidth = width;

        const handleMouseMove = (e: MouseEvent) => {
            const newWidth = startWidth + (e.clientX - startX);
            if (newWidth >= 150 && newWidth <= 500) {
                onWidthChange?.(newWidth);
            }
        };

        const handleMouseUp = () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute top-4 left-4 bottom-4 z-100 bg-background-level-2 text-foreground border
                ${isResizeHovered ? 'border-r-primary' : 'border-r-border'}`}
            style={{ width: `${width}px` }}
        >
            <div className="font-semibold p-4 cursor-default">Workbench</div>
            <div
                className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => setIsResizeHovered(false)}
            />
        </div>
    );
}

const Details: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div
            className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border"
        >
            <div className="font-semibold p-4">Details</div>
        </div>
    );
}

interface ConsoleProps {
    visible: boolean;
    workbenchVisible: boolean;
    detailsOrChatVisible: boolean;
    workbenchWidth?: number;
}

const Console: FC<ConsoleProps> = ({ visible, workbenchVisible, detailsOrChatVisible, workbenchWidth = 230 }) => {
    if (!visible) return null;

    const [height, setHeight] = useState(200);
    const [isResizeHovered, setIsResizeHovered] = useState(false);

    // Consistent spacing (16px / Tailwind spacing-4)
    const spacing = 16;
    // Additional spacing specifically for horizontal gaps
    const horizontalGap = spacing * 2; // Double the spacing (32px) for horizontal gaps
    const detailsChatWidth = 230; // Width of Details/Chat panels

    const handleMouseDown = (e: React.MouseEvent) => {
        e.preventDefault();

        const startY = e.clientY;
        const startHeight = height;

        const handleMouseMove = (e: MouseEvent) => {
            const newHeight = startHeight - (e.clientY - startY);
            if (newHeight >= 100 && newHeight <= 600) {
                setHeight(newHeight);
            }
        };

        const handleMouseUp = () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    };

    return (
        <div
            className={`absolute z-[150] bg-background-level-2 text-foreground border ${isResizeHovered ? 'border-t-primary' : ''}`}
            style={{
                left: workbenchVisible ? `${workbenchWidth + horizontalGap}px` : `${spacing}px`,
                right: detailsOrChatVisible ? `${detailsChatWidth + horizontalGap}px` : `${spacing}px`,
                bottom: `${spacing}px`,
                height: `${height}px`,
            }}
        >
            <div
                className={`absolute top-0 left-0 right-0 h-1 cursor-ns-resize`}
                onMouseDown={handleMouseDown}
                onMouseEnter={() => setIsResizeHovered(true)}
                onMouseLeave={() => setIsResizeHovered(false)}
            />
            <div className="font-semibold p-4">Console</div>
        </div>
    );
}

const Chat: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border"
        >
            <div className="font-semibold p-4">Chat</div>
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

    const detailsOrChatVisible = visiblePanels.details || visiblePanels.chat;

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
        // Cleanup function to clear toolbar when component unmounts or view changes
        return () => setNavbarToolbar(null);
        // Rerun effect if visibility state changes, as ToggleGroup depends on it
    }, [setNavbarToolbar, visiblePanels]);

    return (
        <div className="canvas flex-1 relative">
            <div id="sketchpad-edgeless" className="h-full">
                <ResizablePanelGroup direction="horizontal">
                    <ResizablePanel
                        defaultSize={fullscreenPanel === 'diagram' ? 100 : 50}
                        className={`${fullscreenPanel === 'model' ? 'hidden' : 'block'}`}
                        onDoubleClick={() => handlePanelDoubleClick('diagram')}
                    >
                        <Diagram fullscreen={fullscreenPanel === 'diagram'} onPanelDoubleClick={() => handlePanelDoubleClick('diagram')} />
                    </ResizablePanel>
                    <ResizableHandle className={`border-r ${fullscreenPanel !== null ? 'hidden' : 'block'}`} />
                    <ResizablePanel
                        defaultSize={fullscreenPanel === 'model' ? 100 : 50}
                        className={`${fullscreenPanel === 'diagram' ? 'hidden' : 'block'}`}
                        onDoubleClick={() => handlePanelDoubleClick('model')}
                    >
                        <Model fullscreen={fullscreenPanel === 'model'} onPanelDoubleClick={() => handlePanelDoubleClick('model')} />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </div>
            <Workbench
                visible={visiblePanels.workbench}
                onWidthChange={setWorkbenchWidth}
                width={workbenchWidth} // Pass current width to workbench
            />
            <Details visible={visiblePanels.details} />
            <Console
                visible={visiblePanels.console}
                workbenchVisible={visiblePanels.workbench}
                detailsOrChatVisible={detailsOrChatVisible}
                workbenchWidth={workbenchWidth} // Pass current workbench width
            />
            <Chat visible={visiblePanels.chat} />
        </div>
    );
};

export enum Theme {
    LIGHT = 'light',
    DARK = 'dark',
}

export enum Mode {
    FULL = 'full',
    DIAGRAM = 'diagram',
    MODEL = 'model',
}

interface SketchpadProps {
    mode?: Mode;
    theme?: Theme;
    readonly?: boolean;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.FULL, theme, readonly = false, onWindowEvents }) => {
    const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);

    const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
        if (theme) return theme;
        if (typeof window !== 'undefined') {
            return window.matchMedia('(prefers-color-scheme: dark)').matches ? Theme.DARK : Theme.LIGHT;
        }
        return Theme.LIGHT;
    });

    useEffect(() => {
        const root = window.document.documentElement;
        root.classList.remove("light", "dark");
        root.classList.add(currentTheme);
    }, [currentTheme]);

    const toggleTheme = () => {
        setCurrentTheme(prev => prev === Theme.LIGHT ? Theme.DARK : Theme.LIGHT);
    };

    const ActiveView = DesignEditor;

    return (
        <SketchpadContext.Provider value={{ setNavbarToolbar }}>
            <div className="h-full w-full flex flex-col bg-background text-foreground ">
                <TooltipProvider>
                    <Navbar
                        toolbarContent={navbarToolbar}
                        onWindowEvents={onWindowEvents}
                        readonly={readonly}
                        currentTheme={currentTheme}
                        onToggleTheme={toggleTheme} />
                    <ActiveView />
                </TooltipProvider>
            </div>
        </SketchpadContext.Provider>
    );
};

export default Sketchpad;