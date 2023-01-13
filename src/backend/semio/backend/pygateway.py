from concurrent import futures
import logging

from typing import Tuple

import grpc

from pydantic import Field

from typing import Iterable
from semio.model import Point,Sobject,Attraction,AttractionTree,Layout,Element,Design

from semio.gateway import (GatewayServer, LayoutDesignRequest)

from semio.manager import ManagerProxy,AttractionRequest,AttractionResponse


def getElementsFromAttractionTree(sonjects: Iterable[Sobject], attractions: Iterable[Attraction], attractionTree:AttractionTree, bias:Point|None = None)->Iterable[Element]:
    
    # while attractionTree.childrean.count != 0:

    # attractionTree.attraction_id
    return []

def attract(attraction: Attraction, managerProxy: ManagerProxy, targetTypeUrl: str):
    managerProxy.RequestAttraction(AttractionRequest(attraction=attraction,target_type_url=targetTypeUrl))


class Gateway(GatewayServer):
    managerProxy: ManagerProxy = Field(default_factory=ManagerProxy)
    
    

class GatewayService:
    def LayoutDesign(self, request:LayoutDesignRequest, context) -> Design:
        # attractionTree = request.layout.attractionTrees[0]
        #elements = getElementsFromAttractionTree(request.layout.sobjects,request.layout.attractions,attractionTree)
        return Design()


if __name__ == '__main__':
    logging.basicConfig()
    gateway = Gateway()
    gateway.serve()