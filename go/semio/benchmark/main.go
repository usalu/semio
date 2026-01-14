package main

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/usalu/semio/go/semio"
)

const AssetsPath = "../../assets/semio"
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
	kitInvalid := loadKit("kit_invalid.json")

	// 1. Roundtrip/Metabolism
	bench("Roundtrip/Metabolism", func() {
		data, _ := semio.SerializeKit(kitMetabolism)
		semio.DeserializeKit(data)
	})

	// 2. Flatten Design/Nakagin Capsule Tower
	d1 := findDesign(kitMetabolism, "Nakagin Capsule Tower", "")
	bench("Flatten Design/Nakagin Capsule Tower", func() {
		diff := semio.FlattenDesign(&kitMetabolism, d1.Guid)
		semio.ApplyDesignDiff(d1, diff)
	})

	// 3. Flatten Design/Nakagin Capsule Tower/Slanted
	d2 := findDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Slanted", func() {
		diff := semio.FlattenDesign(&kitMetabolism, d2.Guid)
		semio.ApplyDesignDiff(d2, diff)
	})

	// 4. Flatten Design/Nakagin Capsule Tower/Twisted
	d3 := findDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Twisted", func() {
		diff := semio.FlattenDesign(&kitMetabolism, d3.Guid)
		semio.ApplyDesignDiff(d3, diff)
	})

	// 5. Flatten Design/Nakagin Capsule Tower/Dancing
	d4 := findDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Dancing", func() {
		diff := semio.FlattenDesign(&kitMetabolism, d4.Guid)
		semio.ApplyDesignDiff(d4, diff)
	})

	// 6. Flatten Design/Capsule Dream
	d5 := findDesign(kitMetabolism, "Capsule Dream", "")
	bench("Flatten Design/Capsule Dream", func() {
		diff := semio.FlattenDesign(&kitMetabolism, d5.Guid)
		semio.ApplyDesignDiff(d5, diff)
	})

	// 7. Validation/Invalid Kit
	bench("Validation/Invalid Kit", func() {
		semio.ValidateKit(kitInvalid)
	})

	// 8. Validation/Metabolism
	bench("Validation/Metabolism", func() {
		semio.ValidateKit(kitMetabolism)
	})
}
