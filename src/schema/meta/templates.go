package main

import (
	"strings"
)

func (p Property) LowercaseFirstLetter(name string) string {
	return strings.ToLower(string(name[0])) + name[1:]
}
