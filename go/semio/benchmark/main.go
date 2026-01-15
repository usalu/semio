package main

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/usalu/semio/go/semio"
)

const AssetsPath = "../../../assets/semio"
const Iterations = 100

func loadKit(filename string) semio.Kit {
	data, err := os.ReadFile(AssetsPath + "/" + filename)
	if err != nil {
		panic(err)
	}
	var kit semio.Kit
	if err := json.Unmarshal(data, &kit); err != nil {
		panic(err)
	}
	return kit
}

func loadKitDiff(filename string) semio.KitDiff {
	data, err := os.ReadFile(AssetsPath + "/" + filename)
	if err != nil {
		panic(err)
	}
	var diff semio.KitDiff
	if err := json.Unmarshal(data, &diff); err != nil {
		panic(err)
	}
	return diff
}

func bench(name string, f func()) {
	start := time.Now()
	for i := 0; i < Iterations; i++ {
		f()
	}
	duration := time.Since(start).Seconds() / float64(Iterations)
	fmt.Printf("%s,%.6f\n", name, duration)
}

func findDesign(kit semio.Kit, name string, parentName string) semio.Design {
	var parentGuid string
	if parentName != "" {
		for _, d := range kit.Designs {
			if d.Name == parentName {
				parentGuid = d.Guid
				break
			}
		}
		if parentGuid == "" {
			panic("Parent design not found: " + parentName)
		}
	}

	for _, d := range kit.Designs {
		if d.Name == name {
			if parentName == "" {
				if d.Parent == nil {
					return d
				}
			} else {
				if d.Parent != nil && d.Parent.Guid == parentGuid {
					return d
				}
			}
		}
	}
	panic("Design not found: " + name)
}

func main() {
	kitMetabolism := loadKit("kit_metabolism.json")
	// kitInvalid := loadKit("kit_invalid.json") // Not used in original Go benchmarks apparently

	// 1. Roundtrip/Metabolism
	bench("Roundtrip/Metabolism", func() {
		data, _ := semio.SerializeKit(kitMetabolism)
		semio.DeserializeKit(data)
	})

	// 2. Diff/Metabolism
	diffForward := loadKitDiff("diff_kit_metabolism.json")
	diffInverse := loadKitDiff("diff_kit_metabolism_inverted.json")
	bench("Diff/Metabolism", func() {
		k2 := semio.ApplyKitDiff(kitMetabolism, diffForward)
		semio.ApplyKitDiff(k2, diffInverse)
	})

	// 3. Flatten Design/Nakagin Capsule Tower
	d1 := findDesign(kitMetabolism, "Nakagin Capsule Tower", "")
	bench("Flatten Design/Nakagin Capsule Tower", func() {
		semio.FlattenDesign(&kitMetabolism, d1.Guid)
	})

	// 4. Flatten Design/Nakagin Capsule Tower/Slanted
	d2 := findDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Slanted", func() {
		semio.FlattenDesign(&kitMetabolism, d2.Guid)
	})

	// 5. Flatten Design/Nakagin Capsule Tower/Twisted
	d3 := findDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Twisted", func() {
		semio.FlattenDesign(&kitMetabolism, d3.Guid)
	})

	// 6. Flatten Design/Nakagin Capsule Tower/Dancing
	d4 := findDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Dancing", func() {
		semio.FlattenDesign(&kitMetabolism, d4.Guid)
	})

	// 7. Flatten Design/Capsule Dream
	d5 := findDesign(kitMetabolism, "Capsule Dream", "")
	bench("Flatten Design/Capsule Dream", func() {
		semio.FlattenDesign(&kitMetabolism, d5.Guid)
	})

	// Add validation benchmarks if needed to match others?
	// Sticking to minimal changes + diff/metabolism.
	// If Go didn't have validation benchmarks before, I won't add them now unless asked.
}
