package main

import (
	"os"
	"strings"

	"github.com/gocarina/gocsv"
)

type Definition struct {
	Category, Owner, Kind, Name,
	Symbol, Logogram, Abbreviation, Explanation, Comment,
	PackageImports string
	PropertyOwns                                                       bool
	GrasshopperGuid, GrasshopperConstructGuid, GrasshopperDestructGuid string
}

type Characterization struct {
	Symbol, Logogram, Abbreviation, Name, Explanation string
}

type Type struct { // Either a Class, Number, Bool, Text or Date
	Cardinality string // (exactly) 1 | (zero or one) ? | (zero or more) * | (one or more) +
	Class       *Class
	Name        string
}

type OneOfTypes struct {
	Types []Type
}

type Property struct { // Either the property has one type or it is one of several types.
	Characterization Characterization
	Type             Type
	OneOfTypes       OneOfTypes
	Owns             bool // If the property is owned, then it will be nested in serialization which are hierarchical. Otherwise it will be referenced by id.
}

type Grasshopper struct {
	Guid, ConstructGuid, DestructGuid string // GUIDs for components
}

type Class struct {
	Characterization Characterization
	Super            *Class
	Properties       []Property
	Grasshopper
}

type Package struct {
	Characterization Characterization
	Owner            *Package
	Imports          []*Package
	Classes          []Class
}

type Schema struct {
	Characterization Characterization
	Version          string
	Packages         []Package
}

func parseDefinitions() []*Definition {
	dsf, err := os.Open("definitions.csv")
	if err != nil {
		panic(err)
	}
	defer dsf.Close()
	ds := []*Definition{}
	if err := gocsv.UnmarshalFile(dsf, &ds); err != nil {
		panic(err)
	}
	return ds
}

func (s Schema) addDefinition(d Definition) {

}

func parseSchema() Schema {
	schema := Schema{}
	packageM := map[string]*Package{}
	classM := map[string]*Class{}
	var p *Package
	var c *Class
	for _, dP := range parseDefinitions() {
		d := *dP
		ch := Characterization{
			d.Symbol,
			d.Logogram,
			d.Abbreviation,
			d.Name,
			d.Explanation,
		}
		switch d.Category {
		case "term":
			continue
		case "package":
			is := []*Package{}
			for _, i := range strings.Split(d.PackageImports, ",") {
				is = append(is, packageM[strings.TrimSpace(i)])
			}
			schema.Packages = append(schema.Packages, Package{
				ch, packageM[d.Owner], is, nil,
			})
		case "class":
			p = packageM[d.Owner]
			(*p).Classes = append((*p).Classes, Class{
				ch, classM[d.Kind], nil, Grasshopper{
					d.GrasshopperGuid,
					d.GrasshopperConstructGuid,
					d.GrasshopperDestructGuid,
				},
			})
		case "property":
			c = classM[d.Owner]
			(*c).Properties = append((*p).Properties, Property{
				ch,
			})
		}
	}
	return schema
}

// func parseDefinitions() map[string]map[string]map[string]map[string]interface{} {
// 	dsf, err := os.Open("definitions.csv")
// 	if err != nil {
// 		panic(err)
// 	}
// 	defer dsf.Close()

// 	ds := []*Definition{}

// 	if err := gocsv.UnmarshalFile(dsf, &ds); err != nil {
// 		panic(err)
// 	}

// 	var cm = map[string]map[string]map[string]map[string]interface{}{}

// 	for _, d := range ds {
// 		cm[d.Kind][d.Parent][d.Name]["characterization"] = Characterization{
// 			(*d).Symbol,
// 			(*d).Logogram,
// 			(*d).Abbreviation,
// 			(*d).Name,
// 			(*d).Explanation,
// 		}
// 		cm[d.Kind][d.Parent][d.Name]["grasshopper"] = Grasshopper{
// 			(*d).GrasshopperGuid,
// 			(*d).GrasshopperConstructGuid,
// 			(*d).GrasshopperDestructGuid,
// 		}
// 	}
// 	return cm
// }

