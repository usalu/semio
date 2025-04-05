import React, { FC, Suspense } from 'react';
import { Folder, FlaskConical } from 'lucide-react';
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@semio/js/components/ui/Resizable"
import { Diagram } from '@semio/js';


type TreeItem = {
    children?: TreeItem[];
}

type TreeSection = {
    name: string;
    items: TreeItem[];
}

type Tree = {
    id: string;
    name: string;
    icon: React.ReactNode;
    sections: TreeSection[];
}

const ExplorerTree: Tree = {
    id: 'explorer',
    name: 'Explorer',
    icon: <Folder />,
    sections: [
        {
            name: 'Types',
            items: [],
        },
        {
            name: 'Designs',
            items: [],
        }
    ],
}

const TestTree: Tree = {
    id: 'test',
    name: 'Test',
    icon: <FlaskConical />,
    sections: [
        {
            name: 'Types',
            items: [],
        },
        {
            name: 'Designs',
            items: [],
        }
    ],
}

const trees = [
    ExplorerTree,
    TestTree,
]

interface TreeProps {
    treeId: string;
}
const Tree: FC<TreeProps> = ({ treeId }) => {
    const tree = trees.find(tree => tree.id === treeId);
    return (
        <ResizablePanel defaultSize={300}>
            {tree.sections.map((section, index) => (
                <div key={index} className="">
                    <div className="">
                        <span className="">{section.name}</span>
                    </div>
                    <div className="">
                        {section.items.map((item, index) => (
                            <div key={index} className="">
                                {/* Render item here */}
                            </div>
                        ))}
                    </div>
                </div>
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
        <div className="flex h-full w-12 flex-col items-center justify-top border-r">
            {trees.map((tree) => (
                <div
                    key={tree.id}
                    className={`w-10 h-10 flex items-center justify-center cursor-pointer ${tree.id === activeTreeId ? 'bg-gray-300' : ''
                        }`}
                    onClick={() => onTreeSelect?.(tree.id)} // Add onClick event
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

    const [activeTreeId, setActiveTreeId] = React.useState('explorer');

    return (
        <>
            <TreeBar activeTreeId={activeTreeId} onTreeSelect={setActiveTreeId} />
            <Tree treeId={activeTreeId} />
        </>
    );
};

interface SketchpadProps {
}
const Sketchpad: FC<SketchpadProps> = ({ }) => {
    return (
        <div className="h-[800px] w-[1300px]">
            <ResizablePanelGroup
                direction="horizontal"
                className="bg-dark text-light"
            >
                <TreeSider />
                <ResizableHandle />
                <ResizablePanel defaultSize={800}>
                    <ResizablePanelGroup direction="vertical" className="border-l border-r">
                        <ResizablePanel defaultSize={650}>
                            <ResizablePanelGroup direction="horizontal" className="border-b">
                                <ResizablePanel defaultSize={400} className="border-r">
                                    <Diagram fullscreen={false} />
                                </ResizablePanel>
                                <ResizableHandle />
                                <ResizablePanel defaultSize={400}>
                                    <div className="flex h-full items-center justify-center p-6">
                                        <span className="font-semibold">Three</span>
                                    </div>
                                </ResizablePanel>
                            </ResizablePanelGroup>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel defaultSize={150}>
                            <div className="flex h-full items-center justify-center p-6">
                                Console
                            </div>
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </ResizablePanel>
                <ResizableHandle />
                <ResizablePanel defaultSize={200}>
                    <div className="flex h-full items-center justify-center p-6">
                        <span className="font-semibold">Details</span>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>
    );
};

export default Sketchpad;