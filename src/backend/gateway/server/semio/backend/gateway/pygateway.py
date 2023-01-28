import logging

from typing import Iterable

from semio.model import Point,Sobject,Platform,Connection,Assembly,Layout,Representation,Prototype,Element,Design
from semio.gateway import GatewayServer

class Gateway(GatewayServer):

    def layoutDesign(self, layout:Layout,target_platform:Platform)->Design:
        return Design(prototypes=[Prototype(representations=[Representation(body=b"I am almost Ifc.")])])

if __name__ == '__main__':
    logging.basicConfig()
    gateway = Gateway()
    gateway.serve()