module github.com/usalu/semio/go/cli

go 1.24.0

require (
	github.com/spf13/cobra v1.9.1
	github.com/usalu/semio/go/repo v0.0.0
)

replace github.com/usalu/semio/go/repo => ../repo

require (
	github.com/bmatcuk/doublestar/v4 v4.7.1 // indirect
	github.com/graph-gophers/graphql-go v1.5.0 // indirect
	github.com/inconshreveable/mousetrap v1.1.0 // indirect
	github.com/spf13/pflag v1.0.6 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)
