import logging

from typing import Iterable

from semio.assembler import LayoutDesignRequest

from semio.model import Point,Sobject,Connection,Assembly,Layout,Representation,Element,Design
from semio.gateway import GatewayServer

class Gateway(GatewayServer):

    def LayoutDesign(self, request:LayoutDesignRequest, context) -> Design:
        assemblerProxy = self.getAssemblerProxy()
        response = assemblerProxy.LayoutDesign(request)
        return response
        # return Design(elements=[Element(representations=[Representation(text="I am almost Ifc.")])])

if __name__ == '__main__':
    logging.basicConfig()
    gateway = Gateway()
    gateway.serve()