// #region Header

// go/repo/graph/schema.resolvers.go

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
	"fmt"
	"path/filepath"
	"strings"
)

// #region Query Resolvers

func (r *Resolver) Query() QueryResolver {
	return &queryResolver{r}
}

type queryResolver struct{ *Resolver }

func (r *queryResolver) Node(ctx context.Context, id string) (Node, error) {
	parts := strings.SplitN(id, ":", 2)
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid node id format: %s", id)
	}
	kind, nodeID := parts[0], parts[1]
	switch kind {
	case "repo":
		return r.Repo(ctx)
	case "bundle":
		return r.Bundle(ctx, nodeID)
	case "folder":
		return r.Folder(ctx, nodeID)
	case "file":
		return r.File(ctx, nodeID)
	case "contributor":
		return r.Contributor(ctx, nodeID)
	case "policy":
		return r.Policy(ctx, nodeID)
	case "violationKind":
		return r.ViolationKind(ctx, nodeID)
	default:
		return nil, fmt.Errorf("unknown node kind: %s", kind)
	}
}

func (r *queryResolver) Repo(ctx context.Context) (*Repo, error) {
	return &Repo{
		ID:   "repo:semio",
		Name: "semio",
		Path: r.RootDir,
	}, nil
}

func (r *queryResolver) Bundle(ctx context.Context, name string) (*Bundle, error) {
	if r.Ctx != nil {
		bundles := r.Ctx.GetBundles()
		for _, b := range bundles {
			if b.Name == name {
				return b, nil
			}
		}
	}
	return &Bundle{
		ID:   fmt.Sprintf("bundle:%s", name),
		Name: name,
		URI:  fmt.Sprintf("file://%s", r.RootDir),
		Tags: []string{},
	}, nil
}

func (r *queryResolver) Folder(ctx context.Context, path string) (*Folder, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	return &Folder{
		ID:   fmt.Sprintf("folder:%s", normalizedPath),
		Path: normalizedPath,
		URI:  fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name: name,
	}, nil
}

func (r *queryResolver) File(ctx context.Context, path string) (*File, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	var folderID *string
	if folderPath != "." {
		id := fmt.Sprintf("folder:%s", folderPath)
		folderID = &id
	}
	return &File{
		ID:        fmt.Sprintf("file:%s", normalizedPath),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
	}, nil
}

func (r *queryResolver) Section(ctx context.Context, path string, sectionPath []string) (*Section, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	sectionName := strings.Join(sectionPath, "#")
	fileID := fmt.Sprintf("file:%s", normalizedPath)
	return &Section{
		ID:     fmt.Sprintf("section:%s#%s", normalizedPath, sectionName),
		Name:   sectionName,
		Path:   fmt.Sprintf("%s#%s", normalizedPath, sectionName),
		FileID: fileID,
	}, nil
}

func (r *queryResolver) Definition(ctx context.Context, path string, name string) (*Definition, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	fileID := fmt.Sprintf("file:%s", normalizedPath)
	return &Definition{
		ID:     fmt.Sprintf("definition:%s§%s", normalizedPath, name),
		Name:   name,
		Kind:   DefinitionKindFunction,
		FileID: fileID,
	}, nil
}

func (r *queryResolver) Contributor(ctx context.Context, id string) (*Contributor, error) {
	if r.Ctx != nil {
		contributors, err := r.Ctx.GetContributors()
		if err == nil {
			for _, c := range contributors {
				if c.Github == id {
					return c, nil
				}
			}
		}
	}
	return &Contributor{
		ID:     fmt.Sprintf("contributor:%s", id),
		Github: id,
		Emails: []string{},
		Links:  []ContributorLink{},
	}, nil
}

func (r *queryResolver) Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error) {
	if r.Ctx != nil {
		y, m, d := year, month, day
		tickets, err := r.Ctx.GetTickets(&y, &m, &d, nil)
		if err == nil {
			for _, t := range tickets {
				if t.Slug == slug {
					return t, nil
				}
			}
		}
	}
	ticketID := fmt.Sprintf("ticket:%d/%02d/%02d/%s", year, month, day, slug)
	return &Ticket{
		ID:     ticketID,
		Year:   year,
		Month:  month,
		Day:    day,
		Slug:   slug,
		Path:   fmt.Sprintf("tickets/%d/%02d/%02d/%s/ticket.md", year, month, day, slug),
		URI:    fmt.Sprintf("file://%s/tickets/%d/%02d/%02d/%s/ticket.md", r.RootDir, year, month, day, slug),
		Prompt: "",
		Status: TicketStatusOpen,
	}, nil
}

