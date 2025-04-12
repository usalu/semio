import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React, { FC, useState } from 'react';
import * as Y from 'yjs';
import { Tree, useTree, StudioProvider } from './teststore';

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
        <div style={{ marginLeft: depth * 20 }}>
            <input type="text" value={value} onChange={handleChange} />
            <button onClick={handleAddChild}>Add Child</button>
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
    const { tree, undo, redo } = useTree(rootId);

    return (
        <div>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <TreeNode nodeId="root" depth={0} />
        </div>
    );
};

const Test: FC = () => {
    return (
        <StudioProvider>
            <div style={{ display: 'flex', gap: '20px' }}>
                <div>
                    <h3>Client 1</h3>
                    <TreeList rootId="shared" />
                </div>
                <div>
                    <h3>Client 2</h3>
                    <TreeList rootId="shared" />
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