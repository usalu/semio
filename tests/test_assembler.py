from semio.assembler import LayoutDesignRequest
from semio.model import Assembly, LAYOUTSTRATEGY_BREADTHFIRST
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

def get_assemblies_from_layoutrequest(request: LayoutDesignRequest) -> list[Assembly]:
    if request.layout.strategy != LAYOUTSTRATEGY_BREADTHFIRST:
        raise NotImplementedError
    roots = (assembly.sobject_id for assembly in request.layout.assemblies)
    nodes = list(sobject.id for sobject in request.layout.sobjects)
    edges = ((connection.connecting.sobject_id, connection.connected.sobject_id) for connection in request.layout.connections)
    G = nx.Graph()
    G.add_nodes_from(nodes, root=True)
    G.add_edges_from(edges)
    #visualize_graph(G)
    #visualize_assemblies(nodes, request.layout.assemblies)
    dq = deque(request.layout.assemblies)
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
    #visualize_assemblies(nodes, request.layout.assemblies)
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
        request = ParseDict(sample, LayoutDesignRequest())
        result = get_assemblies_from_layoutrequest(request)
    path_to_solution_file = os.path.join(current_directory, "requests/assembler/sample_02_result.json")
    with open(path_to_solution_file) as file:
        correct_result = [ParseDict(assembly, Assembly()) for assembly in json.load(file)["assemblies"]]
    assert result == correct_result


#test_get_assemblies_from_layoutrequest()