// var cm = parseDefinitions()

// var artifactRCD = cm["class"]["repository"]["artifact"]
// var artifactRC = Class{
// 	artifactRCD["characterization"].(Characterization),
// 	nil,
// 	nil,
// 	artifactRCD["grasshopper"].(Grasshopper),
// }

// var modelP = Package{
// 	cm["package"]["model"]["model"],
// 	nil,
// 	nil,
// 	[]Class{
// 		artifactRC,
// 	},
// }

// var Semio = Schema{
// 	csm["semio"],
// 	"1",
// 	[]Package{
// 		modelP,
// 	},
// }

// var artC ={
// 	Characterization{"📝", "Artifact", "art", "A", "is anything that is produced by human or a computer."},
// 	"",
// 	[]Property{
// 		{
// 			Characterization{"", "Identifier", "id", "ID",
// 				"is a unique identifier to distingusish it from another artifact.\n" +
// 					"This only applies to artifacts of the same subtype"},
// 			"String", "1", true,
// 		},
// 		{
// 			Characterization{"🆔", "Explanation", "id", "ID",
// 				"is a unique identifier to distingusish it from another artifact.\n" +
// 					"This only applies to artifacts of the same subtype"},
// 			"String", "1", true,
// 		},
// 	},
// },
// {
// 	Characterization{"🖋️", "Delineation", "dln", "Dl", "is anything that decouples something complex."},
// 	"Characterization",
// 	[]Property{
// 		{
// 			Characterization{"🔣", "Symbol", "syb", "Sb",
// 				"represents the Characterization symbolically.\n" +
// 					"This is very usefull for making figures.",
// 			},
// 			"String", "?", true,
// 		},
// 	},
// },

// var valC = Class{
// 	Characterization{"🎚️", "Value", "val", "V", "that will be mapped to a native type in the corresponding platform."},
// 	nil,
// 	[]Property{},
// 	Grasshoper{
// 		"CED240A3-A54A-4075-9AB8-365C7CAB9433",
// 	},
// }

// var numC = Class{
// 	Characterization{"💯", "Number", "num", "Nu", "is a numeric value."},
// 	&valC,
// }

// var dcmC = Class{
// 	Characterization{"🧾", "Decimal", "dcm", "DN", "is a decimal numeric value. "},
// 	&valC,
// }

// var repoC = Class{
// 	Characterization{"🗃️", "Repository", "repo", "R", "is a container for Artifacts."},
// 	"",
// 	[]Property{
// 		{
// 			Characterization{"🔗", "Uniqueresourcelocator", "url", "U",
// 				"is a unique location for a resource.\n" +
// 					"An example is: github.com/usalu/semio/examples/nakagincapsuletower"},
// 			"String", "1", true,
// 		},
// 		{
// 			Characterization{"🖧", "Layouts", "lyts", "L", ""},
// 			"Layout", "*", true,
// 		},
// 	},
// }

// var geoP = Package{
// 	Characterization{"🌐", "Geometry", "geo", "G", "contains all geometry related models."},
// }

// var mdlP = Package{
// 	Characterization{"⭕", "Model", "mdl", "M", "contains all semio specific models."},
// 	&geometryPackage,
// 	[]

// 		{
// 			Characterization{"🖧", "Layout", "lyt", "L", ""},
// 			"Delineation",
// 			[]Property{
// 				{
// 					Characterization{"🛈", "Sobjects", "sbjs", "S", ""},
// 					"Sobject", "+", true,
// 				},
// 			},
// 		},
// 		{
// 			csm["sbj"],
// 			&dlnC,
// 			[]Property{},
// 		},
// 	}
// }

// var Semio = Schema{
// 	csm["se"],
// 	"1",
// 	[]Package{

// 	},
// }
