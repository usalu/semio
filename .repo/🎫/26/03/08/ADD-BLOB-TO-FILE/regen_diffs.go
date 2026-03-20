//go:build ignore

package main

import (
"encoding/json"
"fmt"
"os"

semio "github.com/usalu/semio/go/semio"
)

func main() {
assetsDir := "semio/assets/semio/"

origData, err := os.ReadFile(assetsDir + "kit_metabolism.json")
if err != nil {
fmt.Fprintf(os.Stderr, "Error reading original: %v\n", err)
os.Exit(1)
}
var orig semio.Kit
if err := json.Unmarshal(origData, &orig); err != nil {
fmt.Fprintf(os.Stderr, "Error parsing original: %v\n", err)
os.Exit(1)
}
orig.Designs = semio.FilterDesignsWithoutParent(orig.Designs)

diffedData, err := os.ReadFile(assetsDir + "kit_metabolism_diffed.json")
if err != nil {
fmt.Fprintf(os.Stderr, "Error reading diffed: %v\n", err)
os.Exit(1)
}
var diffed semio.Kit
if err := json.Unmarshal(diffedData, &diffed); err != nil {
fmt.Fprintf(os.Stderr, "Error parsing diffed: %v\n", err)
os.Exit(1)
}
diffed.Designs = semio.FilterDesignsWithoutParent(diffed.Designs)

diff := semio.GetKitDiff(orig, diffed)

diffJSON, err := json.MarshalIndent(diff, "", "  ")
if err != nil {
fmt.Fprintf(os.Stderr, "Error marshaling diff: %v\n", err)
os.Exit(1)
}
if err := os.WriteFile(assetsDir+"diff_kit_metabolism.json", diffJSON, 0644); err != nil {
fmt.Fprintf(os.Stderr, "Error writing diff: %v\n", err)
os.Exit(1)
}
fmt.Println("Wrote diff_kit_metabolism.json")

inverseDiff := semio.InverseKitDiff(orig, diff)
inverseJSON, err := json.MarshalIndent(inverseDiff, "", "  ")
if err != nil {
fmt.Fprintf(os.Stderr, "Error marshaling inverse diff: %v\n", err)
os.Exit(1)
}
if err := os.WriteFile(assetsDir+"diff_kit_metabolism_inverted.json", inverseJSON, 0644); err != nil {
fmt.Fprintf(os.Stderr, "Error writing inverse diff: %v\n", err)
os.Exit(1)
}
fmt.Println("Wrote diff_kit_metabolism_inverted.json")

applied := semio.ApplyKitDiff(orig, diff)
if !semio.AreKitsEqual(applied, diffed) {
fmt.Println("WARNING: Applied diff does NOT equal diffed kit!")
} else {
fmt.Println("OK: Applied diff equals diffed kit")
}
appliedBack := semio.ApplyKitDiff(diffed, inverseDiff)
if !semio.AreKitsEqual(appliedBack, orig) {
fmt.Println("WARNING: Applied inverse diff does NOT equal original kit!")
} else {
fmt.Println("OK: Applied inverse diff equals original kit")
}
}
