import logging

from typing import Iterable

from semio.manager import ConnectionRequest,ConnectionResponse, ElementRequest

from semio.model import Point,Sobject,Connection,Assembly,Layout,Element,Design
from semio.assembler import AssemblerServer, LayoutDesignRequest

def getElementsFromAssembly(sonjects: Iterable[Sobject], connections: Iterable[Connection], assembly:Assembly, bias:Point|None = None)->Iterable[Element]:
    # while assembly.childrean.count != 0:
    # assembly.connection_id
    return []

class Assembler(AssemblerServer):

    def connect(self, connection: Connection, targetTypeUrl: str):
        self.managerProxy.RequestConnection(ConnectionRequest(connection=connection,target_type_url=targetTypeUrl))

    def LayoutDesign(self, request:LayoutDesignRequest, context) -> Design:
        managerProxy = self.getManagerProxy()
        elements = []
        for sobject in request.layout.sobjects:
            elements.append(managerProxy.RequestElement(request=ElementRequest(sobject=sobject)))
        # assembly = request.layout.assemblies[0]
        #elements = getElementsFromAssembly(request.layout.sobjects,request.layout.connections,assembly)
        return Design(elements=elements)

if __name__ == '__main__':
    logging.basicConfig()
    assembler = Assembler()
    assembler.serve()