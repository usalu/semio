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

	origData, _ := os.ReadFile(assetsDir + "kit_metabolism.json")
	var orig semio.Kit
	json.Unmarshal(origData, &orig)

	diffedData, _ := os.ReadFile(assetsDir + "kit_metabolism_diffed.json")
	var diffed semio.Kit
	json.Unmarshal(diffedData, &diffed)

	designGuid := "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8"
	pieceGuid := "019b4b71-0cfe-73a8-9259-2fffaaf97c6a"

	fmt.Printf("Before filter - orig designs: %d, diffed designs: %d\n", len(orig.Designs), len(diffed.Designs))

	orig.Designs = semio.FilterDesignsWithoutParent(orig.Designs)
	diffed.Designs = semio.FilterDesignsWithoutParent(diffed.Designs)

	fmt.Printf("After filter - orig designs: %d, diffed designs: %d\n", len(orig.Designs), len(diffed.Designs))

	for _, d := range orig.Designs {
		if d.Guid == designGuid {
			fmt.Printf("Orig design %s (%s): %d pieces\n", d.Guid, d.Name, len(d.Pieces))
			for _, p := range d.Pieces {
				if p.Guid == pieceGuid {
					fmt.Printf("  Found piece %s in orig\n", pieceGuid)
				}
			}
		}
	}

	for _, d := range diffed.Designs {
		if d.Guid == designGuid {
			fmt.Printf("Diffed design %s (%s): %d pieces\n", d.Guid, d.Name, len(d.Pieces))
			for _, p := range d.Pieces {
				if p.Guid == pieceGuid {
					fmt.Printf("  Found piece %s in diffed\n", pieceGuid)
				}
			}
		}
	}

	// Compute diff directly
	diff := semio.GetKitDiff(orig, diffed)
	if diff.Designs != nil {
		for _, u := range diff.Designs.Updated {
			if u.Design.Guid == designGuid {
				fmt.Printf("Design %s updated diff:\n", designGuid)
				fmt.Printf("  Name: %v\n", u.Diff.Name)
				fmt.Printf("  Description: %v\n", u.Diff.Description)
				if u.Diff.Pieces != nil {
					fmt.Printf("  Pieces added: %d, removed: %d\n", len(u.Diff.Pieces.Added), len(u.Diff.Pieces.Removed))
				} else {
					fmt.Println("  Pieces: nil")
				}
			}
		}
	}
}
