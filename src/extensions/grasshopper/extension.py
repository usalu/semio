from logging import basicConfig
from concurrent.futures import ThreadPoolExecutor
from grpc import server
from ..extensionservices.python.adapter.gen.adapter_pb2_grpc import AdapterServicer, add_AdapterServicer_to_server

class GrasshopperAdapter(AdapterServicer):
    def Attract(self, request, context):
        return super().Attract(request, context)
    def RequestRepresentation(self, request, context):
        return super().requestRepresentation(request, context)
    def RequestRepresentations(self, request, context):
        return super().requestRepresentations(request, context)

def serve():
    port = '50051'
    ghServer = server(ThreadPoolExecutor(max_workers=10))
    add_AdapterServicer_to_server(GrasshopperAdapter(), ghServer)
    ghServer.add_insecure_port('[::]:' + port)
    ghServer.start()
    print("Server started, listening on " + port)
    ghServer.wait_for_termination()

if __name__ == '__main__':
    basicConfig()
    serve()