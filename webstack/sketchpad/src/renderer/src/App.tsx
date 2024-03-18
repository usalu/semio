import './App.scss'
import {
    useState,
    forwardRef,
    Ref,
    useMemo,
    Fragment,
    Suspense,
    useEffect,
    SVGProps,
    useImperativeHandle,
    useRef
} from 'react'
import { createPortal } from 'react-dom'
import { KBarProvider } from 'kbar'
import { KBarAnimator, KBarPortal, KBarPositioner, KBarSearch } from 'kbar'
import { KBarResults, useMatches } from 'kbar'
import { ActionId, ActionImpl } from 'kbar'
import {
    Avatar,
    Breadcrumb,
    Button,
    Col,
    Collapse,
    ConfigProvider,
    Divider,
    Flex,
    GetProp,
    Layout,
    Menu,
    Modal,
    Row,
    Select,
    Space,
    Steps,
    Table,
    TableProps,
    Tabs,
    Tag,
    Transfer,
    TransferProps,
    message,
    theme,
    MenuItem
} from 'antd'
import enUS from 'antd/lib/calendar/locale/en_US'
import { Canvas, useLoader } from '@react-three/fiber'
import {
    OrbitControls,
    useGLTF,
    Select as ThreeSelect,
    GizmoHelper,
    GizmoViewport
} from '@react-three/drei'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import { INode, IEdge, IGraphInput, SelectionT, GraphUtils, IPoint, GraphView } from 'react-digraph'
import SVG from 'react-inlinesvg'
import {
    AttractionInput,
    Formation,
    Kit,
    Piece,
    PieceInput,
    Quality,
    Representation,
    Type
} from '@renderer/semio'
import tailwindConfig from '../../../tailwind.config.js'
import CloseSharpIcon from '@mui/icons-material/CloseSharp'
import MinimizeSharpIcon from '@mui/icons-material/MinimizeSharp'
import FullscreenSharpIcon from '@mui/icons-material/FullscreenSharp'
import FullscreenExitSharpIcon from '@mui/icons-material/FullscreenExitSharp'
import HomeSharpIcon from '@mui/icons-material/HomeSharp'
import { DndContext, DragEndEvent, DragOverlay, useDraggable, useDroppable } from '@dnd-kit/core'
import adjectives from './assets/adjectives'
import animals from './assets/animals'
import sampleDiagram from './assets/samplediagram'
// import sampleKit from './assets/samplekit.json'

const { Header, Content, Footer, Sider } = Layout

const {
    theme: { colors }
} = tailwindConfig

class SeededRandom {
    private seed: number

    constructor(seed: number) {
        this.seed = seed % 2147483647
        if (this.seed <= 0) this.seed += 2147483646
    }

    // Returns a pseudo-random number between 1 and 2^31 - 2
    next(): number {
        return (this.seed = (this.seed * 16807) % 2147483647)
    }

    // Returns a pseudo-random number between 0 (inclusive) and 1 (exclusive)
    nextFloat(): number {
        return (this.next() - 1) / 2147483646
    }

    // Returns a pseudo-random number between 0 (inclusive) and max (exclusive)
    nextInt(max: number): number {
        return Math.floor(this.nextFloat() * max)
    }
}

class Generator {
    public static generateRandomId(seed: number): string {
        const random = new SeededRandom(seed)

        let adjective = adjectives[random.nextInt(adjectives.length)]
        let animal = animals[random.nextInt(animals.length)]
        const number = random.nextInt(1000)

        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)

        return `${adjective}${animal}${number}`
    }
}

function tinyKeyStringToHuman(string: string): string {
    return string
        .split('+')
        .map((key) => {
            if (key === '$mod') return 'Ctrl'
            if (key === 'Shift') return '⇧'
            return key
        })
        .join(' + ')
}

const ResultItem = forwardRef(
    (
        {
            action,
            active,
            currentRootActionId
        }: {
            action: ActionImpl
            active: boolean
            currentRootActionId: ActionId
        },
        ref: Ref<HTMLDivElement>
    ): JSX.Element => {
        const ancestors = useMemo(() => {
            if (!currentRootActionId) return action.ancestors
            const index = action.ancestors.findIndex(
                (ancestor) => ancestor.id === currentRootActionId
            )
            // +1 removes the currentRootAction; e.g.
            // if we are on the "Set theme" parent action,
            // the UI should not display "Set theme… > Dark"
            // but rather just "Dark"
            return action.ancestors.slice(index + 1)
        }, [action.ancestors, currentRootActionId])

        return (
            <div
                ref={ref}
                className={`flex justify-between px-4 rounded-md  ${active ? 'bg-primary text-dark' : 'bg-dark bg-opacity-50 text-light'}`}
            >
                <div className="description">
                    {action.icon && action.icon}
                    <div>
                        <div>
                            {ancestors.length > 0 &&
                                ancestors.map((ancestor) => (
                                    <Fragment key={ancestor.id}>
                                        <span>{ancestor.name}</span>
                                        <span>&rsaquo;</span>
                                    </Fragment>
                                ))}
                            <span>{action.name}</span>
                        </div>
                        {action.subtitle && <span>{action.subtitle}</span>}
                    </div>
                </div>
                {action.shortcut?.length ? (
                    <div className="shortcut">
                        {action.shortcut.map((sc) => (
                            <kbd key={sc}>{tinyKeyStringToHuman(sc)}</kbd>
                        ))}
                    </div>
                ) : null}
            </div>
        )
    }
)

