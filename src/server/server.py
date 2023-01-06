from concurrent import futures
import logging

import grpc

from pydantic import BaseModel

from semio.model import Design,Element

from semio.server import ServerServicer, add_ServerServicer_to_server, LayoutDesignRequest

class Server(BaseModel, ServerServicer):
    port: int = 50000

    def LayoutDesign(self, request : LayoutDesignRequest, context):
        elements =  []
        design = Design(elements)
        raise NotImplementedError('Method not implemented!')

    def serve(self):
        server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
        add_ServerServicer_to_server(Server(), server)
        server.add_insecure_port('[::]:' + str(self.port))
        server.start()
        print("Server started, listening on " + str(self.port))
        server.wait_for_termination()

if __name__ == '__main__':
    logging.basicConfig()
    server = Server()
    server.serve()