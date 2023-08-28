package main

import (
	"log"
	"os"
	"text/template"
)

type ShaclGenerator struct {
}

func (g ShaclGenerator) Generate(path string, s Schema) {
	shacl, err := os.ReadFile("templates/shacl.tmpl.ttl")
	if err != nil {
		log.Fatal(err)
	}
	t := template.Must(template.New("shacl").Parse(string(shacl)))
	f, err := os.Create(path)
	if err != nil {
		log.Fatal(err)
	}
	t.Execute(f, s)
}

var DefaultShaclGenerator = ShaclGenerator{}