ResultItem.displayName = 'ResultItem'

interface RenderResultsProps {
    className?: string
}

function RenderResults({ className }: RenderResultsProps): JSX.Element {
    const { results, rootActionId } = useMatches()

    return (
        <KBarResults
            items={results}
            onRender={({ item, active }) =>
                typeof item === 'string' ? (
                    <div className={active ? `${className} active` : className}>{item}</div>
                ) : (
                    <ResultItem action={item} active={active} currentRootActionId={rootActionId} />
                )
            }
        />
    )
}

function CommandBar(): JSX.Element {
    return (
        <KBarPortal>
            <KBarPositioner className="backdrop-blur-sm">
                <KBarAnimator className="w-2/3">
                    <KBarSearch className="w-full bg-light border-none p-4 rounded-2xl placeholder:text-dark focus:bg-primary focus:outline-none focus:placeholder:text-light selection:bg-secondary" />
                    <RenderResults className=" bg-light bg-opacity-50 rounded-md px-2 py-1 box-content" />
                </KBarAnimator>
            </KBarPositioner>
        </KBarPortal>
    )
}

enum IconKind {
    Text,
    Svg,
    Image
}

function getIconData(dataUrl): [string, IconKind] {
    const svgStart = 'data:image/svg+xml;base64,'
    const pngStart = 'data:image/png;base64,'
    const jpegStart = 'data:image/jpeg;base64,'
    let kind
    let data
    if (dataUrl.startsWith(svgStart)) {
        kind = IconKind.Svg
        data = atob(dataUrl.substring(svgStart.length))
    } else if (dataUrl.startsWith(pngStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(pngStart.length));
        data = dataUrl
    } else if (dataUrl.startsWith(jpegStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(jpegStart.length));
        data = dataUrl
    } else {
        kind = IconKind.Text
        data = dataUrl
    }
    return [data, kind]
}

