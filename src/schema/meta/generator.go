package main

type Generator interface {
	Generator(path string, schema Schema)
}
