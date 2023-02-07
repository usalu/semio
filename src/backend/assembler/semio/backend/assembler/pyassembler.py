import logging

from typing import Iterable,Tuple
from collections import deque

from semio.geometry import Point
from semio.model import Pose,Platform,Sobject,Connection,Assembly,Layout,Prototype,Element,Design,LAYOUTSTRATEGY_BREADTHFIRST,PLATFORM_SEMIO
from semio.assembler import AssemblerServer,LayoutToAssembliesResponse,AssemblyToElementsRequest,AssemblyToElementsResponse
from semio.utils import hashObject, subtract, adjustPointOfView

from networkx import Graph,edge_bfs,draw,DiGraph
from matplotlib.pyplot import plot

def visualize_graph(G: Graph):
    draw(G, with_labels=True)
    plot.show()

def visualize_assemblies(nodes: list[str], assemblies: list[Assembly]):
    DG = DiGraph()
    DG.add_nodes_from(nodes)
    dq = deque(assemblies)
    while dq:
        assembly = dq.popleft()
        dq.extend(assembly.parts)
        for child_assembly in assembly.parts:
            DG.add_edge(assembly.sobject_id, child_assembly.sobject_id)
    visualize_graph(DG)

def findSobjectById(sobjects:Iterable[Sobject],id:str)->Sobject:
    return next(sobject for sobject in sobjects if sobject.id==id)

def findConnection(connected_sobject: Sobject, connecting_sobject: Sobject, connections: Iterable[Connection])->Connection:
    return next(connection for connection in connections if (
        (connection.connected==connected_sobject.id) and (connection.connecting==connecting_sobject.id)
        or(connection.connected==connecting_sobject.id) and (connection.connecting==connected_sobject.id)))

def getElement(sobject:Sobject, pointOfView:Point | None = None)->Element:
    if pointOfView:
        sobject = adjustPointOfView(sobject,pointOfView)
    return Element(sobject_id=sobject.id,pose=sobject.pose,prototype_plan_hash=hashObject(sobject.plan))

class Assembler(AssemblerServer):

    def layoutToAssemblies(self, layout: Layout):
        if layout.strategy != LAYOUTSTRATEGY_BREADTHFIRST:
            raise NotImplementedError()
        roots = (assembly.sobject_id for assembly in layout.assemblies)
        nodes = list(sobject.id for sobject in layout.sobjects)
        edges = ((connection.connecting.sobject_id, connection.connected.sobject_id) for connection in layout.connections)
        G = Graph()
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
        for parent, child in edge_bfs(G, source=roots):
            if "assembly" in G.nodes[child]:
                continue
            G.nodes[parent]["assembly"].parts.append(Assembly(sobject_id=child, parts=[]))
            G.nodes[child]["assembly"] = G.nodes[parent]["assembly"].parts[-1]
            G.nodes[child]["root"] = False
        # visualize_assemblies(nodes, layout.assemblies)
        assemblies = []
        for sobject_id in G.nodes:
            if not G.nodes[sobject_id]["root"]:
                continue
            if "assembly" not in G.nodes[sobject_id]:
                assemblies.append(Assembly(sobject_id=sobject_id, parts=[]))
            else:
                assemblies.append(G.nodes[sobject_id]["assembly"])
        return assemblies

    def assemblyToElements(self, assembly:Assembly, sobjects: Iterable[Sobject], connections: Iterable[Connection] | None = None)->Iterable[Element]:
        sobject = findSobjectById(sobjects,assembly.sobject_id)
        elements = [getElement(sobject)]
        if assembly.parts:
            for part in assembly.parts:
                elements += self.partToElements(sobject.pose.point_of_view,sobject,part,sobjects,connections)
        return elements

    def partToElements(self,origin:Point, parent:Sobject,part:Assembly, sobjects: Iterable[Sobject], connections: Iterable[Connection] | None = None)->Iterable[Element]:
        sobject = findSobjectById(sobjects,part.sobject_id)
        pose,point = self.ConnectElement(parent,sobject,findConnection(parent,sobject,connections))
        discrepancy = subtract(sobject.pose.point_of_view,pose.point_of_view)
        elements = [getElement(sobject,pose)]
        if part.parts:
            for part in part.parts:
                elements += self.partToElements(sobject,part,sobjects,connections)
        return elements

if __name__ == '__main__':
    logging.basicConfig()
    assembler = Assembler()
    assembler.serve()