const GraphConfig = {
    NodeTypes: {
        piece: {
            typeText: '',
            shapeId: '#piece',
            shape: (
                <symbol
                    className="piece"
                    viewBox="0 0 50 50"
                    height="40"
                    width="40"
                    id="piece"
                    key="0"
                >
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

interface IPieceNode extends INode {
    piece: PieceInput
}

interface IAttractionEdge extends IEdge {
    attraction: AttractionInput
}

export interface IDraft extends IGraphInput {
    name?: string
    explanation?: string
    icon?: string
    nodes: IPieceNode[]
    edges: IAttractionEdge[]
}

const NODE_KEY = 'id' // Allows D3 to correctly update DOM

interface DiagramEditorProps {
    className?: string
    piece: PieceInput
    onPieceEdit: (piece: PieceInput) => Promise<PieceInput>
    onAttractionEdit: (attraction: AttractionInput) => AttractionInput
}

const DiagramEditor = forwardRef((props: DiagramEditorProps, ref) => {
    const [graph, setGraph] = useState(sampleDiagram)
    const [selected, setSelected] = useState<SelectionT | null>(null)
    const [copiedNodes, setCopiedNodes] = useState<INode[]>([])
    const [copiedEdges, setCopiedEdges] = useState<IEdge[]>([])
    const graphViewRef = useRef(null)

    const { isOver, setNodeRef } = useDroppable({
        id: 'diagramEditor'
    })

    const getDraft = (): IDraft => {
        return {
            nodes: graph.nodes,
            edges: graph.edges
        }
    }

    const setDraft = (draft: IDraft) => {
        setGraph(draft)
    }

    const zoomToFit = () => {
        if (graphViewRef.current) {
            graphViewRef.current.handleZoomToFit()
        }
    }

    useImperativeHandle(ref, () => ({
        onDropPiece,
        getDraft,
        setDraft,
        zoomToFit
    }))

    const onSelect = (newSelection: SelectionT, event?: any): void => {
        // when only an edge is selected, then the attraction editor should be opened
        if (!newSelection.nodes && newSelection.edges?.size === 1) {
            const edge = newSelection.edges.values().next().value
            const sourceNode = graph.nodes.find((node) => node.id === edge.source)
            const targetNode = graph.nodes.find((node) => node.id === edge.target)
            if (sourceNode && targetNode) {
                const attraction = edge.attraction
                const editedAttraction = props.onAttractionEdit(attraction)
                if (editedAttraction) {
                    const newEdge = {
                        ...edge,
                        attraction: editedAttraction
                    }
                    setGraph({
                        ...graph,
                        edges: graph.edges.map((e) => (e === edge ? newEdge : e))
                    })
                }
            }
            return
        }

        // alt key for node edit mode
        if (event && event.altKey === true) {
            const selectedNode = graph.nodes.find(
                (node) => node.id === newSelection?.nodes?.keys().next().value
            )
            if (selectedNode.type === 'piece') {
                props.onPieceEdit(selectedNode.piece).then((updatedPiece) => {
                    if (updatedPiece) {
                        setGraph({
                            ...graph,
                            nodes: graph.nodes.map((node) => {
                                if (node.id === selectedNode.id) {
                                    return {
                                        ...node,
                                        piece: updatedPiece
                                    }
                                }
                                return node
                            })
                        })
                    }
                })
            }
            return
        }

        // event is only active when clicked on a node
        if (event == null && !newSelection.nodes && !newSelection.edges) {
            setSelected(null)
            return
        }
        // Remove the previously selected nodes and edges from the selectionState if they are in the new selection.
        // Add the new selected nodes and edges if they were not in the previous selection.
        const newNodes = new Map(selected?.nodes)
        if (newSelection.nodes) {
            newSelection.nodes.forEach((node, nodeId) => {
                if (GraphUtils.isEqual(node, selected?.nodes?.get(nodeId))) {
                    newNodes.delete(nodeId)
                } else {
                    newNodes.set(nodeId, node)
                }
            })
        }
        const newEdges = new Map(selected?.edges)
        if (newSelection.edges) {
            newSelection.edges.forEach((edge, edgeId) => {
                if (
                    selected?.edges &&
                    [...selected.edges.values()].some((selectedEdge) =>
                        GraphUtils.isEqual(selectedEdge, edge)
                    )
                ) {
                    newEdges.delete(edgeId)
                } else {
                    newEdges.set(edgeId, edge)
                }
            })
        }
        // check for orphaned edges
        newEdges?.forEach((edge, edgeId) => {
            selected?.nodes?.forEach((node) => {
                if (node.id === edge.source || node.id === edge.target) {
                    newEdges.delete(edgeId)
                }
            })
        })
        setSelected({ nodes: newNodes, edges: newEdges })
    }

    const onCreateNode = (x: number, y: number, event: any): void => {
        const id = Generator.generateRandomId(x + y)

        const newNode = {
            id,
            title: '',
            type: 'piece',
            icon: event.typeIcon,
            x,
            y,
            piece: { ...event.piece, id: id } as PieceInput
        } as IPieceNode

        setGraph({
            ...graph,
            nodes: [...graph.nodes, newNode]
        })
    }

    const onUpdateNode = (
        node: INode,
        updatedNodes?: Map<string, INode> | null,
        updatedNodePosition?: IPoint
    ): void | Promise<any> => {}

    const onCreateEdge = (sourceNode: INode, targetNode: INode): void => {
        const newEdge = {
            source: sourceNode.id,
            target: targetNode.id,
            type: 'attraction'
        }

        setGraph({
            ...graph,
            edges: [...graph.edges, newEdge]
        })
    }

    const onDeleteSelected = (selected: SelectionT) => {
        const newNodes = graph.nodes.filter((node) => !selected.nodes?.has(node.id))
        const newEdges = graph.edges.filter((edge) => {
            return (
                !selected.nodes?.has(edge.source.toString()) &&
                !selected.nodes?.has(edge.target.toString())
            )
        })

        setGraph({
            nodes: newNodes,
            edges: newEdges
        })
    }

    const onCopySelected = () => {
        if (selected && selected.nodes) {
            const nodesToCopy = graph.nodes.filter((node) => selected.nodes?.has(node.id))
            const topLeftNode = nodesToCopy.reduce((prev, curr) => ({
                x: Math.min(prev.x, curr.x),
                y: Math.min(prev.y, curr.y)
            }))
            const nodesWithRelativePositions = nodesToCopy.map((node) => ({
                ...node,
                x: node.x - topLeftNode.x,
                y: node.y - topLeftNode.y
            }))
            setCopiedNodes(nodesWithRelativePositions)

            const edgesToCopy = graph.edges.filter(
                (edge) => selected.nodes?.has(edge.source) && selected.nodes?.has(edge.target)
            )
            setCopiedEdges(edgesToCopy)
        }
    }

    const onPasteSelected = (selected?: SelectionT | null, xyCoords?: IPoint): void => {
        if (copiedNodes.length > 0 && xyCoords) {
            const idMap = new Map()
            const newNodes = copiedNodes.map((node) => {
                const newId = Generator.generateRandomId(xyCoords.x + node.x + xyCoords.y + node.y)
                idMap.set(node.id, newId)
                return {
                    ...node,
                    id: newId,
                    x: xyCoords.x + node.x,
                    y: xyCoords.y + node.y
                }
            })
            const newEdges = copiedEdges.map((edge) => ({
                ...edge,
                source: idMap.get(edge.source),
                target: idMap.get(edge.target)
            }))
            setGraph({
                ...graph,
                nodes: [...graph.nodes, ...newNodes],
                edges: [...graph.edges, ...newEdges]
            })
        }
    }

    const onSwapEdge = (sourceNode: INode, targetNode: INode, edge: IEdge): void => {
        const newEdge = {
            source: sourceNode.id,
            target: targetNode.id,
            type: edge.type
        }

        setGraph({
            ...graph,
            edges: graph.edges.map((e) => (e === edge ? newEdge : e))
        })
    }

    const canSwapEdge = (
        sourceNode: INode,
        hoveredNode: INode | null,
        swapEdge: IEdge
    ): boolean => {
        return true
    }

    const onContextMenu = (x: number, y: number, event: any): void => {}

    const renderSvg = (
        svgString: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        // replace all black colors with white and all white colors with black
        const darkSvgString = svgString
            .replace(/#000000/g, colors.dark)
            .replace(/#000/g, colors.dark)
            .replace(/black/g, colors.dark)
            .replace(/#FFFFFF/g, colors.light)
            .replace(/#FFF/g, colors.light)
            .replace(/white/g, colors.light)
        const lightSvgString = svgString
            .replace(/#000000/g, colors.light)
            .replace(/#000/g, colors.light)
            .replace(/black/g, colors.light)
            .replace(/#FFFFFF/g, colors.dark)
            .replace(/#FFF/g, colors.dark)
            .replace(/white/g, colors.dark)
        return (
            <foreignObject x="-16" y="-16" width="32" height="32">
                <SVG
                    className={`cursor-pointer ${isSelected ? 'text-light' : 'text-dark'}`}
                    src={isSelected ? lightSvgString : darkSvgString}
                    width="32"
                    height="32"
                />
            </foreignObject>
        )
    }

    const renderImage = (
        imageData: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        return (
            <foreignObject x="-19" y="-19" width="38" height="38">
                <Avatar
                    className={`cursor-pointer ${isSelected ? 'opacity-50' : 'opacity-100'}`}
                    src={imageData}
                    size={38}
                ></Avatar>
            </foreignObject>
        )
    }

    const renderText = (
        text: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        const className = `node-text ${isSelected ? 'selected' : ''}`
        return (
            <text className={className} textAnchor="middle" dy=".3em">
                {text}
            </text>
        )
    }

    const renderNodeText = (
        data: any,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        const [iconData, iconKind] = getIconData(data.icon)

        switch (iconKind) {
            case IconKind.Svg:
                return renderSvg(iconData, id, isSelected)
            case IconKind.Image:
                return renderImage(iconData, id, isSelected)
            case IconKind.Text:
                return renderText(iconData, id, isSelected)
        }
    }

    // Adaptation of: https://github.com/uber/react-digraph/issues/179
    const onDropPiece = (x: number, y: number, type: Type) => {
        if (graphViewRef.current) {
            const viewTransfrom = graphViewRef.current.state.viewTransform
            const svgX = -1 * ((viewTransfrom.x - x) / viewTransfrom.k)
            const svgY = -1 * ((viewTransfrom.y - y) / viewTransfrom.k)
            onCreateNode(svgX, svgY, {
                piece: {
                    type: {
                        name: type.name,
                        variant: type.variant
                    }
                },
                typeIcon: type.icon
            })
        }
    }

    const nodes = graph.nodes
    const edges = graph.edges

    const NodeTypes = GraphConfig.NodeTypes
    const NodeSubtypes = GraphConfig.NodeSubtypes
    const EdgeTypes = GraphConfig.EdgeTypes

    return (
        <div
            id="formation-editor"
            className={'font-sans h-full ' + props.className + (isOver ? 'bg-dark' : 'bg-darkGrey')}
            ref={setNodeRef}
        >
            <GraphView
                ref={graphViewRef}
                nodeKey={NODE_KEY}
                nodes={nodes}
                edges={edges}
                selected={selected}
                nodeTypes={NodeTypes}
                nodeSubtypes={NodeSubtypes}
                edgeTypes={EdgeTypes}
                // layoutEngineType='VerticalTree'
                allowMultiselect={true}
                // gridSpacing={20}
                gridDotSize={0}
                nodeSize={100}
                edgeHandleSize={200}
                edgeArrowSize={4}
                // rotateEdgeHandle={false}
                minZoom={0.1}
                maxZoom={4}
                showGraphControls={false}
                canSwapEdge={canSwapEdge}
                onSwapEdge={onSwapEdge}
                onArrowClicked={(selectedEdge: IEdge): void => {}}
                onSelect={onSelect}
                onCreateNode={onCreateNode}
                onUpdateNode={onUpdateNode}
                onCreateEdge={onCreateEdge}
                onDeleteSelected={onDeleteSelected}
                onCopySelected={onCopySelected}
                onPasteSelected={onPasteSelected}
                onContextMenu={onContextMenu}
                renderNodeText={renderNodeText}
            ></GraphView>
        </div>
    )
})

DiagramEditor.displayName = 'DiagramEditor'

interface RepresentationThreeProps {
    representation: Representation
}

const RepresentationThree = ({ representation }: RepresentationThreeProps) => {
    const { nodes } = useLoader(GLTFLoader, representation.url)
    return (
        <group>
            {Object.values(nodes).map((node, i) => (
                <primitive
                    key={i}
                    object={node}
                    attach={(parent, self) => {
                        parent.add(self)
                        return () => parent.remove(self)
                    }}
                />
            ))}
        </group>
    )
}

RepresentationThree.displayName = 'Representation'

interface PieceThreeProps {
    piece: Piece
}

const PieceThree = ({ piece }: PieceThreeProps) => {
    const [lod, setLod] = useState('')
    const [tags, setTags] = useState([''])
    return (
        <RepresentationThree
            representation={
                piece.type.representations.find(
                    (representation) =>
                        (representation.lod !== '' || representation.lod === lod) &&
                        (representation.tags.length === 0 ||
                            representation.tags.some((tag) => tags.includes(tag)))
                )!
            }
        />
    )
}

interface FormationThreeProps {
    formation: Formation
}

const FormationThree = ({ formation }: FormationThreeProps) => {
    const { pieces } = formation
    return (
        <group>
            {pieces.map((piece, i) => (
                <PieceThree key={i} piece={piece} />
            ))}
        </group>
    )
}

interface ShapeEditorProps {}

const ShapeEditor = ({}: ShapeEditorProps) => {
    const [kit, setKit] = useState<Kit | null>(null)
    const [blobUrls, setBlobUrls] = useState<{ [key: string]: string }>({})
    const [isSelectionBoxActive, setIsSelectionBoxActive] = useState(false)

    useEffect(() => {
        ;[
            'c:\\git\\semio\\2.x\\examples\\metabolism\\representations\\capsule_1_1to200_volume_wireframe.glb'
        ].forEach((path) => {
            window.electron.ipcRenderer.invoke('get-file-buffer', path).then((buffer) => {
                const name = 'representations/capsule_1_1to200_volume_wireframe.glb'
                const blob = new Blob([buffer], { type: 'model/gltf-binary' })
                const url = URL.createObjectURL(blob)
                useGLTF.preload(url)
                setBlobUrls((prev) => ({ ...prev, [name]: url }))
            })
        })
    }, [kit])

    return (
        <Canvas
            shadows={true}
            // orthographic={true}
        >
            <ThreeSelect
                multiple
                box
                border="1px solid #fff"
                onChange={(selected): void => {
                    if (!isSelectionBoxActive) {
                        setIsSelectionBoxActive(true)
                        console.log('selection starting', selected)
                    }
                }}
                onChangePointerUp={(e) => {
                    if (isSelectionBoxActive) {
                        setIsSelectionBoxActive(false)
                        console.log('selection ending', e)
                    }
                }}
                onClick={(e) => {
                    console.log('select onClick', e)
                }}
            >
                <Suspense fallback={null}>
                    <RepresentationThree
                        representation={{
                            url: blobUrls['representations/capsule_1_1to200_volume_wireframe.glb']
                        }}
                    />
                    <hemisphereLight color={colors.primary} intensity={0.5} />
                    <ambientLight color={colors.primary} intensity={0.5} />
                </Suspense>
            </ThreeSelect>
            <OrbitControls enabled={!isSelectionBoxActive} />
            <GizmoHelper
                alignment="bottom-right" // widget alignment within scene
                margin={[80, 80]} // widget margins (X, Y)
            >
                <GizmoViewport
                    labels={['X', 'Z', '-Y']}
                    axisColors={[colors.primary, colors.tertiary, colors.secondary]}
                    // labelColor={colors.light}
                    // font="Anta"
                />
            </GizmoHelper>
        </Canvas>
    )
}

function getItem(
    label: React.ReactNode,
    key: React.Key,
    icon?: React.ReactNode,
    children?: MenuItem[]
): MenuItem {
    return {
        key,
        icon,
        children,
        label
    } as MenuItem
}

const SemioIcon = (props) => (
    <svg width={48} height={48} overflow="visible" viewBox="0 0 99.95 99.921" {...props}>
        {'-->'}
        <g
            style={{
                stroke: '#000',
                strokeWidth: 1,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        >
            <g
                style={{
                    fill: '#fa9500',
                    fillOpacity: 1,
                    stroke: '#000',
                    strokeWidth: 1,
                    strokeDasharray: 'none',
                    strokeOpacity: 1
                }}
            >
                <path
                    fillOpacity={0}
                    stroke="none"
                    d="M94.789 41.727v77.939l19.984-19.985V41.727Z"
                    style={{
                        fill: '#fa9500',
                        fillOpacity: 1,
                        stroke: 'none',
                        strokeWidth: 0.489687,
                        strokeDasharray: 'none',
                        strokeOpacity: 1
                    }}
                    transform="translate(-94.789 -19.745)"
                />
            </g>
            <g
                fillOpacity={0}
                stroke="none"
                style={{
                    fill: '#ff344f',
                    fillOpacity: 1,
                    stroke: '#000',
                    strokeWidth: 1,
                    strokeDasharray: 'none',
                    strokeOpacity: 1
                }}
            >
                <path
                    d="m194.71 119.666.03-98.535-19.985 19.979-.03 78.556zM94.789 19.745h98.51l-19.984 19.984H94.79Z"
                    style={{
                        fill: '#ff344f',
                        fillOpacity: 1,
                        stroke: 'none',
                        strokeWidth: 0.489687,
                        strokeDasharray: 'none',
                        strokeOpacity: 1
                    }}
                    transform="translate(-94.789 -19.745)"
                />
            </g>
            <g
                fillOpacity={0}
                stroke="none"
                style={{
                    fill: '#00a69d',
                    fillOpacity: 1,
                    stroke: '#000',
                    strokeWidth: 1,
                    strokeDasharray: 'none',
                    strokeOpacity: 1
                }}
            >
                <path
                    d="m134.757 119.666 19.984-19.985h17.987v19.985zM134.757 79.697l19.984-19.984h17.987v19.984z"
                    style={{
                        fill: '#00a69d',
                        fillOpacity: 1,
                        stroke: 'none',
                        strokeWidth: 0.489687,
                        strokeDasharray: 'none',
                        strokeOpacity: 1
                    }}
                    transform="translate(-94.789 -19.745)"
                />
            </g>
        </g>
    </svg>
)

const DesignIcon = (props) => (
    <svg width={48} height={48} {...props}>
        <defs>
            <marker
                id="a"
                markerHeight={0.6}
                markerWidth={0.6}
                orient="auto-start-reverse"
                preserveAspectRatio="xMidYMid"
                refX={0}
                refY={0}
                style={{
                    overflow: 'visible'
                }}
                viewBox="0 0 1 1"
            >
                <path
                    d="m5.77 0-8.65 5V-5Z"
                    style={{
                        fill: colors.light,
                        fillRule: 'evenodd',
                        stroke: colors.light,
                        strokeWidth: '1pt'
                    }}
                    transform="scale(.5)"
                />
            </marker>
        </defs>
        <circle
            cx={15.031}
            cy={10.763}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
        <circle
            cx={15.031}
            cy={35.829}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
        <circle
            cx={34.916}
            cy={24}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
        <path
            d="M15.03 30.822V17.878"
            style={{
                fill: 'none',
                fillRule: 'evenodd',
                stroke: colors.light,
                strokeWidth: '.927333px',
                strokeLinecap: 'butt',
                strokeLinejoin: 'miter',
                strokeMiterlimit: 4,
                strokeOpacity: 1,
                markerEnd: 'url(#a)'
            }}
        />
    </svg>
)

const FormationIcon = (props) => (
    <svg width={48} height={48} {...props}>
        <defs>
            <marker
                id="a"
                markerHeight={0.6}
                markerWidth={0.6}
                orient="auto-start-reverse"
                preserveAspectRatio="xMidYMid"
                refX={0}
                refY={0}
                style={{
                    overflow: 'visible'
                }}
                viewBox="0 0 1 1"
            >
                <path
                    d="m5.77 0-8.65 5V-5Z"
                    style={{
                        fill: colors.light,
                        fillRule: 'evenodd',
                        stroke: colors.light,
                        strokeWidth: '1pt'
                    }}
                    transform="scale(.5)"
                />
            </marker>
        </defs>
        <circle
            cx={24}
            cy={11.739}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
        <circle
            cx={24}
            cy={36.806}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
        <path
            d="M24 31.799V18.855"
            style={{
                fill: 'none',
                fillRule: 'evenodd',
                stroke: colors.light,
                strokeWidth: '.927333px',
                strokeLinecap: 'butt',
                strokeLinejoin: 'miter',
                strokeMiterlimit: 4,
                strokeOpacity: 1,
                markerEnd: 'url(#a)'
            }}
        />
    </svg>
)

const TypeIcon = (props) => (
    <svg width={48} height={48} {...props}>
        <circle
            cx={24}
            cy={24}
            r={5.007}
            style={{
                fill: 'none',
                stroke: colors.light,
                strokeWidth: 0.733,
                strokeDasharray: 'none',
                strokeOpacity: 1
            }}
        />
    </svg>
)

const DraggableAvatar = ({ user, id }) => {
    const { attributes, listeners, setNodeRef, transform } = useDraggable({
        id: id
    })
    return (
        <Avatar
            ref={setNodeRef}
            {...listeners}
            {...attributes}
            size="large"
            className="font-sans text-darkGrey"
        >
            {user}
        </Avatar>
    )
}

interface AppProps {
    onWindowMinimize: () => void
    onWindowMaximize: () => void
    onWindowClose: () => void
    onOpenKit: () => Promise<Kit>
    onReloadKit: () => Promise<Kit>
    onOpenDraft: () => Promise<string>
    onSaveDraft: (draft: IDraft) => Promise<string>
}

const App = ({
    onWindowMinimize,
    onWindowMaximize,
    onWindowClose,
    onOpenKit,
    onReloadKit,
    onOpenDraft,
    onSaveDraft
}: AppProps): JSX.Element => {
    const [fullScreen, setFullScreen] = useState(false)
    const [collapsedToolset, setCollapsedToolset] = useState(false)
    const [isDropped, setIsDropped] = useState(false)
    const [activeId, setActiveId] = useState(null)

    const handleTabClick = () => {
        setCollapsedToolset(!collapsedToolset)
    }

    const diagramEditorRef = useRef(null)

    // const actions = [
    //     {
    //         id: 'open-kit',
    //         name: 'Open Kit',
    //         shortcut: ['$mod+o'],
    //         keywords: 'new',
    //         section: 'Files',
    //         perform: () => {
    //             onOpenKit('').then((kit) => {
    //                 // TODO: Set kit over redux
    //                 // setKit(kit)
    //             })
    //         }
    //     },
    //     {
    //         id: 'reload-kit',
    //         name: 'Reload Kit',
    //         shortcut: ['$mod+r'],
    //         keywords: 'update',
    //         section: 'Files',
    //         perform: () => {
    //             onReloadKit().then((kit) => {
    //                 // TODO: Set kit over redux
    //                 // setKit(kit)
    //             })
    //         }
    //     },
    //     {
    //         id: 'open-draft',
    //         name: 'Open Draft',
    //         shortcut: ['$mod+Shift+o'],
    //         keywords: 'load session',
    //         section: 'Files',
    //         perform: () => {
    //             onOpenDraft('').then((draftJson) => {
    //                 diagramEditorRef.current.setDraft(JSON.parse(draftJson))
    //             })
    //         }
    //     },
    //     {
    //         id: 'save-draft',
    //         name: 'Save draft',
    //         shortcut: ['$mod+s'],
    //         keywords: 'store session',
    //         section: 'Files',
    //         perform: () => {
    //             onSaveDraft(diagramEditorRef.current.getDraft()).then((url) => {
    //                 console.log('Draft saved under: ', url)
    //             })
    //         }
    //     },
    //     {
    //         id: 'zoom-to-fit',
    //         name: 'Zoom to Fit',
    //         shortcut: ['$mod+t'],
    //         keywords: 'formation',
    //         section: 'Navigation',
    //         perform: () => {
    //             if (diagramEditorRef.current) {
    //                 diagramEditorRef.current.zoomToFit()
    //             }
    //         }
    //     }
    // ]

    // useEffect(() => {
    //     if (kit) {
    //         ;[
    //             'c:\\git\\semio\\2.x\\examples\\metabolism\\representations\\capsule_1_1to200_volume_wireframe.glb'
    //         ].forEach((path) => {
    //             window.electron.ipcRenderer.invoke('get-file-buffer', path).then((buffer) => {
    //                 const name = 'representations/capsule_1_1to200_volume_wireframe.glb'
    //                 const blob = new Blob([buffer], { type: 'model/gltf-binary' })
    //                 const url = URL.createObjectURL(blob)
    //                 useGLTF.preload(url)
    //                 setBlobUrls((prev) => ({ ...prev, [name]: url }))
    //             })
    //         })
    //     }
    // }, [kit])

    const onDragEnd = (event: DragEndEvent) => {
        console.log('drag end', event)
        if (event.over && event.over.id === 'diagramEditor') {
            // relative coordinates in the diagram editor
            const relativeX = event.activatorEvent.pageX + event.delta.x - event.over.rect.left
            const relativeY = event.activatorEvent.pageY + event.delta.y - event.over.rect.top
            diagramEditorRef.current.onDropPiece(relativeX, relativeY, {
                name: 'capsule',
                variant: '1',
                icon: '📦1'
            } as Type)
            setIsDropped(true)
        }
    }

    return (
        <div className="h-screen w-screen">
            <ConfigProvider
                locale={enUS}
                theme={{
                    // algorithm: [theme.darkAlgorithm],
                    token: {
                        // primary
                        colorPrimary: colors.light,
                        colorPrimaryBg: colors.light,
                        colorPrimaryBgHover: colors.light,
                        colorPrimaryBorder: colors.light,
                        colorPrimaryBorderHover: colors.light,
                        colorPrimaryHover: colors.light,
                        colorPrimaryActive: colors.light,
                        colorPrimaryText: colors.light,
                        colorPrimaryTextHover: colors.light,
                        colorPrimaryTextActive: colors.light,
                        // text
                        colorText: colors.light, // e.g. title of collapse, leaf of breadcrumb
                        colorTextSecondary: colors.lightGrey,
                        colorTextTertiary: colors.lightGrey, // e.g. x on close button of tab
                        colorTextQuaternary: colors.lightGrey, // e.g. placeholder text
                        // border
                        colorBorder: colors.light,
                        colorBorderSecondary: colors.light,
                        // fill
                        colorFill: colors.light,
                        colorFillSecondary: colors.light,
                        colorFillTertiary: colors.light,
                        colorFillQuaternary: colors.darkGrey, // e.g. background of collapse title
                        // background
                        colorBgContainer: colors.darkGrey, // e.g. active tab, collapse content box
                        colorBgElevated: colors.grey, // e.g. background selected menu
                        colorBgLayout: colors.light,
                        colorBgSpotlight: colors.light,
                        colorBgMask: colors.light,
                        colorBgTextActive: colors.light,
                        colorBgBase: colors.light,
                        // special colors
                        colorError: colors.danger,
                        colorWarning: colors.warning,
                        colorInfo: colors.info,
                        colorSuccess: colors.success,
                        fontFamily: 'Anta, sans-serif',
                        boxShadow: 'none',
                        boxShadowSecondary: 'none',
                        boxShadowTertiary: 'none',
                        wireframe: false,
                        borderRadius: 0,
                        lineWidth: 0
                        // motionUnit: 0.05
                    },
                    components: {
                        Button: {
                            borderColorDisabled: colors.light,
                            dangerColor: colors.light,
                            defaultActiveBg: colors.light,
                            defaultActiveBorderColor: colors.light,
                            defaultActiveColor: colors.light,
                            defaultBg: colors.light,
                            defaultBorderColor: colors.light,
                            defaultColor: colors.lightGrey, // e.g. normal state of buttons
                            defaultGhostBorderColor: colors.light,
                            defaultGhostColor: colors.light,
                            defaultHoverBg: colors.darkGrey, // e.g. hover over window control buttons
                            ghostBg: colors.light,
                            linkHoverBg: colors.light,
                            primaryColor: colors.light,
                            textHoverBg: colors.light
                        },
                        Layout: {
                            bodyBg: colors.dark,
                            footerBg: colors.grey,
                            headerBg: colors.grey, // e.g. space between tabs and content
                            headerColor: colors.light,
                            lightSiderBg: colors.light,
                            lightTriggerBg: colors.light,
                            lightTriggerColor: colors.light,
                            siderBg: colors.darkGrey,
                            triggerBg: colors.light,
                            triggerColor: colors.light,
                            headerPadding: '0px 0px'
                        },
                        Tabs: {
                            cardBg: colors.grey, // background of unselected tabs
                            inkBarColor: colors.light,
                            itemActiveColor: colors.light,
                            itemColor: colors.lightGrey, // text and fill of unselected tabs
                            itemHoverColor: colors.light,
                            itemSelectedColor: colors.light,
                            cardGutter: 0,
                            cardHeight: 38,
                            cardPadding: '0 16px',
                            verticalItemMargin: '0'
                        },
                        Divider: {
                            lineWidth: 0.25,
                            verticalMarginInline: 0
                        },
                        Avatar: {
                            groupBorderColor: colors.light
                        },
                        Collapse: {
                            headerBg: colors.darkGrey,
                            headerPadding: '0 0px',
                            contentBg: colors.darkGrey,
                            contentPadding: '0 0px'
                        },
                        Select: {
                            clearBg: colors.lightGrey,
                            multipleItemBg: colors.darkGrey,
                            optionActiveBg: colors.darkGrey,
                            optionSelectedBg: colors.darkGrey,
                            optionSelectedColor: colors.light,
                            selectorBg: colors.darkGrey
                        }
                    }
                }}
            >
                <Layout style={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
                    <Header style={{ height: 'auto' }}>
                        <div
                            style={{
                                height: '38px',
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'flex-start',
                                WebkitAppRegion: 'drag'
                            }}
                        >
                            <Tabs
                                className="p-0 flex items-center"
                                type="editable-card"
                                style={{
                                    WebkitAppRegion: 'no-drag'
                                }}
                                defaultActiveKey="1"
                                items={[
                                    {
                                        key: '1',
                                        label: <HomeSharpIcon />,
                                        closable: false
                                    },
                                    {
                                        key: '2',
                                        label: 'Nakagin Capsule Tower'
                                    },
                                    {
                                        key: '3',
                                        label: 'Unsaved'
                                    }
                                ]}
                            />
                            <Space />
                            <div
                                style={{
                                    display: 'flex',
                                    height: '100%',
                                    justifyContent: 'flex-end',
                                    alignItems: 'center',
                                    WebkitAppRegion: 'no-drag'
                                }}
                            >
                                <Button
                                    onClick={onWindowMinimize}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    <MinimizeSharpIcon />
                                </Button>
                                <Button
                                    onClick={onWindowMaximize}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    {fullScreen ? (
                                        <FullscreenExitSharpIcon />
                                    ) : (
                                        <FullscreenSharpIcon />
                                    )}
                                </Button>
                                <Button
                                    onClick={onWindowClose}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    <CloseSharpIcon />
                                </Button>
                            </div>
                        </div>
                    </Header>
                    <Row className="items-center justify-between flex h-[47px] w-full bg-darkGrey border-b-thin border-lightGrey">
                        <Col className="flex items-center">
                            {/* TODO: Add icons for main menu and tools */}
                        </Col>
                        <Col className="flex items-center">
                            <Breadcrumb>
                                <Breadcrumb.Item>Metabolism</Breadcrumb.Item>
                                <Breadcrumb.Item>Formations</Breadcrumb.Item>
                                <Breadcrumb.Item>Unsaved</Breadcrumb.Item>
                            </Breadcrumb>
                        </Col>
                        <Col className="flex items-center">
                            {/* TODO: Add icons for sharing, etc */}
                        </Col>
                    </Row>
                    <Layout style={{ flex: 1 }}>
                        <Layout>
                            <DndContext onDragEnd={onDragEnd}>
                                <Sider width="240px" className="border-r-thin border-lightGrey">
                                    <Collapse
                                        className="p-3 border-b-thin border-lightGrey font-thin"
                                        items={[
                                            {
                                                key: '1',
                                                label: 'TYPES',
                                                children: (
                                                    <Collapse
                                                        className="p-2 font-normal text-lightGrey"
                                                        items={[
                                                            {
                                                                key: '1',
                                                                label: 'capsule',
                                                                children: (
                                                                    <Space
                                                                        className="h-auto overflow-auto grid grid-cols-[auto-fill] min-w-[40px] auto-rows-[40px] p-1"
                                                                        direction="vertical"
                                                                        size={10}
                                                                        style={{
                                                                            gridTemplateColumns:
                                                                                'repeat(auto-fill, minmax(40px, 1fr))',
                                                                            gridAutoRows: '40px'
                                                                        }}
                                                                    >
                                                                        {[
                                                                            '📦1',
                                                                            '📦2',
                                                                            '📦3',
                                                                            '📦4',
                                                                            '📦5',
                                                                            '📦6',
                                                                            '📦7',
                                                                            '📦8'
                                                                        ].map((user, index) => (
                                                                            <DraggableAvatar
                                                                                key={index}
                                                                                id={index}
                                                                                user={user}
                                                                            ></DraggableAvatar>
                                                                        ))}
                                                                    </Space>
                                                                )
                                                            }
                                                        ]}
                                                        defaultActiveKey={['1']}
                                                    />
                                                )
                                            },
                                            {
                                                key: '2',
                                                label: 'FORMATIONS',
                                                children: <p>{'Test'}</p>
                                            }
                                        ]}
                                        defaultActiveKey={['1']}
                                    />
                                </Sider>
                                <Content>
                                    <DiagramEditor ref={diagramEditorRef} />
                                </Content>
                                <Divider className="h-full top-0" type="vertical" />
                                <Content>
                                    <ShapeEditor />
                                </Content>
                                {createPortal(
                                    <DragOverlay>
                                        {/* {activeId ? (
                                            <DraggableAvatar id={activeId} user="🏫" />
                                        ) : null} */}
                                        <DraggableAvatar id={activeId} user="🏫" />
                                    </DragOverlay>,
                                    document.body
                                )}
                            </DndContext>
                        </Layout>
                        <Sider className="border-l-thin border-lightGrey" width="240">
                            <Collapse
                                className="p-3"
                                items={[
                                    {
                                        key: '1',
                                        label: 'SCENE',
                                        children: (
                                            <Flex vertical={true} className="p-2 text-lightGrey">
                                                <div className="p-0">Level of Details</div>
                                                <Select
                                                    className="p-1"
                                                    mode="multiple"
                                                    allowClear
                                                    placeholder="Please select"
                                                    defaultValue={['1to500']}
                                                    options={[
                                                        {
                                                            label: '1to500',
                                                            value: '1to500'
                                                        },
                                                        {
                                                            label: '1to200',
                                                            value: '1to200'
                                                        }
                                                    ]}
                                                />
                                            </Flex>
                                        )
                                    }
                                ]}
                                defaultActiveKey={['1']}
                            />
                        </Sider>
                    </Layout>
                    {/* <Footer className='p-0'>
                        <div style={{ height: '25px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <div className="flex items-center">
                            </div>
                        </div>
                    </Footer> */}
                </Layout>
            </ConfigProvider>
        </div>
    )
}
export default App
