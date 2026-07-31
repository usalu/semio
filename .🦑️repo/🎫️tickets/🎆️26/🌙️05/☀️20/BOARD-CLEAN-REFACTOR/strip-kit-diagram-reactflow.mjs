import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const sketchpad = join(import.meta.dirname, "../../../../../../compose/client/lib/sketchpad/react/index.tsx");
let text = readFileSync(sketchpad, "utf8");

const start = text.indexOf("/** KitArtifactNode holds the data fields for a KitArtifactNode record.");
const end = text.indexOf(" * buildKitDiagramData holds the data fields for a buildKitDiagramData record.");
if (start < 0 || end < 0 || end <= start) {
  console.error("[DEBUG] markers not found", { start, end });
  process.exit(1);
}
text = `${text.slice(0, start)}${text.slice(end)}`;

text = text.replace(
  "const buildKitDiagramData = (kit: Kit): { nodes: Node<KitDiagramNode>[]; edges: Edge[] } => {\n  const nodes: Node<KitDiagramNode>[] = [];\n  const edges: Edge[] = [];",
  "const buildKitDiagramData = (kit: Kit): { nodes: KitDiagramLayoutNode[]; edges: KitDiagramEdge[] } => {\n  const nodes: KitDiagramLayoutNode[] = [];\n  const edges: KitDiagramEdge[] = [];",
);

text = text.replace(
  /nodes\.push\(\{\n        id: nodeId,\n        type: "artifact",\n        position: \{ x: 0, y: 0 \},\n        width: frame\.width,\n        height: frame\.height,\n        data: \{/g,
  `nodes.push({
        id: nodeId,
        position: { x: 0, y: 0 },
        width: frame.width,
        height: frame.height,
        selected: false,
        data: {`,
);

text = text.replace(
  /edges\.push\(\{\n          id: `\$\{kind\}-\$\{item\.parentId\}-\$\{item\.id\}`,\n          source: `\$\{parentKind\}:\$\{item\.parentId\}`,\n          target: nodeId,\n          type: "floating",\n          style: edgeStyle\["part-of"\],\n          data: \{ relationship: "part-of" \},\n        \}\);/g,
  `edges.push({
          id: \`\${kind}-\${item.parentId}-\${item.id}\`,
          source: \`\${parentKind}:\${item.parentId}\`,
          target: nodeId,
          relationship: "part-of",
        });`,
);

text = text.replace(
  /edges\.push\(\{\n            id: edgeId,\n            source: sourceId,\n            target: targetId,\n            type: "floating",\n            style: edgeStyle\["reference"\],\n            data: \{ relationship: "reference" \},\n          \}\);/g,
  `edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            relationship: "reference",
          });`,
);

writeFileSync(sketchpad, text, "utf8");
console.log("[DEBUG] stripped legacy kit diagram React Flow components");
