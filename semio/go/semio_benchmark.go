// #region 🔖Header

// [👤semio📚go🧪semiobenchmarkgo](semiorepo://file/SEMIO/GO/SEMIO_BENCHMARK.GO)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

//go:build ignore

package main

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/usalu/semio/go/semio"
)

const AssetsPath = "../assets/semio"
const Iterations = 3

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
	kitInvalid := loadKit("kit_invalid.json")

	bench("Roundtrip/Metabolism", func() {

		kit, files, err := semio.KitFromZip(AssetsPath + "/metabolism.zip")
		if err != nil {
			panic(err)
		}

		schemaPath := "../sqlite/schema.sql"
		schemaData, err := os.ReadFile(schemaPath)
		if err != nil {

			panic("Schema not found at " + schemaPath + ": " + err.Error())
		}
		
		err = semio.KitToZip(kit, files, "temp_benchmark_metabolism.zip", string(schemaData))
		if err != nil {
			panic(err)
		}
		os.Remove("temp_benchmark_metabolism.zip")
	})

	diffForward := loadKitDiff("diff_kit_metabolism.json")
	diffInverse := loadKitDiff("diff_kit_metabolism_inverted.json")
	bench("Diff/Metabolism", func() {
		k2 := semio.ApplyKitDiff(kitMetabolism, diffForward)
		semio.ApplyKitDiff(k2, diffInverse)
	})

	d1 := findDesign(kitMetabolism, "Nakagin Capsule Tower", "")
	bench("Flatten Design/Nakagin Capsule Tower", func() {
		semio.FlattenDesign(&kitMetabolism, d1.Guid)
	})

	d2 := findDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Slanted", func() {
		semio.FlattenDesign(&kitMetabolism, d2.Guid)
	})

	d3 := findDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Twisted", func() {
		semio.FlattenDesign(&kitMetabolism, d3.Guid)
	})

	d4 := findDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower")
	bench("Flatten Design/Nakagin Capsule Tower/Dancing", func() {
		semio.FlattenDesign(&kitMetabolism, d4.Guid)
	})

	d5 := findDesign(kitMetabolism, "Capsule Dream", "")
	bench("Flatten Design/Capsule Dream", func() {
		semio.FlattenDesign(&kitMetabolism, d5.Guid)
	})

	bench("Validation/Invalid Kit", func() {
		semio.ValidateKit(kitInvalid)
	})

	bench("Validation/Metabolism", func() {
		semio.ValidateKit(kitMetabolism)
	})
}
