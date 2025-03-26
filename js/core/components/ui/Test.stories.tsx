import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React, { FC, useState, useCallback, useEffect } from 'react';
import { TreeNode as TreeNodeType, useTree, StudioProvider, useStudio } from './teststore';

interface TreeNodeProps {
    nodeId: string;
    depth: number;
}

const TreeNode: React.FC<TreeNodeProps> = ({ nodeId, depth }) => {
    const { getNode, updateNodeValue, addChild } = useTree("shared");
    const node = getNode(nodeId);

    const [value, setValue] = useState('');

    useEffect(() => {
        if (!node) return;
        setValue(node.value);
    }, [node]);

    if (!node) return null;

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.value;
        setValue(newValue);
        updateNodeValue(nodeId, newValue);
    };

    const handleAddChild = () => {
        addChild(nodeId);
    };

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
            {node.childIds.map(childId => (
                <TreeNode
                    key={childId}
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
    const { undo, redo, canUndo, canRedo, hasInitialized, initializeTree } = useTree(rootId);
    const [initialValue, setInitialValue] = useState('Root');
    const studio = useStudio();
    const fullTreeId = `tree-${rootId}`;

    useEffect(() => {
        console.log(`Undo/Redo state for ${rootId}: canUndo=${canUndo}, canRedo=${canRedo}`);

        // If initialized but undo/redo buttons aren't working, try to trigger undo capability
        if (hasInitialized && !canUndo) {
            // This will create temporary operations to activate the undo functionality
            studio.triggerUndoRedoCapability(fullTreeId);
        }
    }, [canUndo, canRedo, rootId, hasInitialized, studio, fullTreeId]);

    const handleInitializeTree = () => {
        const rootNode = initializeTree(initialValue);

        // After initialization, explicitly trigger the undo capability
        setTimeout(() => {
            studio.triggerUndoRedoCapability(fullTreeId);
        }, 100);

        return rootNode;
    };

    if (!hasInitialized) {
        return (
            <div className="border border-gray-200 rounded-lg p-4 bg-white shadow-sm text-center">
                <div className="flex flex-col sm:flex-row items-center justify-center gap-2 max-w-md mx-auto">
                    <input
                        type="text"
                        value={initialValue}
                        onChange={(e) => setInitialValue(e.target.value)}
                        placeholder="Root node name"
                        className="border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full sm:w-auto"
                    />
                    <button
                        onClick={handleInitializeTree}
                        className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded transition-colors w-full sm:w-auto"
                    >
                        Initialize Tree
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="border border-gray-200 rounded-lg p-4 bg-white shadow-sm">
            <div className="flex gap-2 mb-4">
                <button
                    onClick={undo}
                    disabled={!canUndo}
                    className={`px-3 py-1 rounded transition-colors ${canUndo
                        ? "bg-blue-500 hover:bg-blue-600 text-white"
                        : "bg-gray-300 text-gray-500 cursor-not-allowed"
                        }`}
                >
                    Undo
                </button>
                <button
                    onClick={redo}
                    disabled={!canRedo}
                    className={`px-3 py-1 rounded transition-colors ${canRedo
                        ? "bg-blue-500 hover:bg-blue-600 text-white"
                        : "bg-gray-300 text-gray-500 cursor-not-allowed"
                        }`}
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
            // Use the clean API
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