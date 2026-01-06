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
				"sections":    &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType)))},
				"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType)))},
				"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
				"metrics":     &graphql.Field{Type: graphql.NewNonNull(fileMetricsType)},
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
				"file":        &graphql.Field{Type: graphql.NewNonNull(fileType)},
				"parent":      &graphql.Field{Type: sectionType},
				"children":    &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType)))},
				"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType)))},
				"violations":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
				"range":       &graphql.Field{Type: graphql.NewNonNull(rangeType)},
				"metrics":     &graphql.Field{Type: graphql.NewNonNull(sectionMetricsType)},
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

	repoType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Repo",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":           &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":         &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path":         &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"bundles":      &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"folders":      &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType)))},
				"files":        &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"contributors": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType)))},
				"tickets": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
					Args: graphql.FieldConfigArgument{
						"year":   &graphql.ArgumentConfig{Type: graphql.Int},
						"month":  &graphql.ArgumentConfig{Type: graphql.Int},
						"day":    &graphql.ArgumentConfig{Type: graphql.Int},
						"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
					},
				},
				"policies":       &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType)))},
				"violationKinds": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType)))},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
				},
				"metrics": &graphql.Field{Type: graphql.NewNonNull(repoMetricsType)},
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
