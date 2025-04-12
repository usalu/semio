import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React, { useState } from 'react';
import * as Y from 'yjs';

interface TreeNodeProps {
    node: Y.Map<any>;
    depth: number;
    onAddChild: (parent: Y.Map<any>) => void;
}

const TreeNode: React.FC<TreeNodeProps> = ({ node, depth, onAddChild }) => {
    const [value, setValue] = useState(node.get('value') || '');

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.value;
        setValue(newValue);
        node.set('value', newValue);
    };

    const children = node.get('children') as Y.Array<Y.Map<any>>;

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
    root: Y.Map<any>;
}

const TreeList: React.FC<TreeListProps> = ({ root }) => {
    const handleAddChild = (parent: Y.Map<any>) => {
        const children = parent.get('children') as Y.Array<Y.Map<any>>;
        const newChild = new Y.Map();
        newChild.set('value', '');
        newChild.set('children', new Y.Array());
        children.push([newChild]);
    };

    return <TreeNode node={root} depth={0} onAddChild={handleAddChild} />;
};

const Test: FC = () => {
    return (
        <div style={{ display: 'flex', gap: '20px' }}>
            <div>
                <h3>Client 1</h3>
                <TreeList root={root} />
            </div>
            <div>
                <h3>Client 2</h3>
                <TreeList root={root} />
            </div>
        </div>
    );
};

export default {
    title: 'Studio/Test',
    component: Test,
};

export const Example = () => <Test />;