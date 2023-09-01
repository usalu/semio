package main

import (
	"strings"
)

func LowercaseFirstLetter(name string) string {
	return strings.ToLower(string(name[0])) + name[1:]
}

func PlusTwo(i int) int {
	return i + 2
}

func (a Characterization) Summary() string {
	return a.Symbol + " " + a.Name + " (" + a.Abbreviation + "/" + a.Logogram + ") " + a.Description
}

func (a Characterization) Short() string {
	return a.Name + " (" + a.Symbol + a.Abbreviation + "/" + a.Logogram + ") "
}
