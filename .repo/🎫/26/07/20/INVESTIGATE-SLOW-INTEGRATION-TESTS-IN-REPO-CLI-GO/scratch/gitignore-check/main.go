package main

import (
	"fmt"
	"strings"

	"github.com/bmatcuk/doublestar/v4"
)

func matchesIgnorePattern(path string, isDir bool, pattern string) bool {
	pattern = strings.TrimSpace(pattern)
	if pattern == "" {
		return false
	}
	path = strings.TrimSpace(path)
	if path == "" {
		return false
	}
	candidates := []string{path}
	if isDir && !strings.HasSuffix(path, "/") {
		candidates = append(candidates, path+"/")
	}
	patterns := []string{pattern}
	if strings.HasSuffix(pattern, "/**") {
		base := strings.TrimSuffix(pattern, "/**")
		if base != "" {
			patterns = append(patterns, base, base+"/")
		}
	}
	for _, candidatePattern := range patterns {
		if candidatePattern == "" {
			continue
		}
		for _, candidatePath := range candidates {
			if matched, _ := doublestar.Match(candidatePattern, candidatePath); matched {
				return true
			}
		}
	}
	return false
}

func main() {
	tests := []struct {
		path  string
		isDir bool
	}{
		{"target", true},
		{"framework/core/rs/target", true},
		{"mathematical/entropy/rs/target", true},
		{"node_modules", true},
		{"framework/some/node_modules", true},
	}
	patterns := []string{"target/", "target*", "node_modules"}
	for _, tst := range tests {
		for _, p := range patterns {
			m := matchesIgnorePattern(tst.path, tst.isDir, p)
			fmt.Printf("path=%-40s pattern=%-15s -> %v\n", tst.path, p, m)
		}
	}
}
