package main

type Artifact struct {
	Symbol, Name, Abbreviation, Logogram, Description,  string
}

type Package struct {
	Artifact Artifact
	Imports  []string
}

type Property struct {
	Artifact          Artifact
	Type, Cardinality string
}

type Class struct {
	Artifact                     Artifact
	PackageName, Parent, OwnedBy string
	Properties                   []Property
}

type Schema struct {
	Version  string
	Packages []Package
	Classes  []Class
}

var Semio = Schema{
	"0.2.0",
	[]Package{
		{
			Artifact{"⭕","Model", "mdl", "M", "contains all semio specific models."},
			[]string{},
		},
	},
	[]Class{
		{
			Artifact{"🗃️", "Repository", "repo", "R", "is a container for artifacts."},
			"", "", "",
			[]Property{},
		},
		{
			Artifact{"📝", "Artifact", "art", "A", "is anything that is produced by human or a computer."},
			"", "", "",
			[]Property{},
		},
		{
			Artifact{"🖋️", "Delineation", "dln", "Dl", "is anything that decouples something complex."},
			"", "", "",
			[]Property{},
		},
		{
			Artifact{"🖧", "Layout", "lyt", "L", "is a graph of sobjects (nodes) and attractions (slightly directed edges)."},
			"Model", "Delineation", "Repository",
			[]Property{},
		},
		{
			Artifact{"🛈", "Sobject", "sbj", "S", " is the semantic description of an object.",},
			"Model", "Delineation", "Layout",
			[]Property{},
		},
	},
}
