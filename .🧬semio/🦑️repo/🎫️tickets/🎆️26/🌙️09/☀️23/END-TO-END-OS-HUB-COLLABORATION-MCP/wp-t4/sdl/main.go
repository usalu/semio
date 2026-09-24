package main

import (
	"fmt"

	graphql "github.com/usalu/semio/repo/graphql"
)

func main() {
	fmt.Print(graphql.RenderSDL(graphql.BuildSchema()))
}