func (r *queryResolver) Policy(ctx context.Context, id string) (*Policy, error) {
	if r.Ctx != nil {
		policies := r.Ctx.GetPolicies()
		for _, p := range policies {
			if p.Name == id {
				return p, nil
			}
		}
	}
	return &Policy{
		ID:     fmt.Sprintf("policy:%s", id),
		Name:   id,
		Scopes: []string{},
	}, nil
}

func (r *queryResolver) ViolationKind(ctx context.Context, id string) (*ViolationKind, error) {
	if r.Ctx != nil {
		kinds := r.Ctx.GetViolationKinds()
		for _, k := range kinds {
			if strings.HasSuffix(k.ID, id) {
				return k, nil
			}
		}
	}
	parts := strings.SplitN(id, ":", 2)
	policyID := "unknown"
	if len(parts) > 0 {
		policyID = parts[0]
	}
	return &ViolationKind{
		ID:          fmt.Sprintf("violationKind:%s", id),
		PolicyID:    fmt.Sprintf("policy:%s", policyID),
		Priority:    ViolationPriorityMedium,
		Autofixable: false,
		Reason:      "",
		Solution:    "",
	}, nil
}

func (r *queryResolver) Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Analyze(scope)
	}
	return &AnalyzeResult{
		Violations: []*Violation{},
		Metrics: &AnalyzeMetrics{
			Total:       0,
			ByPriority:  &PriorityCount{High: 0, Medium: 0, Low: 0},
			Autofixable: 0,
		},
	}, nil
}

// #endregion Query Resolvers

// #region Mutation Resolvers

func (r *Resolver) Mutation() MutationResolver {
	return &mutationResolver{r}
}

type mutationResolver struct{ *Resolver }

func (r *mutationResolver) Fix(ctx context.Context, scope *string) (*FixResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Fix(scope)
	}
	return &FixResult{
		Fixed:      0,
		Remaining:  0,
		Violations: []*Violation{},
	}, nil
}

