import { FC, Suspense, ReactNode, useState, useEffect, createContext, useContext, useMemo, useReducer, useSyncExternalStore } from 'react';
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
import { Design, Kit, Mode, Layout, Piece, Type, Theme } from '@semio/js';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@semio/js/components/ui/Tooltip"
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup"
import { ToggleCycle } from "@semio/js/components/ui/ToggleCycle"
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible';
import { createPortal } from 'react-dom';
import { useStudioStore, StudioStoreProvider, DesignEditorStoreProvider, useDesignEditorStore, KitProvider, useKit } from '@semio/js/store';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@semio/js/components/ui/Breadcrumb';
import { Button } from "@semio/js/components/ui/Button";
import { useHotkeys } from 'react-hotkeys-hook';
import { Toggle } from '@semio/js/components/ui/Toggle';
import { Fingerprint } from 'lucide-react';
import { Generator } from '@semio/js/lib/utils';
import DesignEditor from './DesignEditor';


interface NavbarProps {
    mode?: Mode;
    toolbarContent?: ReactNode;
    layout?: Layout
    theme?: Theme;
    setLayout?: (layout: Layout) => void;
    setTheme?: (theme: Theme) => void;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
}

const Navbar: FC<NavbarProps> = ({ mode, toolbarContent, layout, theme, setLayout, setTheme, onWindowEvents }) => {

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
                            tooltip: "Turn theme dark",
                            label: <Moon />
                        },
                        {
                            value: Theme.DARK,
                            tooltip: "Turn theme light",
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
                            tooltip: "Turn touch layout on",
                            label: <Fingerprint />
                        },
                        {
                            value: Layout.TOUCH,
                            tooltip: "Return to normal layout",
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
export default Navbar;