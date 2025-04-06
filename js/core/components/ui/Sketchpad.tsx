import { FC, Suspense, ReactNode, useState } from 'react';
import { Provider as JotaiProvider } from 'jotai';
import { Folder, FlaskConical, ChevronDown, ChevronRight } from 'lucide-react';
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
import { Diagram, Viewer, Type } from '@semio/js';

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from './Collapsible';
import { createPortal } from 'react-dom';
import { useAtomValue } from 'jotai';
import { cleanKit, fetchKit, useTypes } from '@semio/js/store';

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
        <div className="h-auto overflow-auto grid grid-cols-[auto-fill] min-w-[40px] auto-rows-[40px] p-1"
            style={{
                gridTemplateColumns: 'repeat(auto-fill, minmax(40px, 1fr))',
                gridAutoRows: '40px',
            }}
        >
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
    icon: <Folder />,
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
    icon: <FlaskConical />,
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
                {open ? <ChevronDown /> : <ChevronRight />}
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

interface SketchpadProps {
}
const Sketchpad: FC<SketchpadProps> = ({ }) => {
    // const types = useAtomValue(typesAtom);
    // const types = getTypes();

    const [draggedTypeId, setDraggedTypeId] = useState<[string, string] | null>(null);
    const [draggedDesignId, setDraggedDesignId] = useState<[string, string] | null>(null);


    const onDragStart = (event: DragStartEvent) => {
        // event.active.id is either type-name-variant or design-name-variant
        const id = event.active.id.split('-');
        if (id[0] === 'type') {
            setDraggedTypeId([id[1], id[2]]);
        }
        else if (id[0] === 'design') {
            setDraggedDesignId([id[1], id[2]]);
        }
        else {
            console.error('Unknown drag type:', id[0]);
        }
    }

    const onDragEnd = (event: DragEndEvent) => {
        const id = event.active.id.split('-');
        if (id[0] === 'type') {
            setDraggedTypeId(null);
        }
        else if (id[0] === 'design') {
            setDraggedDesignId(null);
        }
        else {
            console.error('Unknown drag type:', id[0]);
        }
    }
    console.log('Sketchpad refreshing...');
    return (
        // <JotaiProvider store={studioStore}>
        <div className="h-[800px] w-[1300px]">
            <button onClick={() => {
                console.log('Fetching kit...');
                fetchKit('kit_metabolism.json')
            }
            }>Fetch kit</button>
            <button onClick={() => {
                cleanKit()
            }}>Clean kit</button>
            <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
                <ResizablePanelGroup
                    direction="horizontal"
                    className="bg-dark text-light"
                >
                    <TreeSider />
                    <ResizableHandle />
                    <ResizablePanel defaultSize={70}>
                        <ResizablePanelGroup direction="vertical" className="border-l border-r">
                            <ResizablePanel defaultSize={80}>
                                <ResizablePanelGroup direction="horizontal" className="border-b">
                                    <ResizablePanel defaultSize={50} className="border-r">
                                        <Diagram fullscreen={false} />
                                    </ResizablePanel>
                                    <ResizableHandle />
                                    <ResizablePanel defaultSize={50}>
                                        <Viewer />
                                    </ResizablePanel>
                                </ResizablePanelGroup>
                            </ResizablePanel>
                            <ResizableHandle />
                            <ResizablePanel defaultSize={20}>
                                <div className="flex h-full items-center justify-center p-6">
                                    Console
                                </div>
                            </ResizablePanel>
                        </ResizablePanelGroup>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel defaultSize={15}>
                        <div className="flex h-full items-center justify-center p-6">
                            <span className="font-semibold">Details</span>
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
                {/* {createPortal(
                        <DragOverlay>
                            {draggedTypeId ? (
                                <TypeAvatar type={types.get(draggedTypeId[0])?.get(draggedTypeId[1])!} />
                            ) : null}
                        </DragOverlay>,
                        document.body
                    )} */}
            </DndContext>
        </div>
        // </JotaiProvider>
    );
};

export default Sketchpad;