import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const sketchpad = join(import.meta.dirname, "../../../../../../semio/client/lib/sketchpad/react/index.tsx");
let text = readFileSync(sketchpad, "utf8");

text = text.replace(
  "\n * buildKitDiagramData holds the data fields for a buildKitDiagramData record.\n **/",
  "\n/**\n * buildKitDiagramData holds the data fields for a buildKitDiagramData record.\n **/",
);

text = text.replace(
  "const buildKitDiagramData = (kit: Kit): { nodes: Node<KitDiagramNode>[]; edges: Edge[] } => {\n  const nodes: Node<KitDiagramNode>[] = [];\n  const edges: Edge[] = [];",
  "const buildKitDiagramData = (kit: Kit): { nodes: KitDiagramLayoutNode[]; edges: KitDiagramEdge[] } => {\n  const nodes: KitDiagramLayoutNode[] = [];\n  const edges: KitDiagramEdge[] = [];",
);

text = text.replaceAll(
  `      nodes.push({
        id: nodeId,
        type: "artifact",
        position: { x: 0, y: 0 },
        width: frame.width,
        height: frame.height,
        data: {`,
  `      nodes.push({
        id: nodeId,
        position: { x: 0, y: 0 },
        width: frame.width,
        height: frame.height,
        selected: false,
        data: {`,
);

const edgePartOf = `        edges.push({
          id: \`\${kind}-\${item.parentId}-\${item.id}\`,
          source: \`\${parentKind}:\${item.parentId}\`,
          target: nodeId,
          type: "floating",
          style: edgeStyle["part-of"],
          data: { relationship: "part-of" },
        });`;
const edgePartOfNew = `        edges.push({
          id: \`\${kind}-\${item.parentId}-\${item.id}\`,
          source: \`\${parentKind}:\${item.parentId}\`,
          target: nodeId,
          relationship: "part-of",
        });`;
text = text.replace(edgePartOf, edgePartOfNew);

const edgeRef = `          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });`;
const edgeRefNew = `          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            relationship: "reference",
          });`;
text = text.replaceAll(edgeRef, edgeRefNew);

text = text.replace(
  "const sketchpadKitBoardNodeCenter = (node: Node<KitDiagramNode>): { x: number; y: number } => {",
  "const sketchpadKitBoardNodeCenter = (node: KitDiagramLayoutNode): { x: number; y: number } => {",
);
text = text.replace(
  "const sketchpadKitBoardCameraFromNodes = (nodes: readonly Node<KitDiagramNode>[]): ElementsBoardCameraState => {",
  "const sketchpadKitBoardCameraFromNodes = (nodes: readonly KitDiagramLayoutNode[]): ElementsBoardCameraState => {",
);
text = text.replace(
  "const sketchpadKitBuildBoardFixture = (nodes: readonly Node<KitDiagramNode>[], edges: readonly Edge[]): BoardFixtureV1 => {",
  "const sketchpadKitBuildBoardFixture = (nodes: readonly KitDiagramLayoutNode[], edges: readonly KitDiagramEdge[]): BoardFixtureV1 => {",
);
text = text.replace(
  "/** @emoji 🗺️ Maps filtered kit diagram React Flow nodes/edges into an `elements.board.fixture/v1` payload for {@link TopologyBoardPane}. */",
  "/** @emoji 🗺️ Maps kit diagram layout nodes/edges into an `elements.board.fixture/v1` payload for {@link TopologyBoardPane}. */",
);

const innerStart = text.indexOf("interface ForceNode extends SimulationNodeDatum {");
const innerEnd = text.indexOf("  const kitBoardBindings = useMemo(");
if (innerStart < 0 || innerEnd < 0) {
  console.error("[DEBUG] KitDiagramInner markers missing", { innerStart, innerEnd });
  process.exit(1);
}

const newInnerHead = `const KitDiagramInner: FC = () => {
  const ks0 = useKitStoreSnapshot();
  const kit = ks0?.kit as Kit | undefined;
  const boardWrapper = useRef<HTMLDivElement>(null);
  const kitCommands = useKitAppCommands();
  const [selection] = useKitAppSelection();
  const [setHover] = useKitAppSetHover();
  const [clearHover] = useKitAppClearHover();
  const kitScope = useActiveKitTab();
  const kitId = kitScope?.id ?? "";
  const [activeTool] = useKitAppActiveTool();
  const isHandTool = activeTool === ToolKind.HAND;
  const actor = useSketchpadActor();
  const [diagramForce] = useKitAppDiagramForce();
  const [diagramNodes, setDiagramNodes] = useState<KitDiagramLayoutNode[]>([]);
  const [diagramEdges, setDiagramEdges] = useState<KitDiagramEdge[]>([]);
  const diagramNodesRef = useRef<KitDiagramLayoutNode[]>([]);
  const diagramEdgesRef = useRef<KitDiagramEdge[]>([]);
  const draggingNodeIdRef = useRef<string | null>(null);
  const diagramForceConfig = useMemo(() => ({ ...defaultDiagramForceSettings, ...diagramForce }), [diagramForce]);
`;

