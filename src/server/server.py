from concurrent import futures
import logging

import grpc

from gen.model_pb2 import Design,Element

from gen.server_pb2_grpc import ServerServicer

class Server(ServerServicer):
    def LayoutDesign(self, request, context):
        
        print("Will try to greet world ...")
        with grpc.insecure_channel('localhost:50051') as channel:
            stub = helloworld_pb2_grpc.GreeterStub(channel)
            response = stub.SayHello(helloworld_pb2.HelloRequest(name='you'))
        print("Greeter client received: " + response.message)
        elements =  
        Design(elements)
        raise NotImplementedError('Method not implemented!')


def serve():
    port = '50051'
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port('[::]:' + port)
    server.start()
    print("Server started, listening on " + port)
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    serve()