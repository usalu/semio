package main

type Generator interface {
	Generator(path string, s Schema)
}
