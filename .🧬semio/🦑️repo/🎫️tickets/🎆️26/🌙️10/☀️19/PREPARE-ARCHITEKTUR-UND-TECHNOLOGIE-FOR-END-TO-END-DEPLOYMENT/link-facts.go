package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func main() {
	if len(os.Args) != 3 {
		panic("expected alias and source paths")
	}
	alias, source := os.Args[1], os.Args[2]
	before, err := os.Lstat(alias)
	if err != nil {
		panic(err)
	}
	actual, err := os.Stat(alias)
	if err != nil {
		panic(err)
	}
	expected, err := os.Stat(source)
	if err != nil {
		panic(err)
	}
	target, linkError := os.Readlink(alias)
	row := map[string]any{"mode": before.Mode().String(), "modeBits": uint32(before.Mode()), "symlink": before.Mode()&os.ModeSymlink != 0, "irregular": before.Mode()&os.ModeIrregular != 0, "directory": before.IsDir(), "sameTarget": os.SameFile(actual, expected), "rawTarget": target, "sys": fmt.Sprintf("%#v", before.Sys())}
	if linkError != nil {
		row["readlinkError"] = linkError.Error()
	}
	bytes, _ := json.Marshal(row)
	fmt.Println(string(bytes))
}
