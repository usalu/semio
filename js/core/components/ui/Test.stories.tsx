import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React, { FC, useState } from 'react';
import * as Y from 'yjs';
import { Tree, useTree } from './teststore';

interface TreeNodeProps {
    node: Y.Map<Tree>;
    depth: number;
    onAddChild: (parent: Y.Map<Tree>) => void;
}

const TreeNode: React.FC<TreeNodeProps> = ({ node, depth, onAddChild }) => {
    const [value, setValue] = useState(node.get('value') || '');

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.value;
        setValue(newValue);
        node.set('value', newValue);
    };

    const children = node.get('children') as Y.Array<Y.Map<Tree>>;

    return (
        <div style={{ marginLeft: depth * 20 }}>
            <input type="text" value={value} onChange={handleChange} />
            <button onClick={() => onAddChild(node)}>Add Child</button>
            {children &&
                children.map((childNode, index) => (
                    <TreeNode
                        key={index}
                        node={childNode}
                        depth={depth + 1}
                        onAddChild={onAddChild}
                    />
                ))}
        </div>
    );
};

interface TreeListProps {
    root: Tree;
}

const TreeList: React.FC<TreeListProps> = ({ root }) => {
    const { tree, undo, redo } = useTree(root);

    const handleAddChild = (parent: Y.Map<Tree>) => {
        const children = parent.get('children') as Y.Array<Y.Map<Tree>>;
        const newChild = new Y.Map();
        newChild.set('value', '');
        newChild.set('children', new Y.Array());
        children.push([newChild]);
    };

    return (
        <div>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <TreeNode node={tree} depth={0} onAddChild={handleAddChild} />
        </div>
    );
};

const Test: FC = () => {
    const { tree: root1 } = useTree('client1');
    const { tree: root2 } = useTree('client2');

    return (
        <div style={{ display: 'flex', gap: '20px' }}>
            <div>
                <h3>Client 1</h3>
                <TreeList root={root1} />
            </div>
            <div>
                <h3>Client 2</h3>
                <TreeList root={root2} />
            </div>
        </div>
    );
};

export default {
    title: 'Studio/Test',
    component: Test,
};

export const Example = () => <Test />;