const newInnerTail = `
  const commitDiagramNodes = useCallback((nextNodes: KitDiagramLayoutNode[]) => {
    diagramNodesRef.current = nextNodes;
    setDiagramNodes(nextNodes);
  }, []);

  useEffect(() => {
    const previousPositions = new Map(diagramNodesRef.current.map((node) => [node.id, node.position]));
    const nextNodes = baseNodes.map((node) => {
      const previousPosition = previousPositions.get(node.id);
      return previousPosition ? { ...node, position: previousPosition } : node;
    });
    commitDiagramNodes(nextNodes);
    diagramEdgesRef.current = baseEdges;
    setDiagramEdges(baseEdges);
  }, [baseNodes, baseEdges, commitDiagramNodes]);

  useEffect(() => {
    if (diagramNodesRef.current.length === 0) return;
    const fixture = sketchpadKitBuildBoardFixture(diagramNodesRef.current, diagramEdgesRef.current);
    const laid = layoutBoardFixtureForceGraph(fixture, sketchpadKitDiagramForceGraphOptions(diagramForceConfig));
    commitDiagramNodes(sketchpadKitApplyFixturePositionsToLayoutNodes(diagramNodesRef.current, laid));
  }, [baseEdgeIdsKey, baseNodeIdsKey, commitDiagramNodes, diagramForceConfig]);

  const onBoardDrag = useCallback(
    (payload: { id: string; x: number; y: number }) => {
      if (isHandTool) return;
      const nodes = diagramNodesRef.current;
      const node = nodes.find((entry) => entry.id === payload.id);
      if (!node) return;
      const frame = getKitDiagramNodeFrameForKind(node.data.kind);
      const position = { x: payload.x - frame.width / 2, y: payload.y - frame.height / 2 };
      draggingNodeIdRef.current = payload.id;
      const selectedNodes = nodes.filter((entry) => entry.selected || selectedBoardIds.has(entry.id));
      const selectedPositions = new Map(selectedNodes.map((entry) => [entry.id, entry.position]));
      if (node.selected || selectedBoardIds.has(node.id)) {
        selectedPositions.set(node.id, position);
      }
      let nextNodes: KitDiagramLayoutNode[];
      if (selectedPositions.size > 1 && (node.selected || selectedBoardIds.has(node.id))) {
        const deltaX = position.x - node.position.x;
        const deltaY = position.y - node.position.y;
        nextNodes = nodes.map((entry) => {
          const pinned = selectedPositions.get(entry.id);
          if (!pinned) return entry;
          return entry.id === node.id
            ? { ...entry, position }
            : { ...entry, position: { x: pinned.x + deltaX, y: pinned.y + deltaY } };
        });
      } else {
        nextNodes = nodes.map((entry) => (entry.id === payload.id ? { ...entry, position } : entry));
      }
      commitDiagramNodes(nextNodes);
    },
    [commitDiagramNodes, isHandTool, selectedBoardIds],
  );

  const onBoardNodeChange = useCallback(() => {
    draggingNodeIdRef.current = null;
  }, []);

`;

text = `${text.slice(0, innerStart)}${newInnerHead}${text.slice(text.indexOf("  const filterSearchSelector", innerStart))}`;
const tailInsertAt = text.indexOf("  const kitBoardBindings = useMemo(");
const beforeTail = text.slice(0, tailInsertAt);
const afterTail = text.slice(tailInsertAt);
const cutSimulation = beforeTail.lastIndexOf("  const commitDiagramNodes = useCallback");
if (cutSimulation < 0) {
  console.error("[DEBUG] commitDiagramNodes not found for tail splice");
  process.exit(1);
}
text = `${beforeTail.slice(0, cutSimulation)}${newInnerTail}${afterTail}`;

writeFileSync(sketchpad, text, "utf8");
console.log("[DEBUG] refactored kit diagram to board WASM layout");
