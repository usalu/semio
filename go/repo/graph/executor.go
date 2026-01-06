// #region Header

// go/repo/graph/executor.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion Header

package graph

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/language/ast"
	"github.com/graphql-go/graphql/language/parser"
)

// #region Executor

func parseFileListInput(f map[string]interface{}) *FileListInput {
	files := &FileListInput{}
	if updated, ok := f["updated"].([]interface{}); ok {
		for _, u := range updated {
			if s, ok := u.(string); ok {
				files.Updated = append(files.Updated, s)
			}
		}
	}
	if created, ok := f["created"].([]interface{}); ok {
		for _, c := range created {
			if s, ok := c.(string); ok {
				files.Created = append(files.Created, s)
			}
		}
	}
	if removed, ok := f["removed"].([]interface{}); ok {
		for _, r := range removed {
			if s, ok := r.(string); ok {
				files.Removed = append(files.Removed, s)
			}
		}
	}
	return files
}

type Executor struct {
	resolver *Resolver
	schema   graphql.Schema
}

func NewExecutor(rootDir string) (*Executor, error) {
	resolver := NewResolver(rootDir)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func NewExecutorWithContext(rootDir string, ctx RepoContext) (*Executor, error) {
	resolver := NewResolverWithContext(rootDir, ctx)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func (e *Executor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	result := graphql.Do(graphql.Params{
		Context:        ctx,
		Schema:         e.schema,
		RequestString:  query,
		VariableValues: variables,
	})
	if len(result.Errors) > 0 {
		return nil, fmt.Errorf("graphql errors: %v", result.Errors)
	}
	return result.Data, nil
}

func (e *Executor) ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error) {
	data, err := e.Execute(ctx, query, variables)
	if err != nil {
		return "", err
	}
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", err
	}
	return string(jsonBytes), nil
}

func (e *Executor) ValidateQuery(query string) error {
	_, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	return err
}

func (e *Executor) GetOperationType(query string) (string, error) {
	doc, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	if err != nil {
		return "", err
	}
	for _, def := range doc.Definitions {
		if opDef, ok := def.(*ast.OperationDefinition); ok {
			return string(opDef.Operation), nil
		}
	}
	return "query", nil
}

// #endregion Executor

// #region Schema Builder