func (r *mutationResolver) TicketCreate(ctx context.Context, input TicketCreateInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketCreate(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketProgress(ctx context.Context, input TicketProgressInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketProgress(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketFinish(ctx context.Context, input TicketFinishInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketFinish(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketReopen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.ContributorAdd(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorRemove(ctx context.Context, github string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.ContributorRemove(github)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderCreate(ctx context.Context, path string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderMove(ctx context.Context, src string, dst string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FolderDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileCreate(ctx context.Context, path string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileMove(ctx context.Context, src string, dst string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FileDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionCreate(file, name, parent)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionMove(file, oldName, newName)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionDelete(ctx context.Context, file string, name string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.SectionDelete(file, name)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

// #endregion Mutation Resolvers

// #region Entity Resolvers

type repoResolver struct{ *Resolver }

func (r *Resolver) Repo_() RepoResolver {
	return &repoResolver{r}
}

func (r *repoResolver) Bundles(ctx context.Context, obj *Repo) ([]*Bundle, error) {
	if r.Ctx != nil {
		return r.Ctx.GetBundles(), nil
	}
	return []*Bundle{}, nil
}

func (r *repoResolver) Folders(ctx context.Context, obj *Repo) ([]*Folder, error) {
	return []*Folder{}, nil
}

func (r *repoResolver) Files(ctx context.Context, obj *Repo) ([]*File, error) {
	return []*File{}, nil
}

func (r *repoResolver) Contributors(ctx context.Context, obj *Repo) ([]*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.GetContributors()
	}
	return []*Contributor{}, nil
}

func (r *repoResolver) Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.GetTickets(year, month, day, status)
	}
	return []*Ticket{}, nil
}

func (r *repoResolver) Policies(ctx context.Context, obj *Repo) ([]*Policy, error) {
	if r.Ctx != nil {
		return r.Ctx.GetPolicies(), nil
	}
	return []*Policy{}, nil
}

func (r *repoResolver) ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKind, error) {
	if r.Ctx != nil {
		return r.Ctx.GetViolationKinds(), nil
	}
	return []*ViolationKind{}, nil
}

func (r *repoResolver) Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error) {
	if r.Ctx != nil {
		result, err := r.Ctx.Analyze(scope)
		if err != nil {
			return nil, err
		}
		return result.Violations, nil
	}
	return []*Violation{}, nil
}

func (r *repoResolver) Metrics(ctx context.Context, obj *Repo) (*RepoMetrics, error) {
	return &RepoMetrics{}, nil
}

// #endregion Entity Resolvers

// #region Interfaces

type QueryResolver interface {
	Node(ctx context.Context, id string) (Node, error)
	Repo(ctx context.Context) (*Repo, error)
	Bundle(ctx context.Context, name string) (*Bundle, error)
	Folder(ctx context.Context, path string) (*Folder, error)
	File(ctx context.Context, path string) (*File, error)
	Section(ctx context.Context, path string, sectionPath []string) (*Section, error)
	Definition(ctx context.Context, path string, name string) (*Definition, error)
	Contributor(ctx context.Context, id string) (*Contributor, error)
	Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error)
	Policy(ctx context.Context, id string) (*Policy, error)
	ViolationKind(ctx context.Context, id string) (*ViolationKind, error)
	Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error)
}

type MutationResolver interface {
	Fix(ctx context.Context, scope *string) (*FixResult, error)
	TicketCreate(ctx context.Context, input TicketCreateInput) (*Ticket, error)
	TicketProgress(ctx context.Context, input TicketProgressInput) (*Ticket, error)
	TicketFinish(ctx context.Context, input TicketFinishInput) (*Ticket, error)
	TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error)
	ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error)
	ContributorRemove(ctx context.Context, github string) (bool, error)
	FolderCreate(ctx context.Context, path string) (*Folder, error)
	FolderMove(ctx context.Context, src string, dst string) (*Folder, error)
	FolderDelete(ctx context.Context, path string) (bool, error)
	FileCreate(ctx context.Context, path string) (*File, error)
	FileMove(ctx context.Context, src string, dst string) (*File, error)
	FileDelete(ctx context.Context, path string) (bool, error)
	SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error)
	SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error)
	SectionDelete(ctx context.Context, file string, name string) (bool, error)
}

type RepoResolver interface {
	Bundles(ctx context.Context, obj *Repo) ([]*Bundle, error)
	Folders(ctx context.Context, obj *Repo) ([]*Folder, error)
	Files(ctx context.Context, obj *Repo) ([]*File, error)
	Contributors(ctx context.Context, obj *Repo) ([]*Contributor, error)
	Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus) ([]*Ticket, error)
	Policies(ctx context.Context, obj *Repo) ([]*Policy, error)
	ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKind, error)
	Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error)
	Metrics(ctx context.Context, obj *Repo) (*RepoMetrics, error)
}

// #endregion Interfaces

// #region Input Types

type TicketCreateInput struct {
	Slug   string         `json:"slug"`
	Prompt string         `json:"prompt"`
	Model  *string        `json:"model,omitempty"`
	Files  *FileListInput `json:"files,omitempty"`
}

type TicketProgressInput struct {
	Year   int            `json:"year"`
	Month  int            `json:"month"`
	Day    int            `json:"day"`
	Slug   string         `json:"slug"`
	Prompt string         `json:"prompt"`
	Model  *string        `json:"model,omitempty"`
	Files  *FileListInput `json:"files,omitempty"`
}

type TicketFinishInput struct {
	Year    int     `json:"year"`
	Month   int     `json:"month"`
	Day     int     `json:"day"`
	Slug    string  `json:"slug"`
	Summary *string `json:"summary,omitempty"`
}

type TicketReopenInput struct {
	Year  int    `json:"year"`
	Month int    `json:"month"`
	Day   int    `json:"day"`
	Slug  string `json:"slug"`
}

type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

type ContributorAddInput struct {
	Github string   `json:"github"`
	Name   *string  `json:"name,omitempty"`
	Emails []string `json:"emails,omitempty"`
}

// #endregion Input Types
