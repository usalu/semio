package main

type ProtobufGenerator struct {
	version string
}

func (g ProtobufGenerator) Generate(path string, schema Schema) {
}

var DefaultProtobufGenerator = ProtobufGenerator{"3"}
