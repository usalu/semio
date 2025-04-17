import { FC, Suspense, ReactNode, useState, useEffect, createContext, useContext } from 'react';
import { Folder, FlaskConical, ChevronDown, ChevronRight, Wrench, Terminal, Info, ChevronDownIcon, Share2, Minus, Square, X, MessageCircle, Home, Sun, Moon, Monitor, Sofa, Glasses, AppWindow } from 'lucide-react';
import {
    DndContext,
    DragEndEvent,
    DragOverlay,
    DragStartEvent,
    UniqueIdentifier,
    useDraggable,
    useDroppable
} from '@dnd-kit/core';
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable"
import { Avatar, AvatarFallback, AvatarImage } from "@semio/js/components/ui/Avatar";
import { Design, Kit, Type } from '@semio/js';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@semio/js/components/ui/Tooltip"
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup"
import { ToggleCycle } from "@semio/js/components/ui/ToggleCycle"
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { createPortal } from 'react-dom';
import { useKit, useDesign, DesignProvider, KitProvider, StudioProvider, useStudio } from '@semio/js/store';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@semio/js/components/ui/Breadcrumb';
import { Button } from "@semio/js/components/ui/Button";
import { useHotkeys } from 'react-hotkeys-hook';
import { Toggle } from '@semio/js/components/ui/Toggle';
import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';
import { default as nakaginCapsuleTower } from '@semio/assets/semio/design_nakagin-capsule-tower_flat.json';
import { Fingerprint } from 'lucide-react';
import { Generator } from '@semio/js/lib/utils';
import { Piece } from '@semio/js';
import DesignEditor from './DesignEditor';

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
                                key={`layout-${currentLayout}`}
                                className="h-full w-full flex flex-col bg-background text-foreground"
                            >
                                <Navbar
                                    toolbarContent={navbarToolbar}
                                    onWindowEvents={onWindowEvents}
                                />
                                <DesignEditor />
                            </div>
                        </DesignProvider>
                    </KitProvider>
                </SketchpadContext.Provider>
            </StudioProvider>
        </TooltipProvider>
    );
};

export default Sketchpad;