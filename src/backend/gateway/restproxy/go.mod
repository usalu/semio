module github.com/usalu/semio/src/backend/gateway/restproxy

go 1.19

// replace github.com/usalu/semio/src/packages/go => ../../../packages/go

require (
	github.com/golang/glog v1.0.0
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.15.0
	github.com/usalu/semio/src/packages/go v0.0.0-20230220010751-6d87fbe883f8
	google.golang.org/grpc v1.52.0
)

require (
	github.com/golang/protobuf v1.5.2 // indirect
	golang.org/x/net v0.4.0 // indirect
	golang.org/x/sys v0.3.0 // indirect
	golang.org/x/text v0.5.0 // indirect
	google.golang.org/genproto v0.0.0-20230119192704-9d59e20e5cd1 // indirect
	google.golang.org/protobuf v1.28.1 // indirect
)
