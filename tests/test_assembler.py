from typing import Iterable
from semio.model import Sobject,Assembly,Layout,Element, LAYOUTSTRATEGY_BREADTHFIRST
from semio.backend.pyassembler import Assembler
import networkx as nx
from collections import deque

from google.protobuf.json_format import ParseDict
import json
import os

import matplotlib.pyplot as plt

def visualize_append_problem():
    root_assembly = Assembly(sobject_id="root", parts=[])
    child_assembly = Assembly(sobject_id="child", parts=[])
    root_assembly.parts.append(child_assembly)
    grandchild_assembly = Assembly(sobject_id="grandchild", parts=[])
    child_assembly.parts.append(grandchild_assembly)
    visualize_assemblies(["root", "child", "grandchild"],[root_assembly])
    visualize_assemblies(["root", "child", "grandchild"],[child_assembly])

def append_problem_solution():
    root_assembly = Assembly(sobject_id="root", parts=[])
    child_assembly = Assembly(sobject_id="child", parts=[])
    root_assembly.parts.append(child_assembly)
    child_assembly = root_assembly.parts[0]
    grandchild_assembly = Assembly(sobject_id="grandchild", parts=[])
    child_assembly.parts.append(grandchild_assembly)
    visualize_assemblies(["root", "child", "grandchild"],[root_assembly])

def visualize_graph(G: nx.Graph):
    nx.draw(G, with_labels=True)
    plt.show()

def visualize_assemblies(nodes: list[str], assemblies: list[Assembly]):
    DG = nx.DiGraph()
    DG.add_nodes_from(nodes)
    dq = deque(assemblies)
    while dq:
        assembly = dq.popleft()
        dq.extend(assembly.parts)
        for child_assembly in assembly.parts:
            DG.add_edge(assembly.sobject_id, child_assembly.sobject_id)
    visualize_graph(DG)

def get_assemblies_from_layoutrequest(layout: Layout) -> list[Assembly]:
    if layout.strategy != LAYOUTSTRATEGY_BREADTHFIRST:
        raise NotImplementedError
    roots = (assembly.sobject_id for assembly in layout.assemblies)
    nodes = list(sobject.id for sobject in layout.sobjects)
    edges = ((connection.connecting.sobject_id, connection.connected.sobject_id) for connection in layout.connections)
    G = nx.Graph()
    G.add_nodes_from(nodes, root=True)
    G.add_edges_from(edges)
    #visualize_graph(G)
    #visualize_assemblies(nodes, layout.assemblies)
    dq = deque(layout.assemblies)
    while dq:
        assembly = dq.popleft()
        if "assembly" in G.nodes[assembly.sobject_id]:
            print("Warning")
            continue
        G.nodes[assembly.sobject_id]["assembly"] = assembly
        #dq.extend(assembly.parts)
        for child_assembly in assembly.parts:
            G.nodes[child_assembly.sobject_id]["root"] = False
            dq.append(child_assembly)
    for parent, child in nx.edge_bfs(G, source=roots):
        if "assembly" in G.nodes[child]:
            continue
        G.nodes[parent]["assembly"].parts.append(Assembly(sobject_id=child, parts=[]))
        G.nodes[child]["assembly"] = G.nodes[parent]["assembly"].parts[-1]
        G.nodes[child]["root"] = False
    # visualize_assemblies(nodes, layout.assemblies)
    result = []
    for sobject_id in G.nodes:
        if not G.nodes[sobject_id]["root"]:
            continue
        if "assembly" not in G.nodes[sobject_id]:
            result.append(Assembly(sobject_id=sobject_id, parts=[]))
        else:
            result.append(G.nodes[sobject_id]["assembly"])
    return result

def test_get_assemblies_from_layoutrequest():
    path_to_current_file = os.path.realpath(__file__)
    current_directory = os.path.dirname(path_to_current_file)
    path_to_file = os.path.join(current_directory, "requests/assembler/sample_layoutDesignRequest_02.json")
    with open(path_to_file, 'r') as file:
        sample = json.load(file)
        request = ParseDict(sample, Layout())
        result = get_assemblies_from_layoutrequest(request)
    path_to_solution_file = os.path.join(current_directory, "requests/assembler/sample_02_result.json")
    with open(path_to_solution_file) as file:
        correct_result = [ParseDict(assembly, Assembly()) for assembly in json.load(file)["assemblies"]]
    assert result == correct_result


def assembliesToElements(self: Assembler, sobjects:Iterable[Sobject],assemblies:Iterable[Assembly])->Iterable[Element]:
    # element : Element = self.RequestElement(
        # sobject: Sobject = Sobject(),
        # target_representation_platforms:Iterable[Platform] = [],
        # target_representation_concepts:Iterable[str] = [],
        # target_representation_lods:Iterable[int] = [],
        # targets_required:bool = False)
    # pose : Pose, point: Point = self.ConnectElement(
        # sobjects:Tuple[Sobject,Sobject],
        # connection:Connection))
    raise NotImplementedError()

#test_get_assemblies_from_layoutrequest()
#visualize_append_problem()