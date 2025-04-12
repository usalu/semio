import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React, { FC, useState, useCallback } from 'react';
import * as Y from 'yjs';
import { Tree, useTree, StudioProvider, useStudio } from './teststore';

interface TreeNodeProps {
    nodeId: string;
    depth: number;
}

const TreeNode: React.FC<TreeNodeProps> = ({ nodeId, depth }) => {
    const { getNode, createNode } = useTree("shared");
    const node = getNode(nodeId);

    if (!node) return null;

    const [value, setValue] = useState(node.get('value') || '');

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.value;
        setValue(newValue);
        node.set('value', newValue);
    };

    const handleAddChild = () => {
        const children = node.get('children') as Y.Map<string>;
        const childId = `node-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
        const childNode = createNode(childId);
        children.set(childId, childId);
    };

    const children = node.get('children') as Y.Map<string>;

    return (
        <div className="ml-[20px] mb-2">
            <div className="flex items-center gap-2 mb-1">
                <input
                    type="text"
                    value={value}
                    onChange={handleChange}
                    className="border border-gray-300 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
                <button
                    onClick={handleAddChild}
                    className="bg-green-500 hover:bg-green-600 text-white px-2 py-1 rounded text-sm transition-colors"
                >
                    Add Child
                </button>
            </div>
            {children &&
                Array.from(children.entries()).map(([key, childId]) => (
                    <TreeNode
                        key={key}
                        nodeId={childId}
                        depth={depth + 1}
                    />
                ))}
        </div>
    );
};

interface TreeListProps {
    rootId: string;
}

const TreeList: React.FC<TreeListProps> = ({ rootId }) => {
    const { tree, undo, redo, canUndo, canRedo } = useTree(rootId);

    return (
        <div className="border border-gray-200 rounded-lg p-4 bg-white shadow-sm">
            <div className="flex gap-2 mb-4">
                <button
                    onClick={undo}
                    disabled={!canUndo}
                    className="bg-blue-500 hover:bg-blue-600 text-white px-3 py-1 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    Undo
                </button>
                <button
                    onClick={redo}
                    disabled={!canRedo}
                    className="bg-blue-500 hover:bg-blue-600 text-white px-3 py-1 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    Redo
                </button>
            </div>
            <TreeNode nodeId="root" depth={0} />
        </div>
    );
};

const StudioControls: React.FC = () => {
    const studio = useStudio();
    const [isLoading, setIsLoading] = useState(false);

    const handleRefresh = useCallback(() => {
        // Refresh the studio by forcing a re-render
        window.location.reload();
    }, []);

    const handleClean = useCallback(async () => {
        try {
            setIsLoading(true);
            // Use the new clean API instead of directly modifying localStorage
            await studio.clean();
            // Reload the page to reflect the cleaned state
            window.location.reload();
        } catch (error) {
            console.error("Failed to clean studio:", error);
            setIsLoading(false);
        }
    }, [studio]);

    return (
        <div className="mb-4 flex gap-2">
            <button
                onClick={handleRefresh}
                className="bg-purple-500 hover:bg-purple-600 text-white px-3 py-1 rounded transition-colors"
            >
                Refresh Studio
            </button>
            <button
                onClick={handleClean}
                disabled={isLoading}
                className="bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded transition-colors disabled:bg-red-300"
            >
                {isLoading ? "Cleaning..." : "Clean Studio"}
            </button>
        </div>
    );
};

const Test: FC = () => {
    return (
        <StudioProvider>
            <div className="p-6 bg-gray-50 min-h-screen">
                <h2 className="text-2xl font-bold mb-6 text-gray-800">Studio Test</h2>
                <StudioControls />
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div className="bg-gray-100 p-4 rounded-lg shadow">
                        <h3 className="text-lg font-semibold mb-4 text-gray-700">Client 1</h3>
                        <TreeList rootId="shared" />
                    </div>
                    <div className="bg-gray-100 p-4 rounded-lg shadow">
                        <h3 className="text-lg font-semibold mb-4 text-gray-700">Client 2</h3>
                        <TreeList rootId="shared" />
                    </div>
                </div>
            </div>
        </StudioProvider>
    );
};

export default {
    title: 'Studio/Test',
    component: Test,
};

export const Example = () => <Test />;