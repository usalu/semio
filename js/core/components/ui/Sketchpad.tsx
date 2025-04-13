import { FC, Suspense, ReactNode, useState } from 'react';
import { Provider as JotaiProvider } from 'jotai';
import { Folder, FlaskConical, ChevronDown, ChevronRight, Wrench, Terminal, Info, ChevronDownIcon, Share2, Minus, Square, X, MessageCircle, Home } from 'lucide-react';
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
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { createPortal } from 'react-dom';
import { useAtomValue } from 'jotai';
import { useTypes } from '@semio/js/store';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@semio/js/components/ui/Breadcrumb';
import { Button } from "@semio/js/components/ui/Button";
import { useHotkeys } from 'react-hotkeys-hook';

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
    // const types = useAtomValue(typesAtom);
    // const types = getTypes();

    if (!types) {
        return null;
    }
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
        <Collapsible className="p-3 border-b-thin border-lightGrey font-thin uppercase"
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
    visiblePanels: PanelToggles;
    onTogglePanel: (panel: keyof PanelToggles) => void;
    readonly?: boolean;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Navbar: FC<NavbarProps> = ({ visiblePanels, onTogglePanel, onWindowEvents, readonly }) => {
    return (
        <div
            className={`w-full h-12 bg-dark border-b border-lightGrey flex items-center justify-between px-4`}
        // TODO: Make webkit app region work for electron
        // style={{ WebkitAppRegion: onWindowEvents ? 'drag' : 'none' } as React.CSSProperties}
        >
            <div
                className="flex items-center"
            // style={{ WebkitAppRegion: onWindowEvents ? 'no-drag' : 'none' } as React.CSSProperties}
            >
                <Breadcrumb className="">
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
            <div
                className={`flex items-center gap-4`}
            // style={{ WebkitAppRegion: onWindowEvents ? 'no-drag' : 'none' } as React.CSSProperties}
            >
                <ToggleGroup
                    type="multiple"
                    variant="outline"
                    size="sm"
                    value={Object.entries(visiblePanels)
                        .filter(([_, isVisible]) => isVisible)
                        .map(([key]) => key)}
                    onValueChange={(values) => {
                        // For each panel, toggle it if its presence in values differs from its current state
                        Object.keys(visiblePanels).forEach(key => {
                            const isCurrentlyVisible = visiblePanels[key as keyof PanelToggles];
                            const shouldBeVisible = values.includes(key);

                            if (isCurrentlyVisible !== shouldBeVisible) {
                                onTogglePanel(key as keyof PanelToggles);
                            }
                        });
                    }}
                >
                    <ToggleGroupItem
                        value="workbench"
                        variant="outline"
                        aria-label="Toggle Workbench"
                    >
                        <Wrench />
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="console"
                        variant="outline"
                        aria-label="Toggle Console"
                    >
                        <Terminal />
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="details"
                        variant="outline"
                        aria-label="Toggle Details"
                    >
                        <Info />
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="chat"
                        variant="outline"
                        aria-label="Toggle Chat"
                    >
                        <MessageCircle />
                    </ToggleGroupItem>
                </ToggleGroup>
                <Avatar className="h-8 w-8">
                    <AvatarImage src="https://github.com/usalu.png" />
                    <AvatarFallback>US</AvatarFallback>
                </Avatar>
                <Button variant="outline" size="sm" className="gap-2">
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <div className="flex items-center justify-center">
                                <Share2 size={16} />
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            Share
                        </TooltipContent>
                    </Tooltip>
                </Button>
                {onWindowEvents && (
                    <div className="flex items-center gap-2 ml-4">
                        <Button
                            variant="ghost"
                            size="sm"
                            onClick={onWindowEvents.minimize}
                        >
                            <Minus size={16} />
                        </Button>
                        <Button
                            variant="ghost"
                            size="sm"
                            className="hover:bg-lightGrey transition-colors p-2"
                            onClick={onWindowEvents.maximize}
                        >
                            <Square size={16} />
                        </Button>
                        <Button
                            variant="ghost"
                            size="sm"
                            className="hover:bg-lightGrey hover:text-red-500 transition-colors p-2"
                            onClick={onWindowEvents.close}
                        >
                            <X size={16} />
                        </Button>
                    </div>
                )}
            </div>
        </div >
    );
};


