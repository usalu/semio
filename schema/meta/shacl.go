package main

import (
	"log"
	"os"
	"path"
	"strings"
	"text/template"
)

type ShaclGenerator struct {
}

func (g ShaclGenerator) Generate(o string, s Schema) {
	shacl, err := os.ReadFile("templates/shacl.tmpl.ttl")
	if err != nil {
		log.Fatal(err)
	}
	t := template.Must(template.
		New("shacl").
		Funcs(map[string]interface{}{
			"LowercaseFirstLetter": LowercaseFirstLetter,
			"Lowercase":            strings.ToLower,
		}).
		Parse(string(shacl)))
	f, err := os.Create(path.Join(o, "semio.shapes.ttl"))
	if err != nil {
		log.Fatal(err)
	}
	t.Execute(f, s)
}

var DefaultShaclGenerator = ShaclGenerator{}