func buildSchema(resolver *Resolver) (graphql.Schema, error) {
	positionType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Position",
		Fields: graphql.Fields{
			"line":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"column": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	rangeType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Range",
		Fields: graphql.Fields{
			"start": &graphql.Field{Type: graphql.NewNonNull(positionType)},
			"end":   &graphql.Field{Type: graphql.NewNonNull(positionType)},
		},
	})

	lineMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "LineMetrics",
		Fields: graphql.Fields{
			"added":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"removed": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	countMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "CountMetrics",
		Fields: graphql.Fields{
			"added":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"updated": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"removed": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	repoMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "RepoMetrics",
		Fields: graphql.Fields{
			"bundles":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"folders":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"files":        &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"sections":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"definitions":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":        &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"contributors": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"tickets":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	bundleMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "BundleMetrics",
		Fields: graphql.Fields{
			"folders":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"files":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"sections":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	folderMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FolderMetrics",
		Fields: graphql.Fields{
			"files":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	fileMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FileMetrics",
		Fields: graphql.Fields{
			"sections":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	sectionMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "SectionMetrics",
		Fields: graphql.Fields{
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	definitionMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "DefinitionMetrics",
		Fields: graphql.Fields{
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	priorityCountType := graphql.NewObject(graphql.ObjectConfig{
		Name: "PriorityCount",
		Fields: graphql.Fields{
			"high":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"medium": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"low":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	analyzeMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeMetrics",
		Fields: graphql.Fields{
			"total":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"byPriority":  &graphql.Field{Type: priorityCountType},
			"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	definitionKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "DefinitionKind",
		Values: graphql.EnumValueConfigMap{
			"FUNCTION":  &graphql.EnumValueConfig{Value: DefinitionKindFunction},
			"CLASS":     &graphql.EnumValueConfig{Value: DefinitionKindClass},
			"VARIABLE":  &graphql.EnumValueConfig{Value: DefinitionKindVariable},
			"INTERFACE": &graphql.EnumValueConfig{Value: DefinitionKindInterface},
			"TYPE":      &graphql.EnumValueConfig{Value: DefinitionKindType},
			"ENUM":      &graphql.EnumValueConfig{Value: DefinitionKindEnum},
			"METHOD":    &graphql.EnumValueConfig{Value: DefinitionKindMethod},
			"PROPERTY":  &graphql.EnumValueConfig{Value: DefinitionKindProperty},
		},
	})

	ticketStatusEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "TicketStatus",
		Values: graphql.EnumValueConfigMap{
			"OPEN":   &graphql.EnumValueConfig{Value: TicketStatusOpen},
			"CLOSED": &graphql.EnumValueConfig{Value: TicketStatusClosed},
		},
	})

	violationPriorityEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "ViolationPriority",
		Values: graphql.EnumValueConfigMap{
			"HIGH":   &graphql.EnumValueConfig{Value: ViolationPriorityHigh},
			"MEDIUM": &graphql.EnumValueConfig{Value: ViolationPriorityMedium},
			"LOW":    &graphql.EnumValueConfig{Value: ViolationPriorityLow},
		},
	})

	var bundleType *graphql.Object
	var folderType *graphql.Object
	var fileType *graphql.Object
	var sectionType *graphql.Object
	var definitionType *graphql.Object
	var violationType *graphql.Object
	var violationKindType *graphql.Object
	var policyType *graphql.Object
	var ticketType *graphql.Object
	var contributorType *graphql.Object
	var repoType *graphql.Object

	bundleType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Bundle",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":          &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"root":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"sourceRoot":  &graphql.Field{Type: graphql.String},
				"projectType": &graphql.Field{Type: graphql.String},
				"tags":        &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"uri":         &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"folders":     &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType)))},
				"files":       &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
				"metrics":     &graphql.Field{Type: graphql.NewNonNull(bundleMetricsType)},
			}
		}),
	})

	folderType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Folder",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":         &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"parent":     &graphql.Field{Type: folderType},
				"children":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType)))},
				"files":      &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"bundle":     &graphql.Field{Type: bundleType},
				"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
				"metrics":    &graphql.Field{Type: graphql.NewNonNull(folderMetricsType)},
			}
		}),
	})

	fileType = graphql.NewObject(graphql.ObjectConfig{
		Name: "File",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":          &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":         &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"extension":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"folder":      &graphql.Field{Type: folderType},
				"bundle":      &graphql.Field{Type: bundleType},
				"sections":    &graphql.Field{Type: graphql.NewList(sectionType)},
				"definitions": &graphql.Field{Type: graphql.NewList(definitionType)},
				"violations":  &graphql.Field{Type: graphql.NewList(violationType)},
				"metrics":     &graphql.Field{Type: fileMetricsType},
				"content":     &graphql.Field{Type: graphql.String},
			}
		}),
	})

	sectionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Section",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":          &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file":        &graphql.Field{Type: fileType},
				"parent":      &graphql.Field{Type: sectionType},
				"children":    &graphql.Field{Type: graphql.NewList(sectionType)},
				"definitions": &graphql.Field{Type: graphql.NewList(definitionType)},
				"violations":  &graphql.Field{Type: graphql.NewList(violationType)},
				"range":       &graphql.Field{Type: rangeType},
				"metrics":     &graphql.Field{Type: sectionMetricsType},
			}
		}),
	})

	definitionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Definition",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":         &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"kind":       &graphql.Field{Type: graphql.NewNonNull(definitionKindEnum)},
				"file":       &graphql.Field{Type: graphql.NewNonNull(fileType)},
				"section":    &graphql.Field{Type: sectionType},
				"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
				"range":      &graphql.Field{Type: graphql.NewNonNull(rangeType)},
				"metrics":    &graphql.Field{Type: graphql.NewNonNull(definitionMetricsType)},
			}
		}),
	})

	textEditType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TextEdit",
		Fields: graphql.Fields{
			"start":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"end":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"newText": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	fileEditType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FileEdit",
		Fields: graphql.Fields{
			"path":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"edits": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(textEditType)))},
		},
	})

	autofixType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Autofix",
		Fields: graphql.Fields{
			"description": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"edits":       &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileEditType)))},
		},
	})

	violationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Violation",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":      &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"kind":    &graphql.Field{Type: graphql.NewNonNull(violationKindType)},
				"scope":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file":    &graphql.Field{Type: fileType},
				"folder":  &graphql.Field{Type: folderType},
				"line":    &graphql.Field{Type: graphql.Int},
				"column":  &graphql.Field{Type: graphql.Int},
				"excerpt": &graphql.Field{Type: graphql.String},
				"autofix": &graphql.Field{Type: autofixType},
			}
		}),
	})

	violationKindType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ViolationKind",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":          &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"policy":      &graphql.Field{Type: graphql.NewNonNull(policyType)},
				"priority":    &graphql.Field{Type: graphql.NewNonNull(violationPriorityEnum)},
				"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"reason":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"solution":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
				},
			}
		}),
	})

	policyType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Policy",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":             &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":           &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"description":    &graphql.Field{Type: graphql.String},
				"scopes":         &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"violationKinds": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType)))},
			}
		}),
	})

	ticketDateType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketDate",
		Fields: graphql.Fields{
			"created":  &graphql.Field{Type: graphql.NewNonNull(graphql.DateTime)},
			"finished": &graphql.Field{Type: graphql.DateTime},
		},
	})

	ticketMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketMetrics",
		Fields: graphql.Fields{
			"iterations": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"bundles":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"files":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":      &graphql.Field{Type: lineMetricsType},
		},
	})

	ticketType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Ticket",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":      &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"year":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"month":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"day":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"slug":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":     &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"prompt":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"summary": &graphql.Field{Type: graphql.String},
				"status":  &graphql.Field{Type: graphql.NewNonNull(ticketStatusEnum)},
				"author":  &graphql.Field{Type: contributorType},
				"model":   &graphql.Field{Type: graphql.String},
				"commit":  &graphql.Field{Type: graphql.String},
				"date":    &graphql.Field{Type: graphql.NewNonNull(ticketDateType)},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"metrics": &graphql.Field{Type: graphql.NewNonNull(ticketMetricsType)},
			}
		}),
	})

	contributorIconsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorIcons",
		Fields: graphql.Fields{
			"avatar":      &graphql.Field{Type: graphql.String},
			"avatarRound": &graphql.Field{Type: graphql.String},
			"github":      &graphql.Field{Type: graphql.String},
		},
	})

	contributorLinkType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorLink",
		Fields: graphql.Fields{
			"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"url":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorMetrics",
		Fields: graphql.Fields{
			"commits":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"tickets":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"bundles":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"folders":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"files":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"sections":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"lines":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	contributorType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Contributor",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":      &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"github":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":    &graphql.Field{Type: graphql.String},
				"emails":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"links":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorLinkType)))},
				"icons":   &graphql.Field{Type: contributorIconsType},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType)))},
				"metrics": &graphql.Field{Type: graphql.NewNonNull(contributorMetricsType)},
			}
		}),
	})

	repoResolver := &repoResolver{resolver}

	repoType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Repo",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":   &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"bundles": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Bundles(p.Context, repo)
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Folders(p.Context, repo)
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Files(p.Context, repo)
					},
				},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Contributors(p.Context, repo)
					},
				},
				"tickets": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
					Args: graphql.FieldConfigArgument{
						"year":   &graphql.ArgumentConfig{Type: graphql.Int},
						"month":  &graphql.ArgumentConfig{Type: graphql.Int},
						"day":    &graphql.ArgumentConfig{Type: graphql.Int},
						"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						var year, month, day *int
						var status *TicketStatus
						if v, ok := p.Args["year"].(int); ok {
							year = &v
						}
						if v, ok := p.Args["month"].(int); ok {
							month = &v
						}
						if v, ok := p.Args["day"].(int); ok {
							day = &v
						}
						if v, ok := p.Args["status"].(TicketStatus); ok {
							status = &v
						}
						return repoResolver.Tickets(p.Context, repo, year, month, day, status)
					},
				},
				"policies": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Policies(p.Context, repo)
					},
				},
				"violationKinds": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.ViolationKinds(p.Context, repo)
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						var scope *string
						if v, ok := p.Args["scope"].(string); ok {
							scope = &v
						}
						return repoResolver.Violations(p.Context, repo, scope)
					},
				},
				"metrics": &graphql.Field{
					Type: graphql.NewNonNull(repoMetricsType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						return repoResolver.Metrics(p.Context, repo)
					},
				},
			}
		}),
	})

	analyzeResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeResult",
		Fields: graphql.Fields{
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
			"metrics":    &graphql.Field{Type: graphql.NewNonNull(analyzeMetricsType)},
		},
	})

	fixResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FixResult",
		Fields: graphql.Fields{
			"fixed":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"remaining":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
		},
	})

	queryResolver := &queryResolver{resolver}

	queryType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"node": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewUnion(graphql.UnionConfig{
					Name:  "Node",
					Types: []*graphql.Object{repoType, bundleType, folderType, fileType, sectionType, definitionType, contributorType, ticketType, policyType, violationKindType, violationType},
					ResolveType: func(p graphql.ResolveTypeParams) *graphql.Object {
						switch p.Value.(type) {
						case *Repo:
							return repoType
						case *Bundle:
							return bundleType
						case *Folder:
							return folderType
						case *File:
							return fileType
						case *Section:
							return sectionType
						case *Definition:
							return definitionType
						case *Contributor:
							return contributorType
						case *Ticket:
							return ticketType
						case *Policy:
							return policyType
						case *ViolationKind:
							return violationKindType
						case *Violation:
							return violationType
						default:
							return nil
						}
					},
				})),
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.ID)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolver.Node(p.Context, id)
				},
			},
			"repo": &graphql.Field{
				Type: graphql.NewNonNull(repoType),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolver.Repo(p.Context)
				},
			},
			"bundle": &graphql.Field{
				Type: bundleType,
				Args: graphql.FieldConfigArgument{
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					name := p.Args["name"].(string)
					return queryResolver.Bundle(p.Context, name)
				},
			},
			"folder": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolver.Folder(p.Context, path)
				},
			},
			"file": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolver.File(p.Context, path)
				},
			},
			"section": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"path":        &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"sectionPath": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					sectionPathRaw := p.Args["sectionPath"].([]interface{})
					sectionPath := make([]string, len(sectionPathRaw))
					for i, v := range sectionPathRaw {
						sectionPath[i] = v.(string)
					}
					return queryResolver.Section(p.Context, path, sectionPath)
				},
			},
			"definition": &graphql.Field{
				Type: definitionType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					name := p.Args["name"].(string)
					return queryResolver.Definition(p.Context, path, name)
				},
			},
			"contributor": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolver.Contributor(p.Context, id)
				},
			},
			"ticket": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"year":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"month": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"day":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"slug":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					year := p.Args["year"].(int)
					month := p.Args["month"].(int)
					day := p.Args["day"].(int)
					slug := p.Args["slug"].(string)
					return queryResolver.Ticket(p.Context, year, month, day, slug)
				},
			},
			"policy": &graphql.Field{
				Type: policyType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolver.Policy(p.Context, id)
				},
			},
			"violationKind": &graphql.Field{
				Type: violationKindType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolver.ViolationKind(p.Context, id)
				},
			},
			"analyze": &graphql.Field{
				Type: graphql.NewNonNull(analyzeResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return queryResolver.Analyze(p.Context, scope)
				},
			},
		},
	})

	mutationResolver := &mutationResolver{resolver}

	fileListInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "FileListInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"updated": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
			"created": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
			"removed": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
		},
	})

	ticketCreateInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketCreateInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"slug":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"model":  &graphql.InputObjectFieldConfig{Type: graphql.String},
			"files":  &graphql.InputObjectFieldConfig{Type: fileListInputType},
		},
	})

	ticketProgressInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketProgressInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"model":  &graphql.InputObjectFieldConfig{Type: graphql.String},
			"files":  &graphql.InputObjectFieldConfig{Type: fileListInputType},
		},
	})

	ticketFinishInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketFinishInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"summary": &graphql.InputObjectFieldConfig{Type: graphql.String},
		},
	})

	ticketReopenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketReopenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorAddInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "ContributorAddInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"github": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"name":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"emails": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
		},
	})

	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"fix": &graphql.Field{
				Type: graphql.NewNonNull(fixResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return mutationResolver.Fix(p.Context, scope)
				},
			},
			"ticketCreate": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketCreateInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketCreateInput{
						Slug:   inputMap["slug"].(string),
						Prompt: inputMap["prompt"].(string),
					}
					if m, ok := inputMap["model"].(string); ok {
						input.Model = &m
					}
					if f, ok := inputMap["files"].(map[string]interface{}); ok {
						input.Files = parseFileListInput(f)
					}
					return mutationResolver.TicketCreate(p.Context, input)
				},
			},
			"ticketProgress": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketProgressInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketProgressInput{
						Year:   inputMap["year"].(int),
						Month:  inputMap["month"].(int),
						Day:    inputMap["day"].(int),
						Slug:   inputMap["slug"].(string),
						Prompt: inputMap["prompt"].(string),
					}
					if m, ok := inputMap["model"].(string); ok {
						input.Model = &m
					}
					if f, ok := inputMap["files"].(map[string]interface{}); ok {
						input.Files = parseFileListInput(f)
					}
					return mutationResolver.TicketProgress(p.Context, input)
				},
			},
			"ticketFinish": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketFinishInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketFinishInput{
						Year:  inputMap["year"].(int),
						Month: inputMap["month"].(int),
						Day:   inputMap["day"].(int),
						Slug:  inputMap["slug"].(string),
					}
					if s, ok := inputMap["summary"].(string); ok {
						input.Summary = &s
					}
					return mutationResolver.TicketFinish(p.Context, input)
				},
			},
			"ticketReopen": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketReopenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketReopenInput{
						Year:  inputMap["year"].(int),
						Month: inputMap["month"].(int),
						Day:   inputMap["day"].(int),
						Slug:  inputMap["slug"].(string),
					}
					return mutationResolver.TicketReopen(p.Context, input)
				},
			},
			"contributorAdd": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(contributorAddInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := ContributorAddInput{
						Github: inputMap["github"].(string),
					}
					if n, ok := inputMap["name"].(string); ok {
						input.Name = &n
					}
					if emails, ok := inputMap["emails"].([]interface{}); ok {
						for _, e := range emails {
							if s, ok := e.(string); ok {
								input.Emails = append(input.Emails, s)
							}
						}
					}
					return mutationResolver.ContributorAdd(p.Context, input)
				},
			},
			"contributorRemove": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"github": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					github := p.Args["github"].(string)
					return mutationResolver.ContributorRemove(p.Context, github)
				},
			},
			"folderCreate": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolver.FolderCreate(p.Context, path)
				},
			},
			"folderMove": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolver.FolderMove(p.Context, src, dst)
				},
			},
			"folderDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolver.FolderDelete(p.Context, path)
				},
			},
			"fileCreate": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolver.FileCreate(p.Context, path)
				},
			},
			"fileMove": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolver.FileMove(p.Context, src, dst)
				},
			},
			"fileDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolver.FileDelete(p.Context, path)
				},
			},
			"sectionCreate": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"parent": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					var parent *string
					if par, ok := p.Args["parent"].(string); ok {
						parent = &par
					}
					return mutationResolver.SectionCreate(p.Context, file, name, parent)
				},
			},
			"sectionMove": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"oldName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"newName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					oldName := p.Args["oldName"].(string)
					newName := p.Args["newName"].(string)
					return mutationResolver.SectionMove(p.Context, file, oldName, newName)
				},
			},
			"sectionDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"file": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					return mutationResolver.SectionDelete(p.Context, file, name)
				},
			},
		},
	})

	_ = rangeType
	_ = countMetricsType

	return graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})
}

// #endregion Schema Builder
