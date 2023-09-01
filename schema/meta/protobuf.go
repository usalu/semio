package main

import (
	"log"
	"os"
	"path"
	"strings"
	"text/template"
)

type ProtobufGenerator struct {
}

func (g ProtobufGenerator) Generate(o string, s Schema) {
	const n = "protobuf"
	tf, err := os.ReadFile(path.Join("templates", n+".tmpl.proto"))
	if err != nil {
		log.Fatal(err)
	}
	t := template.Must(template.
		New(n).
		Funcs(map[string]interface{}{
			"LowercaseFirstLetter": LowercaseFirstLetter,
			"Lowercase":            strings.ToLower,
			"PlusTwo":              PlusTwo,
		}).
		Parse(string(tf)))
	for _, p := range s.Packages {
		f, err := os.Create(path.Join(o, LowercaseFirstLetter(p.Characterization.Name)+".proto"))
		if err != nil {
			log.Fatal(err)
		}
		t.Execute(f, p)
	}
}

var DefaultProtobufGenerator = ProtobufGenerator{}
