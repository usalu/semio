import { Component } from 'react';
import {
    GraphView, // required
    Edge, // optional
    type IEdge, // optional
    Node, // optional
    type INode, // optional
    type LayoutEngineType, // required to change the layoutEngineType, otherwise optional
    // FormationTransformer, // optional, Example JSON transformer
    GraphUtils // optional, useful utility functions
} from 'react-digraph';
import styles from './Graph.module.scss'

const GraphConfig = {
    NodeTypes: {
        piece: {
            typeText: "",
            shapeId: "#piece",
            shape: (
                <symbol className='piece' viewBox="0 0 50 50" id="piece" key="0">
                    <circle cx="25" cy="25" r="20"></circle>
                </symbol>
            )
        }
    },
    NodeSubtypes: {},
    EdgeTypes: {
        attraction: {  // required to show empty edges
            shapeId: "#attraction",
            shape: (
                <symbol viewBox="0 0 50 50" id="attraction" key="0">
                    {/* <circle cx="25" cy="25" r="8" fill="currentColor"> </circle> */}
                </symbol>
            )
        }
    }
}

const NODE_KEY = "id"       // Allows D3 to correctly update DOM

const sample = {
    "nodes": [
        {
            "id": 1,
            "title": "a",
            "x": 258.3976135253906,
            "y": 331.9783248901367,
            "type": "piece"
        },
        {
            "id": 2,
            "title": "b",
            "x": 593.9393920898438,
            "y": 260.6060791015625,
            "type": "piece"
        },
        {
            "id": 3,
            "title": "c",
            "x": 237.5757598876953,
            "y": 61.81818389892578,
            "type": "piece"
        },
        {
            "id": 4,
            "title": "d",
            "x": 600.5757598876953,
            "y": 600.81818389892578,
            "type": "piece"
        }
    ],
    "edges": [
        {
            "source": 1,
            "target": 2,
            "type": "attraction"
        },
        {
            "source": 2,
            "target": 4,
            "type": "attraction"
        }
    ]
}

class Graph extends Component {

    constructor(props) {
        super(props);

        this.state = {
            graph: sample,
            selected: {}
        }
    }

    /* Define custom graph editing methods here */

    render() {
        const nodes = this.state.graph.nodes;
        const edges = this.state.graph.edges;
        const selected = this.state.selected;

        const NodeTypes = GraphConfig.NodeTypes;
        const NodeSubtypes = GraphConfig.NodeSubtypes;
        const EdgeTypes = GraphConfig.EdgeTypes;

        return (
            <div id='graph' className={styles.graph}>
            {/* <div id='graph'> */}

                <GraphView ref='GraphView'
                    nodeKey={NODE_KEY}
                    nodes={nodes}
                    edges={edges}
                    selected={selected}
                    nodeTypes={NodeTypes}
                    nodeSubtypes={NodeSubtypes}
                    edgeTypes={EdgeTypes}
                    allowMultiselect={true}
                    edgeArrowSize={4}
                    onSelect={this.onSelect}
                    onCreateNode={this.onCreateNode}
                    onUpdateNode={this.onUpdateNode}
                    onDeleteNode={this.onDeleteNode}
                    onCreateEdge={this.onCreateEdge}
                    onSwapEdge={this.onSwapEdge}
                    onDeleteEdge={this.onDeleteEdge} />
            </div>
        );
    }
}

export default Graph