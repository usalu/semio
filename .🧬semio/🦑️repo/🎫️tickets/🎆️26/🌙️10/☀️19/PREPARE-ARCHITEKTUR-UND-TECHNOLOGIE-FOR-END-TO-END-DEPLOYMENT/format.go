package main

import (
	"bytes"
	"fmt"
	"go/format"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	root, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	for _, argument := range os.Args[1:] {
		if argument == "--" {
			continue
		}
		path, err := filepath.Abs(argument)
		if err != nil {
			panic(err)
		}
		relative, err := filepath.Rel(root, path)
		if err != nil || relative == ".." || strings.HasPrefix(relative, ".."+string(os.PathSeparator)) {
			panic("formatter target must stay inside the checkout")
		}
		before, err := os.ReadFile(path)
		if err != nil {
			panic(err)
		}
		formatted, err := format.Source(before)
		if err != nil {
			panic(err)
		}
		current, err := os.ReadFile(path)
		if err != nil {
			panic(err)
		}
		if !bytes.Equal(before, current) {
			panic("formatter source changed while being read")
		}
		if !bytes.Equal(before, formatted) {
			facts, err := os.Stat(path)
			if err != nil {
				panic(err)
			}
			if err := os.WriteFile(path, formatted, facts.Mode().Perm()); err != nil {
				panic(err)
			}
		}
		fmt.Println(relative)
	}
}
