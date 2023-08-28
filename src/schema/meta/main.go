package main

func main() {

	// DefaultShaclGenerator.Generate("../linkeddata", Semio)
	// DefaultProtobufGenerator.Generate("../protobuf", Semio)

	DefaultShaclGenerator.Generate("out/semio.shapes.ttl", Semio)
	DefaultProtobufGenerator.Generate("out/semio.proto", Semio)
}
