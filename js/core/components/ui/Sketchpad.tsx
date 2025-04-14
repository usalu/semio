import { FC, Suspense, ReactNode, useState, useEffect } from 'react';
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
                Array.from(variantMap.entries()).map(([variant, type]) => (
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
        // {
        //     name: 'Designs',
        //     children: ()
        // }
    ],
}

const TestTree: Tree = {
    id: 'test',
    name: 'Test',
    icon: <FlaskConical size={14} className="w-3.5 h-3.5" />,
    sections: [
        // {
        //     name: 'Types',
        //     items: [
        //         <Avatar />,
        //     ],
        // },
        // {
        //     name: 'Designs',
        //     items: [],
        // }
    ],
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

export enum Theme {
    LIGHT = 'light',
    DARK = 'dark',
}

export enum Mode {
    FULL = 'full',
    DIAGRAM = 'diagram',
    MODEL = 'model',
}

interface NavbarProps {
    children?: ReactNode;
    readonly?: boolean;
    currentTheme: Theme;
    onToggleTheme: () => void;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Navbar: FC<NavbarProps> = ({ children, onWindowEvents, readonly, currentTheme, onToggleTheme }) => {
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
                {children}

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

                {onWindowEvents && (
                    <div className="flex items-center gap-2 ml-4">
                        <button onClick={onWindowEvents.minimize}>-</button>
                        <button onClick={onWindowEvents.maximize}>□</button>
                        <button onClick={onWindowEvents.close}>×</button>
                    </div>
                )}
            </div>
        </div>
    );
};

interface PanelProps {
    visible: boolean;
    children?: ReactNode;
}

const Panel: FC<PanelProps> = ({ visible, children }) => {
    if (!visible) return null;
    return children;
};

interface PanelToggles {
    workbench: boolean;
    console: boolean;
    details: boolean;
    chat: boolean;
}

interface DesignEditorProps {
    mode?: Mode;
}

const DesignEditor: FC<DesignEditorProps> = ({ mode = Mode.FULL }) => {
    const [fullscreenPanel, setFullscreenPanel] = useState<'diagram' | 'model' | null>(null);
    const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
        workbench: false,
        console: false,
        details: false,
        chat: false,
    });

    const togglePanel = (panel: keyof PanelToggles) => {
        setVisiblePanels(prev => {
            const newState = { ...prev };
            if (panel === 'details' && !prev.details) {
                newState.chat = false;
            }
            if (panel === 'chat' && !prev.chat) {
                newState.details = false;
            }
            newState[panel] = !prev[panel];
            return newState;
        });
    };

    useHotkeys('mod+j', (e) => {
        e.preventDefault();
        togglePanel('workbench');
    });

    useHotkeys('mod+k', (e) => {
        e.preventDefault();
        togglePanel('console');
    });

    useHotkeys('mod+l', (e) => {
        e.preventDefault();
        togglePanel('details');
    });

    useHotkeys(['mod+[', 'mod+semicolon', 'mod+ö'], (e) => {
        e.preventDefault();
        togglePanel('chat');
    });

    const handlePanelDoubleClick = (panel: 'diagram' | 'model') => {
        setFullscreenPanel(currentPanel => currentPanel === panel ? null : panel);
    };

    return (
        <div className="h-full">
            <ResizablePanelGroup direction="horizontal">
                <ResizablePanel
                    defaultSize={fullscreenPanel === 'diagram' ? 100 : 50}
                    className={`${fullscreenPanel === 'model' ? 'hidden' : 'block'}`}
                    onDoubleClick={() => handlePanelDoubleClick('diagram')}
                >
                    <Diagram fullscreen={fullscreenPanel === 'diagram'} onPanelDoubleClick={() => handlePanelDoubleClick('diagram')} />
                </ResizablePanel>
                <ResizableHandle
                    className={`border-r ${fullscreenPanel !== null ? 'hidden' : 'block'}`}
                />
                <ResizablePanel
                    defaultSize={fullscreenPanel === 'model' ? 100 : 50}
                    className={`${fullscreenPanel === 'diagram' ? 'hidden' : 'block'}`}
                    onDoubleClick={() => handlePanelDoubleClick('model')}
                >
                    <Model fullscreen={fullscreenPanel === 'model'} onPanelDoubleClick={() => handlePanelDoubleClick('model')} />
                </ResizablePanel>
            </ResizablePanelGroup>

            <Panel visible={visiblePanels.workbench}>
                <div className="absolute top-4 left-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border">
                    <div className="font-semibold p-4">Workbench</div>
                </div>
            </Panel>

            <Panel visible={visiblePanels.details}>
                <div className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border">
                    <div className="font-semibold p-4">Details</div>
                </div>
            </Panel>

            <Panel visible={visiblePanels.console}>
                <div className="absolute left-[254px] right-[254px] bottom-4 h-[200px] z-[150] bg-background-level-2 text-foreground border">
                    <div className="font-semibold p-4">Console</div>
                </div>
            </Panel>

            <Panel visible={visiblePanels.chat}>
                <div className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border">
                    <div className="font-semibold p-4">Chat</div>
                </div>
            </Panel>
        </div>
    );
};

interface SketchpadProps {
    mode?: Mode;
    theme?: Theme;
    readonly?: boolean;
    children?: ReactNode;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.FULL, theme, readonly = false, children, onWindowEvents }) => {
    const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
        if (theme) return theme;
        if (typeof window !== 'undefined') {
            return window.matchMedia('(prefers-color-scheme: dark)').matches
                ? Theme.DARK
                : Theme.LIGHT;
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

    return (
        <div className="h-full w-full flex flex-col bg-background text-foreground">
            <Navbar
                readonly={readonly}
                currentTheme={currentTheme}
                onToggleTheme={toggleTheme}
                onWindowEvents={onWindowEvents}
            >
                {children}
            </Navbar>
            <div className="canvas flex-1 relative">
                <DesignEditor mode={mode} />
            </div>
        </div>
    );
};

export default Sketchpad;