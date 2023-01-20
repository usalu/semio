package main

import (
  "context"
  "flag"
  "net/http"

  "github.com/golang/glog"
  "github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
  "google.golang.org/grpc"
  "google.golang.org/grpc/credentials/insecure"

  "github.com/usalu/semio/src/packages/go/gateway/v1"
//   gw "github.com/yourorg/yourrepo/proto/gen/go/your/service/v1/your_service"  // Update
)

var (
  // command-line options:
  // gRPC server endpoint
  grpcServerEndpoint = flag.String("grpc-server-endpoint",  "localhost:50001", "gRPC server endpoint")
)

func run() error {
  ctx := context.Background()
  ctx, cancel := context.WithCancel(ctx)
  defer cancel()

  // Register gRPC server endpoint
  // Note: Make sure the gRPC server is running properly and accessible
  mux := runtime.NewServeMux()
  opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
  err := gw.RegisterGatewayServiceHandlerFromEndpoint(ctx, mux,  *grpcServerEndpoint, opts)
  if err != nil {
    return err
  }

  // Start HTTP server (and proxy calls to gRPC server endpoint)
  return http.ListenAndServe(":8081", mux)
}

func main() {
  flag.Parse()
  defer glog.Flush()

  if err := run(); err != nil {
    glog.Fatal(err)
  }
}