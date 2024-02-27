/* eslint-disable @typescript-eslint/explicit-function-return-type */
import { useState } from 'react';
import {
    GraphView, // required
    Edge, // optional
    type IEdge, // optional
    Node, // optional
    type INode, // optional
    type LayoutEngineType, // required to change the layoutEngineType, otherwise optional
    // FormationTransformer, // optional, Example JSON transformer
    GraphUtils, // optional, useful utility functions
    type SelectionT
} from 'react-digraph'
import PieceText from './PieceText';

const GraphConfig = {
    NodeTypes: {
        piece: {
            typeText: '',
            shapeId: '#piece',
            shape: (
                <symbol className="piece" viewBox="0 0 50 50" height="40" width="40" id="piece" key="0">
                    <circle cx="25" cy="25" r="24"></circle>
                </symbol>
            )
        }
    },
    NodeSubtypes: {},
    EdgeTypes: {
        attraction: {
            shapeId: '#attraction',
            shape: (
                <symbol viewBox="0 0 50 50" id="attraction" key="0">
                    {/* <circle cx="25" cy="25" r="8" fill="currentColor"> </circle> */}
                </symbol>
            )
        }
    }
}

const NODE_KEY = 'id' // Allows D3 to correctly update DOM

const sample = {
    nodes: [
        {
            id: 1,
            title: 'keystone',
            x: 0,
            y: 0,
            type: 'piece'
        },
        {
            id: 2,
            title: '🧹',
            x: 150,
            y: 50,
            type: 'piece'
        },
        {
            id: 3,
            title: 'c',
            x: 237.5757598876953,
            y: 61.81818389892578,
            type: 'piece'
        },
        {
            id: 4,
            title: 'd',
            x: 400,
            y: 400,
            type: 'piece'
        }
    ],
    edges: [
        {
            source: 1,
            target: 2,
            type: 'attraction',
            label_from: 's',
            label_to: 'n',
            handleTooltipText: 'South to North',
        },
        {
            source: 2,
            target: 4,
            type: 'attraction'
        }
    ]
}

const FormationEditor = () => {
    const [graph, setGraph] = useState(sample);
    const [selected, setSelected] = useState<SelectionT | null>(null);


    const onSelect = (selected) => {
        setSelected(selected);
    }

    const onCreateNode = (x, y) => {
        const id = graph.nodes.length + 1;

        const newNode = {
            id,
            title: '',
            type: 'piece',
            x,
            y
        }

        setGraph({
            ...graph,
            nodes: [...graph.nodes, newNode]
        });
    }

    const onUpdateNode = (viewNode) => {
        const newNodes = graph.nodes.map((node) => {
            if (node.id === viewNode.id) {
                return viewNode;
            } else {
                return node;
            }
        });

        setGraph({
            ...graph,
            nodes: newNodes
        });
    }

    const onCreateEdge = (sourceViewNode, targetViewNode) => {
        const newEdge = {
            source: sourceViewNode.id,
            target: targetViewNode.id,
            type: 'attraction'
        }

        setGraph({
            ...graph,
            edges: [...graph.edges, newEdge]
        });
    }

    const onSwapEdge = (sourceViewNode, targetViewNode, edge) => {
        const edges = graph.edges.map((e) => {
            if (e.source === edge.source && e.target === edge.target) {
                edge.source = sourceViewNode.id;
                edge.target = targetViewNode.id;
            }
            return edge;
        });

        setGraph({
            ...graph,
            edges
        });
    }

    const renderNodeText = (data: any, id: string | number, isSelected: boolean) => {
        return (
            <PieceText
                id={data.title}
                isSelected={isSelected}
            />
        );
    }

    const nodes = graph.nodes;
    const edges = graph.edges;

    const NodeTypes = GraphConfig.NodeTypes;
    const NodeSubtypes = GraphConfig.NodeSubtypes;
    const EdgeTypes = GraphConfig.EdgeTypes;

    return (
        <div id="formation-editor" style={
            {
                height: "100vh",
                width: "100vw"
            }
        }>
            <GraphView
                // ref="GraphView"
                nodeKey={NODE_KEY}
                nodes={nodes}
                edges={edges}
                selected={selected}
                nodeTypes={NodeTypes}
                nodeSubtypes={NodeSubtypes}
                edgeTypes={EdgeTypes}
                // layoutEngineType='VerticalTree'
                allowMultiselect={true}
                gridSpacing={20}
                gridDotSize={0}
                nodeSize={100}
                edgeArrowSize={4}
                edgeHandleSize={200}
                // rotateEdgeHandle={false}
                minZoom={0.1}
                maxZoom={4}
                onSelect={onSelect}
                onCreateNode={onCreateNode}
                onUpdateNode={onUpdateNode}
                onCreateEdge={onCreateEdge}
                onSwapEdge={onSwapEdge}
                renderNodeText={renderNodeText}
            ></GraphView>
        </div>
    );
}

export default FormationEditor
