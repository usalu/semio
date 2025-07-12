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
import { Design, Kit, Piece, Type } from '@semio/js';
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



interface DesignEditorPageProps {
}

const DesignEditorPage: FC<DesignEditorPageProps> = ({ }) => {
    const studioStore = useStudioStore();
    const [designEditorId, setDesignEditorId] = useState<string>('');

    const kit = useKit();
    const designName = "Nakagin Capsule Tower";
    const designVariant = "";
    const designView = "";
    const designId = {
        name: designName,
        variant: designVariant,
        view: designView
    }

    useEffect(() => {
        if (!designEditorId) {
            try {
                const editorId = studioStore.createDesignEditorStore(kit.name, kit.version, designName, designVariant, designView);
                setDesignEditorId(editorId);
            } catch (error) {
                console.error("Error creating design editor store:", error);
            }
        }
    }, [designEditorId, studioStore, designName, designVariant, designView]);

    const designEditorStore = useMemo(() => {
        if (!designEditorId) return null;
        return studioStore.getDesignEditorStore(designEditorId);
    }, [designEditorId, studioStore]);

    if (!designEditorStore) return null;

    const selection = useSyncExternalStore(
        (listener) => designEditorStore?.subscribe(listener) ?? (() => { }),
        () => designEditorStore?.getState().selection ?? { selectedPieceIds: [], selectedConnections: [] }
    );

    const fileUrls = useSyncExternalStore(
        studioStore.subscribe,
        () => studioStore.getFileUrls()
    );

    const onPieceCreate = (piece: Piece) => {
        designEditorStore.transact(() => {
            studioStore.createPiece(kit.name, kit.version, designName, designVariant, designView, piece);
        });
    };

    const onPiecesUpdate = (pieces: Piece[]) => {
        designEditorStore.transact(() => {
            pieces.forEach((piece) => {
                studioStore.updatePiece(kit.name, kit.version, designName, designVariant, designView, piece);
            });
        });
    };

    const onDeleteSelection = () => {
        designEditorStore.transact(() => {
            designEditorStore.deleteSelectedPiecesAndConnections();
        });
    };

    const onUndo = () => designEditorStore.undo();
    const onRedo = () => designEditorStore.redo();

    return (
        <DesignEditorStoreProvider designEditorId={designEditorId}>
            <DesignEditor
                kit={kit}
                designId={designId}
                selection={selection}
                fileUrls={fileUrls}
                onSelectionChange={designEditorStore.updateDesignEditorSelection}
                onPieceCreate={onPieceCreate}
                onPiecesUpdate={onPiecesUpdate}
                onSelectionDelete={onDeleteSelection}
                onUndo={onUndo}
                onRedo={onRedo}
            />
        </DesignEditorStoreProvider>
    );
}

interface SketchpadProps {
    mode?: Mode;
    readonly?: boolean;
    onWindowEvents?: {
        minimize: () => void;
        maximize: () => void;
        close: () => void;
    }
    userId: string;
}

const Sketchpad: FC<SketchpadProps> = ({ mode = Mode.USER, onWindowEvents, userId }) => {
    const [navbarToolbar, setNavbarToolbar] = useState<ReactNode>(null);
    const [currentLayout, setCurrentLayout] = useState<Layout>(Layout.NORMAL);
    const [currentTheme, setCurrentTheme] = useState<Theme>(() => {
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

    const kitName = "Metabolism";
    const kitVersion = "r25.07-1";

    return (
        <TooltipProvider>
            <StudioStoreProvider userId={userId}>
                <KitProvider kitName={kitName} kitVersion={kitVersion}>
                    <DesignEditorPage />
                </KitProvider>
            </StudioStoreProvider>
        </TooltipProvider>
    );
};

export default Sketchpad;