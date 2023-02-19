import logging

from typing import Iterable

try:
    from semio.geometry import Point
except ImportError:
    from geometry import Point

from semio.model import Sobject,Platform,Connection,Assembly,Layout,Representation,Prototype,Element,Design
from semio.gateway import GatewayServer
from semio.utils import hashObject

class Gateway(GatewayServer):

    def layoutDesign(self, layout:Layout, target_platform:Platform)->Design:
        
        # Task 1
        prototypes = []
        sobjectPlanHashes = {sobject.id:hashObject(sobject.plan) for sobject in layout.sobjects}

        for sobject in layout.sobjects:
            if not sobjectPlanHashes[sobject.id] in prototypes:
                prototype = self.RequestPrototype(sobject.plan,target_platform)
                assert sobjectPlanHashes[sobject.id]==prototype.plan_hash
                prototypes.append(prototype)

        # Task 2
        elements = []
        assemblies = self.LayoutToAssemblies(layout)
        for assembly in assemblies:
            elementsFromAssembly = self.AssemblyToElements(assembly,layout.sobjects,layout.connections)
            elements+=elementsFromAssembly

        # await Task 1 & 2
        return Design(prototypes=prototypes,elements=elements)
    
if __name__ == '__main__':
    logging.basicConfig()
    gateway = Gateway()
    gateway.serve()