interface DesignEditorProps {
}
const DesignEditor: FC<DesignEditorProps> = ({ }) => {
    const [fullscreenPanel, setFullscreenPanel] = useState<'diagram' | 'model' | null>(null);

    useHotkeys('ctrl+a', (e) => {
        e.preventDefault();
        console.log('Select all pieces and connections');
    });

    useHotkeys('ctrl+i', (e) => {
        e.preventDefault();
        console.log('Invert selection (If only pieces are selected, deselect them and select all other pieces. If only connections are selected, deselect them and select all other connections. If both are selected, deselect pieces and select connections and select the other pieces and connections)');
    });

    useHotkeys('ctrl+d', (e) => {
        e.preventDefault();
        console.log('Select closest piece with same variant');
    });

    useHotkeys('ctrl+shift+d', (e) => {
        e.preventDefault();
        console.log('Select all pieces with same variant');
    });

    useHotkeys('ctrl+c', (e) => {
        e.preventDefault();
        console.log('Copy selected pieces and connections');
    });

    useHotkeys('ctrl+v', (e) => {
        e.preventDefault();
        console.log('Paste pieces and connections');
    });

    useHotkeys('ctrl+x', (e) => {
        e.preventDefault();
        console.log('Cut selected pieces and connections');
    });

    useHotkeys('delete', (e) => {
        e.preventDefault();
        console.log('Delete selected pieces and connections');
    });

    useHotkeys('ctrl+z', (e) => {
        e.preventDefault();
        console.log('Undo last action in design editor');
    });

    useHotkeys('ctrl+y', (e) => {
        e.preventDefault();
        console.log('Redo last action in design editor');
    });

    useHotkeys('ctrl+s', (e) => {
        e.preventDefault();
        console.log('Save stash changes of design');
    });

    useHotkeys('ctrl+w', (e) => {
        e.preventDefault();
        console.log('Close design');
    });

    const handlePanelDoubleClick = (panel: 'diagram' | 'model') => {
        setFullscreenPanel(currentPanel => currentPanel === panel ? null : panel);
    };


    return (
        <div id="sketchpad-edgeless" className="h-full">
            <ResizablePanelGroup
                direction="horizontal"
                className="bg-dark text-light h-full"
            >
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
                    <Model fullscreen={fullscreenPanel === 'model'} />
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>
    );
};

interface PanelProps {
    visible: boolean;
}

const Workbench: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div className="absolute top-4 left-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-foreground border border-lightGrey shadow-lg"
        >
            <div className="font-semibold p-4">Workbench</div>
        </div>
    );
}

const Details: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div
            className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-background-level-2 text-light border border-lightGrey shadow-lg"
        >
            <div className="font-semibold p-4">Details</div>
        </div>
    );
}

const Console: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div
            className="absolute left-[254px] right-[254px] bottom-4 h-[200px] z-[150] bg-dark-grey text-light border border-lightGrey shadow-lg"
        >
            <div className="font-semibold p-4">Console</div>
        </div>
    );
}

const Chat: FC<PanelProps> = ({ visible }) => {
    if (!visible) return null;
    return (
        <div className="absolute top-4 right-4 bottom-4 w-[230px] z-100 bg-dark-grey text-light border border-lightGrey shadow-lg"
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

export enum Theme {
    SYSTEM = 'system',
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

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.FULL, theme = Theme.SYSTEM, readonly = false, onWindowEvents }) => {
    const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
        workbench: false,
        console: false,
        details: false,
        chat: false,
    });

    const togglePanel = (panel: keyof PanelToggles) => {
        setVisiblePanels(prev => {
            const newState = { ...prev };

            // If turning on details, ensure chat is off
            if (panel === 'details' && !prev.details) {
                newState.chat = false;
            }

            // If turning on chat, ensure details is off
            if (panel === 'chat' && !prev.chat) {
                newState.details = false;
            }

            // Toggle the requested panel
            newState[panel] = !prev[panel];

            return newState;
        });
    };

    useHotkeys('mod+j', (e) => {
        e.preventDefault();
        e.stopPropagation();
        togglePanel('workbench');
    });

    useHotkeys('mod+k', (e) => {
        e.preventDefault();
        e.stopPropagation();
        togglePanel('console');
    });

    useHotkeys('mod+l', (e) => {
        e.preventDefault();
        e.stopPropagation();
        togglePanel('details');
    });

    useHotkeys(['mod+[', 'mod+semicolon', 'mod+ö'], (e) => {
        e.preventDefault();
        e.stopPropagation();
        togglePanel('chat');
    });

    return (
        <div className="h-full w-full text-light flex flex-col">
            <TooltipProvider>
                <Navbar visiblePanels={visiblePanels} onTogglePanel={togglePanel} onWindowEvents={onWindowEvents} readonly={readonly} />
                <div className="canvas flex-1 relative">
                    <DesignEditor />
                    <Workbench visible={visiblePanels.workbench} />
                    <Details visible={visiblePanels.details} />
                    <Console visible={visiblePanels.console} />
                    <Chat visible={visiblePanels.chat} />
                </div>
            </TooltipProvider>
        </div>
    );
};

export default Sketchpad;