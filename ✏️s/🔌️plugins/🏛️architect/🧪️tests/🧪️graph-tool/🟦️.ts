/** 🕸️ Graphology and Ajv independently judge the graph transaction fixture consumed by the native editor. */
import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { UndirectedGraph } from "graphology";
import schema from "../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows/🔣️.json";
import fixture from "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🛠️graph-tool/🔣️.json";

type GraphOperation = { operation: string; sourceNodeId: string; targetNodeId: string; synapseId: string; nodeIds: string[]; synapseIds: string[] };
const validate = new Ajv({ strict: false }).compile(schema);
for (const row of fixture.cases) test(`architect graph tool ${row.name}`, () => {
  const args = { operations: row.rows };
  let refused = !validate(args);
  const leaves: string[] = [];
  if (!refused) {
    const graph = new UndirectedGraph();
    graph.addNode("a"); graph.addNode("b");
    if (!("linked" in row) || row.linked !== false) graph.addUndirectedEdgeWithKey("edge", "a", "b");
    try {
      for (const operation of row.rows as unknown as GraphOperation[]) {
        if (operation.operation === "connect") {
          if (!graph.hasNode(operation.sourceNodeId) || !graph.hasNode(operation.targetNodeId) || operation.sourceNodeId === operation.targetNodeId) throw Error("unknown node");
          if (!graph.hasEdge(operation.sourceNodeId, operation.targetNodeId)) { graph.addEdge(operation.sourceNodeId, operation.targetNodeId); leaves.push("connect-adjacency"); }
        } else if (operation.operation === "disconnect") {
          if (!graph.hasEdge(operation.synapseId)) throw Error("unknown edge");
          graph.dropEdge(operation.synapseId); leaves.push("disconnect-adjacency");
        } else if (operation.operation === "delete") {
          for (const id of operation.synapseIds) { graph.dropEdge(id); leaves.push("disconnect-adjacency"); }
          const edges = new Set(operation.nodeIds.flatMap(id => graph.edges(id)));
          for (const edge of edges) { graph.dropEdge(edge); leaves.push("disconnect-adjacency"); }
          for (const id of operation.nodeIds) { graph.dropNode(id); leaves.push("delete-program-element"); }
        } else throw Error("unsupported row");
      }
    } catch { refused = true; }
  }
  expect(refused).toBe("refused" in row && row.refused === true);
  if (!refused) expect(leaves).toEqual("mutations" in row ? row.mutations : []);
});
