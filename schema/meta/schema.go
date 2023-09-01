package main

type Characterization struct {
	Symbol, Name, Abbreviation, Logogram, Description string
}

type Package struct {
	Characterization Characterization
	Imports          []string // The (Characterization) name of imported packages.
	Classes          []Class
}

type Property struct {
	Characterization  Characterization
	Type, Cardinality string // Type [Class | Enum | Text | Date | Number] and Cardinality [ (exactly) 1 | (zero or one) ? | (zero or more) * | (one or more) +]
	Owns              bool   // If the property is owned, then it will be nested in serialization and passed by id.
}

type Class struct {
	Characterization Characterization
	Parent           string
	Properties       []Property
}

type Schema struct {
	Characterization Characterization
	Version          string
	Packages         []Package
}

var Semio = Schema{
	Characterization{"✏️", "Semio", "se", "S", "is a framework for designing (mainly architecture)."},
	"1",
	[]Package{
		{
			Characterization{"🌐", "Geometry", "geo", "G", "contains all semio specific models."},
			[]string{},
			[]Class{},
		},
		{
			Characterization{"⭕", "Model", "mdl", "M", "contains all semio specific models."},
			[]string{"Geometry"},
			[]Class{
				{
					Characterization{"🗃️", "Repository", "repo", "R", "is a container for Artifacts."},
					"",
					[]Property{
						{
							Characterization{"🔗", "Uniqueresourcelocator", "url", "U",
								"is a unique location for a resource.\n" +
									"An example is: github.com/usalu/semio/examples/nakagincapsuletower"},
							"String", "1", true,
						},
						{
							Characterization{"🖧", "Layouts", "lyts", "L", ""},
							"Layout", "*", true,
						},
					},
				},
				{
					Characterization{"📝", "Artifact", "art", "A", "is anything that is produced by human or a computer."},
					"",
					[]Property{
						{
							Characterization{"🆔", "Identifier", "id", "ID",
								"is a unique identifier to distingusish it from another artifact.\n" +
									"This only applies to artifacts of the same subtype"},
							"String", "1", true,
						},
					},
				},
				{
					Characterization{"🖋️", "Delineation", "dln", "Dl", "is anything that decouples something complex."},
					"Characterization",
					[]Property{
						{
							Characterization{"🔣", "Symbol", "syb", "Sb",
								"represents the Characterization symbolically.\n" +
									"This is very usefull for making figures.",
							},
							"String", "+", true,
						},
					},
				},
				{
					Characterization{"🖧", "Layout", "lyt", "L", "is a graph of sobjects (nodes) and attractions (slightly directed edges)."},
					"Delineation",
					[]Property{
						{
							Characterization{"🛈", "Sobjects", "sbjs", "S", "(nodes) of the layout (graph)."},
							"Sobject", "+", true,
						},
					},
				},
				{
					Characterization{"🛈", "Sobject", "sbj", "S", "is the semantic description of an object."},
					"Delineation",
					[]Property{},
				},
			},
		},
	},
}
